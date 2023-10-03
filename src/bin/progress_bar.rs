//! # Progress Bar
//! A simple progress bar for iterators that utilizes rust type system

use std::{thread::sleep, time::Duration};

const CLEAR: &str = "\x1B[2J\x1B[1;1H";

struct UnBound;
struct Bounded {
    bound: usize,
    delims: (char, char),
}

struct Progress<Iter, Bound> {
    iter: Iter,
    i: usize,
    bound: Bound,
}

impl<Iter> Progress<Iter, UnBound> {
    fn new(iter: Iter) -> Self {
        Self {
            iter,
            i: 0,
            bound: UnBound,
        }
    }
}

impl<Iter> Progress<Iter, UnBound>
where
    Iter: ExactSizeIterator,
{
    fn with_bound(self) -> Progress<Iter, Bounded> {
        let bound = Bounded {
            bound: self.iter.len(),
            delims: ('[', ']'),
        };
        Progress {
            iter: self.iter,
            i: self.i,
            bound,
        }
    }
}

impl<Iter> Progress<Iter, Bounded> {
    fn with_delims(mut self, delims: (char, char)) -> Self {
        self.bound.delims = delims;
        self
    }
}

trait ProgressExt: Sized {
    fn progress(self) -> Progress<Self, UnBound>;
}

impl<Iter> ProgressExt for Iter {
    fn progress(self) -> Progress<Self, UnBound> {
        Progress::new(self)
    }
}

impl<Iter> Iterator for Progress<Iter, Bounded>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{CLEAR}");
        println!(
            "{}{}{}{}",
            self.bound.delims.0,
            "*".repeat(self.i),
            " ".repeat(self.bound.bound - self.i),
            self.bound.delims.1
        );
        self.i += 1;
        self.iter.next()
    }
}

impl<Iter> Iterator for Progress<Iter, UnBound>
where
    Iter: Iterator,
{
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        print!("{CLEAR}");
        println!("{}", "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }
}

fn do_sth(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = [1, 2, 3, 4, 5];
    let bracks = ('<', '>');
    for i in v.iter().progress().with_bound().with_delims(bracks) {
        do_sth(i);
    }
    for i in (0..).progress() {
        do_sth(&i);
    }
}
