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

// ── Tests (#25 acceptance criteria — deploy & critic A2A review) ──────────────

#[cfg(test)]
mod tests {
    use super::*;

    // ── AC1: `gaia agent deploy` works for at least one system and one cognitive ──

    #[test]
    fn deploy_system_agent_succeeds() {
        let mut host = AgentHost::new();
        let entry = host.deploy("memory-manager").expect("deploy memory-manager");
        assert_eq!(entry.name, "memory-manager");
        assert_eq!(entry.kind, AgentKind::System);
        assert!(host.deployed().iter().any(|e| e.name == "memory-manager"));
    }

    #[test]
    fn deploy_cognitive_agent_succeeds() {
        let mut host = AgentHost::new();
        let entry = host.deploy("researcher").expect("deploy researcher");
        assert_eq!(entry.name, "researcher");
        assert_eq!(entry.kind, AgentKind::Cognitive);
    }

    #[test]
    fn exec_deploy_system_agent_produces_correct_output() {
        let mut host = AgentHost::new();
        let out = host.exec(&["agent", "deploy", "memory-manager"]).unwrap();
        assert!(out.contains("memory-manager"), "output must name the agent");
        assert!(out.contains("system"), "output must name the kind");
        assert!(out.contains("memcube.consolidate"), "output must name the intent");
    }

    #[test]
    fn exec_deploy_cognitive_agent_produces_correct_output() {
        let mut host = AgentHost::new();
        let out = host.exec(&["agent", "deploy", "writer"]).unwrap();
        assert!(out.contains("writer"));
        assert!(out.contains("cognitive"));
        assert!(out.contains("write.draft"));
    }

    #[test]
    fn deploy_unknown_agent_returns_error() {
        let mut host = AgentHost::new();
        assert!(matches!(
            host.deploy("does-not-exist"),
            Err(HostError::UnknownAgent(_))
        ));
    }

    #[test]
    fn deploy_duplicate_returns_error() {
        let mut host = AgentHost::new();
        host.deploy("planner").unwrap();
        assert!(matches!(
            host.deploy("planner"),
            Err(HostError::AlreadyDeployed(_))
        ));
    }

    #[test]
    fn invoke_system_agent_returns_output() {
        let mut host = AgentHost::new();
        host.deploy("resource-optimizer").unwrap();
        let out = host.invoke("resource-optimizer", "test-input").unwrap();
        assert!(out.contains("resource-optimizer"));
        assert!(out.contains("resource.optimize"));
        assert!(out.contains("test-input"));
    }

    #[test]
    fn invoke_before_deploy_returns_not_deployed() {
        let host = AgentHost::new();
        assert!(matches!(
            host.invoke("planner", "anything"),
            Err(HostError::NotDeployed(_))
        ));
    }

    // ── AC2: Critic agent can review another agent's output via A2A ──────────

    #[test]
    fn critic_reviews_writer_output() {
        let mut host = AgentHost::new();
        host.deploy("critic").unwrap();
        host.deploy("writer").unwrap();
        let review = host.review("critic", "writer", "draft output text").unwrap();
        assert_eq!(review.critic, "critic");
        assert_eq!(review.subject_agent, "writer");
        assert_eq!(review.subject_output, "draft output text");
        assert!(review.verdict.contains("writer"),
            "verdict must reference the subject agent");
    }

    #[test]
    fn critic_reviews_coder_output() {
        let mut host = AgentHost::new();
        host.deploy("critic").unwrap();
        host.deploy("coder").unwrap();
        let review = host.review("critic", "coder", "fn main() {}").unwrap();
        assert_eq!(review.subject_agent, "coder");
        assert!(review.verdict.contains("coder"));
    }

    #[test]
    fn exec_review_command_formats_correctly() {
        let mut host = AgentHost::new();
        host.deploy("critic").unwrap();
        host.deploy("analyst").unwrap();
        let out = host.exec(&["agent", "review", "analyst", "some findings"]).unwrap();
        assert!(out.contains("critic="));
        assert!(out.contains("subject=analyst"));
        assert!(out.contains("verdict="));
    }

    #[test]
    fn non_critic_agent_cannot_review() {
        let mut host = AgentHost::new();
        host.deploy("writer").unwrap();
        host.deploy("researcher").unwrap();
        // writer is not the critic agent — review must fail
        assert!(matches!(
            host.review("writer", "researcher", "some output"),
            Err(HostError::Usage(_))
        ));
    }

    #[test]
    fn review_subject_not_deployed_returns_not_deployed() {
        let mut host = AgentHost::new();
        host.deploy("critic").unwrap();
        // planner not deployed
        assert!(matches!(
            host.review("critic", "planner", "output"),
            Err(HostError::NotDeployed(_))
        ));
    }
}
