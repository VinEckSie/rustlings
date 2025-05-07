mod lib_traits;
use lib_traits::{Summarize, Block, Transaction};

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

}