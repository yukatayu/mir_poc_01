# 2092 — P-PROJ-03 boundary schemas

## Objective

Close `P-PROJ-03 boundary schemas` by actualizing bounded packet/FFI boundary-schema emission over the source-first projection lane, proving positive and negative rows, tightening contract-preservation rejection behavior, synchronizing roadmap/status docs, and recording package-close validation.

## Scope and assumptions

- Scope is limited to `P-PROJ-03` from `sub-agent-pro/full-system-completion-001/19-codex-package-sequence.md`.
- Semantic source of truth remains `.mir` source plus typed IR; `package.mir.json` remains alpha compatibility/package artifact only.
- This package is allowed to actualize:
  - packet/FFI schema emission
  - bounded schema-preservation rejection behavior
  - projection helper/runtime/CLI evidence
  - generated projection-artifact and rejection-report bundles
  - snapshot/roadmap promotion to `P-PROJ-04`
- This package does not claim final packet/FFI transport semantics, executable server/client split, provider admission, LLVM/native codegen, final public ABI/SDK, WAN/federation, distributed durable save/load R3/R4, Unity/Unreal/WASM/native provider execution, or arbitrary native package execution.
- Safe-side narrowing was used where preservation could not yet be encoded losslessly:
  - same-shape heterogeneous multi-effect boundaries now reject instead of unioning failure/capability rows into one schema
  - residual obligations distinguish emitted schemas from still-later transport/runtime semantics
  - stale legacy generated artifacts are rejected by the helper

## Start state / dirty state

- Branch: `main`
- Start point: after `P-PROJ-02` closeout (`42173558`) had been committed and pushed.
- Initial local state for this package was not clean:
  - the tree already contained in-scope `P-PROJ-03` source/test/sample/doc work
  - those edits were treated as package work and were not reverted
- During closeout, a projection/backend reviewer found a high-severity preservation gap plus two medium-severity closeout gaps, so package close was paused until those were resolved.

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

1. Extended `crates/mir-semantics::full_system_v1::projection` so projection reports now emit `packet_schemas` and `ffi_schemas` in addition to projection IR, target manifests, and preservation reports.
2. Added payload-shape extraction plus boundary-schema emission so packet/FFI bundles preserve:
  - effect names
  - failure row
  - capability row
  - witness requirements
  - authority policy
  - provider/observation/redaction/retention policy
  - rollback/replay/save-load obligation
3. Added a bounded negative rejection for mixed request/response payload shapes on one boundary: `boundary_payload_shape_mismatch`.
4. After reviewer findings, tightened schema-preservation semantics:
  - same-shape heterogeneous multi-effect boundaries with different failure/capability contracts now reject as `boundary_effect_contract_mismatch`
  - residual obligations now explicitly retain `packet_ffi_transport_semantics_deferred`
  - helper validation now rejects stale legacy generated artifacts
  - legacy `generated/target-manifest.json` was removed from the positive row
5. Added a new negative sample root:
  - `samples/full-system-v1/projection/effect-contract-mismatch-negative/`
6. Regenerated expected summaries and generated artifacts for the projection sample family so all four executable rows match the runtime/CLI helper outputs.
7. Extended Rust and Python regression coverage:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
  - `scripts/tests/test_projection_v1_samples.py`
8. Updated `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, sample/script READMEs, hands-on/summary docs, and relevant plan files so `P-PROJ-03` is closed and `P-PROJ-04 server/client local split` is promoted.

## Files changed

- Rust source/runtime exports:
  - `crates/mir-semantics/src/full_system_v1/projection.rs`
  - `crates/mir-semantics/src/full_system_v1/mod.rs`
  - `crates/mir-runtime/src/full_system_v1_projection.rs`
- Rust tests:
  - `crates/mir-runtime/tests/projection_ir.rs`
  - `crates/mirrorea-cli/tests/full_system_v1_cli.rs`
- Scripts/tests:
  - `scripts/projection_v1_samples.py`
  - `scripts/tests/test_projection_v1_samples.py`
- Samples:
  - `samples/full-system-v1/projection/README.md`
  - `samples/full-system-v1/projection/matrix.json`
  - `samples/full-system-v1/projection/effectful-sugoroku-positive/*`
  - `samples/full-system-v1/projection/client-write-authority-negative/*`
  - `samples/full-system-v1/projection/payload-shape-mismatch-negative/*`
  - `samples/full-system-v1/projection/effect-contract-mismatch-negative/*`
  - deleted `samples/full-system-v1/projection/effectful-sugoroku-positive/generated/target-manifest.json`
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
  - `plan/62-projection-backend-roadmap.md`
  - `docs/hands_on/full_system_v1_roadmap_01.md`
  - `docs/research_abstract/full_system_v1_roadmap_01.md`

## Commands run

```bash
git status --short
date '+%Y-%m-%d %H:%M JST'
df -h .
free -h
cargo test -p mir-runtime --test projection_ir -- --nocapture
cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture
python3 -m unittest scripts.tests.test_projection_v1_samples
python3 scripts/projection_v1_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
cargo fmt
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/minimal_alpha1_patterns.py check-all --format json
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release
out_dir=$(mktemp -d /tmp/mirrorea-alpha1-release-XXXXXX) && python3 scripts/product_alpha1_release_check.py --format json check-all --out "$out_dir"
git add <package files>
git commit --no-gpg-sign -m "P-PROJ-03: boundary schemas"
git push
```

## Evidence / outputs / test results

- Package-specific projection evidence:
  - `cargo test -p mir-runtime --test projection_ir -- --nocapture`: passed, 7 tests
  - `cargo test -p mirrorea-cli --test full_system_v1_cli -- --nocapture`: passed, 2 tests
  - `python3 -m unittest scripts.tests.test_projection_v1_samples`: passed, 9 tests
  - `python3 scripts/projection_v1_samples.py check-all --format json`: passed, 4 executable rows matched expected projections/artifacts
  - `python3 scripts/projection_boundary_samples.py check-all --format json`: accepted and preserved the older product-alpha inventory scaffold
- Docs/source validators:
  - `python3 -m unittest scripts.tests.test_validate_docs`: passed, 17 tests
  - `python3 scripts/check_source_hierarchy.py`: passed
  - `python3 scripts/validate_docs.py`: passed
  - `cargo fmt --check`: passed after formatting
  - `git diff --check`: passed
- Existing major anchors:
  - `python3 scripts/minimal_alpha1_patterns.py check-all --format json`: accepted
  - `python3 scripts/operational_product_samples.py check-all --format json`: accepted
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release`: first attempt failed preflight with `output_dir_not_empty`
  - `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-ax2v3x`: accepted after rerunning with a fresh temp output directory
- Resource snapshot used for the long-running package closeout:
  - `df -h .`: `/dev/vda2` 99G total, 58G used, 37G available, 62%
  - `free -h`: 960Mi total RAM, 601Mi used, 91Mi free, 17Gi swap available

## What changed in understanding

- Boundary-schema preservation is not satisfied by emitting one schema per boundary if that schema unions incompatible per-effect contracts. For the current bounded floor, heterogeneous same-shape effects must reject unless a lossless encoding exists.
- Residual obligations must distinguish "schema bundle emitted" from "transport/runtime semantics closed". Removing the residual entirely overstates package completion even when docs still intentionally stop short of final transport semantics.
- Generated sample artifacts are part of the contract surface for this package. Leaving legacy artifacts or untracked generated evidence in place weakens clean-clone reproducibility even if local helper output is green.

## Open questions

- No blocker remains for `P-PROJ-03`.
- `P-PROJ-04` still needs a bounded local role carrier choice inside the current safe scope:
  - one executable with explicit role flag
  - paired local processes from one manifest
  - Docker-pinned local role split
- Final packet/FFI transport semantics, provider admission, and broader deployment planning remain later packages.

## Suggested next prompt

```text
P-PROJ-04 server/client local split
```

## Plan update status

Updated:

- `plan/58-full-system-v1-roadmap.md`
- `plan/62-projection-backend-roadmap.md`

## Documentation.md update status

Updated for `P-PROJ-03` closeout, corrected projection-schema-floor wording, and promoted `P-PROJ-04 server/client local split`.

## progress.md update status

Updated to show:

- `P-PROJ-03` closed
- current promoted package `P-PROJ-04`
- next promoted package after current closeout `P-ENG-02`
- bounded projection IR now includes packet/FFI schema emission plus payload-shape/effect-contract negative rows

## tasks.md update status

Updated to keep `P-PROJ-04 server/client local split` as the current promoted package and to preserve `P-PROJ-03` closeout semantics in the package summary.

## samples_progress.md update status

Updated to keep `samples/full-system-v1/projection/` evidence-closed and to record the 4-row projection family plus stronger schema-preservation invariants in the status row and recent validation log.

## Reviewer findings and follow-up

- Projection/backend reviewer `Galileo` (`019e4e27-1be9-7232-bf55-aed807d61911`) completed a focused review and reported three issues:
  - same-shape heterogeneous multi-effect boundaries were incorrectly accepted and collapsed into one schema
  - residual obligations overstated schema completion
  - stale/untracked generated artifacts could drift unnoticed
- Follow-up completed in the same package:
  - added `boundary_effect_contract_mismatch`
  - restored an explicit transport-semantics residual obligation
  - added a new negative sample and new Rust/Python regression coverage
  - removed the stale legacy positive artifact and made the helper reject such drift
- Separate language/type, runtime/cut, engine/provider, and docs/status reviewers were not spawned for this package. Those perspectives were covered by local self-review against `specs/35`, `specs/36`, `specs/38`, the docs validators, and the major anchors.

## Skipped validations and reasons

- None. Required package-close validations were executed.

## Commit / push status

- Pending at report authoring time. This package is intended to be committed with:
  - `git commit --no-gpg-sign -m "P-PROJ-03: boundary schemas"`
  - `git push`

## Sub-agent session close status

- `Galileo` (`019e4e27-1be9-7232-bf55-aed807d61911`) completed, its findings were resolved locally, and the session was closed.
- No additional sub-agent sessions remain open for this package close.
