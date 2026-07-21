#[derive(Debug)]
//you are automatically telling the compiler to generate an implementation of the Debug trait for your struct.
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point3D(i32, i32, i32);
//just like tuples

//empty stuct
struct AlwaysEqual;

fn main() {
    //The Debug trait allows your type to be printed using the {:?} or {:#?} format specifiers in println!.
    let p = Point3D(2, 4, 6);
    p.0;

    let p2 = Point { x: 10, y: 20 };
    println!(":?", p2);

    let empty = AlwaysEqual; // how we initialise an empty stryct
}
