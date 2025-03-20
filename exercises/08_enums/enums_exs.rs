use std::net::Ipv6Addr;

fn main() {
    let four_addr = IpAddrType::V4(127,0,0,1);
    let six_addr = IpAddrType::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));
    

    println!("{:?}",four_addr);
    println!("{:?}",six_addr);


    println!("{:?}",Message::new_quit());
    println!("{:?}",Message::new_move(4,4));
    println!("{:?}",Message::new_Write(String::from("loading")));
    println!("{:?}",Message::new_changecolor(3,3,4));

    println!("{:?}",Message::from_command("Quit", None, None));
    println!("{:?}",Message::from_command("Move", Some((34,34,0)), None));
    println!("{:?}",Message::from_command("Write", None, Some(String::from("processing"))));
    println!("{:?}",Message::from_command("ChangeColor", Some((255,120,122)), None));

    let some_number = Some(43);
    let some_char = Some('r');
    let absent_number: Option<u16> = None;

    //this willl not compile, because they are considered as different types
    // let num1: i8 = 5;
    // let num2: Option<i8> = Some(5);
    // let result = num1 + num2;

    //NOTE: everywhere you except a null value is possible >>> USE OPTION<>

    //regular version
    let one_coin = Coin::Penny;
    let coin_value = sort_that_coin(one_coin);
    println!("{:?}",coin_value);

    //shortened version
    println!("{:?}",sort_that_coin(Coin::Quarter(String::from("California"))));
    println!("{:?}",sort_that_coin(Coin::Quarter(String::from("Alabama"))));

    let new_value: Option<u8> = increase_coin_value(coin_value);
    println!("{:?}",new_value);

    //USE LET when you want to manage one pattern and not all the others
    
    //✅ Use let ... else when:
        // You need to destructure a value and handle failure immediately.
        // You want to avoid extra indentation (compared to match or if let).
        // You need a clean, readable way to exit early.

    // ❌ Don’t use let ... else when:
        // You need multiple conditions (Use match).
        // You need to modify the extracted value before using it (Use match).
    
    //panic if one_coin is Eighty
    let Some(value) = new_value else {
        panic!("Error");
    };

    println!("Let ok, value {}", value);
}

// #[derive(Debug)]
// struct IpAddr {
//     address: String,
//     addr_type: IpAddrType,
// }

#[derive(Debug)]
enum IpAddrType {
    V4(u8,u8,u8,u8),
    V6(Ipv6Addr),
}

#[derive(Debug)]
enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32,i32,i32),
}

impl Message {
    fn new_quit() -> Self {
        Message::Quit
    }

    fn new_move(x: i32, y: i32) -> Self {
        Message::Move {x,y}
    }

    fn new_Write(text: String) -> Self {
        Message::Write(String::from(text))
    }

    fn new_changecolor(r: i32, g: i32, b: i32) -> Self {
        Message::ChangeColor(r,g,b)
    }

    fn from_command(command: &str, args: Option<(i32,i32,i32)>, text: Option<String>) -> Self {
        match command {
            "Quit" => Message::Quit,
            "Move" => if let Some((x,y,_)) = args {
                Message::Move {x,y}
            } else {
                Message::Move {x: 5,y: 5}
            }
            "Write" => Message::Write(text.unwrap_or("Default".to_string())),
            "ChangeColor" => if let Some((r,g,b)) = args {
                Message::ChangeColor(r,g,b,)
            } else {
                Message::ChangeColor(255,255,255)
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
        Coin::Dime => Some(10),
        Coin::Quarter(state) => {
            println!("State from this coin is {}",state);
            Some(25)
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

