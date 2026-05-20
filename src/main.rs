use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878");
    let listener2 = TcpListener::bind("127.0.0.1:7878");

    println!("{:?}", listener);
    println!("{:?}", listener2);
    //for stream in listener.incoming() {
    //    println!("{:?}", stream);
    //}
}
