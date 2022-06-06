use super::*;


pub struct ArxmlFileIterator<'a> {
    pub(crate) data: &'a AutosarData,
    pub(crate) index: usize,
}


impl<'a> Iterator for ArxmlFileIterator<'a> {
    type Item = ArxmlFile;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(files) = self.data.files.lock() {
            if self.index < files.len() {
                let result = files[self.index].clone();
                self.index += 1;
                return Some(result);
            }
        }
        None
    }
}