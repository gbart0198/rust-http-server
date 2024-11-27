use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct FileLoader<'a> {
    path: &'a str,
}

impl<'a> FileLoader<'a> {
    pub fn new(path: &'a str) -> Self {
        FileLoader { path }
    }

    pub fn read(self) -> String {
        let file_path = Path::new(self.path);
        let mut file = File::open(file_path).unwrap();

        let mut file_content = String::new();
        file.read_to_string(&mut file_content).unwrap();

        return file_content;
    }
}
