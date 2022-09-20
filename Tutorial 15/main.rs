// Option enum

// Lots of programming languages have null values
// which represent a very useful concept - a value
// can either exist or it can be null i.e. there
// is no value 

// The problem is that the type system can't
// guarantee that, if you use a value, it's not
// null. In Rust, there are NO null values. Instead, 
// we have the option enum

enum Option<T> {
    None,  // stores no value
    Some(T),  // stores some value in here 
}

enum Movement{
    // Variants
    Left, 
    Right,
    Jump,
}

// using the match expression
fn move_player(m: Movement){
    // Perform action depending on information
    match m {
        Movement::Left => println!("Moving left"),
        Movement::Right => println!("Moving right"),
        Movement::Jump => println!("Jumping!")
    }

}

fn main(){
    let number = Some(10);
    let boolean = Some(true);
    // since we are passing the None type, we 
    // need to annotate the type - Rust can't
    // infer it. 
    let nothing: std::option::Option<i32> = None;
    // this can either be 10 or it can be nothing
    let x: std::option::Option<i32> = Some(10);

    // Addition
    // We can't do this as they're not the same
    // data type!
    let y: i32 = 14;
    // let sum = x + y;
    // println!("{}", sum);

    // instead we can use unwrap
    // this unwraps the value under x and if 'None'
    // is passed then it will return 0. 
    let sum = y + x.unwrap_or(0);


    // Match expression
    // We have defined an enum above called movement
    let player1 = Movement::Jump;
    let player2 = Movement::Right;
    let player3 = Movement::Left;

    move_player(player1);
    move_player(player2);
    move_player(player3);


}






