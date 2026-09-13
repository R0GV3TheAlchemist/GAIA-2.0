//! #169 grounded labels. Wonder does not loosen facts.

pub fn wonder_mode(opt_in: bool) -> &'static str {
    if opt_in {
        "label-only"
    } else {
        "off"
    }
}

pub fn tag_answer(cited: bool) -> &'static str {
    if cited {
        "cited"
    } else {
        "unverified invention"
    }
}
