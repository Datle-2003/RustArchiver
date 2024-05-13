use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use tar::Archive;
use xz2::read::XzDecoder;

use crate::process::Process;

pub struct DecompressTarXz;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for DecompressTarXz {
    fn process(&self, origin: T, dest: O) -> Result<PathBuf, io::Error> {
        let file = File::open(&origin)?;
        let xz_decoder = XzDecoder::new(BufReader::new(file));
        let mut archive = Archive::new(xz_decoder);
        archive.unpack(dest.as_ref())?;
        Ok(dest.as_ref().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_tar_xz() {
        let origin = "/home/datle/Code/data_compressing/test.tar.xz";
        let dest = "/home/datle/Code/data_compressing/aaa";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let decompressor = DecompressTarXz;

        let result = decompressor.process(origin_path, dest_path);

        assert!(result.is_ok());
    }
}