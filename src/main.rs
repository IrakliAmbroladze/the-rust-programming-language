#[allow(unused_variables)]
fn main() {
    let rec1: (u32, u32) = (30, 50);
    let width1: u32 = 30;
    let height1: u32 = 50;

    println!("The area of the rectangle is {} square pixels.", area(rec1));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
