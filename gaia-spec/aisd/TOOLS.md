# AISD open-tool registry

Tools are not foundation-model cards (#100). Scores stay empty until measured evals (#124).
Crate already on main: `gaia-aisd::{graphcast, reference_published}`.
Issue this slice: #479 / #127.

## Tool card fields (names)

license, hardware, modalities, linked skills, research-only flag.
Only `graphcast()` is implemented as a `ToolCard` today.

## GraphCast

- kind: `weather-forecast-tool`
- skill: `aisd:science:weather-forecast`
- `is_llm_card == false`
- not live ingest

Other names in `named-tools.csv` are listed only (Whisper, SD, Kokoro, DeepSeek Coder). No constructors added.

## Benchmarks

Named benches in `benches.csv`. Score columns empty until #124.
`reference_published(name, value)` stores the published figure as reference and leaves `gaia_measured` blank.

## Routing hints

19 system-type tags are a *future* routing hint list, not an API on main. Do not invent a tag enum here.

## Refuse

- GraphCast as an LLM card
- published Elo/MMLU as `gaia_measured`
- live Whisper / GraphCast / DestinE
