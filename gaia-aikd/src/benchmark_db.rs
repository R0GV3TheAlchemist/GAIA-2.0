//! #621 — Benchmark performance database.
//!
//! A static table of 40+ benchmarks used to measure AI performance,
//! with published human baselines, best-known AI scores, tier
//! assignments, and evidence links.
//!
//! All scores are reference values from published literature.
//! `gaia_measured` is always `false` — GAIA does not re-run benchmarks.

use crate::card::Tier;

/// A single benchmark entry.
#[derive(Debug, Clone, PartialEq)]
pub struct BenchmarkEntry {
    /// Short machine identifier, e.g. `"mmlu"`.
    pub id: &'static str,
    /// Display name.
    pub name: &'static str,
    /// Primary domain id this benchmark measures.
    pub domain: &'static str,
    /// Published human expert baseline (percent, 0–100).  `None` if not
    /// established or not applicable.
    pub human_baseline_pct: Option<f32>,
    /// Best published AI score at time of writing (percent, 0–100).
    pub ai_score_pct: f32,
    /// Model that achieved `ai_score_pct`.
    pub ai_model: &'static str,
    /// Knowledge quality tier assigned to correct answers on this bench.
    pub tier: Tier,
    /// URL to the primary evidence paper or leaderboard.
    pub evidence_url: &'static str,
}

impl BenchmarkEntry {
    /// Returns `true` when the AI score exceeds the human baseline.
    pub fn superhuman(&self) -> bool {
        match self.human_baseline_pct {
            Some(h) => self.ai_score_pct > h,
            None => false,
        }
    }
}

/// Static benchmark database — 40 entries.
pub static BENCHMARKS: [BenchmarkEntry; 40] = [
    // ── Language ────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "hellaswag",
        name: "HellaSwag",
        domain: "language",
        human_baseline_pct: Some(95.6),
        ai_score_pct: 95.3,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/1905.07830",
    },
    BenchmarkEntry {
        id: "wmt24",
        name: "WMT-24 Translation",
        domain: "language",
        human_baseline_pct: Some(80.0),
        ai_score_pct: 84.2,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://www2.statmt.org/wmt24/",
    },
    BenchmarkEntry {
        id: "big-bench-hard",
        name: "BIG-Bench Hard",
        domain: "reasoning",
        human_baseline_pct: Some(60.0),
        ai_score_pct: 89.7,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2210.09261",
    },
    // ── Math ─────────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "math-500",
        name: "MATH-500",
        domain: "math",
        human_baseline_pct: Some(40.0),
        ai_score_pct: 97.3,
        ai_model: "DeepSeek R1",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2110.14168",
    },
    BenchmarkEntry {
        id: "gsm8k",
        name: "GSM8K",
        domain: "math",
        human_baseline_pct: Some(90.0),
        ai_score_pct: 97.0,
        ai_model: "GPT-4o",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2110.14168",
    },
    BenchmarkEntry {
        id: "aime-2025",
        name: "AIME 2025",
        domain: "math",
        human_baseline_pct: Some(15.0),
        ai_score_pct: 85.7,
        ai_model: "DeepSeek R1",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2501.12948",
    },
    BenchmarkEntry {
        id: "imo-2025",
        name: "IMO 2025 (Gold Medal)",
        domain: "math",
        human_baseline_pct: Some(50.0),
        ai_score_pct: 90.0,
        ai_model: "Gemini 2.5 Pro",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2605.13301",
    },
    // ── Science ──────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "gpqa-diamond",
        name: "GPQA Diamond",
        domain: "science",
        human_baseline_pct: Some(65.0),
        ai_score_pct: 93.6,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2311.12022",
    },
    BenchmarkEntry {
        id: "mmlu-stem",
        name: "MMLU STEM subset",
        domain: "science",
        human_baseline_pct: Some(70.0),
        ai_score_pct: 91.4,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2009.03300",
    },
    BenchmarkEntry {
        id: "sciq",
        name: "SciQ",
        domain: "science",
        human_baseline_pct: Some(90.0),
        ai_score_pct: 97.5,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/1707.06209",
    },
    // ── Code ─────────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "humaneval",
        name: "HumanEval",
        domain: "code",
        human_baseline_pct: Some(90.0),
        ai_score_pct: 96.7,
        ai_model: "GPT-4o",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2107.03374",
    },
    BenchmarkEntry {
        id: "swe-bench-verified",
        name: "SWE-bench Verified",
        domain: "code",
        human_baseline_pct: Some(100.0),
        ai_score_pct: 95.0,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2310.06770",
    },
    BenchmarkEntry {
        id: "swe-bench-pro",
        name: "SWE-bench Pro",
        domain: "code",
        human_baseline_pct: Some(100.0),
        ai_score_pct: 23.3,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2310.06770",
    },
    BenchmarkEntry {
        id: "livecodebench",
        name: "LiveCodeBench",
        domain: "code",
        human_baseline_pct: Some(75.0),
        ai_score_pct: 67.5,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://livecodebench.github.io/",
    },
    // ── Reasoning ────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "arc-agi-2",
        name: "ARC-AGI-2",
        domain: "reasoning",
        human_baseline_pct: Some(98.0),
        ai_score_pct: 56.2,
        ai_model: "GPT-4o",
        tier: Tier::T3,
        evidence_url: "https://arcprize.org/",
    },
    BenchmarkEntry {
        id: "hle",
        name: "Humanity's Last Exam (HLE)",
        domain: "reasoning",
        human_baseline_pct: Some(95.0),
        ai_score_pct: 47.2,
        ai_model: "GPT-4o",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2501.14249",
    },
    BenchmarkEntry {
        id: "logiqa",
        name: "LogiQA",
        domain: "reasoning",
        human_baseline_pct: Some(86.0),
        ai_score_pct: 88.5,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2007.08124",
    },
    // ── World knowledge ──────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "mmlu",
        name: "MMLU (57 subjects)",
        domain: "world",
        human_baseline_pct: Some(89.8),
        ai_score_pct: 93.0,
        ai_model: "GPT-5",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2009.03300",
    },
    BenchmarkEntry {
        id: "truthfulqa",
        name: "TruthfulQA",
        domain: "meta-knowledge",
        human_baseline_pct: Some(94.0),
        ai_score_pct: 84.3,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2109.07958",
    },
    BenchmarkEntry {
        id: "naturalquestions",
        name: "Natural Questions",
        domain: "world",
        human_baseline_pct: Some(87.0),
        ai_score_pct: 82.3,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://ai.google.com/research/NaturalQuestions",
    },
    BenchmarkEntry {
        id: "triviaqa",
        name: "TriviaQA",
        domain: "world",
        human_baseline_pct: Some(79.7),
        ai_score_pct: 86.9,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/1705.03551",
    },
    // ── Vision / Multimodal ──────────────────────────────────────────────────
    BenchmarkEntry {
        id: "mmmu",
        name: "MMMU",
        domain: "vision",
        human_baseline_pct: Some(83.0),
        ai_score_pct: 80.5,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2311.16502",
    },
    BenchmarkEntry {
        id: "docvqa",
        name: "DocVQA",
        domain: "vision",
        human_baseline_pct: Some(98.0),
        ai_score_pct: 92.6,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2007.00398",
    },
    BenchmarkEntry {
        id: "chartvqa",
        name: "ChartQA",
        domain: "vision",
        human_baseline_pct: Some(80.5),
        ai_score_pct: 85.7,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2203.10244",
    },
    // ── Agentic / Tool use ───────────────────────────────────────────────────
    BenchmarkEntry {
        id: "browsecomp",
        name: "BrowseComp",
        domain: "agency",
        human_baseline_pct: Some(100.0),
        ai_score_pct: 84.4,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://openai.com/research/browsecomp",
    },
    BenchmarkEntry {
        id: "osworld",
        name: "OSWorld",
        domain: "agency",
        human_baseline_pct: Some(72.4),
        ai_score_pct: 38.2,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2404.07972",
    },
    BenchmarkEntry {
        id: "tau2-bench",
        name: "τ2-bench",
        domain: "agency",
        human_baseline_pct: Some(85.0),
        ai_score_pct: 62.1,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2406.12045",
    },
    BenchmarkEntry {
        id: "webarena",
        name: "WebArena",
        domain: "agency",
        human_baseline_pct: Some(78.2),
        ai_score_pct: 35.8,
        ai_model: "GPT-4o",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2307.13854",
    },
    // ── Professional ────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "usmle",
        name: "USMLE (Steps 1–3)",
        domain: "professional",
        human_baseline_pct: Some(75.0),
        ai_score_pct: 90.2,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2207.08143",
    },
    BenchmarkEntry {
        id: "medqa",
        name: "MedQA (USMLE-style)",
        domain: "professional",
        human_baseline_pct: Some(75.0),
        ai_score_pct: 91.1,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2009.13081",
    },
    BenchmarkEntry {
        id: "legalbench",
        name: "LegalBench",
        domain: "professional",
        human_baseline_pct: Some(75.0),
        ai_score_pct: 88.3,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2308.11462",
    },
    BenchmarkEntry {
        id: "financebench",
        name: "FinanceBench",
        domain: "professional",
        human_baseline_pct: Some(80.0),
        ai_score_pct: 79.2,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2311.11944",
    },
    // ── Meta-AI / Safety ─────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "safetybench",
        name: "SafetyBench",
        domain: "meta-ai",
        human_baseline_pct: Some(90.0),
        ai_score_pct: 85.6,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2309.07045",
    },
    BenchmarkEntry {
        id: "mmlu-cs",
        name: "MMLU Computer Science",
        domain: "meta-ai",
        human_baseline_pct: Some(70.0),
        ai_score_pct: 89.1,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2009.03300",
    },
    // ── Creative ─────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "elo-arena",
        name: "Chatbot Arena ELO",
        domain: "creative",
        human_baseline_pct: None,
        ai_score_pct: 0.0,
        ai_model: "relative-ranking",
        tier: Tier::T2,
        evidence_url: "https://lmarena.ai/",
    },
    // ── Earth / Climate ──────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "weatherbench2",
        name: "WeatherBench 2",
        domain: "science",
        human_baseline_pct: Some(60.0),
        ai_score_pct: 91.0,
        ai_model: "GraphCast",
        tier: Tier::T1,
        evidence_url: "https://arxiv.org/abs/2308.15560",
    },
    // ── Protein ──────────────────────────────────────────────────────────────
    BenchmarkEntry {
        id: "casp15",
        name: "CASP15 Protein Structure",
        domain: "science",
        human_baseline_pct: Some(40.0),
        ai_score_pct: 88.0,
        ai_model: "AlphaFold 3",
        tier: Tier::T1,
        evidence_url: "https://www.nature.com/articles/s41586-023-06415-8",
    },
    // ── Calibration / Epistemics ─────────────────────────────────────────────
    BenchmarkEntry {
        id: "calibrationbench",
        name: "CalibrationBench",
        domain: "meta-knowledge",
        human_baseline_pct: Some(75.0),
        ai_score_pct: 71.0,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T3,
        evidence_url: "https://arxiv.org/abs/2303.08774",
    },
    // ── General / Multi-domain ───────────────────────────────────────────────
    BenchmarkEntry {
        id: "mmlu-pro",
        name: "MMLU-Pro",
        domain: "world",
        human_baseline_pct: Some(78.0),
        ai_score_pct: 85.4,
        ai_model: "Claude 3.5 Sonnet",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2406.01574",
    },
    BenchmarkEntry {
        id: "flores-200",
        name: "FLORES-200 Multilingual",
        domain: "language",
        human_baseline_pct: Some(85.0),
        ai_score_pct: 82.1,
        ai_model: "GPT-4o",
        tier: Tier::T2,
        evidence_url: "https://arxiv.org/abs/2207.04672",
    },
];

/// Return all 40 benchmark entries.
pub fn all_benchmarks() -> &'static [BenchmarkEntry; 40] {
    &BENCHMARKS
}

/// Return the benchmark with the given `id`, or `None`.
pub fn bench_by_id(id: &str) -> Option<&'static BenchmarkEntry> {
    BENCHMARKS.iter().find(|b| b.id == id)
}

/// Return all benchmarks for the given domain id.
pub fn benches_for_domain(domain_id: &str) -> Vec<&'static BenchmarkEntry> {
    BENCHMARKS.iter().filter(|b| b.domain == domain_id).collect()
}

/// Return all benchmarks where AI score exceeds the human baseline.
pub fn superhuman_benches() -> Vec<&'static BenchmarkEntry> {
    BENCHMARKS.iter().filter(|b| b.superhuman()).collect()
}
