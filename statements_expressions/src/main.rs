fn main() {
    let y = {
        let x = 5;
        x + 4 // expressions don't include ending semi-colons
    }; // this is a statement
    println!("The value of y is: {y}"); //statement

    //functions with return values
    let m = four();
    println!("The value of m is: {m}");

    //another_example below
    let x = plus_one(4);
    println!("The value of x is: {x}");
}

// functions with return values
fn four() -> i32 {
    4
}

// another example
fn plus_one(x: i32) -> i32 {
    x + 1
}