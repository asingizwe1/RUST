//success containing value T and failure contaoining error E
enum Resut<T, E> {
    Ok(T),
    Err(E),
}
//you must input the variables in function bracket
fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("Cannot divide by zero".to_string())
    } else {
        Ok(a / b)
    }
}
fn main() {
    match safe_divide(10, 2) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }

    match safe_divide(10, 0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}
