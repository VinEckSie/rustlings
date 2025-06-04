use std::time::Duration;
use std::{sync, thread};
use sync::{Arc, Mutex};

fn main() {
    let table = Arc::new(Table {
        forks: vec![
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
            Mutex::new(()),
        ],
    });

    // Generally speaking, working with a type which owns its data is easier than working with one that uses references.
    let philosopher = vec![
        Philosopher::new("Friedrich Nietzsche", 0, 1),
        Philosopher::new("Socrate", 1, 2),
        Philosopher::new("Plato", 2, 3),
        Philosopher::new("Aristotle", 3, 4),
        Philosopher::new("Kant", 0, 4),
    ];

    let handles: Vec<_> = philosopher
        .into_iter()
        .map(|p| {
            let table = table.clone();

            thread::spawn(move || {
                p.eat(&table);
            })
        })
        .collect();

    for h in handles {
        h.join().unwrap();
    }
}

struct Philosopher {
    name: String,
    left: usize,
    right: usize,
}

impl Philosopher {
    fn new(name: &str, left: usize, right: usize) -> Self {
        Self {
            name: name.to_string(),
            left,
            right,
        }
    }
    fn eat(&self, table: &Table) {
        let _left = table.forks[self.left].lock().unwrap();
        let _right = table.forks[self.right].lock().unwrap();

        println!("{} is eating.", self.name);
        thread::sleep(Duration::from_millis(1000));
        println!("{} is done eating.", self.name);
    }
}

struct Table {
    forks: Vec<Mutex<()>>,
}
