fn main() {
    let s = String::from("hello"); //s comes into scope

    take_ownership(s); // s' value moves into the function; no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // i32 implements copy trait, x does not move into the function
                    // okay to use x afterward


    // Return Values and scope
    // _underscore is because I did not print out i.e unused
    let _s1 = gives_ownership(); // gives_ownership moves its return
                                // value into s1
    
    let s2 = String::from("hello"); // s2 comes into scope
    
    let _s3 = take_and_gives_back(s2); // s2 is moved into take_and_gives_back
                                    // which also moves its return value into s3


} // x goes out of scope, then s. However, since s' value was moved,
    // nothing special happens

    // s3 goes out of scope and is dropped.
    // s2 was moved, nothing happens
    // s1 goes out of scope and is dropped

fn take_ownership(some_string: String) { // some_string comes into scope
    println!("{some_string}");
} // some_string goes out of scope, memory freed

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{some_integer}");
} // some_integer goes out of scope, nothing special happens

fn gives_ownership() -> String {

    let some_string_two = String::from("yours"); //some_string_two comes into scope

    some_string_two     // some_string_two is returned and
                        // moves out to the calling function
}

// this function takes a string and returns a string
fn take_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    a_string // a_string is returned and 
            // moves out to the calling function
}