// Printing and Formatting

fn main() {

    // Basic formatting
    println!("My name is {} and I love {}!", "John", "glasses");
    // You can't simply write println!(4); - it doesn't work!
    println!("{}", 4);

    // Place holders
    println!("My best friend is {0}. {0} loves {1}.", "Victoria", "accounting");

    // Incorporating arguments
    println!("My name is {name} and I am {age}", name="Ellie", age=22);

    // Tuples - prints multiple values rather than a single one!
        println!("{:?}", ("Ellie", 22, true));

    // Basic mathematics! 

    // Produces the binary equivalent of an integer
    println!("Binary: {:b}", 10);

    // Addition
    println!("12 + 3 = {}", 12 + 3);

}

// Tidying up your code - rustfmt 
// First cd into src 
// Then type rustfmt main.rs and your file is automatically formatted











