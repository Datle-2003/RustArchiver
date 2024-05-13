use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};

use crate::process::Process;

use flate2::write::GzEncoder;
use flate2::Compression;
use std::io::{Read, Write};

use crate::compression::tar::CompressTar;

#[derive(Debug)]
pub struct CompressTarGz;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for CompressTarGz {
    // compress a file/folder to a tar file, then compress the tar file to a tar.gz file
    fn process(&self, origin: T, dest: O) -> Result<PathBuf, io::Error> {
        let origin_path = origin.as_ref();
        let file_name = origin_path.file_stem().unwrap().to_str().unwrap(); // get the name of the file/folder

        println!("file_name: {:?}", file_name);

        let destination_path = dest.as_ref().join(file_name);

        println!("destination_path: {:?}", destination_path);

        // compress the file/folder to a tar file, output is .tar file
        let tar_path = CompressTar::process(&origin, &destination_path)?;

        println!("tar_path: {:?}", tar_path);

        let gz_path = tar_path.with_extension("tar.gz");

        println!("gz_path: {:?}", gz_path);

        let mut tar_file = File::open(&tar_path)?;
        let gz_file = File::create(&gz_path)?;

        let mut gz_encoder = GzEncoder::new(gz_file, Compression::default());

        let mut buffer = Vec::new();
        tar_file.read_to_end(&mut buffer)?;

        gz_encoder.write_all(&buffer)?;

        gz_encoder.finish()?;

        fs::remove_file(&tar_path)?;

        Ok(gz_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress_tar_gz() {
        let origin = "/home/datle/Code/data_compressing/test/a.txt";
        let dest = "/home/datle/Code/data_compressing/";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let compressor = CompressTarGz;

        let result = compressor.process(origin_path, dest_path);

        assert!(result.is_ok());
    }
}
