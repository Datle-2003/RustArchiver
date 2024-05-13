mod gz;
mod tar;
mod xz;
mod zip;

use std::path::Path;

use crate::format::Format;
use crate::process::Process;


pub fn get_decompressor<T: AsRef<Path>, O: AsRef<Path>>(comp_t: Format) -> Box<dyn Process<T, O>> {
    match comp_t {
        Format::Gz => Box::new(gz::DecompressTarGz),
        Format::Xz => Box::new(xz::DecompressTarXz),
        Format::Zip => Box::new(zip::DecompressZip),
    }
}
