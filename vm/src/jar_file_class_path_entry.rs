use std::{
    cell::RefCell,
    fmt::{Debug, Formatter},
    fs::File,
    io::BufReader,
};

use zip::ZipArchive;

pub struct JarFileClassPathEntry {
    file_name: String,
    zip: RefCell<ZipArchive<BufReader<File>>>,
}

impl Debug for JarFileClassPathEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "JarFileClassPathEntry {{ file_name: {} }}",
            self.file_name
        )
    }
}
