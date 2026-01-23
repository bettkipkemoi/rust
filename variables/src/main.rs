fn main() {
    //variables are immutable by default
    let mut x = 4;
    println!("The value of X is: {x}");
    x = 5;
    println!("The value of updated x is: {x}");

    // example of a constant
    const FOUR_HOURS_IN_MINUTES: u32= 60 * 4; //Use all caps as rust's naming convention for constants
    println!("4 hours in minutes: {FOUR_HOURS_IN_MINUTES}");

    // shadowing
    let x = 4;
    let x = x-2;
    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }
    println!("The value of x is: {x}");

    // difference of mut and shadowing
    let spaces = "    ";
    let spaces = spaces.len();
    println!("The length of spaces is: {spaces}");

    // data types
    let guess: u32 = "42".parse().expect("Not a number");
    println!("guess: {guess}");

    //scalar types: integers, floats, booleans & characters
    // all floating point numbers are signed
    let x = 2.0; //f64
    println!("F64 x: {x}");
    let y: f32 = 5.0; //f32
    println!("F32 y: {y}");

    // mathematical operations
    // addition
    let sum = 5 + 10;
    // subtraction
    let difference = 95.5 - 4.3;
    // multiplication
    let product = 4 * 30;
    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    // remainder
    let remainder = 43 % 5;
    println!("sum: {sum}, difference: {difference}
    product: {product}, quotient: {quotient}, truncated: {truncated},
    remainder: {remainder}");

    // boolean
    let t = true; // they are one byte in size
    let f = false;
    println!("true and false resp: {t}, {f}");

    // characters
    let c = 'z'; //note: use single quotation marks
    let heart_eyed_char = '😻';
    println!("characters {c} and {heart_eyed_char}");

    // compound types: grouping multiple values into one type. Two types: tuples and arrays
    // tuples
    let tup = (100, 4.5, 2);
    let (x, y, z) = tup;
    println!("Values in the tuple are: x: {x}, y: {y}, z: {z}");
    // accessing by index: use period (.)
    let x = (100, 4.5,2);
    let two = x.2;
    println!("Value of X: {two}");

    // arrays: must have same type, and same length
    let a = [1, 2, 3, 4, 5]; // will not change in future
    let first = a[0];
    println!("first number: {first}");
    // another example: months
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];
    let fourth = months[3];
    println!("fourth month: {fourth}");
    // initializing with same number for specific length
    let b = [4; 5];
    println!("b_array: {:?}", b);

}