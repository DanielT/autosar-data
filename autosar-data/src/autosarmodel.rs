use std::{collections::HashMap, hash::Hash};

use crate::*;

#[derive(Debug)]
// actions to be taken when merging two elements
enum MergeAction {
    MergeEqual,
    MergeUnequal(Element),
    AOnly,
    BOnly(usize),
}

// a single mutation performed by the merge, which can be undone again
//
// merge_sub_elements can recover from a failed merge of two elements by importing the element
// of the incoming file as an additional sub element instead. That is only correct if the failed
// merge is undone first: the merge works in-place, so by the time it fails it may already have
// moved sub elements of the incoming element into the existing element. Without the rollback
// these sub elements would end up in the content of both elements at once.
enum MergeUndo {
    // an element of the incoming file was moved into the content of new_parent
    Imported {
        new_parent: Element,
        element: Element,
        old_parent: ElementOrModel,
        old_file_membership: HashSet<WeakArxmlFile>,
    },
    // the file membership of an existing element was replaced
    FileMembership {
        element: Element,
        old_file_membership: HashSet<WeakArxmlFile>,
    },
    // a single file was added to the non-empty file membership of an existing element
    FileMembershipExtended {
        element: Element,
        file: WeakArxmlFile,
    },
}

/// Replace `old_prefix` with `new_prefix` in `path`.
///
/// Returns `None` if `path` is neither `old_prefix` itself nor nested inside it. Unlike a plain
/// `str::starts_with`, the prefix comparison respects the '/' path component boundaries, so
/// "/package10/Elem" is not considered to be inside "/package1".
pub(crate) fn replace_path_prefix(path: &str, old_prefix: &str, new_prefix: &str) -> Option<String> {
    let suffix = path.strip_prefix(old_prefix)?;
    if is_component_boundary(suffix) {
        Some(format!("{new_prefix}{suffix}"))
    } else {
        None
    }
}

/// returns true if `prefix` is `path` itself or a prefix of it at a '/' boundary
///
/// This is the prefix test used by the path index range scans. Unlike a plain `str::starts_with`,
/// it respects the '/' path component boundaries, so "/package10/Elem" is not considered to be
/// inside "/package1".
pub(crate) fn is_path_prefix_of(path: &str, prefix: &str) -> bool {
    path.strip_prefix(prefix).is_some_and(is_component_boundary)
}

/// returns true if `suffix` is the empty remainder of a path prefix match, or starts a new component
fn is_component_boundary(suffix: &str) -> bool {
    suffix.is_empty() || suffix.starts_with('/')
}

/// `PathRemap` describes how Autosar paths change as the result of a rename or move operation
///
/// Each entry maps an old path prefix to a new one. Renaming an element and moving an identifiable
/// element both produce a single entry. Several entries are needed when a non-identifiable element
/// which contains identifiable sub-elements is moved: in that case there is no single old path
/// prefix which covers exactly the moved elements.
pub(crate) struct PathRemap(Vec<(String, String)>);

impl PathRemap {
    /// create a `PathRemap` for a single subtree whose path changes from `old` to `new`
    pub(crate) fn single(old: String, new: String) -> Self {
        PathRemap(vec![(old, new)])
    }

    /// create a `PathRemap` from a list of (old prefix, new prefix) pairs
    pub(crate) fn new(remap: Vec<(String, String)>) -> Self {
        PathRemap(remap)
    }

    /// map an old Autosar path to its new value
    ///
    /// Returns `None` if the path is not affected by this remapping.
    pub(crate) fn map(&self, path: &str) -> Option<String> {
        self.0.iter().find_map(|(old, new)| replace_path_prefix(path, old, new))
    }

    /// the (old prefix, new prefix) pairs of this remapping, in order
    pub(crate) fn entries(&self) -> impl Iterator<Item = &(String, String)> {
        self.0.iter()
    }

    /// returns true if this remapping does not change any path
    pub(crate) fn is_noop(&self) -> bool {
        self.0.iter().all(|(old, new)| old == new)
    }
}

impl AutosarModel {
    /// Create an `AutosarData` model
    ///
    /// Initially it contains no arxml files and only has a default `<AUTOSAR>` element
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// ```
    ///
    #[must_use]
    pub fn new() -> AutosarModel {
        let version = AutosarVersion::LATEST;
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
        let root_elem = ElementRaw {
            parent: ElementOrModel::None,
            elemname: ElementName::Autosar,
            elemtype: ElementType::ROOT,
            content: SmallVec::new(),
            attributes: root_attributes,
            file_membership: None,
            comment: None,
        }
        .wrap();
        let model = AutosarModelRaw {
            files: Arc::new(parking_lot::Mutex::new(Vec::new())),
            identifiables: FxIndexMap::default(),
            reference_origins: FxHashMap::default(),
            relative_references: FxHashMap::default(),
            path_index: LazyPathSet::Missing,
            refpath_index: LazyPathSet::Missing,
            shortname_index: LazyShortnameIndex::Missing,
            root_element: root_elem.clone(),
        }
        .wrap();
        root_elem.set_parent(ElementOrModel::Model(model.downgrade()));
        model
    }

    /// Create a new [`ArxmlFile`] inside this `AutosarData` structure
    ///
    /// You must provide a filename for the [`ArxmlFile`], even if you do not plan to write the data to disk.
    /// You must also specify an [`AutosarVersion`]. All methods manipulation the data insdie the file will ensure conformity with the version specified here.
    /// The newly created `ArxmlFile` will be created with a root AUTOSAR element.
    ///
    /// # Parameters
    ///
    ///  - `filename`: The name of the file to create in the model. It must be unique within the model.
    ///    It is not created on disk until `write()` is called.
    ///  - `version`: The [`AutosarVersion`] that will be used by the data created inside this file
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// let file = model.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///
    pub fn create_file<P: AsRef<Path>>(
        &self,
        filename: P,
        version: AutosarVersion,
    ) -> Result<ArxmlFile, AutosarDataError> {
        // the file list lock is held for the whole operation, so that concurrent calls of
        // create_file / load_buffer / remove_file cannot interleave
        let file_list = self.file_list();
        let mut locked_file_list = file_list.lock();

        if locked_file_list.iter().any(|af| af.filename() == filename.as_ref()) {
            return Err(AutosarDataError::DuplicateFilenameError {
                verb: "create",
                filename: filename.as_ref().to_path_buf(),
            });
        }

        let new_file = ArxmlFile::new(filename, version, self);

        locked_file_list.push(new_file.clone());

        // every file contains the root element (but not its children)
        let _ = self.root_element().add_to_file_restricted(&new_file);

        Ok(new_file)
    }

    /// Load a named buffer containing arxml data
    ///
    /// If you have e.g. received arxml data over a network, or decompressed it from an archive, etc, then you may load it with this method.
    ///
    /// # Parameters:
    ///
    ///  - `buffer`: The data inside the buffer must be valid utf-8. Optionally it may begin with a UTF-8-BOM, which will be silently ignored.
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the `AutosarData` instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// This method may be called concurrently on multiple threads to load different buffers
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// # let buffer = b"";
    /// model.load_buffer(buffer, "filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///  - [`AutosarDataError::OverlappingDataError`]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [`AutosarDataError::ParserError`]: The parser detected an error; the source field gives further details
    ///
    pub fn load_buffer<P: AsRef<Path>>(
        &self,
        buffer: &[u8],
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        self.load_buffer_internal(buffer, filename.as_ref().to_path_buf(), strict)
    }

    fn load_buffer_internal(
        &self,
        buffer: &[u8],
        filename: PathBuf,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        // quick check for a duplicate filename, so that duplicate data is rejected before the
        // expensive parsing step. This check is not authoritative: it is repeated below while
        // the file list lock is held
        if self.file_list().lock().iter().any(|file| file.filename() == filename) {
            return Err(AutosarDataError::DuplicateFilenameError { verb: "load", filename });
        }

        // no lock is held while parsing, so any number of files can be parsed in parallel
        let mut parser = ArxmlParser::new(filename.clone(), buffer, strict);
        parser.model = self.downgrade();
        let root_element = parser.parse_arxml()?;
        let version = parser.get_fileversion();
        let arxml_file = ArxmlFileRaw {
            version,
            model: self.downgrade(),
            filename: filename.clone(),
            xml_standalone: parser.get_standalone(),
        }
        .wrap();

        // the file list lock is held from here to the end of the load operation. It serializes
        // the integration of the parsed data into the model
        let file_list = self.file_list();
        let mut locked_file_list = file_list.lock();

        if locked_file_list.iter().any(|file| file.filename() == filename) {
            return Err(AutosarDataError::DuplicateFilenameError { verb: "load", filename });
        }

        if locked_file_list.is_empty() {
            root_element.set_parent(ElementOrModel::Model(self.downgrade()));
            root_element.0.write().insert_file_membership(arxml_file.downgrade());
            self.0.write().root_element = root_element;
        } else {
            let result = self.merge_file_data(&root_element, arxml_file.downgrade(), &locked_file_list);
            if let Err(error) = result {
                let _ = self.root_element().remove_from_file(&arxml_file);
                return Err(error);
            }
        }

        let mut data = self.0.write();
        // import identifiables from the parser, check for conflicts with existing data
        data.identifiables.reserve(parser.identifiables.len());
        let mut overlap_path = None;
        for (key, value) in parser.identifiables {
            // the same identifiables can be present in multiple files
            // in this case we only keep the first one
            if let Some(existing_element) = data.identifiables.get(&key).and_then(WeakElement::upgrade) {
                // present in both
                if let Some(new_element) = value.upgrade()
                    && existing_element.element_name() != new_element.element_name()
                {
                    // referenced element is different on both sides
                    overlap_path = Some(new_element.xml_path());
                    break;
                }
            } else {
                data.index_shortname(&key);
                data.identifiables.insert(key, value);
            }
        }
        if let Some(path) = overlap_path {
            // undo the partial merge of the new file
            drop(data);
            let _ = self.root_element().remove_from_file(&arxml_file);
            return Err(AutosarDataError::OverlappingDataError { filename, path });
        }
        // The path and short-name indexes are derived from the keys of `identifiables`, but nothing
        // in a load reads them: they are materialized on first use instead (see `path_index`).
        // Building them here cost ~115ms for 173k keys on a 188MB file, on every load.

        // import references from the parser. The parser only knows the character data of each
        // reference, so relative references are recorded as unresolved here; they are resolved once the
        // whole file has been merged and its REFERENCE-BASE declarations are known.
        data.reference_origins.reserve(parser.references.len());
        for (refpath, referring_element, base) in parser.references {
            if base.is_some() {
                data.relative_references.insert(referring_element, None);
            } else {
                data.reference_origins
                    .entry(refpath)
                    .or_default()
                    .push(referring_element);
            }
        }
        // The refpath index is derived from `reference_origins` and is materialized on first use,
        // like the other two.

        locked_file_list.push(arxml_file.clone());
        drop(data);

        // The relative references of the new file could not be resolved while it was being loaded. The
        // new file may also declare REFERENCE-BASEs which change what the relative references of the
        // files loaded before it resolve to, so every relative reference is resolved here, not just the
        // ones which were just added.
        self.resolve_relative_references();

        Ok((arxml_file, parser.warnings))
    }

    // Merge the elements from an incoming arxml file into the overall model
    //
    // The Autosar standard specifies that the data can be split across multiple arxml files
    // It states that each ARXML file can represent an "AUTOSAR Partial Model".
    // The possible partitioning is marked in the meta model, where some elements have the attribute "splitable".
    // These are the points where the overall elements can be split into different arxml files, or, while loading, merged.
    // Unfortunately, the standard says nothing about how this should be done, so the algorithm here is just a guess.
    // In the wild, only merging at the AR-PACKAGES and at the ELEMENTS level exists. Everything else seems like a bad idea anyway.
    fn merge_file_data(
        &self,
        new_root: &Element,
        new_file: WeakArxmlFile,
        file_list: &[ArxmlFile],
    ) -> Result<(), AutosarDataError> {
        let root = self.root_element();
        let files: HashSet<WeakArxmlFile> = file_list.iter().map(ArxmlFile::downgrade).collect();

        // log of all mutations performed by the merge, so that merge_sub_elements can undo the
        // partial merge of a pair of elements which turns out to be unmergeable
        let mut undo = Vec::new();

        Self::merge_element(&root, &files, new_root, &new_file, &mut undo).map_err(|e| {
            // transform ElementInsertionConflict into InvalidFileMerge
            if let AutosarDataError::ElementInsertionConflict { parent_path, .. } = &e {
                AutosarDataError::InvalidFileMerge {
                    path: parent_path.clone(),
                }
            } else {
                e
            }
        })?;

        self.root_element().0.write().insert_file_membership(new_file);

        Ok(())
    }

    fn merge_element(
        parent_a: &Element,
        files: &HashSet<WeakArxmlFile>,
        parent_b: &Element,
        new_file: &WeakArxmlFile,
        undo: &mut Vec<MergeUndo>,
    ) -> Result<(), AutosarDataError> {
        let mut iter_a = parent_a.sub_elements().enumerate();
        let mut iter_b = parent_b.sub_elements();
        let mut item_a = iter_a.next();
        let mut item_b = iter_b.next();
        let mut elements_a_only = Vec::<Element>::new();
        let mut elements_b_only = Vec::<(Element, usize)>::new();
        let mut elements_merge = Vec::<(Element, Element)>::new();
        // lazily built lookup maps over the children of parent_b, and the set of b-elements
        // already consumed by a MergeUnequal match. Linear searches for each element would
        // make the merge quadratic in the number of sub elements
        let mut b_item_name_map: Option<HashMap<(ElementName, Option<String>), Element>> = None;
        let mut b_defref_map: Option<HashMap<(ElementName, Option<String>), Element>> = None;
        let mut merged_b_elements: HashSet<WeakElement> = HashSet::new();
        let min_ver_a = files
            .iter()
            .filter_map(|weak| weak.upgrade().map(|f| f.version()))
            .min()
            .unwrap_or(AutosarVersion::LATEST);
        let min_ver_b = new_file.upgrade().map_or(AutosarVersion::LATEST, |f| f.version());
        let version = std::cmp::min(min_ver_a, min_ver_b);
        let splitable = parent_a.element_type().splittable_in(version);

        while let (Some((pos_a, elem_a)), Some(elem_b)) = (&item_a, &item_b) {
            let merge_action = if elem_a.element_name() == elem_b.element_name() {
                if elem_a.is_identifiable() {
                    Self::calc_identifiables_merge(parent_a, parent_b, elem_a, elem_b, splitable, &mut b_item_name_map)?
                } else {
                    Self::calc_element_merge(parent_b, elem_a, elem_b, &mut b_defref_map)
                }
            } else {
                // a and b are different kinds of elements. This is only allowed if parent is splittable
                let parent_type = parent_a.element_type();
                // The following check does not work, real examples still fail:
                // if !parent_type.splittable_in(self.version()) && parent_a.element_name() != ElementName::ArPackage {
                //     return Err(AutosarDataError::InvalidFileMerge { path: parent_a.xml_path() });
                // }

                let (_, indices_a) = parent_type.find_sub_element(elem_a.element_name(), u32::MAX).unwrap();
                let (_, indices_b) = parent_type.find_sub_element(elem_b.element_name(), u32::MAX).unwrap();
                if indices_a < indices_b {
                    // elem_a comes before elem_b, advance only a
                    // a: <parent> | <a = child 1> <child 2>
                    // b: <parent> |               <b = child 2>
                    MergeAction::AOnly
                } else {
                    // elem_b comes before elem_a, advance only b
                    // a: <parent> |               <a = child 2>
                    // b: <parent> | <b = child 1> <child 2>
                    MergeAction::BOnly(*pos_a)
                }
            };

            match merge_action {
                MergeAction::MergeEqual => {
                    elements_merge.push((elem_a.clone(), elem_b.clone()));
                    item_a = iter_a.next();
                    item_b = iter_b.next();
                }
                MergeAction::MergeUnequal(other_b) => {
                    merged_b_elements.insert(other_b.downgrade());
                    elements_merge.push((elem_a.clone(), other_b));
                    item_a = iter_a.next();
                }
                MergeAction::AOnly => {
                    elements_a_only.push(elem_a.clone());
                    item_a = iter_a.next();
                }
                MergeAction::BOnly(position) => {
                    if !merged_b_elements.contains(&elem_b.downgrade()) {
                        elements_b_only.push((elem_b.clone(), position));
                    }
                    item_b = iter_b.next();
                }
            }
        }
        // at least one of the two iterators has reached the end
        // make sure the other one also reaches the end
        if let Some((_, elem_a)) = item_a {
            elements_a_only.push(elem_a);
            for (_, elem_a) in iter_a {
                elements_a_only.push(elem_a);
            }
        }
        if let Some(elem_b) = item_b {
            let elem_count = parent_a.0.read().content.len();
            if !merged_b_elements.contains(&elem_b.downgrade()) {
                elements_b_only.push((elem_b, elem_count));
            }
            for elem_b in iter_b {
                if !merged_b_elements.contains(&elem_b.downgrade()) {
                    elements_b_only.push((elem_b, elem_count));
                }
            }
        }

        // elements in elements_a_only are already present in the model, so they only need to be restricted
        for element in elements_a_only {
            // files contains the permisions of the parent
            let restricted = {
                let mut elem_locked = element.0.write();
                let restricted = elem_locked.file_membership.is_none();
                if restricted {
                    elem_locked.set_file_membership(files.clone());
                }
                restricted
            };
            if restricted {
                undo.push(MergeUndo::FileMembership {
                    element,
                    old_file_membership: HashSet::new(),
                });
            }
        }

        // elements in elements_b_only are not present in the model yet, so they need to be added
        // this step can fail, in which case the merge of this element fails
        Self::import_new_items(parent_a, elements_b_only, new_file, min_ver_b, undo)?;

        // recurse for sub elements that are present on both sides: these need to be checked and merged
        Self::merge_sub_elements(parent_a, elements_merge, files, new_file, version, undo)?;

        Ok(())
    }

    // calculate how to merge two identifiable elements
    // precondition: both elements have the same element_name
    fn calc_identifiables_merge(
        parent_a: &Element,
        parent_b: &Element,
        elem_a: &Element,
        elem_b: &Element,
        splitable: bool,
        b_item_name_map: &mut Option<HashMap<(ElementName, Option<String>), Element>>,
    ) -> Result<MergeAction, AutosarDataError> {
        Ok(if elem_a.item_name() == elem_b.item_name() {
            // equal
            // advance both iterators
            MergeAction::MergeEqual
        } else {
            // assume that the ordering on both sides is different
            // find a match for a among the siblings of b, using a lookup map that is built once
            // per parent; a linear search for each element would make the merge quadratic
            let map = b_item_name_map.get_or_insert_with(|| {
                let mut map = HashMap::new();
                for e in parent_b.sub_elements() {
                    // or_insert: keep the first element for each key, like the linear search did
                    map.entry((e.element_name(), e.item_name())).or_insert(e);
                }
                map
            });
            if let Some(sibling) = map.get(&(elem_a.element_name(), elem_a.item_name())) {
                // matching item found
                MergeAction::MergeUnequal(sibling.clone())
            } else {
                // element is unique in a
                if splitable {
                    MergeAction::AOnly
                } else {
                    return Err(AutosarDataError::InvalidFileMerge {
                        path: parent_a.xml_path(),
                    });
                }
            }
        })
    }

    // calculate how to merge two elements which are not identifiable
    // precondition: both elements have the same element_name
    fn calc_element_merge(
        parent_b: &Element,
        elem_a: &Element,
        elem_b: &Element,
        b_defref_map: &mut Option<HashMap<(ElementName, Option<String>), Element>>,
    ) -> MergeAction {
        // special case for BSW parameters - many elements used here don't have a SHORT-NAME, but they do have a DEFINITION-REF
        let defref_a = elem_a
            .get_sub_element(ElementName::DefinitionRef)
            .and_then(|dr| dr.character_data())
            .and_then(|cdata| cdata.string_value());
        let defref_b = elem_b
            .get_sub_element(ElementName::DefinitionRef)
            .and_then(|dr| dr.character_data())
            .and_then(|cdata| cdata.string_value());
        // defref_a and _b are simply None for all other elements which don't have a definition-ref

        if defref_a == defref_b {
            // either: defrefs exist and are identical, OR they are both None.
            if elem_a.character_data() != elem_b.character_data() {
                // they have different character data, so they are not identical
                // take only a, defer b
                MergeAction::AOnly
            } else {
                // if they are None, then there is nothing else that can be compared, so we just assume the elements are identical.
                // Merge them and advance both iterators.
                MergeAction::MergeEqual
            }
        } else {
            // check if a sibling of elem_b has the same definiton-ref as elem_a
            // this handles the case where the elements on both sides are ordered differently.
            // The lookup map is built once per parent; a linear search for each element would
            // make the merge quadratic
            let map = b_defref_map.get_or_insert_with(|| {
                let mut map = HashMap::new();
                for e in parent_b.sub_elements() {
                    let defref = e
                        .get_sub_element(ElementName::DefinitionRef)
                        .and_then(|dr| dr.character_data())
                        .and_then(|cdata| cdata.string_value());
                    // or_insert: keep the first element for each key, like the linear search did
                    map.entry((e.element_name(), defref)).or_insert(e);
                }
                map
            });
            if let Some(sibling) = map.get(&(elem_a.element_name(), defref_a)) {
                // a match for item_a exists
                MergeAction::MergeUnequal(sibling.clone())
            } else {
                // element is unique in A
                // This case only happens for BSW definition elements, and it appears that these always have a splittable parent
                MergeAction::AOnly
            }
        }
    }

    fn import_new_items(
        parent_a: &Element,
        elements_b_only: Vec<(Element, usize)>,
        new_file: &WeakArxmlFile,
        version: AutosarVersion,
        undo: &mut Vec<MergeUndo>,
    ) -> Result<(), AutosarDataError> {
        // elements in elements_b_only are not present in the model yet, so they need to be added
        for (idx, (new_element, insert_pos)) in elements_b_only.into_iter().enumerate() {
            // idx number of elements have already been inserted, so the destination position must be adjusted
            let dest = insert_pos + idx;

            Self::import_single_item(parent_a, new_element, dest, new_file, version, undo)?;
        }
        Ok(())
    }

    fn import_single_item(
        parent_a: &Element,
        new_element: Element,
        dest: usize,
        new_file: &WeakArxmlFile,
        version: AutosarVersion,
        undo: &mut Vec<MergeUndo>,
    ) -> Result<(), AutosarDataError> {
        let mut parent_a_locked = parent_a.0.write();

        // add the new_element (from side b) to the content of parent_a
        // to do this, first check valid element insertion positions. Nothing may be modified
        // before this fallible step, so that a failed import leaves new_element untouched
        let (first_pos, last_pos) = parent_a_locked.calc_element_insert_range(new_element.element_name(), version)?;

        // clamp dest, so that first_pos <= dest <= last_pos
        let dest = dest.max(first_pos).min(last_pos);

        let (old_parent, old_file_membership) = {
            let mut new_elem_locked = new_element.0.write();
            let old_parent = std::mem::replace(
                &mut new_elem_locked.parent,
                ElementOrModel::Element(parent_a.downgrade(), parent_a_locked.weak_model()),
            );
            // restrict new_element, it is only present in new_file
            let old_file_membership = new_elem_locked.file_membership_cloned();
            new_elem_locked.insert_file_membership(new_file.clone());
            (old_parent, old_file_membership)
        };
        undo.push(MergeUndo::Imported {
            new_parent: parent_a.clone(),
            element: new_element.clone(),
            old_parent,
            old_file_membership,
        });

        // insert the element from b at the calculated position
        parent_a_locked
            .content
            .insert(dest, ElementContent::Element(new_element));

        Ok(())
    }

    // undo all merge actions recorded after the position mark, in reverse order
    fn rollback_merge(undo: &mut Vec<MergeUndo>, mark: usize) {
        for action in undo.drain(mark..).rev() {
            match action {
                MergeUndo::Imported {
                    new_parent,
                    element,
                    old_parent,
                    old_file_membership,
                } => {
                    // take the element out of the content of its new parent again
                    let mut new_parent_locked = new_parent.0.write();
                    if let Some(pos) = new_parent_locked
                        .content
                        .iter()
                        .position(|item| matches!(item, ElementContent::Element(e) if *e == element))
                    {
                        new_parent_locked.content.remove(pos);
                    }
                    drop(new_parent_locked);
                    // the element is still in the content of its original parent, so the
                    // parent reference must point there again
                    let mut elem_locked = element.0.write();
                    elem_locked.set_parent(old_parent);
                    elem_locked.set_file_membership(old_file_membership);
                }
                MergeUndo::FileMembership {
                    element,
                    old_file_membership,
                } => {
                    element.0.write().set_file_membership(old_file_membership);
                }
                MergeUndo::FileMembershipExtended { element, file } => {
                    element.0.write().remove_file_membership(&file);
                }
            }
        }
    }

    fn merge_sub_elements(
        parent_a: &Element,
        elements_merge: Vec<(Element, Element)>,
        files: &HashSet<WeakArxmlFile>,
        new_file: &WeakArxmlFile,
        version: AutosarVersion,
        undo: &mut Vec<MergeUndo>,
    ) -> Result<(), AutosarDataError> {
        for (elem_a, elem_b) in elements_merge {
            // get the list of files that the element from a is present in
            let files = match elem_a.0.read().file_membership.as_deref() {
                Some(elem_files) => elem_files.clone(),
                None => files.clone(),
            };

            // merge the two elements; remember where the actions of this merge start in the
            // undo log, so that the merge can be rolled back if it fails
            let undo_mark = undo.len();
            let result = AutosarModel::merge_element(&elem_a, &files, &elem_b, new_file, undo);
            match result {
                Ok(()) => {
                    // update the file membership of the merged element, if there was any
                    let mut elem_a_locked = elem_a.0.write();
                    if elem_a_locked.file_membership.is_some() && elem_a_locked.insert_file_membership(new_file.clone())
                    {
                        drop(elem_a_locked);
                        undo.push(MergeUndo::FileMembershipExtended {
                            element: elem_a.clone(),
                            file: new_file.clone(),
                        });
                    }
                }
                Err(e) => {
                    if let AutosarDataError::ElementInsertionConflict { parent_path, .. } = &e {
                        // failed to merge sub element due to insertion conflict
                        if elem_a.is_identifiable() {
                            // if the merge of an identifiable element fails, then the whole merge fails
                            return Err(AutosarDataError::InvalidFileMerge {
                                path: parent_path.clone(),
                            });
                        } else if elem_a.get_sub_element(ElementName::DefinitionRef).is_some() {
                            // elements with DefinitionRef are paired for merging based on the DefinitionRef, so if the merge fails, then the whole merge fails
                            return Err(AutosarDataError::InvalidFileMerge {
                                path: parent_path.clone(),
                            });
                        } else if parent_a.element_type().splittable_in(version) {
                            // undo the partial merge: the failed merge_element may already have
                            // moved sub elements of elem_b into elem_a. These must be returned to
                            // elem_b, otherwise they would be part of both elements at once
                            Self::rollback_merge(undo, undo_mark);

                            let old_file_membership = elem_a.0.read().file_membership_cloned();
                            elem_a.set_file_membership(files);
                            undo.push(MergeUndo::FileMembership {
                                element: elem_a.clone(),
                                old_file_membership,
                            });

                            // try to import elem_b as a new item instead
                            let dest = elem_a.position().unwrap_or_default() + 1;
                            Self::import_single_item(parent_a, elem_b, dest, new_file, version, undo).map_err(|_| e)?;
                            // recovery succeeded: continue with the remaining elements
                            continue;
                        }
                    }

                    // propagate the error back to the parent, perhaps it can be handled there
                    return Err(e);
                }
            }
        }
        Ok(())
    }

    /// Load an arxml file
    ///
    /// This function is a wrapper around `load_buffer` to make the common case of loading a file from disk more convenient
    ///
    /// # Parameters:
    ///
    ///  - `filename`: the original filename of the data, or a newly generated name that is unique within the `AutosarData` instance.
    ///  - `strict`: toggle strict parsing. Some parsing errors are recoverable and can be issued as warnings.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// model.load_file("filename.arxml", true)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::IoErrorRead`]: There was an error while reading the file
    ///  - [`AutosarDataError::DuplicateFilenameError`]: The model already contains a file with this filename
    ///  - [`AutosarDataError::OverlappingDataError`]: The new data contains Autosar paths that are already defined by the existing data
    ///  - [`AutosarDataError::ParserError`]: The parser detected an error; the source field gives further details
    ///
    pub fn load_file<P: AsRef<Path>>(
        &self,
        filename: P,
        strict: bool,
    ) -> Result<(ArxmlFile, Vec<AutosarDataError>), AutosarDataError> {
        let filename_buf = filename.as_ref().to_path_buf();
        let buffer = std::fs::read(&filename_buf).map_err(|err| AutosarDataError::IoErrorRead {
            filename: filename_buf.clone(),
            ioerror: err,
        })?;

        self.load_buffer(&buffer, &filename_buf, strict)
    }

    /// remove a file from the model
    ///
    /// # Parameters:
    ///
    ///  - `file`: The file that will be removed from the model
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// let file = model.create_file("filename.arxml", AutosarVersion::Autosar_00050)?;
    /// model.remove_file(&file);
    /// # Ok(())
    /// # }
    /// ```
    pub fn remove_file(&self, file: &ArxmlFile) {
        // the file list lock is held for the whole operation, so that concurrent calls of
        // create_file / load_buffer / remove_file cannot interleave
        let file_list = self.file_list();
        let mut locked_file_list = file_list.lock();

        let find_result = locked_file_list
            .iter()
            .enumerate()
            .find(|(_, f)| *f == file)
            .map(|(pos, _)| pos);
        if let Some(pos) = find_result {
            locked_file_list.swap_remove(pos);
            if locked_file_list.is_empty() {
                // no other files remain in the model, so it reverts to being empty
                let mut locked_model = self.0.write();
                // clear the parent ref of all sub elements in case other handles to them still exist
                for elem in locked_model.root_element.sub_elements() {
                    elem.detach_recursive();
                }
                locked_model.root_element.0.write().content.clear();
                locked_model.root_element.set_file_membership(HashSet::new());
                locked_model.identifiables.clear();
                locked_model.reference_origins.clear();
                locked_model.relative_references.clear();
                locked_model.path_index.clear();
                locked_model.refpath_index.clear();
                locked_model.shortname_index.clear();
            } else {
                // other files still contribute elements, so only the elements specifically associated with this file should be removed
                let _ = self.root_element().remove_from_file(file);
            }
        }
    }

    /// serialize each of the files in the model
    ///
    /// returns the result in a `HashMap` of <`file_name`, `file_content`>
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// for (pathbuf, file_content) in model.serialize_files() {
    ///     // do something with it
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    #[must_use]
    pub fn serialize_files(&self) -> HashMap<PathBuf, String> {
        let mut result = HashMap::new();
        for file in self.files() {
            if let Ok(data) = file.serialize() {
                result.insert(file.filename(), data);
            }
        }
        result
    }

    /// write all files in the model
    ///
    /// This is a wrapper around `serialize_files`. The current filename of each file will be used to write the serialized data.
    ///
    /// If any of the individual files cannot be written, then `write()` will abort and return the error.
    /// This may result in a situation where some files have been written and others have not.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // load or create files
    /// model.write()?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::IoErrorWrite`]: There was an error while writing a file
    pub fn write(&self) -> Result<(), AutosarDataError> {
        for (pathbuf, filedata) in self.serialize_files() {
            std::fs::write(pathbuf.clone(), filedata).map_err(|err| AutosarDataError::IoErrorWrite {
                filename: pathbuf,
                ioerror: err,
            })?;
        }
        Ok(())
    }

    /// create an iterator over all [`ArxmlFile`]s in this `AutosarData` object
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // load or create files
    /// for file in model.files() {
    ///     // do something with the file
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn files(&self) -> ArxmlFileIterator {
        ArxmlFileIterator::new(self.clone())
    }

    /// get the shared list of files in the model
    ///
    /// The returned mutex is held across model-lock acquisitions by `load_buffer`, `create_file`
    /// and `remove_file`, therefore it must only be locked while holding no other lock.
    pub(crate) fn file_list(&self) -> Arc<parking_lot::Mutex<Vec<ArxmlFile>>> {
        self.0.read().files.clone()
    }

    /// Get a reference to the root ```<AUTOSAR ...>``` element of this model
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let _file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let autosar_element = model.root_element();
    /// ```
    #[must_use]
    pub fn root_element(&self) -> Element {
        let locked_model = self.0.read();
        locked_model.root_element.clone()
    }

    /// get a named element by its Autosar path
    ///
    /// This is a lookup in a hash table and runs in O(1) time
    ///
    /// # Parameters
    ///
    ///  - `path`: The Autosar path to look up
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // [...]
    /// if let Some(element) = model.get_element_by_path("/Path/To/Element") {
    ///     // use the element
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_element_by_path(&self, path: &str) -> Option<Element> {
        let model = self.0.read();
        model.identifiables.get(path).and_then(WeakElement::upgrade)
    }

    /// Duplicate the model
    ///
    /// This creates a second, fully independent model.
    /// The original model and the duplicate are not linked in any way and can be modified independently.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// let model = AutosarModel::new();
    /// // [...]
    /// let model_copy = model.duplicate()?;
    /// assert!(model != model_copy);
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    ///
    ///  - [`AutosarDataError::ItemDeleted`]: The current element is in the deleted state and will be freed once the last reference is dropped
    ///  - [`AutosarDataError::ParentElementLocked`]: a parent element was locked and did not become available after waiting briefly.
    ///    The operation was aborted to avoid a deadlock, but can be retried.
    ///  - [`AutosarDataError::IncorrectContentType`]: A sub element may not be created in an element with content type `CharacterData`.
    ///  - [`AutosarDataError::ElementInsertionConflict`]: The requested sub element cannot be created because it conflicts with an existing sub element.
    ///  - [`AutosarDataError::InvalidSubElement`]: The `ElementName` is not a valid sub element according to the specification.
    ///  - [`AutosarDataError::NoFilesInModel`]: The operation cannot be completed because the model does not contain any files
    pub fn duplicate(&self) -> Result<AutosarModel, AutosarDataError> {
        let copy = Self::new();
        let mut filemap = HashMap::new();

        for orig_file in self.files() {
            let filename = orig_file.filename();
            let new_file = copy.create_file(filename.clone(), orig_file.version())?;
            new_file.0.write().xml_standalone = orig_file.0.read().xml_standalone;
            filemap.insert(filename, new_file.downgrade());
        }

        // by inserting copies of the sub elements of <AUTOSAR>, we automatically
        // get up-to-date identifiables and reference_origins
        for element in self.root_element().sub_elements() {
            let copy_root = copy.root_element();
            let root_weak = copy_root.downgrade();
            copy_root
                .0
                .write()
                .create_copied_sub_element_unfiltered(root_weak, &element, &copy)?;
        }

        // the copies contain unresolved relative references, since a reference base can only be
        // resolved once the whole tree is in place
        copy.resolve_relative_references();

        // `create_copied_sub_element_unfiltered` does not transfer information about file
        // membership, so this needs to be added back.
        let orig_iter = self.elements_dfs();
        let copy_iter = copy.elements_dfs();
        let combined = std::iter::zip(orig_iter, copy_iter);
        for ((_, orig_elem), (_, copy_elem)) in combined {
            // If the copy ever stopped being exact, the two iterators would run out of step and the
            // file membership would be written to the wrong elements, which silently moves elements
            // from one file to another.
            debug_assert_eq!(orig_elem.element_name(), copy_elem.element_name());
            let mut locked_copy = copy_elem.0.try_write().ok_or(AutosarDataError::ParentElementLocked)?;
            locked_copy.file_membership = None;

            let orig_files = orig_elem.0.read().file_membership_cloned();
            for orig_file in orig_files.iter().filter_map(WeakArxmlFile::upgrade) {
                if let Some(copy_file) = filemap.get(&orig_file.filename()) {
                    locked_copy.insert_file_membership(copy_file.clone());
                }
            }
        }

        Ok(copy)
    }

    /// create a depth-first iterator over all [Element]s in the model
    ///
    /// The iterator returns all elements from the merged model, consisting of
    /// data from all arxml files loaded in this model.
    ///
    /// Directly printing the return values could show something like this:
    ///
    /// Note: If the model is modified while iterating, the iterator may skip elements or return duplicates.
    ///
    /// <pre>
    /// 0: AUTOSAR
    /// 1: AR-PACKAGES
    /// 2: AR-PACKAGE
    /// ...
    /// 2: AR-PACKAGE
    /// </pre>
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for (depth, element) in model.elements_dfs() {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn elements_dfs(&self) -> ElementsDfsIterator {
        self.root_element().elements_dfs()
    }

    /// Create a depth first iterator over all [Element]s in this model, up to a maximum depth
    ///
    /// The iterator returns all elements from the merged model, consisting of
    /// data from all arxml files loaded in this model.
    ///
    /// Note: If the model is modified while iterating, the iterator may skip elements or return duplicates.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// # let element = model.root_element();
    /// # element.create_sub_element(ElementName::ArPackages).unwrap();
    /// # let sub_elem = element.get_sub_element(ElementName::ArPackages).unwrap();
    /// # sub_elem.create_named_sub_element(ElementName::ArPackage, "test2").unwrap();
    /// for (depth, elem) in model.elements_dfs_with_max_depth(1) {
    ///     assert!(depth <= 1);
    ///     // ...
    /// }
    /// ```
    #[must_use]
    pub fn elements_dfs_with_max_depth(&self, max_depth: usize) -> ElementsDfsIterator {
        self.root_element().elements_dfs_with_max_depth(max_depth)
    }

    /// Recursively sort all elements in the model. This is exactly identical to calling `sort()` on the root element of the model.
    ///
    /// All sub elements of the root element are sorted alphabetically.
    /// If the sub-elements are named, then the sorting is performed according to the item names,
    /// otherwise the serialized form of the sub-elements is used for sorting.
    ///
    /// Element attributes are not taken into account while sorting.
    /// The elements are sorted in place, and sorting cannot fail, so there is no return value.
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # let model = AutosarModel::new();
    /// # let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// model.sort();
    /// ```
    pub fn sort(&self) {
        self.root_element().sort();
    }

    /// Create an iterator over the list of the Autosar paths of all identifiable elements
    ///
    /// The list contains the full Autosar path of each element. It is not sorted.
    ///
    /// Note: If the model is modified while iterating, the iterator may skip elements or return duplicates.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for (path, _) in model.identifiable_elements() {
    ///     let element = model.get_element_by_path(&path).unwrap();
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn identifiable_elements(&self) -> IdentifiablesIterator {
        IdentifiablesIterator::new(self)
    }

    /// return all elements referring to the given target path
    ///
    /// It returns [`WeakElement`]s which must be upgraded to get usable [Element]s.
    ///
    /// This is effectively the reverse operation of `get_element_by_path()`
    ///
    /// # Parameters
    ///
    ///  - `target_path`: The path whose references should be returned
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for weak_element in model.get_references_to("/Path/To/Element") {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn get_references_to(&self, target_path: &str) -> Vec<WeakElement> {
        let locked_model = self.0.read();
        // relative references are registered under the path of their target as well, so a single lookup
        // finds both kinds
        locked_model
            .reference_origins
            .get(target_path)
            .cloned()
            .unwrap_or_default()
    }

    /// return all elements whose reference resolves to a path under `prefix`
    ///
    /// `prefix` is matched on path component boundaries: the prefix itself and every path nested
    /// below it are selected, while `/package10/Elem` is not selected by `/package1`.
    ///
    /// This is the bulk form of [`Self::get_references_to`], for callers that have to rewrite every
    /// reference pointing into a subtree that is being renamed or moved. Since `reference_origins`
    /// is keyed by the resolved target path, a range scan over its sorted keys replaces walking the
    /// element tree for each rewritten element.
    ///
    /// Absolute references are registered under their character data, so for those the result is
    /// exactly the references whose text lies under the prefix. Relative references are registered
    /// under their *resolved* target path while their character data stays relative; callers that
    /// match on the character data must keep filtering the result themselves.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for element in model.references_under_path_prefix("/Path/To") {
    ///     // [...]
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn references_under_path_prefix(&self, prefix: &str) -> Vec<Element> {
        let mut guard = self.0.write();
        guard.materialize_refpath_index();
        let model = parking_lot::RwLockWriteGuard::downgrade(guard);
        let mut out = Vec::new();
        let index = model.refpath_index.get().expect("just materialized");
        for key in index.range::<str, _>((Bound::Included(prefix), Bound::Unbounded)) {
            if !key.starts_with(prefix) {
                break;
            }
            if !is_path_prefix_of(key, prefix) {
                continue;
            }
            if let Some(reflist) = model.reference_origins.get(key) {
                out.extend(reflist.iter().filter_map(WeakElement::upgrade));
            }
        }
        out
    }

    /// check all Autosar path references and return a list of elements with invalid references
    ///
    /// For each reference: The target must exist and the DEST attribute must correctly specify the type of the target
    ///
    /// Relative references are checked as well: their BASE attribute must name a REFERENCE-BASE
    /// which is in scope, i.e. declared by the package containing the reference or by one of its
    /// ancestor packages, and the relative path must lead to an existing element from there.
    ///
    /// If no references are invalid, then the return value is an empty list
    ///
    /// # Example
    /// ```
    /// # use autosar_data::*;
    /// # fn main() -> Result<(), AutosarDataError> {
    /// # let model = AutosarModel::new();
    /// for broken_ref_weak in model.check_references() {
    ///     if let Some(broken_ref) = broken_ref_weak.upgrade() {
    ///         // update or delete ref?
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn check_references(&self) -> Vec<WeakElement> {
        let mut broken_refs = Vec::new();

        let model = self.0.read();
        for (path, element_list) in &model.reference_origins {
            if let Some(target_elem_weak) = model.identifiables.get(path) {
                // reference target exists
                if let Some(target_elem) = target_elem_weak.upgrade() {
                    // the target of the reference exists, but the reference can still be technically invalid
                    // if the content of the DEST attribute on the reference is wrong
                    for referring_elem_weak in element_list {
                        if let Some(referring_elem) = referring_elem_weak.upgrade() {
                            if let Some(CharacterData::Enum(dest_value)) =
                                referring_elem.attribute_value(AttributeName::Dest)
                            {
                                if !target_elem.element_type().verify_reference_dest(dest_value) {
                                    // wrong reference type in the DEST attribute
                                    broken_refs.push(referring_elem_weak.clone());
                                }
                            } else {
                                // DEST attribute does not exist - can only happen if broken data was loaded with strict == false
                                broken_refs.push(referring_elem_weak.clone());
                            }
                        }
                    }
                } else {
                    // The strong ref count of target_elem can only go to zero if the element is removed,
                    // but remove_element() should also update data.identifiables and data.reference_origins.
                    broken_refs.extend(element_list.iter().cloned());
                }
            } else {
                // reference target does not exist
                broken_refs.extend(element_list.iter().cloned());
            }
        }

        // A relative reference whose base is not in scope has no target path at all, so it is not part
        // of reference_origins and the loop above cannot report it.
        for (referring_elem_weak, target_path) in &model.relative_references {
            if target_path.is_none() && referring_elem_weak.upgrade().is_some() {
                broken_refs.push(referring_elem_weak.clone());
            }
        }

        broken_refs
    }

    /// Get the [`AutosarVersion`] of the only file of this model, if it has exactly one file
    ///
    /// Every element of such a model belongs to that one file, which lets `Element::min_version()`
    /// skip the search for the file membership of the element.
    pub(crate) fn single_file_version(&self) -> Option<AutosarVersion> {
        let file_list = self.file_list();
        let locked_file_list = file_list.lock();
        match locked_file_list.as_slice() {
            [file] => Some(file.version()),
            _ => None,
        }
    }

    /// Create a weak reference to this data
    pub(crate) fn downgrade(&self) -> WeakAutosarModel {
        WeakAutosarModel(Arc::downgrade(&self.0))
    }

    /// Add an identifiable element to the cache
    pub(crate) fn add_identifiable(&self, new_path: String, elem: WeakElement) {
        let mut model = self.0.write();
        if let Some(index) = model.path_index.get_mut() {
            index.insert(new_path.clone());
        }
        // Index only a genuinely new key: re-adding an existing path replaces its entry in place,
        // and appending it again would leave the bucket with a duplicate.
        let is_new = model.identifiables.insert(new_path.clone(), elem).is_none();
        if is_new {
            model.index_shortname(&new_path);
        }
    }

    /// All identifiable elements whose short name (last path segment) is `name`, in
    /// `identifiables` insertion order — the order a linear scan of the index produces.
    ///
    /// # Example
    ///
    /// ```
    /// # use autosar_data::*;
    /// let model = AutosarModel::new();
    /// let file = model.create_file("test", AutosarVersion::Autosar_00050).unwrap();
    /// let root = model.root_element();
    /// let pkgs = root.create_sub_element(ElementName::ArPackages).unwrap();
    /// let pkg = pkgs.create_named_sub_element(ElementName::ArPackage, "Pkg").unwrap();
    /// let found = model.identifiables_with_shortname("Pkg");
    /// assert_eq!(found.len(), 1);
    /// assert_eq!(found[0], pkg);
    /// ```
    #[must_use]
    pub fn identifiables_with_shortname(&self, name: &str) -> Vec<Element> {
        let mut model = self.0.write();
        model.materialize_shortname_index();
        let model = parking_lot::RwLockWriteGuard::downgrade(model);
        let Some(paths) = model.shortname_index.get().and_then(|index| index.get(name)) else {
            return Vec::new();
        };
        if paths.len() == 1 {
            // the common case: a short name is unique, so there is only one order to return
            return paths
                .iter()
                .filter_map(|p| model.identifiables.get(p).and_then(WeakElement::upgrade))
                .collect();
        }
        // `identifiables` uses `swap_remove`, so the bucket order is not the map order. Sort the
        // matching paths by their position in the map, which is what a linear scan yields.
        let mut indexed: Vec<(usize, &String)> = paths
            .iter()
            .filter_map(|path| model.identifiables.get_index_of(path).map(|ord| (ord, path)))
            .collect();
        indexed.sort_unstable_by_key(|(ord, _)| *ord);
        indexed
            .into_iter()
            .filter_map(|(_, p)| model.identifiables.get(p).and_then(WeakElement::upgrade))
            .collect()
    }

    /// Fix the caches after one or more subtrees of elements have been renamed or moved.
    ///
    /// This updates the keys of the `identifiables` map
    pub(crate) fn fix_element_paths(&self, remap: &PathRemap) {
        if remap.is_noop() {
            return;
        }
        let mut model = self.0.write();

        // the renamed element might contain other identifiable elements that are affected by the renaming
        //
        // Only the keys inside a remapped prefix can be affected; collecting the complete key list
        // here meant one full scan of `identifiables` per move, which is quadratic over a pipeline
        // that moves many elements. The result is the same key sequence that the full scan produced.
        model.materialize_path_index();
        let keys = model.keys_under_remap_identifiable(remap);
        for key in keys {
            // find keys referring to entries inside the renamed/moved subtree
            if let Some(new_key) = remap.map(&key) {
                // fix the identifiables hashmap
                if let Some(entry) = model.identifiables.swap_remove(&key) {
                    if let Some(index) = model.path_index.get_mut() {
                        index.remove(&key);
                        index.insert(new_key.clone());
                    }
                    model.unindex_shortname(&key);
                    // `insert` replaces an entry that already holds `new_key`; such a key is
                    // already present in its bucket, and appending it again would duplicate it.
                    if model.identifiables.insert(new_key.clone(), entry).is_none() {
                        model.index_shortname(&new_key);
                    }
                }
            }
        }
    }

    /// Fix the caches which record reference targets, after one or more subtrees of
    /// elements have been renamed or moved, and update the referring elements to match.
    ///
    /// This updates the keys of the `reference_origins` map, as well as the character data of each
    /// referring element, so that both absolute and relative references follow their target. The
    /// entries of `relative_references` are updated to the new keys.
    pub(crate) fn fix_reference_paths(
        &self,
        remap: &PathRemap,
        version: AutosarVersion,
    ) -> Result<(), AutosarDataError> {
        if remap.is_noop() {
            return Ok(());
        }
        let mut model = self.0.write();

        // Check all references and update those that point to a renamed/moved element. Since the key is
        // the path of the target, this applies to relative references in exactly the same way; only the
        // character data that has to be written back differs between the two.
        //
        // Only the keys inside a remapped prefix need the (expensive) per-reference rewrite below.
        // Collecting every key relied on `remap.map` returning None for the others, i.e. one full
        // scan of `reference_origins` per move.
        model.materialize_refpath_index();
        let refpaths = model.keys_under_remap_refpaths(remap);
        for refpath in refpaths {
            // if the existing reference points into a renamed/moved subtree, then it needs to be updated
            let Some(refpath_new) = remap.map(&refpath).filter(|new| *new != refpath) else {
                continue;
            };
            let Some(reflist) = model.reference_origins.remove(&refpath) else {
                continue;
            };
            if let Some(index) = model.refpath_index.get_mut() {
                index.remove(&refpath);
            }
            let mut updated = Vec::with_capacity(reflist.len());
            let mut unchanged = Vec::new();
            for weak_ref_elem in reflist {
                let is_relative = model.relative_references.contains_key(&weak_ref_elem);
                let new_content = match weak_ref_elem.upgrade() {
                    Some(ref_elem) => {
                        let new_content = if is_relative {
                            model.fix_relative_reference_content(&ref_elem, &refpath, &refpath_new, remap)
                        } else {
                            Some(refpath_new.clone())
                        };
                        if let Some(new_content) = &new_content {
                            // can't use Element::set_character_data() here, because the model is locked
                            ref_elem.0.write().set_character_data(new_content.clone(), version)?;
                        }
                        new_content
                    }
                    // the element is gone; the entry follows the key so that it is pruned in one place
                    None => Some(refpath_new.clone()),
                };
                if new_content.is_some() {
                    if is_relative {
                        model
                            .relative_references
                            .insert(weak_ref_elem.clone(), Some(refpath_new.clone()));
                    }
                    updated.push(weak_ref_elem);
                } else {
                    // The new target cannot be expressed relative to this reference base, so the
                    // character data is left alone and the entry keeps its old key. The operation which
                    // moved the target out of the base is responsible for calling
                    // resolve_relative_references() to settle what the reference now points at.
                    unchanged.push(weak_ref_elem);
                }
            }
            if !updated.is_empty() {
                if let Some(index) = model.refpath_index.get_mut() {
                    index.insert(refpath_new.clone());
                }
                model.reference_origins.insert(refpath_new, updated);
            }
            if !unchanged.is_empty() {
                if let Some(index) = model.refpath_index.get_mut() {
                    index.insert(refpath.clone());
                }
                model.reference_origins.insert(refpath, unchanged);
            }
        }

        Ok(())
    }

    // remove a deleted element from the cache
    //
    // The cache is keyed by path, but a path does not identify an element: when two inputs define
    // the same identifiable and merge them, the element that survives can belong to either input,
    // while only one of them ever owns the cache entry. Removing the other one must therefore leave
    // the entry alone, otherwise the surviving element becomes unreachable by path forever.
    pub(crate) fn remove_identifiable(&self, path: &str, elem: &WeakElement) {
        let mut model = self.0.write();
        let owns_entry = model.identifiables.get(path).is_some_and(|existing| existing == elem);
        if !owns_entry {
            return;
        }
        model.identifiables.swap_remove(path);
        if let Some(index) = model.path_index.get_mut() {
            index.remove(path);
        }
        model.unindex_shortname(path);
    }

    /// Register a reference element
    ///
    /// `new_ref` is the character data of the reference and `base` its BASE attribute. An absolute
    /// reference is registered under its character data, which is already the path of its target.
    ///
    /// A relative reference cannot be resolved here: this is also called while element locks are held,
    /// and resolving a reference base means reading the element tree. It is therefore only recorded as
    /// unresolved, and [`Self::resolve_relative_references`] computes its target path afterwards.
    pub(crate) fn add_reference_origin(&self, new_ref: &str, base: Option<&str>, origin: WeakElement) {
        let mut data = self.0.write();
        if base.is_some() {
            data.relative_references.insert(origin, None);
        } else {
            if let Some(index) = data.refpath_index.get_mut() {
                index.insert(new_ref.to_owned());
            }
            data.reference_origins
                .entry(new_ref.to_owned())
                .or_default()
                .push(origin);
        }
    }

    /// De-register a reference element
    ///
    /// `reference` is the character data of the reference, which is the key of an absolute reference.
    /// The key of a relative reference is taken from `relative_references` instead: it cannot be
    /// recomputed here, because this is also called while element locks are held and after the element
    /// has been detached from the tree, when its reference base can no longer be resolved.
    pub(crate) fn remove_reference_origin(&self, reference: &str, element: WeakElement) {
        let mut data = self.0.write();
        let key = match data.relative_references.remove(&element) {
            Some(relative_key) => relative_key,
            None => Some(reference.to_owned()),
        };
        if let Some(key) = key {
            data.remove_reference_origin_by_key(&key, &element);
        }
    }

    /// Move a reference element to a different key, after its character data or BASE attribute changed
    ///
    /// A relative reference is left unresolved, exactly as in [`Self::add_reference_origin`].
    pub(crate) fn fix_reference_origins(
        &self,
        old_ref: &str,
        new_ref: &str,
        new_base: Option<&str>,
        origin: WeakElement,
    ) {
        self.remove_reference_origin(old_ref, origin.clone());
        self.add_reference_origin(new_ref, new_base, origin);
    }

    /// Compute the target path of every relative reference and update the caches to match
    ///
    /// This must be called by any operation which can change what a relative reference resolves to:
    /// the reference itself was created or modified, it moved to a package where a different reference
    /// base is in scope, or a REFERENCE-BASE declaration changed. It is idempotent, and free for the
    /// common case of a model which contains no relative references at all.
    ///
    /// It must not be called while an element lock is held, since resolving a reference base reads the
    /// element tree.
    pub(crate) fn resolve_relative_references(&self) {
        let mut model = self.0.write();
        if model.relative_references.is_empty() {
            return;
        }
        let origins: Vec<WeakElement> = model.relative_references.keys().cloned().collect();
        for origin in origins {
            let new_key = origin
                .upgrade()
                .and_then(|origin_elem| origin_elem.resolve_relative_target());
            let old_key = model
                .relative_references
                .insert(origin.clone(), new_key.clone())
                .flatten();
            if old_key == new_key {
                continue;
            }
            if let Some(old_key) = old_key {
                model.remove_reference_origin_by_key(&old_key, &origin);
            }
            match new_key {
                Some(new_key) => {
                    if let Some(index) = model.refpath_index.get_mut() {
                        index.insert(new_key.clone());
                    }
                    model.reference_origins.entry(new_key).or_default().push(origin);
                }
                // the reference base is not in scope: the reference has no target path, and the entry
                // is only kept so that check_references() can report it
                None => {
                    if origin.upgrade().is_none() {
                        // the element is gone, so the entry is of no use to anyone
                        model.relative_references.remove(&origin);
                    }
                }
            }
        }
    }

    /// Get the absolute path that a relative reference currently resolves to
    ///
    /// The result is `None` if `element` is not a registered relative reference, or if its reference
    /// base is not in scope, in which case it has no target path at all.
    pub(crate) fn relative_reference_target(&self, element: &WeakElement) -> Option<String> {
        self.0.read().relative_references.get(element).cloned().flatten()
    }
}

/// the keys of `index` that are `prefix` itself or nested inside it
///
/// The index is sorted by path, so the matching keys form one contiguous range; the range ends at
/// the first key which no longer carries the prefix.
fn keys_under_prefix<'a>(index: &'a BTreeSet<String>, prefix: &str) -> impl Iterator<Item = &'a String> {
    index
        .range::<str, _>((Bound::Included(prefix), Bound::Unbounded))
        .take_while(move |key| key.starts_with(prefix))
        .filter(move |key| is_path_prefix_of(key, prefix))
}

/// the short name of an Autosar path, i.e. its last segment
///
/// `None` for a path with no non-empty last segment (the empty path, or one ending in `/`), which
/// is never a key of `identifiables`.
fn shortname_of(path: &str) -> Option<&str> {
    path.rsplit('/').next().filter(|name| !name.is_empty())
}

impl AutosarModelRaw {
    /// Add `path` to its short-name bucket, if the index is built.
    fn index_shortname(&mut self, path: &str) {
        let Some(name) = shortname_of(path) else {
            return;
        };
        let Some(index) = self.shortname_index.get_mut() else {
            return;
        };
        index.entry(name.to_owned()).or_default().push(path.to_owned());
    }

    /// Remove `path` from its short-name bucket, if the index is built.
    fn unindex_shortname(&mut self, path: &str) {
        let Some(name) = shortname_of(path) else {
            return;
        };
        let Some(index) = self.shortname_index.get_mut() else {
            return;
        };
        let Some(paths) = index.get_mut(name) else {
            return;
        };
        if let Some(pos) = paths.iter().position(|p| p == path) {
            paths.remove(pos);
        }
        if paths.is_empty() {
            index.remove(name);
        }
    }

    /// build `path_index` from the keys of `identifiables`, if it is not built yet
    fn materialize_path_index(&mut self) {
        if self.path_index.get().is_some() {
            return;
        }
        let mut keys: Vec<String> = self.identifiables.keys().cloned().collect();
        keys.sort_unstable();
        self.path_index = LazyPathSet::Ready(keys.into_iter().collect());
    }

    /// build `refpath_index` from the keys of `reference_origins`, if it is not built yet
    fn materialize_refpath_index(&mut self) {
        if self.refpath_index.get().is_some() {
            return;
        }
        let mut keys: Vec<String> = self.reference_origins.keys().cloned().collect();
        keys.sort_unstable();
        self.refpath_index = LazyPathSet::Ready(keys.into_iter().collect());
    }

    /// build `shortname_index` from the keys of `identifiables`, if it is not built yet
    fn materialize_shortname_index(&mut self) {
        if self.shortname_index.get().is_some() {
            return;
        }
        let mut index: FxHashMap<String, Vec<String>> = FxHashMap::default();
        for key in self.identifiables.keys() {
            let Some(name) = shortname_of(key) else {
                continue;
            };
            index.entry(name.to_owned()).or_default().push(key.clone());
        }
        self.shortname_index = LazyShortnameIndex::Ready(index);
    }

    /// all keys of `identifiables` inside one of the remap's old prefixes, in `identifiables`
    /// insertion order - the order in which `keys()` yielded them
    ///
    /// `swap_remove` moves the last entry into the hole of a removed one, so the insertion order of
    /// `identifiables` is observable through the iterator and through `Debug`, and the fixers must
    /// visit the same keys in the same order as they did when they walked the whole map.
    fn keys_under_remap_identifiable(&self, remap: &PathRemap) -> Vec<String> {
        let mut hits: Vec<(usize, &String)> = Vec::new();
        for (old, _new) in remap.entries() {
            let Some(index) = self.path_index.get() else {
                debug_assert!(false, "keys_under_remap_identifiable needs a materialized path_index");
                return Vec::new();
            };
            for key in keys_under_prefix(index, old) {
                if let Some(ord) = self.identifiables.get_index_of(key) {
                    hits.push((ord, key));
                }
            }
        }
        // a key can lie inside several of the remap's prefixes; it is still a single map entry, and
        // the full scan visited it once
        hits.sort_unstable_by_key(|(ord, _)| *ord);
        hits.dedup_by_key(|(ord, _)| *ord);
        hits.into_iter().map(|(_, key)| key.clone()).collect()
    }

    /// all keys of `reference_origins` inside one of the remap's old prefixes
    ///
    /// Each key is rewritten independently of the others, so the order does not matter here; the
    /// order of `reference_origins` is not observable because it is a `HashMap`. A key which lies
    /// inside several of the remap's prefixes is still a single map entry and is returned once, as
    /// it was by the full scan.
    fn keys_under_remap_refpaths(&self, remap: &PathRemap) -> Vec<String> {
        let mut keys = BTreeSet::new();
        let Some(index) = self.refpath_index.get() else {
            debug_assert!(false, "keys_under_remap_refpaths needs a materialized refpath_index");
            return Vec::new();
        };
        for (old, _new) in remap.entries() {
            keys.extend(keys_under_prefix(index, old).filter(|key| self.reference_origins.contains_key(*key)));
        }
        keys.into_iter().cloned().collect()
    }

    pub(crate) fn wrap(self) -> AutosarModel {
        AutosarModel(Arc::new(RwLock::new(self)))
    }

    /// Rewrite the character data of a relative reference after its target moved from `old_target` to
    /// `new_target`, so that it still refers to the same element. The new character data is returned.
    ///
    /// The path of the reference base is recovered from the old target path and the old character data,
    /// so nothing has to be resolved here. The result is `None` if the new target cannot be expressed
    /// relative to the same reference base, which means the reference cannot follow its target.
    fn fix_relative_reference_content(
        &mut self,
        element: &Element,
        old_target: &str,
        new_target: &str,
        remap: &PathRemap,
    ) -> Option<String> {
        let old_content = element.character_data()?.string_value()?;
        // old_target is the base path followed by '/' and the character data, so what remains after
        // removing those is the path of the reference base
        let base_path = old_target.strip_suffix(&old_content)?.strip_suffix('/')?;
        // the reference base may have been renamed or moved as well
        let new_base_path = remap.map(base_path);
        let new_base_path = new_base_path.as_deref().unwrap_or(base_path);
        let new_content = replace_path_prefix(new_target, new_base_path, "")?
            .strip_prefix('/')?
            .to_owned();

        Some(new_content)
    }

    /// remove `element` from the list of referring elements registered for the target path `key`
    fn remove_reference_origin_by_key(&mut self, key: &str, element: &WeakElement) {
        if let Some(origins) = self.reference_origins.get_mut(key) {
            if let Some(index) = origins.iter().position(|origin| origin == element) {
                origins.swap_remove(index);
            }
            if origins.is_empty() {
                self.reference_origins.remove(key);
                if let Some(index) = self.refpath_index.get_mut() {
                    index.remove(key);
                }
            }
        }
    }
}

impl std::fmt::Debug for AutosarModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // the file list mutex must not be locked while the model lock is held, so the list is cloned first
        let files = self.file_list().lock().clone();
        let model = self.0.read();
        // instead of the usual f.debug_struct().field().field() ...
        // this is disassembled here, in order to hold self.0.lock() as briefly as possible
        let rootelem = model.root_element.clone();
        let mut dbgstruct = f.debug_struct("AutosarModel");
        dbgstruct.field("root_element", &rootelem);
        dbgstruct.field("files", &files);
        dbgstruct.field("identifiables", &model.identifiables);
        dbgstruct.field("reference_origins", &model.reference_origins);
        dbgstruct.field("relative_references", &model.relative_references);
        dbgstruct.finish()
    }
}

impl Default for AutosarModel {
    fn default() -> Self {
        Self::new()
    }
}

impl PartialEq for AutosarModel {
    fn eq(&self, other: &Self) -> bool {
        Arc::as_ptr(&self.0) == Arc::as_ptr(&other.0)
    }
}

impl Eq for AutosarModel {}

impl Hash for AutosarModel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_usize(Arc::as_ptr(&self.0) as usize);
    }
}

impl WeakAutosarModel {
    pub(crate) fn upgrade(&self) -> Option<AutosarModel> {
        Weak::upgrade(&self.0).map(AutosarModel)
    }
}

impl std::fmt::Debug for WeakAutosarModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("AutosarModel:WeakRef {:p}", Weak::as_ptr(&self.0)))
    }
}

/// Verification of the model-level caches, for use by the tests
///
/// Every one of these caches is derived from the element tree, so each mutating operation has to keep
/// them in agreement with it. `check_references` cannot serve as that check: it only reports cache
/// entries which no longer resolve, and never notices a reference which exists in the tree but is
/// missing from the cache.
#[cfg(test)]
impl AutosarModel {
    /// Check the per-element invariants that every mutating operation has to maintain
    ///
    /// Each element caches the model it belongs to next to its parent reference, so every operation
    /// that attaches a subtree to a model, or moves one from another model, has to update the whole
    /// subtree. An element also may not store an empty file membership, which is expressed as
    /// `None` instead.
    fn verify_element_invariants(&self) -> Result<(), String> {
        for (_, element) in self.root_element().elements_dfs() {
            match element.model() {
                Ok(model) if model == *self => {}
                Ok(_) => return Err(format!("element {} points at a different model", element.xml_path())),
                Err(error) => return Err(format!("element {} has no model: {error}", element.xml_path())),
            }
            // an element that is in no file at all is meaningless: an empty set has to be stored as
            // None, otherwise the is_none() check for "inherits its file membership" is wrong
            if element
                .0
                .read()
                .file_membership
                .as_deref()
                .is_some_and(HashSet::is_empty)
            {
                return Err(format!(
                    "element {} stores an empty file membership instead of None",
                    element.xml_path()
                ));
            }
        }
        Ok(())
    }

    /// Check that the reference caches agree with the element tree
    ///
    /// The `Err` describes the first inconsistency that was found. Dead `WeakElement`s in the caches
    /// are ignored, since such entries are only pruned when they happen to be encountered.
    // clippy::mutable_key_type fires for the HashSet<Element>, because an Element contains a lock.
    // Hash and Eq of Element are both defined in terms of the pointer, so interior mutability cannot
    // affect them.
    #[allow(clippy::mutable_key_type)]
    pub(crate) fn verify_reference_caches(&self) -> Result<(), String> {
        // not caches of the model, but subject to the same rule that every mutating operation has
        // to keep them consistent, and checked here so that they are covered by the same tests
        self.verify_element_invariants()?;

        let mut tree_elements = std::collections::HashSet::new();
        // (character data, element) of each reference without a BASE attribute
        let mut absolute_refs: Vec<(String, Element)> = Vec::new();
        // each reference with a BASE attribute, i.e. each relative reference
        let mut relative_refs: Vec<Element> = Vec::new();

        // collect what the element tree says
        for (_, element) in self.root_element().elements_dfs() {
            tree_elements.insert(element.clone());
            if element.is_reference()
                && let Some(CharacterData::String(text)) = element.character_data()
            {
                if element.attribute_value(AttributeName::Base).is_some() {
                    relative_refs.push(element.clone());
                } else {
                    absolute_refs.push((text, element.clone()));
                }
            }
        }

        let mut guard = self.0.write();
        guard.materialize_path_index();
        guard.materialize_refpath_index();
        guard.materialize_shortname_index();
        let model = parking_lot::RwLockWriteGuard::downgrade(guard);

        // an absolute reference is registered under its own character data
        for (text, element) in &absolute_refs {
            if model.relative_references.contains_key(&element.downgrade()) {
                return Err(format!(
                    "{} has no BASE attribute, but is registered as a relative reference",
                    element.xml_path()
                ));
            }
            if !model
                .reference_origins
                .get(text)
                .is_some_and(|origins| origins.contains(&element.downgrade()))
            {
                return Err(format!(
                    "{} references \"{text}\", but is missing from reference_origins[\"{text}\"]",
                    element.xml_path()
                ));
            }
        }

        // a relative reference is registered under the path it resolves to, which the reverse index has
        // to agree with
        for element in &relative_refs {
            let expected_target = element.resolve_relative_target();
            let Some(cached_target) = model.relative_references.get(&element.downgrade()) else {
                return Err(format!(
                    "the relative reference {} is missing from relative_references",
                    element.xml_path()
                ));
            };
            if *cached_target != expected_target {
                return Err(format!(
                    "the relative reference {} resolves to {expected_target:?}, but relative_references says {cached_target:?}",
                    element.xml_path()
                ));
            }
            if let Some(target) = cached_target
                && !model
                    .reference_origins
                    .get(target)
                    .is_some_and(|origins| origins.contains(&element.downgrade()))
            {
                return Err(format!(
                    "the relative reference {} resolves to \"{target}\", but is missing from reference_origins[\"{target}\"]",
                    element.xml_path()
                ));
            }
        }

        // ... and every cache entry must describe a reference which is still in the tree
        for (key, origins) in &model.reference_origins {
            for weak_origin in origins {
                let Some(element) = weak_origin.upgrade() else {
                    continue;
                };
                if !tree_elements.contains(&element) {
                    return Err(format!(
                        "reference_origins[\"{key}\"] contains {}, which is not in the element tree",
                        element.xml_path()
                    ));
                }
                let registered_correctly = match model.relative_references.get(weak_origin) {
                    // a relative reference: the reverse index must name this key
                    Some(target) => target.as_deref() == Some(key.as_str()),
                    // an absolute reference: its character data must be this key
                    None => absolute_refs.iter().any(|(text, elem)| elem == &element && text == key),
                };
                if !registered_correctly {
                    return Err(format!(
                        "reference_origins[\"{key}\"] contains {}, whose reference is {:?} with BASE={:?} and reverse index entry {:?}",
                        element.xml_path(),
                        element.character_data().and_then(|cdata| cdata.string_value()),
                        element
                            .attribute_value(AttributeName::Base)
                            .and_then(|cdata| cdata.string_value()),
                        model.relative_references.get(weak_origin)
                    ));
                }
            }
        }
        for weak_origin in model.relative_references.keys() {
            let Some(element) = weak_origin.upgrade() else {
                continue;
            };
            if !relative_refs.contains(&element) {
                return Err(format!(
                    "relative_references contains {}, which is not a relative reference in the element tree",
                    element.xml_path()
                ));
            }
        }

        // each path index mirrors the keys of the map it indexes: every key is in the index, and
        // the index contains nothing else. The indexes are lazily built, so they are materialized
        // here first - that is exactly the state a reader would see.
        let path_index = model.path_index.get().expect("materialized above");
        let expected_paths: BTreeSet<&String> = model.identifiables.keys().collect();
        if path_index.iter().ne(expected_paths.iter().copied()) {
            return Err(format!(
                "path_index does not match the keys of identifiables: {} index entries, {} keys",
                path_index.len(),
                model.identifiables.len()
            ));
        }
        let refpath_index = model.refpath_index.get().expect("just materialized");
        let expected_refpaths: BTreeSet<&String> = model.reference_origins.keys().collect();
        if refpath_index.iter().ne(expected_refpaths.iter().copied()) {
            return Err(format!(
                "refpath_index does not match the keys of reference_origins: {} index entries, {} keys",
                refpath_index.len(),
                model.reference_origins.len()
            ));
        }

        // the short-name index maps each short name to exactly the paths of `identifiables` which end
        // in it
        let mut expected_shortnames: FxHashMap<&str, Vec<&String>> = FxHashMap::default();
        for path in model.identifiables.keys() {
            if let Some(name) = path.rsplit('/').next()
                && !name.is_empty()
            {
                expected_shortnames.entry(name).or_default().push(path);
            }
        }
        let shortname_index = model.shortname_index.get().expect("just materialized");
        if shortname_index.len() != expected_shortnames.len() {
            return Err(format!(
                "shortname_index holds {} names, but identifiables contains {} distinct short names",
                shortname_index.len(),
                expected_shortnames.len()
            ));
        }
        for (name, paths) in &expected_shortnames {
            let Some(indexed) = shortname_index.get(*name) else {
                return Err(format!("shortname_index is missing the entry for \"{name}\""));
            };
            let mut indexed_sorted: Vec<&str> = indexed.iter().map(String::as_str).collect();
            indexed_sorted.sort_unstable();
            let mut expected_sorted: Vec<&str> = paths.iter().map(|path| path.as_str()).collect();
            expected_sorted.sort_unstable();
            if indexed_sorted != expected_sorted {
                return Err(format!(
                    "shortname_index[\"{name}\"] is {indexed:?}, expected the paths {paths:?}"
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn create_file() {
        let model = AutosarModel::new();
        let file = model.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_ok());
        // error: duplicate file name
        let file = model.create_file("test", AutosarVersion::Autosar_00050);
        assert!(file.is_err());

        // the duplicate check also works when the file name is given as a Path / PathBuf
        let filename = PathBuf::from("test");
        let file = model.create_file(&filename, AutosarVersion::Autosar_00050);
        assert!(matches!(
            file,
            Err(AutosarDataError::DuplicateFilenameError { verb: "create", .. })
        ));
        let file = model.create_file(Path::new("test2"), AutosarVersion::Autosar_00050);
        assert!(file.is_ok());
    }

    #[test]
    fn concurrent_load_buffer() {
        fn make_buf(pkg: &str) -> Vec<u8> {
            format!(
                r#"<?xml version="1.0" encoding="utf-8"?>
            <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
            <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>{pkg}</SHORT-NAME></AR-PACKAGE></AR-PACKAGES>
            </AUTOSAR>"#
            )
            .into_bytes()
        }
        // two concurrent load_buffer calls into an empty model must not interleave;
        // without serialization one of the two element trees could be silently lost
        for _ in 0..100 {
            let model = AutosarModel::new();
            let (m1, m2) = (model.clone(), model.clone());
            let t1 = std::thread::spawn(move || m1.load_buffer(&make_buf("PkgA"), "file1.arxml", true).is_ok());
            let t2 = std::thread::spawn(move || m2.load_buffer(&make_buf("PkgB"), "file2.arxml", true).is_ok());
            assert!(t1.join().unwrap());
            assert!(t2.join().unwrap());
            assert_eq!(model.files().count(), 2);
            assert!(model.get_element_by_path("/PkgA").is_some());
            assert!(model.get_element_by_path("/PkgB").is_some());
        }

        // two concurrent loads of the same filename: exactly one of them must succeed,
        // even though the duplicate check before parsing cannot see the other load yet
        for _ in 0..100 {
            let model = AutosarModel::new();
            let (m1, m2) = (model.clone(), model.clone());
            let t1 = std::thread::spawn(move || m1.load_buffer(&make_buf("PkgA"), "file1.arxml", true).is_ok());
            let t2 = std::thread::spawn(move || m2.load_buffer(&make_buf("PkgB"), "file1.arxml", true).is_ok());
            let ok1 = t1.join().unwrap();
            let ok2 = t2.join().unwrap();
            assert!(ok1 != ok2, "exactly one of the two loads must succeed");
            assert_eq!(model.files().count(), 1);
        }
    }

    #[test]
    fn load_buffer() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
              <SYSTEM><SHORT-NAME>Thing</SHORT-NAME></SYSTEM>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>OtherPkg</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF3: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
            <APPLICATION-PRIMITIVE-DATA-TYPE><SHORT-NAME>Thing</SHORT-NAME></APPLICATION-PRIMITIVE-DATA-TYPE>
            </ELEMENTS>
          </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const NON_ARXML: &str = "The quick brown fox jumps over the lazy dog";
        let model = AutosarModel::new();
        // succefully load a buffer
        let result = model.load_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_ok());
        // succefully load a second buffer
        let result = model.load_buffer(FILEBUF2.as_bytes(), "other", true);
        assert!(result.is_ok());
        // error: duplicate file name
        let result = model.load_buffer(FILEBUF.as_bytes(), "test", true);
        assert!(result.is_err());
        // error: overlapping autosar paths
        let result = model.load_buffer(FILEBUF3.as_bytes(), "test2", true);
        assert!(result.is_err());
        // error: not arxml data
        let result = model.load_buffer(NON_ARXML.as_bytes(), "nonsense", true);
        assert!(result.is_err());
    }

    #[test]
    fn load_file() {
        let dir = tempdir().unwrap();

        let model = AutosarModel::new();
        let filename = dir.path().with_file_name("nonexistent.arxml");
        assert!(model.load_file(&filename, true).is_err());

        let filename = dir.path().with_file_name("test.arxml");
        model.create_file(&filename, AutosarVersion::LATEST).unwrap();
        model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .and_then(|ap| ap.create_named_sub_element(ElementName::ArPackage, "Pkg"))
            .unwrap();
        model.write().unwrap();

        assert!(filename.exists());

        // careate a new model without data
        let model = AutosarModel::new();
        model.load_file(&filename, true).unwrap();
        let el_pkg = model.get_element_by_path("/Pkg");
        assert!(el_pkg.is_some());
    }

    #[test]
    fn data_merge() {
        const FILEBUF1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
            <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
              <SHORT-NAME>BswModuleValues</SHORT-NAME>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_B</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_C</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
          </ELEMENTS></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_B</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_C</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const FILEBUF2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg_B</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
            <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
              <SHORT-NAME>BswModuleValues</SHORT-NAME>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_B</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
          </ELEMENTS></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        // test with re-ordered identifiable elements and re-ordered BSW parameter values
        // file2 is a subset of file1, so the total number of elements does not increase
        let model = AutosarModel::new();
        let (file1, _) = model.load_buffer(FILEBUF1, "test1", true).unwrap();
        let file1_elemcount = file1.elements_dfs().count();
        let (file2, _) = model.load_buffer(FILEBUF2, "test2", true).unwrap();
        let file2_elemcount = file2.elements_dfs().count();
        let model_elemcount = model.elements_dfs().count();
        assert_eq!(file1_elemcount, model_elemcount);
        assert!(file1_elemcount > file2_elemcount);
        // verify file membership after merging
        let (local, fileset) = model.root_element().file_membership().unwrap();
        assert!(local);
        assert_eq!(fileset.len(), 2);

        let el_pkg_c = model.get_element_by_path("/Pkg_C").unwrap();
        let (local, fileset) = el_pkg_c.file_membership().unwrap();
        assert!(local);
        assert_eq!(fileset.len(), 1);
        let el_npv2 = model
            .get_element_by_path("/Pkg_A/BswModule/BswModuleValues")
            .and_then(|bmv| bmv.get_sub_element(ElementName::ParameterValues))
            .and_then(|pv| pv.get_sub_element_at(2))
            .unwrap();
        let (loc, fm) = el_npv2.file_membership().unwrap();
        assert!(loc);
        assert_eq!(fm.len(), 1);

        // the following two files diverge on the TIMING-RESOURCE element
        // this is not permitted, because SYSTEM-TIMING is not splittable
        const ERRFILE1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <SYSTEM-TIMING>
              <SHORT-NAME>SystemTimings</SHORT-NAME>
              <CATEGORY>CAT</CATEGORY>
              <TIMING-RESOURCE>
                <SHORT-NAME>Name_One</SHORT-NAME>
              </TIMING-RESOURCE>
            </SYSTEM-TIMING>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const ERRFILE2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <SYSTEM-TIMING>
              <SHORT-NAME>SystemTimings</SHORT-NAME>
              <TIMING-RESOURCE>
                <SHORT-NAME>Name_Two</SHORT-NAME>
              </TIMING-RESOURCE>
            </SYSTEM-TIMING>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let result = model.load_buffer(ERRFILE1, "test1", true);
        assert!(result.is_ok());
        let result = model.load_buffer(ERRFILE2, "test2", true);
        let error = result.unwrap_err();
        assert!(matches!(error, AutosarDataError::InvalidFileMerge { .. }));

        // diverging files, where each file uses a different element from a Choice set.
        // In this case the COMPU-SCALE in ERRFILE3 uses COMPU-CONST while ERRFILE4 uses COMPU-RATIONAL-COEFFS.
        // This is not permitted, because the COMPU-SCALE can only contain one or the other.
        const ERRFILE3: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <COMPU-METHOD><SHORT-NAME>compu</SHORT-NAME>
              <COMPU-INTERNAL-TO-PHYS>
                <COMPU-SCALES>
                  <COMPU-SCALE><COMPU-CONST></COMPU-CONST></COMPU-SCALE>
                </COMPU-SCALES>
              </COMPU-INTERNAL-TO-PHYS>
            </COMPU-METHOD>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const ERRFILE4: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
          <ELEMENTS>
            <COMPU-METHOD><SHORT-NAME>compu</SHORT-NAME>
              <COMPU-INTERNAL-TO-PHYS>
                <COMPU-SCALES>
                  <COMPU-SCALE><COMPU-RATIONAL-COEFFS></COMPU-RATIONAL-COEFFS></COMPU-SCALE>
                </COMPU-SCALES>
              </COMPU-INTERNAL-TO-PHYS>
            </COMPU-METHOD>
          </ELEMENTS>
        </AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let result = model.load_buffer(ERRFILE3, "test3", true);
        assert!(result.is_ok());
        let result = model.load_buffer(ERRFILE4, "test4", true);
        let error = result.unwrap_err();
        assert!(matches!(error, AutosarDataError::InvalidFileMerge { .. }));

        // non-overlapping files
        const FILEBUF3: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Package2</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const FILEBUF4: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        let model_a = AutosarModel::new();
        model_a.load_buffer(FILEBUF3, "test5", true).unwrap();
        model_a.load_buffer(FILEBUF4, "test6", true).unwrap();
        // load the files into model_b in reverse order
        let model_b = AutosarModel::new();
        model_b.load_buffer(FILEBUF4, "test5", true).unwrap();
        model_b.load_buffer(FILEBUF3, "test6", true).unwrap();
        // the two models should be equal
        model_a.sort();
        let model_a_txt = model_a.root_element().serialize();
        model_b.sort();
        let model_b_txt = model_b.root_element().serialize();
        assert_eq!(model_a_txt, model_b_txt);
    }

    #[test]
    fn remove_file() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00049.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package</SHORT-NAME>
        <ELEMENTS><CAN-CLUSTER><SHORT-NAME>CAN_Cluster</SHORT-NAME></CAN-CLUSTER></ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF3: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Package2</SHORT-NAME>
        <ELEMENTS><SYSTEM><SHORT-NAME>System</SHORT-NAME>
        <FIBEX-ELEMENTS><FIBEX-ELEMENT-REF-CONDITIONAL>
            <FIBEX-ELEMENT-REF DEST="CAN-CLUSTER">/Package/CAN_Cluster</FIBEX-ELEMENT-REF>
        </FIBEX-ELEMENT-REF-CONDITIONAL></FIBEX-ELEMENTS>
        </SYSTEM></ELEMENTS></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        // easy case: remove the only file
        let model = AutosarModel::new();
        let (file, _) = model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        assert_eq!(model.files().count(), 1);
        assert_eq!(model.identifiable_elements().count(), 1);
        model.remove_file(&file);
        assert_eq!(model.files().count(), 0);
        assert_eq!(model.identifiable_elements().count(), 0);
        // complicated: remove one of several files
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test1", true).unwrap();
        assert_eq!(model.files().count(), 1);
        let modeltxt_1 = model.root_element().serialize();
        let (file2, _) = model.load_buffer(FILEBUF2.as_bytes(), "test2", true).unwrap();
        assert_eq!(model.files().count(), 2);
        let modeltxt_1_2 = model.root_element().serialize();
        assert_ne!(modeltxt_1, modeltxt_1_2);
        let (file3, _) = model.load_buffer(FILEBUF3.as_bytes(), "test3", true).unwrap();
        assert_eq!(model.files().count(), 3);
        let modeltxt_1_2_3 = model.root_element().serialize();
        assert_ne!(modeltxt_1_2, modeltxt_1_2_3);
        model.get_element_by_path("/Package2/System").unwrap();
        model.remove_file(&file3);
        // the serialized text of the model after deleting file 3 should be the same as it was before loading file 3
        let modeltxt_1_2_x = model.root_element().serialize();
        assert_eq!(modeltxt_1_2, modeltxt_1_2_x);
        model.remove_file(&file2);
        // the serialized text of the model after deleting files 2 and 3 should be the same as it was before loading files 2 and 3
        let modeltxt_1_x_x = model.root_element().serialize();
        assert_eq!(modeltxt_1, modeltxt_1_x_x);
        assert_eq!(model.files().count(), 1);
    }

    // Two files define the same identifiable, but list the package children in opposite order, so the
    // merge imports the second file's element while the first file still owns the cache entry. Removing
    // the second file must not evict the entry of the element that survives - otherwise that element can
    // never be found by path again.
    #[test]
    fn remove_duplicate_file_keeps_index_entry() {
        const FILEBUF_A: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
    <AR-PACKAGES>
        <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <DATA-CONSTR><SHORT-NAME>Dc</SHORT-NAME></DATA-CONSTR>
                <CLIENT-SERVER-INTERFACE><SHORT-NAME>If</SHORT-NAME></CLIENT-SERVER-INTERFACE>
            </ELEMENTS>
        </AR-PACKAGE>
    </AR-PACKAGES>
</AUTOSAR>"#;
        const FILEBUF_B: &str = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
    <AR-PACKAGES>
        <AR-PACKAGE>
            <SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <CLIENT-SERVER-INTERFACE><SHORT-NAME>If</SHORT-NAME></CLIENT-SERVER-INTERFACE>
                <DATA-CONSTR><SHORT-NAME>Dc</SHORT-NAME></DATA-CONSTR>
            </ELEMENTS>
        </AR-PACKAGE>
    </AR-PACKAGES>
</AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF_A.as_bytes(), "a", true).unwrap();
        let (file_b, _) = model.load_buffer(FILEBUF_B.as_bytes(), "b", true).unwrap();
        assert!(model.get_element_by_path("/Pkg/Dc").is_some());
        assert!(model.get_element_by_path("/Pkg/If").is_some());
        model.remove_file(&file_b);
        assert!(
            model.get_element_by_path("/Pkg/Dc").is_some(),
            "removing a duplicate file dropped the index entry of the surviving element"
        );
        assert!(model.get_element_by_path("/Pkg/If").is_some());
    }

    #[test]
    fn remove_last_file_clears_reference_caches() {
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd">
    <AR-PACKAGES>
        <AR-PACKAGE>
            <SHORT-NAME>BasePkg</SHORT-NAME>
            <ELEMENTS>
                <ECU-INSTANCE>
                    <SHORT-NAME>Ecu</SHORT-NAME>
                </ECU-INSTANCE>
            </ELEMENTS>
        </AR-PACKAGE>
        <AR-PACKAGE>
            <SHORT-NAME>RefPkg</SHORT-NAME>
            <REFERENCE-BASES>
                <REFERENCE-BASE>
                    <SHORT-LABEL>BaseA</SHORT-LABEL>
                    <PACKAGE-REF DEST="AR-PACKAGE">/BasePkg</PACKAGE-REF>
                </REFERENCE-BASE>
            </REFERENCE-BASES>
            <ELEMENTS>
                <SYSTEM>
                    <SHORT-NAME>Sys</SHORT-NAME>
                    <FIBEX-ELEMENTS>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="BaseA">Ecu</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                    </FIBEX-ELEMENTS>
                </SYSTEM>
            </ELEMENTS>
        </AR-PACKAGE>
    </AR-PACKAGES>
</AUTOSAR>"#
        .as_bytes();

        let model = AutosarModel::new();
        let (file, _) = model.load_buffer(FILEBUF, "test", true).unwrap();

        assert!(!model.0.read().relative_references.is_empty());
        assert!(!model.0.read().reference_origins.is_empty());
        model.remove_file(&file);

        assert!(model.0.read().relative_references.is_empty());
        assert!(model.0.read().reference_origins.is_empty());
    }

    #[test]
    fn refcount() {
        let model = AutosarModel::default();
        let weak = model.downgrade();
        let project2 = weak.upgrade();
        assert_eq!(Arc::strong_count(&model.0), 2);
        assert_eq!(model, project2.unwrap());
    }

    #[test]
    fn identifiables_iterator() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>OuterPackage1</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        <AR-PACKAGE><SHORT-NAME>OuterPackage2</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        let mut identifiable_elements = model.identifiable_elements().collect::<Vec<_>>();
        identifiable_elements.sort_by(|a, b| a.0.cmp(&b.0));
        assert_eq!(identifiable_elements[0].0, "/OuterPackage1");
        assert_eq!(identifiable_elements[1].0, "/OuterPackage1/InnerPackage1");
        assert_eq!(identifiable_elements[2].0, "/OuterPackage1/InnerPackage2");
        assert_eq!(identifiable_elements[3].0, "/OuterPackage2");
        assert_eq!(identifiable_elements[4].0, "/OuterPackage2/InnerPackage1");
        assert_eq!(identifiable_elements[5].0, "/OuterPackage2/InnerPackage2");
    }

    #[test]
    fn identifiables_with_shortname() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>OuterPackage1</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        <AR-PACKAGE><SHORT-NAME>OuterPackage2</SHORT-NAME>
            <AR-PACKAGES>
                <AR-PACKAGE><SHORT-NAME>InnerPackage1</SHORT-NAME></AR-PACKAGE>
                <AR-PACKAGE><SHORT-NAME>InnerPackage2</SHORT-NAME></AR-PACKAGE>
            </AR-PACKAGES>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        assert_eq!(model.verify_reference_caches(), Ok(()));

        // the two packages share the short name; both are found, in identifiables order
        let found = model.identifiables_with_shortname("InnerPackage1");
        assert_eq!(found.len(), 2);
        let expected: Vec<String> = model
            .identifiable_elements()
            .map(|(path, _)| path)
            .filter(|path| path.ends_with("/InnerPackage1"))
            .collect();
        let actual: Vec<String> = found.iter().map(|e| e.path().unwrap()).collect();
        assert_eq!(actual, expected);

        // an unknown name yields nothing
        assert!(model.identifiables_with_shortname("Nonexistent").is_empty());

        // a removal keeps the result in identifiables order: the surviving package of each name is
        // the one that `swap_remove` did not move
        model
            .get_element_by_path("/OuterPackage1")
            .unwrap()
            .remove_sub_element_kind(ElementName::ArPackages)
            .unwrap();
        assert_eq!(model.verify_reference_caches(), Ok(()));
        let found = model.identifiables_with_shortname("InnerPackage1");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].path().unwrap(), "/OuterPackage2/InnerPackage1");

        // a rename moves the entry to the new name
        model
            .get_element_by_path("/OuterPackage2/InnerPackage1")
            .unwrap()
            .set_item_name("Renamed")
            .unwrap();
        assert!(model.identifiables_with_shortname("InnerPackage1").is_empty());
        assert_eq!(model.identifiables_with_shortname("Renamed").len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn shortname_index_survives_mutations_after_materialization() {
        // The index is maintained in place by every key mutation instead of being dropped, so a
        // pipeline that interleaves lookups with moves does not rebuild it once per lookup. That
        // is only correct if every mutation path keeps it in sync: a missed add, remove or re-key
        // leaves the bucket holding a stale path, a duplicate, or nothing at all.
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <SW-BASE-TYPE><SHORT-NAME>base</SHORT-NAME><BASE-TYPE-SIZE>8</BASE-TYPE-SIZE></SW-BASE-TYPE>
            </ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();

        // Build the index BEFORE any mutation: from here on it is maintained incrementally.
        assert_eq!(model.identifiables_with_shortname("base").len(), 1);

        let pkg = model.get_element_by_path("/Pkg").unwrap();
        let elements = pkg.get_sub_element(ElementName::Elements).unwrap();

        // create -> add_identifiable
        let copy = elements
            .create_named_sub_element(ElementName::SwBaseType, "copied")
            .unwrap();
        assert_eq!(model.identifiables_with_shortname("copied").len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));

        // deep copy -> add_identifiable for the copy and its subtree
        let deep = elements.create_copied_sub_element(&copy).unwrap();
        let deep_name = deep.item_name().unwrap();
        assert_ne!(deep_name, "copied", "the copy gets a unique name");
        assert_eq!(model.identifiables_with_shortname(&deep_name).len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));

        // rename -> fix_element_paths (re-key, not drop)
        copy.set_item_name("renamed").unwrap();
        assert!(model.identifiables_with_shortname("copied").is_empty());
        assert_eq!(model.identifiables_with_shortname("renamed").len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));

        // move -> fix_element_paths, name unchanged so the bucket entry must survive
        let other = model
            .get_element_by_path("/Pkg")
            .unwrap()
            .get_or_create_sub_element(ElementName::ArPackages)
            .unwrap()
            .create_named_sub_element(ElementName::ArPackage, "Other")
            .unwrap()
            .get_or_create_sub_element(ElementName::Elements)
            .unwrap();
        other.move_element_here(&copy).unwrap();
        let moved = model.identifiables_with_shortname("renamed");
        assert_eq!(moved.len(), 1, "the moved element is still found by name");
        assert_eq!(moved[0].path().unwrap(), "/Pkg/Other/renamed");
        assert_eq!(model.verify_reference_caches(), Ok(()));

        // remove -> remove_identifiable
        other.remove_sub_element(copy).unwrap();
        assert!(model.identifiables_with_shortname("renamed").is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn references_under_path_prefix() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <SYSTEM><SHORT-NAME>System</SHORT-NAME>
                    <FIBEX-ELEMENTS>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE">/Pkg/Sub/Ecu</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE">/Pkg/Sub2/Ecu</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                    </FIBEX-ELEMENTS>
                </SYSTEM>
            </ELEMENTS>
        </AR-PACKAGE>
        <AR-PACKAGE><SHORT-NAME>Sub</SHORT-NAME>
            <ELEMENTS><ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE></ELEMENTS>
        </AR-PACKAGE>
        <AR-PACKAGE><SHORT-NAME>Sub2</SHORT-NAME>
            <ELEMENTS><ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE></ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();

        // the prefix selects the subtree; "/Pkg/Sub2/Ecu" is not inside "/Pkg/Sub"
        let under_sub = model.references_under_path_prefix("/Pkg/Sub");
        assert_eq!(under_sub.len(), 1);
        assert_eq!(
            under_sub[0].character_data().unwrap().string_value().unwrap(),
            "/Pkg/Sub/Ecu"
        );
        assert_eq!(model.references_under_path_prefix("/Pkg/Sub2").len(), 1);
        // the whole package contains both
        assert_eq!(model.references_under_path_prefix("/Pkg").len(), 2);
        // a path component boundary is respected: "/Pkg/Su" is not a prefix of "/Pkg/Sub"
        assert!(model.references_under_path_prefix("/Pkg/Su").is_empty());
        assert!(model.references_under_path_prefix("/Nonexistent").is_empty());

        // for an exact reference target the bulk query agrees with get_references_to. A prefix that
        // is only an ancestor of the targets selects them, so the two differ for "/Pkg"
        for path in ["/Pkg/Sub/Ecu", "/Pkg/Sub2/Ecu"] {
            let bulk: Vec<Element> = model.references_under_path_prefix(path);
            let single: Vec<Element> = model
                .get_references_to(path)
                .iter()
                .filter_map(WeakElement::upgrade)
                .collect();
            assert_eq!(bulk, single, "mismatch for {path}");
            assert_eq!(bulk.len(), 1);
        }
    }

    #[test]
    fn check_references() {
        const FILEBUF: &str = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
            <ELEMENTS>
                <SYSTEM><SHORT-NAME>System</SHORT-NAME>
                    <FIBEX-ELEMENTS>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE">/Pkg/EcuInstance</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="I-SIGNAL-I-PDU">/Some/Invalid/Path</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                        <FIBEX-ELEMENT-REF-CONDITIONAL>
                            <FIBEX-ELEMENT-REF DEST="I-SIGNAL">/Pkg/System</FIBEX-ELEMENT-REF>
                        </FIBEX-ELEMENT-REF-CONDITIONAL>
                    </FIBEX-ELEMENTS>
                </SYSTEM>
                <ECU-INSTANCE><SHORT-NAME>EcuInstance</SHORT-NAME></ECU-INSTANCE>
            </ELEMENTS>
        </AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#;
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF.as_bytes(), "test", true).unwrap();
        let el_system = model.get_element_by_path("/Pkg/System").unwrap();
        let el_fibex_elements = el_system.get_sub_element(ElementName::FibexElements).unwrap();
        let el_fibex_element_ref = el_fibex_elements
            .create_sub_element(ElementName::FibexElementRefConditional)
            .and_then(|ferc| ferc.create_sub_element(ElementName::FibexElementRef))
            .unwrap();
        el_fibex_element_ref.set_character_data("/Pkg/System").unwrap();
        // the test data contains 4 references to 3 distinct items:
        // - to /Pkg/EcuInstance (VALID)
        // - to /Some/Invalid/Path (INVALID)
        // - to /Pkg/System, with DEST=I-SIGNAL (INVALID)
        // - to /Pkg/System, without DEST (INVALID)
        assert_eq!(model.0.read().reference_origins.len(), 3);

        // confirm that the first reference, to EcuInstance, is valid
        let el_fbx_ref1 = el_fibex_elements
            .get_sub_element_at(0)
            .and_then(|ferc| ferc.get_sub_element(ElementName::FibexElementRef))
            .unwrap();
        assert_eq!(
            el_fbx_ref1.get_reference_target().unwrap().element_name(),
            ElementName::EcuInstance
        );

        let invalid_refs = model
            .check_references()
            .iter()
            .filter_map(WeakElement::upgrade)
            .collect::<Vec<_>>();
        assert_eq!(invalid_refs.len(), 3);
        let ref0 = &invalid_refs[0];
        assert_eq!(ref0.element_name(), ElementName::FibexElementRef);
        let refpath = ref0.character_data().and_then(|cdata| cdata.string_value()).unwrap();
        // there is no defined order in which the references will be checked, so any of the three broken refs could be returned first
        assert!(refpath == "/Pkg/System" || refpath == "/Some/Invalid/Path");

        model.get_element_by_path("/Pkg/EcuInstance").unwrap();
        let refs = model.get_references_to("/Pkg/EcuInstance");
        assert_eq!(refs.len(), 1);
        let refs = model.get_references_to("nonexistent");
        assert!(refs.is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn serialize_files() {
        let model = AutosarModel::default();
        let file1 = model.create_file("filename1", AutosarVersion::Autosar_00042).unwrap();
        let file2 = model.create_file("filename2", AutosarVersion::Autosar_00042).unwrap();

        let result = model.serialize_files();
        assert_eq!(result.len(), 2);
        assert_eq!(
            result.get(&PathBuf::from("filename1")).unwrap(),
            &file1.serialize().unwrap()
        );
        assert_eq!(
            result.get(&PathBuf::from("filename2")).unwrap(),
            &file2.serialize().unwrap()
        );
    }

    #[test]
    fn duplicate() {
        let model = AutosarModel::new();
        let file1 = model.create_file("filename1", AutosarVersion::Autosar_00042).unwrap();
        let file2 = model.create_file("filename2", AutosarVersion::Autosar_00042).unwrap();
        let el_ar_packages = model
            .root_element()
            .create_sub_element(ElementName::ArPackages)
            .unwrap();
        let el_pkg1 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "pkg1")
            .unwrap();
        let el_pkg2 = el_ar_packages
            .create_named_sub_element(ElementName::ArPackage, "pkg2")
            .unwrap();

        assert_eq!(el_ar_packages.file_membership().unwrap().1.len(), 2);
        el_pkg1.remove_from_file(&file2).unwrap();
        assert_eq!(el_pkg1.file_membership().unwrap().1.len(), 1);
        el_pkg2.remove_from_file(&file1).unwrap();
        assert_eq!(el_pkg2.file_membership().unwrap().1.len(), 1);

        let model2 = model.duplicate().unwrap();
        assert_eq!(model2.files().count(), 2);
        let mut files_iter = model2.files();
        // get the files out of model 2
        let mut model2_file1 = files_iter.next().unwrap();
        let mut model2_file2 = files_iter.next().unwrap();
        // the iterator could return the files in any order - make sure that model2_file1 corresponds to file1
        if model2_file1.filename() != file1.filename() {
            std::mem::swap(&mut model2_file1, &mut model2_file2);
        }

        assert_eq!(file1.filename(), model2_file1.filename());
        assert_eq!(file2.filename(), model2_file2.filename());
        assert_eq!(file1.serialize().unwrap(), model2_file1.serialize().unwrap());
        assert_eq!(file2.serialize().unwrap(), model2_file2.serialize().unwrap());
    }

    /// duplicate() must copy a model with files of different versions exactly
    ///
    /// Each file of the copy keeps the version of the original file, so every element is valid
    /// where it ends up. Filtering the copy by any single version loses data: the oldest version
    /// of the model does not permit the elements which were added in later versions, and the
    /// newest version does not permit the elements which were removed again before it.
    #[test]
    fn duplicate_mixed_versions() {
        // PORT-BLUEPRINT exists in AUTOSAR 4.0.1, but was removed in later versions
        const FILEBUF_OLD: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-0-1.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>PkgOld</SHORT-NAME><ELEMENTS>
          <PORT-BLUEPRINT><SHORT-NAME>Blueprint</SHORT-NAME></PORT-BLUEPRINT>
        </ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        // ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE was only added after 4.0.1
        const FILEBUF_NEW: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>PkgNew</SHORT-NAME><ELEMENTS>
          <ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE><SHORT-NAME>Adaptive</SHORT-NAME></ADAPTIVE-APPLICATION-SW-COMPONENT-TYPE>
        </ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();

        let model = AutosarModel::new();
        let (file_old, _) = model.load_buffer(FILEBUF_OLD, "old.arxml", true).unwrap();
        let (file_new, _) = model.load_buffer(FILEBUF_NEW, "new.arxml", true).unwrap();

        let copy = model.duplicate().unwrap();

        // an element which only exists in versions older than the newest file must not be dropped
        assert!(copy.get_element_by_path("/PkgOld/Blueprint").is_some());
        // an element which only exists in versions newer than the oldest file must not be dropped
        assert!(copy.get_element_by_path("/PkgNew/Adaptive").is_some());
        assert_eq!(model.elements_dfs().count(), copy.elements_dfs().count());

        // The file membership must end up on the same elements as in the original. If any element
        // were dropped from the copy, then the file membership would be shifted onto the wrong
        // elements, and the files would no longer contain the same data.
        assert_eq!(copy.files().count(), 2);
        let copy_old = copy.files().find(|f| f.filename() == file_old.filename()).unwrap();
        let copy_new = copy.files().find(|f| f.filename() == file_new.filename()).unwrap();
        assert_eq!(copy_old.version(), AutosarVersion::Autosar_4_0_1);
        assert_eq!(copy_new.version(), AutosarVersion::Autosar_00050);
        assert_eq!(copy_old.serialize().unwrap(), file_old.serialize().unwrap());
        assert_eq!(copy_new.serialize().unwrap(), file_new.serialize().unwrap());
    }

    #[test]
    fn write() {
        let model = AutosarModel::default();
        // write an empty model, it does nothing since there are no files
        model.write().unwrap();

        let dir = tempdir().unwrap();
        let filename = dir.path().with_file_name("new.arxml");
        model.create_file(&filename, AutosarVersion::LATEST).unwrap();
        model.write().unwrap();
        assert!(filename.exists());

        let filename = PathBuf::from("nonexistent/dir/some_file.arxml");
        let model = AutosarModel::default();
        // creating an ArxmlFile with a non-existent directory is not an error
        model.create_file(&filename, AutosarVersion::LATEST).unwrap();
        // the write operation will fail, because the directory does not exist
        let result = model.write();
        assert!(result.is_err());
    }

    #[test]
    fn traits() {
        // AutosarModel: Debug, Clone, Hash
        let model = AutosarModel::new();
        let model_cloned = model.clone();
        assert_eq!(model, model_cloned);
        assert_eq!(format!("{model:#?}"), format!("{model_cloned:#?}"));
        #[allow(clippy::mutable_key_type)]
        let mut hashset = HashSet::<AutosarModel>::new();
        hashset.insert(model);
        let inserted = hashset.insert(model_cloned);
        assert!(!inserted);

        // CharacterData
        let cdata = CharacterData::String("x".to_string());
        let cdata2 = cdata.clone();
        assert_eq!(cdata, cdata2);
        assert_eq!(format!("{cdata:#?}"), format!("{cdata2:#?}"));

        // ContentType
        let ct: ContentType = ContentType::Elements;
        let ct2 = ct;
        assert_eq!(ct, ct2);
        assert_eq!(format!("{ct:#?}"), format!("{ct2:#?}"));
    }

    #[test]
    fn elements_dfs_with_max_depth() {
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES>
          <AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
            <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
              <SHORT-NAME>BswModuleValues</SHORT-NAME>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_B</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_C</DEFINITION-REF>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
          </ELEMENTS></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_B</SHORT-NAME></AR-PACKAGE>
          <AR-PACKAGE><SHORT-NAME>Pkg_C</SHORT-NAME></AR-PACKAGE>
        </AR-PACKAGES></AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILEBUF, "test1", true).unwrap();
        let all_count = model.elements_dfs().count();
        let lvl2_count = model.elements_dfs_with_max_depth(2).count();
        assert!(all_count > lvl2_count);
        for elem in model.elements_dfs_with_max_depth(2) {
            assert!(elem.0 <= 2);
        }
    }

    #[test]
    fn model_merge() {
        // from github issue #24; test files provided by FlTr
        const FILE_A: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>EcucModuleConfigurationValuess</SHORT-NAME>
      <ELEMENTS>
        <ECUC-MODULE-CONFIGURATION-VALUES>
          <SHORT-NAME>A</SHORT-NAME>
          <DEFINITION-REF DEST="ECUC-MODULE-DEF">/AUTOSAR_A</DEFINITION-REF>
          <CONTAINERS>
            <ECUC-CONTAINER-VALUE>
              <SHORT-NAME>AB</SHORT-NAME>
              <DEFINITION-REF DEST="ECUC-PARAM-CONF-CONTAINER-DEF">/AUTOSAR_A/B</DEFINITION-REF>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-FLOAT-PARAM-DEF">/AUTOSAR_A/B/D</DEFINITION-REF>
                  <VALUE>0.01</VALUE>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-TEXTUAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-ENUMERATION-PARAM-DEF">/AUTOSAR_A/B/E</DEFINITION-REF>
                  <VALUE>ABC42</VALUE>
                </ECUC-TEXTUAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE>
          </CONTAINERS>
        </ECUC-MODULE-CONFIGURATION-VALUES>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>
        "#;

        const FILE_B: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xmlns="http://autosar.org/schema/r4.0"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00048.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>EcucModuleConfigurationValuess</SHORT-NAME>
      <ELEMENTS>
        <ECUC-MODULE-CONFIGURATION-VALUES>
          <SHORT-NAME>A</SHORT-NAME>
          <DEFINITION-REF DEST="ECUC-MODULE-DEF">/AUTOSAR_A</DEFINITION-REF>
          <CONTAINERS>
            <ECUC-CONTAINER-VALUE>
              <SHORT-NAME>AB</SHORT-NAME>
              <DEFINITION-REF DEST="ECUC-PARAM-CONF-CONTAINER-DEF">/AUTOSAR_A/B</DEFINITION-REF>
              <PARAMETER-VALUES>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-INTEGER-PARAM-DEF">/AUTOSAR_A/B/C</DEFINITION-REF>
                  <VALUE>0</VALUE>
                </ECUC-NUMERICAL-PARAM-VALUE>
                <ECUC-NUMERICAL-PARAM-VALUE>
                  <DEFINITION-REF DEST="ECUC-FLOAT-PARAM-DEF">/AUTOSAR_A/B/D</DEFINITION-REF>
                  <VALUE>0.01</VALUE>
                </ECUC-NUMERICAL-PARAM-VALUE>
              </PARAMETER-VALUES>
            </ECUC-CONTAINER-VALUE>
          </CONTAINERS>
        </ECUC-MODULE-CONFIGURATION-VALUES>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

        // loading these files must not hang, regardless of the order
        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILE_A, "file_a", true).unwrap();
        let (_, _) = model.load_buffer(FILE_B, "file_b", true).unwrap();
        // sort the model to ensure that the serialized text is the same
        model.sort();
        let model_txt = model.root_element().serialize();

        let model2 = AutosarModel::new();
        let (_, _) = model2.load_buffer(FILE_B, "file_b", true).unwrap();
        let (_, _) = model2.load_buffer(FILE_A, "file_a", true).unwrap();
        // sort the model to ensure that the serialized text is the same
        model2.sort();
        let model2_txt = model2.root_element().serialize();

        assert_eq!(model_txt, model2_txt);
    }

    #[test]
    fn model_merge_2() {
        // regression test for github issue #30
        const FILEBUF1: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-3-0.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>BSWMD_Package</SHORT-NAME>
      <ELEMENTS>
        <BSW-MODULE-DESCRIPTION>
          <SHORT-NAME>BSWMD</SHORT-NAME>
          <IMPLEMENTED-ENTRYS>
            <BSW-MODULE-ENTRY-REF-CONDITIONAL>
              <BSW-MODULE-ENTRY-REF DEST="BSW-MODULE-ENTRY">/path/to/entry_A0</BSW-MODULE-ENTRY-REF>
            </BSW-MODULE-ENTRY-REF-CONDITIONAL>
          </IMPLEMENTED-ENTRYS>
        </BSW-MODULE-DESCRIPTION>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;
        const FILEBUF2: &[u8] = br#"<?xml version="1.0" encoding="UTF-8"?>
<AUTOSAR xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xmlns="http://autosar.org/schema/r4.0" xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_4-3-0.xsd">
  <AR-PACKAGES>
    <AR-PACKAGE>
      <SHORT-NAME>BSWMD_Package</SHORT-NAME>
      <ELEMENTS>
        <BSW-MODULE-DESCRIPTION>
          <SHORT-NAME>BSWMD</SHORT-NAME>
          <IMPLEMENTED-ENTRYS>
            <BSW-MODULE-ENTRY-REF-CONDITIONAL>
              <BSW-MODULE-ENTRY-REF DEST="BSW-MODULE-ENTRY">/path/to/entry_B0</BSW-MODULE-ENTRY-REF>
            </BSW-MODULE-ENTRY-REF-CONDITIONAL>
            <BSW-MODULE-ENTRY-REF-CONDITIONAL>
              <BSW-MODULE-ENTRY-REF DEST="BSW-MODULE-ENTRY">/path/to/entry_B1</BSW-MODULE-ENTRY-REF>
            </BSW-MODULE-ENTRY-REF-CONDITIONAL>
          </IMPLEMENTED-ENTRYS>
        </BSW-MODULE-DESCRIPTION>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#;

        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILEBUF1, "file1", true).unwrap();
        let (_, _) = model.load_buffer(FILEBUF2, "file2", true).unwrap();

        let a0_refs = model.get_references_to("/path/to/entry_A0");
        assert!(a0_refs.len() == 1);
        assert!(a0_refs[0].upgrade().is_some());
        let b0_refs = model.get_references_to("/path/to/entry_B0");
        assert!(b0_refs.len() == 1);
        assert!(b0_refs[0].upgrade().is_some());
        let b1_refs = model.get_references_to("/path/to/entry_B1");
        assert!(b1_refs.len() == 1);
        assert!(b1_refs[0].upgrade().is_some());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn model_merge_3() {
        const FILEBUF1: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
  <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
    <SHORT-NAME>BswModuleValues</SHORT-NAME>
    <PARAMETER-VALUES>
      <ECUC-NUMERICAL-PARAM-VALUE>
        <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
        <VALUE>1</VALUE>
      </ECUC-NUMERICAL-PARAM-VALUE>
    </PARAMETER-VALUES>
  </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
</ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#;
        const FILEBUF2: &[u8] = br#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
<AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg_A</SHORT-NAME><ELEMENTS>
  <ECUC-MODULE-CONFIGURATION-VALUES><SHORT-NAME>BswModule</SHORT-NAME><CONTAINERS><ECUC-CONTAINER-VALUE>
    <SHORT-NAME>BswModuleValues</SHORT-NAME>
    <PARAMETER-VALUES>
      <ECUC-NUMERICAL-PARAM-VALUE>
        <DEFINITION-REF DEST="ECUC-BOOLEAN-PARAM-DEF">/REF_A</DEFINITION-REF>
        <ANNOTATIONS/>
        <VALUE>2</VALUE>
      </ECUC-NUMERICAL-PARAM-VALUE>
    </PARAMETER-VALUES>
  </ECUC-CONTAINER-VALUE></CONTAINERS></ECUC-MODULE-CONFIGURATION-VALUES>
</ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#;

        // the merge of the two filee has a conflict: the same ECUC-NUMERICAL-PARAM-VALUE is defined in both files, but with different values.
        // Since both params have the same defintion ref they must be merged and cannot be imported side-by-side, which is impossible here.
        // This means that loading the second file must fail.

        let model = AutosarModel::new();
        let (_, _) = model.load_buffer(FILEBUF1, "file1", true).unwrap();
        let result = model.load_buffer(FILEBUF2, "file2", true);
        assert!(result.is_err());
    }

    #[test]
    fn data_merge_after_insertion_conflict() {
        // Both files contain a PORT-API-OPTION. PORT-API-OPTION is neither identifiable nor
        // does it have a DEFINITION-REF, so the two elements are paired for merging. The merge
        // is not possible, because PORT-REF may only appear once and the two files disagree
        // about its value. PORT-API-OPTIONS is splittable, so the element from the second file
        // is added as an additional sub element instead.
        // ENABLE-TAKE-ADDRESS and PORT-ARG-VALUES are unique to the second file and are already
        // merged into the element of the first file when the conflict is detected; the recovery
        // must undo this, otherwise these elements end up in both PORT-API-OPTIONs at once.
        const FILEBUF1: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME><ELEMENTS>
          <SERVICE-SW-COMPONENT-TYPE><SHORT-NAME>Swc</SHORT-NAME>
            <INTERNAL-BEHAVIORS><SWC-INTERNAL-BEHAVIOR><SHORT-NAME>Behavior</SHORT-NAME>
              <PORT-API-OPTIONS>
                <PORT-API-OPTION>
                  <PORT-REF DEST="P-PORT-PROTOTYPE">/Pkg/Swc/PortA</PORT-REF>
                </PORT-API-OPTION>
              </PORT-API-OPTIONS>
            </SWC-INTERNAL-BEHAVIOR></INTERNAL-BEHAVIORS>
          </SERVICE-SW-COMPONENT-TYPE>
        </ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();
        const FILEBUF2: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
        <AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
        <AR-PACKAGES><AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME><ELEMENTS>
          <SERVICE-SW-COMPONENT-TYPE><SHORT-NAME>Swc</SHORT-NAME>
            <INTERNAL-BEHAVIORS><SWC-INTERNAL-BEHAVIOR><SHORT-NAME>Behavior</SHORT-NAME>
              <PORT-API-OPTIONS>
                <PORT-API-OPTION>
                  <ENABLE-TAKE-ADDRESS>true</ENABLE-TAKE-ADDRESS>
                  <PORT-ARG-VALUES>
                    <PORT-DEFINED-ARGUMENT-VALUE>
                      <VALUE-TYPE-TREF DEST="IMPLEMENTATION-DATA-TYPE">/Pkg/SomeType</VALUE-TYPE-TREF>
                    </PORT-DEFINED-ARGUMENT-VALUE>
                  </PORT-ARG-VALUES>
                  <PORT-REF DEST="P-PORT-PROTOTYPE">/Pkg/Swc/PortB</PORT-REF>
                </PORT-API-OPTION>
              </PORT-API-OPTIONS>
            </SWC-INTERNAL-BEHAVIOR></INTERNAL-BEHAVIORS>
          </SERVICE-SW-COMPONENT-TYPE>
        </ELEMENTS></AR-PACKAGE></AR-PACKAGES></AUTOSAR>"#.as_bytes();

        // serialize each file on its own; merging must not change the content of either file
        let single_model = AutosarModel::new();
        let (single_file1, _) = single_model.load_buffer(FILEBUF1, "file1.arxml", true).unwrap();
        let file1_txt = single_file1.serialize().unwrap();
        let single_model = AutosarModel::new();
        let (single_file2, _) = single_model.load_buffer(FILEBUF2, "file2.arxml", true).unwrap();
        let file2_txt = single_file2.serialize().unwrap();

        let model = AutosarModel::new();
        let (file1, _) = model.load_buffer(FILEBUF1, "file1.arxml", true).unwrap();
        let (file2, _) = model.load_buffer(FILEBUF2, "file2.arxml", true).unwrap();

        let el_port_api_options = model
            .get_element_by_path("/Pkg/Swc/Behavior")
            .and_then(|behavior| behavior.get_sub_element(ElementName::PortApiOptions))
            .unwrap();
        let options: Vec<Element> = el_port_api_options.sub_elements().collect();
        // the two options could not be merged, so each of them exists on its own, one per file
        assert_eq!(options.len(), 2);
        assert_eq!(options[0].file_membership().unwrap().1.len(), 1);
        assert_eq!(options[1].file_membership().unwrap().1.len(), 1);

        // the sub elements which are unique to file2 were returned to the option of file2 by the
        // rollback, so each of them is present exactly once in the model
        for element_name in [ElementName::EnableTakeAddress, ElementName::PortArgValues] {
            let count = model
                .elements_dfs()
                .filter(|(_, elem)| elem.element_name() == element_name)
                .count();
            assert_eq!(count, 1, "{element_name} exists {count} times, expected 1");
        }
        // each option contains only the sub elements of its own file
        let sub_elements_0: Vec<ElementName> = options[0].sub_elements().map(|e| e.element_name()).collect();
        assert_eq!(sub_elements_0, vec![ElementName::PortRef]);
        let sub_elements_1: Vec<ElementName> = options[1].sub_elements().map(|e| e.element_name()).collect();
        assert_eq!(
            sub_elements_1,
            vec![
                ElementName::EnableTakeAddress,
                ElementName::PortArgValues,
                ElementName::PortRef
            ]
        );

        // the content of both files is unchanged by the merge
        assert_eq!(file1.serialize().unwrap(), file1_txt);
        assert_eq!(file2.serialize().unwrap(), file2_txt);

        // load the files in the opposite order: the content of both files must still be unchanged
        let model = AutosarModel::new();
        let (file2, _) = model.load_buffer(FILEBUF2, "file2.arxml", true).unwrap();
        let (file1, _) = model.load_buffer(FILEBUF1, "file1.arxml", true).unwrap();
        assert_eq!(file1.serialize().unwrap(), file1_txt);
        assert_eq!(file2.serialize().unwrap(), file2_txt);
    }

    // a model with three reference bases: two in the outer package "/BasesPkg", and one in
    // "/BasesPkg/SubPackage" whose own PACKAGE-REF is relative to the base "BaseA"
    const FILEBUF1_COMPLEX_BASES: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>BasesPkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>BaseA</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/ContentPkg</PACKAGE-REF>
        </REFERENCE-BASE>
        <REFERENCE-BASE>
          <SHORT-LABEL>BaseC</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/ContentPkg2</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
      <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>SubPackage</SHORT-NAME>
          <REFERENCE-BASES>
            <REFERENCE-BASE>
              <SHORT-LABEL>BaseB</SHORT-LABEL>
              <PACKAGE-REF DEST="AR-PACKAGE" BASE="BaseA">SubPackage</PACKAGE-REF>
            </REFERENCE-BASE>
          </REFERENCE-BASES>
          <ELEMENTS>
            <SYSTEM><SHORT-NAME>System</SHORT-NAME>
              <FIBEX-ELEMENTS>
                <FIBEX-ELEMENT-REF-CONDITIONAL>
                  <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="BaseB">Ecu</FIBEX-ELEMENT-REF>
                </FIBEX-ELEMENT-REF-CONDITIONAL>
                <FIBEX-ELEMENT-REF-CONDITIONAL>
                  <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="BaseC">Ecu</FIBEX-ELEMENT-REF>
                </FIBEX-ELEMENT-REF-CONDITIONAL>
              </FIBEX-ELEMENTS>
            </SYSTEM>
          </ELEMENTS>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
    <AR-PACKAGE><SHORT-NAME>ContentPkg</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>SubPackage</SHORT-NAME>
          <ELEMENTS>
            <ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE>
          </ELEMENTS>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
    <AR-PACKAGE><SHORT-NAME>ContentPkg2</SHORT-NAME>
      <ELEMENTS>
        <ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();

    /// a way of corrupting the caches: a description, and the corruption itself. The returned
    /// elements are kept alive by the caller, so that entries referring to them are not skipped as
    /// dead weak references.
    type CacheCorruption = (&'static str, fn(&AutosarModel) -> Vec<Element>);

    // the PACKAGE-REF of "BaseA" in FILEBUF1_COMPLEX_BASES, which is an absolute reference
    fn get_absolute_package_ref(model: &AutosarModel) -> Element {
        model
            .get_element_by_path("/BasesPkg")
            .and_then(|el_package| el_package.get_sub_element(ElementName::ReferenceBases))
            .and_then(|el_bases| el_bases.get_sub_element_at(0))
            .and_then(|el_base| el_base.get_sub_element(ElementName::PackageRef))
            .unwrap()
    }

    #[test]
    fn verify_reference_caches_detects_inconsistency() {
        // verify_reference_caches() is asserted by many tests, so it must be able to fail: this test
        // corrupts the caches in each of the ways that real bugs have corrupted them, and requires
        // every one of them to be reported.
        let corruptions: [CacheCorruption; 8] = [
            // a relative reference which is missing from the reverse index entirely
            ("missing entry", |model| {
                model.0.write().relative_references.clear();
                vec![]
            }),
            // an absolute reference which is missing from the index
            ("missing absolute entry", |model| {
                let el_package_ref = get_absolute_package_ref(model);
                let target = el_package_ref.character_data().unwrap().string_value().unwrap();
                model.0.write().reference_origins.remove(&target);
                vec![el_package_ref]
            }),
            // an absolute reference which is registered as a relative one
            ("absolute reference in the reverse index", |model| {
                let el_package_ref = get_absolute_package_ref(model);
                model
                    .0
                    .write()
                    .relative_references
                    .insert(el_package_ref.downgrade(), None);
                vec![el_package_ref]
            }),
            // an absolute reference registered under a key which is not its character data
            ("wrong key", |model| {
                let el_package_ref = get_absolute_package_ref(model);
                model
                    .0
                    .write()
                    .reference_origins
                    .entry("/Wrong".to_string())
                    .or_default()
                    .push(el_package_ref.downgrade());
                vec![el_package_ref]
            }),
            // an element in the reverse index which is not a reference at all
            ("reverse index entry which is not a reference", |model| {
                let el_system = model.get_element_by_path("/BasesPkg/SubPackage/System").unwrap();
                model.0.write().relative_references.insert(el_system.downgrade(), None);
                vec![el_system]
            }),
            // a relative reference whose reverse index entry names the wrong target path
            ("stale target path", |model| {
                let el_ref = get_complex_bases_refs(model).0;
                model
                    .0
                    .write()
                    .relative_references
                    .insert(el_ref.downgrade(), Some("/Stale".to_string()));
                vec![el_ref]
            }),
            // a reference which is in the reverse index, but not in the bucket it names
            ("missing bucket entry", |model| {
                let el_ref = get_complex_bases_refs(model).0;
                let mut model_locked = model.0.write();
                let target = model_locked
                    .relative_references
                    .get(&el_ref.downgrade())
                    .cloned()
                    .flatten()
                    .unwrap();
                model_locked.reference_origins.remove(&target);
                vec![el_ref]
            }),
            // an entry for an element which has been removed from the tree
            ("removed element", |model| {
                let el_ref = get_complex_bases_refs(model).0;
                let el_parent = el_ref.parent().unwrap().unwrap();
                model
                    .0
                    .write()
                    .reference_origins
                    .insert("/detached".to_string(), vec![el_ref.downgrade()]);
                el_parent.remove_sub_element(el_ref.clone()).unwrap();
                vec![el_ref]
            }),
        ];

        for (description, corrupt) in corruptions {
            let model = AutosarModel::new();
            model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true).unwrap();
            assert_eq!(model.verify_reference_caches(), Ok(()));
            let _keep_alive = corrupt(&model);
            assert!(
                model.verify_reference_caches().is_err(),
                "verify_reference_caches() did not detect: {description}"
            );
        }
    }

    #[test]
    fn complex_reference_bases() {
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true);
        assert!(result.is_ok());

        let el_system = model.get_element_by_path("/BasesPkg/SubPackage/System").unwrap();
        let el_fibex_elements = el_system.get_sub_element(ElementName::FibexElements).unwrap();
        let el_fibex_element_ref = el_fibex_elements
            .get_sub_element_at(0)
            .and_then(|ferc| ferc.get_sub_element(ElementName::FibexElementRef))
            .unwrap();

        // check that we can resolve the reference across multiple levels of reference bases
        let el_ecu = el_fibex_element_ref.get_reference_target().unwrap();
        assert_eq!(el_ecu.element_name(), ElementName::EcuInstance);

        // check that the reference origins are correct
        let origins = model.get_references_to(&el_ecu.path().unwrap());
        assert_eq!(origins.len(), 1);
        let origin = origins[0].upgrade().unwrap();
        assert_eq!(origin.element_name(), ElementName::FibexElementRef);

        let origins2 = model.get_references_to("/ContentPkg2/Ecu");
        assert_eq!(origins2.len(), 1);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    // get the two FIBEX-ELEMENT-REFs of FILEBUF1_COMPLEX_BASES: the first one uses the reference
    // base "BaseB" (= /ContentPkg/SubPackage), the second one uses "BaseC" (= /ContentPkg2)
    fn get_complex_bases_refs(model: &AutosarModel) -> (Element, Element) {
        let el_fibex_elements = model
            .get_element_by_path("/BasesPkg/SubPackage/System")
            .and_then(|el_system| el_system.get_sub_element(ElementName::FibexElements))
            .unwrap();
        let mut refs = el_fibex_elements
            .sub_elements()
            .filter_map(|ferc| ferc.get_sub_element(ElementName::FibexElementRef));
        (refs.next().unwrap(), refs.next().unwrap())
    }

    #[test]
    fn rename_relative_reference_target() {
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true);
        assert!(result.is_ok());
        let (el_ref_base_b, el_ref_base_c) = get_complex_bases_refs(&model);

        // rename /ContentPkg/SubPackage/Ecu; the FibexElementRef refers to it using the reference base "BaseB" (/ContentPkg/SubPackage)
        let el_ecu = model.get_element_by_path("/ContentPkg/SubPackage/Ecu").unwrap();
        el_ecu.set_item_name("RenamedEcu").unwrap();
        assert_eq!(
            el_ref_base_b.character_data().unwrap().string_value().unwrap(),
            "RenamedEcu"
        );
        assert_eq!(el_ref_base_b.get_reference_target().unwrap(), el_ecu);
        // the other reference uses a different base and points at a different element: it is unchanged
        assert_eq!(el_ref_base_c.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(
            el_ref_base_c.get_reference_target().unwrap(),
            model.get_element_by_path("/ContentPkg2/Ecu").unwrap()
        );

        // the cache must follow the renaming as well: both references are still known, each under
        // the (new) relative path which is the character data of the referring element
        assert_eq!(model.get_references_to("/ContentPkg/SubPackage/RenamedEcu").len(), 1);
        assert_eq!(model.get_references_to("/ContentPkg/SubPackage/Ecu").len(), 0);
        assert_eq!(model.get_references_to("/ContentPkg2/Ecu").len(), 1);
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_relative_reference_target_repeatedly() {
        // renaming the target of a relative reference must leave the caches in a state which allows
        // the next rename to find the reference again
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true);
        assert!(result.is_ok());
        let (el_ref_base_b, _) = get_complex_bases_refs(&model);

        let el_ecu = model.get_element_by_path("/ContentPkg/SubPackage/Ecu").unwrap();
        for new_name in ["Ecu2", "Ecu3", "Ecu4"] {
            el_ecu.set_item_name(new_name).unwrap();
            assert_eq!(
                el_ref_base_b.character_data().unwrap().string_value().unwrap(),
                new_name
            );
            assert_eq!(el_ref_base_b.get_reference_target().unwrap(), el_ecu);
            assert_eq!(model.get_references_to(&el_ecu.path().unwrap()).len(), 1);
            assert!(model.check_references().is_empty());
        }
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    // a model with a relative reference whose relative path consists of more than one path
    // component: BASE="BaseA" resolves to /ContentPkg, so "SubPackage/Ecu" leads to
    // /ContentPkg/SubPackage/Ecu
    const FILEBUF_MULTI_COMPONENT_RELATIVE_REF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>BasesPkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>BaseA</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/ContentPkg</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
      <ELEMENTS>
        <SYSTEM><SHORT-NAME>System</SHORT-NAME>
          <FIBEX-ELEMENTS>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="BaseA">SubPackage/Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
          </FIBEX-ELEMENTS>
        </SYSTEM>
      </ELEMENTS>
    </AR-PACKAGE>
    <AR-PACKAGE><SHORT-NAME>ContentPkg</SHORT-NAME>
      <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>SubPackage</SHORT-NAME>
          <ELEMENTS>
            <ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE>
          </ELEMENTS>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();

    // get the FIBEX-ELEMENT-REF of FILEBUF_MULTI_COMPONENT_RELATIVE_REF
    fn get_multi_component_relative_ref(model: &AutosarModel) -> Element {
        model
            .get_element_by_path("/BasesPkg/System")
            .and_then(|el_system| el_system.get_sub_element(ElementName::FibexElements))
            .and_then(|el_fibex_elements| el_fibex_elements.get_sub_element_at(0))
            .and_then(|ferc| ferc.get_sub_element(ElementName::FibexElementRef))
            .unwrap()
    }

    #[test]
    fn rename_package_inside_relative_reference_path() {
        // a relative path can consist of several path components, so a renamed element can also be
        // an ancestor of the reference target
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF_MULTI_COMPONENT_RELATIVE_REF, "test", true);
        assert!(result.is_ok());
        let el_ref = get_multi_component_relative_ref(&model);
        let el_ecu = el_ref.get_reference_target().unwrap();

        // rename the intermediate package, which is part of the relative path
        let el_subpackage = model.get_element_by_path("/ContentPkg/SubPackage").unwrap();
        el_subpackage.set_item_name("SubPkgX").unwrap();
        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "SubPkgX/Ecu");
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert_eq!(model.get_references_to("/ContentPkg/SubPkgX/Ecu").len(), 1);
        assert!(model.check_references().is_empty());

        // renaming the package which the reference base points to does not change the relative path
        let el_content_package = model.get_element_by_path("/ContentPkg").unwrap();
        el_content_package.set_item_name("ContentPkgX").unwrap();
        assert_eq!(el_ref.character_data().unwrap().string_value().unwrap(), "SubPkgX/Ecu");
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert_eq!(model.get_references_to("/ContentPkgX/SubPkgX/Ecu").len(), 1);
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_relative_reference_target() {
        // moving the target of a relative reference inside the subtree of its reference base changes
        // the relative path, and the reference must follow
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF_MULTI_COMPONENT_RELATIVE_REF, "test", true);
        assert!(result.is_ok());
        let el_ref = get_multi_component_relative_ref(&model);
        let el_ecu = el_ref.get_reference_target().unwrap();

        // create /ContentPkg/SubPackage2 and move the reference target there
        let el_elements2 = model
            .get_element_by_path("/ContentPkg")
            .and_then(|el_package| el_package.get_sub_element(ElementName::ArPackages))
            .and_then(|el_packages| {
                el_packages
                    .create_named_sub_element(ElementName::ArPackage, "SubPackage2")
                    .ok()
            })
            .and_then(|el_package2| el_package2.create_sub_element(ElementName::Elements).ok())
            .unwrap();
        el_elements2.move_element_here(&el_ecu).unwrap();

        assert_eq!(el_ecu.path().unwrap(), "/ContentPkg/SubPackage2/Ecu");
        assert_eq!(
            el_ref.character_data().unwrap().string_value().unwrap(),
            "SubPackage2/Ecu"
        );
        assert_eq!(el_ref.get_reference_target().unwrap(), el_ecu);
        assert_eq!(model.get_references_to("/ContentPkg/SubPackage2/Ecu").len(), 1);
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn move_relative_reference_target_out_of_base() {
        // A relative path can only lead to elements inside the subtree of its reference base, so a
        // reference cannot follow a target which is moved out of that subtree. The BASE attribute is
        // never rewritten, so the reference keeps its text and becomes invalid.
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF_MULTI_COMPONENT_RELATIVE_REF, "test", true);
        assert!(result.is_ok());
        let el_ref = get_multi_component_relative_ref(&model);
        let el_ecu = el_ref.get_reference_target().unwrap();

        // /BasesPkg is outside the subtree of the reference base, which is /ContentPkg
        let el_elements = model
            .get_element_by_path("/BasesPkg")
            .and_then(|el_package| el_package.get_sub_element(ElementName::Elements))
            .unwrap();
        el_elements.move_element_here(&el_ecu).unwrap();

        assert_eq!(el_ecu.path().unwrap(), "/BasesPkg/Ecu");
        // the reference could not follow, so it still contains its original path
        assert_eq!(
            el_ref.character_data().unwrap().string_value().unwrap(),
            "SubPackage/Ecu"
        );
        assert!(el_ref.get_reference_target().is_err());
        assert!(model.get_references_to("/BasesPkg/Ecu").is_empty());
        // ... and the now dangling reference is reported
        let broken = model.check_references();
        assert_eq!(broken.len(), 1);
        assert_eq!(broken[0].upgrade().unwrap(), el_ref);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn rename_package_referenced_by_relative_reference_base() {
        // /ContentPkg/SubPackage is named by the PACKAGE-REF of the reference base "BaseB", which is
        // itself relative (BASE="BaseA"). Renaming the package must update the character data of that
        // PACKAGE-REF, otherwise every reference using "BaseB" breaks.
        let model = AutosarModel::new();
        let result = model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true);
        assert!(result.is_ok());
        let (el_ref_base_b, el_ref_base_c) = get_complex_bases_refs(&model);
        let el_ecu = el_ref_base_b.get_reference_target().unwrap();

        let el_subpackage = model.get_element_by_path("/ContentPkg/SubPackage").unwrap();
        el_subpackage.set_item_name("SubPkgX").unwrap();

        let el_package_ref = model
            .get_element_by_path("/BasesPkg/SubPackage")
            .and_then(|el_package| el_package.get_sub_element(ElementName::ReferenceBases))
            .and_then(|el_bases| el_bases.get_sub_element_at(0))
            .and_then(|el_base| el_base.get_sub_element(ElementName::PackageRef))
            .unwrap();
        assert_eq!(
            el_package_ref.character_data().unwrap().string_value().unwrap(),
            "SubPkgX"
        );
        // the chained reference base now resolves to the renamed package
        assert_eq!(
            el_package_ref.resolve_reference_base("BaseB").as_deref(),
            Some("/ContentPkg/SubPkgX")
        );

        // the relative path of the reference is unchanged, but it now resolves through the new base path
        assert_eq!(el_ref_base_b.character_data().unwrap().string_value().unwrap(), "Ecu");
        assert_eq!(el_ref_base_b.get_reference_target().unwrap(), el_ecu);
        assert_eq!(el_ecu.path().unwrap(), "/ContentPkg/SubPkgX/Ecu");
        assert_eq!(
            el_ref_base_c.get_reference_target().unwrap().path().unwrap(),
            "/ContentPkg2/Ecu"
        );
        assert_eq!(model.get_references_to("/ContentPkg/SubPkgX/Ecu").len(), 1);
        assert_eq!(model.get_references_to("/ContentPkg/SubPkgX").len(), 1);
        assert!(model.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn reference_base_scope_respects_path_boundaries() {
        // A reference base is in scope for the package which declares it and for all packages
        // nested inside it. "/Pkg10" is not nested inside "/Pkg1", even though "/Pkg10" starts with
        // the string "/Pkg1", so the base declared by "/Pkg1" must not be usable from "/Pkg10".
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>Pkg1</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>Base</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/Target</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
      <ELEMENTS>
        <SYSTEM><SHORT-NAME>System</SHORT-NAME>
          <FIBEX-ELEMENTS>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="Base">Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
          </FIBEX-ELEMENTS>
        </SYSTEM>
      </ELEMENTS>
      <AR-PACKAGES>
        <AR-PACKAGE><SHORT-NAME>Sub</SHORT-NAME>
          <ELEMENTS>
            <SYSTEM><SHORT-NAME>System</SHORT-NAME>
              <FIBEX-ELEMENTS>
                <FIBEX-ELEMENT-REF-CONDITIONAL>
                  <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="Base">Ecu</FIBEX-ELEMENT-REF>
                </FIBEX-ELEMENT-REF-CONDITIONAL>
              </FIBEX-ELEMENTS>
            </SYSTEM>
          </ELEMENTS>
        </AR-PACKAGE>
      </AR-PACKAGES>
    </AR-PACKAGE>
    <AR-PACKAGE><SHORT-NAME>Pkg10</SHORT-NAME>
      <ELEMENTS>
        <SYSTEM><SHORT-NAME>System</SHORT-NAME>
          <FIBEX-ELEMENTS>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="Base">Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
          </FIBEX-ELEMENTS>
        </SYSTEM>
      </ELEMENTS>
    </AR-PACKAGE>
    <AR-PACKAGE><SHORT-NAME>Target</SHORT-NAME>
      <ELEMENTS>
        <ECU-INSTANCE><SHORT-NAME>Ecu</SHORT-NAME></ECU-INSTANCE>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF, "test", true).unwrap();

        let get_reference = |system_path: &str| {
            model
                .get_element_by_path(system_path)
                .and_then(|e| e.get_sub_element(ElementName::FibexElements))
                .and_then(|e| e.get_sub_element_at(0))
                .and_then(|e| e.get_sub_element(ElementName::FibexElementRef))
                .unwrap()
        };

        // the declaring package itself: in scope
        let el_declaring = get_reference("/Pkg1/System");
        assert_eq!(el_declaring.resolve_reference_base("Base").as_deref(), Some("/Target"));
        assert_eq!(
            el_declaring.get_reference_target().unwrap().path().unwrap(),
            "/Target/Ecu"
        );
        // nested inside the declaring package: in scope
        let el_nested = get_reference("/Pkg1/Sub/System");
        assert_eq!(el_nested.resolve_reference_base("Base").as_deref(), Some("/Target"));
        assert_eq!(el_nested.get_reference_target().unwrap().path().unwrap(), "/Target/Ecu");
        // a sibling package whose name merely starts with the same characters: not in scope, so the
        // relative reference in /Pkg10 cannot be resolved
        let el_sibling = get_reference("/Pkg10/System");
        assert_eq!(el_sibling.resolve_reference_base("Base"), None);
        assert!(el_sibling.get_reference_target().is_err());

        assert_eq!(model.check_references().len(), 1);
        assert_eq!(model.get_references_to("/Target/Ecu").len(), 2);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn cyclic_reference_base_terminates() {
        // A REFERENCE-BASE may use another REFERENCE-BASE as its own base. Invalid data can make
        // that chain cyclic; resolving such a base must fail instead of looping forever.
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>SelfRef</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE" BASE="SelfRef">Sub</PACKAGE-REF>
        </REFERENCE-BASE>
        <REFERENCE-BASE>
          <SHORT-LABEL>MutualA</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE" BASE="MutualB">SubA</PACKAGE-REF>
        </REFERENCE-BASE>
        <REFERENCE-BASE>
          <SHORT-LABEL>MutualB</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE" BASE="MutualA">SubB</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
      <ELEMENTS>
        <SYSTEM><SHORT-NAME>System</SHORT-NAME>
          <FIBEX-ELEMENTS>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="SelfRef">Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
            <FIBEX-ELEMENT-REF-CONDITIONAL>
              <FIBEX-ELEMENT-REF DEST="ECU-INSTANCE" BASE="MutualA">Ecu</FIBEX-ELEMENT-REF>
            </FIBEX-ELEMENT-REF-CONDITIONAL>
          </FIBEX-ELEMENTS>
        </SYSTEM>
      </ELEMENTS>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF, "test", true).unwrap();

        let el_fibex_elements = model
            .get_element_by_path("/Pkg/System")
            .and_then(|e| e.get_sub_element(ElementName::FibexElements))
            .unwrap();
        let mut references = el_fibex_elements
            .sub_elements()
            .filter_map(|ferc| ferc.get_sub_element(ElementName::FibexElementRef));
        // a reference base which is its own base
        let el_self_cycle = references.next().unwrap();
        assert_eq!(el_self_cycle.resolve_reference_base("SelfRef"), None);
        assert!(el_self_cycle.get_reference_target().is_err());
        // two reference bases which are each other's base
        let el_mutual_cycle = references.next().unwrap();
        assert_eq!(el_mutual_cycle.resolve_reference_base("MutualA"), None);
        assert!(el_mutual_cycle.get_reference_target().is_err());

        // the two references above, plus the three PACKAGE-REFs, which are themselves relative
        // references that cannot be resolved because of the cycles they are part of
        assert_eq!(model.check_references().len(), 5);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn load_duplicate_reference_bases() {
        // An AR-PACKAGE may be split across several files, each of them repeating the same
        // REFERENCE-BASE. The merge keeps only one REFERENCE-BASE element, so the cache must not
        // contain a duplicate entry either - otherwise removing the element would leave a stale
        // entry behind which still resolves references.
        const FILEBUF: &[u8] = r#"<?xml version="1.0" encoding="utf-8"?>
<AUTOSAR xsi:schemaLocation="http://autosar.org/schema/r4.0 AUTOSAR_00050.xsd" xmlns="http://autosar.org/schema/r4.0" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance">
  <AR-PACKAGES>
    <AR-PACKAGE><SHORT-NAME>Pkg</SHORT-NAME>
      <REFERENCE-BASES>
        <REFERENCE-BASE>
          <SHORT-LABEL>Base</SHORT-LABEL>
          <PACKAGE-REF DEST="AR-PACKAGE">/Target</PACKAGE-REF>
        </REFERENCE-BASE>
      </REFERENCE-BASES>
    </AR-PACKAGE>
  </AR-PACKAGES>
</AUTOSAR>"#.as_bytes();
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF, "file1", true).unwrap();
        model.load_buffer(FILEBUF, "file2", true).unwrap();

        // the merge keeps a single REFERENCE-BASE element, and the reference resolves through it
        let el_reference_bases = model
            .get_element_by_path("/Pkg")
            .and_then(|e| e.get_sub_element(ElementName::ReferenceBases))
            .unwrap();
        assert_eq!(el_reference_bases.sub_elements().count(), 1);
        let el_package_ref = el_reference_bases
            .get_sub_element_at(0)
            .and_then(|e| e.get_sub_element(ElementName::PackageRef))
            .unwrap();
        assert_eq!(
            el_package_ref.resolve_reference_base("Base").as_deref(),
            Some("/Target")
        );

        // removing that element removes the declaration for good
        let el_package = model.get_element_by_path("/Pkg").unwrap();
        el_package.remove_sub_element(el_reference_bases).unwrap();
        assert_eq!(el_package_ref.resolve_reference_base("Base"), None);
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }

    #[test]
    fn duplicate_model_resolves_relative_references() {
        // AutosarModel::duplicate() copies the element tree into a new model, so the relative references
        // in the copy must be resolved against the reference bases of the copy
        let model = AutosarModel::new();
        model.load_buffer(FILEBUF1_COMPLEX_BASES, "test", true).unwrap();
        let copy = model.duplicate().unwrap();

        let el_fibex_element_ref = copy
            .get_element_by_path("/BasesPkg/SubPackage/System")
            .and_then(|e| e.get_sub_element(ElementName::FibexElements))
            .and_then(|e| e.get_sub_element_at(0))
            .and_then(|e| e.get_sub_element(ElementName::FibexElementRef))
            .unwrap();
        assert_eq!(
            el_fibex_element_ref.get_reference_target().unwrap().path().unwrap(),
            "/ContentPkg/SubPackage/Ecu"
        );
        assert!(copy.check_references().is_empty());
        assert_eq!(model.verify_reference_caches(), Ok(()));
    }
}
