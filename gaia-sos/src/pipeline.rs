//! Seed-bundle admission path: Registry -> Policy -> Guardrails -> Execute.
//! Default deny. No live connectors.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decision {
    pub allowed: bool,
    pub reason: &'static str,
}

pub fn evaluate(
    registered: bool,
    consent: bool,
    purpose_allowed: bool,
    feature_whitelisted: bool,
    payload_ok: bool,
) -> Decision {
    if !registered {
        return Decision { allowed: false, reason: "not-in-registry" };
    }
    if !consent {
        return Decision { allowed: false, reason: "consent-missing" };
    }
    if !purpose_allowed {
        return Decision { allowed: false, reason: "purpose-denied" };
    }
    if !feature_whitelisted {
        return Decision { allowed: false, reason: "feature-not-whitelisted" };
    }
    if !payload_ok {
        return Decision { allowed: false, reason: "guardrail-reject" };
    }
    Decision { allowed: true, reason: "execute-local" }
}

pub fn payload_ok(key: &str, value: &str, max_len: usize) -> bool {
    !key.is_empty()
        && key.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
        && value.len() <= max_len
}
