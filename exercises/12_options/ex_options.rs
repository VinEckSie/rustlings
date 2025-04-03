fn main() {
    //nullable pointers
    let nulabble = None;
    check_option(nulabble);
    
    let value = Some(Box::new(20));
    check_option(value);
}

//question mark () shorter way as match pattern
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

//Nullable pointer
fn check_option(optional: Option<Box<i32>>) {
    match optional {
        Some(x) => println!("Value: {}", x),
        None => println!("None"),
    }

    // println!("Size of u8: {}", size_of::<u8>());
    // println!("Size of Option<u8>: {}", size_of::<Option<u8>>());

    println!("Size of &u8: {}", size_of::<&u8>());
    println!("Size of Option<&u8>: {}", size_of::<Option<&u8>>());
}


//NPO (Nullablle pointer optimization)

// Concept	                                What to remember
// Option<Box<T>>, Option<&T>	            Zero-cost — same size as the pointer
// Use Option for optional fields	        It’s safe and efficient
// #[repr(transparent)] structs	        Inherit optimization from inner type
// Don't worry about transmute	            Unless doing unsafe systems-level work
// Safe nullable pointers	                Use Option always — no null bugs


//as_ref() is used to avoid moving the Option<String>, so we can still use user later.

// as_mut() lets you change the value inside the Option directly.

// 🔍 You don’t want to clone or own the string. as_deref() gives you an Option<&str> to work with.

// as_deref_mut 🔍 This is useful for editing values inside smart pointers like String, Box<T>, etc.

//🔍 Needed when working with async code or other pinned types where moving is forbidden.

//🔍 Used when building custom futures, generators, or safely mutating data that must not move




// When you want to fail explicitly with a fixed error message or value if the Option is None.

//Same as ok_or, but lazily calls a function to produce the error only if needed.


// Method	Panics?	Return if Some	Return if None	Use When...
// expect("msg")	✅ Yes	T	Panics with your custom "msg"	Must-have value, want clear error
// unwrap()	✅ Yes	T	Panics with generic message	Quick debug code / tests only
// unwrap_or(x)	❌ No	T	Returns x	Simple default
// unwrap_or_default()	❌ No	T	Returns T::default()	Type’s default is fine
// unwrap_or_else(f)	❌ No	T	Calls f() and returns result	Expensive or lazy fallback


// Use .ok_or(...) when you want to convert to Result with a simple error
// 
// Use .ok_or_else(...) when the error needs logic or is expensive
// 
// Use .transpose() when you have an Option<Result> and want clean error handling

//transformation
//filter Use case: keep Some(t) only if it passes a test.
//flatten  Use case: chaining optional logic that might return another Option.
//map  Use case: Transform the inner value.

//method that return a value (not an option)
//map transform the inner value
//map or Use case: You want to always get a value, even if it's None.
//map or else Use case: Computing the fallback is expensive or contextual.

//methods that combine two options
//zip Use case: Combine two options into a tuple only if both exist.
//zip with Use case: Combine two Options with logic (e.g., add, concat, etc.)


//bool ope
//and = continue only if both sides are Some      Use when each step depends on the previous being valid.
// 
// or = fallback to second if first is None     Use when you have a backup plan
// 
// xor = keep exactly one Some Rare —      used when you expect only one side to be Some.
// 
// and_then = call a function only if Some     Use when chaining fallible operations like parsing or validation.
// 
// or_else = call a function only if None    Use when fallback requires logic, not just a static value.

//iterate





