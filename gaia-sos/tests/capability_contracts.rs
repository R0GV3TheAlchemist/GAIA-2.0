use std::collections::BTreeSet;
use gaia_sos::{hal_tiers, revoke_mode, t0_kernel_kloc, AuthorizationError, Capability, CapabilityAuthority, CapabilityId, Constraints, Delegation, EntityId, EntityKind, HalTier, Operation};

fn ops(values: &[Operation]) -> BTreeSet<Operation> { values.iter().cloned().collect() }
fn entity(kind: EntityKind, id: &str) -> EntityId { EntityId::new(kind, id) }
fn root() -> Capability {
    Capability {
        id: CapabilityId::new("cap-root"), issuer: entity(EntityKind::Organization, "org:gaia"),
        subject: entity(EntityKind::Agent, "agent:climate"), resource: "earth-twin/amazon-drought".into(),
        operations: ops(&[Operation::Read, Operation::Advise]), constraints: Constraints::advise_only("climate-risk"),
        not_before: 10, expires_at: 100, parent: None, delegation: Delegation { remaining_depth: 1 }, issued_epoch: 0,
    }
}
#[test]
fn all_entity_kinds_and_hal_tiers_are_declared() {
    let kinds = [EntityKind::Human, EntityKind::Agent, EntityKind::Device, EntityKind::Organization, EntityKind::Service];
    assert_eq!(kinds.len(), 5);
    assert_eq!(hal_tiers(), [HalTier::T0, HalTier::T1, HalTier::T2, HalTier::T3, HalTier::T4]);
    assert_eq!(t0_kernel_kloc(), 0);
}
#[test]
fn capability_requires_matching_subject_resource_operation_and_time() {
    let mut authority = CapabilityAuthority::new(); let capability = root(); let id = capability.id.clone(); authority.issue(capability).unwrap();
    let subject = entity(EntityKind::Agent, "agent:climate");
    authority.authorize(&id, &subject, "earth-twin/amazon-drought", &Operation::Read, 20).unwrap();
    assert_eq!(authority.authorize(&id, &subject, "earth-twin/amazon-drought", &Operation::Actuate, 20), Err(AuthorizationError::Denied));
    assert_eq!(authority.authorize(&id, &subject, "earth-twin/amazon-drought", &Operation::Read, 100), Err(AuthorizationError::Denied));
}
#[test]
fn delegation_attenuates_and_parent_revoke_invalidates_child() {
    let mut authority = CapabilityAuthority::new(); let parent = root(); let parent_id = parent.id.clone(); authority.issue(parent).unwrap();
    let child = Capability {
        id: CapabilityId::new("cap-child"), issuer: entity(EntityKind::Agent, "agent:climate"), subject: entity(EntityKind::Service, "service:report"),
        resource: "earth-twin/amazon-drought".into(), operations: ops(&[Operation::Read]), constraints: Constraints::advise_only("climate-risk"),
        not_before: 20, expires_at: 90, parent: Some(parent_id.clone()), delegation: Delegation { remaining_depth: 0 }, issued_epoch: 0,
    };
    let child_id = child.id.clone(); authority.issue(child).unwrap(); let subject = entity(EntityKind::Service, "service:report");
    authority.authorize(&child_id, &subject, "earth-twin/amazon-drought", &Operation::Read, 30).unwrap();
    assert_eq!(authority.revoke(&parent_id), Ok(1));
    assert_eq!(authority.authorize(&child_id, &subject, "earth-twin/amazon-drought", &Operation::Read, 30), Err(AuthorizationError::Revoked));
}
#[test]
fn revoke_mode_names_conformance_boundary() { assert_eq!(revoke_mode(), "strongly-consistent-single-authority"); }
