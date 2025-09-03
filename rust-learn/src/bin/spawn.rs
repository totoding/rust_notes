use std::{thread::{self, sleep}, time::Duration};

fn main() {
    let data = String::from("hello");
    let handle = thread::spawn(move || {
        println!("abc");
        println!("{}", data);
        sleep(Duration::from_millis(2000));
        43
    });
    let h1: thread::JoinHandle<i32> = thread::spawn(|| {
      22
    });
    let result = handle.join().expect("线程崩了");
    let result2 = h1.join().expect("线程崩了");
    println!("结果1 = {result}");
    println!("结果2 = {result2}");
}
