
fn main() {
    // TODO: Fix the code to print "Hello world!".
    //println!("Hello world!");
    let sentence = String::from("Hello World");

    let first_word = first_word(&sentence);

    println!("First word of sentence '{}' is '{}'", sentence, first_word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for(i,&item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
