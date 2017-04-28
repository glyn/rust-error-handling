use std::path::PathBuf;
use std::fs::File;
use std::io::Read;

pub fn read_file(path: PathBuf) -> Result<String, ParserError> {
    e(File::open(path))
        .and_then_e(size_from_file)
        .and_then_e(catch_one)
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

impl From<usize> for ParserError {
    fn from(err: usize) -> ParserError {
        ParserError::BadSize(err.to_string())
    }
}

fn e<T, D, E>(r: Result<T,D>) -> Result<T,E>
where E: std::convert::From<D> {
           r.map_err(From::from)
}

trait ErrorMapping<T, E, D>
    where E: std::convert::From<D> {
    fn and_then_e<U, F: FnOnce(T) -> Result<U, D>>(self, op: F) -> Result<U, E>;
}

impl<T, E, D> ErrorMapping<T, E, D> for Result<T, E>
    where E: std::convert::From<D> {
    fn and_then_e<U, F: FnOnce(T) -> Result<U, D>>(self, op: F) -> Result<U, E> {
        match self {
            Ok(t) => op(t).map_err(From::from),
            Err(e) => Err(e).map_err(From::from),
        }
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

fn catch_one(s: usize) -> Result<usize, usize> {
    if s == 1 {
        Err(1)
    } else {
        Ok(s)
    }
}

#[cfg(test)]
mod tests;
