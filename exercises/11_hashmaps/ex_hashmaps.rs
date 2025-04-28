use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Red"), 30);

    let score_team = String::from("Blue");
    let score_blue_team = scores.get(&score_team).copied().unwrap_or(0);

    println!("Blue team is {}\n", score_blue_team);

    //overwriting
    scores.insert(String::from("Red"), 100);
    //adding key only if not exists
    scores.entry(String::from("Blue")).or_insert(200);
    scores.entry(String::from("White")).or_insert(300);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let sentence = "Rust Solana Rust Anchor";
    let mut map = HashMap::new();

    //update based on old value
    for concept in sentence.split_whitespace() {
        let count = map.entry(concept).or_insert(0);
        *count += 1;
    }

    println!("\n{:?}", map);
}
