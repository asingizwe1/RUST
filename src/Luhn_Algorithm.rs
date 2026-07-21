/*
The Luhn algorithm is used to validate credit card numbers. 

The algorithm takes a string as
 input and does the following to validate the credit card number:
 • Ignore all spaces. Reject numbers with fewer than two digits.
 • Moving from right to left, double every second digit: for the number 1234, we double
 3 and1. For the number 98765, we double 6 and 8.
 • Afterdoublingadigit, sumthedigitsiftheresultisgreaterthan9. Sodoubling7becomes
 14 which becomes 1 + 4 = 5.
 • Sum all the undoubled and doubled digits.
 • Thecredit card number is valid if the sum ends with 0.

*/
/* 
pub fn Luhn(cc_number:&str)-> bool{
let mut sum =0;
let mut double =false;
//.chars() method converts a string slice (&str) into an iterator over its Unicode characters.
//The .rev() method reverses the order of an iterator.
for c in cc_number.chars().rev(){

if let Some(digit)=c.to_digit(10){
    if double {
        let double_digit = digit * 2;
        sum +=
        if double_digit > 9 { double_digit- 9 } else { double_digit };
        } else {
        sum += digit;
        }
        double = !double;
        } else {
        continue;
        }
}
sum % 10 == 0
}
*/
 // This is the solution and passes all of the tests below.
 pub fn luhn(cc_number: &str)-> bool {
    let mut sum = 0;
    let mut double = false;
    let mut digits = 0;
//.to_digit(radix) method is used to convert a character (char) into a numeric value (u32)
//, based on a given radix (base).
    for c in cc_number.chars().rev() {
//if let is a shorthand way to match and extract values from an Option, Result
//, or any enum when you only care about one specific case
//if let is just a shorter way of writing this when we only care about Some(x).
    if let Some(digit) = c.to_digit(10) {
    digits += 1;
    if double {
    let double_digit = digit * 2;
    sum +=
    if double_digit > 9 { double_digit- 9 } else { double_digit };
    } else {
    sum += digit;
    }
    double = !double;
    } else if c.is_whitespace() {
    // New: accept whitespace.
    continue;
    } else {
    // New: reject all other characters.
    return false;
    }
    }
    // New: check that we have at least two digits
    digits >= 2 && sum % 10 == 0
    }

    fn main() {
        let cc_number = "1234 5678 1234 5670";
        println!(
        "Is {cc_number} a valid credit card number? {}",
        if luhn(cc_number) { "yes" } else { "no" }
        );
        }

/* 
### **Understanding `if let` in Rust**  

#### **What is `if let`?**  
`if let` is a shorthand way to **match** and extract values from an `Option`, `Result`, or any enum **when you only care about one specific case**.  

---

### **1. Basic Usage (`Option<T>`)**  
Instead of using a full `match`, `if let` lets you handle only the case you care about.

#### **Example: Checking for `Some` in `Option<T>`**
```rust
fn main() {
    let some_value = Some(42);

    // Using `if let`
    if let Some(x) = some_value {
        println!("The number is: {}", x);
    } 
}
```
- ✅ **Extracts `x` only if `some_value` is `Some(42)`**.
- ❌ **Ignores `None` values without needing a `match` statement**.

✅ **Same code using `match` (longer way)**:
```rust
match some_value {
    Some(x) => println!("The number is: {}", x),
    None => {}
}
```
- **`if let` is just a shorter way** of writing this when we **only care about `Some(x)`**.

---

### **2. Handling `Err` in `Result<T, E>`**
If you're working with `Result`, you can **use `if let` to handle only errors**.

```rust
fn main() {
    let result: Result<i32, &str> = Err("Something went wrong");

    if let Err(e) = result {
        println!("Error: {}", e);
    }
}
```
- ✅ Extracts the **error message (`e`)** **only** if the result is `Err(e)`.
- ❌ Ignores `Ok` values automatically.

---

### **3. Using `if let` with Enums**
If you have a **custom enum**, `if let` helps extract only the variant you need.

```rust
enum Animal {
    Dog(String),
    Cat(String),
}

fn main() {
    let pet = Animal::Dog(String::from("Buddy"));

    if let Animal::Dog(name) = pet {
        println!("It's a dog named {}", name);
    }
}
```
- ✅ Checks if `pet` is **`Dog(name)`**, then prints the name.
- ❌ **Ignores `Cat(_)` without an explicit `else`**.

---

### **4. Using `else` with `if let`**
You can **add an `else`** to handle the case **when the pattern doesn’t match**.

```rust
fn main() {
    let value = Some(10);

    if let Some(x) = value {
        println!("Got value: {}", x);
    } else {
        println!("It's None");
    }
}
```
- ✅ If `value` is `Some(x)`, it prints `Got value: x`.
- ✅ If `value` is `None`, it prints `"It's None"`.

---

### **5. When to Use `if let` vs `match`?**
| Situation | Use `if let` | Use `match` |
|-----------|-------------|-------------|
| When you **only care about one case** | ✅ Yes | ❌ No |
| When you **need to handle multiple cases** | ❌ No | ✅ Yes |
| When extracting values from `Option` or `Result` | ✅ Yes | ✅ Yes |
| When ignoring `None` or `Err` | ✅ Yes | ❌ No |

---

### **6. Practical Example: Iterating with `if let`**
You can use `if let` inside loops to **skip `None` values**.

```rust
fn main() {
    let numbers = vec![Some(1), None, Some(3), Some(5), None];

    for num in numbers {
        if let Some(x) = num {
            println!("Number: {}", x);
        }
    }
}
```
- ✅ **Prints only the numbers**, skipping `None`.

---

### **Final Summary**
✅ **Use `if let` when:**  
- You only **care about one specific pattern**.  
- You **don’t need to handle all possible cases**.  
- You **want to extract values** from `Option`, `Result`, or enums **without using `match`**.  

Would you like more examples? 🚀
*/























