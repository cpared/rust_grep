use std::fs;

#[derive(Debug)]
pub enum FileReaderError {
    FileNotFound,
}

#[derive(Debug)]
pub struct FileReader {
    text: String,
}

impl FileReader {
    pub fn new(path: &str) -> Result<FileReader, FileReaderError> {
        match fs::read_to_string(path) {
            Ok(contents) => Ok(FileReader { text: contents }),
            Err(_) => Err(FileReaderError::FileNotFound),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.text
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn success_file_exist() {
        let path = String::from("test");
        let reader = super::FileReader::new(&path);
        let file_reader = reader.unwrap();
        assert_eq!("example", file_reader.get_text())
    }

    #[test]
    fn error_file_does_not_exist() {
        let path = String::from("invalid");
        let reader = super::FileReader::new(&path);
        assert!(matches!(reader, Err(super::FileReaderError::FileNotFound)))
    }
}
