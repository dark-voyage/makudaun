use std::io::{Read, Write};
use std::path::Path;

#[derive(Debug, Clone)]
pub struct File {
    title: String,
    content: String,
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

impl File {
    pub fn new() -> File {
        File {
            title: String::from("Unknown"),
            content: String::new(),
        }
    }

    pub fn read_from_file(&mut self, location: &Path) {
        let mut file = std::fs::File::open(location).unwrap();
        file.read_to_string(&mut self.content).unwrap();
    }

    pub fn get_title(&self) -> &String {
        &self.title
    }

    pub fn get_content(&self) -> &String {
        &self.content
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title;
    }

    pub fn exists(&self) -> bool {
        std::path::Path::new(&self.title).exists()
    }

    pub fn write_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn erase_content(&mut self) {
        self.content.clear();
    }

    pub fn append_content(&mut self, content: &str) {
        self.content.push_str(content);
    }

    pub fn read_file(&mut self, location: &str) {
        let mut file = std::fs::File::open(location).unwrap();
        file.read_to_string(&mut self.content).unwrap();
    }

    pub fn write_file(&self) {
        let mut file = std::fs::File::create(&self.title).unwrap();
        file.write_all(self.content.as_bytes()).unwrap();
    }
}
