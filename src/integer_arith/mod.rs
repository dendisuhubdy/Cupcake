// Copyright (c) Facebook, Inc. and its affiliates.
//
// This source code is licensed under the MIT license found in the
// LICENSE file in the root directory of this source tree.
pub mod scalar;

#[cfg(feature = "bigint")]
pub mod bigint;

use rand::StdRng;
/// The trait for utility functions related to scalar-like types.
pub trait ArithUtils<T> {
    fn modulus(a: &T, q: &T) -> T;

    fn double(a: &T) -> T;

    // sample a value in [0, bound-1]
    fn sample_blw(bound: &T) -> T;

    fn sample_below_from_rng(bound: &T, rng: &mut StdRng) -> T;

    fn one() -> T {
        Self::from_u32_raw(1u32)
    }

    fn zero() -> T {
        Self::from_u32_raw(0u32)
    }

    fn add_mod(a: &T, b: &T, q: &T) -> T;
    fn sub_mod(a: &T, b: &T, q: &T) -> T;
    fn mul_mod(a: &T, b: &T, q: &T) -> T;
    fn inv_mod(a: &T, q: &T) -> T;

    fn from_u32(a: u32, q: &T) -> T;

    fn pow_mod(a: &T, b: &T, c: &T) -> T;

    fn add(a: &T, b: &T) -> T;

    fn sub(a: &T, b: &T) -> T;

    fn div(a: &T, b: &T) -> T;

    fn mul(a: &T, b: &T) -> T;

    // conversion
    fn from_u32_raw(a: u32) -> T;
    fn from_u64_raw(a: u64) -> T;
    fn to_u64(a: T) -> u64;
}
