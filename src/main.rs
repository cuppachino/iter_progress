use std::{ thread::sleep, time::Duration };

const CLEAR: &str = "\x1B[2J\x1B[0;0H";

/* 📦 Thing */
pub struct Printer<Iter> {
    iter: Iter,
    i: usize,
}

/* 🏗️ Constructor */
impl<Iter> Printer<Iter> {
    pub fn new(iter: Iter) -> Self {
        Printer {
            iter,
            i: 0,
        }
    }
}

/* 🪜 Iterator - Implement the required traits of the Iterator */
impl<Iter> Iterator for Printer<Iter> where Iter: Iterator {
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("state: {}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

/* 🏷️ Trait -  `.printer()` */
trait PrinterIteratorExt: Sized {
    fn printer(self) -> Printer<Self>;
}

impl<Iter> PrinterIteratorExt for Iter where Iter: Iterator {
    fn printer(self) -> Printer<Self> {
        Printer::new(self)
    }
}

/* 🚀 App */
fn compute(_n: &i32) {
    sleep(Duration::from_secs(1))
}

fn main() {
    // unbounded progress bar needed:
    for n in (0..).printer() {
        compute(&n);
        if n > 2 {
            break;
        }
    }

    let v = vec![1, 2, 3];
    for n in v.iter().printer() {
        compute(n);
    }
}
