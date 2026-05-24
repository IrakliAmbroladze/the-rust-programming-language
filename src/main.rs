fn main() {
    let mut a = String::from("Hello");
    let b = &mut a;
    b.push_str(" world");
    println!("{}", a);
}
