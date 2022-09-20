// Vectors

// Recall, you cannot add to arrays like you can 
// in other programming languages - this makes arrays
// hard to deal with. Instead, we deal with vectors!

fn main() {
    // vectors are resizeable arrays
    // vectors can only store values of the same type! 
    let mut vector1: Vec<i32> = vec![1, 2, 3, 4];

    // Create a new, empty vector
    // We need to state the data type of an empty
    // array because Rust doesn't know what elements
    // we may eventually store in it
    let vector2: Vec<i32> = Vec::new();

    // Operations

    // Reassign values
    vector1[0] = 20;
    println!("{:?}", vector1);

    // Specific element within the vector
    println!("{}", vector1[1]);

    // Length of a vector
    println!("{}", vector1.len());

    // Add a specific value to a vector
    vector1.push(10); // this adds 10 on the end
    println!("{:?}", vector1);

    // Remove the last entry from a vector
    vector1.pop();
    println!("{:?}", vector1);

    // Looping - loops through all the values in your vector
    for element in vector1.iter(){
        println!("{}", element)
    }

    // Looping and Mutating
    for y in vector1.iter_mut(){
        *y += 10;  // * - dereference operator 
        println!("{}", y)
    }
    





   
}
