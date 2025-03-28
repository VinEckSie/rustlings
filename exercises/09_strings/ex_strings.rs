fn main() {
    //strings are utf8 encoded
    let mut sentence = String::from("\nЗдравствуйте");
    let second_word = ", World";
    let second_sentence = String::from("\nDetermined to master Rust");
    let third_sentence = String::from("\nAnd to work full remote");
    
    sentence.push_str(second_word);
    sentence.push('!');
    
    println!("sentence :  {}", sentence);
    println!("second world is still usable because is a string slice: {}", second_word);

    let s3 = sentence + &second_sentence;
    println!("\nfinal sentence :  {}", s3);
    
    //Because format macro implement the Display trait, values are borrowed
    println!("\nFormat: {}", format!("{}, {}", second_sentence, third_sentence));
    
    //Iterate over string
    let hello = "Здравствуйте";
    //Compile error, because 3 takes two bytes (remember bytes,character, grapheme cluster)
    //let t = &hello[0..1];
    let t = &hello[0..2];
    println!("\nByte value and string : {:?}", t);
    
    //Best way to iterate over string is by char or bytes (Grapheme is complex, so not implemented in stdlib, but can find crates for that)
    println!("\nChar loop`;");
    for c in t.chars() {
        println!("{}", c);
    }

    println!("\nByte loop`;");
    for c in t.bytes() {
        println!("{}", c);
    }
    
    
}
