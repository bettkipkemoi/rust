fn main() {
    let mut s = String::from("hello world");
    let word = first_word(&s);
    let word2 = second_word(&s);
    println!("the first word is: {word}, second word: {word2}");
    s.clear(); // error!
    //println!("the first word is: {word}"); // won't work here
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i..];
        }
    }
    &s[..]
}
