//! Tests for the 40-benchmark performance database (#621).

use gaia_aikd::{all_benchmarks, bench_by_id, benches_for_domain, superhuman_benches, Tier};

#[test]
fn forty_benchmarks_registered() {
    assert_eq!(all_benchmarks().len(), 40);
}

#[test]
fn all_benchmarks_have_non_empty_fields() {
    for b in all_benchmarks() {
        assert!(!b.id.is_empty(), "benchmark id is empty");
        assert!(!b.name.is_empty(), "benchmark name empty for {}", b.id);
        assert!(!b.domain.is_empty(), "domain empty for {}", b.id);
        assert!(!b.ai_model.is_empty(), "ai_model empty for {}", b.id);
        assert!(!b.evidence_url.is_empty(), "evidence_url empty for {}", b.id);
    }
}

#[test]
fn key_benchmarks_present_with_correct_tiers() {
    let mmlu = bench_by_id("mmlu").expect("mmlu must be present");
    assert_eq!(mmlu.tier, Tier::T2);
    assert!(mmlu.ai_score_pct > mmlu.human_baseline_pct.unwrap());

    let math = bench_by_id("math-500").expect("math-500 must be present");
    assert_eq!(math.tier, Tier::T1);
    assert!(math.superhuman());

    let humaneval = bench_by_id("humaneval").expect("humaneval must be present");
    assert_eq!(humaneval.tier, Tier::T1);

    let arc = bench_by_id("arc-agi-2").expect("arc-agi-2 must be present");
    assert_eq!(arc.tier, Tier::T3);
    assert!(!arc.superhuman());
}

#[test]
fn superhuman_benches_are_correct_subset() {
    let sh = superhuman_benches();
    // Must include confirmed superhuman domains
    let ids: Vec<&str> = sh.iter().map(|b| b.id).collect();
    assert!(ids.contains(&"math-500"));
    assert!(ids.contains(&"mmlu"));
    assert!(ids.contains(&"humaneval"));
    assert!(ids.contains(&"gpqa-diamond"));
    assert!(ids.contains(&"imo-2025"));
    // Must NOT include benchmarks where AI trails humans
    assert!(!ids.contains(&"arc-agi-2"));
    assert!(!ids.contains(&"hle"));
    assert!(!ids.contains(&"osworld"));
}

#[test]
fn domain_filter_works() {
    let math_benches = benches_for_domain("math");
    assert!(math_benches.len() >= 3);
    let ids: Vec<&str> = math_benches.iter().map(|b| b.id).collect();
    assert!(ids.contains(&"math-500"));
    assert!(ids.contains(&"gsm8k"));
    assert!(ids.contains(&"imo-2025"));

    let code_benches = benches_for_domain("code");
    let code_ids: Vec<&str> = code_benches.iter().map(|b| b.id).collect();
    assert!(code_ids.contains(&"humaneval"));
    assert!(code_ids.contains(&"swe-bench-verified"));
}

#[test]
fn no_scores_above_100() {
    for b in all_benchmarks() {
        assert!(
            b.ai_score_pct <= 100.0,
            "benchmark {} has ai_score_pct > 100: {}",
            b.id, b.ai_score_pct
        );
        if let Some(h) = b.human_baseline_pct {
            assert!(
                h <= 100.0,
                "benchmark {} has human_baseline_pct > 100: {}",
                b.id, h
            );
        }
    }
}
