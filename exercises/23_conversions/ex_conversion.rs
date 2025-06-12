use crate::CusError::{IoError, ParseError};
use std::fs;
use std::io;
use std::num;
use std::str::FromStr;

fn main() {
    //always prefer implementing From<T> or TryFrom<T> rather than Into<U> or TryInto<U>, as From and TryFrom provide greater flexibility
    // and offer equivalent Into or TryInto implementations for free

    //  If `From` is implemented, an implementation of `Into` is automatically provided.
    // You can read more about it in the documentation:

    // When you implement From<SomeError> for your own error type, you automatically enable the ? operator to convert SomeError
    // into your error type, thanks to the From/Into machinery used by ?.

    let str_number = "345";
    let number = i32::from_str(str_number);
    println!("{:?}", number);

    //convert &str to String
    let user = "test".to_string();
    let guest = String::from("guest");

    //FromStr
    // open_file("fake path").unwrap();
}

//FromStr
#[derive(Debug)]
enum CusError {
    IoError(io::Error),
    ParseError(num::ParseIntError),
}

impl From<io::Error> for CusError {
    fn from(error: io::Error) -> Self {
        IoError(error)
    }
}

impl From<num::ParseIntError> for CusError {
    fn from(error: num::ParseIntError) -> Self {
        ParseError(error)
    }
}

fn open_file(file: &str) -> Result<i32, CusError> {
    let file = fs::read_to_string(file)?;
    let content: i32 = file.trim().parse()?;

    Ok(content)
}
