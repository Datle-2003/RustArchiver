
#[derive(PartialEq)]
#[derive(Debug)]
pub enum Format {
    Zip,
    _7z,
    Xz,
    Gz,
}

impl Format {
    pub fn from(format_str: &str) -> Self {
        match format_str {
            "7z" => Format::_7z,
            "xz" => Format::Xz,
            "zip" => Format::Zip,
            "gz" => Format::Gz,
            _ => panic!("wrong format string!"),
        }
    }

    pub fn extension(&self) -> String {
        match self {
            Format::_7z => String::from(".7z"),
            Format::Xz => String::from(".tar.xz"),
            Format::Zip => String::from(".zip"),
            Format::Gz => String::from(".tar.gz"),
        }
    }
}

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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_from() {
        assert_eq!(Format::from("7z"), Format::_7z);
        assert_eq!(Format::from("xz"), Format::Xz);
        assert_eq!(Format::from("zip"), Format::Zip);
        assert_eq!(Format::from("gz"), Format::Gz);
    }

    #[test]
    fn test_format_extension() {
        assert_eq!(Format::_7z.extension(), ".7z");
        assert_eq!(Format::Xz.extension(), ".tar.xz");
        assert_eq!(Format::Zip.extension(), ".zip");
        assert_eq!(Format::Gz.extension(), ".tar.gz");
    }

    #[test]
    fn test_format_to_string() {
        assert_eq!(Format::_7z.to_string(), "7z");
        assert_eq!(Format::Xz.to_string(), "xz");
        assert_eq!(Format::Zip.to_string(), "zip");
        assert_eq!(Format::Gz.to_string(), "gz");
    }
}
