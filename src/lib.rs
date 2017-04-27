use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: PathBuf) -> Result<String, ParserError> {
    File::open(path)
        .map_err(From::from)
        .and_then(|file| size_from_file(file).map_err(From::from))
        .and_then(|_| Ok(String::from("File is non-empty")))
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
mod tests;
