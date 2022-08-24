// Data Types 

// We'll be looking at Primitive Data types - the fundamental ones

// There are 2 categories of data types:

// Scalar - A scalar type represents a single value. Rust has four 
// primary scalar types: integers, floating-point numbers, Booleans, 
// and characters. 

// Compound - Compound types can group multiple values into one type. 
// Rust has two primitive compound types: tuples and arrays.

// Recall, Rust is a statically typed programming language which
// means it must know the types of all the variables when it
// gets to compile time. Despite that sounding like a lot of work, 
// Rust can usually infer the data type if left blank. 


fn main() {

    // Integer Types:
    // i8 has the range -128 to 127
    let x: i8 = 12;

    // This tells you the maximum and minimum of an i32 
    println!("Maximum i32 is {}", std::i32::MAX);
    println!("Maximum i32 is {}", std::i32::MIN);

    // Floating Point-Types:
    // A float is a number with a decimal after it e.g. 10.5
    // Floats can  be f32 or f64 - default is f64
    let y: f32 = 10.5;
    println!("{}", y);

    // Booleans
    // Consists either true or false
    let true_or_false: bool = true;
    println!("{}", true_or_false);

    // Characters
    // These consist of ONE letter/character 
    let character: char = '\u{1F60A}';
    println!("{}", character);


    // COMPOUND TYPES

    // Tuples
    let mut tuple1: (i32, char, bool) = (10, 'e', false);
    println!("{:?}", tuple1);
    // Modifying the elements - need to use 'mut' and same data types
    tuple1.0 = 205;
    println!("{:?}", tuple1);

    // Arrays 
    let mut arr1 = [10, 20, 30];  // all the elements in this array will be i32's
    // Unlike any other programming language, you can't add any extra elements
    // onto this array :( BUT there's something called 'vectors' which
    // allow you to do this and we'll touch upon them later!
    // Modifying the elements
    arr1[0] = 10000;

    // explicitly state the types in our array
    let arr2: [i64; 5] = [1, 2, 3, 4, 5];


}

// Signed (i) and unsigned (u) refer to whether it’s possible for the number 
// to be negative. In other words, whether the number needs to have
// a sign with it (signed) or whether it will only ever be positive
// and can therefore be represented without a sign (unsigned).

// 32 - takes up 32 bits of space 











