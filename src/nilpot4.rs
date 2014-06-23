
use std::num::One;
use std::num::Zero;

use {
    mul,
    inv
};

/// Represents a nilpotent 2 number.
#[deriving(Copy)]
pub struct Nilpot4<T>(pub [T, ..4]);

impl<T: Num> Num for Nilpot4<T> {

}

impl<T: Add<T, T>> Add<Nilpot4<T>, Nilpot4<T>> for Nilpot4<T> {
    #[inline(always)]
    fn add(&self, rhs: &Nilpot4<T>) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        let Nilpot4(ref b) = *rhs;
        Nilpot4([a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3]])
    }
}

impl<T: Sub<T, T>> Sub<Nilpot4<T>, Nilpot4<T>> for Nilpot4<T> {
    #[inline(always)]
    fn sub(&self, rhs: &Nilpot4<T>) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        let Nilpot4(ref b) = *rhs;
        Nilpot4([a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3]])
    }
}

impl<T: Rem<T, T>> Rem<Nilpot4<T>, Nilpot4<T>> for Nilpot4<T> {
   #[inline(always)]
   fn rem(&self, rhs: &Nilpot4<T>) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        let Nilpot4(ref b) = *rhs;
        Nilpot4([a[0] % b[0], a[1] % b[1], a[2] % b[2], a[3] % b[3]])
    }
}

impl<T: Neg<T>> Neg<Nilpot4<T>> for Nilpot4<T> {
    #[inline(always)]
    fn neg(&self) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        Nilpot4([a[0].neg(), a[1].neg(), a[2].neg(), a[3].neg()])
    }
}

impl<T: Num + Copy> One for Nilpot4<T> {
    #[inline(always)]
    fn one() -> Nilpot4<T> {
        Nilpot4([One::one(), ..4])
    }
}

impl<T: Zero + Copy> Zero for Nilpot4<T> {
    #[inline(always)]
    fn zero() -> Nilpot4<T> {
        Nilpot4([Zero::zero(), ..4])
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        let Nilpot4(ref a) = *self;
        a.iter().all(|a| a.is_zero())
    }
}

impl<T: PartialEq> PartialEq for Nilpot4<T> {
    #[inline(always)]
    fn eq(&self, rhs: &Nilpot4<T>) -> bool {
        let Nilpot4(ref a) = *self;
        let Nilpot4(ref b) = *rhs;
        a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Num + Copy> Mul<Nilpot4<T>, Nilpot4<T>> for Nilpot4<T> {
    #[inline(always)]
    fn mul(&self, &Nilpot4(ref b): &Nilpot4<T>) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        let mut res = [Zero::zero(), ..4];
        mul(a.as_slice(), b.as_slice(), res.as_mut_slice());
        Nilpot4(res)
    }
}

impl<T: Num + Copy> Div<Nilpot4<T>, Nilpot4<T>> for Nilpot4<T> {
    #[inline(always)]
    fn div(&self, &Nilpot4(ref b): &Nilpot4<T>) -> Nilpot4<T> {
        let Nilpot4(ref a) = *self;
        let mut b_inv = [Zero::zero(), ..4];
        inv(b.as_slice(), b_inv.as_mut_slice());
        let mut res = [Zero::zero(), ..4];
        mul(a.as_slice(), b_inv.as_slice(), res.as_mut_slice());
        Nilpot4(res)
    }
}

#[test]
fn test_div() {
    let a = Nilpot4([1.0, 0.0, 0.0, 0.0]);
    let b = Nilpot4([3.2, 23.1, 22.0, 15.0]);
    let c = a * b / b;
    assert!(c == Nilpot4([1.0, 0.0, 0.0, 0.0]));
}
