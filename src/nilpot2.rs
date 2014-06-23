use std::num::One;
use std::num::Zero;

use {
    mul,
    inv
};

/// Represents a nilpotent 2 number.
#[deriving(Copy)]
pub struct Nilpot2<T>(pub [T, ..2]);

impl<T: Num> Num for Nilpot2<T> {

}

impl<T: Add<T, T>> Add<Nilpot2<T>, Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn add(&self, rhs: &Nilpot2<T>) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        let Nilpot2(ref b) = *rhs;
        Nilpot2([a[0] + b[0], a[1] + b[1]])
    }
}

impl<T: Sub<T, T>> Sub<Nilpot2<T>, Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn sub(&self, rhs: &Nilpot2<T>) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        let Nilpot2(ref b) = *rhs;
        Nilpot2([a[0] - b[0], a[1] - b[1]])
    }
}

impl<T: Rem<T, T>> Rem<Nilpot2<T>, Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn rem(&self, rhs: &Nilpot2<T>) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        let Nilpot2(ref b) = *rhs;
        Nilpot2([a[0] % b[0], a[1] % b[1]])
    }
}

impl<T: Neg<T>> Neg<Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn neg(&self) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        Nilpot2([a[0].neg(), a[1].neg()])
    }
}

impl<T: Num + Copy> One for Nilpot2<T> {
    #[inline(always)]
    fn one() -> Nilpot2<T> {
        Nilpot2([One::one(), ..2])
    }
}

impl<T: Zero + Copy> Zero for Nilpot2<T> {
    #[inline(always)]
    fn zero() -> Nilpot2<T> {
        Nilpot2([Zero::zero(), ..2])
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        let Nilpot2(ref a) = *self;
        a.iter().all(|a| a.is_zero())
    }
}

impl<T: PartialEq> PartialEq for Nilpot2<T> {
    #[inline(always)]
    fn eq(&self, rhs: &Nilpot2<T>) -> bool {
        let Nilpot2(ref a) = *self;
        let Nilpot2(ref b) = *rhs;
        a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Num + Copy> Mul<Nilpot2<T>, Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn mul(&self, &Nilpot2(ref b): &Nilpot2<T>) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        let mut res = [Zero::zero(), ..2];
        mul(a.as_slice(), b.as_slice(), res.as_mut_slice());
        Nilpot2(res)
    }
}

impl<T: Num + Copy> Div<Nilpot2<T>, Nilpot2<T>> for Nilpot2<T> {
    #[inline(always)]
    fn div(&self, &Nilpot2(ref b): &Nilpot2<T>) -> Nilpot2<T> {
        let Nilpot2(ref a) = *self;
        let mut b_inv = [Zero::zero(), ..2];
        inv(b.as_slice(), b_inv.as_mut_slice());
        let mut res = [Zero::zero(), ..2];
        mul(a.as_slice(), b_inv.as_slice(), res.as_mut_slice());
        Nilpot2(res)
    }
}

#[test]
fn test_div() {
    let a = Nilpot2([1.0, 0.0]);
    let b = Nilpot2([3.2, 23.1]);
    let c = a * b / b;
    assert!(c == Nilpot2([1.0, 0.0]));
}
