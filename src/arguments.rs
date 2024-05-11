use std::path::PathBuf;

use crate::format::Format;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Action {
    Compress,
    Decompress,
}

// struct to hold the arguments
pub struct Arguments {
    pub source: Option<PathBuf>,
    pub destination: Option<PathBuf>,
    pub format: Option<Format>,
    pub password: Option<String>,
    pub action: Option<Action>,
}

impl Arguments {
    pub fn new() -> Self {
        Arguments {
            source: None,
            destination: None,
            format: None,
            password: None,
            action: None,
        }
    }

    pub fn set_source<T: ToString>(&mut self, source: T) {
        self.source = Some(PathBuf::from(source.to_string()));
    }

    pub fn set_destination<T: ToString>(&mut self, destination: T) {
        self.destination = Some(PathBuf::from(destination.to_string()));
    }

    pub fn set_format<T: ToString>(&mut self, format: T) {
        self.format = Some(Format::from(&format.to_string()));
    }

    pub fn set_password<T: ToString>(&mut self, password: T) {
        self.password = Some(password.to_string());
    }

    pub fn set_action<T: ToString>(&mut self, action: T) {
        match action.to_string().as_str() {
            "compress" => self.action = Some(Action::Compress),
            "decompress" => self.action = Some(Action::Decompress),
            _ => self.action = None,
        }
    }
    fn help() {
        println!("Usage: cargo run source=[source_folder|source_file] destination=[destination_directory] format=[zip|7z|gz|xz] password=[password] action=[compress|decompress]");
        std::process::exit(1);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arguments_new() {
        let args = Arguments::new();
        assert_eq!(args.source, None);
        assert_eq!(args.destination, None);
        assert_eq!(args.format, None);
        assert_eq!(args.password, None);
        assert_eq!(args.action, None);
    }

    #[test]
    fn test_arguments_set_source() {
        let mut args = Arguments::new();
        args.set_source("source");
        assert_eq!(args.source, Some(PathBuf::from("source")));
    }

    #[test]
    fn test_arguments_set_destination() {
        let mut args = Arguments::new();
        args.set_destination("destination");
        assert_eq!(args.destination, Some(PathBuf::from("destination")));
    }

    #[test]
    fn test_arguments_set_format() {
        let mut args = Arguments::new();
        args.set_format("zip");
        assert_eq!(args.format, Some(Format::Zip));
    }

    #[test]
    fn test_arguments_set_password() {
        let mut args = Arguments::new();
        args.set_password("password");
        assert_eq!(args.password, Some(String::from("password")));
    }

    #[test]
    fn test_arguments_set_action() {
        let mut args = Arguments::new();
        args.set_action("compress");
        assert_eq!(args.action, Some(Action::Compress));
    }
}
