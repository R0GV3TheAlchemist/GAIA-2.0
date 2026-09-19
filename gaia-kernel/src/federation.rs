//! #31 federated instances and digital sovereignty.
//! In-process peering only. No wire protocol between boxes.

use crate::audit::AuditLog;
use crate::identity::{Principal, PrincipalKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    pub did: String,
    pub region: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResidentCube {
    pub id: String,
    pub region: String,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JointTask {
    pub intent: String,
    pub from: String,
    pub to: String,
    pub redacted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FederationError {
    UnknownInstance(String),
    NotPeered,
    ResidencyDenied { cube: String, region: String },
}

impl std::fmt::Display for FederationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownInstance(id) => write!(f, "unknown instance: {id}"),
            Self::NotPeered => write!(f, "instances are not peered"),
            Self::ResidencyDenied { cube, region } => {
                write!(f, "cube {cube} may not leave {region}")
            }
        }
    }
}

pub struct Federation {
    operator: Principal,
    instances: Vec<(Principal, Instance)>,
    peers: Vec<(String, String)>,
    pub audit: AuditLog,
}

impl Default for Federation {
    fn default() -> Self {
        Self {
            operator: Principal::generate(PrincipalKind::Node),
            instances: Vec::new(),
            peers: Vec::new(),
            audit: AuditLog::default(),
        }
    }
}

impl Federation {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn spawn(&mut self, region: &str) -> Instance {
        let principal = Principal::generate(PrincipalKind::Node);
        let instance = Instance {
            did: principal.did(),
            region: region.into(),
        };
        self.audit
            .append(&principal, "instance.spawn", &format!("region={region}"));
        self.instances.push((principal, instance.clone()));
        instance
    }

    pub fn peer(&mut self, a: &Instance, b: &Instance) -> Result<(), FederationError> {
        self.require(a)?;
        self.require(b)?;
        self.peers.push((a.did.clone(), b.did.clone()));
        self.peers.push((b.did.clone(), a.did.clone()));
        self.audit.append(
            &self.operator,
            "instance.peer",
            &format!("{} <-> {}", a.did, b.did),
        );
        Ok(())
    }

    pub fn cube(id: &str, region: &str, content: &str) -> ResidentCube {
        ResidentCube {
            id: id.into(),
            region: region.into(),
            content: content.into(),
        }
    }

    /// Cross-instance intent. Cube plaintext stays home unless regions match.
    pub fn joint_task(
        &mut self,
        from: &Instance,
        to: &Instance,
        intent: &str,
        cube: &ResidentCube,
    ) -> Result<JointTask, FederationError> {
        self.require(from)?;
        self.require(to)?;
        if !self.is_peered(&from.did, &to.did) {
            return Err(FederationError::NotPeered);
        }
        if cube.region != to.region {
            self.audit.append(
                &self.operator,
                "egress.denied",
                &format!("cube={} from={} to={}", cube.id, cube.region, to.region),
            );
            return Err(FederationError::ResidencyDenied {
                cube: cube.id.clone(),
                region: cube.region.clone(),
            });
        }
        self.audit.append(
            &self.operator,
            "egress.allowed",
            &format!("cube={} region={} intent={intent}", cube.id, cube.region),
        );
        Ok(JointTask {
            intent: intent.into(),
            from: from.did.clone(),
            to: to.did.clone(),
            redacted: true,
        })
    }

    pub fn egress_denied(&self, cube_id: &str) -> bool {
        self.audit.prove("egress.denied", cube_id).is_some()
    }

    fn require(&self, instance: &Instance) -> Result<(), FederationError> {
        if self.instances.iter().any(|(_, i)| i.did == instance.did) {
            Ok(())
        } else {
            Err(FederationError::UnknownInstance(instance.did.clone()))
        }
    }

    fn is_peered(&self, a: &str, b: &str) -> bool {
        self.peers.iter().any(|(l, r)| l == a && r == b)
    }
}
