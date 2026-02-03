fn main() {
let reference_to_s = no_dangle(); // could have been to nothing
println!("{reference_to_s}")
}

fn no_dangle() -> String { // dangle & could have returned a reference to a String
    let s = String::from("hello bett"); // s is a new String
    s // dangle & could have returned a reference to the String, s
} 

//summary of references
// At any given time, you can have either one mutable reference or any number of
// immutable references.
// References must always be valid.