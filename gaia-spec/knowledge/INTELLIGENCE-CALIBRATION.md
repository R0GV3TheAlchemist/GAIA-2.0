# Intelligence Layer calibration — listed (#805)

C210: calibrated confidence, not plausible certainty. Sol Niger is fluency mistaken for accuracy.

| Requirement | Existing surface | Status |
| --- | --- | --- |
| Honesty / uncertainty bands | `gaia-aikd` NeedVerify (#575 T5) | implemented band |
| Grounding enforcement | `gaia-runtime` `enforce_grounding` | implemented |
| Faithfulness score | `gaia-runtime` `score_faithfulness` | lexical only |
| Retrieval confidence tiers | `gaia-aikd` `tier2::retrieval_confidence` | listed |
| Live inference provider | — | not implemented |
| `CalibratedConfidence` type | — | refused |

Honesty flags stay false until earned. Sibling protocol: [SOL-NIGER.md](SOL-NIGER.md) (#806).
