# 2090 — P-POSE-04 pose save/devtools

## Objective

Close `P-POSE-04 pose save/devtools` by actualizing bounded PoseGraph save/load admissibility and observer-safe devtools export, synchronizing the source-first sample root, updating snapshot docs, and recording package-close validation.

## Scope and assumptions

- Scope is limited to `P-POSE-04` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- The semantic owner remains the Full System V1 source-first lane. Product Alpha helper PoseGraph rows remain comparison evidence only.
- This package widens the existing runtime PoseGraph lane with:
  - accepted save/load roundtrip evidence
  - negative load-inadmissibility evidence
  - observer-safe PoseGraph/devtools export
- This package does not claim distributed durable pose save/load, final devtools/viewer ABI, renderer semantic ownership, Unity/Unreal/VRM compatibility, WAN/federation, final ABI/SDK, or final public grammar.
- Where the spec allowed `reject or force explicit reacquire`, the implementation took the narrow side for save/load mismatches:
  - fallback-only state still uses `reacquire_required`
  - other save/load mismatches reject as `save_load_inadmissible`

## Start state / dirty state

- Branch: `main`
- Start point: after `P-POSE-03` closeout (`9f39eb53`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-POSE-04` runtime/test/sample work
  - those edits were treated as package work and were not reverted
- During the package, two reviewer-found semantic gaps kept the package open:
  - accepted-path save/load mismatches were being reported as admissible
  - split-frame `ViolationExport` incorrectly poisoned save/load admissibility

## Documents consulted

- Core repo docs:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
- Full System V1 specs:
  - `specs/33-full-system-v1-scope.md`
  - `specs/34-textual-mir-alpha-grammar.md`
  - `specs/35-mir-typed-ir-and-interpreter.md`
  - `specs/36-projection-ir-and-boundary-preservation.md`
  - `specs/37-posegraph-runtime-semantics.md`
  - `specs/38-engine-provider-admission.md`
- Full System V1 plans:
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/59-textual-mir-roadmap.md`
  - `plan/60-computational-runtime-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `plan/62-projection-backend-roadmap.md`
  - `plan/63-engine-provider-roadmap.md`
- Handoff package:
  - `sub-agent-pro/full-system-completion-001/*.md`
- Additional policy/status doc required by `AGENTS.md` for roadmap/status work:
  - `.docs/progress-task-axes.md`

## Actions taken

1. Widened `crates/mir-runtime::posegraph_runtime` with `save_load_state` and `devtools_export` so runtime reports now carry bounded pose save/load and observer-safe PoseGraph/devtools evidence.
2. Added `PoseGraphSaveLoadState` and `PoseGraphDevtoolsExport` carriers to runtime reports and surfaced them through the existing `posegraph_runtime_session` example and helper projection.
3. Added bounded save/load/devtools runtime logic:
   - accepted save/load roundtrip reports
   - mismatch collection for saved/restored snapshot, membership epoch, owner epoch, anchor-switch sequence, anchor witness, and active anchor
   - explicit `save_load_inadmissible` runtime rejection when accepted-path save/load state is not admissible
   - preserved `reacquire_required` for fallback-only state
   - separated split-frame `ViolationExport` from save/load admissibility so no-split-frame violations do not silently poison save/load state
4. Added and tightened Rust regression coverage in `crates/mir-runtime/tests/posegraph_runtime.rs` for:
   - accepted save/load roundtrip
   - save/load membership mismatch rejection
   - stale anchor-witness rejection
   - split-frame violation that preserves save/load admissibility
5. Actualized the previously planned `pose-06-save-load-roundtrip` row under `samples/full-system-v1/avatar-pose/` and made all 9 rows executable.
6. Added `save_load` carriers to the negative runtime package rows so `pose-07..09` now also expose bounded load-inadmissibility state alongside their existing runtime rejection reasons.
7. Extended `scripts/posegraph_runtime_samples.py` projection with save/load/devtools summary fields and regenerated every executable `expected/run.json` under `samples/full-system-v1/avatar-pose/`.
8. Updated the helper/unit test expectations to match the fully executable 9-row matrix and the new projection surface.
9. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, and relevant plan/docs summary files so `P-POSE-04` is closed and `P-PROJ-02 projection IR realization` is promoted.
10. Requested a focused reviewer, integrated the findings in two iterations, reran package-specific validation, and then reran docs/source validators plus the existing major anchors after the final fixes.

## Files changed

- Rust source/tests:
  - `crates/mir-runtime/src/posegraph_runtime.rs`
  - `crates/mir-runtime/tests/posegraph_runtime.rs`
- Scripts/tests:
  - `scripts/posegraph_runtime_samples.py`
  - `scripts/tests/test_posegraph_runtime_samples.py`
- Full System V1 PoseGraph samples:
  - `samples/full-system-v1/avatar-pose/README.md`
  - `samples/full-system-v1/avatar-pose/matrix.json`
  - `samples/full-system-v1/avatar-pose/save-load-roundtrip/package.mir.json`
  - `samples/full-system-v1/avatar-pose/save-load-roundtrip/expected/run.json`
  - updated package/expected files for the other executable `avatar-pose` rows
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/full-system-v1/README.md`
  - `scripts/README.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
python3 - <<'PY' ... posegraph_runtime_samples direct projection inspection for pose-06..09 ... PY
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 scripts/posegraph_runtime_samples.py matrix --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
cargo fmt
cargo fmt --check
git diff --check
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"
python3 scripts/operational_product_samples.py check-all --format json
```

## Evidence / outputs / test results

- Package-specific runtime/test evidence:
  - `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`: passed, 11 tests
  - `python3 -m unittest scripts.tests.test_posegraph_runtime_samples`: passed, 8 tests
  - `python3 scripts/posegraph_runtime_samples.py matrix --format json`: passed, 9 executable rows, 5 accepted, 1 violation-export, 3 runtime rejection
  - `python3 scripts/posegraph_runtime_samples.py check-all --format json`: passed, all 9 executable rows matched expected projections
- Existing helper floor:
  - `python3 scripts/posegraph_samples.py check-all --format json`: accepted, preserved 1 helper accepted row, 1 helper violation row, and 7 planned rows
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed after formatting
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"`: accepted
    - final accepted out dir: `/tmp/mirrorea-alpha1-release-wyddOX`
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted
- Reviewer recheck evidence:
  - final reviewer pass confirmed no remaining findings
  - mixed split-frame plus save/load repro now stays `ViolationExport` while preserving `save_load_state.load_admissible = true`

## What changed in understanding

- Save/load admissibility is a distinct judgment from runtime acceptance. A runtime report can be non-accepted for one reason while still carrying an admissible save/load relation.
- Split-frame `ViolationExport` and save/load inadmissibility must stay separate. Collapsing them hides which invariant actually failed.
- For the current narrow floor, generic save/load mismatches are safest as explicit `save_load_inadmissible` rejections, while fallback-only restoration still remains an explicit `reacquire_required` path.
- The current helper projection is stable enough for bounded regression evidence without freezing a final public devtools or save/load ABI.

## Open questions

- No blocker remains for `P-PROJ-02`.
- Wider policy around `reject` vs `explicit reacquire` for additional pose save/load subcases can stay deferred until a later projection/runtime package actually needs finer distinction.
- The current runtime package JSON remains package-local alpha evidence, not a frozen public grammar or ABI.

## Suggested next prompt

```text
P-PROJ-02 projection IR realization
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`

## Documentation.md update status

Updated for `P-POSE-04` closeout and `P-PROJ-02` promotion.

## progress.md update status

Updated to show:

- `P-POSE-04` closed
- current promoted package `P-PROJ-02`
- next promoted package after current closeout `P-PROJ-03`
- recent log entry for the pose save/devtools closeout

## tasks.md update status

Updated to promote `P-PROJ-02 projection IR realization` as the current package and `P-PROJ-03 boundary schemas` as the next promoted closeout target.

## samples_progress.md update status

Updated to mark `samples/full-system-v1/avatar-pose/` as evidence-closed bounded runtime PoseGraph plus pose save/devtools evidence and to append the `P-POSE-04` closeout log entry.

## Reviewer findings and follow-up

- Focused reviewer `Pauli` completed a read-only package-close review and first reported one high-severity and one medium-severity issue:
  - accepted-path save/load mismatches were still reported as admissible
  - direct coverage for save/load mismatch cases was missing
- Follow-up completed in the same package:
  - rejected accepted-path save/load mismatches as `save_load_inadmissible`
  - added direct Rust coverage for membership mismatch and stale anchor-witness mismatch
- On re-review, `Pauli` reported one remaining medium-severity issue:
  - split-frame `ViolationExport` still incorrectly forced `save_load_state.load_admissible = false`
- Final follow-up completed in the same package:
  - separated split-frame violation export from save/load admissibility
  - added mixed-case regression coverage that proves `ViolationExport` can preserve admissible save/load state
- Final reviewer pass: no findings remain in the reviewed file set.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending until package-close commit and push are executed.

## Sub-agent session close status

- Reviewer `Pauli` (`019e4de3-1da7-75e2-91ce-a762b1f8aeaf`) completed and remains available until explicit close after commit/push.
