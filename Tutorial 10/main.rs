// User/Console Input 

// Importing 'input output'
use std::io;
// use - use a specific library/package/crate
// std - standard library
// io - module we need to get user input 
// :: - path separator operator - allows us to
// go into a package and then into a module we want
// We can then go from that module to a specific
// function/method. 

// Looking at collecting user input 

fn main() {
    // this returns an empty string 
    let mut input = String::new();
    // String is a type in Rust standard library
    // ::new() - new is a function of the String type

    // Collecting user input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    // stdin gives us a handle to the standard input of 
    // the current process - the terminal where you'll input your word
    // read_line - takes in the users input and appends
    // it to the specified buffer which in our case is a string
    // .expect()
    // read_line returns a result type - either returns Ok
    // or an error. Rust is making us account for if an 
    // error occurs - forces us to handle an error case 
    println!("{}", input);
}









