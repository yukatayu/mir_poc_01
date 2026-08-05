# Report 2591 - Mir Theory v0 / I1+ Milestone 10: conformance closeout

Date: 2026-08-05
Milestone: M10
Status: accepted / closed

## Objective

Close the owner-approved ADR-0015 Mir Theory v0 / I1+ Milestones 0--10 program
with fresh, source-first C-static and C-runtime evidence for frozen SCN-01..10,
exact proof/evidence classifications, independent review, and explicit
non-claims.

## Scope and assumptions

Normative source remains `mirrorea_canon/`. This closeout records the accepted
immutable R5 source cut commit `23f5a8130334bf0c8516d51e9dcea38b92f50db1`,
tree `d8a296fac7a94a37da92563d5feeeeaa96dbc682`. M10 covers only the finite
I1+ deterministic reference profile selected by ADR-0025 and spec/11. It does
not claim final public grammar/API/ABI/wire, C-distributed, sockets, production
deployment, I2 activation, public product completion, or broad PHASE-I1 exit.

## Start state / dirty state

- M10 started after M9 close at pushed cut
  `187e93663a65a782e99a31c1bb7395e79d74294f`.
- Earlier root-disk pressure was observed during M10: existing `target/`
  linking grew the cache and left about 17 MiB free before bounded cleanup of
  only disposable `target/debug/incremental` contents.
- Old disposable fresh-clone directories were cleaned before R5; earlier target
  cleanup had already recovered root space. At R5 acceptance, the current root
  filesystem had about 18 GiB free. `/dev/shm` had about 7.2 GiB free before
  the R5 fresh-clone reproduction.
- The accepted fresh clone was
  `/dev/shm/mir_i1plus_r5_EpcI8h/repo`.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `CANON.md`.
- `mirrorea_canon/adr/ADR-0015.md`, `mirrorea_canon/adr/ADR-0025.md`.
- `mirrorea_canon/spec/06-conformance.md`,
  `mirrorea_canon/spec/11-m10-i1plus-conformance.md`.
- `mirrorea_canon/theory/11-metatheory-ledger.md` and M3--M9 Lean foundation
  files.
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`.
- This M10 report and the final reviewer ACCEPT summary supplied for R5.

## Actions taken

1. Preserved the M10 finite source-first conformance boundary: ordinary `.mir`
   source, typed profile/carrier inputs, M6/M7 checking, M8/M9 runtime,
   trace/projection, and exact correspondence verification remain distinct.
2. Recorded the rejected/corrected R1--R5 history instead of reinterpreting
   earlier failed or partial greens as success.
3. Accepted only the immutable R5 cut after reviewer ACCEPT with no P0/P1/P2.
4. Synchronized the Canon acceptance record, Plan 247, and the derived LAB
   closeout snapshots to mark M10 closed and the owner-approved M0--M10 program
   complete.
5. Kept the accepted R5 production/test tree immutable while recording its
   exact finite claims and post-program non-effects.

## Files changed

The M10 source-cut commits changed the bounded I1+ implementation and evidence
surfaces under `crates/mir-ast/`, `crates/mir-semantics/`,
`crates/mir-runtime/`, and `samples/clean-near-end/i1plus-reference*`, together
with the selected M10 Canon profile/specification files. The closeout cut then
updated:

- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/adr/ADR-0025.md`
- `mirrorea_canon/plan/01-phases.md`
- `README.md`
- `AGENTS.md`
- `plan/00-index.md`
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`
- `docs/reports/2591-mir-theory-v0-i1plus-milestone-10-conformance-closeout.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`

The exact source-cut inventory is reproducible with
`git diff --name-only 187e93663a65a782e99a31c1bb7395e79d74294f..23f5a8130334bf0c8516d51e9dcea38b92f50db1`.

## Commands run

Evidence accepted for R5:

- `git rev-parse HEAD`
- `git rev-parse HEAD^{tree}`
- `git clone ... /dev/shm/mir_i1plus_r5_EpcI8h/repo`
- `cargo test -p mir-runtime --test m10_conformance -- --nocapture`
- `cargo test -p mir-runtime --test m10_source_execution -- --nocapture`
- `cargo test -p mir-runtime --test m10_cli -- --nocapture`
- `cargo test -p mir-runtime --lib -- --nocapture`
- `cargo test --workspace --all-targets`
- `cargo fmt --all -- --check`
- `cargo clippy -p mir-runtime --all-targets -- -D warnings`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `lean --version`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M5SharedModel.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M6Surface.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M7CheckedElaboration.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M8DeterministicRuntime.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M9AuthVerification.lean`
- generated `#print axioms` audit over those seven Lean foundation files
- placeholder/user-axiom scans for `sorry`, `admit`, user `axiom`, `unsafe`,
  `partial`, `implemented_by`, and theorem-as-`True` stubs
- `cd mirrorea_canon && python3 meta/build-index.py --check`
- `python3 scripts/check_source_hierarchy.py`
- `python3 scripts/validate_docs.py`
- `git diff --check`
- `df -h`, `/dev/shm` space check, and focused cleanup/size inspection commands

Local closeout-doc validation after this status-only edit:

- `date '+%Y-%m-%d %H:%M %Z'`
- `make docs`
- `cargo fmt --all -- --check`
- `cargo clippy -p mir-runtime --all-targets -- -D warnings`
- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=1 cargo test --workspace --all-targets`
- seven M3--M9 `lean --trust=0` foundation commands and the placeholder scan
- the full `mir conform` command twice against the committed corpus, schedule,
  typed carriers, and predicate profile, each piped to `sha256sum`
- `git diff --check`

## Evidence / outputs / test results

Accepted R5 source cut:

- Commit: `23f5a8130334bf0c8516d51e9dcea38b92f50db1`
- Tree: `d8a296fac7a94a37da92563d5feeeeaa96dbc682`
- Fresh clone: `/dev/shm/mir_i1plus_r5_EpcI8h/repo`
- Output SHA256 reproduced twice:
  `083523518fdae0a111522f49b148c818ca0d5c21b4b7cc4f34dd476f10d172e7`
- Conformance result: accepted; static 26/26, runtime 47/47, mismatch 0,
  missing 0, anchor true, waiver null.
- Profile/manifest hash: `fnv1a64:6a1cfac2a0950323`
- Verifier profile hash: `fnv1a64:420308515cf98e18`
- Source revision: `fnv1a64:7bff6aa952a8ad53`
- Execution identity: `fnv1a64:5b4d58cf1cd20428`
- Focused runtime evidence: 392 passed / 0 failed across 67 targets.
- Workspace evidence: 886 passed / 0 failed across 149 targets.
- M10 conformance suite: 67 passed / 0 failed.
- Lean: 4.29.1, `--trust=0`, seven M3--M9 foundation files compiled.
- Lean axiom audit: actual dependencies include `propext` and selected
  `Quot.sound`; no file is described as axiom-free where the audit says
  otherwise.
- Placeholder audit: no `sorry`, `admit`, user axiom placeholder, `unsafe`,
  `partial`, `implemented_by`, or theorem-as-`True` placeholder was accepted.
- Proof ledger: unchanged by M10. M10 adds no general theorem or OBL discharge.
- Final current-checkout verification after closeout edits reran the full
  workspace all-target suite successfully, recompiled all seven Lean
  foundations with `--trust=0`, and reproduced the same conformance SHA-256
  twice. Closeout edits did not change the accepted R5 source/test tree.

Rejected/corrected history:

- R1 rejected: partial green evidence still included fixture/result lookup and
  incomplete correspondence; it was not admissible M10 evidence.
- R2 rejected: nominal green paths still inferred facts from typed event
  vocabulary rather than real source-bound transitions.
- R3 rejected: `target_leave` created a fresh target M9 session at leave time,
  minted membership/capability/witness authority, and reached M8 presence
  mutation through a public/raw-provenance boundary.
- R4 rejected/corrected: the no-mint and sealed-presence fixes closed that P0,
  but an external `target_leave` control row still carried a source reference;
  R5 separated checked-source admission from source-free external control.
- R5 accepted: reviewer ACCEPT, no P0/P1/P2, immutable committed source cut and
  fresh-clone reproduction recorded above.

## What changed in understanding

M10 is a finite deterministic reference-profile acceptance, not a report
aggregator and not a public release. The accepted profile proves that the
selected I1+ reference path can reproduce frozen SCN-01..10 C-static/C-runtime
without waivers from the same committed source/profile inputs. It leaves Theory
`T1` as the official lifecycle state and does not unlock I2 by itself.

## Open questions

- OPEN-030 / carrier boundary remains open for post-program direction.
- Broad PHASE-I1 exit, public ABI/wire/carrier freeze, and I2 activation await
  new owner direction.
- General OBL-001..025 and OBL-027 remain intentionally deferred as recorded in
  Canon; M10 does not change the proof ledger.

## Suggested next prompt

Owner should define the post-M0--M10 direction: retain the accepted I1+
deterministic reference profile as a closed baseline, then decide whether the
next program targets public ABI/wire/carrier freeze, I2 entry, OPEN-030 carrier
boundary, or another explicitly scoped line.

## Plan update status

更新済み: the independent planning writer changed
`plan/247-mir-theory-v0-i1plus-current-roadmap.md` from active M10 to a closed
M0--M10 record, with no next active roadmap, the exact R5 evidence, and a
non-activating I2 entry contract.

## Documentation.md update status

更新済み: marked M10 accepted/closed, recorded decisions taken, verification
status, and open risks/non-claims.

## docs/project-status.md update status

更新済み: concise LAB derived status view now says M0--M10 closed, I1+ finite
reference profile accepted, post-program direction owner-defined.

## progress.md update status

更新済み: snapshot and recent log use the `date` command timestamp
`2026-08-05 15:53 JST`.

## tasks.md update status

更新済み: fully rewritten as the current task map; no autonomous package remains inside
M0--M10; owner-defined post-program direction is the next gate.

## samples_progress.md update status

更新済み: the Lean/M10 evidence row names the runnable conformance command and
accepted evidence without claiming public/product 100% completion.

## Reviewer findings and follow-up

Final semantic reviewer disposition for R5: ACCEPT with no P0/P1/P2. Earlier
reviewer findings rejected R1--R4 as recorded above; their corrections are
incorporated in the accepted R5 cut. The closeout planner then found two P1s:

1. Plan 247 and three reader-facing references still called the closed record
   the current queue. The closeout cut corrected them to state that no active
   roadmap exists.
2. Plan 247 still expressed ADR-0015 milestone-local autonomy in present/future
   tense. After the first Plan recut, two clauses in the live `AGENTS.md` also
   retained that stale post-close authority. The closeout cut recast both
   surfaces as authority that applied only while M0--M10 was active, ended at
   M10 close, and does not authorize post-program work; ADR-0014 or new owner
   direction governs as applicable after close.

After validating this complete forward-only finding record, the final planner
disposition was **ACCEPT with no remaining P0/P1/P2**. Post-program direction
remains owner-defined.

## Skipped validations and reasons

The final closeout reran the full workspace all-target suite, runtime Clippy,
formatting, the seven trusted Lean foundation checks, placeholder scan,
documentation/Canon/agent validators, and current-checkout double conformance
digest. It did not create a second post-close fresh remote clone because the
accepted immutable R5 fresh-clone reproduction already exists and no source or
test file changed after R5. The earlier full generated all-theorem axiom audit
was not regenerated; the final Lean commands printed the selected dependency
inventory again. Neither skipped item is claimed as a new pass.

## Commit / push status

No commit or push performed by this delegated closeout agent. The accepted R5
source cut is already pushed at `23f5a8130334bf0c8516d51e9dcea38b92f50db1`.
This status synchronization remains an uncommitted working-tree change for the
parent/integration owner.

## Sub-agent session close status

This delegated closeout session edited only the six authorized LAB closeout
files. No additional sub-agent was spawned from this session. Prior R5 reviewer
ACCEPT is recorded as supplied evidence.
