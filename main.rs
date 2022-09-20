// Command Line Arguments

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = args[1].clone();

    println!("{:?}", args);
    println!("{}", input);

    if input == "hello" {
        println!("Hey, what's up?")
    }
}
