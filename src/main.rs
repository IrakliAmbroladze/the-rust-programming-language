fn main() {
    let v = vec![String::from("Hello ")];
    let mut s = v[0].clone();
    s.push_str("world");
    println!("{s}");
}
