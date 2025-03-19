
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
    //Regular structs
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

    println!("Structs: {:?}",userb);
    println!("Structs: {:?}",userc);


    //Tuple structs
    #[derive(Debug)]
    struct Color(u32,u32,u32);

    #[derive(Debug)]
    struct Point(u32,u32,u32);

    let blue = Color(0,0,0);
    let x = Point(12,33,33);

    println!("Tuple structs: {:?}",blue);
    println!("Tuple structs: {:?}",x);


    //Unit structs
    #[derive(Debug)]
    struct Equal;

    let use_equal = Equal;

    println!("Tuple with no fields: {:?}",use_equal);


    //Structs methods
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }

        //Struct methods with parameters
        fn can_contain(&self, other_figure: &Rectangle) -> bool {
            self.area() > other_figure.area()
        }

        //Structs associated methods    
        fn build_square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }

    let first_figure = Rectangle {height:50, width:30};
    let second_figure = Rectangle {height:40, width:20};
    let third_figure = Rectangle {height:60, width:40};
    
    println!("Struct methods: {:?}",first_figure);
    println!("Struct methods, area: {:?}",first_figure.area());
    println!("Struct methods, can contain 1 vs 2: {:?}",first_figure.can_contain(&second_figure));
    println!("Struct methods, can contain 1 vs 3: {:?}",first_figure.can_contain(&third_figure));
    println!("Struct method, building square, area: {:?}", Rectangle::build_square(30).area());
    
}