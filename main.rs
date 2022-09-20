// Overflow, Type Casting and Conversion

fn main() {
    // If you assign a variable to a given data 
    // type but that number is out of the range
    // for that data type then you will get an 
    // overflow! 

    // Basic maths
    let a: f32 = 93.0; // 0 to 225
    let b: f32 = 9.0;  // 0 to 225

    // What would happen if we added an i8 and a u8?
    // We would get an error because they're both the
    // same type! 
    let c = a + b;
    // println!("{}", c)

    // What about other operators?
    // If the result of an operation between two
    // variables (of the same data type) exceeds
    // the range of that specific data type then
    // we get an overflow - we don't want this!  
    // let d = a * b;
    // println!("{}", d);

    // Typically we don't want to be overflowing.
    // To account for this, make variables larger
    // bits. 

    // Division 
    // dividing 93 by 9 - expect to get 10 plus change
    // expect 10.33333
    let e = a / b;
    println!("{}", e);


    // Type Casting and Conversion

    // How you go about adding values of different
    // types. A lot of the time in Rust, you won't 
    // actually specifically state the type of variable
    // you're using. 

    // Assigning types
    // 1)
    let x1: u8 = 9; 

    // 2)
    let x2 = 9_u8;  // treating 9 as a u8 data type

    // 3)
    let x3 = 9 as u8;  // also how you do explicit type conversion

    let x = 113.0 as f64;
    let y = 5_i8;
    let z = x / y as f64; // casting y into an f64
    println!("{}", z);

    // when performing type casting, make sure to take
    // the number with the least number of bits and 
    // change it to the type with the most number of
    // bits - this helps avoid errors and overflow

    // You have to be careful with Rust because sometimes
    // overflow occurs and rust doesn't catch it. 

}