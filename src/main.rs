//4.1 Stack-Only Data: Copy

fn main() {
   let x =  5;
   let y = x;
   println!("x={x}, y={y}");
//    Types such as integers that have a known size at a compile time
//are stored entirely on the stack. So there is no need for deep copy
//with clone
}