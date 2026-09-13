//! #199/#200 threat model + v1.0 gate. Formal verify is a plan.

pub fn threats() -> [&'static str; 5] {
    [
        "prompt-injection",
        "capability-escalation",
        "memory-poison",
        "mitm",
        "supply-chain",
    ]
}

pub fn formal_verify_done() -> bool {
    false
}

pub fn sos_v1_tagged() -> bool {
    false
}
