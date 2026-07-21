fn main() {
    //borrow temporarily use a value without taking ownership
    //we create a reference to use the borroeing tule
    //immutable-readonly
    //mutable- only one mutable reference can be created
    ///for mutable we can only have one read and write access at a time
    /// for immutable its any number of read-only access to a value
    // must no t outlive the value
    let s = "xrp".to_string();
    let s1 = &s;
    let s2 = &s;
    let s3 = s2;
    //above is for immmutable reference - read only

    //converting the function to use borrowing
    //we turn the parameter into a reference
    fn borrow(s: &String) -> String {
        println!("{}", s);
    };

    //MUTABLE BORROWS
    let s1 = &mut s;
    //so we can modify value of s
    s1.push_str("_is goint to pump");

    //mutable and immutable refrences cant exist simultaneously
    //basically you cant have mut s and let s1=&s
    {
        let s2 = s;
    } //when you print s1-> reference s1 will live longer than refernce s

    //refrence must not outlive the value
    //after the scope s2 and 2 will nolonger exist
    //but there is still s1 which has reference to s

    let s = String::from("that");
    //we put a reference sign before variable s because the parameter was a reference

    borrow(&s);
    println!("{}", s);
    //basically when s prints it means ownership was transfered
    //but alsos wasnt lost when it got out of scope of borrow
}
