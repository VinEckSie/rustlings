use std::time::Duration;
use std::{sync, thread};
use sync::Mutex;

fn main() {
    // Generally speaking, working with a type which owns its data is easier than working with one that uses references.
    let philosopher = vec![
        Philosopher::new("Friedrich Nietzsche"),
        Philosopher::new("Socrate"),
        Philosopher::new("Plato"),
        Philosopher::new("Aristotle"),
        Philosopher::new("Kant"),
    ];

    // for p in &philosopher {
    //     p.eat();
    // }

    let handles: Vec<_> = philosopher
        .into_iter()
        .map(|p| {
            thread::spawn(move || {
                p.eat();
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}

struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
    fn eat(&self) {
        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    fork: Mutex<Vec<()>>,
}
