// ENUMS
// you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example
// You can define methods on enums
// You can put data inside the variants
// You can define different types of data for different variants

// 0. Enum is a way to define a type by enumerating its possible values
// 1. Defining an Enum
/*
enum IpAddrKind {
    V4,
    V6,
}
*/

fn main() {

    // 2. Using the Enum Values
    /*
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    */

    // 4. Using the function route
    //route(IpAddrKind::V4);
    //route(IpAddrKind::V6);

    // 6. Using the IpAddr enum with data  
    //let home = IpAddr::V4(127, 0, 0, 1);
    //let loopback = IpAddr::V6(String::from("::1"));

    // 9. Using the Message enum
    let m = Message::Write(String::from("hello")); 
    println!("{:?}", m);
    //m.call();
}

// 3. Define a function that takes any IpAddrKind
/*
fn route(ip_type: IpAddrKind) {
    // Code here
}
*/

// 5. Enums with data
/*
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
*/

// 7. Enums with variants
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
/* NOTE: 
Defining an enum with variants is similar to defining different kinds of struct definitions, except the enum doesn’t use the struct keyword and all the variants are grouped together under the Message type. The following structs could hold the same data that the preceding enum variants hold:
   struct QuitMessage; // unit struct
   struct MoveMessage {
        x: i32,
        y: i32, 
    }
   struct WriteMessage(String); // tuple struct
   struct ChangeColorMessage(i32, i32, i32); // tuple struct 
*/
/* But if we used the different structs, which each have their own type, we couldn’t as easily define a function to take any of these kinds of messages as we could with the Message enum */

// 8. Defining methods on an enum
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}


