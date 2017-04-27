use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: PathBuf) -> Result<String, ParserError> {
    File::open(path)
        .map_err(|err| ParserError::BadFile(err.to_string()))
        .and_then(|file| size_from_file(file).map_err(|err| ParserError::BadSize(err)))
        .and_then(|_| Ok(String::from("File is non-empty")))
}

#[derive(Debug)]
pub enum ParserError {
    BadFile(String),
    BadSize(String),
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