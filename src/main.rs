use std::{time::Duration, thread::sleep};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

fn  progress<Iter>(iter: Iter, f: fn(Iter::Item)-> ())
where  Iter: Iterator {
    let mut i:usize = 1;

    for n in iter {
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
    progress(v.iter(), expensive_calculation);

    println!("----------");
    use std::collections::HashSet;
    let mut h = HashSet::new();
    h.insert(0);
    h.insert(1);
    h.insert(3);
    progress(h.iter(), expensive_calculation);
}
