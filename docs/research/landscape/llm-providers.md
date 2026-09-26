# LLM provider layer — listed (#859)

**Points at:** `gaia-hal`, `gaia-gateway`. No new SDK.

Named field (2026 session list, not a live probe):
Anthropic, OpenAI, Google, Mistral, DeepSeek, Groq.
Named abstraction field: LiteLLM, aisuite.

| Question | Listed answer |
| --- | --- |
| What exists here | `gaia-gateway` listed routes + `gaia-hal` HAL crate |
| Routing / fallback / cost | Absent as live policy. Health route only |
| Formal support list | Not mapped. Do not add keys in this PR |

## Refuse

- No provider client. No API key path. No load balancer.
