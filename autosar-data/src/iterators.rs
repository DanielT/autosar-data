use super::*;

pub struct ArxmlFileIterator {
    data: AutosarProject,
    index: usize,
}

impl ArxmlFileIterator {
    pub(crate) fn new(data: AutosarProject) -> Self {
        Self { data, index: 0 }
    }
}

impl Iterator for ArxmlFileIterator {
    type Item = ArxmlFile;

    fn next(&mut self) -> Option<Self::Item> {
        let project = self.data.0.lock();
        if self.index < project.files.len() {
            let result = project.files[self.index].clone();
            self.index += 1;
            return Some(result);
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
        let element = self.element.0.lock();
        while self.index < element.content.len() {
            let ec = &element.content[self.index];
            self.index += 1;
            if let ElementContent::Element(sub_element) = ec {
                return Some(sub_element.clone());
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
        let element = self.element.0.lock();
        if self.index < element.content.len() {
            let ec = &element.content[self.index];
            self.index += 1;
            return Some(ec.clone());
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

pub struct ArxmlFileElementsDfsIterator {
    file: WeakArxmlFile,
    dfs_iter: ElementsDfsIterator,
}

impl ArxmlFileElementsDfsIterator {
    pub(crate) fn new(file: WeakArxmlFile, element: &Element) -> Self {
        Self {
            file,
            dfs_iter: ElementsDfsIterator::new(element),
        }
    }
}

impl Iterator for ArxmlFileElementsDfsIterator {
    type Item = (usize, Element);

    fn next(&mut self) -> Option<Self::Item> {
        let mut next_element = self.dfs_iter.next();
        while let Some((depth, elem)) = next_element {
            let files = elem.file_membership_local();
            if files.is_empty() || files.contains(&self.file) {
                return Some((depth, elem));
            } else {
                next_element = self.dfs_iter.next_sibling();
            }
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

    pub fn next_sibling(&mut self) -> Option<(usize, Element)> {
        // descend throgh the hierarchy of dfs_iters
        if let Some(dfs_iter) = &mut self.current_dfs_iter {
            // check if the dfs_iter for the "grandchildren" exists. If it does, then descend to the child
            // otherwise do not descend, because it is time to emit the sibling
            if dfs_iter.current_dfs_iter.is_some() {
                if let Some(result) = dfs_iter.next_sibling() {
                    return Some(result);
                }
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
        let element = self.element.0.lock();
        if self.index < element.attributes.len() {
            let value = element.attributes[self.index].clone();
            self.index += 1;
            return Some(value);
        }
        None
    }
}

/// An iterator over all identifiable elements in the [AutosarProject]
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
    pub(crate) fn new(identifiables: &FxHashMap<String, WeakElement>) -> Self {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elements_dfs_iterator() {
        let sub_sub_element = Element(Arc::new(Mutex::new(ElementRaw {
            parent: ElementOrProject::None,
            elemname: ElementName::ArPackage, // doesn't matter for this test
            elemtype: ElementType::ROOT,      // doesn't matter for this test
            attributes: SmallVec::new(),
            content: SmallVec::new(),
            file_membership: HashSet::with_capacity(0),
        })));
        let sub_element = Element(Arc::new(Mutex::new(ElementRaw {
            parent: ElementOrProject::None,
            elemname: ElementName::ArPackages, // doesn't matter for this test
            elemtype: ElementType::ROOT,       // doesn't matter for this test
            attributes: SmallVec::new(),
            content: smallvec::smallvec![
                ElementContent::Element(sub_sub_element.clone()),
                ElementContent::Element(sub_sub_element.clone())
            ],
            file_membership: HashSet::with_capacity(0),
        })));
        let element = Element(Arc::new(Mutex::new(ElementRaw {
            parent: ElementOrProject::None,
            elemname: ElementName::Autosar, // doesn't matter for this test
            elemtype: ElementType::ROOT,    // doesn't matter for this test
            attributes: SmallVec::new(),
            content: smallvec::smallvec![
                ElementContent::Element(sub_element.clone()),
                ElementContent::Element(sub_element.clone())
            ],
            file_membership: HashSet::with_capacity(0),
        })));
        let dfs_iter = element.elements_dfs();
        assert_eq!(dfs_iter.count(), 7);
    }

    #[test]
    fn elements_dfs_next_sibling() {
        let project = AutosarProject::new();
        let el_autosar = project.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package_1 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "Package1") .unwrap();
        el_ar_package_1.create_sub_element(ElementName::Elements).unwrap();
        let el_ar_package_2 = el_ar_packages.create_named_sub_element(ElementName::ArPackage, "Package2") .unwrap();
        el_ar_package_2.create_sub_element(ElementName::Elements).unwrap();

        let mut dfs_iter = project.elements_dfs();
        let (_, item) = dfs_iter.next().unwrap();
        assert_eq!(item, el_autosar);
        let (_, item) = dfs_iter.next().unwrap();
        assert_eq!(item, el_ar_packages);
        let (_, item) = dfs_iter.next().unwrap();
        assert_eq!(item, el_ar_package_1);
        // next() would return the element ELEMENTS inside el_ar_package_1, but next_sibling should return el_ar_package_2 instead
        let (_, item) = dfs_iter.next_sibling().unwrap();
        assert_eq!(item, el_ar_package_2);
    }
}
