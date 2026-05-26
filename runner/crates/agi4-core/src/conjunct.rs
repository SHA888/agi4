//! Conjunct definitions and status enumeration.

use serde::{Deserialize, Serialize};

/// The four AGI/4 conjuncts, all of which must pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Conjunct {
    Generality,
    EconomicSubstitutability,
    EnvironmentalTransfer,
    AutonomousAgency,
}

/// Per-conjunct status after evaluation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConjunctStatus {
    Pass,
    Partial,
    Fail,
    InsufficientData,
}
