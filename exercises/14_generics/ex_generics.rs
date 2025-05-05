use std::fs::File;
use std::io::{read_to_string, ErrorKind, Read};
use std::{fs, io};
use std::error::Error;
use std::fmt::{Debug, Display};

// fn main() -> Result<(), Error> {
fn main() {
    //generics
    //monomorphization; process of turning generic code to specific code

    //function
    let number_list = vec![34, 50, 125, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {result:?}");

    let char_list = vec!['f', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {result:?}");

    //struct
    let coords_1 = PointSimpleGeneric { x: 1, y: 2 };
    let coords_3 = PointSimpleGeneric { x: 5.0, y: 10.0 };
    // let coords_2 = PointSimpleGeneric { x: 2, y: 3.0 }; //KO because this struct only afford 1 type as generics

    let coords4 = PointDoubleGeneric { x: 5.0, y: 4 };
    let coords5 = PointDoubleGeneric { x: 3, y: 2.0 }; //OKs because this is a double generic struct
    println!("If you’re finding you need lots of generic types in your code, \nIt could indicate that your code needs restructuring into smaller pieces.");
    println!("DoubleGeneric, get_x(): {}", coords5.get_x());

    //generic methods from generic struct
    let coords_u32 = PointSimpleGeneric{x: 65, y: 560};
    let coords_u8 = PointSimpleGeneric{x: -1, y: -1};
    println!("coords_u32 y : {}",coords_u32.get_y());
    println!("coords_u32 x : {}",coords_u32.get_x());
    // let y_u8 = coords_u8.get_y();//not working, because get_y() implemented only for i32 type

    let new_double_coords = coords4.mix(coords5);
    println!("new double coords: {:#?}", new_double_coords);
}

//Generics part (with where clause for bounds)
fn largest<T>(list: &Vec<T>) -> io::Result<&T>
where
    T: PartialOrd + Debug + Display,
{
    if list.is_empty() {
        return Err(io::Error::new(io::ErrorKind::Other, "Empty list"));
    }

    let mut largest = &list[0];

    for item in list.into_iter() {
        if item > largest {
            largest = item;
        }
    }

    Ok(largest)
}

struct PointSimpleGeneric<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct PointDoubleGeneric<T,U> {
    x: T,
    y: U,
}

//inline bound
impl<T: Debug> PointSimpleGeneric<T>{
    fn get_x(&self) -> &T {
        &self.x
    }
}

//filter a simple generic on a tyep
impl PointSimpleGeneric<u32>{
    fn get_y(&self) -> &u32{
        &self.y
    }
}

//implement a constructor and a get on a double generic
impl<T, U> PointDoubleGeneric<T, U>{
    fn new(x: T, y: U) -> Self {
        PointDoubleGeneric{x, y}
    }

    fn get_x(&self) -> &T{
        &self.x
    }

    //mix up generic from the struct origin with a new point with other generics
    fn mix<T1, U1>(self, other_point: PointDoubleGeneric<T1, U1>) -> PointDoubleGeneric<T,U1> {
        PointDoubleGeneric{
            x: self.x,
            y: other_point.y,
        }
    }
}

//filter on one type for the double generic
impl<T> PointDoubleGeneric<T, f64>{
    fn get_y(&self) -> &f64{
        &self.y
    }
}
