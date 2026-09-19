use crate::pack::{self, AgentKind, PackEntry};
use crate::{AgentRuntime, RuntimeError};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub critic: String,
    pub subject_agent: String,
    pub subject_output: String,
    pub verdict: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HostError {
    UnknownAgent(String),
    NotDeployed(String),
    AlreadyDeployed(String),
    Policy(RuntimeError),
    Usage(String),
}

impl std::fmt::Display for HostError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownAgent(name) => write!(f, "unknown agent: {name}"),
            Self::NotDeployed(name) => write!(f, "agent not deployed: {name}"),
            Self::AlreadyDeployed(name) => write!(f, "agent already deployed: {name}"),
            Self::Policy(error) => write!(f, "policy rejected: {error:?}"),
            Self::Usage(msg) => write!(f, "{msg}"),
        }
    }
}

/// Local pack host. Deploy admits policy and records the agent.
/// This is not a marketplace, container runtime, or A2A transport.
pub struct AgentHost {
    runtime: AgentRuntime,
    deployed: Vec<&'static PackEntry>,
}

impl Default for AgentHost {
    fn default() -> Self {
        Self::new()
    }
}

impl AgentHost {
    pub fn new() -> Self {
        Self {
            runtime: AgentRuntime::default(),
            deployed: Vec::new(),
        }
    }

    pub fn catalog(&self) -> &'static [PackEntry] {
        pack::catalog()
    }

    pub fn deployed(&self) -> &[&'static PackEntry] {
        &self.deployed
    }

    pub fn deploy(&mut self, name: &str) -> Result<&'static PackEntry, HostError> {
        let entry = pack::find(name).ok_or_else(|| HostError::UnknownAgent(name.into()))?;
        if self.deployed.iter().any(|e| e.name == entry.name) {
            return Err(HostError::AlreadyDeployed(name.into()));
        }
        self.runtime
            .admit(&entry.policy_manifest())
            .map_err(HostError::Policy)?;
        self.deployed.push(entry);
        Ok(entry)
    }

    pub fn invoke(&self, name: &str, input: &str) -> Result<String, HostError> {
        let entry = self.require(name)?;
        if entry.kind == AgentKind::Bridge {
            return Err(HostError::Usage(
                "mcp-bridge: external API is not wired".into(),
            ));
        }
        Ok(format!(
            "{} handled {} input={}",
            entry.name,
            entry.intent,
            input.trim()
        ))
    }

    /// Host-local critic review. Not the #26 A2A protocol.
    pub fn review(&self, critic: &str, subject: &str, output: &str) -> Result<Review, HostError> {
        let critic_entry = self.require(critic)?;
        if critic_entry.name != "critic" || !critic_entry.handles("review.output") {
            return Err(HostError::Usage(format!(
                "{critic} cannot review; deploy critic"
            )));
        }
        let subject_entry = self.require(subject)?;
        Ok(Review {
            critic: critic_entry.name.into(),
            subject_agent: subject_entry.name.into(),
            subject_output: output.into(),
            verdict: format!("reviewed {} output", subject_entry.name),
        })
    }

    /// `gaia agent deploy <name>` shape. Bin is `gaia-agent` to avoid colliding with L4 `gaia`.
    pub fn exec(&mut self, args: &[&str]) -> Result<String, HostError> {
        let args = match args.first().copied() {
            Some("agent") => &args[1..],
            _ => args,
        };
        match args.first().copied() {
            Some("deploy") => {
                let name = args
                    .get(1)
                    .copied()
                    .ok_or_else(|| HostError::Usage("usage: gaia agent deploy <name>".into()))?;
                let entry = self.deploy(name)?;
                Ok(format!(
                    "deployed {} agent {} intent={}",
                    entry.kind.as_str(),
                    entry.name,
                    entry.intent
                ))
            }
            Some("list") => {
                let names: Vec<&str> = self.deployed.iter().map(|e| e.name).collect();
                Ok(format!("deployed={}", names.join(",")))
            }
            Some("invoke") => {
                let name = args.get(1).copied().ok_or_else(|| {
                    HostError::Usage("usage: gaia agent invoke <name> <input>".into())
                })?;
                let input = args.get(2..).unwrap_or(&[]).join(" ");
                self.invoke(name, &input)
            }
            Some("review") => {
                let subject = args.get(1).copied().ok_or_else(|| {
                    HostError::Usage("usage: gaia agent review <subject> <output>".into())
                })?;
                let output = args.get(2..).unwrap_or(&[]).join(" ");
                let review = self.review("critic", subject, &output)?;
                Ok(format!(
                    "critic={} subject={} verdict={}",
                    review.critic, review.subject_agent, review.verdict
                ))
            }
            _ => Err(HostError::Usage(
                "usage: gaia agent <deploy|list|invoke|review>".into(),
            )),
        }
    }

    fn require(&self, name: &str) -> Result<&'static PackEntry, HostError> {
        pack::find(name).ok_or_else(|| HostError::UnknownAgent(name.into()))?;
        self.deployed
            .iter()
            .copied()
            .find(|e| e.name == name)
            .ok_or_else(|| HostError::NotDeployed(name.into()))
    }
}
