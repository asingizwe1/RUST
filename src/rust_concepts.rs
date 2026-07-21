/*
Packages
A package is like a toy box 📦 that holds one or more toys (called crates). It helps you organize your toys, test if they work, and even share them with your friends
this toy box is managed using Cargo (Rust’s helper tool).

bash
Copy code
cargo new my_toy_box
That creates a package (your toy box)./

A crate is one big toy, like a robot 🤖 or a car 🚗. It can be a library crate (something other toys can use) or a binary crate (something that can run, like a toy with batteries).
cargo new robot_toy
You just created a crate (a full robot toy) inside a package. Every Rust package has at least one crate.

🔧 The file main.rs (for binaries) or lib.rs (for libraries) is like the manual to build your toy.

 Modules and use — Rooms in Your House
What it is:
Modules are like rooms 🛏️ in your toy house. Each room can have its own stuff (functions, structs, etc.).
The use keyword lets you bring stuff from another room into your current room so you can use it.
*/
//MODULES
// When you put functions in a file (like kitchen.rs) and write mod kitchen; in main.rs, that file becomes a module called kitchen.
//in main.rs
mod kitchen;
use kitchen::cook;
fn main() {
    cook();
} //modules keep things neet and tidy, just like rooms in a house. You can have a kitchen module for cooking, a living room module for relaxing, and so on. Each module can have its own functions and structures.
  /*
  A package is basically your entire project folder.
  It’s defined by the Cargo.toml file.

  👉 That Cargo.toml file says:
  What the name of your package is

  What dependencies (other crates) you want to use

  What crate(s) it contains

  📦 What’s a Crate?
  Inside the package, a crate is the actual code being built — like the toy inside your toy box.

  There are two types:

  Binary crate → Creates a program you can run

  Library crate → Code you can use from other crates (like reusable tools)
  */
/*
 Simple example: binary + library
Let’s say you want to make a simple calculator:

Step 1: Create a new package
bash
Copy code
cargo new calculator
cd calculator
This creates a package with a binary crate (because it has a main.rs).

Step 2: Add a library crate
Inside your calculator folder, create a new file:

📄 src/lib.rs

rust
Copy code
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
📄 src/main.rs

rust
Copy code
// main.rs is the binary crate
// it uses the library crate defined in lib.rs

fn main() {
    let result = calculator::add(5, 3);
    println!("5 + 3 = {}", result);
}
BUT this won’t work yet… because Rust doesn’t know the name calculator refers to your lib.rs crate.

✅ Step 3: Tell Rust to build both crates
Edit Cargo.toml and add:

toml
Copy code
[lib]
name = "calculator"
path = "src/lib.rs"
Now:

main.rs is your binary crate

lib.rs is your library crate

Both are part of the same package

Rust builds both when you run:

bash
Copy code
cargo run
🎉 You just used:

1 package (your project)

1 binary crate (main.rs)

1 library crate (lib.rs)


*/
