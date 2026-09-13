//! #79 learning path fixture. Not NetworkX.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub id: String,
    pub resource: String,
    pub difficulty: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LearningPath {
    pub goal: String,
    pub steps: Vec<Step>,
}

pub fn path_to_quantum_computing() -> LearningPath {
    LearningPath {
        goal: "quantum computing".into(),
        steps: vec![
            Step {
                id: "ukd:math:linear-algebra".into(),
                resource: "https://ocw.mit.edu/".into(),
                difficulty: 2,
            },
            Step {
                id: "ukd:math:probability".into(),
                resource: "https://www.khanacademy.org/".into(),
                difficulty: 3,
            },
            Step {
                id: "ukd:eng:quantum-computing".into(),
                resource: "https://www.gutenberg.org/".into(),
                difficulty: 5,
            },
        ],
    }
}
