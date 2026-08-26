# Report 2594 — Mirrorea I2 Systems Foundation SYS-2: ST/OW1 concurrency refinement

- Date: 2026-08-27 JST
- Milestone: SYS-2
- Status: **completed / closed** at accepted source/evidence cut
  `920d3fe050b8b909253f8511d9ad897272323ced`; SYS-3 active, SYS-4 next
- Evidence classes: OBL-058 `model-checked-bounded`; OBL-059
  `runtime-monitored`; no new Lean claim
- Lifecycle: theory T1; broad PHASE-I1 and official I2 entry/exit remain
  unaccepted

## Objective

Give the selected SYS-1 owner/designated-input kernel fragment one
deterministic single-thread (ST) reference and one safe one-owner-worker (OW1)
execution profile whose implementation occurrences refine the required Mir
ordering edges, and close the immutable-M9-snapshot revocation visibility gap
without exposing low-level memory order in Surface or freezing a public
contract.

## Scope and assumptions

OW1 is deliberately bounded to exactly one combined semantic owner/source-
owner locus per kernel. One dedicated operating-system thread exclusively owns
`M8LocalRuntime`; a coordinator uses acknowledged zero-capacity mailbox
commands and has no public shared mutable M8 store. This is not multi-locus
generated dispatch.

Successful owner mutation linearizes at the actual acknowledged M8
`OwnerWrite` trace node and records the corresponding enqueue/read,
per-location version, and preceding writer. Designated remote input is derived
from an acknowledged source-owner read. The same admitted M9 seam retains the
only production successor publisher; complete retranslation and owner refresh
acknowledgement precede generation publication.

The bounded model covers the exact ten required-edge families at bound 6.
Bounded exploration is not a proof of arbitrary traces, schedulers, hardware
memory, fairness, liveness, or data-race freedom. `WeakMemoryCalibration` is a
separate store-buffering calibration, not a third Mir execution profile.

```text
Direct consumer: SYS-3 artifact requirements and SYS-4 ST/OW1 execution
Blocker reduced: no bounded thread/authority-publication refinement contract
  was available for generated locus programs to preserve
Acceptance use: OBL-058/059, SYS-3 projection preservation, and later SYS-6
  finite correspondence
```

## Start state / dirty state

- Branch: `main`.
- Initial `HEAD == origin/main`:
  `920d3fe050b8b909253f8511d9ad897272323ced`
  (`feat(runtime): add SYS-2 ST OW1 refinement`).
- Worktree was clean at this planning/status writer's start; no uncommitted
  user change was present.
- Full SYS-2 source/test delta from the SYS-1 closeout `0bb83524...` through
  the accepted cut `920d3fe0...`: 7 runtime/test files, 5,914 insertions,
  196 deletions across five commits. The final implementation commit alone
  changed 6 files with 4,625 insertions and 213 deletions.
- Resource recheck: root filesystem 188 GiB total, 57 GiB available; memory
  15 GiB total, about 13 GiB available. No heavy artifact was added by this
  docs writer.

## Documents consulted

- Canon entry/direction: `mirrorea_canon/README.md`, `MAP.md`,
  `NORTH-STAR.md`, `DESIGN-CONSTITUTION.md`, and root `CANON.md`.
- Authority/lifecycle/program: ADR-0025--0027, Canon plan/00-gates,
  plan/01-phases, and plan/02-operating-model.
- Task-specific semantics: architecture/04-runtime-carriers,
  theory/04-ordering-and-cuts, theory/05-authority,
  theory/11-metatheory-ledger, theory/13-evaluation-materialization,
  theory/18-m9-auth-verification, and spec/05-runtime-semantics.
- Sole current execution control: LAB Plan 249.
- Source evidence at `920d3fe0...`: `sys2_execution_backend.rs`,
  `sys2_bounded_model.rs`, `semantic_runtime_kernel.rs`,
  `m9_auth_verification.rs`, their focused tests, and crate module boundary.
- Current derived views: root README, `Documentation.md`,
  `docs/project-status.md`, the primary HTML reader, `progress.md`, `tasks.md`,
  `plan/00-index.md`, and `samples_progress.md`.
- Only directly relevant Report 2593 was read; `docs/reports/` was not read in
  bulk.

## Actions taken

1. Fixed ST as the deterministic reference and OW1 as one coordinator plus one
   dedicated worker-exclusive M8 runtime for exactly one combined owner/
   source-owner locus; other locus counts reject typed.
2. Bound successful mutation linearization and reads-from/coherence to actual
   M8 enqueue, `OwnerRead`, and `OwnerWrite` observations. Failed/revoked serve
   produces no fabricated success evidence.
3. Required remote-input results to derive from the acknowledged source-owner
   read and retained explicit evaluator consumption; legacy mismatched supplied
   value rejects before reply/receipt/mutation.
4. Closed live authority visibility through the same-seam M9 publisher: actual
   revoke, full inventory retranslation, monotone tombstones, unrelated-
   lineage preservation, owner install acknowledgement, then generation
   publication.
5. Added a typed finite transition model and replayable edge-removal
   counterexamples for ten high-level ordering families; kept store buffering
   as a separate weak-memory calibration.
6. Recorded PROPOSAL-031 / ADR-0028 and OBL-058/059 through the normal Canon
   process. No older Lean/general OBL status moved.
7. Advanced the sole roadmap and derived snapshots to SYS-2 completed,
   SYS-3 active, SYS-4 next while preserving theory T1 and broad lifecycle
   non-acceptance.
8. Kept the report count to this single milestone report and opened no WRK.

## Files changed

Created:

- `mirrorea_canon/meta/proposals/PROPOSAL-031-sys2-st-ow1-concurrency-refinement.md`
- `mirrorea_canon/adr/ADR-0028.md`
- `docs/reports/2594-mirrorea-i2-systems-foundation-sys2-concurrency-refinement.md`

Created or updated earlier in the complete SYS-2 source/test delta:

- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/m9_auth_verification.rs`
- `crates/mir-runtime/src/semantic_runtime_kernel.rs`
- `crates/mir-runtime/src/sys2_bounded_model.rs`
- `crates/mir-runtime/src/sys2_bounded_model_tests.rs`
- `crates/mir-runtime/src/sys2_execution_backend.rs`
- `crates/mir-runtime/src/sys2_execution_backend_tests.rs`

Updated:

- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/theory/04-ordering-and-cuts.md`
- `mirrorea_canon/theory/05-authority.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/theory/13-evaluation-materialization.md`
- `mirrorea_canon/theory/18-m9-auth-verification.md`
- `mirrorea_canon/spec/05-runtime-semantics.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/INDEX.json` (generated)
- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `plan/00-index.md`
- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py` (parent integration
  update for the SYS-3/SYS-4 reader pointer)
- `progress.md`
- `tasks.md`

`samples_progress.md` was inspected and remains unchanged. This writer did not
edit the HTML regression test under the assigned docs-only boundary; the
parent applied the exact current-pointer assertion update after the initial
expected RED result.

## Commands run

Baseline/source/resource inspection by this writer:

```text
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
git show --stat --oneline --decorate 920d3fe0
df -h .
free -h
TZ=Asia/Tokyo date '+%Y-%m-%d %H:%M:%S %Z'
cargo test -p mir-runtime --lib -- --list
cargo test -p mir-runtime --lib sys2_ -- --nocapture
cd mirrorea_canon && python3 meta/build-index.py
make docs
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html
git diff --check
rg -n 'active goal (is|は) SYS-2|next goal (is|は) SYS-3|SYS-2 active|SYS-3 next' \
  README.md Documentation.md docs/project-status.md \
  docs/mirrorea-project-overview.html progress.md tasks.md \
  plan/249-mirrorea-i2-systems-foundation-current-roadmap.md \
  mirrorea_canon/README.md mirrorea_canon/MAP.md \
  mirrorea_canon/plan/01-phases.md
```

The parent supplied fresh source-cut close evidence from:

```text
cargo fmt --all -- --check
cargo clippy -p mir-runtime --all-targets -- -D warnings
cargo test -p mir-runtime --lib sys2_ -- --nocapture
cargo test -p mir-runtime --lib sys1_runtime_kernel_tests -- --nocapture
cargo test -p mir-runtime --test m10_source_execution -- --nocapture
cargo test -p mir-runtime --test m10_cli -- --nocapture
cargo test -p mir-runtime --test m10_conformance -- --nocapture
cargo test -p mir-runtime --no-fail-fast
git diff --check
```

Post-edit Canon/docs commands and their exact outcomes are recorded in the
evidence section.

## Evidence / outputs / test results

- Combined SYS-2 focused group: **27/27 pass** = 13 external bounded-model
  tests + 9 backend tests + 5 internal model review-regression tests.
- SYS-1 kernel regression: **13/13 pass**.
- M10 source integration: **2/2 pass**.
- M10 CLI integration: **4/4 pass**.
- M10 conformance regression: **67/67 pass**.
- Full `mir-runtime` suite: exit 0.
- Formatting, all-target warnings-denied changed-crate Clippy, and source diff
  check: exit 0.
- Canon index generation: **176 files indexed**; generated index check passes.
- Final `make docs`: exit 0. Agent configuration passes, Canon index checks
  **176 files**, source hierarchy finds **799/799** required paths, and docs/
  secret validation reports `Documentation scaffold looks complete` with
  1,748 numbered reports found. Intermediate runs detected only the rewritten
  `tasks.md` Canon-notice/required-heading/source-reference contract; those
  findings were corrected before the final wrapper rerun.
- Primary HTML reader regression: initial expected RED was **2/8 failures**
  against the stale SYS-2-active/SYS-3-next test contract; after the parent
  updated only those assertions, **8/8 pass**.
- Current-pointer audit over Plan 249, current Canon entry/phase views, and
  current LAB readers/status snapshots finds no stale SYS-2-active or
  SYS-3-next claim.
- Final docs diff check: exit 0.
- ST/OW1 two-request same-owner RMW results/lifecycles agree. OW1 evidence
  comes from the actual worker-owned M8 enqueue/read/write path.
- Revocation after enqueue rejects later serve as `MissingCapability` with no
  mutation; commit-before-revoke remains completed and later receipt remains
  non-authority. Full retranslation preserves unrelated owner and remote-
  release lineages.
- Remote input uses the latest worker-owned owner version and reaches explicit
  evaluator consume; mismatched supplied input rejects before reply/receipt/
  mutation.
- Full-edge ST/OW1 bounded-model selected outcomes agree; each omitted required
  edge has a replayable typed bad-state counterexample. Bound-zero/
  insufficient search cannot report completion.
- Evidence classification: OBL-058 `model-checked-bounded`; OBL-059
  `runtime-monitored`. No Lean statement/theorem or general proof is claimed.

## What changed in understanding

The first threaded contract does not require shared mutable runtime state or
low-level Surface memory syntax. A worker-exclusive owner store and
acknowledged mailbox are sufficient for the selected fragment when success
evidence is tied to actual M8 trace nodes rather than coordinator bookkeeping.

The SYS-1 immutable admission does not need to be discarded. It can become one
immutable generation in a monotone M9-produced sequence, provided complete
retranslation and owner acknowledgement occur before kernel publication.
The resulting direct SYS-3 input is a semantic backend requirement, not the
concrete Rust channel/worker layout.

## Open questions

- What is the smallest internal `GlobalProjectionResult` and `LocusProgram`
  representation that preserves the accepted Core and SYS-2 requirements?
- How will SYS-3 prove generated communication completeness and reject one
  omitted or extra edge without turning the conformance manifest into runtime
  architecture?
- Which one conservative finite DAG pressure case best tests relation/fallback
  extension without claiming arbitrary-DAG theory?
- OPEN-026, OPEN-027, full carrier freeze, multi-owner execution, public
  encoding, real transport, retry/exactly-once, and general memory/fairness
  proofs remain later scope.

## Suggested next prompt

Execute Plan 249 SYS-3: define deterministic checked-Core-driven per-locus
artifacts and generated communication/effect/observation/persistence plans,
write no-hidden-edge/owner-preservation/malformed-projection falsifiers first,
carry the semantic SYS-2 ST/OW1 requirements without exposing backend layout,
and keep runtime dispatch, public ABI/wire, real transport, and arbitrary-DAG
theory outside this milestone.

## Plan update status

更新済み: Plan 249 records SYS-2 completed at exact cut `920d3fe0...`, OBL-058/
059 evidence, SYS-3 active, SYS-4 next, the projection direct blocker, and
reopen triggers. Plan 00 index matches the sole current roadmap.

## Documentation.md update status

更新済み: `Documentation.md` now says SYS-0--SYS-2 completed, SYS-3 active,
SYS-4 next; it records OBL-058/059 and preserves theory T1/broad lifecycle
non-claims.

## docs/project-status.md update status

更新済み: `docs/project-status.md` records the SYS-2 cut/evidence and current
SYS-3 projection blocker without treating bounded program progress as official
I2 acceptance.

## progress.md update status

更新済み: `progress.md` synchronizes the three axes, milestone/macro/feature
rows, startability, exact evidence classes, and timestamped recent log.

## tasks.md update status

更新済み: `tasks.md` is rewritten as the SYS-3-active snapshot with ordered
self-driven packages, research-discovery items, owner-reserved decisions,
rough estimates, and evidence pointers.

## samples_progress.md update status

更新不要: SYS-2 changed no runnable sample path, user-facing validation
command, debug surface, or sample blocker. The new Rust/model tests are
milestone assurance, not a runnable sample workflow. Conclusion:
`samples_progress.md 更新不要`.

## Reviewer findings and follow-up

- Semantic/specification review: **ACCEPT**, no remaining P0/P1/P2.
- Concurrency/code-quality review: **ACCEPT**, no remaining P0/P1/P2.
- Finite-model review: **ACCEPT**, no remaining P0/P1/P2.
- Test-contract review: **ACCEPT** after the RED contract was strengthened.

Corrections included retaining the same-seam M9 publisher; preserving
unrelated owner/designated lineages through full retranslation; removing
caller-supplied production remote results; requiring acknowledged source-owner
reads; binding LP/reads-from/coherence to actual M8 nodes; suppressing
fabricated success rows on failure; exercising post-generation evaluator
consume; and making model replay/state fingerprints, store buffering, patch
reject/execute separation, authority no-mutation, terminal uniqueness, and
coverage non-vacuous.

The independent Canon-first close planner review is owned by the parent and is
complete after one P2 report-accuracy correction: this report now distinguishes
the full five-commit SYS-2 delta from the final implementation commit and lists
all seven runtime/test files. The reviewer found no Canon/status semantic drift
or remaining P0/P1/P2 issue after that correction.

## Skipped validations and reasons

- Lean `--trust=0` was not run because SYS-2 changes no Lean source or Lean
  claim. OBL-058/059 are explicitly bounded-model/runtime evidence.
- Fresh-clone M10 release reproduction was not rerun because its immutable M10
  identity is regression baseline, not SYS-2 runtime identity.
- Real transport, browser renderer, durable persistence, lock-free/performance,
  and production checks are outside SYS-2.
- No workspace-wide Rust or Lean suite was rerun by this docs writer. Source
  validation is the parent's fresh cut evidence listed above; this writer
  independently reran the combined 27-test SYS-2 group and all docs-specific
  checks.
- No additional validation was skipped for the changed Canon/docs/HTML layer;
  the final `make docs`, HTML regression, and diff checks all passed.

## Commit / push status

The accepted implementation/evidence cut
`920d3fe050b8b909253f8511d9ad897272323ced` is the clean
`HEAD == origin/main` baseline for this closeout. This Canon/report/status diff
is not committed or pushed by the planning writer. The parent owns the
integration commit, push, clean-worktree check, and remote parity before SYS-3
source work begins.

The successor docs commit cannot embed its own hash without another successor;
the report therefore pins the exact source cut and leaves commit/push/parity to
the parent integration record.

## Sub-agent session close status

Implementation, all four independent source/test review lanes, the independent
Canon-first close planner review, the parent HTML-test update, and final docs
integration validation are complete. This planning/status writer spawned no
additional sub-agent. Integration commit/push, clean-worktree check, and remote
parity remain open at this handoff.
