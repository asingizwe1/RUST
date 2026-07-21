use std::thread::spawn;
//std::thread::spawn creates a new thread to run a piece of code concurrently.
//The new thread cannot access data from the main thread unless that data is 'static (lives for the entire program’s duration).
//This is why we often need to clone data or move ownership to the thread.
fn main(){let v = vec![1, 2, 3];
let midpoint = v.len() / 2;

let handle1 = std::thread::spawn(|| {
    let first = &v[..midpoint];
    println!("Here's the first half of v: {first:?}");
});
let handle2 = std::thread::spawn(|| {
    let second = &v[midpoint..];
    println!("Here's the second half of v: {second:?}");
});

handle1.join().unwrap();
handle2.join().unwrap();

println!("Here's v: {v:?}");
}