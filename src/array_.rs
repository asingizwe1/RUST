fn main() {
    //[] - for an array
    let primes = [2, 3, 5]; //could also be a vec
    for A_PRIME in primes {
        println!("{}", A_PRIME);
    }

    for p in primes {
        for q in primes {
            println!(
                "
         {},{}
        
          ",
                p, q
            );
        }
    }
}
