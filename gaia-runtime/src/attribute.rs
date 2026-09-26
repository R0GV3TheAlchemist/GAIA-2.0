//! Sentence-level `[source: hex]` markers. Lexical overlap only.

use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributedSentence {
    pub text: String,
    pub source_ids: Vec<String>,
    pub marked: String,
}

fn tokens(text: &str) -> HashSet<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|t| t.len() > 2)
        .map(|t| t.to_ascii_lowercase())
        .collect()
}

fn split_sentences(response: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut buf = String::new();
    for ch in response.chars() {
        buf.push(ch);
        if matches!(ch, '.' | '!' | '?') {
            let s = buf.trim().to_string();
            if !s.is_empty() {
                out.push(s);
            }
            buf.clear();
        }
    }
    let tail = buf.trim();
    if !tail.is_empty() {
        out.push(tail.to_string());
    }
    out
}

/// Attach source ids whose chunk text shares a token with the sentence.
pub fn attribute_sentences(
    response: &str,
    sources: &[(String, String)],
) -> Vec<AttributedSentence> {
    split_sentences(response)
        .into_iter()
        .map(|text| {
            let sent_toks = tokens(&text);
            let mut source_ids = Vec::new();
            for (id, chunk) in sources {
                let chunk_toks = tokens(chunk);
                if sent_toks.intersection(&chunk_toks).next().is_some() {
                    source_ids.push(id.clone());
                }
            }
            let marked = if source_ids.is_empty() {
                text.clone()
            } else {
                format!("{} [source: {}]", text.trim_end(), source_ids.join("; "))
            };
            AttributedSentence {
                text,
                source_ids,
                marked,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn overlapping_sentence_gets_marker() {
        let sources = vec![(
            "abc".into(),
            "The treaty was signed in 1992 in Rio.".into(),
        )];
        let out = attribute_sentences("The treaty was signed in 1992. Water is wet.", &sources);
        assert_eq!(out.len(), 2);
        assert!(out[0].marked.contains("[source: abc]"));
        assert!(out[1].source_ids.is_empty());
    }

    #[test]
    fn no_overlap_has_no_marker() {
        let sources = vec![("abc".into(), "purple piano recipes".into())];
        let out = attribute_sentences("Climate models observe ice.", &sources);
        assert!(out[0].source_ids.is_empty());
        assert_eq!(out[0].marked, out[0].text);
    }
}
