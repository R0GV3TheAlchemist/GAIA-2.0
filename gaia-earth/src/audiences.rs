//! #53 scientist API, policy UI, local agent. No surveillance endpoints.

use crate::library::ScenarioLibrary;
use crate::{allow_purpose, TwinError};

#[derive(Debug, Clone, PartialEq)]
pub struct AgentAnswer {
    pub text: String,
    pub source: String,
    pub uncertainty: f64,
}

pub struct ScientistApi;

impl ScientistApi {
    pub fn scenario(id: &str) -> Result<String, TwinError> {
        ScenarioLibrary::spec(id).map(|s| s.id)
    }
}

pub struct PolicyUi;

impl PolicyUi {
    pub fn scenario(id: &str) -> Result<String, TwinError> {
        ScientistApi::scenario(id)
    }
}

pub struct LocalAgent;

impl LocalAgent {
    pub fn ask(place: &str, question: &str) -> Result<AgentAnswer, TwinError> {
        allow_purpose(question)?;
        if place.trim().is_empty() {
            return Err(TwinError::UnlabeledPoint);
        }
        Ok(AgentAnswer {
            text: format!("fixture answer for {place}"),
            source: "fixture-station".into(),
            uncertainty: 1.0,
        })
    }
}

pub fn surveillance_endpoints() -> [&'static str; 0] {
    []
}
