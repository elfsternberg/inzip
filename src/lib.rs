//! Simple accessor for zip files
//!
//! Just experimenting to see if I understand how Zip works.
//!

use std::{
    fs::File,
    io::{Read, Seek},
};
use zip::ZipArchive;

pub enum InzipResult {
    File(Vec<u8>),
    Folder(Vec<String>),
    NotFound,
}

pub struct InZip<R: Read + Seek> {
    archive: ZipArchive<R>,
}

impl InZip<File> {
    pub fn from_file(path: impl AsRef<std::path::Path>) -> Result<Self, zip::result::ZipError> {
        let file = File::open(path)?;
        InZip::new(file)
    }
}

// Making space for the include_bytes! implementation
impl<R: Read + Seek> InZip<R> {
    pub fn new(reader: R) -> Result<Self, zip::result::ZipError> {
        Ok(InZip {
            archive: ZipArchive::new(reader)?,
        })
    }

    pub fn is_empty(&self) -> bool {
        self.archive.is_empty()
    }

    pub fn exists(&self, path: &str) -> bool {
        if self.archive.is_empty() {
            return false;
        }
        if path.is_empty() {
            return true;
        }
        self.archive.index_for_name(path).is_some()
    }

    pub fn contents(&self) -> Vec<String> {
        if self.archive.is_empty() {
            return vec![];
        }
        self.archive.file_names().map(String::from).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn dpath(path: &str) -> String {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push(path);
        d.display().to_string()
    }

    #[test]
    fn archive_empty() {
        let archive = InZip::from_file(dpath("test-data/empty.zip")).unwrap();
        assert!(archive.is_empty());
    }
}
