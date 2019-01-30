/*
- A crate is a collection of Rust source code files
- The project we’ve been building is a binary crate, which is an executable.
*/
use std::io;
use std::cmp::Ordering;
use rand::Rng; // Rng: A trait tells the Rust compiler about functionality a particular type has and can share with other types.

fn main() {
    // println! calls a Rust macro.
    // If it called a function instead, it would be entered as println (without the !)
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // variables in rust are immutable
        // mut makes it mutable
        //
        // '::' declares an associated function
        // 'r#' declares guess as a raw identifier
        let mut r#guess = String::new();

        // '&' declares the argument is reference.
        // this allows multiple parts of the program to access this single piece of data.
        // References are immutable by default so we need to append 'mut' to write to it.
        io::stdin().read_line(&mut guess)
            // Because it might fail, the read_line method returns a 'Result' type.
            .expect("Failed to read line");

        // 'Shadow' the previous version of "guess"
        // 'Shadowing' allows the use of a variable instead of creating new unique ones.
        // A technique used for casting
        // Type annotation - applying the type u32 to variable 'guess'
        // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters
        // Use 'match' to handle an Error
        let guess: u32 = match guess.trim().parse() {
            // parse will return an 'ok' value and Ok will return the number.
            Ok(num) => num,
            // If it cannot parse, the Err pattern is matched ('_' is a catch-all pattern)
            Err(_) => continue,
        };

        // '{}' prints the value. You can have multiple braces and values.
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                // This will exit the program because the loop is last part of main.
                break;
            }
        }        
    }
}
