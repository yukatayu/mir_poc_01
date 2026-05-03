# Report 1137 — P-A1-01 Alpha Source Front-Door

- Date: 2026-05-03
- Author / agent: Codex
- Scope: practical alpha-1 first source/package front-door, practical sample root initial cut, parser/loader tests, snapshot/validator sync
- Decision levels touched: L1, L2
- 日本語要約: `P-A1-01` では、practical alpha-1 の first front-door として `samples/practical-alpha1/` と `crates/mir-ast::practical_alpha1` を追加し、limited `package.mir.json` loader を actualize した。`SRC-01..05` の positive/negative parse/load tests は通るが、これは library-first の parse/load floor に留まり、final textual grammar、typed checker verdict、runtime execution、practical CLI/Docker command はまだ claim しない。

## Objective

`P-A1-01` を閉じ、practical alpha-1 が sample-ID keyed bridge ではなく source/package input から始まる最初の narrow front-door を持つ状態にする。

## Scope and assumptions

- `samples/alpha/` は alpha-0 evidence root のまま維持し、practical front-door root に昇格しない。
- front-door は final public grammar を凍結しない narrow cut に留める。
- `P-A1-01` の current implementation は library-first parse/load floor であり、typed checker/runtime execution には進まない。
- current practical front-door は limited `package.mir.json` family だけを扱う。
- practical CLI / Docker / save-load / devtools / product prototype は後続 package に残す。

## Start state / dirty state

- start state: `main` branch, worktree clean after `P-A1-00`
- resource preflight:
  - `df -h .`: `/dev/vda2` 99G total, 65G used, 30G available
  - `free -h`: 960Mi total, 403Mi available, swap 19Gi with 1.3Gi used
- prior repo state: `P-A1-00` had rebased `100%` semantics to practical alpha-1 readiness and promoted `P-A1-01` as the next package

## Documents consulted

- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/07-repository-structure.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/13-autonomous-package-sequence.md`
- `README.md`
- `Documentation.md`
- `AGENTS.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/41-save-load-checkpoint-roadmap.md`
- `plan/42-runtime-package-avatar-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `scripts/README.md`
- `docs/reports/1136-p-a1-00-practical-alpha-rebaseline.md`
- `docs/reports/TEMPLATE.md`

## Actions taken

1. Added `samples/practical-alpha1/` as a separate practical front-door root with `packages/`, `expected/`, `source/`, and `docker/` subroots plus README boundary docs.
2. Added initial practical fixtures:
   - `SRC-01` minimal world
   - `SRC-02` fallback chain
   - `SRC-03` layer attach
   - `SRC-04` layer/package manifest
   - `SRC-05` invalid syntax
3. Added `crates/mir-ast/src/practical_alpha1.rs` with:
   - `PracticalAlpha1FrontDoorManifest`
   - limited `package.mir.json` loader
   - directory-or-file entrypoint resolution
   - explicit `JsonParse` vs `SchemaDecode` error classification
   - narrow shape validation for format version and package kind
4. Added `crates/mir-ast/tests/practical_alpha1_front_door.rs` and support code, then performed RED→GREEN:
   - RED: unresolved `mir_ast::practical_alpha1` import and missing `serde`
   - GREEN: `SRC-01..05` plus the front-door manifest boundary passed after minimal implementation
5. Updated `README.md`, `Documentation.md`, `specs/18`, `plan/01`, `plan/44`, `samples/README.md`, `samples/alpha/README.md`, `scripts/README.md`, `progress.md`, `tasks.md`, and `samples_progress.md` so the practical lane now records a real front-door without overstating it as checker/runtime/product execution.
6. Extended source-hierarchy/docs validators to require the new practical root README and directory.
7. Incorporated reviewer findings by:
   - tightening the loader boundary so only literal `package.mir.json` entrypoints are accepted
   - widening positive fixture assertions across world/place/fallback/layer/manifest fields
   - adding negative expected coverage for `SRC-05` plus missing-package / unsupported-shape rejects
   - extending docs/source-hierarchy validators to require the practical subroot README set
8. Completed four parallel read-only reviews over semantics, runtime boundary, docs/progress consistency, and sample/validation integrity; closeout uses their concrete findings and does not treat silence as approval.

## Files changed

- `crates/mir-ast/Cargo.toml`
- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/tests/practical_alpha1_front_door.rs`
- `crates/mir-ast/tests/support/practical_alpha1_front_door_support.rs`
- `samples/practical-alpha1/README.md`
- `samples/practical-alpha1/packages/README.md`
- `samples/practical-alpha1/source/README.md`
- `samples/practical-alpha1/expected/README.md`
- `samples/practical-alpha1/docker/README.md`
- `samples/practical-alpha1/packages/src-01-minimal-world/package.mir.json`
- `samples/practical-alpha1/packages/src-02-fallback-chain/package.mir.json`
- `samples/practical-alpha1/packages/src-03-layer-debug/package.mir.json`
- `samples/practical-alpha1/packages/src-04-layer-manifest/package.mir.json`
- `samples/practical-alpha1/packages/src-05-invalid-syntax/package.mir.json`
- `samples/practical-alpha1/expected/src-01-minimal-world.expected.json`
- `samples/practical-alpha1/expected/src-02-fallback-chain.expected.json`
- `samples/practical-alpha1/expected/src-03-layer-debug.expected.json`
- `samples/practical-alpha1/expected/src-04-layer-manifest.expected.json`
- `samples/practical-alpha1/expected/src-05-invalid-syntax.expected.json`
- `README.md`
- `Documentation.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/01-status-at-a-glance.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/README.md`
- `samples/alpha/README.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/review-1137-review-p-a1-01-front-door-boundary.md`
- `docs/reports/review-1138-p-a1-01-sample-validation-integrity.md`
- `docs/reports/review-2026-05-03-pa1-01-front-door-overclaim-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
find docs/reports -maxdepth 1 -type f -name '*.md' | sort | tail -n 12
cargo test -p mir-ast practical_alpha1_front_door -- --nocapture
cargo test -p mir-ast
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt
cargo fmt --check
git diff --check
df -h .
free -h
date '+%Y-%m-%d %H:%M %Z'
```

## Evidence / outputs / test results

- RED step:
  - `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` first failed with unresolved `mir_ast::practical_alpha1` imports and missing `serde`.
- first GREEN step:
  - after adding the module and dependencies, `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` passed the initial front-door suite.
- second RED→GREEN step:
  - reviewer findings exposed that arbitrary existing JSON files were accepted as front-door input, wider than the documented `package.mir.json` boundary.
  - after tightening path resolution and adding a negative regression test, `cargo test -p mir-ast practical_alpha1_front_door -- --nocapture` passed `11` tests over `SRC-01..05`, manifest boundary, missing-package rejects, and unsupported-shape rejects.
- full crate validation:
  - `cargo test -p mir-ast` passed all `73` crate tests, including the new `practical_alpha1_front_door` suite.
- docs/validator floor:
  - `python3 -m unittest scripts.tests.test_validate_docs` passed `11` tests.
  - `python3 scripts/check_source_hierarchy.py` reported required/present/missing = `73/73/0`.
  - `python3 scripts/validate_docs.py` reported `Documentation scaffold looks complete.` and `Found 1139 numbered report(s).`
  - `cargo fmt --check` passed after one formatting rerun on `practical_alpha1_front_door_support.rs`.
  - `git diff --check` was clean.

## What changed in understanding

- A practical front-door does not require final textual `.mir` parsing to begin; a documented, tested, non-final `package.mir.json` cut is narrow enough to move the practical line forward honestly.
- The key honesty boundary is not “text vs JSON,” but whether the sample root is real, separate from `samples/alpha/`, and actually enters the toolchain by source/package input rather than sample ID.
- The first practical package can stay library-first. Practical CLI / runtime / Docker entrypoints belong to later packages once IR/checker/runtime plan surfaces exist.

## Open questions

- `P-A1-02` still needs the reusable typed IR/checker shape: whether the current package struct itself is the first IR, or whether a distinct lowered IR module should be introduced immediately.
- The current front-door validates shape, format version, and package kind only. More semantic obligations remain intentionally deferred to typed checker integration.

## Suggested next prompt

Continue with `P-A1-02 typed IR/checker integration` by defining the first reusable checked package report over the current `practical_alpha1` loader output, reusing alpha-0 LIF/VAR/CUT evidence without collapsing helper-local carriers into the public checker surface.

## Plan update status

`plan/` 更新済み: `plan/01-status-at-a-glance.md` と `plan/44-practical-alpha1-roadmap.md` に first practical front-door cut、separate practical root actualization、next promoted package `P-A1-02` を反映した。

## Documentation.md update status

`Documentation.md` 更新済み: practical alpha-1 line が `samples/practical-alpha1/` と `crates/mir-ast::practical_alpha1` の narrow front-door floorを持つこと、ただし checker/runtime/product execution ではないことを追記した。

## progress.md update status

`progress.md` 更新済み: `PA1-1` を close、`PA1-2` を next promoted package に更新し、practical alpha-1 overall reading を separate practical root + library-first front-door actualization へ同期した。

## tasks.md update status

`tasks.md` 更新済み: `P-A1-01` を closed、`P-A1-02` を current top work item として再配置し、practical root を future path ではなく current repo state に更新した。

## samples_progress.md update status

`samples_progress.md` 更新済み: `PA1-1` を closed に更新し、`SRC-01..05` front-door row、practical toolchain maturity update、`P-A1-01` recent validation row、timestamp refresh を追加した。

## Reviewer findings and follow-up

- `Bacon` (semantics/type-system):
  - found that the loader accepted arbitrary existing file paths, wider than the documented `package.mir.json` cut
  - follow-up: tightened `resolve_package_path()`, added `practical_alpha1_front_door_rejects_existing_non_package_json_file`, and kept the review artifact as non-numbered [review-2026-05-03-pa1-01-front-door-overclaim-review.md](/home/yukatayu/dev/mir_poc_01/docs/reports/review-2026-05-03-pa1-01-front-door-overclaim-review.md)
- `Feynman` (runtime/transport boundary):
  - independently confirmed the same basename-boundary drift between the implementation and the documented practical front-door contract
  - follow-up: shared the same loader-boundary fix and recorded the focused review in [review-1137-review-p-a1-01-front-door-boundary.md](/home/yukatayu/dev/mir_poc_01/docs/reports/review-1137-review-p-a1-01-front-door-boundary.md)
- `Hypatia` (docs/progress consistency):
  - found stale future-axis repository-memory pointers in `README.md` / `Documentation.md` and package-state drift in `plan/01-status-at-a-glance.md`
  - follow-up: added the `plan/44` pointer and made `P-A1-01 closed / P-A1-02 next` explicit in `plan/01`
- `Popper` (sample/validation integrity):
  - found lossy positive expectations, missing negative-branch coverage, overclaimed `SRC-01..05` expected coverage, and validator weakness around practical subroots
  - follow-up: widened fixture summaries, added `src-05-invalid-syntax.expected.json`, added missing-package / shape rejects, required practical subroot README files in validators, and kept the focused review in [review-1138-p-a1-01-sample-validation-integrity.md](/home/yukatayu/dev/mir_poc_01/docs/reports/review-1138-p-a1-01-sample-validation-integrity.md)
- all four reviewer sessions returned concrete findings, were integrated, and were then closed explicitly.

## Skipped validations and reasons

- broad runtime / transport / Docker tests are skipped because `P-A1-01` does not modify runtime execution, package attach, or transport codepaths; the new implementation scope is `mir-ast` loader-only.
- no alpha runtime, Docker, or save/load runner was rerun because this package does not change `mir-runtime`, `samples/alpha/`, or transport/save-load implementation surfaces. Those floors remain prerequisite evidence, not freshness requirements for this loader-only cut.

## Commit / push status

Pending at report write; validation floor is green and commit/push is the remaining closeout step.

## Sub-agent session close status

- `Bacon`: completed and closed
- `Feynman`: completed and closed
- `Hypatia`: completed and closed
- `Popper`: completed and closed
