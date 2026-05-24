# Report 2103 — P-SURF-03 Surface-to-Core Elaboration

- Date: 2026-05-24 16:48 JST
- Author / agent: Codex
- Scope: `P-SURF-03 Surface-to-Core elaboration`
- Decision levels touched: L1/L2 Surface-to-Core elaboration evidence; no auto
  communication, runtime, role-admission, source patch activation, final public
  grammar, ABI, or SDK claim

## Objective

Implement the narrow Surface-to-Core elaboration evidence floor for canonical
Surface Mir `S { ... }` source. The package lowers cross-locus indexed
read/write access into explicit generated Core IR evidence: transitions, remote
request rows, generated edges, source spans, and obligations. It also rejects
underdeclared generated failure rows and unsupported Surface statements rather
than silently dropping semantics.

## Scope and assumptions

This package is elaboration/sample evidence only. It consumes `.mir` source as
semantic authority and keeps `package.mir.json` as an alpha artifact. It does
not claim MessageEnvelope generation, auto publish/observe completion, runtime
delivery, role admission capability grants, source-patch activation, final
grammar, ABI, or SDK completion.

The implemented request failure row is intentionally narrow:
`MissingCapability`, `MissingWitness`, `RouteUnavailable`, and
`StaleMembership`. Missing rows are rejected as
`generated_failure_not_declared`. Unsupported statements such as `join`,
`publish`, `grant`, `require`, and raw statements are rejected as
`unsupported_surface_statement_for_elaboration`; later packages own their
lowering.

## Start state / dirty state

The branch started this package from pushed `P-SURF-02` commit `42d89a32`.
The handoff directory `sub-agent-pro/surface-mir-brace-completion-001/`
remained present as untracked local directive material and was intentionally not
staged as normative source.

## Documents consulted

Consulted the required repo entry points and status documents: `README.md`,
`Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, Surface
specs `39..43`, Surface plans `64..68`, and the handoff package under
`sub-agent-pro/surface-mir-brace-completion-001/` including the sample matrix.
The P-SURF-01 parser and P-SURF-02 indexed-state checker implementations were
also consulted as source AST and state identity boundaries.

## Actions taken

- Added `crates/mir-semantics::surface_to_core_elaboration`.
- Added `surface_to_core_elaborate` example for JSON elaboration reports.
- Added `surface_to_core_elaboration` Rust tests.
- Collected indexed state declarations by owner-scoped `(owner_locus,
  state_name)`.
- Generated Core IR transition rows for Surface `when` blocks and generated
  remote requests.
- Generated remote read and write request rows for cross-locus indexed state
  access.
- Generated observe / remote-write edge rows linked to request ids.
- Preserved source spans for transitions, remote requests, and generated edges.
- Added accepted and residual obligation rows to the Core IR report.
- Rejected underdeclared generated failure rows for read and write requests.
- Rejected unsupported statements instead of silently accepting and dropping
  them.
- Added `ELAB-01/02/04/05/06/07/08` executable elaboration sample rows.
- Updated `surface_mir_samples.py`, `surface_mir_release_check.py`, validator
  inventories, focused Python tests, and snapshot docs.

## Files changed

Primary implementation:

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/surface_to_core_elaboration.rs`
- `crates/mir-semantics/examples/surface_to_core_elaborate.rs`
- `crates/mir-semantics/tests/surface_to_core_elaboration.rs`

Sample/helper/test surface:

- `samples/full-system-v1-surface/README.md`
- `samples/full-system-v1-surface/elaboration/**`
- `scripts/surface_mir_samples.py`
- `scripts/surface_mir_release_check.py`
- `scripts/tests/test_surface_mir_samples.py`
- `scripts/tests/test_surface_mir_release_check.py`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`

Documentation/status:

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `scripts/README.md`
- `docs/hands_on/surface_mir_alpha_01.md`
- `docs/research_abstract/surface_mir_alpha_01.md`
- `specs/00-document-map.md`
- `specs/39-surface-mir-placement-elaboration.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/64-surface-mir-placement-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

Focused implementation validation:

```bash
cargo fmt
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
```

Required package-close validation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
cargo test -p mir-ast --test surface_mir_parser -- --nocapture
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-03
```

Compatibility anchors:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-03-rerun
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

One first Product Alpha compatibility rerun used the already-populated
`/tmp/mirrorea-alpha1-release-p-surf-03` directory and returned
`preflight:output-dir-empty`; it was rerun with
`/tmp/mirrorea-alpha1-release-p-surf-03-rerun` and accepted.

## Evidence / outputs / test results

- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`:
  7 tests passed.
- `cargo test -p mir-ast --test surface_mir_parser -- --nocapture`: 13 tests
  passed.
- `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`:
  7 tests passed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`:
  23 tests passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: accepted,
  `sample_count: 21`, passed 21 rows, failed `[]`, `workflow_ready: false`.
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`:
  accepted, `source_count: 21`, diagnostics `[]`.
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-03`:
  `surface_mir_release_check_ready: true`, scope
  `p_surf_03_surface_to_core_elaboration`, failed commands `[]`, result count
  12.
- `python3 -m unittest scripts.tests.test_validate_docs`: 18 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 457, present 457,
  missing 0.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1254
  numbered reports found before this report.
- `cargo fmt --check`: passed.
- `git diff --check`: passed.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-03-rerun`:
  accepted, product alpha release candidate ready, failed commands `[]`, passed
  command count 29.
- `python3 scripts/operational_product_samples.py check-all --format json`:
  accepted, failed `[]`.
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`:
  accepted, failed `[]`, strict family count 4.

## What changed in understanding

Elaboration cannot treat unsupported statements as no-ops. Even in a narrow
alpha floor, accepting a `join` or `publish` statement without Core IR evidence
would overclaim semantic coverage and violate the no-silent-drop discipline. The
P-SURF-03 floor therefore rejects unsupported statements until P-SURF-04/P-SURF-05
lower them explicitly.

Nested foreign `S { ... }` blocks remain placement syntax, not ambient authority.
For both writes and reads, the evidence now records owner-directed generated
request rows instead of direct access.

## Open questions

- `ELAB-03` private-field auto-publish rejection remains P-SURF-04 work.
- MessageEnvelope, auto publish, auto observe, and generated communication
  failure row completeness remain P-SURF-04 work.
- Role admission and capability grant semantics remain P-SURF-05 work.
- Runtime source patch activation remains P-SURF-06 work.

## Suggested next prompt

Continue autonomously with `P-SURF-04 auto communication publish/observe`.

## Plan update status

`plan/00-index.md`, `plan/64-surface-mir-placement-roadmap.md`, and
`plan/68-surface-full-system-v1-roadmap.md` were updated. `plan/64` now records
P-SURF-03 as closed elaboration evidence and keeps `ELAB-03` private-field
communication rejection in P-SURF-04.

## Documentation.md update status

`Documentation.md` was updated to include the P-SURF-03 elaboration evidence
floor and to point the current promoted package to P-SURF-04.

## progress.md update status

`progress.md` was updated with the P-SURF-03 closeout, `ELAB-01/02/04/05/06/07/08`,
current runnable commands, non-claims, timestamp, and current package
`P-SURF-04 auto communication publish/observe`.

## tasks.md update status

`tasks.md` was rewritten as the current task map with P-SURF-03 closed and
P-SURF-04 first in the self-driven queue.

## samples_progress.md update status

`samples_progress.md` was updated in-place to track
`samples/full-system-v1-surface/elaboration/` as `ELAB-01/02/04/05/06/07/08`
elaboration evidence only, not communication/runtime workflow readiness.

## Reviewer findings and follow-up

Docs/status reviewer found that the closeout report was still missing while
snapshot docs already promoted P-SURF-04, and that the hands-on guide omitted
the Surface release check from the current validation floor. Follow-up: this
report was added, the hands-on guide now includes
`scripts/surface_mir_release_check.py --format json check-all`, and a small
`progress.md` typo was fixed.

Type/elaboration reviewer found that unsupported statements were silently
accepted and dropped, and that there was no sample/test guard for this. The
reviewer also noted missing write-side underdeclared-failure coverage and
nested-place read coverage. Follow-up: unsupported statements now reject with
`unsupported_surface_statement_for_elaboration`; `ELAB-06`, `ELAB-07`, and
`ELAB-08` plus Rust regression tests cover unsupported statement rejection,
write-side generated failure-row rejection, and nested-place read placement.

## Skipped validations and reasons

No requested validation is intentionally skipped. One Product Alpha compatibility
anchor first failed because the selected `/tmp` output directory was non-empty;
it was rerun with a fresh output directory and accepted.

## Commit / push status

Pending at report write. The package commit will use
`git commit --no-gpg-sign -m "p-surf-03: add surface to core elaboration"` and
will be pushed after final staged validation.

## Sub-agent session close status

Two reviewer sub-agent sessions were used and closed:

- `019e58fd-94ef-7783-89f8-2bea185fb6a2`: type/elaboration review; findings
  fixed.
- `019e58fd-95f6-78f2-87cc-653b4eded049`: docs/status review; findings fixed.
