use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use xz2::write::XzEncoder;

use crate::compression::tar::CompressTar;
use crate::process::Process;

pub struct CompressTarXz;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for CompressTarXz {
    fn process(&self, origin: T, dest: O) -> Result<PathBuf, io::Error> {
        let origin_path = origin.as_ref();
        let file_name = origin_path.file_stem().unwrap().to_str().unwrap(); // get the name of the file/folder

        println!("file_name: {:?}", file_name);

        let destination_path = dest.as_ref().join(file_name);

        println!("destination_path: {:?}", destination_path);

        // compress the file/folder to a tar file, output is .tar file
        let tar_path = CompressTar::process(&origin, &destination_path)?;

        let xz_path = tar_path.with_extension("tar.xz");

        let mut tar_file = File::open(&tar_path)?; // Open the tar file
        let xz_file = File::create(&xz_path)?; // Create the xz file

        let mut xz_encoder = XzEncoder::new(xz_file, 9); // Create a new xz encoder

        let mut buffer = Vec::new();
        tar_file.read_to_end(&mut buffer)?; // Read the tar file into the buffer

        xz_encoder.write_all(&buffer)?; // Write the buffer to the xz file

        xz_encoder.finish()?; // Finish the xz file

        std::fs::remove_file(&tar_path)?; // Remove the tar file (no longer needed

        Ok(xz_path) // Return the path to the xz file
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compress_tar_xz() {
        let origin = "/home/datle/Code/data_compressing/test/";
        let dest = "/home/datle/Code/data_compressing/";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let compressor = CompressTarXz;

        let result = compressor.process(origin_path, dest_path);

        assert!(result.is_ok());
    }
}
