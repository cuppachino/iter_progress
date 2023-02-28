use std::{thread::sleep, time::Duration};

fn compute(_n: &i32) {
    sleep(Duration::from_secs(1))
}

const CLEAR: &str = "\x1B[2J\x1B[0;0H";

struct Unbounded;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

pub struct Print<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

trait PrintDisplay: Sized {
    fn display<Iter>(&self, progress: &Print<Iter, Self>);
}

impl PrintDisplay for Unbounded {
    fn display<Iter>(&self, print: &Print<Iter, Self>) {
        println!("{}", "*".repeat(print.i));
    }
}

impl PrintDisplay for Bounded {
    fn display<Iter>(&self, print: &Print<Iter, Self>) {
        println!(
            "{}{}{}{}",
            self.delims.0,
            "*".repeat(print.i),
            " ".repeat(self.bound - print.i),
            self.delims.1
        )
    }
}

/* 🏗️ Constructor implementation */
impl<Iter> Print<Iter, Unbounded> {
    pub fn new(iter: Iter) -> Self {
        Print {
            iter,
            i: 0,
            bound: Unbounded,
        }
    }
}

/* 🧱
    If P is Unbounded and its exact length is known,
    implement `with_bound` on P.
*/
impl<Iter> Print<Iter, Unbounded>
where
    Iter: ExactSizeIterator,
{
    pub fn with_bound(self) -> Print<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Print {
            i: self.i,
            iter: self.iter,
            bound,
        }
    }
}

/* 💄 Delims - Change the appearance of your progress bar */
impl<Iter> Print<Iter, Bounded> {
    pub fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

/* 📦 Print */
impl<Iter, Bound> Iterator for Print<Iter, Bound>
where
    Iter: Iterator,
    Bound: PrintDisplay,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        print!("{}", CLEAR);
        self.bound.display(&self);
        self.i += 1;
        self.iter.next()
    }
}

/* 🏷️ Trait -  `.print()` */
trait PrintIteratorExt: Sized {
    fn print(self) -> Print<Self, Unbounded>;
}

impl<Iter> PrintIteratorExt for Iter
where
    Iter: Iterator,
{
    fn print(self) -> Print<Self, Unbounded> {
        Print::new(self)
    }
}

fn main() {
    let _curly = ('{', '}');

    /* UNCOMMENT TO SEE THE ERROR
    ! the method `with_bound` exists for struct `Print<RangeFrom<{integer}>, Unbounded>`
    ! but the following trait bounds were not satisfied: `RangeFrom<{integer}>: ExactSizeIterator`
    */
    // for n in (0..).print().with_bound() {
    //     compute(&n);
    // }

    /* UNCOMMENT TO SEE THE ERROR
    ! no method named `with_delims` found for struct `Print<RangeFrom<{integer}>, Unbounded>` in the current scope
    ! the method was found for `Print<Iter, Bounded>`
    */
    // for n in (0..).print().with_delims(_curly) {
    //       compute(&n);
    //   }

    let v = vec![1, 2, 3, 4, 5];
    for n in v.iter().print().with_bound().with_delims(_curly) {
        compute(n)
    }
}
