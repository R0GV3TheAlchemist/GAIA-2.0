//! #621 — Typed 12-domain knowledge taxonomy.
//!
//! Each `Domain` carries its identifier, human-readable name, a short
//! description, the benchmark tier range that applies to it, the key
//! benchmarks that measure it, and any `cannot-know` keys that belong
//! to the domain.

/// A single domain in the 12-domain AI Knowledge taxonomy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Domain {
    /// Short machine identifier, e.g. `"math"`.  Matches `card::DOMAINS`.
    pub id: &'static str,
    /// Display name, e.g. `"Mathematical Knowledge"`.
    pub name: &'static str,
    /// One-sentence description of what the domain covers.
    pub description: &'static str,
    /// Primary benchmarks that measure performance in this domain.
    pub benchmarks: &'static [&'static str],
    /// `cannot-know` catalog keys that belong to this domain.
    pub cannot_know_keys: &'static [&'static str],
}

/// The complete 12-domain taxonomy from GAIA 2.0 — AI KNOWLEDGE DATABASE.
pub static DOMAINS: [Domain; 12] = [
    Domain {
        id: "language",
        name: "Language & Linguistic Knowledge",
        description: "Grammar, syntax, semantics, pragmatics, translation, \
                      and multilingual transfer across 100+ languages.",
        benchmarks: &["WMT-24", "FLORES-200", "BIG-Bench-Hard", "HellaSwag"],
        cannot_know_keys: &["post-cutoff-without-tools"],
    },
    Domain {
        id: "math",
        name: "Mathematical Knowledge",
        description: "Arithmetic through olympiad-level competition mathematics, \
                      formal proof, and symbolic reasoning.",
        benchmarks: &["MATH-500", "AIME-2025", "IMO-2025", "GSM8K"],
        cannot_know_keys: &["undecidable"],
    },
    Domain {
        id: "science",
        name: "Scientific Knowledge",
        description: "Physics, chemistry, biology, earth science, climate, \
                      and graduate-level research reasoning.",
        benchmarks: &["GPQA-Diamond", "MMLU-STEM", "SciQ"],
        cannot_know_keys: &["post-cutoff-without-tools", "genuine-novelty"],
    },
    Domain {
        id: "code",
        name: "Coding & Software Engineering Knowledge",
        description: "Algorithm design, debugging, architecture, testing, \
                      documentation, and real-world repo-level tasks.",
        benchmarks: &["HumanEval", "SWE-bench-Verified", "SWE-bench-Pro", "LiveCodeBench"],
        cannot_know_keys: &["post-cutoff-without-tools"],
    },
    Domain {
        id: "reasoning",
        name: "Reasoning & Cognitive Knowledge",
        description: "Deductive, inductive, abductive, causal, and counterfactual \
                      reasoning; world models and metacognition.",
        benchmarks: &["ARC-AGI-2", "BIG-Bench-Hard", "HLE", "LogiQA"],
        cannot_know_keys: &["chaotic-long-horizon", "undecidable"],
    },
    Domain {
        id: "world",
        name: "World Knowledge & Factual Knowledge",
        description: "General factual knowledge spanning MMLU's 57 subjects, \
                      commonsense, and encyclopaedic coverage.",
        benchmarks: &["MMLU", "TruthfulQA", "NaturalQuestions", "TriviaQA"],
        cannot_know_keys: &["post-cutoff-without-tools", "genuine-novelty"],
    },
    Domain {
        id: "vision",
        name: "Multimodal Knowledge",
        description: "Vision, video, audio, 3D, and cross-modal understanding \
                      including document parsing and chart interpretation.",
        benchmarks: &["MMMU", "DocVQA", "VideoMME", "ChartQA"],
        cannot_know_keys: &["tacit-embodied"],
    },
    Domain {
        id: "agency",
        name: "Agentic & Tool-Use Knowledge",
        description: "Multi-step task execution, tool selection, web navigation, \
                      computer use, and multi-agent coordination.",
        benchmarks: &["BrowseComp", "OSWorld", "tau2-bench", "WebArena"],
        cannot_know_keys: &["chaotic-long-horizon", "post-cutoff-without-tools"],
    },
    Domain {
        id: "professional",
        name: "Professional & Domain-Specific Knowledge",
        description: "Legal, medical, and financial knowledge at licensing-exam \
                      level; reference only, not a practice licence.",
        benchmarks: &["USMLE", "MedQA", "LegalBench", "FinanceBench"],
        cannot_know_keys: &["post-cutoff-without-tools"],
    },
    Domain {
        id: "creative",
        name: "Creative & Generative Knowledge",
        description: "Long-form writing, poetry, code synthesis from intent, \
                      image and audio generation.",
        benchmarks: &["ELO-arena", "WritingBench"],
        cannot_know_keys: &["genuine-novelty"],
    },
    Domain {
        id: "meta-ai",
        name: "Meta-AI Knowledge",
        description: "ML theory, model architectures, safety, alignment, \
                      ethics, and interpretability.",
        benchmarks: &["MMLU-CS", "SafetyBench"],
        cannot_know_keys: &["post-cutoff-without-tools", "genuine-novelty"],
    },
    Domain {
        id: "meta-knowledge",
        name: "Knowledge About Knowledge",
        description: "Epistemics, calibration, uncertainty quantification, \
                      self-knowledge, fact-checking, and knowledge-gap detection.",
        benchmarks: &["TruthfulQA", "CalibrationBench"],
        cannot_know_keys: &["undecidable", "genuine-novelty"],
    },
];

/// Return the domain with the given `id`, or `None`.
pub fn domain_by_id(id: &str) -> Option<&'static Domain> {
    DOMAINS.iter().find(|d| d.id == id)
}

/// Return all 12 domains.
pub fn all_domains() -> &'static [Domain; 12] {
    &DOMAINS
}
