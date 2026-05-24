fn main() {
    let mut a = String::from("Hello");
    let b = &mut a;
    b.push_str(" world");
    b.push_str("!");
    println!("{}", a);
}
