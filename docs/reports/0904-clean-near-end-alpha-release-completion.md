# Report 0904 — clean near end alpha release completion

- Date: 2026-04-23 00:07 JST
- Author / agent: Codex completion lead / implementation lead / documentation integrator
- Scope: clean near-end active sample suite actualization, first finite-index strong typing layer, order/handoff reinterpretation, model-check second line, Lean-first proof skeleton, old sample migration, repo-local alpha-ready closeout
- Decision levels touched: L2 principal sample-suite / helper-surface / proof-skeleton closeout; no L0 invariant change

## 1. Objective

Complete the authoritative handoff in
`sub-agent-pro/codex_clean_mir_near_end_completion_with_new_samples_2026-04-22.md`
to the point where the repo has one active clean near-end sample suite, old
sample lanes are removed from the active path, the first strong typing layer is
implemented as a finite decidable index fragment, order/handoff and
memory-order reinterpretation samples are runnable, mutex-style examples live on
the model-check second line, Lean-first proof skeletons are repo-locally
verifiable, and docs/specs/plan/progress/tasks/report all describe that current
state consistently.

## 2. Scope and assumptions

- `specs/` remained normative. Handoff-guided organization changes were mirrored
  into `specs/00`, `specs/10`, `specs/11`, and `specs/12`; no silent override
  of existing norms was made.
- This task targeted repo-local alpha readiness, not final public language,
  final parser grammar, final public verifier contract, packaging, or app-level
  productization.
- Old prototype / rough-stimulus sample lines were preserved only as historical
  archive material under `samples/old/2026-04-22-pre-clean-near-end/` and
  `samples/lean/old/2026-04-22-pre-clean-near-end/`.
- Lean validation was treated as repo-local `lean` availability plus generated
  stub/foundation checks. This repository is not a Lake project at present.

## 3. Documents consulted

- Primary handoff:
  `sub-agent-pro/codex_clean_mir_near_end_completion_with_new_samples_2026-04-22.md`
- Core docs and snapshots:
  `README.md`, `Documentation.md`, `progress.md`, `tasks.md`
- Normative specs:
  `specs/00-document-map.md`, `specs/01-charter-and-decision-levels.md`,
  `specs/02-system-overview.md`, `specs/03-layer-model.md`,
  `specs/04-mir-core.md`, `specs/05-mirrorea-fabric.md`,
  `specs/07-typed-effects-wiring-platform.md`,
  `specs/09-invariants-and-constraints.md`, `specs/10-open-questions.md`,
  `specs/11-roadmap-and-workstreams.md`, `specs/12-decision-register.md`
- Repository memory:
  `plan/00-index.md`, `plan/01-status-at-a-glance.md`,
  `plan/03-decision-strengths-and-boundaries.md`,
  `plan/04-core-semantics-current-l2.md`, `plan/06-surface-notation-status.md`,
  `plan/07-parser-free-poc-stack.md`,
  `plan/08-representative-programs-and-fixtures.md`,
  `plan/09-helper-stack-and-responsibility-map.md`,
  `plan/10-roadmap-overall.md`, `plan/11-roadmap-near-term.md`,
  `plan/12-open-problems-and-risks.md`,
  `plan/13-heavy-future-workstreams.md`,
  `plan/14-glossary-and-boundary-rules.md`,
  `plan/16-shared-space-membership-and-example-boundary.md`,
  `plan/17-research-phases-and-autonomy-gates.md`,
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`,
  `plan/90-source-traceability.md`, `plan/91-maintenance-rules.md`
- FAQ and abstract material:
  `faq_006.md`, `faq_007.md`, `faq_008.md`, `faq_010.md`, `faq_011.md`,
  `docs/research_abstract/`
- Sample / code / script surfaces audited:
  `samples/`, `crates/`, `scripts/`, `docs/reports/`

## 4. Actions taken

### 4.1 Handoff file read

- Read the authoritative handoff and extracted the required active sample set,
  validation command set, progress percentages, non-negotiable design rules, and
  acceptance criteria for repo-local alpha readiness.

### 4.2 Old sample migration summary

- Inventoried stale sample references with `rg` across repo docs, scripts,
  crates, and samples.
- Moved pre-clean-near-end active sample directories to
  `samples/old/2026-04-22-pre-clean-near-end/`.
- Moved old Lean sample corpus to
  `samples/lean/old/2026-04-22-pre-clean-near-end/`.
- Moved old active research abstracts to
  `docs/research_abstract/old/2026-04-22-pre-clean-near-end/`.
- Rewrote active README / Documentation / plan / research abstract text so old
  paths remain only as archive or migration notes, not as current active lanes.

### 4.3 Clean sample suite implemented

- Added `samples/clean-near-end/README.md` and one active suite with sixteen
  samples:
  - typing: 5
  - order-handoff: 6
  - model-check: 3
  - modal: 2
- Added `samples/clean-near-end/00_index_theories.mir` as shared source-adjacent
  declaration anchor.
- Added a runtime-backed clean sample registry and runner in
  `crates/mir-runtime/src/clean_near_end.rs`.
- Added clean-suite binaries:
  `crates/mir-runtime/src/bin/mir-clean-near-end.rs` and
  `crates/mir-runtime/src/bin/mir-current-l2.rs`.
- Reworked `scripts/current_l2_guided_samples.py` into a compatibility wrapper
  and added `scripts/clean_near_end_samples.py` as the active suite entrypoint.

### 4.4 First strong typing finite-index layer implementation

- Implemented the first finite decidable index layer in
  `crates/mir-runtime/src/clean_near_end.rs` with the following runtime-visible
  entities:
  - `IndexTheoryDecl`
  - `IndexElementDecl`
  - `IndexOrderDecl`
  - `IndexLawDecl`
  - `FinitePreorder`
  - `FiniteLattice`
  - `FinitePowerset`
  - `RegionPreorder`
  - `CaptureSet`
  - `SimpleCostBound`
  - `Constraint`
  - `ConstraintKind`
  - `ConstraintSourceRef`
  - `ConstraintSolver`
  - `ConstraintSolverResult`
  - `ResidualObligation`
- Implemented first-layer constraint kinds:
  - `label_flow`
  - `authority_ge`
  - `capture_subset`
  - `region_outlives`
  - `cost_leq`
  - `effect_row_subset`
  - `requires_witness`
  - `requires_publication`
- Implemented solver triage as:
  - `valid` for solved finite constraints
  - `invalid` for definitely false finite constraints
  - `residual` for later-line obligations instead of pretending success

### 4.5 Authority / label / capture / lifetime / cost definitions

- Active clean samples now define authority and label hierarchies as user
  theories rather than magical predicates.
- Security labels are represented as a user-defined finite lattice.
- Authority is represented as a user-defined finite preorder.
- Capture is represented via finite capture-set inclusion.
- Lifetime/region is represented via a finite preorder.
- Cost is represented via a simple decidable pointwise resource bound.
- `declassify_authority(...)`, `observer_role(...)`, `fingerprint_bound(...)`,
  and `fingerprint_visible(...)` were left only in historical archive material.
  The active suite does not treat them as builtins.

### 4.6 Order / handoff and memory-order reinterpretation

- Added clean order/handoff relations:
  `program_order`, `dependency_order`, `publication_order`,
  `observation_order`, `witness_order`, `finalization_order`,
  `scoped_happens_before`.
- Kept low-level `memory_order_*` wording as backend/reference family only; the
  source-principal active suite does not use exact C++ names.
- Added valid and invalid handoff samples plus stage-block, delegated RNG, and
  auditable witness variants.

### 4.7 Model-check second-line and mutex samples

- Added `01_peterson_sc_pass`, `02_peterson_relaxed_counterexample`, and
  `03_broken_mutex_counterexample` as explicit model-check second-line samples.
- Kept mutex / visibility failure as model-check territory, not as first-layer
  finite-index typing.
- Added report shapes for pass/counterexample outcomes and counterexample traces.

### 4.8 Lean / theorem skeleton state

- Replaced the old sample-sync flow with an active clean near-end Lean sync in
  `scripts/current_l2_lean_sample_sync.py`.
- Generated and validated stubs under `samples/lean/clean-near-end/`.
- Preserved `samples/lean/foundations` as the proof-spine floor.
- Confirmed repo-local Lean validation through direct `lean` use in tests and
  sync flow. Lake packaging is still unavailable because the repository has no
  `lakefile.lean` or `lakefile.toml`.

### 4.9 Docs / specs / plan / progress / tasks updates

- Updated active overview docs:
  `README.md`, `Documentation.md`, `progress.md`, `tasks.md`
- Updated normative/source-trace synchronization:
  `specs/00-document-map.md`, `specs/10-open-questions.md`,
  `specs/11-roadmap-and-workstreams.md`, `specs/12-decision-register.md`
- Updated repository memory:
  `plan/01-status-at-a-glance.md`, `plan/06-surface-notation-status.md`,
  `plan/08-representative-programs-and-fixtures.md`,
  `plan/09-helper-stack-and-responsibility-map.md`,
  `plan/11-roadmap-near-term.md`, `plan/12-open-problems-and-risks.md`,
  `plan/13-heavy-future-workstreams.md`,
  `plan/16-shared-space-membership-and-example-boundary.md`,
  `plan/17-research-phases-and-autonomy-gates.md`,
  `plan/18-type-proof-modelcheck-and-ordering-research-program.md`,
  `plan/90-source-traceability.md`
- Updated research-facing docs:
  `docs/research_abstract/README.md`,
  `docs/research_abstract/clean_near_end_typing_01.md`,
  `docs/research_abstract/clean_near_end_order_model_01.md`,
  `docs/research_abstract/clean_near_end_lean_01.md`,
  `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

### 4.10 Files changed

- New files:
  - `crates/mir-runtime/src/clean_near_end.rs`
  - `crates/mir-runtime/src/bin/mir-clean-near-end.rs`
  - `crates/mir-runtime/src/bin/mir-current-l2.rs`
  - `crates/mir-runtime/tests/clean_near_end_samples.rs`
  - `scripts/clean_near_end_samples.py`
  - `samples/clean-near-end/**`
  - `samples/lean/clean-near-end/**`
  - `docs/research_abstract/clean_near_end_*.md`
  - this report
- Updated files:
  - active docs / specs / plan / progress / tasks listed above
  - `crates/mir-runtime/src/current_l2.rs`
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/src/lib.rs`
  - `scripts/current_l2_guided_samples.py`
  - `scripts/current_l2_lean_sample_sync.py`
  - `scripts/tests/test_current_l2_guided_samples.py`
  - `scripts/tests/test_current_l2_lean_sample_sync.py`
- Moved to archive:
  - `samples/prototype/current-l2-order-handoff/`
  - `samples/prototype/current-l2-typed-proof-model-check/`
  - `samples/prototype/current-l2-parser-companion/`
  - `samples/not_implemented/order-handoff/`
  - `samples/problem-bundles/`
  - `samples/lean/current-l2/`
  - old `docs/research_abstract/{static_analysis_01*,order_01*,lean_01*}`
- Deleted active-path-only compatibility files:
  - old prototype-facing runtime tests
  - old problem-bundle script tests
  - old theorem bundle example emitter
  - old sample-emitted-artifact support shim

## 5. Evidence / outputs / test results

### 5.1 Fresh validation commands

- `python3 scripts/current_l2_guided_samples.py list --format json`
  - exit 0
  - listed all 16 clean near-end samples from `samples/clean-near-end/`
- `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  - exit 0
  - typing family matched 1 valid + 4 static rejections
  - order-handoff family matched 4 valid + 2 static rejections
  - model-check family matched 1 pass + 2 counterexamples
  - modal family matched 2 valid
  - matrix reported `total_samples = 16`
- `python3 scripts/current_l2_guided_samples.py closeout --format json`
  - exit 0
  - `active_sample_root = samples/clean-near-end`
  - `archive_sample_root = samples/old/2026-04-22-pre-clean-near-end`
- `python3 scripts/clean_near_end_samples.py list --format json`
  - exit 0
  - listed the same 16 active clean samples
- `python3 scripts/clean_near_end_samples.py run typing --format json`
  - exit 0
  - `01_authorized_declassification` valid/success
  - `02_unauthorized_declassification_rejected` malformed `authority_preorder_constraint_failed`
  - `03_label_flow_rejected` malformed `label_flow_constraint_failed`
  - `04_capture_escape_rejected` malformed `capture_escape`
  - `05_cost_bound_rejected` malformed `cost_bound_exceeded`
- `python3 scripts/clean_near_end_samples.py run order-handoff --format json`
  - exit 0
  - valid: `01`, `04`, `05`, `06`
  - malformed: `02_missing_witness_rejected`, `03_handoff_before_publication_rejected`
- `python3 scripts/clean_near_end_samples.py run model-check --format json`
  - exit 0
  - `01_peterson_sc_pass` passed under `sequential_consistency`
  - `02_peterson_relaxed_counterexample` returned `counterexample`
  - `03_broken_mutex_counterexample` returned `counterexample`
- `python3 scripts/clean_near_end_samples.py run modal --format json`
  - exit 0
  - both modal samples valid
- `python3 scripts/clean_near_end_samples.py matrix --format json`
  - exit 0
  - families: typing `5`, order-handoff `6`, model-check `3`, modal `2`
- `python3 scripts/clean_near_end_samples.py closeout --format json`
  - exit 0
  - clean suite roots, vocabulary split, proof sample set, and Lean roots were
    reported as expected
- `cargo test -p mir-runtime`
  - exit 0
  - active clean suite tests passed
  - operational CLI tests passed
  - current-L2 fixed-subset and manifest tests passed
- `cargo test -p mir-ast`
  - exit 0
  - all tests passed
  - dead-code warnings remain in parser-spike support test helpers
- `cargo test -p mir-semantics`
  - exit 0
  - all tests passed
  - direct Lean theorem stub execution test passed under installed `lean`
  - dead-code warnings remain in detached-bundle/formal-hook support helpers
- `python3 -m unittest discover -s scripts/tests`
  - exit 0
  - `Ran 100 tests in 0.535s`
  - `OK`
- `python3 scripts/current_l2_lean_sample_sync.py`
  - exit 0
  - rewrote `samples/lean/manifest.json`
- `python3 scripts/validate_docs.py`
  - exit 0
  - `Documentation scaffold looks complete.`
  - `Found 902 numbered report(s).`
- `lake build`
  - exit 1
  - failure reason:
    `no configuration file with a supported extension`
  - interpretation: repo-level Lake packaging is not configured; this does not
    block repo-local Lean stub/foundation validation

### 5.2 Consistency scans

- Stale sample path scan after cleanup:
  `rg "samples/prototype|samples/not_implemented|p07-dice|p08-dice|p10-typed|p11-typed|p12-typed|p13-dice|p14-dice" README.md Documentation.md progress.md tasks.md plan docs/research_abstract scripts crates --glob '!docs/research_abstract/old/**' --glob '!samples/old/**' --glob '!samples/lean/old/**'`
  - remaining hits were only explicit migration notes in `README.md` and
    `Documentation.md`
- Magical predicate scan after cleanup:
  `rg "declassify_authority\\(|observer_role\\(|fingerprint_bound\\(|fingerprint_visible\\(" README.md Documentation.md progress.md tasks.md plan docs/research_abstract scripts crates --glob '!docs/research_abstract/old/**' --glob '!samples/old/**' --glob '!samples/lean/old/**'`
  - remaining hit was the explicit anti-builtin explanation in
    `Documentation.md`
- Active closeout evidence therefore matches the intended rule:
  old symbolic-predicate design is not described as the active design anywhere
  outside archive/historical explanations.

### 5.3 Why repo-local alpha-ready is achieved

- The clean near-end suite is the active sample corpus and is wired into active
  scripts, runtime entrypoints, tests, docs, and Lean sync.
- Old sample lanes were archived out of the active path.
- The first strong typing layer now uses finite user-defined theories for
  authority, labels, capture, lifetime, and simple cost.
- Order/handoff and modal bridge samples are active and validated.
- Mutex / weak-memory style examples are implemented as model-check second-line
  samples and validated.
- Lean-first proof skeletons and generated clean sample stubs are repo-locally
  validated.
- Docs/specs/plan/progress/tasks/report now describe the clean suite as current
  and old prototype material as historical.

## 6. What changed in understanding

- The repo no longer benefits from treating old prototype identifiers as the
  active representative set. Keeping them visible in active docs made the
  architecture look less settled than the current current-layer implementation
  actually is.
- The finite-index first layer is sufficient to make the current typing story
  concrete without pretending that full dependent type theory or theorem-prover
  integration is already the public checker surface.
- Mutex / visibility bugs fit better as model-check second-line evidence than as
  type-checker obligations. Encoding them that way clarifies the boundary
  between first-line finite static constraints and second-line execution/model
  exploration.
- Lean validation in this repository is presently a direct-tool repo-local path,
  not a Lake-packaged project boundary. That distinction matters for honest
  closeout wording.

## 7. Open questions

- Mixed gates still open:
  - final public parser grammar and public API
  - final public theorem / model-check / witness-provider contracts
  - concrete external theorem prover and model-check tool bindings
  - installed binary / packaging / distribution criteria
  - final shared-space operational catalog and public-shape completion
- True user-spec gates still open:
  - target distribution / packaging expectations
  - upper application target and guarantee envelope
  - final product-facing syntax and public API surface

## 8. Suggested next prompt

Take the new clean near-end alpha floor as fixed and execute the next package:
public-seam residual narrowing. Focus on parser/public API residuals, theorem /
model-check / witness-provider public contract shaping, and packaging criteria,
without reopening the archived prototype sample line.

## 9. Discord progress log summary

- `begin` baseline recorded before edits/commands.
- Progress checkpoints were sent at repository audit / implementation / doc-sync
  / validation milestones.
- Final `complete` notification is reserved for task stop, per repo policy.
