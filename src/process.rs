use crossbeam_queue::SegQueue;
use std::{
    error::Error,
    path::Path,
    sync::{mpsc::Sender, Arc},
};


impl Clone for Format {
    fn clone(&self) -> Self {
        match self {
            Format::_7z => Format::_7z,
            Format::Xz => Format::Xz,
            Format::Zip => Format::Zip,
            Format::Gz => Format::Gz,
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::_7z => String::from("7z"),
            Format::Xz => String::from("xz"),
            Format::Zip => String::from("zip"),
            Format::Gz => String::from("gz"),
        }
    }
}

impl Default for Format {
    fn default() -> Self {
        Format::Zip
    }
}

pub trait Process<T: AsRef<Path>, O: AsRef<Path>> {
    fn process(&self, queue: Arc<SegQueue<T>>, dest: Arc<O>, sender: Option<Sender<String>>);
}

pub struct Message {
    format: Format,
}

impl Message {
    pub fn new(format: Format) -> Self {
        Message { format: format }
    }

    pub fn completion_message<P: AsRef<Path>>(&self, target_path: P) -> String {
        format!(
            "{} archiving complete: {}",
            self.format.to_string(),
            match target_path.as_ref().to_str() {
                Some(s) => s,
                None => "",
            }
        )
    }

    pub fn error_message<E: Error>(&self, error: E) -> String {
        format!(
            "{} archiving error occured!: {}",
            self.format.to_string(),
            error
        )
    }
}

// T is the type of the input file, O is the type of the output file.
pub fn get_compressor<T: AsRef<Path>, O: AsRef<Path>>(comp_t: Format) -> Box<dyn Process<T, O>> {
    return match comp_t {
        Format::Xz => Box::new(p_xz::ProcessXz::default()),
        Format::_7z => Box::new(p_7z::Process7z::default()),
        Format::Zip => Box::new(p_zip::ProcessZip::default()),
        Format::Gz => Box::new(p_gz::ProcessGz::default()),
    };
}
