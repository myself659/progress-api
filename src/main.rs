use std::{time::Duration, thread::sleep};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    iter: Iter,
    i: usize
}

impl <Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter: iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator {
    type Item =Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

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

trait ProgressIteratorExt:Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn  progress(self)->Progress<Self>{
        Progress::new(self)
    }
}

fn main() {
    let v: Vec<i32> = vec![1,2,3];
    for n in v.iter().progress() {
        expensive_calculation(n);
    }
    // for n in Progress::new(v.iter()){
    //     expensive_calculation(n);
    // }
    // progress(v.iter(), expensive_calculation);

    // println!("----------");
    // use std::collections::HashSet;
    // let mut h = HashSet::new();
    // h.insert(0);
    // h.insert(1);
    // h.insert(3);
    // progress(h.iter(), expensive_calculation);
}
