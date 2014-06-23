
use std::num::One;
use std::num::Zero;

use {
    mul,
    inv
};

/// Represents a nilpotent 2 number.
#[deriving(Copy)]
pub struct Nilpot5<T>(pub [T, ..5]);

impl<T: PartialOrd> PartialOrd for Nilpot5<T> {
    fn lt(&self, rhs: &Nilpot5<T>) -> bool {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        a.iter().zip(b.iter()).any(|(a, b)| a < b)
    }
}

impl<T: Signed> Signed for Nilpot5<T> {
    fn abs(&self) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        Nilpot5([a[0].abs(), a[1].abs(), a[2].abs(), a[3].abs(),
            a[4].abs()])
    }
    
    fn abs_sub(&self, rhs: &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        Nilpot5([
            a[0].abs_sub(&b[0]), 
            a[1].abs_sub(&b[1]), 
            a[2].abs_sub(&b[2]), 
            a[3].abs_sub(&b[3]),
            a[4].abs_sub(&b[4])
        ])
    }

    fn signum(&self) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        Nilpot5([
            a[0].signum(),
            a[1].signum(),
            a[2].signum(),
            a[3].signum(),
            a[4].signum()
        ])
    }

    fn is_positive(&self) -> bool {
        let Nilpot5(ref a) = *self;
        a[0].is_positive()
    }
    
    fn is_negative(&self) -> bool {
        let Nilpot5(ref a) = *self;
        a[0].is_negative()
    }
}

impl<T: Num> Num for Nilpot5<T> {

}

impl<T: Add<T, T>> Add<Nilpot5<T>, Nilpot5<T>> for Nilpot5<T> {
    #[inline(always)]
    fn add(&self, rhs: &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        Nilpot5([a[0] + b[0], a[1] + b[1], a[2] + b[2], a[3] + b[3],
            a[4] + b[4]])
    }
}

impl<T: Sub<T, T>> Sub<Nilpot5<T>, Nilpot5<T>> for Nilpot5<T> {
    #[inline(always)]
    fn sub(&self, rhs: &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        Nilpot5([a[0] - b[0], a[1] - b[1], a[2] - b[2], a[3] - b[3],
            a[4] - b[4]])
    }
}

impl<T: Rem<T, T>> Rem<Nilpot5<T>, Nilpot5<T>> for Nilpot5<T> {
   #[inline(always)]
   fn rem(&self, rhs: &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        Nilpot5([a[0] % b[0], a[1] % b[1], a[2] % b[2], a[3] % b[3],
            a[4] % b[4]])
    }
}

impl<T: Neg<T>> Neg<Nilpot5<T>> for Nilpot5<T> {
    #[inline(always)]
    fn neg(&self) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        Nilpot5([a[0].neg(), a[1].neg(), a[2].neg(), a[3].neg(),
            a[4].neg()])
    }
}

impl<T: Num + Copy> One for Nilpot5<T> {
    #[inline(always)]
    fn one() -> Nilpot5<T> {
        Nilpot5([One::one(), ..5])
    }
}

impl<T: Zero + Copy> Zero for Nilpot5<T> {
    #[inline(always)]
    fn zero() -> Nilpot5<T> {
        Nilpot5([Zero::zero(), ..5])
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        let Nilpot5(ref a) = *self;
        a.iter().all(|a| a.is_zero())
    }
}

impl<T: PartialEq> PartialEq for Nilpot5<T> {
    #[inline(always)]
    fn eq(&self, rhs: &Nilpot5<T>) -> bool {
        let Nilpot5(ref a) = *self;
        let Nilpot5(ref b) = *rhs;
        a.iter().zip(b.iter()).all(|(a, b)| a == b)
    }
}

impl<T: Num + Copy> Mul<Nilpot5<T>, Nilpot5<T>> for Nilpot5<T> {
    #[inline(always)]
    fn mul(&self, &Nilpot5(ref b): &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let mut res = [Zero::zero(), ..5];
        mul(a.as_slice(), b.as_slice(), res.as_mut_slice());
        Nilpot5(res)
    }
}

impl<T: Num + Copy> Div<Nilpot5<T>, Nilpot5<T>> for Nilpot5<T> {
    #[inline(always)]
    fn div(&self, &Nilpot5(ref b): &Nilpot5<T>) -> Nilpot5<T> {
        let Nilpot5(ref a) = *self;
        let mut b_inv = [Zero::zero(), ..5];
        inv(b.as_slice(), b_inv.as_mut_slice());
        let mut res = [Zero::zero(), ..5];
        mul(a.as_slice(), b_inv.as_slice(), res.as_mut_slice());
        Nilpot5(res)
    }
}

#[test]
fn test_div() {
    let a = Nilpot5([1.0, 0.0, 0.0, 0.0, 0.0]);
    let b = Nilpot5([3.2, 23.1, 22.0, 15.0, -12.0]);
    let c = a * b / b;
    println!("{:?}", c);
    assert!((c - Nilpot5([1.0, 0.0, 0.0, 0.0, 0.0])).abs() < Nilpot5([0.001, 0.001, 0.001, 0.001, 0.001]));
}


