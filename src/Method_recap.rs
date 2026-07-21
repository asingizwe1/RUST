struct Rectangle{
width:u32,
length:u32,
}

impl Rectangle{// in the implement block of rectangle
// in this impl block is where we put functions relating to the Struct
fn area(&self)->u32{
self.width*self.length
}
}
trait Speak {
    fn speak(&self)-> String;//trait requires returning a string
}
// structs will implement the trait
// Implement trait for a struct
struct Dog;
struct Cat;
//the structs will use the Impl block to implement the trait methods
impl Speak for Dog{
fn speak(&self)-> String{
String::from("bark")
}

}


fn main() {
    let rect1 = Rectangle { width: 30, length: 50 };
    let rect2 = Rectangle { width: 10, length: 40 };
let d = Dog;
    println!("Area: {}", rect1.area());
 println!("Dog: {}", d.speak());
}