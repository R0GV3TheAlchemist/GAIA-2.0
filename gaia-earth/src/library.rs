//! #51 scenario library. Canned runs. Not SSP physics or a cascade GCM.

use crate::{SourceKind, TwinError};

#[derive(Debug, Clone, PartialEq)]
pub struct ScenarioSpec {
    pub id: String,
    pub horizon: String,
    pub model: String,
    pub method: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Dist {
    pub metric: String,
    pub mean: f64,
    pub uncertainty: f64,
    pub source: SourceKind,
    pub model: String,
    pub uncertainty_method: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CascadeEdge {
    pub from: String,
    pub to: String,
}

pub struct ScenarioLibrary;

impl ScenarioLibrary {
    pub fn canned() -> [&'static str; 3] {
        ["net-zero by 2040", "AMOC -30%", "Amazon dieback"]
    }

    pub fn spec(id: &str) -> Result<ScenarioSpec, TwinError> {
        if !Self::canned().contains(&id) {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(ScenarioSpec {
            id: id.into(),
            horizon: "fixture-century".into(),
            model: "fixture-ensemble-0".into(),
            method: "fixture-distribution".into(),
        })
    }

    pub fn run(id: &str) -> Result<Vec<Dist>, TwinError> {
        let spec = Self::spec(id)?;
        Ok(["climate", "biodiversity", "economy", "welfare", "tipping-risk"]
            .into_iter()
            .map(|metric| Dist {
                metric: metric.into(),
                mean: 0.0,
                uncertainty: 1.0,
                source: SourceKind::Synthetic,
                model: spec.model.clone(),
                uncertainty_method: spec.method.clone(),
            })
            .collect())
    }

    pub fn cascade() -> [CascadeEdge; 2] {
        [
            CascadeEdge {
                from: "AMOC".into(),
                to: "Amazon".into(),
            },
            CascadeEdge {
                from: "Amazon".into(),
                to: "rainfall".into(),
            },
        ]
    }
}
