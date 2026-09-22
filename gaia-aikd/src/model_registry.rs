//! #621 — Open-weight model registry.
//!
//! Lists every open-weight model in the GAIA 2.0 stack with its licence,
//! domain coverage, and an honest `gaia_measured: false` flag for all
//! entries — published benchmark scores are reference only.

/// A single entry in the open-weight model registry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelEntry {
    /// Short machine identifier, e.g. `"llama-3.3-70b"`.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// SPDX licence expression.
    pub license: &'static str,
    /// Domain ids this model primarily covers.
    pub domains: &'static [&'static str],
    /// `true` for all entries — this registry is open-weight only.
    pub open_weight: bool,
    /// Always `false`: GAIA has not independently measured these models.
    pub gaia_measured: bool,
    /// Brief note on the model's primary strength.
    pub notes: &'static str,
}

/// Static open-weight model registry for GAIA 2.0.
///
/// Sources: model cards, Hugging Face, and the benchmark literature cited
/// in issue #621.  `gaia_measured` is `false` for every entry — we report
/// published scores as reference only.
pub static MODEL_REGISTRY: [ModelEntry; 10] = [
    ModelEntry {
        id: "llama-3.3-70b",
        name: "Llama 3.3 70B Instruct",
        license: "Meta Llama 3 Community License",
        domains: &["language", "world", "reasoning", "code"],
        open_weight: true,
        gaia_measured: false,
        notes: "Meta's flagship open-weight general-purpose model.",
    },
    ModelEntry {
        id: "mistral-7b",
        name: "Mistral 7B Instruct v0.3",
        license: "Apache-2.0",
        domains: &["language", "world", "reasoning"],
        open_weight: true,
        gaia_measured: false,
        notes: "Efficient 7B model; strong instruction-following.",
    },
    ModelEntry {
        id: "deepseek-r1",
        name: "DeepSeek R1",
        license: "MIT",
        domains: &["math", "reasoning", "code", "science"],
        open_weight: true,
        gaia_measured: false,
        notes: "Chain-of-thought reasoning specialist; IMO-grade math.",
    },
    ModelEntry {
        id: "deepseek-coder-v2",
        name: "DeepSeek Coder V2",
        license: "DeepSeek License",
        domains: &["code"],
        open_weight: true,
        gaia_measured: false,
        notes: "State-of-the-art open code model; SWE-bench leader.",
    },
    ModelEntry {
        id: "qwen2.5-math",
        name: "Qwen2.5-Math 72B",
        license: "Apache-2.0",
        domains: &["math"],
        open_weight: true,
        gaia_measured: false,
        notes: "Specialised open math model; MATH-500 / AIME leader.",
    },
    ModelEntry {
        id: "llava-1.6",
        name: "LLaVA 1.6 (34B)",
        license: "Apache-2.0",
        domains: &["vision", "language"],
        open_weight: true,
        gaia_measured: false,
        notes: "Open vision-language model for image understanding.",
    },
    ModelEntry {
        id: "moondream-2",
        name: "Moondream 2",
        license: "Apache-2.0",
        domains: &["vision"],
        open_weight: true,
        gaia_measured: false,
        notes: "Tiny on-device vision model; runs on CPU.",
    },
    ModelEntry {
        id: "graphcast",
        name: "GraphCast",
        license: "Apache-2.0",
        domains: &["science", "world"],
        open_weight: true,
        gaia_measured: false,
        notes: "Google DeepMind 10-day weather forecasting model.",
    },
    ModelEntry {
        id: "esfm",
        name: "Earth System Foundation Model (ESFM)",
        license: "Apache-2.0",
        domains: &["science", "world"],
        open_weight: true,
        gaia_measured: false,
        notes: "Multi-variable Earth system simulation and forecasting.",
    },
    ModelEntry {
        id: "aurora-1.5",
        name: "Aurora 1.5",
        license: "MIT",
        domains: &["science", "world"],
        open_weight: true,
        gaia_measured: false,
        notes: "Microsoft atmospheric foundation model.",
    },
];

/// Return all open-weight models.
pub fn open_models() -> &'static [ModelEntry; 10] {
    &MODEL_REGISTRY
}

/// Return all models that cover the given domain id.
pub fn models_for_domain(domain_id: &str) -> Vec<&'static ModelEntry> {
    MODEL_REGISTRY
        .iter()
        .filter(|m| m.domains.contains(&domain_id))
        .collect()
}

/// Return the model with the given `id`, or `None`.
pub fn model_by_id(id: &str) -> Option<&'static ModelEntry> {
    MODEL_REGISTRY.iter().find(|m| m.id == id)
}
