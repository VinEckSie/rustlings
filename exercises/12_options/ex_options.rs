use std::ops::Deref;
use std::option::Option; // this is optional, but for clarity


fn main() {
    //Nullable pointer
    let nullable = None;
    check_option(nullable);
    
    let value = Some(Box::new(20));
    check_option(value);

    //? operator (shorter way as match pattern)
    // let question_value = Some(String::from("question mark test"));
    let question_value = None;
    println!("\n? Operator: ");
    println!("Value is {:?}", get_value(question_value));


    println!("\n? Adapters to work with references: ");
    println!("as_ref: "); //as_ref() is used to avoid moving the Option<String>, so we can still use user later.
    let mut user_id: Option<String> = Some("89859574973".to_string());
    //as_ref
    match user_id.as_ref() {
        Some(user_id) => println!("User ID is {:?}", user_id),
        None => println!("User ID none"),
    }
    // name is still usable because we didn't take ownership
    println!("User ID Confirmation: {:?}", user_id);


    println!("as_mut: "); // as_mut() lets you change the value inside the Option directly.
    match user_id.as_mut() {
        Some(user_id) => {
            *user_id = "new8787878787".to_string();
            println!("User ID is {:?}", user_id)
        },
        None => println!("User ID none"),
    }
    println!("User ID Confirmation: {:?}", user_id);


    println!("as_deref: ");// as_deref() You don’t want to clone or own the string. as_deref() gives you an Option<&str> to work with.
    // let user_mail: Option<Box<String>> = Some(Box::new("vinsie@gmail.com".to_string()));
    // 
    // match user_mail.as_deref() {
    //     Some("vinsie@gmail.com") => println!("This is a vinsie!"),
    //     Some(other) => println!("This is not a vinsie!: {}", other),
    //     None => println!("No user email"),
    // }

    // fn main() {
    //     let user_mail: Option<Box<String>> = Some(Box::new("vinsie@gmail.com".to_string()));
    // 
    //     match user_mail.as_deref() {
    //         Some("vinsie@gmail.com") => println!("This is a vinsie!"),
    //         Some(other) => println!("This is not a vinsie!: {}", other),
    //         None => println!("No user email"),
    //     }
    // }

//OK
        // let user_mail: Option<Box<String>> = Some(Box::new("vinsie@gmail.com".to_string()));
        // 
        // match user_mail {
        //     Some(ref boxed_string) if boxed_string.as_str() == "vinsie@gmail.com" => {
        //         println!("This is a vinsie!")
        //     }
        //     Some(ref other) => {
        //         println!("This is not a vinsie!: {}", other)
        //     }
        //     None => println!("No user email"),
        // }

fn main() {
    let user_mail: Option<Box<String>> = Some(Box::new("vinsie@gmail.com".to_string()));

    let string_ref: Option<&str> = user_mail.as_deref(); // ✅ this compiles

    match string_ref {
        Some("vinsie@gmail.com") => println!("This is a vinsie!"),
        Some(other) => println!("This is not a vinsie!: {}", other),
        None => println!("No user email"),
    }
}


    

















}

//Nullable pointer
fn check_option(optional: Option<Box<i32>>) {
    match optional {
        Some(x) => println!("Value: {}", x),
        None => println!("None"),
    }
}

//? operator (shorter way as match pattern)
// Use ? when…	
// You just want to propagate None upwards	
// You’re in a function that returns Option	
// You want cleaner, linear code	
// You want fewer nested blocks	
// 
// Use match when…
// You want to handle None differently
// You're doing something more complex
// You’re matching on multiple values at once
// You need different logic per match arm
fn get_value(optional: Option<String>)  -> Option<String> {
    Some(optional)?
}

//NPO (Nullable pointer optimization)
// Concept	                                What to remember
// Option<Box<T>>, Option<&T>	            Zero-cost — same size as the pointer
// Use Option for optional fields	        It’s safe and efficient
// #[repr(transparent)] structs	            Inherit optimization from inner type
// Don't worry about transmute	            Unless doing unsafe systems-level work
// Safe nullable pointers	                Use Option always — no null bugs


//Adapters to work with references


// as_deref_mut 🔍 This is useful for editing values inside smart pointers like String, Box<T>, etc.
//as pin ref Needed when working with async code or other pinned types where moving is forbidden.
//as pin mut 🔍 Used when building custom futures, generators, or safely mutating data that must not move


//Extracting contained value
// Method	            Panics?	    Use When...
// expect("msg")	    ✅ Yes	T	Panics with your custom "msg"	Must-have value, want clear error
// unwrap()	            ✅ Yes	T	Panics with generic message	Quick debug code / tests only
// unwrap_or(x)	        ❌ No	T	Returns x	Simple default
// unwrap_or_default()	❌ No	T	Returns T::default()	Type’s default is fine
// unwrap_or_else(f)	❌ No	T	Calls f() and returns result	Expensive or lazy fallback


//Transforming contained values
    //Transform Option to Result
    // Use .ok_or(...) when you want to convert to Result with a simple error
    // Use .ok_or_else(...) when the error needs logic or is expensive
    // Use .transpose() when you have an Option<Result> and want clean error handling

    //Transform Some variant
    //filter Use case: keep Some(t) only if it passes a test.
    //flatten  Use case: chaining optional logic that might return another Option.
    //map  Use case: Transform the inner value.

    //Return a value (not an option) : transform Option<T> to a value of a possibly different type U
    //map or Use case: You want to always get a value, even if it's None.
    //map or else Use case: Computing the fallback is expensive or contextual.

    //Combine two options
    //zip Use case: Combine two options into a tuple only if both exist.
    //zip with Use case: Combine two Options with logic (e.g., add, concat, etc.)


//bool operator
// and_then:
    // chaining multiple operations like parsing or validation that might return Option without match/unwrap.
// or_else (LAZY FEEDBACK)
    // Use or_else to call a function only if None—a lazy fallback that only computes if needed,
    // useful when the fallback requires logic, and you don't want to run heavy code unnecessarily.
// or (STATIC FEEDBACK)
    // static fallback when you have a backup plan, easy way to say “use this if missing”.

//NOT USED OFTEN
//and(opt) = Too limited — and_then() is always more flexible (can return different type, apply logic)
// xor(opt) = Rarely needed — only useful in weird “only one must be Some” situations, almost never seen in real code.
// and_then(None → None) & or_else(Some → ignored) : These are already covered by and_then() and or_else() — no need to remember the internals.


//iterate
// into_iter
    // You want to move out the value when you do not need anymore, turning it into an iterator of 0 or 1 item.
// iter
    // You want to read the value, Borrows the value inside the Option as &T
// iter_mut
    // You want to modify the value inside the Option safely, Borrows the value inside as &mut T.


//collecting into (if any is None => None is returned)
//collect
    // when you're transforming or validating data and want to stop on first failure.
    // when you want all-or-nothing collection
//sum
    // when you're aggregating validated inputs (e.g. user scores, parsed data).
    // when You want to add only if all are valid
//product
    // when you’re computing totals from optional or fallible data sources.
    // when You want to multiply only if all valid

//modifying in place
//insert
    //you want to force a new value
//get or insert
    //you want to ensure a value is present
//get or insert with
    //when value is expensive to compute
//take
    //you want to move out the value
//replace
    //you want to swap the value cleanly

