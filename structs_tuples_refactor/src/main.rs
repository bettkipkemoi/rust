//debug trait
#[derive(Debug)]
//using structs_refactor
struct Rectangle {
    width: u32,
    height: u32
}

fn main() {
    // using struct
    let width1 = 30;
    let height1 = 50;
    println!(
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    // refactor with tuples
    let rect1 = (30, 50);
    println!(
        "The area of the rectangle using method 2 is {}",
        area_method2(rect1)
    );

    // structs_refactoring
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle using struct_refactor is {}",
        area_structs_refactor(&rect2)
    );

    //debug trait
    println!("rect2 is: {rect2:#?}");

    // using dbg! - takes ownership of an expression
    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(rect3); //printing
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

// better than above, adds a bit of structure
fn area_method2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// structs refactoring
fn area_structs_refactor(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}