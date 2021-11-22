use std::io;

fn main() {
    // rust uses macros for printing
    println!("Guess the number");
    println!("Please input guess");

    // call constructor to get empty string
    // immutable by default; prefix with mut to get mutable variable
    let mut guess = String::new();

    // read into mutable variable from stdin using a ref
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // read_line returns a result type so we match on it and fail early if nothing
                                        // string formatting uses {}
    println!("Guessed: {}", guess);
}
