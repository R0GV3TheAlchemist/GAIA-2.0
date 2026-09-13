//! #28 permission console, intent trace, and Studio recipe list.
//! Renders HTML for a browser view. This is not React, Vite, or a WebSocket.

use crate::session::{AgentState, Permission, Session};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AgentRow {
    pub id: String,
    pub state: String,
    pub revocable: bool,
    pub pausable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TraceEvent {
    pub intent_id: u64,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StudioRecipe {
    pub name: String,
    pub intent: String,
}

#[derive(Debug, Default)]
pub struct Studio {
    recipes: Vec<StudioRecipe>,
}

impl Studio {
    pub fn compose(&mut self, name: &str, intent: &str) -> Result<StudioRecipe, String> {
        let name = name.trim();
        let intent = intent.trim();
        if name.is_empty() || intent.is_empty() {
            return Err("studio recipe needs a name and an intent".into());
        }
        if self.recipes.iter().any(|r| r.name == name) {
            return Err(format!("recipe exists: {name}"));
        }
        let recipe = StudioRecipe {
            name: name.into(),
            intent: intent.into(),
        };
        self.recipes.push(recipe.clone());
        Ok(recipe)
    }

    pub fn recipes(&self) -> &[StudioRecipe] {
        &self.recipes
    }
}

#[derive(Debug)]
pub struct PermissionConsole<'a> {
    session: &'a Session,
    studio: &'a Studio,
}

impl<'a> PermissionConsole<'a> {
    pub fn new(session: &'a Session, studio: &'a Studio) -> Self {
        Self { session, studio }
    }

    pub fn agents(&self) -> Vec<AgentRow> {
        self.session
            .agents()
            .iter()
            .map(|agent| AgentRow {
                id: agent.id.clone(),
                state: match agent.state {
                    AgentState::Running => "Running".into(),
                    AgentState::Paused => "Paused".into(),
                    AgentState::Revoked => "Revoked".into(),
                },
                revocable: agent.state != AgentState::Revoked,
                pausable: agent.state == AgentState::Running,
            })
            .collect()
    }

    pub fn matrix(&self) -> &[Permission] {
        self.session.permissions()
    }

    pub fn traces(&self) -> Vec<TraceEvent> {
        self.session
            .intents()
            .iter()
            .flat_map(|intent| {
                intent.events.iter().map(|event| TraceEvent {
                    intent_id: intent.id,
                    kind: event.kind.clone(),
                    detail: event.detail.clone(),
                })
            })
            .collect()
    }

    pub fn render_html(&self) -> String {
        let agents = self
            .agents()
            .into_iter()
            .map(|row| {
                let pause = if row.pausable {
                    format!(
                        "<button type=\"button\" aria-label=\"Pause {}">Pause</button>",
                        escape(&row.id)
                    )
                } else {
                    String::new()
                };
                let revoke = if row.revocable {
                    format!(
                        "<button type=\"button\" aria-label=\"Revoke {}">Revoke</button>",
                        escape(&row.id)
                    )
                } else {
                    "<span>not revocable</span>".into()
                };
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{} {}</td></tr>",
                    escape(&row.id),
                    escape(&row.state),
                    pause,
                    revoke
                )
            })
            .collect::<Vec<_>>()
            .join("");
        let matrix = self
            .matrix()
            .iter()
            .map(|cell| {
                format!(
                    "<tr><td>{}</td><td>{}</td><td>{}</td></tr>",
                    escape(&cell.agent_id),
                    escape(&cell.capability),
                    if cell.granted { "granted" } else { "denied" }
                )
            })
            .collect::<Vec<_>>()
            .join("");
        let events = self
            .traces()
            .into_iter()
            .map(|event| {
                format!(
                    "<li>intent {} · {} · {}</li>",
                    event.intent_id,
                    escape(&event.kind),
                    escape(&event.detail)
                )
            })
            .collect::<Vec<_>>()
            .join("");
        let recipes = self
            .studio
            .recipes()
            .iter()
            .map(|recipe| {
                format!(
                    "<li>{} handles {}</li>",
                    escape(&recipe.name),
                    escape(&recipe.intent)
                )
            })
            .collect::<Vec<_>>()
            .join("");
        format!(
            r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<title>GAIA permission console</title>
<style>
body {{ background:#111; color:#eee; font-family:sans-serif; line-height:1.5; }}
a:focus, button:focus {{ outline:3px solid #ffd166; }}
button {{ background:#222; color:#fff; border:2px solid #eee; }}
table {{ border-collapse:collapse; }}
td, th {{ border:1px solid #666; padding:0.4rem 0.6rem; }}
</style>
</head>
<body>
<a href="#main">Skip to content</a>
<main id="main">
<h1>GAIA permission console</h1>
<section aria-labelledby="agents-h">
<h2 id="agents-h">Agents</h2>
<table>
<thead><tr><th>Agent</th><th>State</th><th>Control</th></tr></thead>
<tbody>{agents}</tbody>
</table>
</section>
<section aria-labelledby="matrix-h">
<h2 id="matrix-h">Permission matrix</h2>
<table>
<thead><tr><th>Agent</th><th>Capability</th><th>Grant</th></tr></thead>
<tbody>{matrix}</tbody>
</table>
</section>
<section aria-labelledby="trace-h" aria-live="polite">
<h2 id="trace-h">Intent stream</h2>
<ol>{events}</ol>
</section>
<section aria-labelledby="studio-h">
<h2 id="studio-h">Studio recipes</h2>
<ul>{recipes}</ul>
</section>
</main>
</body>
</html>
"#
        )
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
