# 0580 — Phase 5 checker-cluster row and evidence attachment package

## Objective

Phase 5 の current promoted line として、

- `specs/examples/257-current-l2-checker-cluster-matrix-ready-minimal-checker-cluster-row-threshold.md`
- `specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md`

を追加し、small decidable core / checker-cluster matrix line の next cut を

1. minimal checker-cluster row core
2. `fixture_evidence_refs` attachment

まで source-backed に整理する。

## Scope and assumptions

- docs-first package に限る。
- actual checker artifact / public checker API / final type system actualization には進まない。
- theorem-line retained bridge の current stop line は
  `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body`
  のまま維持する。
- `checked_reason_codes` は current source evidence として参照するが、cluster row core には immediate に混ぜない。

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
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/49-current-l2-shared-family-checker-support-helper.md`
- `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md`
- `specs/examples/255-current-l2-theorem-line-minimal-handoff-transport-channel-body-ready-low-level-memory-order-family-threshold.md`
- `specs/examples/256-current-l2-small-decidable-core-ready-checker-cluster-matrix-comparison.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `tasks.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`

## Actions taken

1. Phase 5 の theorem-line stop line と checker-side return judgment を `255` / `256` まで再確認した。
2. minimal checker-cluster row の field 候補を
   - `cluster_kind`
   - `checker_subject`
   - `decidable_class`
   - `external_handoff`
   - `fixture_evidence_refs`
   - `typed_reason_family_hint`
   に絞って比較した。
3. `257` を追加し、row core は 4 field に留め、`fixture_evidence_refs` と `typed_reason_family_hint` は row core に混ぜない current first choice を固定した。
4. `258` を追加し、row core の次段では `fixture_evidence_refs` を attachment として足し、`typed_reason_family_hint` は still next threshold に残す current first choice を固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/13-heavy-future-workstreams.md`、`plan/17-research-phases-and-autonomy-gates.md`、`plan/90-source-traceability.md`、`progress.md`、`tasks.md`、`docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md` を current snapshot へ追随させた。
6. report 本体としてこのファイルを追加し、review record として `docs/reports/0581-review-phase5-checker-cluster-row-and-evidence-attachment-package.md` を残した。
7. reviewer completion を受けて、`plan/12`、`plan/90`、本 report の evidence / order を補正した。
8. `plan/` 更新あり、`tasks.md` 更新あり、`progress.md` 更新あり。

## Evidence / outputs / test results

```text
$ date '+%Y-%m-%d %H:%M %Z'
2026-04-11 14:08 JST

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

- checker-side judgment:
  - row core は `cluster_kind + checker_subject + decidable_class + external_handoff` に留めるのが最小
  - `fixture_evidence_refs` は row attachment として足すのが自然
  - `typed_reason_family_hint` は cluster row core に immediate に混ぜず、still next threshold に残す方が current L2 の readiness 差を壊さない
- reviewer judgment:
  - `257` keeps the row core at `cluster_kind + checker_subject + decidable_class + external_handoff`
  - `258` keeps `fixture_evidence_refs` as a separate attachment and still leaves `typed_reason_family_hint` to the next threshold

## What changed in understanding

- Phase 5 の current promoted line は theorem-line の further actualization ではなく、checker-side cluster line の整理へ移ったと明確になった。
- minimal checker-cluster row core と source evidence attachment を分けることで、current repo に already ある `checked_reason_codes` / checker spike / static-only baseline を user-facing に見せつつ、actual checker artifact を premature に固定しない cut が取れると分かった。
- `typed_reason_family_hint` は useful だが、current phase では row core ではなく next threshold で扱う方が自然だと整理できた。

## Open questions

- `typed_reason_family_hint` を current checker-cluster line に optional attachment として入れてよいか
- `fixture_evidence_refs` attachment を row-local list にするか cluster-bundle-local にするか
- checker-side docs-only matrix を later public checker artifact へどう ratchet するか

## Suggested next prompt

`specs/examples/258-current-l2-minimal-checker-cluster-row-ready-checker-cluster-fixture-evidence-attachment-comparison.md` を前提に、Phase 5 の current promoted line として `checker-cluster-fixture-evidence-attachment-ready typed-reason-family-hint threshold` を docs-first に比較し、`typed_reason_family_hint` を current checker-cluster line に入れてよいかを narrow に整理してください。
