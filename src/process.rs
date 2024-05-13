use std::io;
use std::path::Path;
use std::path::PathBuf;

use crate::compression::get_compressor;
use crate::decompression::get_decompressor;

use crate::format::Format;
use crate::archiver::Action;



pub fn get_archiver<T: AsRef<Path>, O: AsRef<Path>>(
    format: Format,
    action: Action,
) -> Box<dyn Process<T, O>> {
    match action {
        Action::Compress => get_compressor(format),
        Action::Decompress => get_decompressor(format),
    }
}

pub trait Process<T: AsRef<Path>, O: AsRef<Path>> {
    fn process(&self, source: T, destination: O) -> Result<PathBuf, io::Error>;
}


