use std::{fs, path::Path};

#[cfg(test)]
use mockall::{automock, predicate::*};
#[cfg_attr(test, automock)]
pub trait FileWriterTrait {
    fn write_to_destination(&mut self, destination: &Path, content: &str) -> Result<(), String>;
}

pub struct FileWriter;

impl FileWriterTrait for FileWriter {
    fn write_to_destination(&mut self, destination: &Path, content: &str) -> Result<(), String> {
        match fs::write(destination, content) {
            Ok(k) => Ok(k),
            Err(e) => {
                println!("Error writing file {}", e);

                Err(String::from("Error writing file"))
            }
        }
    }
}
