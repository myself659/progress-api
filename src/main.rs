use std::{time::Duration, thread::sleep};

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    let mut i: usize = 0;
    for  n in  v.iter() {
        println!("{}", "*".repeat(i));
        i += 1;
        expensive_calculation(n);
    }
}
