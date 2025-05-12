// Lifetimes are annotated below with lines denoting the creation
// and destruction of each variable.
// `i` has the longest lifetime because its scope entirely encloses
// both `borrow1` and `borrow2`. The duration of `borrow1` compared
// to `borrow2` is irrelevant since they are disjoint.
fn main() {
    let word1 = String::from("loooong_chain_here");
    let result;

    {
        let word2 = String::from("short_chain_here");
        result = longest(word1.as_str(), word2.as_str());
        println!("{result:?}");
    }

    //println!("{result}"); KO if word2 is of String type because of lifetime rule in longest function signature

    let text = String::from(" All humans are born equals. Just some work harder.");
    let quote = text.split(".").nth(1).unwrap_or_else(|| "no quote");
    let my_note = Notes { quote };

    println!("{quote}");
}

fn longest<'a>(first_num: &'a str, second_num: &'a str) -> &'a str {
    if first_num.len() > second_num.len() {
        first_num
    } else {
        second_num
    }
}

//if the return type has a lifetime, one of the parameters must have too otherwise dangling reference about the local var returned
// fn longest_local<'a>(first_num: &str, second_num: &str) -> &'a str {
//     let result = String::from("dangling reference");
//     result.as_str()
// }

//struct def
struct Notes<'a> {
    quote: &'a str,
}
