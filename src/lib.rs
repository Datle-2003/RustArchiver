use std::{collections::VecDeque, error::Error, path::{Path, PathBuf}};

pub mod format;
pub mod arguments;

pub use format::Format;
pub use arguments::Arguments;
pub use arguments::Action;


// Archiver struct to hold the archiving information
pub struct Archiver {
    destination: Option<PathBuf>,     // Path to the destination directory
    format: Format,                   // Compression format
    queue: Option<VecDeque<PathBuf>>, // Queue of files to compress
    action: Option<Action>,           // Action to perform
    password: Option<String>,         // Password for encryption

}


impl Archiver {
    pub fn new(args: Arguments) -> Self {
        Archiver {
            destination: args.destination,
            format: args.format.unwrap(),
            queue: None,
            action: args.action,
            password: args.password,
        }
    }

    pub fn set_destination<T: AsRef<Path>>(&mut self, destination: T) {
        self.destination = Some(destination.as_ref().to_path_buf());
    }

    pub fn set_format(&mut self, format: Format) {
        self.format = format;
    }

    pub fn set_format_str<T: ToString>(&mut self, format_str: T) {
        self.format = Format::from(&format_str.to_string())
    }

    pub fn push_from_iter<I>(&mut self, iter: I)
    where
        I: Iterator,
        I::Item: AsRef<Path>,
    {
        if let None = self.queue {
            self.queue = Some(VecDeque::new());
        }

        for i in iter {
            self.queue
                .as_mut()
                .unwrap()
                .push_back(i.as_ref().to_path_buf());
        }
    }

    // push a single element to the queue
    pub fn push<T: AsRef<Path>>(&mut self, path: T) {
        if let None = self.queue {
            self.queue = Some(VecDeque::new());
        }

        self.queue
            .as_mut()
            .unwrap()
            .push_back(path.as_ref().to_path_buf());
    }

    pub fn verify_destination(&self) -> Result<(), Box<dyn Error>> {
        match &self.destination {
            Some(path) => {
                if !path.is_dir() {
                    // create the directory
                    std::fs::create_dir_all(path)?;
                }
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Destination directory is not set",
            ))),
        }
    }

    pub fn verify_queue(&self) -> Result<(), Box<dyn Error>> {
        match &self.queue {
            Some(q) => {
                if q.is_empty() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Queue is empty",
                    )));
                }
                Ok(())
            }
            None => {
                Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "The queue is empty",
                )))
            }
        }
    }

    pub fn archive(&self) -> Result<(), Box<dyn Error>> {
        self.verify_destination()?;
        self.verify_queue()?;

        // create arc object to share between threads

        Ok(())
    }

}
