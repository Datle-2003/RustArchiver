use std::{error::Error, path::PathBuf};

// use format from format.rs in the same directory
use crate::format::Format;
use crate::process;

#[derive(Debug, PartialEq)]

pub enum Action {
    Compress,
    Decompress,
}

impl Action {
    pub fn from(action: &str) -> Self {
        match action {
            "compress" => Action::Compress,
            "decompress" => Action::Decompress,
            _ => panic!("wrong action string!"),
        }
    }
}

impl Clone for Action {
    fn clone(&self) -> Self {
        match self {
            Action::Compress => Action::Compress,
            Action::Decompress => Action::Decompress,
        }
    }
}

// Archiver struct to hold the archiving information
pub struct Archiver {
    path: Option<PathBuf>,        // paths to be archived
    destination: Option<PathBuf>, // destination directory
    format: Option<Format>,       // format of the archive
    action: Option<Action>,       // action to be performed
}

impl Archiver {
    // create by input file
    pub fn new(path: String, dest: String, format: String, action: String) -> Self {
        Archiver {
            path: Some(PathBuf::from(path)),
            destination: Some(PathBuf::from(dest)),
            format: Some(Format::from(&format)),
            action: Some(Action::from(&action)),
        }
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

    pub fn verify_source(&self) -> Result<(), Box<dyn Error>> {
        match &self.path {
            Some(path) => {
                if !path.exists() {
                    return Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        "Source file/folder does not exist",
                    )));
                }
                Ok(())
            }
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Source file/folder is not set",
            ))),
        }
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        self.verify_destination()?;
        self.verify_source()?;

        let path = self.path.as_ref().ok_or("Path is not specified")?;
        let destination = self
            .destination
            .as_ref()
            .ok_or("Destination is not specified")?;
        let format = self.format.as_ref().ok_or("Format is not specified")?;
        let action = self.action.as_ref().ok_or("Action is not specified")?;

        let archiver = process::get_archiver(format.clone(), action.clone());

        archiver.process(path, destination)?;
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_from() {
        assert_eq!(Action::from("compress"), Action::Compress);
        assert_eq!(Action::from("decompress"), Action::Decompress);
    }

    #[test]
    fn test_archiver_new() {
        let archiver = Archiver::new(
            "test".to_string(),
            "test".to_string(),
            "zip".to_string(),
            "compress".to_string(),
        );
        assert_eq!(archiver.path, Some(PathBuf::from("test")));
        assert_eq!(archiver.destination, Some(PathBuf::from("test")));
        assert_eq!(archiver.format, Some(Format::Zip));
        assert_eq!(archiver.action, Some(Action::Compress));
    }

    #[test]
    fn test_verify_destination() {
        let archiver = Archiver::new(
            "test".to_string(),
            "test".to_string(),
            "zip".to_string(),
            "compress".to_string(),
        );
        assert!(archiver.verify_destination().is_ok());
    }

    #[test]
    fn test_verify_source() {
        let archiver = Archiver::new(
            "tests".to_string(),
            "tests".to_string(),
            "zip".to_string(),
            "compress".to_string(),
        );
        assert!(archiver.verify_source().is_err());
    }
}
