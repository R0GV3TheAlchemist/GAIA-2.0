//! #40 ingest policy. Charter is the doc. This crate refuses a ticket that breaks it.

use crate::{allow_purpose, TwinError};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LicenseClass {
    RawCc0,
    ProductCcBy4,
    WeightsApache2,
    SpecCc0,
}

impl LicenseClass {
    pub fn spdx(self) -> &'static str {
        match self {
            Self::RawCc0 | Self::SpecCc0 => "CC0-1.0",
            Self::ProductCcBy4 => "CC-BY-4.0",
            Self::WeightsApache2 => "Apache-2.0",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IngestTicket {
    pub license: LicenseClass,
    pub residency: Option<String>,
    pub process_in: Option<String>,
}

impl IngestTicket {
    pub fn admit(
        license: LicenseClass,
        uncertainty: Option<f64>,
        residency: Option<&str>,
        process_in: Option<&str>,
        purpose: &str,
    ) -> Result<Self, TwinError> {
        allow_purpose(purpose)?;
        let uncertainty = uncertainty.ok_or(TwinError::MissingUncertainty)?;
        if uncertainty < 0.0 {
            return Err(TwinError::MissingUncertainty);
        }
        if let (Some(home), Some(node)) = (residency, process_in) {
            if home != node {
                return Err(TwinError::UnlabeledPoint);
            }
        }
        Ok(Self {
            license,
            residency: residency.map(|s| s.into()),
            process_in: process_in.map(|s| s.into()),
        })
    }
}
