/*
- A crate is a collection of Rust source code files
- The project we’ve been building is a binary crate, which is an executable.
*/
use std::io;
use rand::Rng; // Rng: A trait tells the Rust compiler about functionality a particular type has and can share with other types.

// cargo check - check if it compiles
// cargo run - run in dev mode
// cargo build - build the app
// ./target/release/guessing_game - run the compiled version
// cargo update - update the project dependencies (Cargo.lock)
// cargo doc --open - open project documentation

fn main() {
    // println! calls a Rust macro.
    // If it called a function instead, it would be entered as println (without the !)
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    // variables in rust are immutable
    // mut makes it mutable
    //
    // '::' declares an associated function
    let mut guess = String::new();

    // '&' declares the argument is reference.
    // this allows multiple parts of the program to access this single piece of data.
    // References are immutable by default so we need to append 'mut' to write to it.
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    // '{}' prints the value. You can have multiple braces and values.
    println!("You guessed: {}", guess);
}
