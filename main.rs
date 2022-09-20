// Functions

// The main function is our entry point to our
// program and anything inside here will run.

// We can actually make our own functions!
fn main() {
    first_function();
    addition(2, 3);
    let number = subtraction(10, 9);
    println!("{}", number);

    // Statements and Expressions

    // Statements
    // A statement in Rust performs an action but doesn't
    // return a value. A statement can be a variable
    // declaration or function declaration. 

    // Variable declarations
    let x = 113;  // this is a statement - it doesn't
                  // evaluate anything. 

    // Function declarations
    // declaring a function - fn function(){}


    // Expressions 
    // An expression is anything else in Rust that
    // evaluates something i.e. it returns a value
    // println! macro is an expression 
    // first_function is an expression (function call)
    // 113 is an expression - it evaluates to something! 

    // 'let' is not always a statement:
    let z = {
        let x = 1; // this is a statement - variable declaration
        x + 6  // expression! so don't need ;
    };
    println!("{}", z);


    // Closure 
    let add_numbers = |s:i32, t:i32| s + t;
    println!("Closure sum: {}", add_numbers(3, 6));

    // Super handy because you can use outside variables
    // which you can't do with a standard function
    // because it's block scoped 

    let u: i32 = 15;
    let add_numbers = |s:i32, t:i32| s + t + u;
    println!("Closure sum: {}", add_numbers(3, 6));

}

// Creating our first function (snakecase)
fn first_function(){
    println!("Hello from function 1!");
}

// You HAVE to specify the type of the parameter
fn addition(x: i32, y: i32){
    println!("Addition: {}", x + y);
}

// Returning Values from a Function
// The -> i32 is like a return operator - this illustrates
// the type that will be returned from our function
fn subtraction(x: i32, y: i32) -> i32 {
    x - y // this is an expression so NO semi-colon
}

// You can also use 'return' key word and this works
// with or without a semi-colon












