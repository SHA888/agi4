//! Upstream source adapters for AGI/4 evidence ingestion.
//!
//! Each upstream benchmark source implements the Source trait.
//! Adapters handle: fetch → parse → typed validation → Evidence emission.

use agi4_core::evidence::Evidence;
use serde::de::DeserializeOwned;
use std::error::Error;
use url::Url;

/// Stable identifier for an adapter source.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SourceId {
    ArcAgI2,
    ArcAgI3,
    HLE,
    GPQA,
    GDPval,
    RLI,
    APEXAgents,
    OSWorld,
    REBench,
    SWEBench,
    METR,
}

/// Model identifier for evidence ingestion.
#[derive(Debug, Clone)]
pub struct ModelId(pub String);

/// The Source trait: each upstream source implements this.
pub trait Source {
    type Raw: DeserializeOwned;
    type Error: Error + Send + Sync + 'static;

    fn id(&self) -> SourceId;
    fn endpoint(&self) -> &Url;
    fn parse(&self, raw: &str) -> Result<Self::Raw, Self::Error>;
    fn to_evidence(&self, raw: Self::Raw, model: &ModelId) -> Result<Vec<Evidence>, Self::Error>;
}

/// Fetcher abstraction for I/O (HTTP, file, in-memory).
pub trait Fetcher {
    type Error: Error + Send + Sync + 'static;

    fn fetch(&self, url: &Url) -> Result<String, Self::Error>;
}
