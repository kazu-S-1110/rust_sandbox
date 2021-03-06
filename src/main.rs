use std::fs::File;
use std::io::prelude::*;
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let target = Arc::new(Mutex::new(File::create("buffer.txt").unwrap()));
    let counter = Arc::new(Mutex::new(0));
    let mut handles1 = vec![];
    let mut handles2 = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let target = Arc::clone(&target);
        let handle1 = thread::spawn(move || {
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
        handles1.push(handle1);
    }

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let target = Arc::clone(&target);
        let handle2 = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            let mut target_file = target.lock().unwrap();

            let content = format!("thread B: {} \n", num);

            let f = write!(target_file, "{}", content);
            let _res = match f {
                Ok(file) => file,
                Err(error) => panic!("why rust {:?}", error),
            };
            *num += 1;
        });
        handles2.push(handle2);
    }

    handles1.into_iter().for_each(|h| h.join().unwrap());
    handles2.into_iter().for_each(|h| h.join().unwrap());
}
