use std::fmt::format;
use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let target = Arc::new(Mutex::new(File::create("buffer.txt").unwrap()));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let target = Arc::clone(&target);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            let mut target_file = target.lock().unwrap();

            let content = format!("thread A: {} \n", num);

            let f = write!(target_file, "{}", content);
            let _res = match f {
                Ok(file) => file,
                Err(error) => panic!("why rust {:?}", error),
            };
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
