fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    *num += 1;
    println!("Third element is {}", *num);
    println!("Vector is now {:?}", v);
    println!("Vector is now {:?}", v);
    println!("Vector is now {}", v[2]);

    v[1] += 3;
    println!("Vector is now {}", v[1]);
    println!("Vector is now {:?}", v);
}
