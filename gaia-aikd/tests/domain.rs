//! Tests for the typed 12-domain taxonomy (#621).

use gaia_aikd::{all_domains, domain_by_id};

#[test]
fn twelve_domains_are_present() {
    let domains = all_domains();
    assert_eq!(domains.len(), 12);
}

#[test]
fn all_domains_have_non_empty_fields() {
    for d in all_domains() {
        assert!(!d.id.is_empty(), "domain id is empty");
        assert!(!d.name.is_empty(), "domain name empty for {}", d.id);
        assert!(!d.description.is_empty(), "description empty for {}", d.id);
        assert!(!d.benchmarks.is_empty(), "no benchmarks for {}", d.id);
    }
}

#[test]
fn domain_lookup_by_id() {
    let math = domain_by_id("math").expect("math domain must exist");
    assert_eq!(math.name, "Mathematical Knowledge");
    assert!(math.benchmarks.contains(&"MATH-500"));
    assert!(math.cannot_know_keys.contains(&"undecidable"));

    let code = domain_by_id("code").expect("code domain must exist");
    assert!(code.benchmarks.contains(&"HumanEval"));
    assert!(code.benchmarks.contains(&"SWE-bench-Verified"));

    assert!(domain_by_id("nonexistent").is_none());
}

#[test]
fn all_twelve_domain_ids_present() {
    let ids: Vec<&str> = all_domains().iter().map(|d| d.id).collect();
    for expected in [
        "language", "math", "science", "code", "reasoning", "world",
        "vision", "agency", "professional", "creative", "meta-ai", "meta-knowledge",
    ] {
        assert!(ids.contains(&expected), "missing domain id: {expected}");
    }
}
