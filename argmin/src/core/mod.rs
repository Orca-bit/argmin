// Copyright 2018-2022 argmin developers
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! Argmin Optimization toolbox core
//!
//! This module contains the traits and types necessary for implementing optimization algorithms
//! and tools for observing the state of optimization runs and checkpointing.

/// Macros
#[macro_use]
pub mod macros;
/// Checkpointing
pub mod checkpointing;
/// Error handling
mod errors;
/// Executor
mod executor;
/// Trait alias for float types
mod float;
/// Key value datastructure
mod kv;
/// Observers
pub mod observers;
/// Traits and structs for defining and handling optimization problems
mod problem;
/// Definition of the return type of the solvers
mod result;
/// Trait alias for `serde`s `Serialize` and `DeserializeOwned`
mod serialization;
/// `Solver` trait
mod solver;
/// iteration state
mod state;
/// Definition of termination reasons
mod termination;
/// Convenience utilities for testing
pub mod test_utils;

pub use crate::solver::conjugategradient::beta::NLCGBetaUpdate;
pub use crate::solver::linesearch::LineSearch;
pub use crate::solver::trustregion::TrustRegionRadius;
pub use anyhow::Error;
pub use errors::ArgminError;
pub use executor::Executor;
pub use float::ArgminFloat;
pub use kv::KV;
pub use problem::{CostFunction, Gradient, Hessian, Jacobian, LinearProgram, Operator, Problem};
pub use result::OptimizationResult;
pub use serialization::{DeserializeOwnedAlias, SerializeAlias};
pub use solver::Solver;
pub use state::{IterState, LinearProgramState, State};
pub use termination::TerminationReason;
