//================================================= VEC =============================================================================
fn main() {
let mut shopping_cart: Vec<i32> = Vec::new();
let _infer_shopping_cart = [1,2,3].to_vec();

shopping_cart.push(23);
shopping_cart.push(45);
shopping_cart.push(68);
shopping_cart.push(63);

let second_article: &i32 = &shopping_cart[1];
println!("Second article has the value {second_article}");

let third_article: Option<&i32> = shopping_cart.get(2);

match third_article {
    Some(third_article) => println!("Third article has the value {third_article}") ,
    None => println!("Third article does not have any value") ,
}


for article in &shopping_cart{
    println!("View: {article}");
}

for article_to_update in &mut shopping_cart{
    *article_to_update += 10;
    println!("Update: {article_to_update}");
}

for article_to_update in shopping_cart.iter_mut() {
    *article_to_update += 2;
    println!("Iter_mut: {article_to_update}");
}

let shopping_cart = shopping_cart.iter().map(|a| a + 2);
shopping_cart.for_each(|article| println!("Map: {article}"));

#[derive(Clone)]
enum CustomerList{
    Name(String),
    Company(bool),
    InvestorRisk(u32),
    InterestRateAVG(f64),
}

let _customer1 = [
    CustomerList::Name(String::from("Alain")),
    CustomerList::Company(true),
    CustomerList::InvestorRisk(8),
    CustomerList::InterestRateAVG(15.5),
    ].to_vec();
}
