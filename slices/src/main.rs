fn main() {
    let mut s = String::from("hey bett");

    let word = first_word(&s); //word will get the value 5
    s.clear(); // this empties the string, makes it equal to ""
    // word has the value 3 (hey) here, but s no longer has any content
    // safe to say word is totally invalid now since nothing meaningful can be done with the value 3
    println!("values of word: {word} and s: {s}");
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
