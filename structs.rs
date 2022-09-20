// Structs #1 

// Structs in Rust are similar to classes in other
// programming languages. 
// Structs are similar to tuples in that you can hold
// multiple values and they can be different types
// Unlike tuples, in a struct you name each piece 
// of data so you know what the values mean 
// Structs are more flexible than tuples. 

// Traditional Structs
struct Colour {
    red: u8, // u8 has range 0 to 255 
    green: u8, 
    blue: u8,
}

// Tuple Struct
struct Colour2(u8, u8, u8);

fn main() {
    // to use a struct after we've defined it, we
    // create an instance of that struct by
    // specifying concrete values for each of the fields
    let mut colour_option1 = Colour {
        red: 0,
        green: 255,
        blue: 0,
    };
    // you don't need to specify the fields in the
    // same order in which we declared them in 
    // the struct. 
    // The struct definition is a general template
    // for the type and instances fill in that
    // template with particular data. 
    
    // Changing these values/properties
    colour_option1.blue = 200;
    colour_option1.red = 20;

    // access values in colour_option1 instance
    println!("Colour: {}, {}, {}", 
        colour_option1.red,
        colour_option1.green,
        colour_option1.blue);

    // Tuple Struct
    let mut colour_option2 = Colour2(0, 13, 231);
    println!("Colour: {}, {}, {}", 
        colour_option2.0,
        colour_option2.1,
        colour_option2.2);

}
