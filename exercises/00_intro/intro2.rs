use std::io;

fn main() {
    // TODO: Fix the code to print "Hello world!".
    //println!("Hello world!");
    let collection = [1,2,3,4,5,];

    println!("Please, enter an index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("wrong type index typed");

    let index: usize = index
    .trim()
    .parse()
    .expect("Wrong index");

    let element = collection[index];

    println!("index {index}, valeur : {element}");


}
