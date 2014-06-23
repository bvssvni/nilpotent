#![crate_id = "nilpotent"]
#![deny(missing_doc)]

//! Nilpotent numbers are numbers with a property
//! that raised to some power N they equal zero.
//!
//! They can be used to perform numerical analysis
//! around a local coordinate.

#[cfg(test)]
extern crate debug;

use std::num::One;
use std::num::Zero;

pub use nilpot2::Nilpot2;
pub use nilpot3::Nilpot3;
pub use nilpot4::Nilpot4;
pub use nilpot5::Nilpot5;

mod nilpot2;
mod nilpot3;
mod nilpot4;
mod nilpot5;

/// Multiplies two nilpotent numbers.
///
/// Assumes all numbers of having equal number of components.
#[inline(always)]
pub fn mul<T: Num>(a: &[T], b: &[T], res: &mut [T]) {
    for i in range(0, res.len()) {
        res[i] = Zero::zero();
        for j in range(0, i + 1) {
            res[i] = res[i] + a[j] * b[i - j];
        }
    }
}

/// Inverts a nilpotent number.
///
/// Assumes all numbers of having equal number of components.
#[inline(always)]
pub fn inv<T: Num>(a: &[T], res: &mut [T]) {
    let one: T = One::one();
    res[0] = one / a[0];
    for i in range(1, res.len()) {
        res[i] = Zero::zero();
        for j in range(1, i + 1) {
            res[i] = res[i] - a[j] * res[i - j];
        }
        res[i] = res[i] / a[0];
    }
}

