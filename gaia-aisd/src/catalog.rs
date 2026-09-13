//! #127 tools vs model cards. Published scores are reference only.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolCard {
    pub name: String,
    pub kind: String,
    pub skill: String,
    pub is_llm_card: bool,
}

pub fn graphcast() -> ToolCard {
    ToolCard {
        name: "GraphCast".into(),
        kind: "weather-forecast-tool".into(),
        skill: "aisd:science:weather-forecast".into(),
        is_llm_card: false,
    }
}

pub fn reference_published(name: &str, value: &str) -> String {
    format!("{name}=reference_published:{value};gaia_measured=")
}
