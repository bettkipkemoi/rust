#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle { // Everything within this impl block will be
// associated with the Rectangle type
    fn area(&self) -> u32 {
        self.width * self.height //Self is an alias for the
                //type that the impl block is for
    }
}

//using same name for a method as in struct's field
impl Rectangle{
    fn width(&self) -> bool {
        self.width > 0
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    //same name example
    // methods like this called getters, useful for making fields private
    let rect2 = Rectangle {
        width: 20,
        height: 40,
    };

    if rect2.width() {
        println!(
            "The area of the rectangle that has nonzero width is: {}",
            rect2.width
        );
    }
}