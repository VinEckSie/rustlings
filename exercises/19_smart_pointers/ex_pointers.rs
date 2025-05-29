use std::borrow::Cow;
use std::fmt::Debug;
use std::rc::Rc;
use List::{Cons as BCons, Nil as BNil};
use ListRc::{Cons as RcCons, Nil as RcNil};
fn main() {
    // A pointer is a general concept for a variable that contains an address in memory

    // In many cases smart pointers own the data they point to.
    // String and Vec<T> count as smart pointers because they own some memory and allow you to manipulate it.
    // Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement
    //the Deref trait: allows an instance of the smart pointer struct to behave like a reference so you can write your code
    //      to work with either references or smart pointers.
    // And Drop trait: allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

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

    //take care about references cycles causing memory leaks when two Rcs reference each other
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

    // Cow (Clone-On-Write)
    // Cow<'a, T> is an enum: Borrowed(&'a T) or Owned(T)
    // Acts like a reference (&T) until mutation is needed → then clones to an owned value
    // Enables shared logic for borrowed and owned data without duplicating code
    // Ideal for read-mostly cases with occasional mutation

    // Why Cow Exists (Real-World Need)
    // In real programs:
    // Borrowed data: default values, constants, compile-time strings → "hello"
    // Owned data: user input, file content, runtime strings → String
    // You often work in mixed scenarios: some data comes from memory you don’t own, others are dynamically created
    // You don't want to write two versions of every function or struct → Cow unifies both cases

    // Use Cases
    // Use Case	Why Cow Helps
    // Static or dynamic input	One function for both; avoids unnecessary allocation
    // Optional mutation	Clones only if mutation is needed (.to_mut() is lazy)
    // Long-lived config or struct	Supports literals and runtime-generated data

    // Rust Behavior Notes
    // Mutability is not transitive: you can mutate a variable, and not when it is used as parameters
    // Rust auto-derefs for method calls → no need to use *

    // Practical Usage Tips
    // If a function takes Cow<str> or Cow<[T]>:
    // "hello".into() → becomes Cow::Borrowed
    // String::from("...").into() → becomes Cow::Owned
    // Prefer .into() over Cow::Owned(...) or .to_string() for ergonomic code

    //read-only logging function
    //static log
    let default_temp: Cow<str> = "30 degrees".into();
    read_log(default_temp);
    //dynamics log
    let user_input_temp: Cow<str> = String::from("45 degrees").into();
    read_log(user_input_temp);

    //optional mutation (sanitizing user input before saving)
    //static input
    let mut default_input: Cow<str> = "Hello".into();
    read_input(&mut default_input);
    //dynamics input no need to mutate
    let mut user_input_forbidden: Cow<str> = String::from("Forbidden to update").into();
    read_input(&mut user_input_forbidden);
    //dynamics input need to mutate
    let mut user_input_change: Cow<str> = String::from("ok to change").into();
    read_input(&mut user_input_change);

    //user config struct with flexibility
    let default_config = ModuleConfig::default_config();
    dbg!(default_config);
    let user_config = ModuleConfig::dyn_config(String::from("Cubesat"), String::from("Jupiter"));
    dbg!(user_config);
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

// fn may_change_altitude(alt: &mut Cow<str>) {
//     let need_mutation = true;
//
//     if need_mutation {
//         alt.to_mut().push_str(", second text");
//     }
// }

//Cow functions
//read-only
fn read_log<T>(log: Cow<T>)
where
    T: std::fmt::Display + ToOwned + ?Sized,
    T::Owned: std::fmt::Display,
{
    println!("{}", log);
}

//might mutate
fn read_input(input: &mut Cow<str>) {
    if input.contains("change") {
        *input.to_mut() = String::from("Update");
        input.to_mut().push_str(", OK");
    }
    println!("{}", input);
}

//flexible config struct
#[derive(Debug)]
struct ModuleConfig<'a> {
    name: Cow<'a, str>,
    mission: Cow<'a, str>,
}

impl<'a> ModuleConfig<'a> {
    fn default_config() -> Self {
        Self {
            name: Cow::Borrowed("Stars"),
            mission: Cow::Borrowed("Mars"),
        }
    }

    fn dyn_config(name: String, mission: String) -> Self {
        Self {
            // name: String::from(name).into(),
            name: Cow::Owned(name),
            mission: Cow::Owned(mission),
        }
    }
}
