use gaia_hmgd::{parse_node, EvidenceClass, HmgdError, REALMS};

#[test]
fn prayer_and_i_ching_validate_recipe_rejected() {
    assert_eq!(REALMS.len(), 10);
    assert_eq!(parse_node("prayer rct").unwrap().evidence, EvidenceClass::Measured);
    assert_eq!(parse_node("i ching").unwrap().evidence, EvidenceClass::Traditional);
    assert_eq!(parse_node("recipe:x").unwrap_err(), HmgdError::RecipeForbidden);
    assert_eq!(parse_node("curse").unwrap_err(), HmgdError::CurseForbidden);
}
