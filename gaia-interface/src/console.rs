//! #28 permission console, intent trace, and Studio recipe list.

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
                    AgentState::Created => "Created".into(),
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
        let mut agents = String::new();
        for row in self.agents() {
            agents.push_str("<tr><td>");
            agents.push_str(&escape(&row.id));
            agents.push_str("</td><td>");
            agents.push_str(&escape(&row.state));
            agents.push_str("</td><td>");
            if row.pausable {
                agents.push_str("<button type=\"button\" aria-label=\"Pause ");
                agents.push_str(&escape(&row.id));
                agents.push_str("\">Pause</button> ");
            }
            if row.revocable {
                agents.push_str("<button type=\"button\" aria-label=\"Revoke ");
                agents.push_str(&escape(&row.id));
                agents.push_str("\">Revoke</button>");
            } else {
                agents.push_str("<span>not revocable</span>");
            }
            agents.push_str("</td></tr>");
        }
        let mut matrix = String::new();
        for cell in self.matrix() {
            matrix.push_str("<tr><td>");
            matrix.push_str(&escape(&cell.agent_id));
            matrix.push_str("</td><td>");
            matrix.push_str(&escape(&cell.capability));
            matrix.push_str("</td><td>");
            matrix.push_str(if cell.granted { "granted" } else { "denied" });
            matrix.push_str("</td></tr>");
        }
        let mut events = String::new();
        for event in self.traces() {
            events.push_str("<li>intent ");
            events.push_str(&event.intent_id.to_string());
            events.push_str(" / ");
            events.push_str(&escape(&event.kind));
            events.push_str(" / ");
            events.push_str(&escape(&event.detail));
            events.push_str("</li>");
        }
        let mut recipes = String::new();
        for recipe in self.studio.recipes() {
            recipes.push_str("<li>");
            recipes.push_str(&escape(&recipe.name));
            recipes.push_str(" handles ");
            recipes.push_str(&escape(&recipe.intent));
            recipes.push_str("</li>");
        }
        let mut page =
            String::from("<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"utf-8\">");
        page.push_str("<title>GAIA permission console</title>");
        page.push_str(
            "<style>body{background:#111;color:#eee;font-family:sans-serif;line-height:1.5;}",
        );
        page.push_str("a:focus,button:focus{outline:3px solid #ffd166;}");
        page.push_str("button{background:#222;color:#fff;border:2px solid #eee;}");
        page.push_str(
            "table{border-collapse:collapse;}td,th{border:1px solid #666;padding:0.4rem 0.6rem;}",
        );
        page.push_str("</style></head><body>");
        page.push_str("<a href=\"#main\">Skip to content</a><main id=\"main\">");
        page.push_str("<h1>GAIA permission console</h1>");
        page.push_str("<section aria-labelledby=\"agents-h\"><h2 id=\"agents-h\">Agents</h2>");
        page.push_str(
            "<table><thead><tr><th>Agent</th><th>State</th><th>Control</th></tr></thead><tbody>",
        );
        page.push_str(&agents);
        page.push_str("</tbody></table></section>");
        page.push_str(
            "<section aria-labelledby=\"matrix-h\"><h2 id=\"matrix-h\">Permission matrix</h2>",
        );
        page.push_str(
            "<table><thead><tr><th>Agent</th><th>Capability</th><th>Grant</th></tr></thead><tbody>",
        );
        page.push_str(&matrix);
        page.push_str("</tbody></table></section>");
        page.push_str("<section aria-labelledby=\"trace-h\" aria-live=\"polite\"><h2 id=\"trace-h\">Intent stream</h2><ol>");
        page.push_str(&events);
        page.push_str("</ol></section>");
        page.push_str(
            "<section aria-labelledby=\"studio-h\"><h2 id=\"studio-h\">Studio recipes</h2><ul>",
        );
        page.push_str(&recipes);
        page.push_str("</ul></section></main></body></html>");
        page
    }
}

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace("\"", "&quot;")
}
