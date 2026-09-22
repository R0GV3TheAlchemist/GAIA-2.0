//! Tests for the open-weight model registry (#621).

use gaia_aikd::{model_by_id, models_for_domain, open_models};

#[test]
fn ten_open_models_registered() {
    assert_eq!(open_models().len(), 10);
}

#[test]
fn all_models_are_open_weight_and_not_gaia_measured() {
    for m in open_models() {
        assert!(m.open_weight, "model {} is not open_weight", m.id);
        assert!(
            !m.gaia_measured,
            "model {} claims gaia_measured=true — published scores are reference only",
            m.id
        );
    }
}

#[test]
fn model_lookup_by_id() {
    let llama = model_by_id("llama-3.3-70b").expect("llama must be registered");
    assert_eq!(llama.license, "Meta Llama 3 Community License");
    assert!(llama.domains.contains(&"language"));

    let deepseek = model_by_id("deepseek-r1").expect("deepseek-r1 must be registered");
    assert!(deepseek.domains.contains(&"math"));
    assert!(deepseek.domains.contains(&"reasoning"));

    assert!(model_by_id("gpt-4o").is_none(), "closed models must not be in the registry");
}

#[test]
fn domain_filter_returns_correct_models() {
    let math_models = models_for_domain("math");
    let ids: Vec<&str> = math_models.iter().map(|m| m.id).collect();
    assert!(ids.contains(&"deepseek-r1"));
    assert!(ids.contains(&"qwen2.5-math"));

    let earth_models = models_for_domain("world");
    let earth_ids: Vec<&str> = earth_models.iter().map(|m| m.id).collect();
    assert!(earth_ids.contains(&"graphcast"));
    assert!(earth_ids.contains(&"esfm"));
    assert!(earth_ids.contains(&"aurora-1.5"));
}

#[test]
fn no_closed_model_in_registry() {
    for m in open_models() {
        let id_lower = m.id.to_ascii_lowercase();
        assert!(!id_lower.contains("gpt"), "closed model gpt in registry");
        assert!(!id_lower.contains("claude"), "closed model claude in registry");
        assert!(!id_lower.contains("gemini"), "closed model gemini in registry");
    }
}
