use std::fs::File;

fn main() {
    //errorhandling
    let file = File::open("hello.txt");
    
    let result_file = match file {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
    
    //double check error
    //update with unwrap or else
    //propagating with ?
    //shorten on ?
    // fs::read_to_string("hello.txt")

    // fn last_char_of_first_line(text: &str) -> Option<char> {
    //     text.lines().next()?.chars().last()
    // }

    
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
    //We can also define structs to use a generic type parameter in one or more fields using the <> syntax. Listing 10-6 defines a Point<T> struct to hold x and y coordinate values of any type.
    //with one generics and two generics
    //If you’re finding you need lots of generic types in your code, it could indicate that your code needs restructuring into smaller pieces.

    //enum
    

}