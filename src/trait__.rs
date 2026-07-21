/*fn print_value<T>(value: T) {
    println!("{}", value);
} */
//if you write this generic function rust will complain
// because not every type can be printed with {}
//rust needs to know that does T know how to display itself
//that capability comes from display trait
use std::fmt::Display;
fn print_value<T: Display>(value: T) {
    //T:Display -> it means that T must implement the display trait
    //any type that implements Display is accepted
    println!("{}", value);
}
fn main() {
    print_value(10);
    print_value("Hello");
}
