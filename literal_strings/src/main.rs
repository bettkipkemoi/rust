fn main() {
    let my_string = String::from("hello world");
    // `first_word` works on slices of `String`s, whether partial or whole.
    let word = first_word(&my_string[0..6]);
    let word2 = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s.
    let word3 = first_word(&my_string);
    let my_string_literal = "hello world";
    // `first_word` works on slices of string literals, whether partial or
    // whole.
    let word4 = first_word(&my_string_literal[0..6]);
    let word5 = first_word(&my_string_literal[..]);
    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word6 = first_word(my_string_literal);
    println!("word: {word}, word2: {word2}, word3: {word3},
    my_string_literal: {my_string_literal}, word4: {word4},
    word5: {word5}, word6: {word6}");

    // other slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]);
    println!("slice: {:?}", slice);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}