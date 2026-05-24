# Report 2102 — P-SURF-02 Indexed State Semantics

- Date: 2026-05-24 16:14 JST
- Author / agent: Codex
- Scope: `P-SURF-02 indexed-state semantics`
- Decision levels touched: L1/L2 indexed-state checker evidence; no runtime,
  elaboration, role-admission, or final public grammar claim

## Objective

Implement the narrow Surface Mir indexed-state semantic checker floor for:

```mir
S {
  state player[p: Participant]: Player
}
```

The package fixes owner locus, Participant keyspace, value type metadata,
key-not-authority rejection, stale-key rejection, retained-savepoint compaction
rejection, nested-place ambient-authority rejection, executable sample rows,
docs/status updates, validation, and handoff to `P-SURF-03 Surface-to-Core
elaboration`.

## Scope and assumptions

This package is semantic checker/sample evidence only. It accepts `.mir` source
as semantic authority and keeps `package.mir.json` as an alpha artifact. It does
not claim Surface-to-Core elaboration, generated Core IR, runtime membership
carrier, role-admission capability grants, source-patch activation, final
grammar, ABI, or SDK completion.

Nested `S { ... }` blocks from a non-owner locus are rejected in this floor
because `P-SURF-03` must elaborate them into owner-directed generated requests.
Explicit capability-mediated remote writes are intentionally left as residual
P-SURF-03/P-SURF-05 work rather than being faked by a local marker.

## Start state / dirty state

The branch started this package from pushed `P-SURF-01` commit
`271b8146da1876ac93bcac3ffdec0c5ee8d805c6`. The handoff directory
`sub-agent-pro/surface-mir-brace-completion-001/` remained present as untracked
local directive material and was intentionally not staged as normative source.

## Documents consulted

Consulted the required repo entry points and status documents: `README.md`,
`Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, Surface
specs `39..43`, Surface plans `64..68`, and the handoff package under
`sub-agent-pro/surface-mir-brace-completion-001/` including sample blueprints.
The P-SURF-01 parser implementation and tests were also consulted as the source
AST boundary for this checker.

## Actions taken

- Added `crates/mir-semantics::surface_indexed_state`.
- Added `surface_indexed_state_check` example for JSON semantic reports.
- Added `indexed_state_semantics` Rust tests.
- Collected indexed state declarations as owner-scoped `(owner_locus,
  state_name)` entries.
- Recorded owner locus, state name, key name, Participant keyspace, value type,
  visible fields, and alpha authority model.
- Accepted owner-local writes while keeping `key_authority_granted = false`.
- Rejected direct role writes to S-owned indexed state with
  `indexed_state_key_is_not_authority`.
- Rejected stale-key writes after a `leave <key>` marker with
  `stale_indexed_state_key`.
- Rejected retained-savepoint compaction with
  `indexed_state_compaction_blocked_by_retained_evidence`.
- Rejected nested-place ambient-authority bypasses with
  `indexed_state_nested_place_requires_generated_request`.
- Added `IDX-01..05` executable indexed-state sample rows under
  `samples/full-system-v1-surface/indexed-state/`.
- Updated `surface_mir_samples.py`, `surface_mir_release_check.py`, validator
  inventories, focused Python tests, and snapshot docs.

## Files changed

Primary implementation:

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/src/surface_indexed_state.rs`
- `crates/mir-semantics/examples/surface_indexed_state_check.rs`
- `crates/mir-semantics/tests/indexed_state_semantics.rs`

Sample/helper/test surface:

- `samples/full-system-v1-surface/README.md`
- `samples/full-system-v1-surface/indexed-state/**`
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
- `specs/40-indexed-state-semantics.md`
- `specs/43-surface-mir-v1-alpha-scope.md`
- `plan/00-index.md`
- `plan/65-indexed-state-roadmap.md`
- `plan/68-surface-full-system-v1-roadmap.md`

## Commands run

Focused implementation validation:

```bash
cargo fmt
cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture
python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check
python3 scripts/surface_mir_samples.py check-all --format json
python3 scripts/surface_mir_authoring_check.py check-all --format json
```

Required package-close validation after report creation:

```bash
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-02
```

Compatibility anchors:

```bash
python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-02
python3 scripts/operational_product_samples.py check-all --format json
python3 scripts/minimal_alpha1_patterns.py check-all --format json
```

## Evidence / outputs / test results

- `cargo test -p mir-semantics --test indexed_state_semantics -- --nocapture`:
  7 tests passed.
- `python3 -m unittest scripts.tests.test_surface_mir_samples scripts.tests.test_surface_mir_release_check`:
  16 tests passed.
- `python3 scripts/surface_mir_samples.py check-all --format json`: accepted,
  `sample_count: 14`, passed `SURF-01..09` and `IDX-01..05`, failed `[]`,
  `workflow_ready: false`.
- `python3 scripts/surface_mir_authoring_check.py check-all --format json`:
  accepted, `source_count: 14`, diagnostics `[]`.
- `python3 -m unittest scripts.tests.test_validate_docs`: 18 tests passed.
- `python3 scripts/check_source_hierarchy.py`: required 434, present 434,
  missing 0.
- `python3 scripts/validate_docs.py`: documentation scaffold complete, 1254
  numbered reports found.
- `cargo fmt --check`: passed.
- `git diff --check`: passed.
- `python3 scripts/surface_mir_release_check.py --format json check-all --out /tmp/mirrorea-surface-release-p-surf-02`:
  `surface_mir_release_check_ready: true`, failed commands `[]`.
- `python3 scripts/product_alpha1_release_check.py --format json check-all --out /tmp/mirrorea-alpha1-release-p-surf-02`:
  accepted, product alpha release candidate ready, failed commands `[]`.
- `python3 scripts/operational_product_samples.py check-all --format json`:
  accepted, failed commands `[]`.
- `python3 scripts/minimal_alpha1_patterns.py check-all --format json`:
  accepted, failed `[]`.

## What changed in understanding

The indexed-state checker cannot use nested place blocks as a temporary owner
switch. Even before P-SURF-03 elaboration exists, the checker must preserve the
distinction between authoring placement and authority. A nested foreign
`S { ... }` from a role locus is a future owner-directed request, not direct
write authority.

State names also need owner-scoped storage identity. Treating `player` as a
module-global name collapses distinct `S.player` and `T.player` maps, which
conflicts with the owner-locus reading in `specs/40`.

## Open questions

- Explicit capability-mediated remote indexed-state writes remain for
  P-SURF-03/P-SURF-05, where generated requests and admission-derived grants can
  be represented without faking authority.
- Witness / in-flight / lease compaction blockers remain lifecycle obligations
  for later runtime carriers; P-SURF-02 only checks the retained-savepoint
  marker row.
- Surface-to-Core source spans, Core IR transitions, generated communication,
  and obligations remain for P-SURF-03/P-SURF-04.

## Suggested next prompt

Continue autonomously with `P-SURF-03 Surface-to-Core elaboration`.

## Plan update status

`plan/00-index.md`, `plan/65-indexed-state-roadmap.md`, and
`plan/68-surface-full-system-v1-roadmap.md` were updated. `plan/65` now records
P-SURF-02 as closed checker evidence and keeps generated request / runtime
carrier work in later packages.

## Documentation.md update status

`Documentation.md` was updated to include the P-SURF-02 indexed-state semantic
checker floor and to point the current promoted package to P-SURF-03.

## progress.md update status

`progress.md` was updated with the P-SURF-02 closeout, `IDX-01..05`, current
runnable commands, non-claims, timestamp, and current package
`P-SURF-03 Surface-to-Core elaboration`.

## tasks.md update status

`tasks.md` was rewritten as the current task map with P-SURF-02 closed and
P-SURF-03 first in the self-driven queue.

## samples_progress.md update status

`samples_progress.md` was updated in-place to track
`samples/full-system-v1-surface/indexed-state/` as `IDX-01..05` semantic
checker evidence only, not runtime workflow readiness.

## Reviewer findings and follow-up

Docs/status reviewer found stale P-SURF-02-as-next wording in README,
Documentation, progress/tasks, sample dashboard, sample README, script README,
plans, and scope docs. Follow-up: updated those files to mark P-SURF-02 closed
as a narrow checker/sample floor and P-SURF-03 as next.

Indexed-state/security reviewer found three blocking or important issues:
nested `S { ... }` could become ambient owner authority, state names were keyed
module-globally rather than by owner locus, and retained-evidence wording
overstated the implemented compaction blocker. Follow-up: added
`indexed_state_nested_place_requires_generated_request`, owner-scoped state
keys, `IDX-05`, Rust regression tests, and narrowed retained-savepoint wording.

The reviewer also requested explicit capability-mediated access coverage. That
is intentionally deferred because capability grants belong to P-SURF-05 and
generated owner-directed requests belong to P-SURF-03; this report records the
gap as a residual obligation rather than adding fake authority syntax.

## Skipped validations and reasons

No requested validation is intentionally skipped. Heavy Product Alpha and
operational compatibility anchors are rerun for package close with fresh output
directories where required.

## Commit / push status

Pending at report write. The package commit will use
`git commit --no-gpg-sign -m "p-surf-02: add indexed state semantics"` and will
be pushed after final staged validation.

## Sub-agent session close status

Docs/status and indexed-state/security reviewer agents completed. Their
findings were addressed or recorded as deferred residual work. Both sessions
were closed before moving to P-SURF-03.
