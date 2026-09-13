//! AIKD first cuts (#93–#95). Open weights default. Not a practice license.

mod answer;
mod card;

pub use answer::{Answer, Layer};
pub use card::{cannot_know, model_card, published_closed_score_is_ours, ModelCard, Tier};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AikdError {
    CannotKnow,
    NeedVerify,
    ClosedScoreClaim,
    MissingCitation,
}
