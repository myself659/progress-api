use std::{time::Duration, thread::sleep};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn  progress<T>(v: Vec<T>, f: fn(&T)-> ()){
    let mut i:usize = 1;
    let n: &T;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n);
    }
}
fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    let mut i: usize = 1;
    for  n in  v.iter() {
        println!("{} {}", CLEAR, "*".repeat(i));
        i += 1;
        expensive_calculation(n);
    }
}
