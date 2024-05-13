use std::path::{Path, PathBuf};
use std::fs::File;
use std::io;
use zip::ZipArchive;

use crate::process::Process;

pub struct DecompressZip;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for DecompressZip {
    fn process(&self, origin: T, dest: O) -> Result<PathBuf, io::Error> {
        let origin_path = origin.as_ref();
        let origin_file = File::open(&origin_path)?;

        let mut achiver = ZipArchive::new(origin_file)?;

        for i in 0..achiver.len() {
            let mut file = achiver.by_index(i)?;
            let out_path = format!("{}/{}", dest.as_ref().display(), file.name());

            // Create any necessary parent directories
            if let Some(parent_dir) = std::path::Path::new(&out_path).parent() {
                std::fs::create_dir_all(parent_dir)?;
            }

            // Create a new file in the output directory
            let mut out_file = File::create(out_path)?;

            // Copy the contents of the file from the archive to the output file
            io::copy(&mut file, &mut out_file)?;
        }

        Ok(dest.as_ref().to_path_buf())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_zip() {
        let origin = "/home/datle/Code/data_compressing/test.zip";
        let dest = "/home/datle/Code/data_compressing/aaa";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let decompressor = DecompressZip;

        let result = decompressor.process(origin_path, dest_path);

        assert!(result.is_ok());
    }
}