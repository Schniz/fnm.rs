use super::extract::{Error, Extract};
use reqwest::Response;
use std::io::Read;
use std::path::Path;
use tempdir::TempDir;
use tempfile::tempfile;

pub struct Zip {
    response: Response,
}

impl Zip {
    #[allow(dead_code)]
    pub fn new(response: Response) -> Self {
        Self { response }
    }
}

impl Extract for Zip {
    fn extract_into<P: AsRef<Path>>(mut self, path: P) -> Result<(), Error> {
        let mut tmp_zip_file = tempfile().expect("Can't get a temporary file");
        let tmp_dir = TempDir::new("node_extraction").expect("Can't get a temporary dir");

        self.response.copy_to(&mut tmp_zip_file)?;
        let mut zip_archive = zip::read::ZipArchive::new(tmp_zip_file)?;

        for index in 0..zip_archive.len() {
            let mut file = zip_archive.by_index(index)?;
            let destination = tmp_dir.path().join(file.sanitized_name());
            if file.is_dir() {
                std::fs::create_dir(&destination)?;
            } else {
                let mut vec = vec![];
                file.read_to_end(&mut vec)?;
                std::fs::write(destination, vec)?;
            }
        }

        std::fs::rename(&tmp_dir, path)?;

        Ok(())
    }
}
