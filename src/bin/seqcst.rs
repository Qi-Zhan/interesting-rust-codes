//! Code about understanding memory order, atmoics
//! Copy from <https://www.youtube.com/watch?v=rMGWeSjctlY&t=4382s>

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;

fn foo() -> usize {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    thread::spawn(move || {
        x.store(true, Ordering::Release);
    });
    thread::spawn(move || {
        y.store(true, Ordering::Release);
    });

    let t1 = thread::spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        while !x.load(Ordering::Acquire) {}
        if y.load(Ordering::Acquire) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    z.load(Ordering::SeqCst)
}

fn foo_seqcst() -> usize {
    let x: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let y: &'static _ = Box::leak(Box::new(AtomicBool::new(false)));
    let z: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    thread::spawn(move || {
        x.store(true, Ordering::Release);
    });

    thread::spawn(move || {
        y.store(true, Ordering::Release);
    });

    let t1 = thread::spawn(move || {
        while !x.load(Ordering::SeqCst) {}
        if y.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    let t2 = thread::spawn(move || {
        while !y.load(Ordering::SeqCst) {}
        if x.load(Ordering::SeqCst) {
            z.fetch_add(1, Ordering::Relaxed);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    z.load(Ordering::SeqCst)
}
fn main() {
    let mut set = std::collections::HashSet::new();
    for _ in 0.. {
        let result = foo();
        set.insert(result);
        if set.len() == 3 {
            break;
        }
    }
    println!("release & acquire {:?}", set);
    set.clear();
    for _ in 0..100000 {
        let result = foo_seqcst();
        set.insert(result);
        if set.len() == 3 {
            panic!("seqcst cannot be reordered")
        }
    }
    println!("seqcst {:?}", set)
}
