use std::{time::Duration, thread::sleep};
const CLEAR: &str = "\x1B[2J\x1B[1;1H";
struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char)
}
struct Progress<Iter, Bounded> {
    iter: Iter,
    i: usize,
    bound: Bounded
    // bound: Option<usize>,
    // delims: (char, char)
}

trait ProgressDisplay:Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

impl ProgressDisplay for  Unbounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}", "*".repeat(progress.i));
    }
}

impl ProgressDisplay for  Bounded {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
        println!("{}{}{}{}",
        self.delims.0,
        "*".repeat(progress.i),
        " ".repeat(self.bound -progress.i),
        self.delims.1);
    }
}

impl <Iter> Progress<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Progress { iter: iter, i: 0, bound: Unbounded }
    }
}

// impl <Iter> Progress<Iter, Bounded>
impl <Iter> Progress<Iter, Unbounded>
where Iter: ExactSizeIterator {
    pub fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']')
        };
        Progress { i: self.i, iter: self.iter, bound}
    }
}

impl <Iter> Progress<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) ->Self {
        self.bound.delims = delims;
        self
    }
}

impl<Iter, Bound > Iterator for Progress<Iter, Bound>
where Iter: Iterator, Bound: ProgressDisplay {
    type Item =Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}",CLEAR);
        self.bound.display(&self);
        // match self.bound {
        //     Some(bound) => {
        //         println!("{}{}{}{}",
        //             self.delims.0,
        //             "*".repeat(self.i),
        //             " ".repeat(bound -self.i),
        //             self.delims.1);
        //     },
        //     None => {
        //         println!("{}", "*".repeat(self.i));
        //     }
        // }

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
    fn progress(self) -> Progress<Self, Unbounded>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn  progress(self)->Progress<Self,Unbounded>{
        Progress::new(self)
    }
}

fn main() {

    // for  n in (0 .. ).progress().with_bound() {
    //     expensive_calculation(n);
    // }
    let brkts = ('<', '>');
    let v: Vec<i32> = vec![1,2,3];
    for n in v.iter().progress().with_bound().with_delims(brkts) {
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
