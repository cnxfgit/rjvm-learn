use crate::class_path_entry::ClassPathEntry;

#[allow(dead_code)]
#[derive(Default, Debug)]
pub struct ClassPath {
    entries: Vec<Box<dyn ClassPathEntry>>,
}
