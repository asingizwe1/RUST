//option enum - represents that something may happen or not
use std::env;
// Some(value)
// None
fn main() {
    let some_number: Option<i32> = Some(10);
    let no_number: Option<i32> = None;
    let maybe_number: Option<i32> = Some(5);
    //maybe number of type Option<i32> = Some(5)
    println!("Some number: {:?}", some_number);
    println!("No number: {:?}", no_number);
    match maybe_number {
        Some(n) => println!("We got a number: {}", n),
        None => println!("No number found"),
    }

    //APPLICATIONS-> especially when handling values like inputs or existence of values of variables
    //searching for a value
    let numbers = vec![1, 2, 3, 4];

    // find returns Option<&T>
    let found = numbers.iter().find(|&&x| x == 3);

    match found {
        Some(n) => println!("Found: {}", n),
        None => println!("Not found"),
    }

    //USER input
    let input = "42";

    // parse returns Result, but we can map it to Option
    let number: Option<i32> = input.parse().ok();
    println!("Parsed: {:?}", number); // Some(42)
                                      //if parsing fails it returns None instead of crashing

    //Working with environment variables
    match env::var("HOME").ok() {
        Some(path) => println!("Home directory: {}", path),
        None => println!("HOME not set"),
    } // it will return None if no value found and wont crush but will return Some(exact path if value found)
}
//to remove value from some you can use
//1 match
// rust
// fn main() {
//     let maybe_number = Some(10);

//     match maybe_number {
//         Some(n) => println!("Got the number: {}", n),
//         None => println!("No number"),
//     }
// }

// or
// //unwrap
// fn main() {
//     let x = Some(5);
//     println!("{}", x.unwrap()); // prints 5
// }
