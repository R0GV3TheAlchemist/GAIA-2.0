//! #179/#186 twins. No household key on Earth Twin ingest.

#[derive(Debug, Clone)]
pub struct Twin {
    pub occupancy: String,
    pub uncertainty: bool,
    pub household_id: Option<String>,
    pub interval: bool,
}

impl Twin {
    pub fn building() -> Self {
        Self {
            occupancy: "anonymous-count".into(),
            uncertainty: true,
            household_id: None,
            interval: true,
        }
    }
}

pub fn aggregate(t: &Twin) -> Option<String> {
    t.household_id.clone()
}
