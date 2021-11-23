extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // rust uses macros for printing
    println!("Guess the number");

    // generate a secret number from rand crate
    // thread_rng uses a seed that is local to current thread
    // range is [lower, upper)
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("the secret number is {}", secret_number);

    // infinite loop
    loop {
        println!("Please input guess");
        // call constructor to get empty string
        // immutable by default; prefix with mut to get mutable variable
        let mut guess = String::new();

        // read into mutable variable from stdin using a ref
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line"); // read_line returns a result type so we match on it and fail early if nothing
                                            // string formatting uses {}

        // shadows existing declaration - allows variable name reuse rather than creating a new variable
        // parse takes a variable that has a typeclass, FromStr, that allows conversion from str to type T.
        // match on result type to prevent crash
        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smol"),
            Ordering::Equal => {
                println!("pog congrats");
                break;
            }
            Ordering::Greater => println!("too big"),
        };
        println!("Guessed: {}", guess);
    }
}
