//! AGI/4 specification and reference runner.
//!
//! This is the public-facing facade. Version tracks SPEC.md exactly.
//! Library API re-exports from agi4-core and agi4-schema.

// Re-export core types
pub use agi4_core::conjunct::{Conjunct, ConjunctStatus};
pub use agi4_core::evidence::{Evidence, SourceId, MeasurementId};
pub use agi4_core::verdict::Verdict;
pub use agi4_core::threshold;

// Re-export schema types
pub use agi4_schema::VerdictOutput;

// Crate metadata
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SPEC_VERSION: &str = "0.1.0";
