# Report 0581 — review phase5 checker cluster row and evidence attachment package

## Objective

Review the current working tree changes for the Phase 5 checker-cluster package, focusing on:

- whether checker-cluster row core stays separate from fixture evidence attachment without prematurely freezing an actual checker artifact or public API
- whether stale snapshot / promoted-line / package-close references remain in docs or plan mirrors
- whether `docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md` satisfies AGENTS structure and accurately reflects what changed

## Scope and assumptions

- Review only; no normative decision is changed here.
- Normative judgment remains in `specs/`.
- `progress.md`, `tasks.md`, and `plan/` are evaluated as mirrors / repository memory, not as normative sources.
- `plan/ 更新不要`
- `progress.md 更新不要`
- `tasks.md 更新不要`

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`
- `specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md`

## Actions taken

1. Read repository context in AGENTS-mandated order.
2. Inspected the scoped working tree diff for mirrors and summaries.
3. Read `257` and `258` directly to verify the row-core / evidence-attachment separation.
4. Verified the example evidence refs against repo fixtures and checker-spike materials.
5. Ran the commands claimed by `0580` to check whether the report evidence matches the current tree.

## Evidence / outputs / test results

```text
$ df -h .
Filesystem      Size  Used Avail Use% Mounted on
/dev/vda2        99G   93G  1.4G  99% /

$ free -h
               total        used        free      shared  buff/cache   available
Mem:           960Mi       802Mi        92Mi       1.1Mi       220Mi       157Mi
Swap:           19Gi       2.3Gi        17Gi

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 579 numbered report(s).

$ git diff --check
[no output]

$ git status --short --branch
## main...origin/main
 M Documentation.md
 M docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md
 M plan/11-roadmap-near-term.md
 M plan/12-open-problems-and-risks.md
 M plan/13-heavy-future-workstreams.md
 M plan/17-research-phases-and-autonomy-gates.md
 M plan/90-source-traceability.md
 M progress.md
 M specs/00-document-map.md
 M tasks.md
?? docs/reports/0578-phase5-handoff-low-level-memory-order-threshold.md
?? docs/reports/0579-review-phase5-handoff-low-level-memory-order-threshold.md
?? docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md
?? specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md
?? specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md
?? specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md
?? specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md
```

- `257` keeps the row core at `cluster_kind + checker_subject + decidable_class + external_handoff` and explicitly leaves `fixture_evidence_refs` and `typed_reason_family_hint` out of the row core.
- `258` keeps `fixture_evidence_refs` as a separate attachment and still leaves `typed_reason_family_hint` to the next threshold.
- Fixture ids cited in `258` such as `e4_malformed_lineage`, `e12_underdeclared_target_missing`, `e13_malformed_capability_strengthening`, and `e20_malformed_late_capability_strengthening` do exist in the current fixture corpus.

## What changed in understanding

- The checker-cluster package is semantically sound on its main point: the new specs preserve the distinction between row core and evidence attachment and do not prematurely harden an actual checker artifact or public API.
- The remaining defects are in mirror hygiene and traceability/report hygiene rather than in the new spec judgments themselves.

## Open questions

- Whether `plan/90-source-traceability.md` is intended to carry cumulative overlapping addenda for adjacent packages, or whether same-day overlapping addenda should be consolidated when a later package supersedes the earlier mirror state.

## Suggested next prompt

Address the review findings in the current checker-cluster package: update `plan/12-open-problems-and-risks.md` to reflect the `257...258` promoted-line state, fix `plan/90-source-traceability.md` so the checker-cluster addendum cites `docs/reports/0580-phase5-checker-cluster-row-and-evidence-attachment-package.md` and does not leave overlapping stale addenda, and bring `0580` into AGENTS order while adding the missing command/output and file-change evidence.
