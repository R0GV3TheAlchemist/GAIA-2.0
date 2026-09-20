# HSPD prohibited-implementation list

The following six items are hard-coded refusals. No call path, agent, surface,
or user request may override them.

1. **`unprescribed_drug_protocols`** — any node, surface, or response that
   provides specific dosing, cycling, or sourcing instructions for pharmacological
   augmentation without a prescribing clinician in the loop.

2. **`recreational_psychedelic_guidance`** — any instructions for recreational
   or unsupervised psychedelic use, including dosing, trip preparation, or
   combination guidance. Research context may be named; protocols may not be given.

3. **`diy_gene_edit`** — any instructions, reagent lists, or procedural guidance
   for self-administered genetic modification, including CRISPR home-use framing.

4. **`implant_surgery_steps`** — any surgical procedural guidance for implanting
   technological augmentation devices, regardless of framing as educational.

5. **`performance_drug_sourcing`** — any guidance on obtaining controlled or
   prescription-only performance-enhancing substances outside a clinical pathway.

6. **`stack_catalog_items`** — listing compounding pharmacies, supplement stacks,
   or experimental compound combinations as HSPD catalog nodes.

All six are blocked at the schema level — `RiskClass::Prohibited` nodes and
`Medical`-class nodes with `dose`/`protocol` fields return `Err(MedicalDiyPath)`
or `Err(ProhibitedItem)` before any output is generated.
