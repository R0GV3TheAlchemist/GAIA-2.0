//! #151 active open nodes. Exams are reference only. Not a license.

#[derive(Debug, Clone)]
pub struct ActiveNode {
    pub id: String,
    pub cites: Vec<String>,
    pub score_kind: String,
}

pub fn weather_node() -> ActiveNode {
    ActiveNode {
        id: "aispd:weather:graphcast-class".into(),
        cites: vec!["#48".into(), "#104".into()],
        score_kind: "gaia_measured_pending".into(),
    }
}

pub fn license_exam(name: &str) -> ActiveNode {
    ActiveNode {
        id: format!("aispd:exam:{name}"),
        cites: vec![],
        score_kind: "reference_published".into(),
    }
}
