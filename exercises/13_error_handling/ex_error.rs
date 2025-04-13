use std::fs;
use std::io::{BufRead, Error};
use std::fs::File;
use std::io::{read_to_string, ErrorKind, Read};




// fn main() -> Result<(), Error> {
fn main() {
    //errorhandling
    //let file = File::open("hello.txt");
    
    //v1 double check error
    // let result_file  = match file {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(newfile) => newfile,
    //             Err(error) => panic!("Tried to create file but there was a problem: {:?}", error),
    //         },
    //         _ => panic!("There was a problem with the file: {:?}", error),
    //     }
    // };
    
    //v2 update with unwrap or else
    // let file = File::open("hello.txt").unwrap_or_else(|error| 
    //     if error.kind() == ErrorKind::NotFound {
    //         File::create("hello.txt").unwrap_or_else(|error| {
    //             panic!("could not create file: {:?}", error);
    //         })
    //     } else {
    //         panic!("Problem opening the file: {error:?}");
    //     }
    // );
        
    //propagating with ?
    // let mut username= "".to_string();
    // let file = File::open("hello.txt")?.read_to_string(&mut username)?;
    // print!("{username}");
    // Ok(())
    
    //generics
    //function
    // let number_list = vec![34, 50, 25, 100, 65];
    // 
    // let result = largest(&number_list);
    // println!("The largest number is {result}");
    // 
    // let char_list = vec!['y', 'm', 'a', 'q'];
    // 
    // let result = largest(&char_list);
    // println!("The largest char is {result}");

    // The help text mentions std::cmp::PartialOrd,
    
    
    //struct
    //with one generic, then with two generics (3 examples for this point)
    
    //We can also define structs to use a generic type parameter in one or more fields using the <> syntax. 
    // Listing 10-6 defines a Point<T> struct to hold x and y coordinate values of any type.
    //with one generics and two generics
    //If you’re finding you need lots of generic types in your code, 
    // it could indicate that your code needs restructuring into smaller pieces.

    //enum
    //option and result example
    
    //method
    //implememt method x on struct Point
    //implement method only for a certain type
    //example with mixup
    
    //monomorphization; explain the concept
    
    //result
    //use of parse with number litteral
    
    //boxing error greate for smalll projects or prototyping
}


//shorten on ? 2 examples:
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}