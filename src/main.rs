#[derive(Debug)]
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
}

#[allow(unused_variables)]
fn main() {
    let rect1: Rectangle = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2: Rectangle = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3: Rectangle = Rectangle {
        width: 60,
        height: 45,
    };

    let r = &mut Box::new(Rectangle {
        width: 1,
        height: 2,
    });
    let area1 = r.area();
    let area2 = Rectangle::area(&**r);
    assert_eq!(area1, area2);
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
