fn main() {
    println!("Learning about primitive types!");
    let x: i32 = 5;
    let y: bool = true;
    let z: f64 = 3.1415;
    let a = 3.1415_f32; // can declare type like this too
    println!("{} {} {} {}", x, y, z, a);
    compound_types();
    // compound types
}

fn compound_types() {
    // * tuples can store elements of dissimilar types
    // ! tuples are not designed to have more than 12 elements right now !
    let tuple: (u8, f32, u32) = (1, 3.3, 999);
    let (first, second, third) = tuple; // can destructure like this
    println!("{}, {}, {}, {}", tuple.0, first, second, third);

    // * arrays can store elements of the same type
    // ! arrays are limited to 32 elements, vectors are used for larger sizes
    let buff = [0, 1, 2]; // literal declaration
    let init_array = [1; 100]; // declare fill value and length
    println!("{}", buff[0]);
    println!("{}", init_array[0]);
}
