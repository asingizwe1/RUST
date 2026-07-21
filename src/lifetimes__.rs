fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //"Give me two borrowed strings that both live at least as long as 'a, and I’ll return a borrowed string that’s safe for as long as 'a."
    if x.len() > y.len() {
        x
    } else {
        y
    } // We’re returning a reference to one of the inputs — NOT a new String.
}

fn main() {
    let s1 = String::from("apple");
    let s2 = String::from("banana");
    let result = longest(&s1, &s2);
    //longest returns a reference to the longer one → Rust knows this is safe because s1 and s2 live long enough
    println!("Longest: {}", result);
    //so if u  try to print result outside this bracket then you get an error because its out of scope and memory is freed
}
//Without lifetimes, Rust doesn’t know how long the returned reference is valid.
/*
You could have written:
fn longest(x: &str, y: &str) -> &str
But Rust would complain:
"Hey! What if the returned reference outlives the inputs? I need you to tell me!"
*/
