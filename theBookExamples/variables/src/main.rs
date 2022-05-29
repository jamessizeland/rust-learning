#[allow(unused_variables)]
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // https://doc.rust-lang.org/reference/const_eval.html

fn main() {
    #[allow(unused_variables)]

    // let
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // shadowing
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }

    // float
    let z = 2.0;
    let zz: f32 = 3.1;

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0

    // remainder
    let remainder = 43 % 5;

    // boolean
    let t: bool = true; // booleans are 1 byte

    // char
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("{}, {}, {}", c, z, heart_eyed_cat);
    // we specify char literals with single quotes, as opposed to string literals, which use double quotes. Rust’s char type is four bytes in size and represents a Unicode Scalar Value, which means it can represent a lot more than just ASCII.

    // tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (a,b,c) = tup; // destructure
    println!("{}, {}, {}", a, b, c);
    println!("{:?}, {}", tup, tup.2); // print whole tuple and index 2

}
