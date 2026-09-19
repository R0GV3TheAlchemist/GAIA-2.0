# Agent hygiene (#slop / #credit)

## What is broken in the wider world (2025–2026)

These are observed classes, not capabilities we ship:

| Class | What happens | Counter here |
| --- | --- | --- |
| API hallucination | Agent writes `Lake::in_memory` when only `Lake::new` exists | `check_invented_symbols.py` + AGENTS.md |
| Package / repo hallucination | Agent clones a guessed slug; attacker registered it first (HalluSquatting, FakeGit) | Official URL only |
| Workslop / slop grenades | Looks finished, wastes the next human | One issue, tests on real types, no Complete-Build dumps |
| Credit strip | Tools emit code without CMI; courts may not treat that as DMCA "removal" | NOTICE + CITATION.cff + keep headers |
| Unauthorized edit | Agent changes files the user forbade | AGENTS.md hard refuse |

## What this repo will not pretend

A public GitHub tree cannot stop a lab from training. Ninth Circuit *Doe v. GitHub* (16 Sep 2026) said generating new text without CMI is not automatically §1202 removal. License and attribution still apply to **copies and substantial excerpts**. That is why NOTICE exists.

## Merge gate

A PR that introduces a symbol on the invented-symbol list fails CI.
