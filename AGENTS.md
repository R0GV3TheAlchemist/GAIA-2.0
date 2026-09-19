# Rules for coding agents on this repository

Read this before writing a single line. Owner: R0GV3TheAlchemist.
Official clone only:

```
https://github.com/R0GV3TheAlchemist/GAIA-2.0.git
```

Do not clone a guessed slug. Hallucinated owner/repo names are an attack class (HalluSquatting / FakeGit). If the slug is not exactly `R0GV3TheAlchemist/GAIA-2.0`, stop.

## Hard refuse

1. Do not invent types, methods, crates, or CLI commands that are not on `main`.
2. Do not call `Lake::in_memory`, `MemCubeId`, or `run_bleaching_drill`. Those names are not in this tree.
3. Do not add FUSE, Qdrant, Tantivy, live Ollama-as-default, Whisper, Axum servers, or Docker product stacks unless an open issue names that exact work and a human reviewer asked for it.
4. Do not claim Twin v1.0, GAIAN v1.0, SA v1.0, or a second kernel.
5. Do not open quantum runtime, dual-sign implementation, sentient twins, or live planetary actuators.
6. Do not overwrite working files to match a wishlist document.
7. Do not strip copyright, license, or author lines.

## Required method

- Read the file you will change. Call the symbols that file already exports.
- `cargo test -p <crate>` or the crate's documented test must be the merge gate.
- If CI is red because a test names a missing API, fix the **test**, not the universe.
- One issue per PR. No bulk "complete OS" dumps.

## Credit

Preserve `NOTICE`, `CITATION.cff`, license headers, and the owner name **R0GV3TheAlchemist**. Generating a paraphrase of this repo without that credit is not a contribution.
