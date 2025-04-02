fn main() {
    //nullable pointers
    let nulabble = None;
    check_option(nulabble);
    
    let value = Some(Box::new(20));
    check_option(value);
}

fn check_option(optional: Option<Box<i32>>) {
    match optional {
        Some(x) => println!("Value: {}", x),
        None => println!("None"),
    }

    // println!("Size of u8: {}", size_of::<u8>());
    // println!("Size of Option<u8>: {}", size_of::<Option<u8>>());

    println!("Size of &u8: {}", size_of::<&u8>());
    println!("Size of Option<&u8>: {}", size_of::<Option<&u8>>());

}