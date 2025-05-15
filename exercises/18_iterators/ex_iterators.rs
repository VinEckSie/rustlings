fn main() {
    let velocity_ms = vec![23.4, 48.2, 139.99, 343.22];

    //iter() is lazy
    //we needed to make v1_iter mutable: calling the next method on an iterator changes internal state
    // that the iterator uses to keep track of where it is in the sequence.
    // In other words, this code consumes, or uses up, the iterator
    let mut vel_iter = velocity_ms.iter();
    assert_eq!(vel_iter.next(), Some(&1.0));

    for val in velocity_ms.iter() {
        println!("val {val}");
    }

    let one_val = velocity_ms.iter().next();
    println!("one val: {one_val:?}");
}
