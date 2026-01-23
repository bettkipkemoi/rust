fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{s1}' is {len}.");

    // mutable references
    let mut t = String::from("hello");
    change(&mut t);
    println!("The value of t: '{t}'");

    //combining mutable and immutable references
    let mut s = String::from("hello");
    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("r1: {r1} and r2: {r2}");
    // Variables r1 and r2 will not be used after this point.
    let r3 = &mut s; // no problem
    println!("r3: {r3}");
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
}// Here, s goes out of scope. But because s does not have ownership of what
// it refers to, the String is not dropped.

//mutable references
fn change(some_string: &mut String) {
    some_string.push_str(", world");
    println!("The value of the changed string, taken to t: {some_string}")
}