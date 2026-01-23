fn main() {
    // string types
    let mut s = String::from("hello"); //mutable string
    s.push_str(", bett!"); // push_str() appends a literal to a string
    println!("{s}"); // prints `hello, bett!`

    // integers: works well because they have know size at compile time
    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // similar data types that implement copy

    // All the integer types, such as u32.
    // The Boolean type, bool, with values true and false.
    // All the floating-point types, such as f64.
    // The character type, char.
    // Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
    // 

    // strings
    let s1 = String::from("hello");
    let s2 = s1; // s1 out of scope. println!("{s1}, bett") won't work
    println!("{s2}, bett!");

    // clone method
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {s1}, s2 = {s2}");

} // memory allocation: scope of s is over, no longer valid
