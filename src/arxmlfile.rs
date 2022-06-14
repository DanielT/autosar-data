use crate::*;

impl ArxmlFile {
    pub(crate) fn new(filename: OsString, version: AutosarVersion, container: &AutosarData) -> Self {
        let xsi_schemalocation = CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", version.filename()));
        let xmlns = CharacterData::String("http://autosar.org/schema/r4.0".to_string());
        let xmlns_xsi = CharacterData::String("http://www.w3.org/2001/XMLSchema-instance".to_string());
        let root_attributes = smallvec::smallvec![
            Attribute {attrname: AttributeName::xsiSchemalocation, content: xsi_schemalocation},
            Attribute {attrname: AttributeName::xmlns, content: xmlns},
            Attribute {attrname: AttributeName::xmlnsXsi, content: xmlns_xsi},
        ];
        let root_element = Element(Arc::new(Mutex::new(ElementRaw {
            parent: ElementOrFile::None,
            elemname: specification::ElementName::Autosar,
            type_id: specification::ROOT_DATATYPE,
            content: SmallVec::new(),
            attributes: root_attributes,
        })));
        let file = Self(Arc::new(Mutex::new(ArxmlFileRaw {
            autosar_data: container.downgrade(),
            root_element,
            version,
            filename,
        })));
        let new_parent = ElementOrFile::File(file.downgrade());
        file.root_element().set_parent(new_parent);
        file
    }

    pub fn filename(&self) -> OsString {
        if let Ok(inner) = self.0.lock() {
            inner.filename.clone()
        } else {
            OsString::new()
        }
    }

    pub fn set_filename(&self, new_filename: OsString) {
        if let Ok(mut inner) = self.0.lock() {
            inner.filename = new_filename;
        }
    }

    pub fn version(&self) -> AutosarVersion {
        if let Ok(inner) = self.0.lock() {
            inner.version
        } else {
            // don't expect to ever hit this case, so here's a dummy value
            AutosarVersion::LATEST
        }
    }

    pub fn set_version(&self, new_ver: AutosarVersion) {
        if let Ok(inner) = self.0.lock() {
            let attributevalue = CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", new_ver.filename()));
            inner.root_element.set_attribute_internal(AttributeName::xsiSchemalocation, attributevalue, new_ver);
        }
    }

    pub fn autosar_data(&self) -> Option<AutosarData> {
        if let Ok(locked_file) = self.0.lock() {
            locked_file.autosar_data.upgrade()
        } else {
            None
        }
    }

    pub fn root_element(&self) -> Element {
        let inner = self.0.lock().unwrap();
        inner.root_element.clone()
    }

    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        let inner = self.0.lock().unwrap();
        inner.root_element.elements_dfs()
    }

    pub fn downgrade(&self) -> WeakArxmlFile {
        WeakArxmlFile(Arc::downgrade(&self.0))
    }
}

impl PartialEq for ArxmlFile {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}


impl WeakArxmlFile {
    pub fn upgrade(&self) -> Option<ArxmlFile> {
        Weak::upgrade(&self.0).map(ArxmlFile)
    }
}


impl PartialEq for WeakArxmlFile {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}