//! Canonical upstream source identifiers.
//!
//! Defines the normalized source IDs used throughout the verdict pipeline.
//! All code paths (evaluators, consistency check) must reference these constants
//! to prevent source-ID drift.

/// Generality conjunct sources
pub mod generality {
    pub const ARC_AGI_2: &str = "arc-agi-2";
    pub const ARC_AGI_3: &str = "arc-agi-3";
    pub const HLE: &str = "hle";
    pub const GPQA_DIAMOND: &str = "gpqa-diamond";

    pub fn all() -> &'static [&'static str] {
        &[ARC_AGI_2, ARC_AGI_3, HLE, GPQA_DIAMOND]
    }
}

/// Economic Substitutability conjunct sources
pub mod economic_substitutability {
    pub const GDPVAL: &str = "gdpval";
    pub const GDPVAL_AA: &str = "gdpval-aa";
    pub const RLI: &str = "rli";
    pub const APEX_AGENTS: &str = "apex-agents";

    pub fn all() -> &'static [&'static str] {
        &[GDPVAL, GDPVAL_AA, RLI, APEX_AGENTS]
    }
}

/// Environmental Transfer conjunct sources
pub mod environmental_transfer {
    pub const ARC_AGI_3: &str = "arc-agi-3";
    pub const OSWORLD: &str = "osworld";
    pub const NES: &str = "nes";

    pub fn all() -> &'static [&'static str] {
        &[ARC_AGI_3, OSWORLD, NES]
    }
}

/// Autonomous Agency conjunct sources
pub mod autonomous_agency {
    pub const METR_80PCT_TIME_HORIZON: &str = "metr-80pct-time-horizon";
    pub const RE_BENCH: &str = "re-bench";
    pub const SWE_BENCH_VERIFIED: &str = "swe-bench-verified";

    pub fn all() -> &'static [&'static str] {
        &[METR_80PCT_TIME_HORIZON, RE_BENCH, SWE_BENCH_VERIFIED]
    }
}

/// All unique canonical upstream source IDs.
pub fn all_unique_sources() -> &'static [&'static str] {
    &[
        generality::ARC_AGI_2,
        generality::ARC_AGI_3,
        generality::HLE,
        generality::GPQA_DIAMOND,
        economic_substitutability::GDPVAL,
        economic_substitutability::GDPVAL_AA,
        economic_substitutability::RLI,
        economic_substitutability::APEX_AGENTS,
        environmental_transfer::OSWORLD,
        environmental_transfer::NES,
        autonomous_agency::METR_80PCT_TIME_HORIZON,
        autonomous_agency::RE_BENCH,
        autonomous_agency::SWE_BENCH_VERIFIED,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_unique_sources_are_unique() {
        let all = all_unique_sources();
        let mut seen = std::collections::HashSet::new();
        for source in all {
            assert!(seen.insert(source), "Duplicate source ID: {}", source);
        }
    }

    #[test]
    fn conjunct_source_lists_are_correct() {
        assert_eq!(generality::all().len(), 4);
        assert_eq!(economic_substitutability::all().len(), 4);
        assert_eq!(environmental_transfer::all().len(), 3);
        assert_eq!(autonomous_agency::all().len(), 3);
    }
}
