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
            chain: vec!["Arctic ice".into(), "jet stream".into(), "drought".into()],
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

    pub fn nine() -> [Self; 9] {
        [
            row("climate-change", "CO2", "ppm", "monthly"),
            row("biosphere-integrity", "extinction-rate", "E/MSY", "annual"),
            row("land-system-change", "forest-fraction", "1", "annual"),
            row("freshwater-change", "runoff", "mm", "monthly"),
            row("biogeochemical-flows", "N-P", "Tg", "annual"),
            row("ocean-acidification", "pH", "1", "monthly"),
            row("atmospheric-aerosols", "AOD", "1", "daily"),
            row("novel-entities", "index", "1", "annual"),
            row("stratospheric-ozone", "DU", "DU", "monthly"),
        ]
    }
}

fn row(name: &str, owner: &str, unit: &str, cadence: &str) -> BoundaryIndicator {
    BoundaryIndicator {
        name: name.into(),
        owner: owner.into(),
        unit: unit.into(),
        cadence: cadence.into(),
    }
}
