use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mut file = File::create("buffer.txt").unwrap();
    let target = Arc::new(Mutex::new(file));
    let counter = Arc::new(Mutex::new(0));

    let th1 = thread::spawn(move || {
        let counter = Arc::clone(&counter);
        let target = Arc::clone(&target);
        let mut num = counter.lock().unwrap();
        let mut target_file = target.lock().unwrap();

        let content = format!("th1 : {}", num);

        let f = write!(target_file, "{}", content);
        let _res = match f {
            Ok(file) => file,
            Err(error) => panic!("why rust {:?}", error),
        };
        *num += 1;
    });
}
