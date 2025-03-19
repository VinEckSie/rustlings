struct ColorRegularStruct {
    red: u8,
    green: u8,
    blue: u8,
    // TODO: Add the fields that the test `regular_structs` expects.
    // What types should the fields have? What are the minimum and maximum values for RGB colors?
    //u8 because 0 and 255
}

struct ColorTupleStruct(u8,u8,u8);

#[derive(Debug)]
struct UnitStruct;

fn main() {
    // You can optionally experiment here.
    regular_structs();
    tuple_structs();
    unit_structs();
}


    fn regular_structs() {
        // TODO: Instantiate a regular struct.
        let green = ColorRegularStruct{
            green: 255,
            blue: 0,
            red: 0,
        };

        assert_eq!(green.red, 0);
        assert_eq!(green.green, 255);
        assert_eq!(green.blue, 0);
    }

    fn tuple_structs() {
        // TODO: Instantiate a tuple struct.
        let green = ColorTupleStruct(0,255,0);

        assert_eq!(green.0, 0);
        assert_eq!(green.1, 255);
        assert_eq!(green.2, 0);
    }

    fn unit_structs() {
        // TODO: Instantiate a unit struct.
        let unit_struct = UnitStruct;
        let message = format!("{unit_struct:?}s are fun!");

        assert_eq!(message, "UnitStructs are fun!");
    }

