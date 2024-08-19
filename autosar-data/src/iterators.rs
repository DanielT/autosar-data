use std::iter::FusedIterator;

use super::*;

#[doc(hidden)]
pub struct ArxmlFileIterator {
    data: AutosarModel,
    index: usize,
}

impl ArxmlFileIterator {
    pub(crate) fn new(data: AutosarModel) -> Self {
        Self { data, index: 0 }
    }
}

impl Iterator for ArxmlFileIterator {
    type Item = ArxmlFile;

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.data.0.read();
        if self.index < model.files.len() {
            let result = model.files[self.index].clone();
            self.index += 1;
            return Some(result);
        }
        None
    }
}

#[doc(hidden)]
pub struct ElementsIterator {
    element: Element,
    index: usize,
    last_output: Option<Element>,
}

impl ElementsIterator {
    pub(crate) fn new(element: Element) -> Self {
        Self {
            element,
            index: 0,
            last_output: None,
        }
    }
}

impl Iterator for ElementsIterator {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.element.0.read();
        while self.index < element.content.len() {
            let ec = &element.content[self.index];
            if let ElementContent::Element(sub_element) = ec {
                if let Some(prev_sub_element) = &self.last_output {
                    if prev_sub_element != sub_element {
                        // self.index does not point to the previously seen element
                        // either self.index was already incremented in this loop, OR
                        // an element was deleted from element.content since the last call to next()
                        self.last_output = Some(sub_element.clone());
                        return self.last_output.clone();
                    } else {
                        // self.index still points to the element that was returned by the previous call
                        self.index += 1;
                    }
                } else {
                    // return the first entry in element.content
                    self.last_output = Some(sub_element.clone());
                    return self.last_output.clone();
                }
            } else {
                // skip character content
                self.index += 1;
            }
        }
        self.index = usize::MAX; // fused
        None
    }
}

impl FusedIterator for ElementsIterator {}

#[doc(hidden)]
pub struct ElementContentIterator {
    element: Element,
    index: usize,
}

impl Iterator for ElementContentIterator {
    type Item = ElementContent;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.element.0.read();
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

#[doc(hidden)]
pub struct ArxmlFileElementsDfsIterator {
    weak_file: WeakArxmlFile,
    dfs_iter: Option<ElementsDfsIterator>,
}

impl ArxmlFileElementsDfsIterator {
    pub(crate) fn new(file: &ArxmlFile) -> Self {
        let weak_file = file.downgrade();
        let dfs_iter = file.model().ok().map(|m| m.elements_dfs());
        Self { weak_file, dfs_iter }
    }
}

impl Iterator for ArxmlFileElementsDfsIterator {
    type Item = (usize, Element);

    fn next(&mut self) -> Option<Self::Item> {
        let iter = self.dfs_iter.as_mut()?;
        let mut next_element = iter.next();
        while let Some((depth, elem)) = next_element {
            let files = elem.file_membership_local();
            if files.is_empty() || files.contains(&self.weak_file) {
                // found an element that is present in this file
                return Some((depth, elem));
            }
            // skip the current subtree
            next_element = iter.next_sibling();
        }
        None
    }
}

#[doc(hidden)]
pub struct ElementsDfsIterator {
    elements: Vec<Element>,
    position: Vec<usize>,
    max_depth: usize,
}

impl ElementsDfsIterator {
    pub(crate) fn new(element: &Element, max_depth: usize) -> Self {
        Self {
            elements: vec![element.clone()],
            position: vec![],
            max_depth,
        }
    }

    pub fn next_sibling(&mut self) -> Option<(usize, Element)> {
        // when an element has bee returned, next always immediatly sets up to show its sub-elements
        // to show a sibling instead we just need to discard the info related to the sub-element
        self.elements.pop();
        self.position.pop();
        self.next()
    }
}

impl Iterator for ElementsDfsIterator {
    type Item = (usize, Element);

    fn next(&mut self) -> Option<Self::Item> {
        while !self.elements.is_empty() {
            let depth = self.elements.len() - 1;
            let element = &self.elements[depth];

            if self.position.len() == depth {
                // return the current element and set up to return its sub-elements next
                self.position.push(0);

                return Some((depth, element.clone()));
            } else {
                // return sub elements?
                let max = self.max_depth;
                if (max == 0 || max > depth) && element.content_item_count() > self.position[depth] {
                    // more items to show
                    if let Some(e) = element.get_sub_element_at(self.position[depth]) {
                        self.elements.push(e);
                    }
                    // show the next item in the next call
                    self.position[depth] += 1;
                } else {
                    // back up one level
                    self.elements.pop();
                    self.position.pop();
                }
            }
        }

        None
    }
}

impl FusedIterator for ElementsDfsIterator {}

#[doc(hidden)]
pub struct AttributeIterator {
    pub(crate) element: Element,
    pub(crate) index: usize,
}

impl Iterator for AttributeIterator {
    type Item = Attribute;

    fn next(&mut self) -> Option<Self::Item> {
        let element = self.element.0.read();
        if self.index < element.attributes.len() {
            let value = element.attributes[self.index].clone();
            self.index += 1;
            Some(value)
        } else {
            self.index = usize::MAX;
            None
        }
    }
}

impl FusedIterator for AttributeIterator {}

#[doc(hidden)]
pub struct IdentifiablesIterator {
    pub(crate) model: AutosarModel,
    pub(crate) position: usize,
}

impl IdentifiablesIterator {
    pub(crate) fn new(model: &AutosarModel) -> Self {
        Self {
            model: model.clone(),
            position: 0,
        }
    }
}

impl Iterator for IdentifiablesIterator {
    type Item = (String, WeakElement);

    fn next(&mut self) -> Option<Self::Item> {
        let model = self.model.0.read();
        if self.position < model.identifiables.len() {
            let pos = self.position;
            self.position += 1;
            model
                .identifiables
                .get_index(pos)
                .map(|(key, elem)| (key.clone(), elem.clone()))
        } else {
            self.position = usize::MAX;
            None
        }
    }
}

impl FusedIterator for IdentifiablesIterator {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn elements_iterator() {
        let model = AutosarModel::new();
        model.create_file("filename", AutosarVersion::LATEST).unwrap();
        let element = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        element
            .create_named_sub_element(ElementName::ArPackage, "pkg1")
            .unwrap();
        element
            .create_named_sub_element(ElementName::ArPackage, "pkg2")
            .unwrap();
        element
            .create_named_sub_element(ElementName::ArPackage, "pkg3")
            .unwrap();
        element
            .create_named_sub_element(ElementName::ArPackage, "pkg4")
            .unwrap();

        // verify that all elements are returned by the iterator
        assert_eq!(element.sub_elements().count(), 4);
        let mut iter = element.sub_elements();
        assert_eq!(iter.next().unwrap().item_name().unwrap(), "pkg1");
        assert_eq!(iter.next().unwrap().item_name().unwrap(), "pkg2");
        assert_eq!(iter.next().unwrap().item_name().unwrap(), "pkg3");
        assert_eq!(iter.next().unwrap().item_name().unwrap(), "pkg4");

        // delete elements from the list while iterating
        let mut steps = 0;
        for se in element.sub_elements() {
            element.remove_sub_element(se).unwrap();
            steps += 1;
        }
        assert_eq!(steps, 4);
        assert_eq!(element.sub_elements().count(), 0);
    }

    #[test]
    fn elements_dfs_iterator() {
        let sub_sub_element = ElementRaw {
            parent: ElementOrModel::None,
            elemname: ElementName::ArPackage, // doesn't matter for this test
            elemtype: ElementType::ROOT,      // doesn't matter for this test
            attributes: SmallVec::new(),
            content: SmallVec::new(),
            file_membership: HashSet::with_capacity(0),
            comment: None,
        }
        .wrap();
        let sub_element = ElementRaw {
            parent: ElementOrModel::None,
            elemname: ElementName::ArPackages, // doesn't matter for this test
            elemtype: ElementType::ROOT,       // doesn't matter for this test
            attributes: SmallVec::new(),
            content: smallvec::smallvec![
                ElementContent::Element(sub_sub_element.clone()),
                ElementContent::Element(sub_sub_element.clone())
            ],
            file_membership: HashSet::with_capacity(0),
            comment: None,
        }
        .wrap();
        let element = ElementRaw {
            parent: ElementOrModel::None,
            elemname: ElementName::Autosar, // doesn't matter for this test
            elemtype: ElementType::ROOT,    // doesn't matter for this test
            attributes: SmallVec::new(),
            content: smallvec::smallvec![
                ElementContent::Element(sub_element.clone()),
                ElementContent::Element(sub_element.clone())
            ],
            file_membership: HashSet::with_capacity(0),
            comment: None,
        }
        .wrap();
        let dfs_iter = element.elements_dfs();
        assert_eq!(dfs_iter.count(), 7);
    }

    #[test]
    fn elements_dfs_next_sibling() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        let el_ar_package_1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package1")
            .unwrap();
        el_ar_package_1.create_sub_element(ElementName::Elements).unwrap();
        let el_ar_package_2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package2")
            .unwrap();
        el_ar_package_2.create_sub_element(ElementName::Elements).unwrap();

        let mut dfs_iter = model.elements_dfs();
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

    #[test]
    fn identifiable_elements_iterator() {
        let model = AutosarModel::new();
        model.create_file("test", AutosarVersion::LATEST).unwrap();
        let el_autosar = model.root_element();
        let el_ar_packages = el_autosar.create_sub_element(ElementName::ArPackages).unwrap();
        el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package1")
            .unwrap();
        el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "Package2")
            .unwrap();

        let iter = model.identifiable_elements();
        assert_eq!(iter.count(), 2);
    }
}
