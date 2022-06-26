use std::thread;

fn main() {
    let x = vec![100, 200];
    thread::spawn(move || println!("x is {:?}", x));

    println!("ok")
}
