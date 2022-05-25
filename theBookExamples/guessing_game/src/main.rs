use std::io;
use rand::Rng;

/// guessing game

// String is the dynamic heap string type, like Vec: use it when you need to own or modify your string data.
// str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory. Since the size is unknown, one can only handle it behind a pointer. This means that str most commonly2 appears as &str: a reference to some UTF-8 data, normally called a "string slice" or just a "slice".

// https://stackoverflow.com/questions/24158114/what-are-the-differences-between-rusts-string-and-str#:~:text=String%20is%20the%20dynamic%20heap,handle%20it%20behind%20a%20pointer.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed {}", guess);
}
