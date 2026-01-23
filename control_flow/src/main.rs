fn main() {
    // if expressions
    let number = 2;

    if number < 5 {
        println!("True")
    }
    else {
        println!("False")
    }

    // else if
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // using if in a let statement
    let condition = false;
    let number = if condition {8} else {10};
    println!("The value of the number is: {number}");

    // loop statements
    let mut count = 0;
    'count_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'count_up;
            }
            remaining -=1;
        }
        count += 1;
    }
    println!("end count = {count}");

    // streamlining the loop statements with while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}");

        number -= 1;
    }
    println!("LiftOFF!");

    // looping through elements
    let a = [0, 20, 40, 60, 80, 100];
    let mut index = 0;
    
    while index < 6 {
        println!("the value at index {index} is: {}", a[index]);
        index += 1;
    }

    // using for loop instead
    for element in a {
        println!("the value is: {element}");
    } //safety improved, conciseness

    // reverse the range
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LiftOFF!");
}
