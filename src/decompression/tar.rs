use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};

use tar::Archive;

use crate::process::Process;

pub struct DecompressTar;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for DecompressTar {
    fn process(&self, source: T, destination: O) -> Result<PathBuf, io::Error> {
        let file = File::open(&source)?;
        let mut archive = Archive::new(file);
        archive.unpack(destination.as_ref())?;
        Ok(destination.as_ref().to_path_buf())
    }
}