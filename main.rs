// Control Flow: if, else if, while, etc. 

fn main() {

    // Conditions
    // any expression that evaluates
    // to true or false (boolean data type)
    // You can create conditions using operators such
    // as < <= > >= != == (conditional operators)

    let condition1 = 5 > 7;
    println!("{}", condition1);

    // both types must be the same - you can use
    // type casting to counteract this
    let condition2 = (5 as f32) < 7.2;
    println!("{}", condition2);

    // Compound Conditions
    // Multiple conditions changed together using 
    // logical operators 
    // and &&
    // or ||
    // not ! - negates the boolean 
    // the ordering: not, and, or

    let condition3 = true || !condition2;
    println!("{}", condition3);  

    // Control Flow - If, Else if, Else Statement
    let number = 100;

    // If statement
    if number == 10 {
        println!("The number is 10!")
    }
    else if number == 100 {
        println!("The number is 100!")
    }
    else {
        println!("The number is not 10 or 100!")
    }

    // Short-hand control flow
    let age = 15;
    let teenager = if age >= 13 && age < 18 {true} else {false};
    println!("{}", teenager);


    // Loops
    // These are used to iterate until a condition is met

    // Infinite Loop
    // let mut count = 0; 
    // loop{
    //     println!("Number count is: {}", count)
    //     count += 1;
    // }

    // To make the infinite loop stop, we add a condition
    let mut count = 0; 
    loop{
        println!("Number count is: {}", count);
        count += 1;
        if count == 10 {
            break;
        };
    }

    // While loop 
    count = 0; 
    while count <=100 {
        if count % 15 == 0 {
            println!("Fizzbuzz");
        }
        else if count % 3 == 0 {
            println!("Fizz");
        }
        else if count % 5 == 0 {
            println!("Buzz");
        }
        else {
            println!("{}", count)
        }

        // Increment
        count += 1;
    }

    // For Range Loop
    for x in 0..10 { // this will loop from 0 to 9
        println!("{}", x);
    }

    // Iterating Loop
    let a = [10, 20, 30, 40];
    for element in a.iter(){
        println!("value is {}", element);
    }


}
