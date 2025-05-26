use std::rc::Rc;
use List::{Cons as BCons, Nil as BNil};
use ListRc::{Cons as RcCons, Nil as RcNil};
fn main() {
    // A pointer is a general concept for a variable that contains an address in memory

    // in many cases smart pointers own the data they point to.
    // String and Vec<T> count as smart pointers because they own some memory and allow you to manipulate it.
    // Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement
    //the Deref trait: allows an instance of the smart pointer struct to behave like a reference so you can write your code
    //      to work with either references or smart pointers.
    // and Drop trait: allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

    // most common smart pointers:
    // Box<T>, for allocating values on the heap
    //When to use boxes:
    // When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size
    // When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so
    // When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type

    let num = Box::new(17); // box is stored on the stack and data on the heap
    println!("{num}");

    //Unknown size example
    let my_list = BCons(2, Box::new(BCons(5, Box::new(BCons(7, Box::new(BNil))))));
    dbg!(my_list);

    // Rc<T>, a reference counting type that enables multiple ownership
    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only
    // RC:  Reference counting pointer enables you to allow data to have multiple owners
    // by keeping track of the number of owners and, when no owners remain, cleaning up the data.
    //only for a single thread
    let list_1 = Rc::new(RcCons(
        2,
        Rc::new(RcCons(5, Rc::new(RcCons(7, Rc::new(RcNil))))),
    ));
    dbg!(Rc::strong_count(&list_1));

    let list_2 = RcCons(2, Rc::clone(&list_1));
    dbg!(&list_2);
    dbg!(Rc::strong_count(&list_1));

    {
        let list_3 = RcCons(3, Rc::clone(&list_1));
        dbg!(&list_3);

        dbg!(Rc::strong_count(&list_1));
    }

    dbg!(Rc::strong_count(&list_1));

    // Ref<T> and RefMut<T>, accessed through RefCell<T>, a type that enforces the borrowing rules at runtime instead of compile time
    //missing in rusting doc, upgrade doc here

    //Mutex stands for Mutual Exclusion
    //Enables to share data between multiple thread by a locking guard system
    //rules:
    //must attempt to acquire the lock before to use data
    //must unlock the data, once you are done with so another thread can acquire the lock

    //example in a single thread context
    use std::sync::Mutex;

    let temperature = Mutex::new(56);

    // let mut lock1 = temperature.lock().unwrap();
    // *lock1 = 52;

    {
        let mut temp = temperature.lock().unwrap();
        *temp = 67;
    }

    dbg!(&temperature);
} // both are deallocated here

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
enum ListRc {
    Cons(i32, Rc<ListRc>),
    Nil,
}
