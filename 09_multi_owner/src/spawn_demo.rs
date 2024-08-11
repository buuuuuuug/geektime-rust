use std::sync::{Arc, Mutex};

fn main() {
    let arr = vec![1];

    let t = std::thread::spawn(move || {
       println!("{:?}", arr);
    });
    match t.join() {
        Ok(_) => {println!("join ok")}
        Err(_) => {println!("join error")}
    }

    let l = Arc::new(Mutex::new(String::from("this is origin:")));

    let lc = l.clone();

    let t = std::thread::spawn(move || {
        println!("thread: {:?}", lc);
        let mut data = lc.lock().unwrap();
        *data = "this is modified".to_string();
    });
    println!("main: {:?}", l);
    t.join().unwrap();
    println!("main: {:?}", l);

}