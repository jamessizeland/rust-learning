use std::cmp::Ordering;
use rand::Rng; // The Rng trait defines methods that random number generators implement, and this trait must be in scope for us to use those methods.
use std::io;

/// guessing game

// String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
// str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice".

// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str#:~:text=String%20is%20the%20dynamic%20heap,handle%20it%20behind%20a%20pointer.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100); // this type is infered to be u32 because of the match comparison with guess later!

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number"); // Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variables

    println!("You guessed {}", guess);

    // match against the guess
    // A match expression is made up of arms. An arm consists of a pattern to match against, and the code that should be run if the value given to match fits that arm’s pattern.
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("too small"),
        Ordering::Equal => println!("you win!"),
        Ordering::Greater => println!("too large!"),
    }
}
