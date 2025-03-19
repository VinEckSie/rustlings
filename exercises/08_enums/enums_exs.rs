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

}

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

// #[derive(Debug)]
// struct IpAddr {
//     address: String,
//     addr_type: IpAddrType,
// }