use std::fs::File;
use std::io;
use std::path::{Path, PathBuf};
use tar::Builder;

pub struct CompressTar;

impl CompressTar {
    pub fn process<T: AsRef<Path>, O: AsRef<Path>>(origin: T, dest: O) -> Result<PathBuf, io::Error> {

        let mut tar_path = dest.as_ref().to_path_buf();
        tar_path.set_extension("tar");
        let tar_file = File::create(&tar_path)?;
        let mut tar_builder = Builder::new(tar_file);

        println!("origin: {:?}", origin.as_ref());

        let metadata = std::fs::metadata(origin.as_ref())?;

        match metadata.is_dir() {
            true => {
                tar_builder.append_dir_all(origin.as_ref().file_name().unwrap(), origin.as_ref())?; // Append the directory to the tar file
            }
            false => {
                tar_builder.append_file(origin.as_ref().file_name().unwrap(), &mut File::open(origin.as_ref())?)?; // Append the file to the tar file
            }
        }
        tar_builder.finish()?;
        
        Ok(tar_path) 
    }
}
