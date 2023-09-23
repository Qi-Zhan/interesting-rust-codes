//! # TypeState Example
//!
//! We can use typestate to model state machine, which benifits from Rust's type system.
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
//!
//! # Example
//! ```
//! use interesting_rust_codes::statemachine::*;  
//! let a = A::new();
//! let b = a.b();
//! let b = b.b();
//! let c = b.c();
//! let _a = c.a();
//! ```
//! next example try to create a A instance directly, which will not compile
//!
//! ```compile_fail
//! use interesting_rust_codes::statemachine::*;
//! let a = A { _secret: () };
//! ```
//! next example try to reuse old states, which will not compile
//!
//! ```compile_fail
//! use interesting_rust_codes::statemachine::*;
//! let a = A::new();
//! let b = a.b();
//! let _a = a.b();
//! ```
//!
//! next example try to take an incorrect transition, which will not compile
//! ```compile_fail
//! use interesting_rust_codes::statemachine::*;
//! let a = A::new();
//! let c = a.c();
//! ```
//!
//! # Reference
//! [cs242](https://stanford-cs242.github.io/f19/lectures/08-2-typestate)

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
