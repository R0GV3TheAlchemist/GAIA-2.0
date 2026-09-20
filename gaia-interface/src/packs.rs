//! #104 pack wiring for gaia-interface — science cards, professional disclaimers,
//! and insurer-automation refusal surface.
//!
//! This module calls the `gaia_aikd` pack APIs and formats output for the
//! existing HTML console and JSON gateway patterns in this crate.

use gaia_aikd::{
    gaia_certifies_usmle_or_bar, insurer_automation, professional_disclaimer, AikdError,
    ScienceAnswer,
};

// ── Science card ─────────────────────────────────────────────────────────────

/// Build a rendered science-answer card.
///
/// Returns `Err(AikdError::MissingCitation)` when `product_id` is empty,
/// propagating the AIKD requirement that every science answer cites an Earth Twin product.
pub fn render_science_card(question: &str, product_id: &str) -> Result<String, AikdError> {
    let pack = ScienceAnswer::with_product(product_id)?;
    Ok(format!(
        "[science] {q}\n\nEarth Twin product: {et}\n\n{txt}",
        q = escape(question),
        et = escape(&pack.earth_twin_product),
        txt = escape(&pack.text),
    ))
}

/// Render a science card as an HTML `<article>` block, suitable for embedding
/// in the `render_html` output of `PermissionConsole` or a future answer page.
pub fn render_science_card_html(question: &str, product_id: &str) -> Result<String, AikdError> {
    let pack = ScienceAnswer::with_product(product_id)?;
    Ok(format!(
        "<article class=\"science-card\">\n\
         <h2>{q}</h2>\n\
         <p class=\"et-product\">Earth Twin product: <code>{et}</code></p>\n\
         <p>{txt}</p>\n\
         </article>",
        q = escape(question),
        et = escape(&pack.earth_twin_product),
        txt = escape(&pack.text),
    ))
}

// ── Professional disclaimer ───────────────────────────────────────────────────

/// Append a professional disclaimer to `body` for the given pack `kind`
/// (e.g. `"medical"`, `"legal"`, `"finance"`).
///
/// The disclaimer is always appended; callers must not suppress it.
pub fn render_professional_pack(kind: &str, body: &str) -> String {
    let disclaimer = professional_disclaimer(kind);
    format!("{body}\n\n{disclaimer}")
}

/// HTML variant — appends the disclaimer inside a `<p role="note">` so screen
/// readers announce it after the answer body.
pub fn render_professional_pack_html(kind: &str, body: &str) -> String {
    let disclaimer = professional_disclaimer(kind);
    format!(
        "<div class=\"professional-pack\">\n\
         <div class=\"answer-body\">{body}</div>\n\
         <p role=\"note\" class=\"disclaimer\">{disc}</p>\n\
         </div>",
        body = body,
        disc = escape(&disclaimer),
    )
}

// ── USMLE / Bar non-certification guard ──────────────────────────────────────

/// Return an error string if any surface attempts to claim GAIA certifies
/// USMLE or Bar exams.  Always returns `Err`; this function exists so call
/// sites have a named guard rather than an inline assertion.
pub fn guard_no_exam_cert() -> Result<(), String> {
    if gaia_certifies_usmle_or_bar() {
        return Err("GAIA does not certify USMLE or Bar exams".into());
    }
    Ok(())
}

// ── Insurer automation refusal ────────────────────────────────────────────────

/// Attempt insurer automation — always returns an error with a user-facing message.
///
/// Wire any insurer/employer automation path to call this and surface the message;
/// never silently skip the error.
pub fn try_insurer_automation() -> Result<(), String> {
    match insurer_automation() {
        Ok(()) => Err("unexpected success: insurer automation must be refused by GAIA".into()),
        Err(AikdError::ClosedScoreClaim) => {
            Err("insurer automation refused: closed score claims are not permitted by GAIA".into())
        }
        Err(_) => Err("insurer automation refused".into()),
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn escape(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn science_card_requires_product_id() {
        assert!(render_science_card("What is albedo?", "").is_err());
        let card = render_science_card("What is albedo?", "et:climate:albedo").unwrap();
        assert!(card.contains("et:climate:albedo"));
    }

    #[test]
    fn professional_pack_always_has_disclaimer() {
        let out = render_professional_pack("medical", "Aspirin reduces fever.");
        assert!(out.contains("not professional advice"));
        let out = render_professional_pack("legal", "Contracts require offer and acceptance.");
        assert!(out.contains("not professional advice"));
    }

    #[test]
    fn exam_cert_guard_always_passes() {
        assert!(guard_no_exam_cert().is_ok());
    }

    #[test]
    fn insurer_automation_is_refused_with_message() {
        let err = try_insurer_automation().unwrap_err();
        assert!(err.contains("refused"));
    }

    #[test]
    fn html_variants_include_markup() {
        let html = render_science_card_html("What is SST?", "et:ocean:sst").unwrap();
        assert!(html.contains("<article"));
        assert!(html.contains("et:ocean:sst"));
        let html = render_professional_pack_html("finance", "Bonds are debt instruments.");
        assert!(html.contains("role=\"note\""));
        assert!(html.contains("not professional advice"));
    }
}
