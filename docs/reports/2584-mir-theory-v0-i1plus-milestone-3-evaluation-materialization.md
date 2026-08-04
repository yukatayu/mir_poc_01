# Report 2584 — Mir Theory v0 / I1+ Milestone 3 evaluation/materialization

- Date: 2026-08-04 12:09 JST
- Author / agent: Codex orchestrator
- Scope: close the finite, parser-free M3 evaluation/materialization calculus
  only. It is the bridge from the M2 T1 entry to M4 maintained relations; it
  does not establish Surface v0, relation projection, M8 runtime, save/load,
  patching, conformance, I1 authorization, public API/ABI/wire, or deployment.
- Decision levels touched: L1-fixed ADR-0018/theory-13 finite calculus and its
  exact proof-ledger classifications. The owner Constitution and all
  owner-reserved conditions remain unchanged.

## Objective

Define and execute one deterministic finite calculus in which semantic form,
evaluation site, trigger/clock, authority origin, and materialization are
orthogonal. Demonstrate same-owner RMW at the owner, explicit release-admitted
cross-owner receipt use without a hidden transaction, and a designated
evaluator that publishes a versioned result before a named consumer consumes it.

## Scope and assumptions

- M3 starts from clean `21ec5825034d406a0fc93e9743d79367b2892599 ==
  origin/main` after M2's accepted T1 entry.
- ADR-0015 delegates the bounded program; no owner-reserved trigger was found.
- The reference is syntax-free and parser-free. A deterministic operation key
  is its M3 origin surrogate; source spans remain M6 work.
- A receipt release grant is distinct from O's write capability. It admits one
  exact `(caller, producer, target, label)` release tuple, transfers no
  authority, and has no final wire or identity-provider implication.
- M3's designated store is deliberately finite and single-consumer. M8 owns
  durable one-shot consumption, save/load, and distributed delivery.

## Start state / dirty state

- Official lifecycle was `T1`; M0--M2 were closed and remote-parity checked.
- The M3 worktree started clean. No user change, untracked user file, submodule,
  configured external workdir, or external worktree was present.
- Resources and tools were inspected before generated work: approximately 21G
  disk free, `/mnt/mirrorea-work` not mounted, Lean 4.29.1 / Lake 5.0.0, and
  Rust 1.94.0. M3 does not create a heavy external artifact.

## Documents consulted

- Canon first: `README`, `MAP`, Constitution, ADR-0015/0016, theory 01--03,
  05, 11, SCN-02, and the direct M3 proposal/ADR routes.
- LAB: Plan 247, current status snapshots, the directly relevant M0--M2
  reports, sample/script readmes, and the narrow `mir-semantics` code map.
  The complete `docs/reports/` history was not read.
- Independent inputs: pre-edit planner/code-map reviews, test-first author,
  production implementer, and a distinct final reviewer. A bounded external
  Oracle review was advisory only and was checked against Canon before any
  repository decision.

## Actions taken

1. Added PROPOSAL-021, ADR-0018, theory-13, and SCN-11. One pure
   `eval(key, body, EP)` Core carrier records the five axes; owner RMW,
   receipt use, and designated materialization are normalized forms/rows.
2. Kept a same-owner state read/RHS/write within the owner FIFO service. The
   requester remains the authority origin and never receives the private RHS
   operand.
3. Defined an explicit receipt path. A value is available to owner O only after
   release admission and the deterministic request → serve → reply → receive
   sequence. The receipt checks producer, target, label, canonical frontier,
   and stored identity; it is neither a snapshot nor a capability grant.
4. Separated designated evaluation from consumer consumption. A fixed
   evaluator/key/canonical-frontier publishes one policy-stamped version; a
   named consumer has one explicit bounded consumption and cannot re-evaluate
   the semantic expression.
5. Made the parser-free provenance boundary explicit: every plan and trace row
   has a deterministic nonempty operation key. M6 must replace it with source
   span mapping rather than treating this as a completed Surface source map.
6. Corrected the M3/M4 boundary: consumer relation classification is
   `{local-only}` in M3. Upstream relation publication/execution remains M4.
7. Used RED/GREEN focused tests. The independent review exposed five initial
   P1 flaws (unproven receipt causality, publication/consume conflation,
   ordered frontier, missing operation origin, and consumer target conflict),
   then a release-admission P1. Each was corrected once and independently
   re-reviewed.

## Files changed

- Canon: PROPOSAL-021, ADR-0018, theory-13, SCN-11, theory/core/elaboration/
  ledger/spec/scenario indexes and changelog updates.
- Formal evidence: `samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.lean`
  and its companion, plus the active Lean sync inventory/script documentation.
- Reference behavior: `crates/mir-semantics/src/evaluation_materialization.rs`,
  module exposure, and the two focused M3 test files.
- Generated inventory and reader-facing snapshot: `samples/lean/manifest.json`,
  Plan 247, Documentation, project status, progress, tasks, samples dashboard,
  and this report.

## Commands run

- Read-only start-state, Canon-first, resource/toolchain, code-map, and agent
  configuration audits.
- TDD RED/GREEN focused M3 Rust tests, including all `2^6 = 64` target subsets
  and adverse receipt/release/designated-consumption cases.
- `cargo fmt --check`
- `cargo clippy -p mir-semantics --tests -- -D warnings`
- `cargo test -p mir-semantics --test evaluation_materialization_calculus --test evaluation_materialization_owner_rmw`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.lean`
- targeted `sorry` / `admit` / `axiom` scan and `#print axioms` inspection.
- `python3 scripts/current_l2_lean_sample_sync.py`
- `cd mirrorea_canon && python3 meta/build-index.py && python3 meta/build-index.py --check`
- `make docs`, `git diff --check`, secret-scan coverage within the documentation
  validation, and independent review/re-review.

## Evidence / outputs / test results

- Rust focused suite: 18 passed (10 calculus, 8 owner-RMW); zero failures.
  It verifies deterministic same-owner replay, `100 → 90 → 80`, missing write
  capability no-mutation, release denial/no receipt rows, manual receipt
  rejection, wrong producer/target rejection, canonical frontier permutations,
  all 64 materialization subsets, and single publish/single consumer consume.
- Clippy with `-D warnings` and workspace formatting passed for the affected
  crate/tests.
- Lean compiled with `--trust=0`. The direct source scan found no `sorry`,
  `admit`, or user `axiom`. `elaboration_deterministic` and
  `two_attacks_are_serial_owner_rmw` have no axioms; the bounded WF theorem
  depends only on Lean's standard `propext`/`Quot.sound`, and duplicate decision
  stability on standard `propext`. No user-defined hidden axiom was found.
- OBL-029--032 are `lean-proved` only for the stated finite model. OBL-033 is
  the exact 64-subset `model-checked-bounded` enumeration; OBL-034 is a
  parser-free typed-trace `runtime-monitored` result. General OBL-001--028 are
  intentionally deferred, not renamed proofs.
- Canon index regenerated and checked at 146 files. M3 documentation validation
  and the final independent re-review passed with no P0/P1 finding.

## What changed in understanding

The receipt path needs both visible causal rows and a producer-side release
admission; causal ordering alone does not preserve authority or information
flow. Likewise, a designated decision is not a consumer action. Keeping the
canonical frontier, policy stamp, operation-origin key, and consumption state
separate produces a small model that M4 can extend without smuggling relation
projection, a transaction, or a final source/wire interface into M3.

## Open questions

- No owner decision is required.
- M4 must define maintained relation DAGs, derived labels, late projection,
  split-frame rejection, semantic/presentation fallback, and fresh reacquire.
- M5 must lift the M3 finite evidence into the shared model; M6 must add actual
  source spans; M8 must add runtime/save/load/patch and durable consumption.

## Suggested next prompt

No prompt is required. Continue autonomously with M4 maintained relation / late
projection, starting from the bird/shoulder pressure scenario.

## Plan update status

更新済み: Plan 247 now marks M3 closed, M4 as the sole active milestone, M5
next, and M4 relation/projection as the direct blocker.

## Documentation.md update status

更新済み: the reader entry now routes to M4 and accurately separates finite M3
ledger evidence from general deferred obligations.

## docs/project-status.md update status

更新済み: the derived control view records M3 non-effects, exact proof-status
vocabulary, and M4 as the current frontier.

## progress.md update status

更新済み: the three-axis snapshot, milestone map, feature row, and dated recent
log now record M3 evidence and M4 startability.

## tasks.md update status

更新済み: the current task map is recut to M4 active / M5 next with no parallel
semantic candidate family.

## samples_progress.md update status

更新済み: the active Lean foundation, trusted command, generated manifest, and
finite-evidence/non-claim classification are now listed.

## Reviewer findings and follow-up

- Pre-edit planner confirmed the M3/M4 boundary, one small explicit receipt
  alternative, no owner-reserved trigger, and the exact evidence needed.
- The first independent final review found five P1 findings: manual receipt
  insertion without causal trace, decision/consume conflation, order-sensitive
  frontier, missing parser-free origin key, and a consumer target contradiction.
  Test-first corrections closed all five.
- Narrow re-review then found one P1: a caller could choose a producer/label
  without producer admission. The exact release-grant check plus negative tests
  corrected it. Final re-review found no P0/P1.

## Skipped validations and reasons

- Full workspace test/release matrices are not M3 surfaces; the changed
  `mir-semantics` crate, Canon, Lean foundation, and documentation were tested
  directly.
- Parser/Surface source spans, relation projection, M8 runtime, save/load,
  patch, distributed transport, final public formats, and broad conformance are
  intentionally later milestones and are not claimed.

## Commit / push status

The M3 integration uses `--no-gpg-sign`, is pushed to `origin/main`, and its
post-push remote parity is verified as the final closeout action.

## Sub-agent session close status

- M3 pre-edit planner: complete, read-only; its accidental Lean sync execution
  was retained as the required generated manifest update and recorded here.
- M3 test author: complete; test-only RED/GREEN ownership, including final
  release-admission negatives.
- M3 implementer: complete; production module only.
- M3 independent reviewer: complete, read-only; one P1 correction cycle and
  a final no-P0/P1 re-review.
