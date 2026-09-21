//! Cryptographic provenance receipts for ingested data.
//!
//! Every byte that enters the Earth Twin must carry a `ProvenanceReceipt`
//! that records *exactly* where it came from, *when* it was fetched, and
//! a SHA-256 hash of the raw payload it was derived from.  This makes
//! every datum auditable and reversible — if a source is found to be
//! unreliable, every observation derived from it can be identified and
//! quarantined by its `source` field.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::schema::DataSource;

/// A sealed, cryptographically-bound provenance receipt.
///
/// Constructed via [`ProvenanceBuilder`]; the `sha256` field is computed
/// by `seal()` and cannot be set manually.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProvenanceReceipt {
    /// Which external data source produced the raw payload.
    pub source: DataSource,
    /// The URL or endpoint from which the raw payload was fetched.
    pub source_url: String,
    /// Source-native record identifier (e.g. GBIF occurrenceKey, NOAA station ID).
    pub external_id: String,
    /// Unix timestamp (seconds) when the payload was fetched by GAIA.
    pub fetched_at_unix: u64,
    /// Unix timestamp (seconds) of the underlying observation.
    pub observed_at_unix: u64,
    /// Lower-hex SHA-256 of the raw bytes from which this record was derived.
    pub sha256: String,
    /// SPDX license identifier for this data item.
    pub license: String,
}

impl ProvenanceReceipt {
    /// Returns `true` if the receipt is internally self-consistent:
    /// - `fetched_at_unix >= observed_at_unix`
    /// - `sha256` is a 64-character hex string
    /// - no required string fields are empty
    pub fn is_valid(&self) -> bool {
        self.fetched_at_unix >= self.observed_at_unix
            && self.sha256.len() == 64
            && self.sha256.chars().all(|c| c.is_ascii_hexdigit())
            && !self.source_url.is_empty()
            && !self.external_id.is_empty()
            && !self.license.is_empty()
    }
}

/// Builder for [`ProvenanceReceipt`].  Call `seal(data)` to compute the
/// SHA-256 and obtain an immutable receipt.
pub struct ProvenanceBuilder {
    source: DataSource,
    source_url: String,
    external_id: String,
    fetched_at_unix: u64,
    observed_at_unix: u64,
    license: Option<String>,
}

impl ProvenanceBuilder {
    pub fn new(
        source: DataSource,
        source_url: impl Into<String>,
        external_id: impl Into<String>,
        fetched_at_unix: u64,
        observed_at_unix: u64,
    ) -> Self {
        Self {
            source,
            source_url: source_url.into(),
            external_id: external_id.into(),
            fetched_at_unix,
            observed_at_unix,
            license: None,
        }
    }

    /// Override the default license for this source.
    pub fn license(mut self, license: impl Into<String>) -> Self {
        self.license = Some(license.into());
        self
    }

    /// Compute SHA-256 of `data` and return a sealed [`ProvenanceReceipt`].
    ///
    /// # Errors
    /// Returns [`ProvenanceError::FetchedBeforeObserved`] if timestamps are
    /// inconsistent, or [`ProvenanceError::EmptyField`] if any required string
    /// is empty.
    pub fn seal(self, data: &[u8]) -> Result<ProvenanceReceipt, ProvenanceError> {
        if self.fetched_at_unix < self.observed_at_unix {
            return Err(ProvenanceError::FetchedBeforeObserved {
                fetched: self.fetched_at_unix,
                observed: self.observed_at_unix,
            });
        }
        if self.source_url.is_empty() {
            return Err(ProvenanceError::EmptyField("source_url"));
        }
        if self.external_id.is_empty() {
            return Err(ProvenanceError::EmptyField("external_id"));
        }
        let sha256 = hex::encode(Sha256::digest(data));
        let license = self
            .license
            .unwrap_or_else(|| self.source.default_license().into());
        Ok(ProvenanceReceipt {
            source: self.source,
            source_url: self.source_url,
            external_id: self.external_id,
            fetched_at_unix: self.fetched_at_unix,
            observed_at_unix: self.observed_at_unix,
            sha256,
            license,
        })
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum ProvenanceError {
    #[error("fetched_at_unix ({fetched}) < observed_at_unix ({observed}) — time travel not supported")]
    FetchedBeforeObserved { fetched: u64, observed: u64 },
    #[error("required field '{0}' is empty")]
    EmptyField(&'static str),
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn builder() -> ProvenanceBuilder {
        ProvenanceBuilder::new(
            DataSource::Noaa,
            "https://api.noaa.gov/obs/test",
            "OBS-001",
            1_700_000_001,
            1_700_000_000,
        )
    }

    #[test]
    fn seal_produces_valid_receipt() {
        let receipt = builder().seal(b"raw bytes from noaa").unwrap();
        assert!(receipt.is_valid());
        assert_eq!(receipt.sha256.len(), 64);
        assert_eq!(receipt.license, "CC0-1.0");
    }

    #[test]
    fn seal_sha256_is_deterministic() {
        let r1 = builder().seal(b"same bytes").unwrap();
        let r2 = builder().seal(b"same bytes").unwrap();
        assert_eq!(r1.sha256, r2.sha256);
    }

    #[test]
    fn seal_sha256_differs_on_different_data() {
        let r1 = builder().seal(b"data a").unwrap();
        let r2 = builder().seal(b"data b").unwrap();
        assert_ne!(r1.sha256, r2.sha256);
    }

    #[test]
    fn seal_rejects_fetched_before_observed() {
        let b = ProvenanceBuilder::new(
            DataSource::Noaa,
            "https://api.noaa.gov/obs/test",
            "OBS-002",
            999,        // fetched_at
            1_000,      // observed_at — AFTER fetch, impossible
        );
        assert!(matches!(
            b.seal(b"data"),
            Err(ProvenanceError::FetchedBeforeObserved { .. })
        ));
    }

    #[test]
    fn seal_rejects_empty_source_url() {
        let b = ProvenanceBuilder::new(
            DataSource::Noaa,
            "",          // empty
            "OBS-003",
            1_000,
            1_000,
        );
        assert!(matches!(b.seal(b"data"), Err(ProvenanceError::EmptyField("source_url"))));
    }

    #[test]
    fn license_override_is_respected() {
        let receipt = builder()
            .license("proprietary")
            .seal(b"commercial data")
            .unwrap();
        assert_eq!(receipt.license, "proprietary");
    }
}
