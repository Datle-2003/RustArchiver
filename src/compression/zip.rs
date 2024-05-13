use std::fs::File;
use std::io;
use std::io::Write;
use std::path::{Path, PathBuf};
use zip::{write::FileOptions, ZipWriter};

use super::{get_content_vec, list_all_files};
use crate::process::Process;
pub struct CompressZip;

impl<T: AsRef<Path>, O: AsRef<Path>> Process<T, O> for CompressZip {
    fn process(&self, origin: T, dest: O) -> Result<PathBuf, io::Error> {
        let mut zip_file_name =
            PathBuf::from(dest.as_ref().join(&origin.as_ref().file_name().unwrap()));
        zip_file_name.set_extension("zip");
        let zip_file = File::create(&zip_file_name)?;

        let mut zip_writer = ZipWriter::new(zip_file);
        let options = FileOptions::default().compression_method(zip::CompressionMethod::Deflated);

        for file in list_all_files(&origin)? {
            let content = get_content_vec(&file)?;
            zip_writer.start_file(
                file.strip_prefix(&origin.as_ref().parent().unwrap())
                    .unwrap()
                    .to_str()
                    .unwrap(),
                options,
            )?;
            zip_writer.write_all(&content)?;
        }

        zip_writer.finish()?;

        return Ok(zip_file_name);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_compress_zip() {
        let origin = "/home/datle/Code/data_compressing/test/a.txt";
        let dest = "/home/datle/Code/data_compressing/";

        let origin_path = Path::new(origin);
        let dest_path = Path::new(dest);

        let compressor = CompressZip;

        let result = compressor.process(origin_path, dest_path);
        assert!(result.is_ok());
    }
}