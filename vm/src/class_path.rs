use log::debug;

use crate::class_path_entry::{ClassLoadingError, ClassPathEntry};

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct ClassPath {
    entries: Vec<Box<dyn ClassPathEntry>>,
}

impl ClassPath {
    pub fn resolve(&self, class_name: &str) -> Result<Option<Vec<u8>>, ClassLoadingError> {
        for entry in self.entries.iter() {
            debug!("looking up class {} in {:?}", class_name, entry);
            let entry_result = entry.resolve(class_name)?;
            if let Some(class_bytes) = entry_result {
                return Ok(Some(class_bytes));
            }
        }

        Ok(None)
    }
}
