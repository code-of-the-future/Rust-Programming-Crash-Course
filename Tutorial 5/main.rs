// Variables 

fn main() {
    // This is how you create a variable

    // implicit
    let x = 5;
    // Recall that Rust is statically and strongly typed - when you
    // define a variable, Rust will automatically give it a type
    // this is known as 'implicit' assignment

    // x = "Ellie" - you cannot do this as "Ellie" is not an integer


    // This creates a variable, y, AND assigns a type to it
    // explicit
    let y: u32 = 10;

    println!("x is: {}", x);
    println!("y is: {}", y);


    // By default, in Rust, all variables that are assigned are
    // immutable - we cannot change them!

    // Creating a MUTABLE variable
    // When using 'mut' - the variable (z) you wish to change, must be reassigned
    // to the SAME type! 
    let mut z = 10;
    println!("{}", z);
    z = 99;
    println!("{}", z);

    // Another way of reassigning variables - writing 'let' twice!
    // The variable can be reassigned to a DIFFERENT type! 
    let a = 100;
    println!("{}", a);
    let a = 50.5;
    println!("{}", a);


    // You can assign multiple variables at once!
    let (name, age, hobby) = ("Ellie", 22, "coding");
    println!("My name is {} and I am {}. I love {}", name, age, hobby);


    // CONSTANTS
    // A constant is immutable - it CANNOT be changed and we cannot use the
    // mut command here. Once it's there, it can't be changed!
    const SECONDS: i32 = 001;
    println!("{}", SECONDS);


    // SHADOWING 

    let mut b = 3;
    println!("{}", b);

    // {} are known as 'scopes'
    // You can use b from the exterior scope in the interior scope without
    // affecting the value of b in the exterior scope 
    {
    let b = 100000000;
    println!("{}", b);
    }

    b = b + 100;
    println!("{}", b);

}












