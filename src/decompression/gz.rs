use std::fs::File;
use std::io;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use flate2::read::GzDecoder;
use tar::Archive;

use crate::process::Process;

pub struct DecompressTarGz;
impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for DecompressTarGz {
    fn process(&self, source: T, destination: O) -> Result<PathBuf, io::Error> {

        let file = File::open(source)?;
        let gz_decoder = GzDecoder::new(BufReader::new(file));

        let mut archive = Archive::new(gz_decoder);
        archive.unpack(destination.as_ref())?;

        Ok(destination.as_ref().to_path_buf())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decompress_tar_gz() {
        let origin = "/home/datle/Code/data_compressing/a.tar.gz";
        let dest = "/home/datle/Code/data_compressing/aaa";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let decompressor = DecompressTarGz;

        let result = decompressor.process(origin_path, dest_path);

        assert!(result.is_ok());
    }
}