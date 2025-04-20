use std::{error::Error, fs};

#[derive(Debug)]
pub struct Config {
    query: String,
    file_path: String,
}

impl Config {
    pub fn new(query: String, file_path: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self { query, file_path })
    }

    pub fn read_file(&self) -> Result<String, Box<dyn Error>> {
        match fs::read_to_string(&self.file_path) {
            Ok(file_content) => Ok(file_content),
            Err(e) => Err(e.into()),
        }
    }

    pub fn is_file_having_query(&self) -> bool {
        let file = self
            .read_file()
            .expect("An error occured during the file reading.");
        file.contains(&self.query)
    }

    pub fn get_query(&self) -> &String {
        &self.query
    }

    pub fn get_file_path(&self) -> &String {
        &self.file_path
    }
}
