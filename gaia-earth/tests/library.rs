use gaia_earth::{ScenarioLibrary, SourceKind};

#[test]
fn three_canned_scenarios_emit_distributions() {
    for id in ScenarioLibrary::canned() {
        let out = ScenarioLibrary::run(id).unwrap();
        assert_eq!(out.len(), 5);
        assert!(out.iter().all(|d| d.uncertainty > 0.0));
        assert!(out.iter().all(|d| d.source == SourceKind::Synthetic));
        let spec = ScenarioLibrary::spec(id).unwrap();
        assert!(!spec.model.is_empty());
        assert!(!spec.method.is_empty());
    }
}
