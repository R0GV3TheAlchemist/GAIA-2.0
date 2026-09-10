//! L4 Cognitive orchestration (Phase 2 / #4).
//! Part 1 (#20): Intent Engine stub. No cloud. No Ollama process.

mod intent;

pub use intent::{
    Compute, Constraints, IntentBackend, IntentEngine, IntentGraph, Privacy, SubIntent,
};
