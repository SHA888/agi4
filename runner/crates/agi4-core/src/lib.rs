//! Pure verdict logic for AGI/4 attestation.
//!
//! This crate contains the core types and verdict function with zero external dependencies
//! beyond serialization. The verdict function is pure and total: given valid input, it always
//! returns a verdict with no panics or side effects.

pub mod conjunct;
pub mod consistency;
pub mod evidence;
pub mod threshold;
pub mod verdict;

pub use conjunct::{Conjunct, ConjunctStatus};
pub use evidence::Evidence;
pub use verdict::Verdict;
