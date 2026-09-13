//! #82 GAIAN knowledge modes. Not 100 languages and not UKD v1.0.

use crate::UkdError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GaianMode {
    Discover,
    Explore,
    Learn,
    Research,
    Contribute,
}

impl GaianMode {
    pub fn all() -> [GaianMode; 5] {
        [
            Self::Discover,
            Self::Explore,
            Self::Learn,
            Self::Research,
            Self::Contribute,
        ]
    }
}

pub fn teach_offline(path_downloaded: bool, topic: &str) -> Result<String, UkdError> {
    if !path_downloaded {
        return Err(UkdError::UnknownNode);
    }
    Ok(format!("offline pack: {topic}"))
}

pub fn earth_twin_cites(ukd_id: &str) -> Result<String, UkdError> {
    if ukd_id.is_empty() {
        return Err(UkdError::UnknownNode);
    }
    Ok(format!("earth-twin cites {ukd_id}"))
}

pub fn tek_export(community_grant: bool) -> Result<(), UkdError> {
    if !community_grant {
        return Err(UkdError::NoAgreement);
    }
    Ok(())
}

pub fn ukd_v1_tagged() -> bool {
    false
}
