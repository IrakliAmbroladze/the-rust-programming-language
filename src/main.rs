//4.1 Return Values and Scope

fn main() {
   let s1 = gives_ownership();
   let s2 = String::from("hello");
   let s3 = takes_and_gives_back(s2);
   let s4 = gives_ownership();
   let s5 = takes_and_gives_back(s3);
   println!("{}", s1);
   println!("{}", s4);
   println!("{}", s5);
}

fn gives_ownership()->String{
   let some_string= String::from("yours");
   
   some_string
}

fn takes_and_gives_back(a_string: String)->String{
   a_string
}