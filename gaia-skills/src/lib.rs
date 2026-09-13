//! Skills first cuts (#107–#110). Local-default. No child EI inference.

mod path;
mod profile;
mod schema;

pub use path::novice_public_speaking;
pub use profile::SkillProfile;
pub use schema::{active_listening, SkillNode, REALMS};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    ChildEiBlocked,
    SyncDenied,
    UnknownSkill,
}
