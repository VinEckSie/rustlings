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

    // What does the ? operator do?
    // It unwraps a Result or Option, but propagates the error (or None) if it fails, instead of panicking.

    //if you deal with position instead of characters (number slice vs str slice), use TRY_INTO, instead of split

    let str_number = "345";
    let number = i32::from_str(str_number);
    println!("{:?}", number);

    //convert &str to String
    let user = "test".to_string();
    let guest = String::from("guest");

    //From
    // open_file("fake path").unwrap();

    //FromStr
    let expected = Ok(Point { x: 3, y: 5 });

    assert_eq!(Point::from_str("(3,5)"), expected);
    assert_eq!("(3,5)".parse(), expected);
    assert_eq!("(3,5)".parse::<Point>(), expected);
    assert!(Point::from_str("(44 5)").is_err());
}

//From
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

//FromStr
#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Eq, PartialEq)]
struct ParseIntError;

impl FromStr for Point {
    type Err = ParseIntError;

    fn from_str(data: &str) -> Result<Point, ParseIntError> {
        let (x, y) = data
            .strip_prefix('(')
            .and_then(|data| data.strip_suffix(')'))
            .and_then(|data| data.split_once(','))
            .ok_or(ParseIntError)?;

        let x_coords = x.parse::<i32>().map_err(|_| ParseIntError)?;
        let y_coords = y.parse::<i32>().map_err(|_| ParseIntError)?;

        Ok(Point {
            x: x_coords,
            y: y_coords,
        })
    }
}
