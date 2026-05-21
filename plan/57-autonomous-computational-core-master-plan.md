# plan/57 — autonomous computational core master plan

> **For agentic workers:** Use `superpowers:subagent-driven-development` for implementation packages when disjoint work can be delegated, or `superpowers:executing-plans` for inline package execution. Each package must close with report, validation, commit, and push.

## goal

Carry the repo from `P-COMP-00` recognition rebaseline through Mir-owned computation, PoseGraph, projection boundary, and engine-adapter boundary without stopping for user questions, while preserving Product Alpha-1 operational evidence and non-claims.

This plan is repository memory. It does not replace `progress.md` / `tasks.md` as the current queue snapshot, and it does not replace `specs/28..32` as normative source.

## architecture

The chain is a ratchet. Each package either adds bounded executable evidence or sharpens boundary docs so the next package can safely implement. Final-product decisions are isolated as `user-spec-required`; they do not block lower-layer work.

The first cut is a docs/scaffold front half. It fixes the sample matrix, PoseGraph terms, projection/provider compatibility, and engine-provider contract inventory before the runtime packages start making behavior claims.

## tech stack

- Rust crates: `mir-semantics`, `mir-ast`, `mir-runtime`, `mirrorea-cli`
- Python orchestration helpers under `scripts/`
- sample roots under `samples/product-alpha1/`
- normative specs under `specs/`
- repository memory under `plan/`
- reports under `docs/reports/`

## execution mode

Default execution mode after user approval:

```text
continue package-by-package
do not ask for intermediate choices
choose smallest safe default
record wider choices as kept_later or user_spec_required
validate before claims
commit and push after each package
```

## package chain

| Order | Package | Type | Output | Close condition |
|---:|---|---|---|---|
| 1 | `P-COMP-01` | scaffold | computational sample matrix and helper/test scaffold | planned vs executable rows are machine-readable and docs synced |
| 2 | `P-POSE-01` | scaffold | PoseGraph sample matrix and helper/test scaffold | no-split-frame / anchor / pose snapshot rows are classified |
| 3 | `P-PROJ-01` | boundary inventory | target manifest / packet / FFI inventory | helper/devtools report exists and is explicitly non-codegen |
| 4 | `P-ENG-01` | boundary inventory | provider classes and adapter contract rows | engine/WASM/native remain adapters, not semantic owners |
| 5 | front-half closeout | audit | docs/spec scaffold closure | planned rows and non-claims are synchronized before runtime claims |
| 6 | `P-COMP-02` | implementation | pure AddOne in Mir | host input / Mir compute / host output are distinct observable events |
| 7 | `P-COMP-03` | implementation | variables / arrays / records / control-flow first floor | each family has positive and negative evidence |
| 8 | `P-COMP-04` | implementation | effect boundary around internal computation | undeclared effect / failure / capability rows reject |
| 9 | `P-POSE-02` | implementation | no-split-frame positive and negative evidence | same snapshot accepted, split snapshot rejected or exported as violation |
| 10 | all-up closeout | audit | all-up docs / samples / validation report | all current rows synced, remaining non-claims explicit |

## current chain status

- all packages in this chain closed on 2026-05-21
- front-half scaffold closeout:
  `P-COMP-01`, `P-POSE-01`, `P-PROJ-01`, `P-ENG-01`
- implementation half closeout:
  `P-COMP-02`, `P-COMP-03`, `P-COMP-04`, `P-POSE-02`
- all-up closeout audit:
  focused helper suites, Cargo regressions, product alpha release check, installed-binary probe, operational suite, docs validator, and source hierarchy checks passed
- no promoted self-driven package remains in the current chain; later reopenings must be promoted explicitly from kept-later or user-spec-required lines

## P-COMP-01 plan

Current status:

- closed on 2026-05-21 as the planned-only computational scaffold actualization package
- the front-half queue is closed; later promoted reopen points are tracked in the implementation-half sections below

Purpose:

- actualize the computational sample scaffold that `specs/28` / `plan/53` already define.
- create the computational sample root without claiming runtime execution beyond scaffold rows.
- create a helper that can list / matrix / check planned rows.
- create tests proving the matrix classifies planned rows and does not mark them workflow-ready.

This package is not a second docs-only rebaseline. `P-COMP-00` already fixed the conceptual non-claim. `P-COMP-01` must create machine-readable planned-only surfaces and reject attempted runs as `planned_only`.

Expected files:

- create `samples/product-alpha1/computational/README.md`
- create planned sample roots under `samples/product-alpha1/computational/*`
- create `samples/product-alpha1/computational/matrix.json`
- create `scripts/mir_computational_samples.py`
- create `scripts/tests/test_mir_computational_samples.py`
- update `samples/README.md`, `scripts/README.md`, `samples_progress.md`, `tasks.md`, `progress.md`
- add report

Validation:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py matrix --format json
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Non-claim:

- no Mir-owned runtime execution yet.
- no final textual grammar.

Required negative rows:

- `run comp-02-pure-add-one` rejects as `planned_only` until `P-COMP-02`.
- missing planned root fails the matrix / check-all classification.

## P-COMP-02 plan

Purpose:

- make one AddOne path demonstrably Mir-owned.
- host input and output remain typed external adapter boundaries.

Current status:

- closed on 2026-05-21 with one executable product-alpha computational row
- legacy adapter-owned `typed_host_io.add_one` remains unchanged for demo / operational samples

Default implementation strategy:

- add a narrow computational-core module under `crates/mir-semantics` for pure expressions and functions.
- introduce `Expr`, `Stmt`, `FnDef`, typed values, lexical environments, and a pure typechecker/evaluator before runtime wrapping.
- use versioned `package.mir.json` as executable input.
- keep representative `.mir` explanatory only.
- add runtime/event evidence only after the arithmetic is owned by the Mir computational layer.

Expected files:

- add `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json`
- add expected JSON showing event sequence:
  `host_input_received`, `mir_compute_step`, `host_output_emitted`
- extend `scripts/mir_computational_samples.py run comp-02-pure-add-one`
- add Rust tests under `mir-semantics` for the pure computational module and focused runtime tests only for the event wrapper
- update docs and report

Validation:

```bash
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 scripts/mir_computational_samples.py run comp-02-pure-add-one --format json
cargo test -p mir-semantics mir_computational -- --nocapture
cargo test -p mir-runtime mir_computational -- --nocapture
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- JSON evidence separates host input, Mir compute, and host output.
- adapter does not own `x + 1`.
- adapter-owned transform paths are rejected as `not_mir_owned`.

Observed closeout:

- `samples/product-alpha1/computational/add-one-pure-mir/package.mir.json` is executable
- `scripts/mir_computational_samples.py run comp-02-pure-add-one --format json` returns accepted
- runtime/session/savepoint surfaces now preserve `mir_compute_history`
- `P-COMP-03` later widened the same core to first-floor positive / negative rows

Required negative rows:

- payload type mismatch.
- sample missing distinct `mir_compute_step` event.
- adapter-owned `AddOne` evidence rejected as `not_mir_owned`.

## P-COMP-03 plan

Current status:

- closed on 2026-05-21 with helper-executable first-floor widening and runtime/schema test coverage

Purpose:

- add first-floor C-like computation evidence.

Rows:

- `variables-scope-positive`
- `variables-scope-negative-shadow-or-use-before-declare`
- `arrays-bounds-positive`
- `arrays-bounds-negative`
- `records-vec3-positive`
- `records-vec3-negative-field`
- `control-flow-positive`
- `control-flow-negative-nontermination-or-type`
- `imports-functions-positive`
- `imports-functions-negative-missing-import`

Default implementation strategy:

- keep finite, explicit, parser-free or JSON-backed representation.
- choose runtime reject for dynamic bounds unless statically obvious.
- declare failure rows rather than creating implicit builtins.

Validation:

```bash
python3 scripts/mir_computational_samples.py check-all --format json
python3 -m unittest scripts.tests.test_mir_computational_samples
cargo test -p mir-semantics mir_computational -- --nocapture
cargo test -p mir-runtime mir_computational -- --nocapture
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- each feature family has at least one accepted and one rejected machine-readable row.

Observed closeout:

- `scripts/mir_computational_samples.py check-all --format json` now reports 6 accepted rows, 5 expected runtime rejection rows, and 1 planned-only row
- `crates/mir-semantics::computational_core` now covers variables / arrays / records / control-flow / imports
- product-alpha schema/runtime tests accept the widened computational registry and reject the negative rows with stable reasons
- next promoted package is `P-COMP-04`

## P-COMP-04 plan

Purpose:

- prove internal computation and effectful boundaries are separated.

Rows:

- accepted pure computation wrapped by declared host input/output.
- rejected undeclared host effect.
- rejected undeclared failure row.
- rejected missing capability.
- later accepted publish/observe/witness/handoff only when declared; do not widen to these before the host read/write split is stable.

Validation:

```bash
python3 scripts/mir_computational_samples.py check-all --format json
cargo test -p mir-semantics mir_computational -- --nocapture
cargo test -p mir-runtime mir_computational -- --nocapture
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- effect row, failure row, and capability rejects are visible.
- pure fragment does not contain `perform`, `publish`, `observe`, `witness`, `handoff`, or `atomic_cut`.

Observed closeout:

- closed on 2026-05-21 with one direct accepted host read/write row and three expected `check` rejections
- `scripts/mir_computational_samples.py check-all --format json` now reports 7 accepted rows, 5 expected runtime rejections, and 3 expected check rejections
- `samples/product-alpha1/computational/host-io-internal-transform/` now contains `positive/`, `negative-undeclared-effect/`, `negative-undeclared-failure/`, and `negative-missing-capability/`
- product-alpha schema/runtime tests now cover `required_capabilities` and `failure_tag` as declared admission-boundary evidence without claiming broad effectful runtime semantics
- next promoted package is `P-POSE-02`

## P-POSE-01 plan

Purpose:

- scaffold PoseGraph rows without runtime overclaim.

Expected files:

- create `samples/product-alpha1/posegraph/README.md`
- create planned roots and matrix JSON
- create `scripts/posegraph_samples.py`
- create `scripts/tests/test_posegraph_samples.py`
- update docs and dashboards

Rows:

- `avatar-head-transform`
- `anchored-object`
- `sparkle-fallback-anchor`
- `no-split-frame-positive`
- `split-frame-negative`
- `save-load-roundtrip`
- `stale-anchor-after-membership-advance`
- `anchor-switch-frontier-negative`
- `stale-anchor-reacquire-required`

Validation:

```bash
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/posegraph_samples.py matrix --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## P-POSE-02 plan

Current status:

- closed on 2026-05-21 with bounded helper-backed no-split-frame evidence
- current remaining package in this line is the all-up closeout audit

Purpose:

- add one positive and one negative no-split-frame evidence path.

Default implementation strategy:

- use same-session observation snapshot as conformance point.
- export `target_pose_version`, `anchored_pose_version`, and `pose_snapshot_ref`.
- for negative row, choose runtime reject or devtools violation row before model-checker work if that is smaller.

Validation:

```bash
python3 scripts/posegraph_samples.py run no-split-frame-positive --format json
python3 scripts/posegraph_samples.py run split-frame-negative --format json
python3 -m unittest scripts.tests.test_posegraph_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- positive row proves same snapshot version.
- negative row cannot be mistaken for accepted stable state.

Observed closeout:

- `scripts/posegraph_samples.py check-all --format json` now reports 1 accepted row, 1 `violation_export` row, and 7 planned rows
- `samples/product-alpha1/posegraph/no-split-frame-positive/package.mir.json` and `split-frame-negative/package.mir.json` provide helper-only executable input
- current no-split-frame reading remains `same_client_same_observation_snapshot`
- save/load, devtools panel family, anchor-switch, and stale-anchor reacquire remain later

## P-PROJ-01 plan

Purpose:

- add projection target / packet / FFI schema inventory without codegen claim.

Expected files:

- create `samples/product-alpha1/projection/README.md`
- create target manifest, packet schema, and FFI schema sample rows
- create `scripts/projection_boundary_samples.py`
- create tests
- update docs and dashboards

Validation:

```bash
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- inventory is visible in helper or devtools-style output.
- source contract to target manifest / packet schema / FFI schema / provider contract compatibility is visible with positive and negative rows.
- no generated server/client binary or backend execution is claimed.

## P-ENG-01 plan

Purpose:

- fix provider adapter contract rows for engine / WASM / native boundaries.

Expected files:

- create `samples/product-alpha1/engine-adapter/README.md` or equivalent inventory root if needed.
- create provider contract matrix.
- create `scripts/engine_adapter_boundary_samples.py`.
- create tests.
- update docs and dashboards.

Validation:

```bash
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- provider classes and contract fields are machine-readable.
- rollback / replay / cut-admissibility policy is machine-readable per provider.
- native / WASM execution remains disabled or inventory-only by default.
- world semantics remain in Mir / Mirrorea.
- bounded native / WASM provider admission remains `user_spec_required` unless an explicit later package supplies schema, effects, failures, capability, observation, sandbox, and rollback policy evidence.

## front-half closeout plan

Purpose:

- stop once after the four docs/scaffold front-half packages to prove the implementation half is not building on vague inventory.

Validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 -m unittest scripts.tests.test_posegraph_samples
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Close condition:

- all planned rows remain planned unless implemented.
- all stop lines are present before runtime evidence claims.
- `tasks.md` names `P-COMP-02` as next implementation package.
- final grammar, backend realization, bounded native/WASM admission, and final engine ABI remain user-spec-required or kept-later.

## closeout package plan

Purpose:

- audit that all packages remain synchronized and no stale active/planned wording remains.

Validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 -m unittest scripts.tests.test_mir_computational_samples
python3 -m unittest scripts.tests.test_posegraph_samples
python3 -m unittest scripts.tests.test_projection_boundary_samples
python3 -m unittest scripts.tests.test_engine_adapter_boundary_samples
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/mir_computational_samples.py check-all --format json
python3 scripts/posegraph_samples.py check-all --format json
python3 scripts/projection_boundary_samples.py check-all --format json
python3 scripts/engine_adapter_boundary_samples.py check-all --format json
cargo fmt --check
git diff --check
```

Product Alpha regression validation should be rerun if any shared product-alpha crates or CLI behavior changed.

## sub-agent review plan

For each implementation package, use:

- theory / invariant reviewer before closeout.
- runtime/toolchain reviewer for `P-COMP-02..04` and any CLI/helper change.
- PoseGraph reviewer for `P-POSE`.
- projection/backend reviewer for `P-PROJ` and `P-ENG`.
- docs/source hierarchy reviewer before commit.
- security/auth reviewer when effect, capability, native, or host boundary changes.

Wait for reviewers to complete. If a reviewer appears hung, retry once; if still unavailable, record skipped review and local focused review in the report.

## report / commit cadence

Each package:

```bash
git status --short
git add <changed files>
git commit --no-gpg-sign -m "mirrorea: <package summary>"
git push origin main
```

Each report must include validation results, reviewer findings, skipped validations, commit / push status, and sub-agent close status.

## self-review checklist

Before closing each package:

- planned-only roots are not marked workflow-ready.
- current `AddOne` adapter is not used as Mir-owned computation evidence.
- final grammar / ABI / backend / distribution non-claims remain explicit.
- positive and negative rows exist for behavior claims.
- docs/source hierarchy validators include new roots only when they exist.
- heavy artifacts are not created in repo root.
