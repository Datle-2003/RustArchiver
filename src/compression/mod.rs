mod gz;
mod xz;
mod tar;
mod zip;

use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::format::Format;
use crate::process::Process;


pub fn get_compressor<T: AsRef<Path>, O: AsRef<Path>>(comp_t: Format) -> Box<dyn Process<T, O>> {
    match comp_t {
        Format::Gz => Box::new(gz::CompressTarGz),
        Format::Xz => Box::new(xz::CompressTarXz),
        Format::Zip => Box::new(zip::CompressZip),
    }
}

fn list_all_files<T: AsRef<Path>>(path: T) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let path = path.as_ref();
    if path.is_dir() {
        for entry in std::fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.extend(list_all_files(&path)?);
            } else {
                files.push(path);
            }
        }
    } else {
        files.push(path.to_path_buf());
    }
    Ok(files)
}

fn get_content_vec<T: AsRef<Path>>(path: T) -> Result<Vec<u8>, io::Error> {
    let mut file = File::open(path)?;
    let mut content = Vec::new();
    file.read_to_end(&mut content)?;
    Ok(content)
}
