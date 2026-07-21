use std::cell::RefCell;

fn main() {
    let x = RefCell::new(10); // like a box holding the value 10

    {
        let r = x.borrow(); // read access
        println!("Reading: {}", r);
    } // r goes out of scope, safe to continue

    {
        let mut w = x.borrow_mut(); // write access
        *w += 5;
        println!("Writing: {}", w);
    }

    println!("Final value: {}", x.borrow()); // should print 15
}
