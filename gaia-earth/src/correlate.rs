//! #49 cross-system hook. One named case study. Correlation is not policy.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundaryIndicator {
    pub name: String,
    pub owner: String,
    pub unit: String,
    pub cadence: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseStudy {
    pub id: String,
    pub chain: Vec<String>,
}

impl CaseStudy {
    pub fn ice_jet_drought() -> Self {
        Self {
            id: "arctic-ice-jet-drought".into(),
            chain: vec![
                "Arctic ice".into(),
                "jet stream".into(),
                "drought".into(),
            ],
        }
    }

    pub fn is_policy(&self) -> bool {
        false
    }
}

impl BoundaryIndicator {
    pub fn climate_co2() -> Self {
        Self {
            name: "atmospheric CO2".into(),
            owner: "fixture-steward".into(),
            unit: "ppm".into(),
            cadence: "monthly".into(),
        }
    }
}
