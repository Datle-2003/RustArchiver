// Module to handle the format of the file(s)

#[derive(PartialEq, Debug)]
pub enum Format {
    Zip,
    Xz,
    Gz,
}

impl Format {
    pub fn from(format_str: &str) -> Self {
        match format_str {
            "xz" => Format::Xz,
            "zip" => Format::Zip,
            "gz" => Format::Gz,
            _ => panic!("wrong format string!"),
        }
    }

}

impl Clone for Format {
    fn clone(&self) -> Self {
        match self {
            Format::Xz => Format::Xz,
            Format::Zip => Format::Zip,
            Format::Gz => Format::Gz,
        }
    }
}

impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Xz => String::from("xz"),
            Format::Zip => String::from("zip"),
            Format::Gz => String::from("gz"),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from() {
        assert_eq!(Format::from("xz"), Format::Xz);
        assert_eq!(Format::from("zip"), Format::Zip);
        assert_eq!(Format::from("gz"), Format::Gz);
    }

    #[test]
    #[should_panic]
    fn test_format_from_panic() {
        Format::from("tar");
    }

    #[test]
    fn test_format_clone() {
        assert_eq!(Format::Xz.clone(), Format::Xz);
        assert_eq!(Format::Zip.clone(), Format::Zip);
        assert_eq!(Format::Gz.clone(), Format::Gz);
    }

    #[test]
    fn test_format_to_string() {
        assert_eq!(Format::Xz.to_string(), "xz");
        assert_eq!(Format::Zip.to_string(), "zip");
        assert_eq!(Format::Gz.to_string(), "gz");
    }
}