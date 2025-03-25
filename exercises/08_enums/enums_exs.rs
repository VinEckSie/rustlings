use std::net::Ipv6Addr;

fn main() {
    println!("\nEnums: ");
    let four_addr = IpAddrType::V4(127, 0, 0, 1);
    let six_addr = IpAddrType::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    

    println!("{four_addr:?}");
    println!("{six_addr:?}");

    println!("\nMessage enum: ");

    println!("{:?}", Message::new_quit());
    println!("{:?}", Message::new_move(4, 4));
    println!("{:?}", Message::new_write(String::from("loadin")));
    println!("{:?}", Message::new_changecolor_hsv(30, 50, 30));

    println!("{:?}", Message::from_command("Quit", None, None));
    println!(
        "{:?}",
        Message::from_command("Move", Some((34, 34, 0)), None)
    );
    println!(
        "{:?}",
        Message::from_command("Write", None, Some(String::from("processing")))
    );
    println!(
        "{:?}",
        Message::from_command("ChangeColor", Some((255, 120, 122)), None)
    );

    let _some_number = Some(43);
    let _some_char = Some('r');
    let _absent_number: Option<u16> = None;

    //this willl not compile, because they are considered as different types
    // let num1: i8 = 5;
    // let num2: Option<i8> = Some(5);
    // let result = num1 + num2;

    //NOTE: everywhere you expect that a null value is possible >>> USE OPTION<>

    //regular version
    println!("\nEnums advanced: ");
    let one_coin = Coin::Penny;
    let coin_value = sort_that_coin(one_coin);
    println!("{coin_value:?}");

    //shortened version
    println!(
        "{:?}",
        sort_that_coin(Coin::Quarter(String::from("California")))
    );
    println!(
        "{:?}",
        sort_that_coin(Coin::Quarter(String::from("Alabama")))
    );

    let new_value: Option<u8> = increase_coin_value(coin_value);
    println!("{new_value:?}");

    //USE LET when you want to manage one pattern and not all the others

    //✅ Use let ... else when:
    // You need to destructure a value and handle failure immediately.
    // You want to avoid extra indentation (compared to match or if let).
    // You need a clean, readable way to exit early.

    // ❌ Don’t use let ... else when:
    // You need multiple conditions (Use match).
    // You need to modify the extracted value before using it (Use match).

    //panic if one_coin is Eighty
    println!("\nLet else: ");
    let Some(value) = new_value else {
        panic!("Error");
    };

    println!("Let ok, value {value}");

    //Variable scope
    println!("\nVariable scope: ");
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");

    //
    println!("\nOr Operator (better for few values) and range (better for a lot of values but works only on chars and numerics): ");
    let letter = 'l';

    match letter {
        'a'..='k' => println!("first part of alphabet"),
        'l' | 'm' => println!("l ou m"),
        'n'..='z' => println!("second part of alphabet"),
        _ => println!("This is not a letter"),
    }

    //Structs destructuring
    println!("\nStruct destructuring: ");
    struct Point {
        x: u8,
        y: i8,
    }

    let p = Point { x: 5, y: 10 };
    let Point { x, y } = p;

    println!("{x:?} {y:?}");

    match p {
        Point { x: 0, y: 12 } => println!("y axis"),
        Point { x: 3, y: 0 } => println!("x axis"),
        other => println!("On neither axis {x} {y}"),
    }

    println!("\nEnum destructuring: ");
    let message = Message::Move { x: 23, y: 34 };

    match message {
        Message::Quit => println!("Quit"),
        Message::Move { x: _, y: _ } => println!("Move"),
        Message::Write(_) => println!("Write"),
        //.. bc quicker than having to list r: _, g: _, etc.
        Message::ChangeColor(Color::Rgb(..) | Color::Hsv(..)) => println!("ChangeColor"),
    }

    println!("\nTuple and structs destructuring: ");
    let ((altitude, latitude), Point { x, y }) = ((143, 165), Point { x: 55, y: 75 });
    println!("Altitude: {altitude:?}, Latitude: {latitude:?}, x: {x:?}, y: {y:?}");

    println!("\nIgnoring some variables (useful to keep in signature with traits): ");
    get_x_coordonate(34, 44);

    let y: Option<u8> = None;

    match y {
        Some(_) => println!("y is not null"),
        _ => println!("Partial coordonates"),
    }

    println!("\nIgnoring remaining parts: ");
    let number_list = (1, 2, 3, 4, 5, 6, 7, 8, 9);

    match number_list {
        (first, .., last) => println!("First: {first}, Last: {last}"),
    }

    println!("\nMatch guard: ");
    let x = Some(20);
    let y = Some(20);

    match x {
        Some(num) if (0..=30).contains(&num) && num % 2 == 0 && Some(num) == y => {
            println!("Even between 0 and 30 and equal to y: {num}")
        }
        Some(num) if num % 2 == 0 && Some(num) == y => println!("Even equal to y: {num}"),
        Some(num) if num % 2 == 0 && Some(num) != y => {
            println!("Even different to y: {num}, {y:?}")
        }
        Some(num) => println!("Odd {num}"),
        None => (),
    }

    println!("\nBindings: ");
    let coords = Point { x: 10, y: 16 };

    match coords {
        Point {
            x: _,
            y: yvar @ 0..=20,
        } => println!("y between 0 and 20: {yvar}"),
        Point { x: _, y: 21.. } => println!("y over 20"),
        Point { .. } => println!("y negative"),
    }
}

// #[derive(Debug)]
// struct IpAddr {
//     address: String,
//     addr_type: IpAddrType,
// }

#[derive(Debug)]
enum IpAddrType {
    V4(u8, u8, u8, u8),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Color {
    Rgb(u8, u8, u8),
    Hsv(u8, u8, u8),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move { x: u8, y: u8 },
    Write(String),
    ChangeColor(Color),
}

impl Message {
    fn new_quit() -> Self {
        Message::Quit
    }

    fn new_move(x: u8, y: u8) -> Self {
        Message::Move { x, y }
    }

    fn new_write(text: String) -> Self {
        Message::Write(String::from(text))
    }

    fn new_changecolor_rgb(r: u8, g: u8, b: u8) -> Self {
        Message::ChangeColor(Color::Rgb(r, g, b))
    }

    fn new_changecolor_hsv(h: u8, s: u8, v: u8) -> Self {
        Message::ChangeColor(Color::Hsv(h, s, v))
    }

    fn from_command(command: &str, args: Option<(u8, u8, u8)>, text: Option<String>) -> Self {
        match command {
            "Quit" => Message::Quit,
            "Move" => {
                if let Some((x, y, _)) = args {
                    Message::Move { x, y }
                } else {
                    Message::Move { x: 5, y: 5 }
                }
            }
            "Write" => Message::Write(text.unwrap_or("Default".to_string())),
            "ChangeColor" => {
                if let Some((r, g, b)) = args {
                    Message::ChangeColor(Color::Rgb(r, g, b))
                } else {
                    Message::ChangeColor(Color::Rgb(255, 255, 255))
                }
            }
            _ => Message::Quit,
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(String),
    Fifty,
    Eighty,
}

fn sort_that_coin(coin: Coin) -> Option<u8> {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            Some(1)
        }
        Coin::Nickel => Some(5),
        Coin::Quarter(state) => {
            println!("State from this coin is {state}");
            Some(25)
        }
        Coin::Fifty | Coin::Eighty => {
            println!("This coin do not exist");
            None
        }
        _ => None,
        //_ => (), if it was not a function
    }
}

fn increase_coin_value(value: Option<u8>) -> Option<u8> {
    match value {
        Some(coin) => Some(coin + 5),
        None => None,
    }
}

fn get_x_coordonate(x: u8, _: u8) {
    println!("X coordonate: {x:?}");
}


