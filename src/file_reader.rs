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
    // Uso unwrap solamente para poder testear la clase
    use super::*;

    #[test]
    fn success_file_exist() {
        let reader = FileReader::new("test");
        let file_reader = reader.unwrap();
        assert_eq!("example", file_reader.get_text())
    }

    #[test]
    fn error_file_does_not_exist() {
        let path = String::from("invalid");
        let reader = FileReader::new(&path);
        assert!(matches!(reader, Err(FileReaderError::FileNotFound)))
    }
}
