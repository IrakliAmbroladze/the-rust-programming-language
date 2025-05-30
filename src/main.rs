//4.1 Return Values and Scope: Listing 4-5: Returning ownership of parameters

fn main() {
   let s1 = String::from("hello");
   let (s2, len) = calculate_length(s1);
   println!("The length of '{s2}' is {len}."); 
}

fn calculate_length(s: String)->(String, usize){
   let length = s.len();
   (s, length)
}