use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    //single threading
    let handle = thread::spawn(move || {
        for i in 0..10 {
            println!("{:?} {}, Spawned thread ", v, i);
        }
    });

    //multi threading case:
    //dynamics thread spawning: one thread per file/per network connection
    //batch processing: split work across thread and wait for all to complete
    //parallel computation: use multicore effectively

    //main thread
    for i in 0..5 {
        println!("{}, Main thread ", i);
    }

    handle.join().unwrap();
}
