// Enums

// Structs and enums are the building blocks for 
// creating new types in rust

// Enums allow us to enumerate a list of variants
// but you may be wondering, when is it
// appropriate to use enums over structs?

// Let's consider IP Addresses - we can enumerate
// all the variants - there's only 2! V4 or V6

// enum for the IP address kind
enum IpAddressKind{
    // to store actual data inside your enum,
    // you just write () and whatever data you
    // want inside
    V4(u8, u8, u8, u8), // 4 bit integers 
    V6(String),
}

// Enum variants can store a wide variety of types:
enum Message{
    Quit, // a variant that stores no data
    Move {x: i32, y: i32}, // stores anonymous struct
    Write(String) // single string
    ChangeColour(i32, i32, i32) // 3 integers
}

// The above could have been written as structs
// but the issue is that they are all of DIFFERENT
// types. 

// The benefit of an enum is that all variants 
// are grouped under the same message type
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct


// The IP Address struct allows us to group
// the version of the IP Address with the 
// actuall address!
struct IpAddress{
    kind: IpAddressKind, // this is our enum
    address: String, 
}

// We can also make methods and associated functions
// in enums just like we did with structs!
impl Message{
    fn function1(){
        println!("Code of the Future!");
    }
}

fn main() {
    // we can create instances for each of our
    // variants
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    // So we could create a function that will take
    // in our enum type so either 4 or 6. We can
    // do this because they're both the same type!

    fn route(ip_kind: IpAddressKind){
        // input something in here 
    };

    // let local_host = IpAddress{
    //     kind: IpAddressKind::V4,
    //     address: String::from("127.0.0.1"),
    // };

    let local_host = IpAddressKind::V4(127, 0, 0, 1);





}






