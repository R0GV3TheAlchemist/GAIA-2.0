//! #103 execution verify stub. Not a real sandbox runner.

use crate::Tier;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Executed {
    pub tier: Tier,
    pub error: Option<String>,
}

impl Executed {
    pub fn code(tests_pass: bool, error: Option<&str>) -> Self {
        if tests_pass {
            Self {
                tier: Tier::T1,
                error: None,
            }
        } else {
            Self {
                tier: Tier::T4,
                error: Some(error.unwrap_or("execution failed").into()),
            }
        }
    }

    pub fn proof(verified: bool) -> Self {
        if verified {
            Self {
                tier: Tier::T1,
                error: None,
            }
        } else {
            Self {
                tier: Tier::T4,
                error: Some("unverified proof".into()),
            }
        }
    }
}
