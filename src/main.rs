use std::{sync::mpsc, thread};

fn main() {
    let (sender1, receiver1) = mpsc::channel();
    let (sender2, receiver2) = mpsc::channel();

    thread::spawn(move || {
        let val = receiver1.recv().unwrap();
        println!("send from main thread. {}", val);

        sender2.send("hi".to_string()).unwrap();
    });

    sender1.send("hello".to_string()).unwrap();

    let val = receiver2.recv().unwrap();
    println!("send from sub thread. {}", val);

    // println!("x is {:?}", x) 所有権が写ってしまったので実行不可。
}
