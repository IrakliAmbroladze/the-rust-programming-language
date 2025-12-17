fn main() {
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0];
    let s: String = *s_ref;
    println!("{s}")
}
