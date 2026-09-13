use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Profile {
    pub name: String,
    pub local_only: bool,
    pub cloud_opt_in: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AgentState {
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SessionError {
    NotInitialized,
    NotStarted,
    UnknownProfile(String),
    CloudDenied,
    UnknownAgent(String),
    AlreadyRevoked(String),
    AlreadyPaused(String),
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
    next_intent: u64,
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
        self.permissions = vec![
            Permission {
                agent_id: "local-researcher".into(),
                capability: "MemoryRead".into(),
                granted: true,
            },
            Permission {
                agent_id: "local-researcher".into(),
                capability: "Network".into(),
                granted: false,
            },
        ];
        Ok(profile)
    }

    pub fn start(&mut self) -> Result<(), SessionError> {
        if self.profile.is_none() {
            return Err(SessionError::NotInitialized);
        }
        self.started = true;
        Ok(())
    }

    pub fn declare_intent(&mut self, text: &str) -> Result<IntentRecord, SessionError> {
        self.require_started()?;
        let text = text.trim();
        if text.is_empty() {
            return Err(SessionError::Usage("intent text required".into()));
        }
        if wants_cloud(text) && !self.profile.as_ref().unwrap().cloud_opt_in {
            return Err(SessionError::CloudDenied);
        }
        let agent = self
            .agents
            .iter()
            .find(|a| a.state == AgentState::Running)
            .ok_or_else(|| SessionError::Usage("no running agent".into()))?;
        self.next_intent += 1;
        let mut record = IntentRecord {
            id: self.next_intent,
            text: text.into(),
            agent_id: agent.id.clone(),
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
        Ok(record)
    }

    pub fn pause(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = self
            .agents
            .iter_mut()
            .find(|a| a.id == agent_id)
            .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
        match agent.state {
            AgentState::Revoked => return Err(SessionError::AlreadyRevoked(agent_id.into())),
            AgentState::Paused => return Err(SessionError::AlreadyPaused(agent_id.into())),
            AgentState::Running => agent.state = AgentState::Paused,
        }
        Ok(agent.clone())
    }

    pub fn resume(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = self
            .agents
            .iter_mut()
            .find(|a| a.id == agent_id)
            .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
        match agent.state {
            AgentState::Revoked => return Err(SessionError::AlreadyRevoked(agent_id.into())),
            AgentState::Running => {
                return Err(SessionError::Usage(format!("{agent_id} is already running")))
            }
            AgentState::Paused => agent.state = AgentState::Running,
        }
        Ok(agent.clone())
    }

    pub fn revoke(&mut self, agent_id: &str) -> Result<Agent, SessionError> {
        self.require_started()?;
        let agent = self
            .agents
            .iter_mut()
            .find(|a| a.id == agent_id)
            .ok_or_else(|| SessionError::UnknownAgent(agent_id.into()))?;
        if agent.state == AgentState::Revoked {
            return Err(SessionError::AlreadyRevoked(agent_id.into()));
        }
        agent.state = AgentState::Revoked;
        for intent in &mut self.intents {
            if intent.agent_id == agent_id && !intent.cancelled {
                intent.cancelled = true;
                intent.events.push(IntentEvent {
                    kind: "cancelled".into(),
                    detail: format!("revoked:{agent_id}"),
                });
            }
        }
        Ok(agent.clone())
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
            Some("pause") => {
                let id = args.get(1).copied().ok_or_else(|| {
                    SessionError::Usage("usage: pause <agent>".into())
                })?;
                let agent = self.pause(id)?;
                Ok(format!("paused {}", agent.id))
            }
            Some("resume") => {
                let id = args.get(1).copied().ok_or_else(|| {
                    SessionError::Usage("usage: resume <agent>".into())
                })?;
                let agent = self.resume(id)?;
                Ok(format!("resumed {}", agent.id))
            }
            Some("revoke") => {
                let id = args.get(1).copied().ok_or_else(|| {
                    SessionError::Usage("usage: revoke <agent>".into())
                })?;
                let agent = self.revoke(id)?;
                Ok(format!("revoked {}", agent.id))
            }
            Some("status") => Ok(format!(
                "started={} agents={} intents={}",
                self.started,
                self.agents.len(),
                self.intents.len()
            )),
            _ => Err(SessionError::Usage(
                "usage: gaia <init|start|intent|pause|resume|revoke|status>".into(),
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
