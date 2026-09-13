//! #91 teach-me + Earth Twin links. TEK-ecology only if granted.

use crate::{GaianMode, KnowledgeState, UkdError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub topic: String,
    pub interval_days: u8,
}

pub fn teach_me(topic: &str, state: &KnowledgeState) -> Result<String, UkdError> {
    if topic.is_empty() {
        return Err(UkdError::UnknownNode);
    }
    let known = if state.known.iter().any(|k| k == topic) {
        "known"
    } else {
        "new"
    };
    Ok(format!("teach {topic} from-state:{known}"))
}

pub fn sm2(topic: &str, quality: u8) -> Review {
    let interval = if quality >= 4 { 6 } else { 1 };
    Review {
        topic: topic.into(),
        interval_days: interval,
    }
}

pub fn climate_boundary_links(tek_granted: bool) -> Result<Vec<&'static str>, UkdError> {
    let mut links = vec!["ukd:earth-systems:climatology"];
    if tek_granted {
        links.push("ukd:traditional-knowledge:ecology");
    }
    Ok(links)
}

pub fn offline_pack(mode: GaianMode) -> &'static str {
    match mode {
        GaianMode::Learn => "offline-learn-pack-stub",
        _ => "mode-pack-stub",
    }
}
