
//traits as parameters (simple and multiples)
//where clause
//as return type
//conditional implements with traits


//simple trait
pub trait Summarize {
    fn sum_up(&self) -> String;
    
    fn sum_up_default(&self) -> String {
        format!("Default Sum up method, and {}", self.sum_up())
    }
}

pub struct Transaction {
    from: String,
    to: String,
    amount: f64,
    token: String,
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {
            from: String::from("ffeefeij84875489357eoie"),
            to: String::from("fjiejfowejiowefjeifo"),
            amount: 50.00,
            token: String::from("SOL")
        }
    }

    pub fn get_amount(&self) -> f64 {
        self.amount
    }
}
impl Summarize for Transaction {
    fn sum_up(&self) -> String {
        format!("Transaction processed from {} to {} for an amount of {}", self.from, self.to, self.amount)
    }

    fn sum_up_default(&self) -> String {
        "Transaction default sum up".to_string()
    }
}

pub struct Block {
    bloc_id: u64,
    hash: String,
    prev_hash: String,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new() -> Self {
        Block {
            bloc_id: 87587587394035,
            hash: String::from("hffehfehfuehfeuhfeuhe8787789"),
            prev_hash: String::from("hffeherrfgfffehfeuhe8787789"),
            transactions: Vec::new(),
        }
    }
    
    pub fn get_hash(&self) -> &String {
        &self.hash
    }
    
    pub fn add_transaction(&mut self, trans: Transaction) {
        self.transactions.push(trans);
    }
}

impl Summarize for Block {
    fn sum_up(&self) -> String {
        format!("Block #{}), hash({}), transactions num ({})", self.bloc_id, self.hash, self.transactions.len())
    }
}