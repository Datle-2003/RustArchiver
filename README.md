# Archiver
This project is a simple archiver that can compress and decompress files and folders. It is written in Rust can support `xz`, `gz`, and `zip` formats.

# Usage
cargo run source=[file/folder directory] destination=[destination folder] format=[gz|xz|zip] action=[decompress|compress]