use crate::*;

impl ArxmlFile {
    pub(crate) fn new(filename: OsString, version: AutosarVersion, container: &AutosarData) -> Self {
        let xsi_schemalocation =
            CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", version.filename()));
        let xmlns = CharacterData::String("http://autosar.org/schema/r4.0".to_string());
        let xmlns_xsi = CharacterData::String("http://www.w3.org/2001/XMLSchema-instance".to_string());
        let root_attributes = smallvec::smallvec![
            Attribute {
                attrname: AttributeName::xsiSchemalocation,
                content: xsi_schemalocation
            },
            Attribute {
                attrname: AttributeName::xmlns,
                content: xmlns
            },
            Attribute {
                attrname: AttributeName::xmlnsXsi,
                content: xmlns_xsi
            },
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

    /// get the filename of this ArxmnlFile
    pub fn filename(&self) -> OsString {
        if let Ok(inner) = self.0.lock() {
            inner.filename.clone()
        } else {
            OsString::new()
        }
    }

    /// set the filename of this arxml filename
    ///
    /// This will not rename any existing file on disk, but the new filename will be used when writing the data.
    pub fn set_filename(&self, new_filename: OsString) {
        if let Ok(mut inner) = self.0.lock() {
            inner.filename = new_filename;
        }
    }

    /// get the [AutosarVersion] of the file
    pub fn version(&self) -> AutosarVersion {
        if let Ok(inner) = self.0.lock() {
            inner.version
        } else {
            // don't expect to ever hit this case, so here's a dummy value
            AutosarVersion::LATEST
        }
    }

    /// set the [AutosarVersion] of the file
    pub fn set_version(&self, new_ver: AutosarVersion) {
        if let Ok(mut inner) = self.0.lock() {
            inner.version = new_ver;
            let attributevalue =
                CharacterData::String(format!("http://autosar.org/schema/r4.0 {}", new_ver.filename()));
            inner
                .root_element
                .set_attribute_internal(AttributeName::xsiSchemalocation, attributevalue, new_ver);
        }
    }

    /// get a reference to the [AutosarData] object that contains this file
    pub fn autosar_data(&self) -> Result<AutosarData, AutosarDataError> {
        let locked_file = self.0.lock().map_err(|_| AutosarDataError::MutexPoisoned)?;
        // This reference must always be valid, so it is an error if upgrade() fails
        locked_file.autosar_data.upgrade().ok_or(AutosarDataError::ItemDeleted)
    }

    /// get a referenct to the root ```<AUTOSAR ...>``` element of this file
    pub fn root_element(&self) -> Element {
        let inner = self.0.lock().unwrap();
        inner.root_element.clone()
    }

    /// create a depth-first search iterator over all [Element]s in this file
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        let inner = self.0.lock().unwrap();
        inner.root_element.elements_dfs()
    }

    /// create a weak reference to this ArxmlFile
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
    /// try to get a strong reference to the [ArxmlFile]
    ///
    /// This succeeds if the ArxmlFile still has any other strong reference to it, otherwise None is returned
    pub fn upgrade(&self) -> Option<ArxmlFile> {
        Weak::upgrade(&self.0).map(ArxmlFile)
    }
}

impl PartialEq for WeakArxmlFile {
    fn eq(&self, other: &Self) -> bool {
        Weak::as_ptr(&self.0) == Weak::as_ptr(&other.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn create() {
        let data = AutosarData::new();
        let result = data.create_file(OsString::from("test"), AutosarVersion::Autosar_4_0_1);
        assert!(result.is_ok());
    }

    #[test]
    fn filename() {
        let data = AutosarData::new();
        let result = data.create_file(OsString::from("test"), AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let filename = OsString::from("newname.arxml");
        file.set_filename(filename.clone());
        assert_eq!(file.filename(), filename);
    }

    #[test]
    fn version() {
        let data = AutosarData::new();
        let result = data.create_file(OsString::from("test"), AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        file.set_version(AutosarVersion::Autosar_00050);
        assert_eq!(file.version(), AutosarVersion::Autosar_00050);
    }

    #[test]
    fn references() {
        let data = AutosarData::new();
        let result = data.create_file(OsString::from("test"), AutosarVersion::Autosar_4_0_1);
        let file = result.unwrap();
        let weak_file = file.downgrade();
        let file2 = weak_file.upgrade().unwrap();
        assert_eq!(Arc::strong_count(&file.0), 3); // 3 references are: AutosarData, file, file2
        assert_eq!(file, file2);
    }
}
