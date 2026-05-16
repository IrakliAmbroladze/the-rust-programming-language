trait Pilot {
    fn fly(&self);
}
trait Wizard {
    fn fly(&self);
}

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}
struct Human;
impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*")
    }
}
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
