fn main(){let v = vec![1, 2, 3];
let midpoint = v.len() / 2;
/*
Why .scope() is Better:
No need to clone or move v — scoped threads can borrow directly!
Guarantees safety — main thread waits until all scoped threads are done.
Cleaner code — no .join() calls needed for each thread manually.
*/
std::thread::scope(|scope| {
    scope.spawn(|| {
        let first = &v[..midpoint   ];
        println!("Here's the first half of v: {first:?}");
    });
    scope.spawn(|| {
        let second = &v[midpoint..];
        println!("Here's the second half of v: {second:?}");
    });
});

println!("Here's v: {v:?}");}
//The std::thread::scope function creates a new scope.
//std::thread::scope takes a closure as input, with a single argument: a Scope instance.