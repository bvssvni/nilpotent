
use std::num::One;
use std::num::Zero;

use {
    mul,
    inv
};

/// Represents a nilpotent 2 number.
#[deriving(Copy)]
pub struct Nilpot3<T>(pub [T, ..3]);

impl<T: Num> Num for Nilpot3<T> {

}

impl<T: Add<T, T>> Add<Nilpot3<T>, Nilpot3<T>> for Nilpot3<T> {
    #[inline(always)]
    fn add(&self, rhs: &Nilpot3<T>) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        let Nilpot3(ref b) = *rhs;
        Nilpot3([a[0] + b[0], a[1] + b[1], a[2] + b[2]])
    }
}

impl<T: Sub<T, T>> Sub<Nilpot3<T>, Nilpot3<T>> for Nilpot3<T> {
    #[inline(always)]
    fn sub(&self, rhs: &Nilpot3<T>) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        let Nilpot3(ref b) = *rhs;
        Nilpot3([a[0] - b[0], a[1] - b[1], a[2] - b[2]])
    }
}

impl<T: Rem<T, T>> Rem<Nilpot3<T>, Nilpot3<T>> for Nilpot3<T> {
   #[inline(always)]
   fn rem(&self, rhs: &Nilpot3<T>) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        let Nilpot3(ref b) = *rhs;
        Nilpot3([a[0] % b[0], a[1] % b[1], a[2] % b[2]])
    }
}

impl<T: Neg<T>> Neg<Nilpot3<T>> for Nilpot3<T> {
    #[inline(always)]
    fn neg(&self) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        Nilpot3([a[0].neg(), a[1].neg(), a[2].neg()])
    }
}

impl<T: Num + Copy> One for Nilpot3<T> {
    #[inline(always)]
    fn one() -> Nilpot3<T> {
        Nilpot3([One::one(), ..3])
    }
}

impl<T: Zero + Copy> Zero for Nilpot3<T> {
    #[inline(always)]
    fn zero() -> Nilpot3<T> {
        Nilpot3([Zero::zero(), ..3])
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        let Nilpot3(ref a) = *self;
        a.iter().all(|a| a.is_zero())
    }
}

impl<T: PartialEq> PartialEq for Nilpot3<T> {
    #[inline(always)]
    fn eq(&self, rhs: &Nilpot3<T>) -> bool {
        let Nilpot3(ref a) = *self;
        let Nilpot3(ref b) = *rhs;
        a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Num + Copy> Mul<Nilpot3<T>, Nilpot3<T>> for Nilpot3<T> {
    #[inline(always)]
    fn mul(&self, &Nilpot3(ref b): &Nilpot3<T>) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        let mut res = [Zero::zero(), ..3];
        mul(a.as_slice(), b.as_slice(), res.as_mut_slice());
        Nilpot3(res)
    }
}

impl<T: Num + Copy> Div<Nilpot3<T>, Nilpot3<T>> for Nilpot3<T> {
    #[inline(always)]
    fn div(&self, &Nilpot3(ref b): &Nilpot3<T>) -> Nilpot3<T> {
        let Nilpot3(ref a) = *self;
        let mut b_inv = [Zero::zero(), ..3];
        inv(b.as_slice(), b_inv.as_mut_slice());
        let mut res = [Zero::zero(), ..3];
        mul(a.as_slice(), b_inv.as_slice(), res.as_mut_slice());
        Nilpot3(res)
    }
}

#[test]
fn test_div() {
    let a = Nilpot3([1.0, 0.0, 0.0]);
    let b = Nilpot3([3.2, 23.1, 22.0]);
    let c = a * b / b;
    assert!(c == Nilpot3([1.0, 0.0, 0.0]));
}
