use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub local_only: bool,
    pub cloud_opt_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
    Created,
    Running,
    Paused,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Agent {
    pub id: String,
    pub state: AgentState,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Permission {
    pub agent_id: String,
    pub capability: String,
    pub granted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentEvent {
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentRecord {
    pub id: u64,
    pub text: String,
    pub agent_id: String,
    pub events: Vec<IntentEvent>,
    pub cancelled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MemoryNote {
    pub id: u64,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AuditLine {
    pub sequence: u64,
    pub event: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    NotInitialized,
    NotStarted,
    UnknownProfile(String),
    CloudDenied,
    UnknownAgent(String),
    AlreadyRevoked(String),
    AlreadyPaused(String),
    AlreadyExists(String),
    Usage(String),
}

impl std::fmt::Display for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "run init before start"),
            Self::NotStarted => write!(f, "run start before intent"),
            Self::UnknownProfile(name) => write!(f, "unknown profile: {name}"),
            Self::CloudDenied => write!(f, "cloud is opt-in only"),
            Self::UnknownAgent(id) => write!(f, "unknown agent: {id}"),
            Self::AlreadyRevoked(id) => write!(f, "agent already revoked: {id}"),
            Self::AlreadyPaused(id) => write!(f, "agent already paused: {id}"),
            Self::AlreadyExists(id) => write!(f, "agent already exists: {id}"),
            Self::Usage(msg) => write!(f, "{msg}"),
        }
    }
}

#[derive(Debug, Default)]
pub struct Session {
    profile: Option<Profile>,
    started: bool,
    agents: Vec<Agent>,
    permissions: Vec<Permission>,
    intents: Vec<IntentRecord>,
    notes: Vec<MemoryNote>,
    audit: Vec<AuditLine>,
    next_intent: u64,
    next_note: u64,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn profile(&self) -> Option<&Profile> {
        self.profile.as_ref()
    }

    pub fn started(&self) -> bool {
        self.started
    }

    pub fn agents(&self) -> &[Agent] {
        &self.agents
    }

    pub fn permissions(&self) -> &[Permission] {
        &self.permissions
    }

    pub fn intents(&self) -> &[IntentRecord] {
        &self.intents
    }

    pub fn memory(&self) -> &[MemoryNote] {
        &self.notes
    }

    pub fn audit(&self) -> &[AuditLine] {
        &self.audit
    }

    pub fn init(&mut self, profile: &str) -> Result<Profile, SessionError> {
        if profile != "developer" {
            return Err(SessionError::UnknownProfile(profile.into()));
        }
        let profile = Profile {
            name: "developer".into(),
            local_only: true,
            cloud_opt_in: false,
        };
        self.profile = Some(profile.clone());
        self.started = false;
        self.agents = vec![Agent {
            id: "local-researcher".into(),
            state: AgentState::Running,
        }];
        self.permissions = default_grants("local-researcher");
        self.record("init profile=developer");
        Ok(profile)
    }

    pub fn start(&mut self) -> Result<(), SessionError> {
        if self.profile.is_none() {
            return Err(SessionError::NotInitialized);
        }
        self.started = true;
        self.record("start");
        Ok(())
    }

    pub fn create_agent(&mut self, id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let id = id.trim();
        if id.is_empty() {
            return Err(SessionError::Usage("usage: agent create <name>".into()));
        }
        if self.agents.iter().any(|a| a.id == id) {
            return Err(SessionError::AlreadyExists(id.into()));
        }
        let agent = Agent {
            id: id.into(),
            state: AgentState::Created,
        };
        self.agents.push(agent.clone());
        self.permissions.extend(default_grants(id));
        self.record(format!("agent-create {id}"));
        Ok(agent)
    }

    pub fn deploy_agent(&mut self, id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = {
            let agent = self
                .agents
                .iter_mut()
                .find(|a| a.id == id)
                .ok_or_else(|| SessionError::UnknownAgent(id.into()))?;
            match agent.state {
                AgentState::Revoked => return Err(SessionError::AlreadyRevoked(id.into())),
                AgentState::Running => return Err(SessionError::AlreadyExists(id.into())),
                AgentState::Paused => {
                    return Err(SessionError::Usage(format!("{id} is paused; resume first")))
                }
                AgentState::Created => agent.state = AgentState::Running,
            }
            agent.clone()
        };
        self.record(format!("agent-deploy {id}"));
        Ok(agent)
    }

    pub fn remember(&mut self, text: &str) -> Result<MemoryNote, SessionError> {
        self.require_started()?;
        let text = text.trim();
        if text.is_empty() {
            return Err(SessionError::Usage("memory text required".into()));
        }
        self.next_note += 1;
        let note = MemoryNote {
            id: self.next_note,
            text: text.into(),
        };
        self.notes.push(note.clone());
        self.record(format!("memory {}", note.id));
        Ok(note)
    }

    pub fn declare_intent(&mut self, text: &str) -> Result<IntentRecord, SessionError> {
        self.require_started()?;
        let text = text.trim();
        if text.is_empty() {
            return Err(SessionError::Usage("intent text required".into()));
        }
        // require_started() guarantees profile is Some; use ok_or to convert
        // explicitly rather than relying on that implicit invariant.
        let cloud_opt_in = self
            .profile
            .as_ref()
            .ok_or(SessionError::NotInitialized)?
            .cloud_opt_in;
        if wants_cloud(text) && !cloud_opt_in {
            return Err(SessionError::CloudDenied);
        }
        let agent_id = self
            .agents
            .iter()
            .find(|a| a.state == AgentState::Running)
            .ok_or_else(|| SessionError::Usage("no running agent".into()))?
            .id
            .clone();
        self.next_intent += 1;
        let mut record = IntentRecord {
            id: self.next_intent,
            text: text.into(),
            agent_id,
            events: vec![
                IntentEvent {
                    kind: "admitted".into(),
                    detail: format!("local-only intent {}", self.next_intent),
                },
                IntentEvent {
                    kind: "streamed".into(),
                    detail: format!("working:{text}"),
                },
            ],
            cancelled: false,
        };
        if agent_revoked(&self.agents, &record.agent_id) {
            record.cancelled = true;
            record.events.push(IntentEvent {
                kind: "cancelled".into(),
                detail: format!("revoked:{}", record.agent_id),
            });
        } else {
            record.events.push(IntentEvent {
                kind: "completed".into(),
                detail: format!("result:{}", record.agent_id),
            });
        }
        self.intents.push(record.clone());
        self.record(format!("intent {}", record.id));
        Ok(record)
    }

    pub fn pause(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = {
            let agent = self
                .agents
                .iter_mut()
                .find(|a| a.id == agent_id)
                .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
            match agent.state {
                AgentState::Revoked => return Err(SessionError::AlreadyRevoked(agent_id.into())),
                AgentState::Paused => return Err(SessionError::AlreadyPaused(agent_id.into())),
                AgentState::Created => {
                    return Err(SessionError::Usage(format!("{agent_id} is not deployed")))
                }
                AgentState::Running => agent.state = AgentState::Paused,
            }
            agent.clone()
        };
        self.record(format!("pause {agent_id}"));
        Ok(agent)
    }

    pub fn resume(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = {
            let agent = self
                .agents
                .iter_mut()
                .find(|a| a.id == agent_id)
                .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
            match agent.state {
                AgentState::Revoked => return Err(SessionError::AlreadyRevoked(agent_id.into())),
                AgentState::Running => {
                    return Err(SessionError::Usage(format!(
                        "{agent_id} is already running"
                    )))
                }
                AgentState::Created => {
                    return Err(SessionError::Usage(format!("{agent_id} is not deployed")))
                }
                AgentState::Paused => agent.state = AgentState::Running,
            }
            agent.clone()
        };
        self.record(format!("resume {agent_id}"));
        Ok(agent)
    }

    pub fn revoke(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = {
            let agent = self
                .agents
                .iter_mut()
                .find(|a| a.id == agent_id)
                .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
            if agent.state == AgentState::Revoked {
                return Err(SessionError::AlreadyRevoked(agent_id.into()));
            }
            agent.state = AgentState::Revoked;
            agent.clone()
        };
        for intent in &mut self.intents {
            if intent.agent_id == agent_id && !intent.cancelled {
                intent.cancelled = true;
                intent.events.push(IntentEvent {
                    kind: "cancelled".into(),
                    detail: format!("revoked:{agent_id}"),
                });
            }
        }
        self.record(format!("revoke {agent_id}"));
        Ok(agent)
    }

    pub fn exec(&mut self, args: &[&str]) -> Result<String, SessionError> {
        match args.first().copied() {
            Some("init") => {
                let profile = parse_profile(args)?;
                let profile = self.init(profile)?;
                Ok(format!(
                    "initialized profile={} local_only={} cloud_opt_in={}",
                    profile.name, profile.local_only, profile.cloud_opt_in
                ))
            }
            Some("start") => {
                self.start()?;
                Ok("started local session".into())
            }
            Some("agent") => {
                match args.get(1).copied() {
                    Some("create") => {
                        let id = args.get(2).copied().ok_or_else(|| {
                            SessionError::Usage("usage: agent create <name>".into())
                        })?;
                        let agent = self.create_agent(id)?;
                        Ok(format!("created agent {} state=Created", agent.id))
                    }
                    Some("deploy") => {
                        let id = args.get(2).copied().ok_or_else(|| {
                            SessionError::Usage("usage: agent deploy <name>".into())
                        })?;
                        let agent = self.deploy_agent(id)?;
                        Ok(format!("deployed agent {} state=Running", agent.id))
                    }
                    _ => Err(SessionError::Usage("usage: agent <create|deploy>".into())),
                }
            }
            Some("intent") => {
                let text = args.get(1..).unwrap_or(&[]).join(" ");
                let record = self.declare_intent(&text)?;
                Ok(format!(
                    "intent_id={} agent={} cancelled={} events={}",
                    record.id,
                    record.agent_id,
                    record.cancelled,
                    record.events.len()
                ))
            }
            Some("memory") => {
                if args.len() == 1 {
                    Ok(format!("memory_notes={}", self.notes.len()))
                } else {
                    let text = args[1..].join(" ");
                    let note = self.remember(&text)?;
                    Ok(format!("memory_id={} text={}", note.id, note.text))
                }
            }
            Some("audit") => Ok(format!("audit_events={}", self.audit.len())),
            Some("pause") => {
                let id = args
                    .get(1)
                    .copied()
                    .ok_or_else(|| SessionError::Usage("usage: pause <agent>".into()))?;
                let agent = self.pause(id)?;
                Ok(format!("paused {}", agent.id))
            }
            Some("resume") => {
                let id = args
                    .get(1)
                    .copied()
                    .ok_or_else(|| SessionError::Usage("usage: resume <agent>".into()))?;
                let agent = self.resume(id)?;
                Ok(format!("resumed {}", agent.id))
            }
            Some("revoke") => {
                let id = args
                    .get(1)
                    .copied()
                    .ok_or_else(|| SessionError::Usage("usage: revoke <agent>".into()))?;
                let agent = self.revoke(id)?;
                Ok(format!("revoked {}", agent.id))
            }
            Some("status") => Ok(format!(
                "started={} agents={} intents={} memory={} audit={}",
                self.started,
                self.agents.len(),
                self.intents.len(),
                self.notes.len(),
                self.audit.len()
            )),
            _ => Err(SessionError::Usage(
                "usage: gaia <init|start|agent|intent|memory|audit|pause|resume|revoke|status>"
                    .into(),
            )),
        }
    }

    fn require_started(&self) -> Result<(), SessionError> {
        if self.profile.is_none() {
            return Err(SessionError::NotInitialized);
        }
        if !self.started {
            return Err(SessionError::NotStarted);
        }
        Ok(())
    }

    fn record(&mut self, event: impl Into<String>) {
        self.audit.push(AuditLine {
            sequence: self.audit.len() as u64 + 1,
            event: event.into(),
        });
    }
}

fn default_grants(agent_id: &str) -> Vec<Permission> {
    vec![
        Permission {
            agent_id: agent_id.into(),
            capability: "MemoryRead".into(),
            granted: true,
        },
        Permission {
            agent_id: agent_id.into(),
            capability: "Network".into(),
            granted: false,
        },
    ]
}

fn parse_profile<'a>(args: &[&'a str]) -> Result<&'a str, SessionError> {
    for arg in args {
        if let Some(value) = arg.strip_prefix("--profile=") {
            return Ok(value);
        }
    }
    Ok("developer")
}

fn wants_cloud(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.contains("cloud") || lower.contains("--cloud")
}

fn agent_revoked(agents: &[Agent], id: &str) -> bool {
    agents
        .iter()
        .any(|a| a.id == id && a.state == AgentState::Revoked)
}
