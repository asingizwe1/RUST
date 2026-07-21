//pattern matching is done with match, if let and while let

fn main() {
    let number = 2;

    match number {
        1 => println!("one"),
        2 => println!("2"),
        _ => println!("nill"),
    }

    //matching enums
    enum Direction {
        Left,
        Right,
    }

    let move_to = Direction::Left;

    match move_to {
        /// if value is, then print
        Direction::Left => println!("valid"),
        Direction::Right => println!("invalid"),
        //you must include all cases
        _ => println!("null"),
    }
}
