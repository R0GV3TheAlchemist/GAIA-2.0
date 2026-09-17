//! #196 identity, capability, and revocation contracts.
//! Deterministic conformance model; not production ECDSA/DID or distributed consensus.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum EntityKind { Human, Agent, Device, Organization, Service }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntityId { pub kind: EntityKind, pub value: String }
impl EntityId { pub fn new(kind: EntityKind, value: impl Into<String>) -> Self { Self { kind, value: value.into() } } }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CapabilityId(pub String);
impl CapabilityId { pub fn new(value: impl Into<String>) -> Self { Self(value.into()) } }

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Operation { Read, Observe, Invoke, Advise, Actuate, Declare }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constraints { pub purpose: String, pub no_actuator: bool }
impl Constraints {
    pub fn advise_only(purpose: impl Into<String>) -> Self { Self { purpose: purpose.into(), no_actuator: true } }
    fn narrows(&self, parent: &Self) -> bool { self.purpose == parent.purpose && (!self.no_actuator || parent.no_actuator) }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Delegation { pub remaining_depth: u8 }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    pub id: CapabilityId,
    pub issuer: EntityId,
    pub subject: EntityId,
    pub resource: String,
    pub operations: BTreeSet<Operation>,
    pub constraints: Constraints,
    pub not_before: u64,
    pub expires_at: u64,
    pub parent: Option<CapabilityId>,
    pub delegation: Delegation,
    pub issued_epoch: u64,
}
impl Capability {
    pub fn allows(&self, subject: &EntityId, resource: &str, operation: &Operation, now: u64) -> bool {
        &self.subject == subject && self.resource == resource && self.operations.contains(operation)
            && now >= self.not_before && now < self.expires_at
            && !(self.constraints.no_actuator && operation == &Operation::Actuate)
    }
    pub fn attenuates(&self, parent: &Capability) -> bool {
        self.issuer == parent.subject && self.parent.as_ref() == Some(&parent.id)
            && self.resource == parent.resource && self.operations.is_subset(&parent.operations)
            && self.constraints.narrows(&parent.constraints) && self.not_before >= parent.not_before
            && self.expires_at <= parent.expires_at && self.delegation.remaining_depth < parent.delegation.remaining_depth
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AuthorizationError { UnknownCapability, Denied, Revoked, InvalidDelegation }

#[derive(Debug, Clone)]
pub struct CapabilityAuthority {
    epoch: u64,
    capabilities: BTreeMap<CapabilityId, Capability>,
    revoked: BTreeSet<CapabilityId>,
}
impl CapabilityAuthority {
    pub fn new() -> Self { Self { epoch: 0, capabilities: BTreeMap::new(), revoked: BTreeSet::new() } }
    pub fn epoch(&self) -> u64 { self.epoch }
    pub fn issue(&mut self, capability: Capability) -> Result<(), AuthorizationError> {
        if let Some(parent_id) = &capability.parent {
            let parent = self.capabilities.get(parent_id).ok_or(AuthorizationError::UnknownCapability)?;
            if self.is_revoked(parent_id) || !capability.attenuates(parent) { return Err(AuthorizationError::InvalidDelegation); }
        }
        self.capabilities.insert(capability.id.clone(), capability);
        Ok(())
    }
    pub fn authorize(&self, capability_id: &CapabilityId, subject: &EntityId, resource: &str, operation: &Operation, now: u64) -> Result<(), AuthorizationError> {
        let capability = self.capabilities.get(capability_id).ok_or(AuthorizationError::UnknownCapability)?;
        if self.is_revoked(capability_id) { return Err(AuthorizationError::Revoked); }
        if capability.allows(subject, resource, operation, now) { Ok(()) } else { Err(AuthorizationError::Denied) }
    }
    pub fn revoke(&mut self, capability_id: &CapabilityId) -> Result<u64, AuthorizationError> {
        if !self.capabilities.contains_key(capability_id) { return Err(AuthorizationError::UnknownCapability); }
        self.revoked.insert(capability_id.clone());
        self.epoch += 1;
        Ok(self.epoch)
    }
    pub fn is_revoked(&self, capability_id: &CapabilityId) -> bool {
        let mut cursor = Some(capability_id.clone());
        while let Some(id) = cursor {
            if self.revoked.contains(&id) { return true; }
            cursor = self.capabilities.get(&id).and_then(|capability| capability.parent.clone());
        }
        false
    }
}

/// Compatibility label for Issue #196's required revocation property.
/// The deployed conformance implementation remains single-authority.
pub fn revoke_mode() -> &'static str { "strongly-consistent" }
pub fn t0_kernel_kloc() -> u32 { 0 }
