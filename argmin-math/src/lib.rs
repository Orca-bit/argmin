// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! argmin-math provides mathematics related abstractions needed in argmin. It supports
//! implementations of these abstractions for basic `Vec`s and for `ndarray` and `nalgebra`.
//! The traits can of course also be implemented for your own types to make them compatible with
//! argmin.
//!
//! # Usage
//!
//! Add the following line to your dependencies list:
//!
//! ```toml
//! [dependencies]
#![doc = concat!(" argmin-math = \"", env!("CARGO_PKG_VERSION"), "\"")]
//! ```
//!
//! This will activate the `primitives` and `vec` features. For other backends see the section
//! below.
//!
//! ## Features
//!
//! Support for the various backends can be switched on via features:
//!
//! | Feature                | Default | Backend                                               |
//! |------------------------|---------|-------------------------------------------------------|
//! | `primitives`           | yes     | basic integer and floating point types                |
//! | `vec`                  | yes     | `Vec`s (basic functionality)                          |
//! | `ndarray_latest`       | no      | `ndarray` (latest supported version)                  |
//! | `ndarray_latest-serde` | no      | `ndarray` (latest supported version) + serde support  |
//! | `ndarray_v0_15`        | no      | `ndarray` (version 0.15)                              |
//! | `ndarray_v0_15-serde`  | no      | `ndarray` (version 0.15) + serde support              |
//! | `ndarray_v0_14`        | no      | `ndarray` (version 0.14)                              |
//! | `ndarray_v0_14-serde`  | no      | `ndarray` (version 0.14) + serde support              |
//! | `ndarray_v0_13`        | no      | `ndarray` (version 0.13)                              |
//! | `ndarray_v0_13-serde`  | no      | `ndarray` (version 0.13) + serde support              |
//! | `nalgebra_latest`      | no      | `nalgebra` (latest supported version)                 |
//! | `nalgebra_latest-serde`| no      | `nalgebra` (latest supported version) + serde support |
//! | `nalgebra_v0_30`       | no      | `nalgebra` (version 0.30)                             |
//! | `nalgebra_v0_30-serde` | no      | `nalgebra` (version 0.30) + serde support             |
//! | `nalgebra_v0_29`       | no      | `nalgebra` (version 0.29)                             |
//! | `nalgebra_v0_29-serde` | no      | `nalgebra` (version 0.29) + serde support             |
//!
//! It is not possible to activate two versions of the same backend.
//!
//! The features labelled `*_latest*` are an alias for the most recent supported version of the
//! respective backend.
//!
//! Note that `argmin` by default compiles with `serde` support. Therefore, unless `serde` is
//! deliberately turned off in `argmin`, it is necessary to activiate the `serde` support in
//! `argmin-math` as well.
//!
//! The default features `primitives` and `vec` can be turned off in order to only compile the
//! trait definitions. If another backend is chosen, they will automatically be turned on again.
//!
//! Using the `ndarray_*` features on Windows might require to explicitly choose the
//! `ndarray-linalg` BLAS backend in the `Cargo.toml` (see the [`ndarray-linalg` documentation for
//! details](https://github.com/rust-ndarray/ndarray-linalg)):
//!
//! ```toml
//! ndarray-linalg = { version = "*", features = ["intel-mkl-static"] }
//! ```
//!
//! ### Example
//!
//! Activate support for the latest supported `ndarray` version:
//!
//! ```toml
//! [dependencies]
#![doc = concat!(" argmin-math = { version = \"", env!("CARGO_PKG_VERSION"), "\", features = [\"ndarray_latest-serde\"] }")]
//! ```
//!
//! # Contributing
//!
//! You found a bug? Your favourite backend is not supported? Feel free to open an issue or ideally
//! submit a PR.
//!
//! # License
//!
//! Licensed under either of
//!
//!   * Apache License, Version 2.0,
//!     ([LICENSE-APACHE](https://github.com/argmin-rs/argmin/blob/main/LICENSE-APACHE) or
//!     <http://www.apache.org/licenses/LICENSE-2.0>)
//!   * MIT License ([LICENSE-MIT](https://github.com/argmin-rs/argmin/blob/main/LICENSE-MIT) or
//!     <http://opensource.org/licenses/MIT>)
//!
//! at your option.
//!
//! ## Contribution
//!
//! Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion
//! in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above,
//! without any additional terms or conditions.

#![warn(missing_docs)]
// Explicitly disallow EQ comparison of floats. (This clippy lint is denied by default; however,
// this is just to make sure that it will always stay this way.)
#![deny(clippy::float_cmp)]

cfg_if::cfg_if! {
    if #[cfg(feature = "nalgebra_v0_30")] {
        extern crate nalgebra_0_30 as nalgebra;
    } else if #[cfg(feature = "nalgebra_v0_29")] {
        extern crate nalgebra_0_29 as nalgebra;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ndarray_v0_15")] {
        extern crate ndarray_0_15 as ndarray;
    } else if #[cfg(feature = "ndarray_v0_14")] {
        extern crate ndarray_0_14 as ndarray;
    } else if #[cfg(feature = "ndarray_v0_13")] {
        extern crate ndarray_0_13 as ndarray;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ndarray-linalg_0_14")] {
        extern crate ndarray_linalg_0_14 as ndarray_linalg;
    } else if #[cfg(feature = "ndarray-linalg_0_13")] {
        extern crate ndarray_linalg_0_13 as ndarray_linalg;
    } else if #[cfg(feature = "ndarray-linalg_0_12")] {
        extern crate ndarray_linalg_0_12 as ndarray_linalg;
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "num-complex_0_2")] {
        extern crate num_complex_0_2 as num_complex;
    } else if #[cfg(feature = "num-complex_0_3")] {
        extern crate num_complex_0_3 as num_complex;
    } else if #[cfg(feature = "num-complex_0_4")] {
        extern crate num_complex_0_4 as num_complex;
    }
}

#[cfg(feature = "primitives")]
mod primitives;
#[cfg(feature = "primitives")]
pub use crate::primitives::*;

#[cfg(feature = "ndarray_all")]
mod ndarray_m;
#[cfg(feature = "ndarray_all")]
pub use crate::ndarray_m::*;

#[cfg(feature = "nalgebra_all")]
mod nalgebra_m;
#[cfg(feature = "nalgebra_all")]
pub use crate::nalgebra_m::*;

#[cfg(feature = "vec")]
mod vec;
#[cfg(feature = "vec")]
pub use crate::vec::*;

use anyhow::Error;

/// Dot/scalar product of `T` and `self`
pub trait ArgminDot<T, U> {
    /// Dot/scalar product of `T` and `self`
    fn dot(&self, other: &T) -> U;
}

/// Dot/scalar product of `T` and `self` weighted by W (p^TWv)
pub trait ArgminWeightedDot<T, U, V> {
    /// Dot/scalar product of `T` and `self`
    fn weighted_dot(&self, w: &V, vec: &T) -> U;
}

/// Return param vector of all zeros (for now, this is a hack. It should be done better)
pub trait ArgminZero {
    /// Return zero(s)
    fn zero() -> Self;
}

/// Return the conjugate
pub trait ArgminConj {
    /// Return conjugate
    #[must_use]
    fn conj(&self) -> Self;
}

/// Zero for dynamically sized objects
pub trait ArgminZeroLike {
    /// Return zero(s)
    #[must_use]
    fn zero_like(&self) -> Self;
}

/// Identity matrix
pub trait ArgminEye {
    /// Identity matrix of size `n`
    fn eye(n: usize) -> Self;
    /// Identity matrix of same size as `self`
    #[must_use]
    fn eye_like(&self) -> Self;
}

/// Add a `T` to `self`
pub trait ArgminAdd<T, U> {
    /// Add a `T` to `self`
    fn add(&self, other: &T) -> U;
}

/// Subtract a `T` from `self`
pub trait ArgminSub<T, U> {
    /// Subtract a `T` from `self`
    fn sub(&self, other: &T) -> U;
}

/// (Pointwise) Multiply a `T` with `self`
pub trait ArgminMul<T, U> {
    /// (Pointwise) Multiply a `T` with `self`
    fn mul(&self, other: &T) -> U;
}

/// (Pointwise) Divide a `T` by `self`
pub trait ArgminDiv<T, U> {
    /// (Pointwise) Divide a `T` by `self`
    fn div(&self, other: &T) -> U;
}

/// Add a `T` scaled by an `U` to `self`
pub trait ArgminScaledAdd<T, U, V> {
    /// Add a `T` scaled by an `U` to `self`
    fn scaled_add(&self, factor: &U, vec: &T) -> V;
}

/// Subtract a `T` scaled by an `U` from `self`
pub trait ArgminScaledSub<T, U, V> {
    /// Subtract a `T` scaled by an `U` from `self`
    fn scaled_sub(&self, factor: &U, vec: &T) -> V;
}

/// Compute the l2-norm (`U`) of `self`
pub trait ArgminNorm<U> {
    /// Compute the l2-norm (`U`) of `self`
    fn norm(&self) -> U;
}

// Suboptimal: self is moved. ndarray however offers array views...
/// Return the transpose (`U`) of `self`
pub trait ArgminTranspose<U> {
    /// Transpose
    fn t(self) -> U;
}

/// Compute the inverse (`T`) of `self`
pub trait ArgminInv<T> {
    /// Compute the inverse
    fn inv(&self) -> Result<T, Error>;
}

/// Create a random number
pub trait ArgminRandom {
    /// Get a random element between min and max,
    fn rand_from_range(min: &Self, max: &Self) -> Self;
}

/// Minimum and Maximum of type `T`
pub trait ArgminMinMax {
    /// Select piecewise minimum
    fn min(x: &Self, y: &Self) -> Self;
    /// Select piecewise maximum
    fn max(x: &Self, y: &Self) -> Self;
}
