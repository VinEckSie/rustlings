use std::sync::RwLockReadGuard;

fn main() {
    //Nullable pointer
    let nullable = None;
    check_option(nullable);
    
    let value = Some(Box::new(20));
    check_option(value);

    let question_value = None;
    println!("\n? Operator: ");
    println!("Value is {:?}", get_value(question_value));


    println!("\n? Adapters to work with references: ");
    println!("as_ref: "); //used to avoid moving the Option<String>, so we can still use user later.
    let mut user_id: Option<String> = Some("89859574973".to_string());
    //as_ref
    match user_id.as_ref() {
        Some(user_id) => println!("User ID is {:?}", user_id),
        None => println!("User ID none"),
    }
    println!("User ID Confirmation: {:?}", user_id); // name is still usable because we didn't take ownership

    println!("as_mut: "); //lets you change the value inside the Option directly.
    match user_id.as_mut() {
        Some(user_id) => {
            *user_id = "new8787878787".to_string();
            println!("User ID is {:?}", user_id)
        },
        None => println!("User ID none"),
    }
    println!("User ID Confirmation: {:?}", user_id);

    println!("as_deref: "); // rust issue stand by

    println!("as_deref_mut: "); // editing values inside smart pointers like String, Box<T>, etc.
    let mut boxed_name = Box::new(Some("locust".to_string()));
    clear_user_id(&mut boxed_name);

    //as pin ref() used inside custom future/stream types or low-level async code. Not needed in everyday Rust.
    //as pin mut() Used when building custom futures, generators, or safely mutating data that must not move
    
    println!("Extracting contained value: ");
    // expect("msg")	    ✅ Panic	custom error "msg"	Must-have value, want clear error
    let my_id_custom: Option<i32> = Some(-76767676);
    //let my_id_custom: Option<i32> = None;

    // my_id_custom.expect("wallet_id not found");
    // unwrap()	            ✅ Panic	generic error message	Quick debug code / tests only | Avoid in Production
    //my_id_custom.unwrap();
    // unwrap_or(x)	        ❌ No panic	Very common in apps, CLI, configs.
    //my_id_custom.unwrap_or(00000000);
    // unwrap_or_default()	❌ No panic	Great for configs, deserialization, init logic.
    //my_id_custom.unwrap_or_default();
    // unwrap_or_else(f)	❌ No panic	Most useful in performance-sensitive or async cases | Expensive or lazy fallback
    // my_id_custom.unwrap_or_else(|| wallet_default_error());
    println!("my_id_custom: {:?}", my_id_custom);


    println!("\n? Transforming contained values: ");
    println!("Transform Option to Result: ");
    // Use .ok_or(...) when you want to convert to Result with a simple error
    let mut id_result: Result<_, &str> = my_id_custom.ok_or("Error: No id result");
    // Use .ok_or_else(...) when the error needs logic or is expensive
    id_result = my_id_custom.ok_or_else(|| result_default_error());
    // Use .transpose() when you have an Option<Result> and want clean error handling
    let maybe_id: Option<Result<_,_>> = Some(id_result);
    let id_verified: Result<_,_> = maybe_id.transpose();
    println!("id_result: {:?}", id_result);
    println!("id_verified: {:?}", id_verified);
    
    println!("Transform Some variant: ");
    println!("Filter: keep Some(t) only if it passes a test: ");
    let my_megative_id = my_id_custom.filter(|var| var.is_negative()); 
    
    if let Some(my_megative_id) = my_megative_id {
        println!("id is negative: {:?}", my_megative_id);
    }
    
    println!("Flatten: chaining optional logic that might return another Option:");
    println!("Flatten, parsing value: ");
    let account_id = Some("588");
    let parsed =  account_id.map(|acc| acc.parse::<u32>().ok()).flatten();
    println!("account_id parsed: {:?}", parsed);
    println!("Flatten, error handling: ");
    let account_error: Result<Result<i32, &str>, &str> = Ok(Ok(642));
    let account_error_flat = account_error.iter().flatten();
    println!("Flatten, flat error: {:?}", account_error_flat);
    println!("Flatten, working with iterators: ");
    let my_vec = vec![Some("588"), None, Some("554")];
    println!("my_vec_processed: {:?}", my_vec
            .iter()
            .flatten()
            .collect::<Vec<_>>());
    
        
    println!("Return a value (not an option) : transform Option<T> to a value: ");
    //map or Use case: You want to always get a value, even if it's None.
    //map or else Use case: Computing the fallback is expensive or contextual.
    
    println!("Combine two options: ");
    //zip Use case: Combine two options into a tuple only if both exist.
    //zip with Use case: Combine two Options with logic (e.g., add, concat, etc.)
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
// You just want to propagate None upwards | You’re in a function that returns Option | You want cleaner, linear code | You want fewer nested blocks
// Use match when…
// You want to handle None differently | You're doing something more complex | You’re matching on multiple values at once | You need different logic per match arm
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
// as_deref_mut
fn clear_user_id(user_id: &mut Box<Option<String>>) {
    if let Some(id) = user_id.as_deref_mut() {
        println!("Clearing user {:?}", id.to_uppercase());
    }
}

//unwrap_or_else()
fn wallet_default_error() -> i32 {
    println!("wallet_default_error");
    9999
}

//ok_or_else
fn result_default_error() -> &'static str {
    "wallet_default_error"
}


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

