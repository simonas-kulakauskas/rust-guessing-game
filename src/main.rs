use std::io;
use std::cmp::Ordering; // Package that allows for comparison of two values, to see whether they are larger, smaller or equal.
use rand::Rng;

fn main() {
    println!("Guess the number!"); // these run a macro that outputs what we put in the parentheses into the console
    
    // ! let secret_number = rand::thread_rng().gen_range(1..=100); // Now depreciated, using line below makes it work instead.

    let secret_number = rand::rng().random_range(1..=100); 

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); // Variables in rust are immutable by default, hence why we specifed 'mut'
    // The line above also binds String to the return of function 'String' which creates a new instance of String. It's a standard library that is growable.
    // the :: means that new is an associated function of the String type.
    // Associated functions are those that are implemented on a type like in this case String. This creates a new empty string.

    // We also could've just done std::io::stdin() below and it would work.
    io::stdin()// We're calling the 'stdin()' function from our previously imported 'std::io', it allows us to handler user input.
        .read_line(&mut guess) // calls the 'read_line' method on the standard input handle to get input from the user
        // we also pass &mut guess as an argument to tell the string to store the user input into it. The '&' symbol indicates a reference.
        .expect("Failed to read line"); // This handles any potensial errors by throwing a fail message if the enum returns a err instead of ok.
    
    let guess: u32 = guess // shadow the guess variable, explicitly set its type to unsigned 32-bit.
        .trim() // Trim off any trailing or leading white space or other such parts (like the /n and/or /r which is expected when we press enter in the consol)
        .parse() // It will parse our variable converting it to the type we specified (u32) returns a Result varient.
        .expect("Please type a number!"); // If result is 'Err', stop program and throw error message, otherwise pass value.

    println!("You guessed: {guess}"); // Lastly we print the vlaue using the guess placeholder within curly braces {}. Quite similar to React for example...

    match guess.cmp(&secret_number) { // pattern matching like a switch statement, takes the guess and compares it to the secret_number.
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too large!"),
        Ordering::Equal => println!("You Win!"),
    }
}
