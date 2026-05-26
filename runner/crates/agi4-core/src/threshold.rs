//! Threshold constants from SPEC.md §3.
//!
//! Every threshold and floor in the specification lives here as a named const.
//! No magic numbers in evaluation logic. Every value here maps to exactly one
//! source line in SPEC.md.

pub mod generality {
    pub const ARC_AGI_2_PASS: f64 = 0.85;
    pub const ARC_AGI_3_PASS: f64 = 0.50;
    pub const ARC_AGI_3_FLOOR: f64 = 0.05;
    pub const HLE_PASS: f64 = 0.80;
    pub const GPQA_DIAMOND_PASS: f64 = 0.90;
}

pub mod economic_substitutability {
    pub const GDPVAL_PASS: f64 = 0.85;
    pub const RLI_PASS: f64 = 0.60;
    pub const RLI_FLOOR: f64 = 0.10;
    pub const APEX_AGENTS_PASS: f64 = 0.75;
}

pub mod environmental_transfer {
    pub const ARC_AGI_3_PASS: f64 = 0.50;
    pub const ARC_AGI_3_FLOOR: f64 = 0.05;
    pub const OSWORLD_PASS: f64 = 0.85;
    // NES thresholds: TBD in v0.1.x
}

pub mod autonomous_agency {
    pub const METR_80PCT_PASS_HOURS: f64 = 168.0;
    pub const METR_80PCT_FLOOR_HOURS: f64 = 8.0;
    pub const REBENCH_PASS: f64 = 0.60;
    pub const SWEBENCH_VERIFIED_PASS_AT_5: f64 = 0.85;
}

pub mod consistency {
    pub const MARGIN_VARIANCE_RATIO: f64 = 0.5;
}
