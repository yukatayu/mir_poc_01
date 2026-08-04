# Report 2589 — Mir Theory v0 / I1+ Milestone 8: Deterministic runtime

- Date: 2026-08-04 (started after M7 parity at 19:20 JST)
- Author / agent: parent orchestrator with independent planner, theory/formalization, production implementer, test-author, and reviewer roles
- Scope: ADR-0015 M8 only — deterministic single-process logical multi-locus runtime consuming the M7 checked artifact
- Start revision: `420432e1f2507cbd475ea6bd327b580fc6376571`

## Objective

Build the bounded M8 reference runtime whose only source-program input is `CheckedSurfaceV0`, with explicit typed runtime admission, one unified semantic state, deterministic owner service, relation/designated behavior, occurrence/dependency/authority/failure/observation trace, local cut/save/load, bounded checked patch, and observer-safe export.

## Scope and assumptions

M7 is fixed input and is not reparsed or reconstructed inside the runtime. Runtime admission may explicitly validate and record the base visibility, lifetime, fallback, and redaction evidence needed to resolve M7 relation/designated residuals. `AuthDeferred` and `VerifyDeferred` remain non-success until M9. M8 is single-process and logical multi-locus; it does not add sockets, distributed transactions, exactly-once transport, final public ABI/wire, or production deployment.

## Start state / dirty state

M8 started clean with `HEAD == origin/main == 420432e1f2507cbd475ea6bd327b580fc6376571`. Plan 247 names M8 as the sole active semantic frontier and M9 as next. The root filesystem had about 7.2 GiB available; `/mnt/mirrorea-work` was not mounted. No cleanup or destructive action was taken.

## Documents consulted

Canon-first reading includes ADR-0015, ADR-0020, ADR-0022, specs 03/04/08, theories 13--16, the proof ledger, Plan 247, and Report 2588. Current LAB snapshots were read only for the M8 frontier. Existing alpha/current-L2/full-system runtimes are LAB evidence and are not authoritative runtime inputs.

## Actions taken

1. Obtained independent Canon-first M8 pre-edit review.
2. Audited the M7/M5/runtime implementation seam and the M8 formal/Lean seam without editing production files.
3. Selected one M8 façade over one unified M5-shaped semantic state; rejected aggregation of M3/M4/legacy runtime harnesses.
4. Identified explicit runtime residual admission as the first safety gate and opened this sole M8 report.
5. Conservatively extended the M7 artifact with a structural checked-program identity, enumerable checked evaluations, checked static environment, residual rows, and stable source references.  M8 admission binds every evidence row to that identity and defers M9 auth/verification rows without semantic success.
6. Added source-map-ordered lowering and a typed owner FIFO runtime.  Owner requests carry uses of already-admitted membership/capability/witness records; service-time RMW serializes the canonical two attacks as `100 -> 90 -> 80`, while failed validation and cross-owner operands do not mutate state.
7. Added the owner-held maintained-relation runtime.  It projects at consumer C from one presentation frontier without an absolute-value stream, distinguishes semantic and presentation fallback, rejects split/stale presentation samples, advances fallback monotonically, and requires a fresh epoch/witness-backed reacquire lineage.
8. Added the designated-evaluator runtime.  It consumes an explicit source-bound input receipt, validates evaluator authority before receipt provenance/value access, retains tick/frontier/version/policy/stamp, publishes once, makes duplicate evaluation idempotent, and gives the consumer a one-shot delivery without semantic re-evaluation.
9. Adopted Proposal 026 / ADR-0023, spec 09, and theory 17.  Added the finite M8 Lean model and OBL-050--057 ledger rows; OBL-050--056 are exact finite Lean proofs and OBL-057 remains the Rust correspondence/runtime-monitoring target.
10. Fixed two self-review counterexamples before continuing: trace nodes now have distinct IDs with strict earlier dependencies rather than self-edges, and authority checks consult a pre-admitted inventory rather than accepting constructed strings or provider names as grants.
11. Added one unified local runtime session and local-cut carrier.  The owner, relation, and designated transition façades move one shared semantic snapshot through each operation rather than reconciling independently seeded snapshots.  The cut retains admission provenance, authority/lease inventory, owner FIFO and counters, relation lineage, designated receipt/result/version/consumption, patch lifecycle, and trace prefix while excluding presentation state.
12. Added restore validation against an external live floor.  Same-current restore resumes the pending second owner request deterministically; stale membership/capability/witness/lease, consumed delivery rollback, result-version rollback, and old relation-lineage rollback reject before payload replacement and add only outside-cut failure evidence.
13. Added initial bounded patch and observer-safe export implementations.  The patch route revalidates checked/admission structure and records a single-session activation cut; reject/defer paths preserve non-lifecycle semantic state.  Observer export requires typed principal/authority/label/redaction/retention/reason/proof policy and erases private value/authority/witness payloads.
14. Found and corrected an authority-ordering information leak in designated consumption: an unauthorized consumer can no longer distinguish unpublished from published state because authority validation now precedes result-store lookup.  The focused indistinguishability regression passes.
15. Ran the independent final review once.  It found no P0 and four P1 counterexamples in relation evidence enforcement, inventory-bound reacquire, unified failure history, and actual checked-plan installation at patch activation.  M8 remained open and OBL-057 remained `intentionally-deferred`.
16. Added a test-only correction RED batch without changing production semantics.  The batch requires exact retained relation evidence payloads and live leases, rejects a forged reacquire witness without mutation, requires typed component failures in saved unified `H`, and requires a quiescent accepted patch to install and execute the candidate checked plan in the same local session and local-cut carrier.
17. Corrected the finite Canon/Lean model without broadening the milestone: OBL-050 now carries exact relation/designated payloads, OBL-051/052 use one typed success/authority/witness/failure history, OBL-053 rejects forged/reused witness evidence and installs the declared fresh epoch/lineage, OBL-054 saves the active plan and history, OBL-055 installs one checked candidate atomically at a local cut, and OBL-056 erases raw authority/capability/witness payloads.  OBL-057 remains deferred pending the matching Rust GREEN evidence.
18. Made the first correction batch GREEN: exact evidence fields are retained, the declared relation lease is gated, witness identity is inventory-bound, unified failure rows are saved, and accepted patch activation installs the candidate plan in the same local runtime.  Fresh focused/full Rust and trusted Lean validations passed.
19. The narrow correction re-review accepted the actual patch-plan installation and saved failure retention, but found four adjacent P1 falsifiers: missing/inexact lease inventory was still fail-open, presentation fallback omitted admitted privacy, the raw failure-row API remained publicly exposed while observer export was marker-only, and re-acquire epoch/frontier were not bound to admitted fresh evidence.  It also found one P2 duplicate-evidence order dependency.  OBL-057 therefore remains deferred and the permitted second correction pass is active.
20. Added the second test-only RED batch.  It requires deterministic duplicate/conflicting residual-evidence rejection, private-policy preservation on consumer-local fallback, exact inventory-bound fresh lease/epoch/frontier re-acquire, a missing/expired/mismatched lease matrix, and privileged raw history versus typed redacted observer rows.  The existing patch correction remains GREEN 8/8.
21. Made the second correction batch GREEN and reran the full validation stack.  The final bounded review confirmed exact operation-time lease checks, private fallback, duplicate/conflict rejection, fresh lease/epoch/frontier binding, and patch coherence, but found two remaining statement/API mismatches: Canon/Lean incorrectly placed dynamic live-inventory validation at admission rather than each relation operation, and observer failure rows omitted explicit label/redaction while a test-only raw privilege remained publicly constructible.  M8 remains open for this final local alignment; OBL-057 remains deferred.
22. Closed the final statement/API alignment.  Admission now validates the declared lease reference/frontier/epoch payload while projection, transition, and re-acquire each recheck the current exact live inventory.  Raw authority/witness/failure history is crate-internal; public observer failure rows retain source/occurrence/dependency/failure structure plus typed `Private` label and structural-only redaction.  The final fresh validation matrix is GREEN and supplies the bounded Rust correspondence evidence for OBL-057.

## Files changed

- `crates/mir-semantics/src/surface_v0_pipeline.rs`
- `crates/mir-runtime/src/m8_runtime_{admission,authority,owner_queue,relation_projection,designated_value}.rs`
- `crates/mir-runtime/src/m8_runtime_{local_cut,patch,observer}.rs`
- `crates/mir-runtime/tests/m8_runtime_{admission,owner_queue,relation_projection,designated_value,local_cut,patch,observer}.rs`
- `crates/mir-ast/tests/fixtures/surface-v0/m8_unified_local_cut_no_m9_residuals.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/m8_unified_patch_designated_plus_two.mir`
- `mirrorea_canon/meta/proposals/PROPOSAL-026-m8-deterministic-runtime-admission.md`
- `mirrorea_canon/adr/ADR-0023.md`
- `mirrorea_canon/spec/09-m8-deterministic-runtime.md`
- `mirrorea_canon/theory/17-m8-deterministic-runtime.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md` and navigation/index files
- `samples/lean/foundations/MirTheoryV0M8DeterministicRuntime.{lean,md}`
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`
- `docs/reports/2589-mir-theory-v0-i1plus-milestone-8-deterministic-runtime.md`

## Commands run

- Clean start, remote parity, disk/memory, block-device, and external-workdir checks.
- Canon-first M8 acceptance reading.
- Read-only planner, implementation-seam, and formal-seam audits.
- Focused admission, owner queue, relation projection, and designated evaluator Rust tests.
- `cargo test -p mir-semantics`
- `cargo test -p mir-runtime`
- `cargo clippy -p mir-semantics -p mir-runtime --all-targets -- -D warnings`
- `cargo fmt --all -- --check`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M8DeterministicRuntime.lean`
- Canon index, source-hierarchy, documentation, forbidden-proof-token, and whitespace validation.

## Evidence / outputs / test results

M7 payload/closeout revision `420432e1f2507cbd475ea6bd327b580fc6376571` is the accepted input.  The following focused evidence is complete at this intermediate report cut:

- M8 admission: 7/7 tests pass.
- owner FIFO / RMW / authority / replay: 6/6 tests pass.
- maintained relation / consumer-local projection / fallback: 10/10 tests pass.
- designated evaluator / version / one-shot delivery: 6/6 tests pass.
- unified local cut / no-stale restore / typed failure observation: 11/11 tests pass.
- bounded checked patch lifecycle: 8/8 tests pass.
- initial observer-safe export: 4/4 tests pass.
- designated evaluator after the consumer-authority ordering correction: 7/7 tests pass.
- full `mir-semantics` and `mir-runtime` tests pass at the Phase 3 cut.
- targeted Clippy for both changed crates, Rustfmt, and `git diff --check` pass at the Phase 3 cut.
- the final M8 Lean file compiles with `--trust=0`; all 28 printed theorem inventories report no axiom dependency, and the scoped `sorry` / `admit` / user-axiom / proof-stub scan is clean.
- Canon index validation passes with 163 documents; source hierarchy validation passes 798/798; documentation validation passes.

Patch-specific activation capability, incomplete-candidate admission rejection, actual candidate-plan installation, quiescent activation, candidate-cut restore, observer trace correspondence, and unified deterministic replay pass their focused tests.  All independent-review findings listed below have been corrected and revalidated.  Official SCN-01..10 C-static/C-runtime conformance remains M10 work.

The correction RED batch has the expected evidence boundary:

- admission does not yet compile because `RelationEvidencePayloadMismatch` and a retained relation-plan evidence accessor do not exist;
- relation projection compiles with 6 passing and 2 failing tests: a private admitted policy can still be weakened and a forged fresh-witness string still succeeds;
- local-cut tests do not yet compile because live-relation-lease diagnostics and a typed `failure_rows()` unified-history API do not exist;
- patch tests do not yet compile because candidate receipt binding, current active admission, candidate designated execution, patch-local save/restore, owner-FIFO quiescence exposure, and `NonQuiescentSession` rejection do not yet exist.

Rustfmt and whitespace validation pass for this test-only RED tranche.  These expected failures are not validation passes; they are the executable falsifiers for the active correction pass.

The corrected finite Lean file compiles with `--trust=0`; 22 selected theorem axiom checks are empty, the scoped forbidden-stub scan is clean, and the three newly introduced core theorem names pass the import check.  This is formal evidence for the corrected finite statements, not yet an implementation-correspondence claim; OBL-057 therefore remains deferred until the Rust falsifiers are GREEN.

The first correction implementation reached GREEN on 47 focused M8 tests, full `mir-runtime`, full `mir-semantics`, targeted all-target Clippy with `-D warnings`, Rustfmt, and whitespace validation.  The correction re-review then supplied new direct counterexamples rather than accepting this test count as correspondence proof.  In particular, it confirmed the checked candidate `+2` plan is actually installed and survives candidate-cut restore, while keeping OBL-057 deferred for the remaining lease/privacy/observer/re-acquire gaps.

## What changed in understanding

M8 cannot treat `CheckedSurfaceV0::execution_is_admissible()` as a blanket runtime gate because relation/designated residuals are deliberate M8 handoffs. The safe finite design is a typed `RuntimeAdmissionEvidence → AdmittedProgram` transition that leaves the M7 artifact immutable, resolves only base runtime evidence, and keeps M9 auth/verify residuals deferred. M8 also needs a stable checked static environment/program identity and enumerable evaluations so patch compatibility and lowering do not reparse source or create hidden name side tables.

The component runners are useful focused façades but cannot themselves be the save/load boundary: separately seeded owner, relation, and designated snapshots would duplicate and reconcile authority state after the fact.  The selected local-cut route therefore owns one admitted instance and one extracted semantic state, includes queues/counters/lineages/versions/consumption/provenance in the cut, and keeps presentation-only contexts outside it.  A live non-rollback floor is external to the cut and must be validated before any replacement.

## Open questions

No owner decision is required.  M8's direct blockers are closed; the next active frontier is M9 typed auth/verification extension.  Reopen M8 only if M9 integration falsifies the checked-artifact/runtime boundary or M10 release conformance exposes a source-to-runtime correspondence failure.

## Suggested next prompt

No prompt is required. Continue autonomously into M9 typed auth/verification extension after M8 commit/push parity.

## Plan update status

更新済み: Plan 247 marks M8 closed, M9 as the sole active semantic frontier, and M10 as next.

## Documentation.md update status

更新済み: It records the bounded M8 runtime/formal evidence and points current work to M9.

## docs/project-status.md update status

更新済み: It records M8 close evidence, OBL-057's bounded `runtime-monitored` status, and M9 as current.

## progress.md update status

更新済み: The macro/feature axes and timestamped recent log record M8 close and M9 entry.

## tasks.md update status

更新済み: The task map was rewritten with M9's typed auth/verification package as the direct critical path and M10 next.

## samples_progress.md update status

更新済み: M8's Rust/Lean fixture matrix is recorded as bounded evidence, not a public runnable sample or official release-conformance claim.

## Reviewer findings and follow-up

The independent pre-edit planner requires `CheckedSurfaceV0` as the only source-program input; explicit source-bound residual admission; one state `K8 = ⟨AdmittedProgram,H,Q,S,M,G,W,L,R,D,J,X⟩`; presentation context outside semantic/save state; owner FIFO seriality; relation owner-only mutation; designated version/consumption state; no stale restore; patch no-mutation on reject/defer; and observer-safe rather than raw trace export. The implementation and formal audits agree that M7 needs conservative enumerable/static-environment evidence and M5 needs only the minimum runtime-facing extensions.  Self-review corrected a trace self-edge, constructed authority rows, an auth-provider name used as a capability, designated-consumer publication-existence leakage, a patch activation-only false positive, and fabricated observer occurrence/dependency markers.

The one independent final review then found no P0 and four P1 findings.  These findings remain part of the forward record and M8 is not closed on this intermediate cut:

1. relation admission checked evidence kind/name/source reference but did not enforce or install visibility/redaction, lease/frontier, and fallback-epoch payloads;
2. accepted patch changed an outer active identity marker without installing candidate plans/admission into the one local session or making post-patch local cut/restore coherent;
3. unified local history filtered component failure rows and therefore did not preserve the required typed failure/authority/witness facts in one saved `H`;
4. relation reacquire accepted an arbitrary nonempty caller string as a fresh witness instead of resolving it against the admitted witness inventory and new lineage.

OBL-057 therefore remains `intentionally-deferred` at this cut.  Test-author, production, and formalization correction work is active; the final status will record exact RED/GREEN evidence and a narrow correction re-review rather than erasing these findings.

The narrow correction re-review found the first P1 set substantially repaired and the patch correction correct, then produced the following second-pass findings:

1. live-lease validation accepted a missing inventory and did not exact-match relation, owner, frontier, and epoch;
2. consumer-local presentation fallback did not join/retain the admitted private label/redaction;
3. raw unified failure rows remained publicly accessible while observer export did not yet carry the typed structural row required by OBL-056;
4. re-acquire witness identity was inventory-bound, but the installed epoch/frontier remained arbitrary caller inputs;
5. duplicate/conflicting relation evidence could make admission depend on row order (P2).

These are bounded same-frontier corrections, not owner-level design alternatives.  The permitted second correction pass uses executable falsifiers for all five and keeps OBL-057 deferred until they pass and the final narrow review finds no new correctness blocker.

The second RED batch fails at the intended pre-implementation boundaries: admission lacks duplicate/conflict diagnostics; relation projection has 8 passing and 2 failing cases for private fallback and forged epoch/frontier; local-cut compilation lacks privileged raw-history access and typed `observer_failure_rows()`; patch remains 8/8 GREEN.  Test Rustfmt and whitespace checks pass.  These RED results are not counted as completed validation.

The second correction became GREEN on 53 focused M8 tests and the full Rust/Lean/Canon validation stack.  The final bounded review found no further patch, privacy-fallback, duplicate-evidence, or lease-operation counterexample.  It did identify that OBL-050's prose/Lean carrier overclaimed dynamic live-inventory validation as an admission prerequisite even though the Rust route correctly rechecks liveness at relation operation time, and that the redacted failure-row carrier lacked label/redaction while its raw test privilege was forgeable.  The selected correction moves exact live-inventory validation to OBL-053 operation steps, retains only declared lease payload validation in OBL-050 admission, adds label/redaction to the observer row, and closes raw `H` inspection to the crate boundary.

The final local alignment is complete: admission and operation-time lease claims now match; observer failure rows retain typed label/redaction; raw failure/authority/witness carriers are not public; the public-surface absence and observer-field scans pass.  OBL-057 is therefore `runtime-monitored` for this exact 53-test/source-bound fixture correspondence only.  It is not a general correspondence theorem, M9 proof, M10 official conformance result, or public API/ABI/wire claim.  No reviewer finding remains uncorrected.

## Skipped validations and reasons

M9 auth/verification and M10 official release conformance are later milestones and are not M8 claims.  Workspace-wide Clippy is not claimed: an unrelated pre-existing CLI boolean-assert lint exists, while both changed crates pass targeted all-target `-D warnings`.  A first scoped secret-scan command had a shell-quoting operator error; the corrected docs validation/secret scan passed and exposed no secret.  No required M8 validation remains skipped.

## Commit / push status

The M8 implementation/formal/status closeout is commit `588f2f4204b2f11025b77e80eec5441b4bc0e10c`, pushed to `origin/main`.  Post-push fetch verified `HEAD == origin/main == 588f2f4204b2f11025b77e80eec5441b4bc0e10c` with a clean worktree.  This report-status correction is the final report-only closeout commit and is pushed immediately after creation; no production, proof, or status meaning changes in that correction.

## Sub-agent session close status

- M8 pre-edit planner: complete, read-only.
- M8 implementation-seam audit: complete, read-only.
- M8 test-author correction tranche: complete; final focused matrix is 53/53 GREEN.
- M8 production correction tranche: complete.
- M8 formalization / Canon / Lean correction tranche: complete with trusted finite proofs.
- M8 independent reviewer: complete; every reported P1/P2 has a recorded correction and fresh validation.
