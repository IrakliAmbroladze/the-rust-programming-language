#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(unused_variables)]
fn main() {
    let rec1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rec1)
    );
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
