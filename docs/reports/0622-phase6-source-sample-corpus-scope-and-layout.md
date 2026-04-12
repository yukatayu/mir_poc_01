# Report 0622 — phase6 source sample corpus scope and layout

- Date: 2026-04-12T02:22:11.839620Z
- Author / agent: Codex
- Scope: Phase 6 fixed-subset source-sample corpus scope / file layout package、repo-root corpus path actualization、snapshot / roadmap / traceability mirror 更新
- Decision levels touched: L1 / L2

## 1. Objective

- `tasks.md` 先頭 task だった Phase 6 fixed-subset source-sample corpus scope / file layout を source-backed に閉じる。
- representative prose / fixture corpus / source sample の 3 層分離を維持したまま、repo-root path / flat layout / neutral extension / sample ID policy を narrow に固定する。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/`、research abstract、traceability mirror を `representative / fixture / source mapping matrix` mainline へ切り替える。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/311-current-l2-phase6-reserve-formal-tool-binding-inventory-ready-phase6-parser-side-follow-up-package-sequencing-comparison.md`
- `specs/examples/312-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-minimal-phase6-parser-side-follow-up-package-sequencing-threshold.md`
- `specs/examples/313-current-l2-phase6-parser-side-follow-up-package-sequencing-ready-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-comparison.md`
- `specs/examples/314-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-minimal-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `crates/mir-ast/tests/fixtures/current-l2/`
- `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`

## 3. Actions taken

- `specs/examples/315...316` を追加し、source-sample corpus を repo-root `samples/current-l2/` flat `.txt` layer に置く comparison / threshold を fixed した。
- `samples/current-l2/README.md` を新設し、source corpus の位置づけ、initial cluster、non-goals、next steps を repo-tracked path として actualize した。
- `Documentation.md`、`progress.md`、`tasks.md`、`plan/01`、`plan/08`、`plan/10`、`plan/11`、`plan/12`、`plan/15`、`plan/17`、`plan/90`、Phase 6 abstract を更新し、current mainline を `representative / fixture / source mapping matrix` に切り替えた。
- reviewer 指摘を踏まえ、report template を本文化し、Phase 6 abstract の stale wording、`tasks.md` の package numbering drift、`plan/08` の `e23` fixture omission を解消した。

## 4. Files changed

- `Documentation.md`
- `docs/reports/0622-phase6-source-sample-corpus-scope-and-layout.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `samples/current-l2/README.md`
- `specs/00-document-map.md`
- `specs/examples/315-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-ready-fixed-subset-source-sample-corpus-scope-and-file-layout-comparison.md`
- `specs/examples/316-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-ready-minimal-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold.md`
- `tasks.md`

## 5. Commands run and exact outputs

- `python3 scripts/new_report.py --slug phase6-source-sample-corpus-scope-and-layout`
  - `docs/reports/0622-phase6-source-sample-corpus-scope-and-layout.md`
- `ls samples/current-l2`
  - `ls: cannot access 'samples/current-l2': No such file or directory`
- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-12 11:26 JST`

## 6. Evidence / findings

- fixed-subset source sample は parser test private directory に閉じず、repo-root `samples/current-l2/` の第 3 層として置く方が、later lowering / runner / formal ladder の cross-crate path に自然である。
- `.txt` と flat layout に留めることで、final grammar、verdict/stage naming、fixture reverse-generation を premature に固定せずに済む。
- initial cluster `e1` / `e2` / `e3` / `e4` / `e21` / `e23` は current fixture stem と整合しており、`plan/08` に `e23` row を戻すことで mapping matrix の足場が揃った。
- current package は source corpus の path / layout / naming / non-goal までで十分閉じられ、actual sample content、mapping matrix、lowering、runner、verification ladder は next package に押し出すのが staged line に合う。

## 7. Changes in understanding

- source-sample corpus task の本質は sample file を増やすことではなく、representative prose / fixture corpus / source sample の 3 層を path policy で切り分けることにある。
- `e23` は source-only 例ではなく、既存 fixture corpus に already ある stage 2 malformed structural anchor であり、source corpus initial cluster に入れても fixture alignment を壊さない。
- docs mirror 側では、compile-ready checkpoint close、parser-side follow-up actualization、source corpus scope/layout fixed を分けて書かないと stale wording が入りやすい。

## 8. Open questions

- representative prose と `source_example_id` / `fixture_id` を mapping matrix でどう 1 表へ揃えるか
- fixed-subset source sample の actual file content authoring をどの順で始めるか
- sample ごとの `static gate` / `interpreter` / `formal hook` reached stage をどの schema で mirror するか

## 9. Suggested next prompt

- `tasks.md` 先頭 task である `representative / fixture / source mapping matrix` を閉じてください。initial cluster `e1` / `e2` / `e3` / `e4` / `e21` / `e23` を representative prose、fixture corpus、source sample policy の 3 層対応表に整理し、その後の lowering / runner / verification ladder へ渡す最小 matrix shape を fixed してください。
