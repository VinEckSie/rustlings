fn main() {
    let velocity_ms = vec![23.4, 48.2, 139.99, 343.22];

    //iter() is lazy
    //we needed to make v1_iter mutable: calling the next method on an iterator changes internal state
    // that the iterator uses to keep track of where it is in the sequence.
    // In other words, this code consumes, or uses up, the iterator
    let mut vel_iter = velocity_ms.iter();
    assert_eq!(vel_iter.next(), Some(&23.4)); //consume the first value here

    for val in velocity_ms.iter() {
        println!("val loop: {val}");
    }

    let one_val = velocity_ms.iter().next();
    println!("first val: {one_val:?}");

    //iter produces immutable refs
    //iter_mut produces mutable refs
    //into_iter produces owned values

    //consuming adapters
    // let sum_val: f64 = vel_iter.copied().sum();
    // let sum_val2: f64 = vel_iter.collect();
    // println!("Sum: {sum_val}");

    //iterators adapters (don't consume but produce other adapters): map, filter, take
    //they are lazy, so we need to call a consuming adapter, like collect()
    let increase_velocity: Vec<_> = vel_iter.map(|x| x + 5.0).collect();
    dbg!(increase_velocity);

    //closure to capture environment
    // let minimum_velocity: Vec<_> = velocity_ms.into_iter().filter(|&val| val > 100.0).collect();
    // dbg!(minimum_velocity);

    //execute directly, no need to use variable yet collect()
    velocity_ms.iter().for_each(|x| println!("{x}"));

    //turn an infinite operator into a finite one
    let num = 0..;
    let range = num.take(5);

    range.for_each(|x| println!("{x}"));
}
