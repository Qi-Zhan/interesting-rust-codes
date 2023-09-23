//! # Church Numerals
//!
//! - <https://en.wikipedia.org/wiki/Church_encoding>

use std::ops::{Add, Mul};
use std::{cell::RefCell, rc::Rc};

/// Church numerals are represented as higher-order functions that take a function `f`
type ChurchNumber<T> = Rc<dyn Fn(Rc<dyn Fn(T) -> T>) -> Rc<dyn Fn(T) -> T>>;

struct Church<T>(ChurchNumber<T>);

impl<T> Clone for Church<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T: 'static> Church<T> {
    fn zero() -> Self {
        Self(Rc::new(|_| Rc::new(|x| x)))
    }

    /// λn.λf.λx.(f((nf)x))
    fn succ(self) -> Self {
        Self(Rc::new(move |f| {
            let f_clone = f.clone();
            let self_clone = self.0.clone();
            Rc::new(move |x| f_clone((self_clone)(f.clone())(x)))
        }))
    }

    /// λf.λ x. f x
    fn one() -> Self {
        Self::zero().succ()
    }

    /// λf.λ x. f (f x)
    fn two() -> Self {
        Self::one().succ()
    }
}

impl<T: 'static> Add for Church<T> {
    type Output = Self;

    /// λm.λn.λf.λx.((nf)((mf)x))
    fn add(self, rhs: Self) -> Self::Output {
        Self(Rc::new(move |f| {
            let self_clone = self.0.clone();
            let rhs_clone = rhs.0.clone();
            Rc::new(move |x| (rhs_clone(f.clone()))((self_clone(f.clone()))(x)))
        }))
    }
}

impl<T: 'static> Mul for Church<T> {
    type Output = Self;

    /// λm.λn.λf.(m(nf))
    fn mul(self, rhs: Self) -> Self::Output {
        Self(Rc::new(move |f| {
            let self_clone = self.0.clone();
            let rhs_clone = rhs.0.clone();
            Rc::new(move |x| (self_clone(rhs_clone(f.clone())))(x))
        }))
    }
}

impl<T: 'static> From<usize> for Church<T> {
    /// λf.λ x. f (f x)
    fn from(n: usize) -> Self {
        let mut num = Self::zero();
        for _ in 0..n {
            num = num.succ();
        }
        num
    }
}

/// note that this implementation is not efficient
impl<T: Default> From<Church<T>> for usize {
    fn from(n: Church<T>) -> Self {
        let num = Rc::new(RefCell::new(0));
        let num_clone = num.clone();
        let f = Rc::new(move |x| {
            *num_clone.borrow_mut() += 1;
            x
        });
        let _ = n.0(f)(T::default());
        let x = num.borrow();
        *x
    }
}

/// λn.λm.mn
fn exp<T: 'static>(n: usize, m: usize) -> Church<T> {
    let n = Church::from(n);
    let m = Church::from(m);
    Church(m.0(n.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn church_test() {
        let c_zero = Church::<usize>::zero();
        assert_eq!(usize::from(c_zero.clone()), 0);

        let c_one = c_zero.clone().succ();
        assert_eq!(usize::from(c_one.clone()), 1);

        let c_two = c_one.clone().succ();
        let c_three = c_one.clone().add(c_two.clone());
        assert_eq!(usize::from(c_three.clone()), 3);

        let c_plus = c_one.clone() + c_two.clone();
        assert_eq!(usize::from(c_plus), 3);

        let c_product = c_three * c_two;
        assert_eq!(usize::from(c_product), 6);

        let c_exponent = exp::<()>(6, 3);
        assert_eq!(usize::from(c_exponent), 216);

        let c_exponent2 = exp::<()>(3, 0);
        assert_eq!(usize::from(c_exponent2), 1);
    }
}
