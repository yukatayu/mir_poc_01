# 2089 — P-POSE-03 runtime PoseGraph

## Objective

Close `P-POSE-03 runtime PoseGraph` by actualizing a bounded source-first PoseGraph runtime lane, adding positive and negative runtime evidence rows, synchronizing snapshot/docs, and recording package-close validation.

## Scope and assumptions

- Scope is limited to `P-POSE-03` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- The semantic owner remains the source-first Full System V1 lane. Product Alpha helper PoseGraph rows are retained as comparison evidence and are not reinterpreted as runtime ownership.
- This package implements a bounded local PoseGraph runtime only:
  - Transform / PoseVersion / AnchorBinding runtime state
  - same-client same-observation-snapshot no-split-frame acceptance
  - split-frame violation export
  - stale-anchor membership rejection
  - anchor-switch frontier monotonicity rejection
  - fallback-only reacquire requirement
- This package does not claim pose-aware save/load completion, final devtools panels, renderer ownership, Unity/Unreal/VRM compatibility, WAN/federation, distributed durable save/load, final ABI/SDK, or final public grammar.
- Where semantics were still open, the implementation took the narrow side: unsupported or unproven behavior remains explicit rejection or planned-only inventory.

## Start state / dirty state

- Branch: `main`
- Start point: after `P-MIR-04` closeout (`5276bfeb`) had been committed and pushed
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-POSE-03` runtime, sample, and doc edits
  - those edits were treated as package work and were not reverted
- During package validation, `cargo fmt --check` first failed on new PoseGraph runtime files. `cargo fmt` was run and the package remained open until formatting drift was cleared.

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

1. Added `crates/mir-runtime::posegraph_runtime` as a bounded runtime surface that loads `posegraph-runtime-package-v0`, builds PoseGraph runtime state, and returns accepted / violation-export / runtime-rejection reports.
2. Implemented runtime checks for the current floor:
   - same-client same-observation-snapshot no-split-frame acceptance
   - split-frame `no_split_frame` violation export
   - stale anchor membership epoch rejection
   - stale anchor-switch owner/frontier rejection
   - fallback-only explicit reacquire requirement
3. Added a runnable example surface in `crates/mir-runtime/examples/posegraph_runtime_session.rs` so the runtime package path can be executed and rendered as JSON/pretty output.
4. Added integration tests in `crates/mir-runtime/tests/posegraph_runtime.rs` covering accepted no-split-frame, split-frame violation export, stale-anchor rejection, anchor-switch frontier regression rejection, and fallback-only reacquire-required rejection.
5. Added the source-first sample root `samples/full-system-v1/avatar-pose/` with:
   - 8 executable rows
   - 1 planned save/load row reserved for `P-POSE-04`
   - representative `.mir` files
   - runtime package inputs
   - expected projected JSON
   - row-level READMEs
6. Added `scripts/posegraph_runtime_samples.py` and `scripts/tests/test_posegraph_runtime_samples.py` to validate matrix consistency and runtime outputs over the new root.
7. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, and relevant plan/docs summary files so `P-POSE-03` is closed and `P-POSE-04 pose save/devtools` is promoted.
8. Preserved the earlier product-alpha helper PoseGraph line as a separate alpha comparison floor rather than silently collapsing it into the runtime lane.
9. Requested a focused package-close reviewer and used the findings to drive a red-green fix cycle before package close.
10. Added failing regression coverage for:
   - non-monotone anchor-switch log ordering
   - anchor-switch/runtime frontier mismatch
   - stale anchor-switch membership epoch
   - fallback-only missing-witness reacquire rejection
   - helper `closeout` planned-row drift
11. Fixed the runtime to validate every anchor-switch row against current membership epoch, current owner epoch, previous sequence frontier, and resolved runtime frontier, and tightened fallback-only reacquire handling.
12. Rebased `pose-07`, `pose-08`, and `pose-09` sample inputs so the source-first sample root covers the reviewer-found stale switch membership, internal switch ordering regression, and missing-witness reacquire cases.
13. Regenerated the affected expected JSON projections and reran package-specific validation, docs/source validators, and major anchors after the fixes.

## Files changed

- Rust source/tests:
  - `crates/mir-runtime/src/lib.rs`
  - `crates/mir-runtime/src/posegraph_runtime.rs`
  - `crates/mir-runtime/examples/posegraph_runtime_session.rs`
  - `crates/mir-runtime/tests/posegraph_runtime.rs`
- Scripts/tests:
  - `scripts/posegraph_runtime_samples.py`
  - `scripts/tests/test_posegraph_runtime_samples.py`
- Full System V1 PoseGraph samples:
  - `samples/full-system-v1/avatar-pose/README.md`
  - `samples/full-system-v1/avatar-pose/matrix.json`
  - sample roots under `samples/full-system-v1/avatar-pose/` for `pose-01..09`
- Snapshot/docs/repository memory:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `samples/full-system-v1/README.md`
  - `samples/product-alpha1/posegraph/README.md`
  - `scripts/README.md`
  - `plan/58-full-system-v1-roadmap.md`
  - `plan/61-posegraph-runtime-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py closeout --format json
python3 scripts/posegraph_samples.py check-all --format json
cargo fmt --check
cargo fmt
cargo test -p mir-runtime --test posegraph_runtime -- --nocapture
python3 -m unittest scripts.tests.test_posegraph_runtime_samples
python3 scripts/posegraph_runtime_samples.py run pose-07-stale-anchor-after-membership-advance --format json
python3 scripts/posegraph_runtime_samples.py run pose-08-anchor-switch-frontier-negative --format json
python3 scripts/posegraph_runtime_samples.py run pose-09-stale-anchor-reacquire-required --format json
python3 scripts/posegraph_runtime_samples.py check-all --format json
python3 scripts/posegraph_runtime_samples.py closeout --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"
python3 scripts/operational_product_samples.py check-all --format json
```

## Evidence / outputs / test results

- Package tests after implementation:
  - red phase:
    - `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`: failed 4 new tests
    - `python3 -m unittest scripts.tests.test_posegraph_runtime_samples`: failed 1 new test
  - green phase:
    - `cargo test -p mir-runtime --test posegraph_runtime -- --nocapture`: passed, 8 tests
    - `python3 -m unittest scripts.tests.test_posegraph_runtime_samples`: passed, 7 tests
- Source-first helper:
  - `python3 scripts/posegraph_runtime_samples.py check-all --format json`: passed 8 executable rows and preserved 1 planned row
    - accepted: `pose-01`, `pose-02`, `pose-03`, `pose-04`
    - violation export: `pose-05`
    - runtime rejection: `pose-07`, `pose-08`, `pose-09`
  - `python3 scripts/posegraph_runtime_samples.py closeout --format json`: passed and now reports only `pose-06-save-load-roundtrip` as planned
- Existing helper floor:
  - `python3 scripts/posegraph_samples.py check-all --format json`: accepted, preserved 1 helper accepted row, 1 helper violation row, and 7 planned rows
- Formatting:
  - `cargo fmt --check`: initially failed on new runtime files
  - `cargo fmt`: applied
  - final `cargo fmt --check`: passed
- Doc/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out "$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX)"`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted

## What changed in understanding

- The useful first runtime PoseGraph floor is narrower than a full spatial runtime. It is enough to keep same-snapshot coherence, explicit stale-anchor rejection, and explicit fallback/reacquire state without claiming renderer ownership or final world sync semantics.
- Source-first runtime PoseGraph and product-alpha helper PoseGraph should remain separate evidence lines. The helper line is still useful as an alpha comparison floor, but it is not the same thing as runtime ownership.
- Runtime reports need observer-safe summaries and projected state rather than raw internal structures. The current helper script can compare stable projected JSON without freezing a final public devtools ABI.
- `P-POSE-04` is the right place for pose-aware save/load and panel work. Folding those into `P-POSE-03` would have blurred the runtime/state boundary and over-claimed completion.

## Open questions

- `P-POSE-04` still needs to decide the exact pose save/load admissibility projection and the minimum observer-safe PoseGraph panel family.
- The current runtime package JSON schema is package-local evidence, not a frozen public grammar or ABI.
- Renderer/backend integration remains later work and must continue to treat pose snapshots as provider input rather than semantic ownership.

## Suggested next prompt

```text
P-POSE-04 pose save/devtools
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/61-posegraph-runtime-roadmap.md`

## Documentation.md update status

Updated for `P-POSE-03` closeout and `P-POSE-04` promotion.

## progress.md update status

Updated to show:

- `P-POSE-03` closed
- current promoted package `P-POSE-04`
- next promoted package after current closeout `P-PROJ-02`
- recent log entry for the PoseGraph runtime closeout

## tasks.md update status

Updated to promote `P-POSE-04 pose save/devtools` as the current package and `P-PROJ-02 projection IR realization` as the next promoted closeout target.

## samples_progress.md update status

Updated to mark `samples/full-system-v1/avatar-pose/` as evidence-closed bounded runtime PoseGraph evidence and to append the `P-POSE-03` closeout log entry.

## Reviewer findings and follow-up

- Focused reviewer `Rawls` completed a read-only package-close review and reported five issues:
  - missing anchor-switch log ordering/frontier coherence enforcement
  - fallback-only missing-witness reacquire bypass
  - missing stale anchor-switch membership rejection
  - helper `closeout` planned-row drift
  - missing package report artifact at the time of review
- Follow-up completed in this package:
  - wrote failing Rust and Python regression tests first
  - tightened `crates/mir-runtime::posegraph_runtime` to validate anchor-switch rows against sequence, frontier, owner epoch, and membership epoch
  - tightened fallback-only handling so missing-witness rows reject with `reacquire_required`
  - fixed `scripts/posegraph_runtime_samples.py closeout`
  - added the package report artifact and reran validation
- No reviewer findings remain open at package close.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending until package-close commit and push are executed.

## Sub-agent session close status

- Reviewer `Rawls` (`019e4dc9-0db7-72d0-a0b1-b0847f645a87`) completed and was closed after findings were integrated.
