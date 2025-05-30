//4.1 Ownership and Functions

fn main() {
   let s = String::from("hello");
   println!("{}", s);
   takes_ownership(s);

   let x = 5;
   makes_copy(x);

   println!("{}", x);
}

fn takes_ownership(_some_string: String) {
    
}
fn makes_copy(_some_integer: i32) {
    
}