use std::fs::{create_dir, File, remove_file};
use std::io::{Read, Write, Result};

pub struct Bucket {
    pub name: String,
    pub path: String,
}

impl Bucket {
    pub fn new(name: &str) -> Bucket {
        Bucket {
            name: String::from(name),
            path: format!("./{name}")
        }
    }

    pub fn create(&self) -> Result<()> {
        return create_dir(&self.path);
    }

    fn get_file_path(&self, name: &str) -> &str {
        return &format!("{}/{}", &self.path, name);
    }

    pub fn eq(&self, other: &Bucket) -> bool {
        return &self.name == other.name;
    }

    pub fn create_object(&self, name: &str, data: &[u8]) -> Result<()> {
        let file_path: &str = &self.get_file_path(name);
        let mut file: File = File::create(file_path)?;
        file.write_all(&data)?;

        return Ok(());
    }

    pub fn read_object(&self, name: &str) -> &[u8] {
        let file_path: &str = &self.get_file_path(name);
        let mut data: Vec<u8> = Vec::new();
        let mut file: File = File::open(file_path)?;
        file.read_to_end(&mut data)?;
        return data.as_slice();
    }

    pub fn remove_object(&self, name: &str) -> Result<()> {
        let file_path: &str = &self.get_file_path(name);
        return remove_file(file_path);
    }
}
