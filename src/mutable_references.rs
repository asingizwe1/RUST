fn main() {
    let mut cord = (1, 3);
    let point = &mut cord.0;

    // point = 20; - wrong when ever we have areference we must first dereference
    *point = 20; //dereference point since its pointing to cord
    println!("{:?}", cord);
}
