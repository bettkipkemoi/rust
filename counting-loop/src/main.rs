fn main() {
// Print numbers from 1 to 10 using a counting loop
    let mut count: i32 = 0;
    let result: i32 = loop {
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("The final count is: {}", result);

    // print numbers in a range
    for number in 1..=10 {
        println!("Number: {}", number);
    }
}
