
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    signin_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User{
        username,
        email,
        active: true,
        signin_count: 555,
    }
}



fn main() { 
    
    let mut usera = User {
        username: String::from("vincent"),
        email: String::from("vescapital@pm.me"),
        active: true,
        signin_count: 5,
    };

    usera.username = String::from("new name");

    let userb = build_user(String::from("jean"), String::from("jdup@gmail.com"));

    let userc = User {
        username: String::from("userc"),
        email: String::from("ves@pm.me"),
        ..userb     
    };

    //let userd = build_user(userb.username, String::from("jdup@gmail.com"));

    println!("{:?}",userb);
    println!("{:?}",userc);
}