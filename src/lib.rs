use std::path::PathBuf;
use std::fs::File;
use std::io::Read;
use std::io::Error;

pub fn read_file(path: PathBuf) -> Result<String, ParserError> {
    let size = size_from_file(File::open(path)?)?;
    Ok(String::from("File is non-empty"))
}

#[derive(Debug)]
pub enum ParserError {
    BadFile(std::io::Error),
    BadSize(String),
}

impl From<std::io::Error> for ParserError {
    fn from(err: std::io::Error) -> ParserError {
        ParserError::BadFile(err)
    }
}

impl From<std::string::String> for ParserError {
    fn from(err: std::string::String) -> ParserError {
        ParserError::BadSize(err)
    }
}


fn size_from_file(mut file: File) -> Result<usize, String> {
    let mut my_string = String::new();
    match file.read_to_string(&mut my_string) {
        Err(err) => Err(err.to_string()),
        Ok(_) => {
            match my_string.len() {
                0 => Err(String::from("File is empty")),
                l => Ok(l),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    #[test]
    fn non_existent_file() {
        let path = PathBuf::from("src/fixtures/no-such-file");

        let result = super::read_file(path);
        match result {
            Err(err) => match err {
                super::ParserError::BadFile(msg) => println!("{}", msg),
                super::ParserError::BadSize(msg) => assert!(false, msg)
            },
            Ok(message) => assert!(false, message)
        }
    }

    #[test]
    fn empty_file() {
        let path = PathBuf::from("src/fixtures/empty");

        let result = super::read_file(path);
        match result {
            Err(err) => match err {
                super::ParserError::BadFile(msg) => assert!(false, msg),
                super::ParserError::BadSize(msg) => println!("{}", msg)
            },
            Ok(message) => assert!(false, message)
        }
    }

    #[test]
    fn non_empty_file() {
        let path = PathBuf::from("src/fixtures/nonempty");

        let result = super::read_file(path);
        match result {
            Err(err) => match err {
                super::ParserError::BadFile(msg) => assert!(false, msg),
                super::ParserError::BadSize(msg) => assert!(false, msg)
            },
            Ok(message) => println!("{}", message)
        }
    }
}
