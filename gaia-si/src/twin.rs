//! #179 twin stub. Occupancy anonymized. Uncertainty required.

#[derive(Debug, Clone)]
pub struct Twin {
    pub occupancy: String,
    pub uncertainty: bool,
    pub household_id: Option<String>,
}

impl Twin {
    pub fn building() -> Self {
        Self {
            occupancy: "anonymous-count".into(),
            uncertainty: true,
            household_id: None,
        }
    }
}
