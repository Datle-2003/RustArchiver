mod archiver;
mod format;
mod process;
mod compression;
mod decompression;


use archiver::Archiver;

use std::collections::HashMap;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut map = HashMap::new();
    for arg in args.iter().skip(1) {
        let parts: Vec<&str> = arg.split('=').collect();
        if parts.len() == 2 {
            map.insert(parts[0].to_string(), parts[1].to_string());
        }
    }

    let source = map.get("source").expect("source not found").to_string();
    let destination = map.get("destination").expect("destination not found").to_string();
    let format = map.get("format").expect("format not found").to_string();
    let action = map.get("action").expect("action not found").to_string();

    let archiver = Archiver::new(source, destination, format, action);
    archiver.run().expect("Error running archiver");
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        let args = vec![
            "archiver".to_string(),
            "source=src/main.rs".to_string(),
            "destination=src/main.rs".to_string(),
            "format=zip".to_string(),
            "action=archive".to_string(),
        ];
        let mut map = HashMap::new();
        for arg in args.iter().skip(1) {
            let parts: Vec<&str> = arg.split('=').collect();
            if parts.len() == 2 {
                map.insert(parts[0].to_string(), parts[1].to_string());
            }
        }

        let source = map.get("source").expect("source not found").to_string();
        let destination = map.get("destination").expect("destination not found").to_string();
        let format = map.get("format").expect("format not found").to_string();
        let action = map.get("action").expect("action not found").to_string();

        let archiver = Archiver::new(source, destination, format, action);
        archiver.run().expect("Error running archiver");
    }
}
