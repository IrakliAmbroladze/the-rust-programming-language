#[derive(Debug, Copy, Clone)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        self.width > rectangle.width && self.height > rectangle.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
    fn max(self, other: Self) -> Self {
        Rectangle {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
    fn set_to_max(&mut self, other: Rectangle) {
        let max = self.max(other);
        *self = max;
    }
}

#[allow(unused_variables)]
fn main() {
    let rect = Rectangle {
        width: 0,
        height: 0,
    };
    println!("{}", rect.area());
    let other_rect = Rectangle {
        width: 1,
        height: 1,
    };

    let max_rect: Rectangle = rect.max(other_rect);
    println!("max_rect is {max_rect:#?}");
}
