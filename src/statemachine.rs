//! ## TypeState  example
//! 
//! copy from [cs242](https://stanford-cs242.github.io/f19/lectures/08-2-typestate)
//!
//! Benifits:
//! - Starting in a non-start state is a compile error
//! 
//! struct without a constructor cannot be created, sicne field is private
//! 
//! - Taking an incorrect transition is a compile error
//! - Reusing old states is a compile error
//! 
//! function parameter is consumed, i.e. `self` is consumed, so you cannot use it again

pub struct A {
    _secret: (),
}
pub struct B {
    _secret: (),
}
pub struct C {
    _secret: (),
}

impl A {
    #[allow(clippy::new_without_default)]
    pub fn new() -> A {
        A { _secret: () }
    }
    pub fn b(self) -> B {
        B { _secret: () }
    }
}

impl B {
    pub fn b(self) -> B {
        B { _secret: () }
    }
    pub fn c(self) -> C {
        C { _secret: () }
    }
}

impl C {
    pub fn a(self) -> A {
        A { _secret: () }
    }
}
