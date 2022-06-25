use super::*;

pub struct ArxmlFileIterator {
    data: AutosarData,
    index: usize,
}

impl ArxmlFileIterator {
    pub(crate) fn new(data: AutosarData) -> Self {
        Self { data, index: 0 }
    }
}

impl Iterator for ArxmlFileIterator {
    type Item = ArxmlFile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(inner) = self.data.0.lock() {
            if self.index < inner.files.len() {
                let result = inner.files[self.index].clone();
                self.index += 1;
                return Some(result);
            }
        }
        None
    }
}

pub struct ElementsIterator {
    element: Element,
    index: usize,
}

impl ElementsIterator {
    pub(crate) fn new(element: Element) -> Self {
        Self { element, index: 0 }
    }
}

impl Iterator for ElementsIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(inner) = self.element.0.lock() {
            while self.index < inner.content.len() {
                let ec = &inner.content[self.index];
                self.index += 1;
                if let ElementContent::Element(sub_element) = ec {
                    return Some(sub_element.clone());
                }
            }
        }
        None
    }
}

pub struct ElementContentIterator {
    element: Element,
    index: usize,
}

impl Iterator for ElementContentIterator {
    type Item = ElementContent;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(inner) = self.element.0.lock() {
            if self.index < inner.content.len() {
                let ec = &inner.content[self.index];
                self.index += 1;
                return Some(ec.clone());
            }
        }
        None
    }
}

impl ElementContentIterator {
    pub(crate) fn new(element: &Element) -> Self {
        Self {
            element: element.clone(),
            index: 0,
        }
    }
}

pub struct AutosarDataElementsDfsIterator {
    files_iter: ArxmlFileIterator,
    current_dfs_iter: Option<ElementsDfsIterator>,
}

impl AutosarDataElementsDfsIterator {
    pub(crate) fn new(files_iter: ArxmlFileIterator) -> Self {
        Self {
            files_iter,
            current_dfs_iter: None,
        }
    }
}

impl Iterator for AutosarDataElementsDfsIterator {
    type Item = (usize, Element);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(dfs_iter) = &mut self.current_dfs_iter {
            if let Some(result) = dfs_iter.next() {
                return Some(result);
            }
        }
        if let Some(current_file) = &self.files_iter.next() {
            self.current_dfs_iter = Some(current_file.elements_dfs());
            return self.next();
        }

        None
    }
}

pub struct ElementsDfsIterator {
    element: Element,
    elements_iter: Option<ElementsIterator>,
    current_dfs_iter: Option<Box<ElementsDfsIterator>>,
    depth: usize,
}

impl ElementsDfsIterator {
    pub(crate) fn new(element: &Element) -> Self {
        Self {
            element: element.clone(),
            elements_iter: None,
            current_dfs_iter: None,
            depth: 0,
        }
    }
}

impl Iterator for ElementsDfsIterator {
    type Item = (usize, Element);

    fn next(&mut self) -> Option<Self::Item> {
        // first try to descend
        if let Some(dfs_iter) = &mut self.current_dfs_iter {
            if let Some(result) = dfs_iter.next() {
                return Some(result);
            }
        }
        // switch to next sub_element
        if let Some(elements_iter) = &mut self.elements_iter {
            if let Some(next_elem) = elements_iter.next() {
                self.current_dfs_iter = Some(Box::new(ElementsDfsIterator {
                    element: next_elem,
                    elements_iter: None,
                    current_dfs_iter: None,
                    depth: self.depth + 1,
                }));
                // first call of the dfs_iter should alway yield Some(x), because the first element
                // to be returned is the same one that was used to construct the dfs iterator
                return self.next();
            }
            return None;
        }
        // return the starting element
        self.elements_iter = Some(self.element.sub_elements());
        Some((self.depth, self.element.clone()))
    }
}

pub struct AttributeIterator {
    pub(crate) element: Element,
    pub(crate) index: usize,
}

impl Iterator for AttributeIterator {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(inner) = self.element.0.lock() {
            if self.index < inner.attributes.len() {
                let value = inner.attributes[self.index].clone();
                self.index += 1;
                return Some(value);
            }
        }
        None
    }
}

/// An iterator over all identifiable elements in the [AutosarData]
pub struct AutosarDataIdentElementsIterator {
    // The implementation of this iterator has two problems:
    // 1) it's not possible to return references to data protected by a mutex, which makes sense (we would be bypassing the mutex to read through the references)
    //    solution: make a copy
    // 2) As far as I know it's not possible to return references to data contained inside the iterator. It seems GATs would be required for this.
    //    solution: make another copy
    // Overall: not ideal
    identifiables_list: Vec<(String, WeakElement)>,
    position: usize,
}

impl AutosarDataIdentElementsIterator {
    pub(crate) fn new(identifiables: &HashMap<String, WeakElement>) -> Self {
        let mut identifiables_list: Vec<(String, WeakElement)> = identifiables
            .iter()
            .map(|(path, elementref)| (path.to_owned(), elementref.clone()))
            .collect();
        // since we've already copied the data, we can at least sort it
        identifiables_list.sort_by(|(path1, _), (path2, _)| path1.cmp(path2));
        Self {
            identifiables_list,
            position: 0,
        }
    }
}

impl Iterator for AutosarDataIdentElementsIterator {
    type Item = (String, WeakElement);

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.identifiables_list.get(self.position).cloned();
        self.position += 1;
        result
    }
}

#[test]
fn test_elements_dfs_iterator() {
    let sub_sub_element = Element(Arc::new(Mutex::new(ElementRaw {
        parent: ElementOrFile::None,
        elemname: specification::ElementName::ArPackage, // doesn't matter for this test
        type_id: 0,                                      // doesn't matter for this test
        attributes: SmallVec::new(),
        content: SmallVec::new(),
    })));
    let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
        parent: ElementOrFile::None,
        elemname: specification::ElementName::ArPackages, // doesn't matter for this test
        type_id: 0,                                       // doesn't matter for this test
        attributes: SmallVec::new(),
        content: smallvec::smallvec![
            ElementContent::Element(sub_sub_element.clone()),
            ElementContent::Element(sub_sub_element.clone())
        ],
    })));
    let element = Element(Arc::new(Mutex::new(ElementRaw {
        parent: ElementOrFile::None,
        elemname: specification::ElementName::Autosar, // doesn't matter for this test
        type_id: 0,                                    // doesn't matter for this test
        attributes: SmallVec::new(),
        content: smallvec::smallvec![
            ElementContent::Element(sub_element.clone()),
            ElementContent::Element(sub_element.clone())
        ],
    })));
    let dfs_iter = element.elements_dfs();
    assert_eq!(dfs_iter.count(), 7);
}
