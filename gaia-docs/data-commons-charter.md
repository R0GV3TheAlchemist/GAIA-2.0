# GAIA Data Commons charter v0 (#40)

Working policy for ingest tickets. Not a Foundation legal opinion and not a ratified ethics board.

## License map

| Layer | License |
| --- | --- |
| Raw measurements | CC0 |
| Derived products | CC-BY-4.0 |
| Model weights | Apache-2.0 |
| Specs / schemas | CC0 |

Cite one of these on every ingest ticket. A blank license is refused in `gaia-earth` policy.

## Mandatory conventions

FAIR, STAC, OGC, and CF Conventions are required for a live collection. Fixture collections in this tree only have to carry license, provenance, and uncertainty.

## Ethics

- Equity: basic planetary status is not paywalled.
- Transparency: method and provenance travel with the point.
- Uncertainty is truth. A point without uncertainty is refused.
- Non-weaponization: targeting, individual surveillance, and harm uses are refused.
- Data sovereignty: national-node processing stays in the tagged region.
- Species inclusion: biosphere observations are first-class, not optional decoration.
- Seventh-generation impact: course-correction must name a trackable indicator.

## Forbidden uses

- Weapon targeting or battle-damage assessment
- Individual surveillance or doxxing
- Knowingly harmful environmental intervention without disclosure

## National nodes

A collection tagged `residency=US-TX` may not be processed as `EU-DE`. In-country processing is the default when a residency tag is present.

## How an ingest ticket cites this

1. License class from the map above.
2. Quality / uncertainty rule: `Observation::admit` plus this charter.
3. Purpose string that survives `allow_purpose`.
