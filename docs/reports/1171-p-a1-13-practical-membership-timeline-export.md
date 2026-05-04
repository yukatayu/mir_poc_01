# Report 1171 — P-A1-13 Practical Membership Timeline Export Widening

- Date: 2026-05-04
- Author / agent: Codex
- Scope: `P-A1-13` practical membership timeline export widening
- Decision levels touched: `L1`, `L2`

## Objective

Close `P-A1-13` by widening the practical devtools/export lane with `VIS-A1-03` only, using exact existing `SL-A1-02` save/load evidence and without introducing new runtime semantics or collapsing the existing carrier split.

## 日本語要約

`P-A1-13` では、`VIS-A1-03` を `SL-A1-02` exact save-load report から作る thin な membership timeline export として追加した。saved frontier、later live membership advance、restored frontier、stale-membership reject を viewer/export surface に残したまま扱い、distributed durable membership timeline や witness/lease co-timeline、retention/on-demand completion へは広げていない。

## Scope and assumptions

- Scope is limited to `VIS-A1-03`.
- Source carrier remains exact `SL-A1-02`; no synthetic timeline rows were introduced.
- The practical devtools carrier split remains:
  `exact practical reports -> distinct devtools export bundle -> non-final viewer`.
- No parser/front-door widening, no new Rust runtime semantics, and no runnable-root promotion are claimed.
- `VIS-A1-07`, broader save/load widening, and same-session product runtime semantics remain outside this package.

## Start state / dirty state

Started from a dirty worktree containing in-progress `P-A1-13` edits in:

- `README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `scripts/practical_alpha1_export_devtools.py`
- `scripts/tests/test_practical_alpha1_export_devtools.py`
- `specs/18-practical-alpha1-scope.md`
- new file `samples/practical-alpha1/expected/vis-a1-03-membership-timeline.expected.json`

No unrelated user changes were introduced during this package.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `docs/reports/TEMPLATE.md`
- implementation/test files:
  `scripts/practical_alpha1_export_devtools.py`,
  `scripts/practical_alpha1_save_load.py`,
  `scripts/tests/test_practical_alpha1_export_devtools.py`,
  `scripts/tests/test_practical_alpha1_save_load.py`

## Actions taken

1. Confirmed the exact source carrier choice: `SL-A1-02` already exposes saved frontier, later live membership advance, restored frontier, and stale-membership rejection, so `VIS-A1-03` can be widened without new semantics.
2. Applied TDD RED->GREEN on `scripts/tests/test_practical_alpha1_export_devtools.py` for `VIS-A1-03`.
3. Added `VIS-A1-03` bundle assembly to `scripts/practical_alpha1_export_devtools.py`, reusing exact `practical_alpha1_save_load.run_sample("SL-A1-02")`.
4. Added exact expected bundle file `vis-a1-03-membership-timeline.expected.json`.
5. Synced `specs/18`, `plan/01`, `plan/44`, `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample READMEs, and `scripts/README.md` to make `P-A1-13` the latest practical closeout and keep `VIS-A1-07` as the only remaining practical devtools observable.
6. Added this closeout report and reran the focused validation floor for save/load + devtools + docs scaffolding.

## Files changed

- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/expected/README.md`
- `scripts/README.md`
- `scripts/practical_alpha1_export_devtools.py`
- `scripts/tests/test_practical_alpha1_export_devtools.py`
- `samples/practical-alpha1/expected/vis-a1-03-membership-timeline.expected.json`
- `docs/reports/1171-p-a1-13-practical-membership-timeline-export.md`

## Commands run

```bash
date '+%Y-%m-%d %H:%M %Z'
python3 -m unittest scripts.tests.test_practical_alpha1_export_devtools
python3 scripts/practical_alpha1_save_load.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py check-all --format json
python3 scripts/practical_alpha1_export_devtools.py closeout --format json
python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-03 --format json
python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_export_devtools
python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- RED was observed first: `python3 -m unittest scripts.tests.test_practical_alpha1_export_devtools` failed before implementation because `VIS-A1-03` was unknown and closeout expectations still deferred it.
- GREEN focused helper floor passed after implementation:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_export_devtools`
  - `python3 scripts/practical_alpha1_save_load.py check-all --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py check-all --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py closeout --format json`
  - `python3 scripts/practical_alpha1_export_devtools.py render-html VIS-A1-03 --format json`
  - `python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_export_devtools`
- Final package floor passed after snapshot/report sync:
  - `python3 -m unittest scripts.tests.test_practical_alpha1_save_load scripts.tests.test_practical_alpha1_export_devtools scripts.tests.test_validate_docs`
  - `python3 scripts/check_source_hierarchy.py`
  - `python3 scripts/validate_docs.py`
  - `cargo fmt --check`
  - `git diff --check`
- The widened devtools floor now actualizes `VIS-A1-01/02/03/04/05/06`, with `VIS-A1-07` remaining deferred.

## What changed in understanding

- `VIS-A1-03` did not require a new carrier family. The exact `SL-A1-02` save/load report already carried the precise membership frontier data needed for a thin export widening.
- The honest boundary is not “membership timeline complete”; it is “membership timeline export over one exact local save/load evidence row”.
- After `P-A1-13`, the remaining practical devtools gap is narrower and cleaner: `VIS-A1-07` only.

## Open questions

- Can `VIS-A1-07` be widened from exact existing practical carriers without synthesizing retention/on-demand semantics?
- If not, should the next package be a blocker split / docs-first package rather than another implementation widening?

## Suggested next prompt

Proceed autonomously from `P-A1-13` and determine whether `VIS-A1-07` can be closed honestly from exact existing practical carriers; if not, record the blocker precisely and stop only at a true user-decision boundary.

## Plan update status

`plan/` 更新済み:
`plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` を `P-A1-13` latest closeout, `VIS-A1-03` actualization, and `VIS-A1-07` only remaining devtools observable へ同期した。

## Documentation.md update status

`Documentation.md` 更新済み:
current practical closeout を `P-A1-13` に進め、`VIS-A1-03` が exact `SL-A1-02` membership evidence over a devtools bundle only であることを明記した。

## progress.md update status

`progress.md` 更新済み:
`PA1-6` を 85% へ更新し、`P-A1-13` current status / validation floor / recent log / next reopen point を同期した。

## tasks.md update status

`tasks.md` 更新済み:
last closed package, practical package map, ordered current work, and autonomous package inventory を `P-A1-13` 前提へ更新した。

## samples_progress.md update status

`samples_progress.md` 更新済み:
`PA1-6` row, `VIS-A1-*` row, top practical snapshot, `PH0`, and recent validation table を `P-A1-13` closeout に同期した。

## Reviewer findings and follow-up

- Local focused review only in this package.
- No sub-agent review was launched in this closeout. Current package was completed by direct diff review plus fresh validation on the focused save/load/devtools/doc floor.
- Follow-up retained:
  - keep `VIS-A1-07` blocked unless exact existing practical carriers are sufficient;
  - do not reinterpret `VIS-A1-03` as distributed durable membership semantics.

## Skipped validations and reasons

- Broader Cargo practical floors were not rerun because `P-A1-13` changed Python export assembly, expected JSON, and snapshot/docs wording only; no Rust source changed in this package.
- No transport, hot-plug, avatar, or product-preview focused suites were rerun because this widening depended only on exact `SL-A1-02` save/load evidence and the shared devtools helper path.

## Commit / push status

Pending at report write.

## Sub-agent session close status

No sub-agent sessions were opened in this package.
