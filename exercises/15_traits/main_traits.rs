mod lib_traits;

use std::fmt::Display;
use lib_traits::{
    Summarize,
    Block, Transaction,
    display_notification, notification
};

fn main() {
    let transaction1 = Transaction::new();
    let mut block1 = Block::new();

    println!("\nTransaction amount: {}", transaction1.get_amount());
    println!("\nTransaction sum up: {}", transaction1.sum_up());
    println!("Default sum up for transaction: {:?}", transaction1.sum_up_default());
    
    block1.add_transaction(transaction1);
    
    println!("Bloch hash: {}", block1.get_hash());
    println!("Block sum up: {}",block1.sum_up());
    
    println!("Default sum up for block: {:?}", block1.sum_up_default());
    println!("{}",notification(&block1));
    println!("{}",display_notification(&block1));
     
    //conditional implementation
    let new_coords = Coords::new(64,55);
    new_coords.get_highest();

}

//conditional trait implementation
struct Coords<T> {
    x:T,
    y:T,
}

impl<T> Coords<T> {
    fn new(x: T, y: T) -> Self {
        Coords {x, y}
    }
} 

impl<T: Display + PartialOrd> Coords<T> {
    fn get_highest(&self) {
        if self.x > self.y {
            println!("x is the biggest: {}", self.x);
        }
        else {
            println!("y is the biggest: {}", self.y);
        }
    }
}











