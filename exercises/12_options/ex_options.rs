
fn main() {
    println!("\n\nNullable pointer: ");
    let nullable = None;
    check_option(nullable);
    
    let value = Some(Box::new(20));
    check_option(value);
    let question_value = None;


    println!("\n\n? operator: ");
    println!("Value is {:?}", get_value(question_value));


    println!("\n\nAdapters to work with references: ");
    println!("\nas_ref: "); //used to avoid moving the Option<String>, so we can still use user later.
    let mut user_id: Option<String> = Some("89859574973".to_string());
    //as_ref
    match user_id.as_ref() {
        Some(user_id) => println!("User ID is {:?}", user_id),
        None => println!("User ID none"),
    }
    println!("User ID Confirmation: {:?}", user_id); // name is still usable because we didn't take ownership

    println!("\nas_mut: "); //lets you change the value inside the Option directly.
    match user_id.as_mut() {
        Some(user_id) => {
            *user_id = "new8787878787".to_string();
            println!("User ID is {:?}", user_id)
        },
        None => println!("User ID none"),
    }
    println!("User ID Confirmation: {:?}", user_id); // name is still usable because we didn't take ownership

    println!("\nas_deref: "); // rust issue #87121, work around: 
    let user_mail: Option<Box<String>> = Some(Box::new("mail@example.com".to_string()));
    let string_slice = user_mail.as_deref();

    match string_slice.as_deref().map(|ref_to_string| ref_to_string.as_str()) {
        Some("mail@example.com") => println!("Matched!"),
        Some(other) => println!("Other: {}", other),
        None => println!("None"),
    }

    println!("\nas_deref_mut: "); // editing values inside smart pointers like String, Box<T>, etc.
    let mut boxed_name = Box::new(Some("locust".to_string()));
    clear_user_id(&mut boxed_name);

    //as pin ref() used inside custom future/stream types or low-level async code. Not needed in everyday Rust.
    //as pin mut() Used when building custom futures, generators, or safely mutating data that must not move


    println!("\n\nExtracting contained value: ");
    println!("Use .expect() when you're absolutely sure the value is present and you want a meaningful panic. But in most real-world situations, prefer .unwrap_or(...), .unwrap_or_else(...), or pattern matching (if let).");
    println!("\nexpect: "); //✅ Panic	custom error "msg"	Must-have value, want clear error
    let my_id_custom: Option<i32> = Some(-76767676);
    //let my_id_custom: Option<i32> = None;
    // my_id_custom.expect("wallet_id not found");

    println!("\nunwrap: "); //✅ Panic	generic error message	Quick debug code / tests only | Avoid in Production
    //my_id_custom.unwrap();

    println!("\nunwrap_or: "); //❌ No panic	Very common in apps, CLI, configs.
    my_id_custom.unwrap_or(00000000);

    println!("\nunwrap_or_default: "); //❌ No panic	Great for configs, deserialization, init logic.
    my_id_custom.unwrap_or_default();

    println!("\nunwrap_or_else: "); //❌ No panic	Most useful in performance-sensitive or async cases | Expensive or lazy fallback
     my_id_custom.unwrap_or_else(|| wallet_default_error());
    println!("my_id_custom: {:?}", my_id_custom);


    println!("\n\nTransforming contained values: ");
    println!("Transform Option to Result: ");

    println!("\nok_or: ");//when you want to convert to Result with a simple error
    let mut id_result: Result<_, &str> = my_id_custom.ok_or("Error: No id result");
    println!("id_result: {:?}", id_result);

    println!("\nok_or_else: ");//when the error needs logic or is expensive
    id_result = my_id_custom.ok_or_else(|| result_default_error());
    println!("id_result: {:?}", id_result);

    println!("\ntranspose: ");//when you have an Option<Result> and want clean error handling
    let maybe_id: Option<Result<_,_>> = Some(id_result);
    let id_verified: Result<_,_> = maybe_id.transpose();
    println!("id_result: {:?}", id_result);
    println!("id_verified: {:?}", id_verified);
    
    println!("\nTransform Some variant: ");
    println!("Filter: keep Some(t) only if it passes a test: ");
    let my_negative_id = my_id_custom.filter(|var| var.is_negative());
    if let Some(my_negative_id) = my_negative_id {
        println!("id is negative: {:?}", my_negative_id);
    }
    
    println!("\nFlatten: chaining optional logic that might return another Option:");
    println!("Flatten, parsing value: ");
    let account_id = Some("588");
    let parsed =  account_id.map(|acc| acc.parse::<u32>().ok()).flatten();
    println!("account_id parsed: {:?}", parsed);

    println!("\nFlatten, error handling: ");
    let account_error: Result<Result<i32, &str>, &str> = Ok(Ok(642));
    let account_error_flat = account_error.iter().flatten();
    println!("Flatten, flat error: {:?}", account_error_flat);

    println!("\nFlatten, working with iterators: ");
    let my_vec = vec![Some("588"), None, Some("554")];
    println!("my_vec_processed: {:?}", my_vec
            .iter()
            .flatten()
            .collect::<Vec<_>>());
    
        
    println!("\n\nReturn a value (not an option): ");
    println!("\nmap: "); //apply a function to the option value, need to add unwrap to access the value
    println!(" you just apply a transformation");
    println!("if the closure return T, use map");
    let mut liquidation_threshold = Some(0.75);
    let liquidation_threshold_per = liquidation_threshold.map(|lt| lt * 100.0);
    if let Some(liquid) = liquidation_threshold_per {
        println!("liquidation in %: {}", liquid);
    }

    println!("\nmap_or: "); //You want to always get a value, even if it's None, give the inner value, no need unwrap
    liquidation_threshold = None;
    let liquidation_threshold_dft = liquidation_threshold.map_or(50.0, |lt| lt * 100.0);
    println!("liquidation (default value) {}", liquidation_threshold_dft);

    println!("\nmap_or_else: ");//map or else Use case: Computing the fallback is expensive or contextual.
    let liquidation_threshold_exp = liquidation_threshold.map_or_else (|| map_default(), |lt| lt * 100.0);
    println!("liquidation (default expensive value) {}", liquidation_threshold_exp);

    println!("\n\nCombine two options: ");

    println!("\nzip: ");//zip Use case: Combine two options into a tuple only if both exist.
    let liquidation_zipped = my_negative_id.zip(parsed);
    println!("liquidation zipped: {liquidation_zipped:?}");

    println!("\nzip with: ");//zip with Use case: Combine two Options with logic (e.g., add, concat, etc.)
    println!("WARNING: works only in nightly rust");
    let liquidation_zipped_with = my_negative_id.zip(Some(-35)).map(|(a,b)| a + b);
    println!("liquidation zipped: {liquidation_zipped_with:?}");

    println!("\n\nBool operator: ");
    println!("\nand_then: "); // chaining multiple operations like parsing or validation that might return Option without match/unwrap.
    println!(" you apply a tranformation that may fail");
    println!("if the closure returns option, use and_then");
    
    
    println!("or (STATIC feedback): ");
    // static fallback when you have a backup plan, easy way to say “use this if missing”.
    //  let network_name = Some("SOLANA");
    let network_name: Option<&str> = None;
    let network_updated = network_name
        .and_then(|n| if n.contains("SOLANA") {Some(n)} else {None})
        .or(Some("Polygon"));
    println!("network_updated with or: {:?}", network_updated);

    println!("\nand_then: "); // chaining multiple operations like parsing or validation that might return Option without match/unwrap.
    println!("or_else (LAZY feedback): "); // Use or_else to call a function only if None—a lazy fallback that only computes if needed,
    let network_updated_else = network_name
        .and_then(|n| if n.contains("SOLANA") {Some(n)} else {None})
        .or_else(|| or_else_function(network_name));
    println!("network_updated with or_else: {:?}", network_updated_else);
    
    //NOT USED OFTEN
    //and(opt) = Too limited — and_then() is always more flexible (can return different type, apply logic)
    // xor(opt) = Rarely needed — only useful in weird “only one must be Some” situations, almost never seen in real code.
    // and_then(None → None) & or_else(Some → ignored) : These are already covered by and_then() and or_else() — no need to remember the internals.


    println!("\n\nIterate: ");
    println!("\ninto_iter: ");
    // You want to move out the value when you do not need anymore, turning it into an iterator of 0 or 1 item.
    let protocol_name: Option<&str> = Some("uniswap");
    println!("protocol_name: {:?}", protocol_name.into_iter()); //here is consumed
    

    println!("\niter (more often used with vec, hashmap,...): "); // You want to read the value, Borrows the value inside the Option as &T
    println!("iter_mut (more often used with vec, hashmap,..): "); // You want to modify the value inside the Option safely, Borrows the value inside as &mut T.


    println!("\n\nCollecting infos (not very useful except for writing generic code): "); //collecting into (if any is None => None is returned)
    println!("collect: "); // when you're transforming or validating data and want to stop on first failure. when you want all-or-nothing collection
    println!("sum: "); // when you're aggregating validated inputs (e.g. user scores, parsed data). when You want to add only if all are valid
    println!("product: "); // when you’re computing totals from optional or fallible data sources. when You want to multiply only if all valid


    println!("\n\nModifying in place: ");
    println!("\ninsert: ");//you want to force a new value
    let mut collateral_ratio: Option<f64> = Some(0.75);
    let _ = collateral_ratio.insert(0.85);
    println!("collateral_ratio (inserted): {:?}", collateral_ratio);
    
    println!("\nget or insert: "); //you want to ensure a value is present
    collateral_ratio = None;
    collateral_ratio.get_or_insert(0.34);
    println!("collateral_ratio (got or inserted): {:?}", collateral_ratio);

    println!("\nget or insert with: "); //when value is expensive to compute
    collateral_ratio = None; 
    collateral_ratio.get_or_insert_with(|| get_or_inserted_with_default());
    println!("collateral_ratio (got or inserted with): {:?}", collateral_ratio);
    
    println!("\ntake: "); //NOT OFTEN USED you want to move out the value
    println!("\nreplace: "); //RARELY USED you want to swap the value cleanly
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

//ok_or_else
fn map_default() -> f64 {
    25.0
}

fn or_else_function(network: Option<&str>) -> Option<&str> {
    Some(network.as_ref()).and(Some(" and Ethereum"))
}

fn get_or_inserted_with_default() -> f64 {
    0.92
}


