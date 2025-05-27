use std::sync::{Arc, Mutex};
use std::thread;
// Using this strategy, you can divide a calculation into independent parts, split those parts across threads,
// and then use a Mutex<T> to have each thread update the final result with its part.

//Arc is a concurrency primitive where Rc can be use only in single thread context
//Arc is not set by default for all primitive because it has a cost over performance to ensure the safety side of Arc

// Arc: lets you share the data across threads.
// Mutex: lets you safely change that data one thread at a time.
// Mutex: take care about deadlock when an operation needs to lock two resources, and two threads have locked each other these resources
//they can wait each other forever

//Use Mutex only if you need to change the date, otherwise an Arc is enough

//Replace Mutex by RwLock if you can data reader be able to read at the same time.
//Mutex: one reader at a time
//RwLock: x reader at a time, but locked regarding the writer
fn main() {
    //Read only data through multi threads: only Arc necessary
    let greet = Arc::new("Hey Rustacean!");

    for _ in 0..5 {
        let shared_greet = Arc::clone(&greet);
        thread::spawn(move || {
            println!("{}", shared_greet);
        });
    }

    //Read and update date through multi threads: need Arc and Mutex
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    dbg!(*counter.lock().unwrap());
}
