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
