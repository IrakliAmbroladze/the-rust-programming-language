fn main() {
   let y = {
       let x = 3;
       x + 1
   };

   println!("The value of y is: {y}");

   let x = five();
   println!("The value of x is: {x}");
}

fn five() -> i32{
   5
}