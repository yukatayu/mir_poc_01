# Mirrorea I3-3 failure and ordering refinement — ongoing milestone report

## Title and identifier

I3-3 / Report 2606. Opened 2026-09-07 11:37 JST, obtained from
`date --iso-8601=seconds` (`2026-09-07T11:37:54+09:00`). Status: ongoing;
I3-3 is active and has not been accepted. This single report accumulates the
resume, implementation, validation and eventual milestone review; no separate
resume report is created.

## Objective

Execute every one of the 20 Canon plan/05 network failure families and their
controls on the accepted source-derived multi-process runtime, with explicit
request-bound retry/ambiguity, Mir ordering refinement, owner preservation,
typed/redacted evidence and bounded termination. I3-4 is the direct consumer.
The original Plan 250 parent objective and sequence through NEXT-0 are unchanged.

## Scope and assumptions

The explicit owner instruction resumes the ADR-0034 program at I3-3 only.
PROPOSAL-043 / ADR-0040 record execution control; ADR-0039 and Report 2605
remain immutable I3-2 acceptance/pause history. Later milestones stay inactive
and dependency-gated. The accepted source/evidence cut is
`19c5b386613d6adb1f0b934e6ced81acb327d245`; the existing private QUIC reliable
stream and generated process images are inputs.

Semantic S4 over replaceable S6, project/product PL-2 consuming PL-1/PL-0,
and lifecycle are separate axes. Theory T1, broad PHASE-I1 unaccepted,
official I2 exit accepted and official I3 unentered are preserved. No general
proof/ledger promotion, public wire/API/ABI freeze, production/WAN guarantee,
Browser/Host implementation or upper-domain/Core promotion follows.

Duplicate handling is operation-specific: accepted stored-result/no-new-consume
behavior or typed duplicate rejection. Post-admission uncertainty is explicit
and bound to the original request; no blind retry, false success, universal
retry contract or global exactly-once is allowed. Every failure family must
reach a meaningful runtime/admission/adapter boundary; harness labels alone
are not execution. The cut/patch/save family may exercise a positive quiescent
admission boundary and in-flight/late-traffic rejection without implementing
durability or live distributed patch.

## Start state / dirty state

Resume baseline: `648425f6bd4304d003d36bc04d346ddf0e78c058`.
The parent reported the baseline clean; this writer independently observed
an empty `git status --short` and matching `git rev-parse HEAD` /
`git rev-parse origin/main` at that revision. This is a local remote-tracking
parity check; milestone close still requires the parent's fresh remote check.

Parent and bounded agents share the worktree. This writer owns only Plan 250
and this new report and preserves other agents' edits. Parent-supplied resource
baseline: root approximately 15 GiB free, configured external work mount absent,
and no orphan children after focused baseline execution. No heavy builds,
artifact allocation or cleanup are performed by this planning assignment.

## Documents consulted

Operational instructions: AGENTS.md. Normative: Canon README/MAP, NORTH-STAR,
architecture/01-strata, architecture/06-project-product-layers,
architecture/10-i3-multi-process-runtime, ADR-0034, ADR-0039, PROPOSAL-042,
and plan/05-i3-entry-contract. The parent supplies the narrow resume authority
in `mirrorea_canon/meta/proposals/PROPOSAL-043-mirrorea-i3-program-resume.md`
and `mirrorea_canon/adr/ADR-0040.md`; no network semantics are accepted by it.

LAB: README.md, Documentation.md, progress.md, tasks.md, samples_progress.md,
.docs/progress-task-axes.md, plan/00-index.md, Plan 250, Report 2605,
docs/reports/TEMPLATE.md and the report validation helpers in
scripts/validate_docs.py. Historical plans/reports are evidence, not authority.

## Actions taken

- Preserved the parent Goal Statement, fixed milestone sequence, all 20 failure
  rows and the full ordering inventory in Plan 250.
- Mirrored the explicit resume cut and I3-3-only activation; refreshed source
  inputs through accepted architecture/10, PROPOSAL-042 / ADR-0039 / Report
  2605 and the subsequent resume control record.
- Added the complete I3-3 Goal Statement, ordered work phases, ownership,
  acceptance evidence, unresolved questions and stop/reopen rules.
- Opened this single ongoing report. No I3-3 implementation, failure-family
  coverage or acceptance is claimed at the planning checkpoint.

Subsequent parent-reported implementation checkpoint: the test owner added
`i3_3_duplicate_source_generated_owner_request_rejects_without_a_second_owner_serve_or_write`
and observed the expected failing baseline. Parent approved LAB implementation
candidate B for this owner-request boundary: bounded owner seen-ledger keyed
with exact carrier fingerprint, no eviction/reset, and a tombstone recorded
before potential mutation and retained even if later execution errors. A
duplicate returns typed `DuplicateRequestRejected`; successful stored-result
recovery is not claimed. This candidate is under implementation/review and is
not a Canon adoption or a universal duplicate policy. Accepted operation-specific
stored-result/no-new-consume contracts remain preserved.

## Files changed

Resume integration:

- `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`
- `docs/reports/2606-mirrorea-i3-distributed-foundation-i3-3-failure-ordering.md`
- `AGENTS.md`
- `CANON.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/project-status.md`
- `plan/00-index.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- `mirrorea_canon/meta/proposals/PROPOSAL-043-mirrorea-i3-program-resume.md`
- `mirrorea_canon/adr/ADR-0040.md`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/architecture/06-project-product-layers.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/meta/style-guide.md`
- `mirrorea_canon/plan/README.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/plan/05-i3-entry-contract.md`

Actual test work subsequently reported by the parent:

- `crates/mir-runtime/tests/sys5_i3_process_runtime.rs`
- `crates/mir-runtime/src/sys5_i3_process_runtime.rs` (implementation in progress)
- `crates/mirrorea-i3-probe/tests/i3_process_localnet.rs` (new fault contracts)

This is the ongoing task-wide inventory, not an accepted source cut. Source,
test and status writers have distinct ownership; parent owns integration.

## Commands run

This writer ran read-only source/status inspections with `rg`, `sed`, `head`,
`cat`, `git status --short`, `git rev-parse HEAD`, `git rev-parse origin/main`
and `date --iso-8601=seconds`; edits used `apply_patch`.

The parent supplied these completed fresh baseline commands at the resume cut;
they were not rerun by the planning writer:

```bash
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet -- --test-threads=1
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mirrorea-i3-probe --tests -- --test-threads=1
```

The parent subsequently reported this first failing duplicate-request test:

```bash
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test -p mir-runtime --test sys5_i3_process_runtime --features i3-process-test-seams i3_3_duplicate_source_generated_owner_request_rejects_without_a_second_owner_serve_or_write -- --exact
```

Planning validation ran `git diff --check` and an inline Python check using
`scripts.validate_docs` report helpers plus exact `git show` comparisons
against the pinned resume cut. `python3 scripts/validate_docs.py` is the
repository-wide documentation check. Milestone runtime/model commands and
final regression evidence will accumulate here as I3-3 proceeds.

## Evidence / outputs / test results

Parent-reported resume baseline: focused localnet 12/12 and full probe package
62/62 passed at `648425f6bd4304d003d36bc04d346ddf0e78c058`; no orphan children.
These are fresh accepted-I3-2 regression checks, classified `runtime-monitored`,
not I3-3 acceptance or complete network-failure coverage.

Planning checks passed: `git diff --check`; exact preservation of the parent
Goal Statement, all 20 failure-family rows, the complete ordering inventory and
all I3-4 through NEXT-0 contracts; all 22 report sections in order, nonempty and
without duplicate headings or unresolved template placeholders; valid scoped
project-status update declaration. These are documentation checks only.

Resume checks: Canon index generation/check passed with 210 indexed files;
`python3 scripts/validate_docs.py` exited 0 (1760 numbered reports). The
overview test first exposed three stale pause-era assertions; after the test
owner updated only current-state expectations,
`python3 -m unittest scripts/tests/test_mirrorea_project_overview_html.py`
passed all 12 tests. Historical pause records were retained. The parent's
integrated `make docs` also exited 0: agent configuration, Canon index,
800/800 hierarchy paths and the 1760-report documentation scaffold passed.
This validates resume integration only, not I3-3 runtime acceptance.

First RED result, supplied by the parent: 1 failed, 47 filtered out, compilation
1.74 s. The existing endpoint returned a second `SomeReply` rather than the
expected typed duplicate rejection, exposing the missing owner-request dedup
guard. This is a reproduced failing baseline, not passing implementation or
milestone acceptance. Source and test owners are working on the bounded fix.

The I3-3 20-family execution inventory, operation-specific retry/ambiguity
results, ordering evidence and full exit checks are not yet established.
No new `lean-proved`, `lean-stated` or `model-checked-bounded` result is claimed.
Any later finite model must state its bounds and remain separate from runtime
evidence; unexecuted/deferred checks cannot be counted as passing rows.

## What changed in understanding

The owner pause has been explicitly lifted without changing the program goal
or reopening accepted I3-2. The accepted actual-process seam is now I3-3's
input. A post-admission disconnect may follow a real owner mutation and cannot
be classified as nonmutation merely because delivery failed. All 20 families
need real boundary evidence; generic fault labels do not close that gap.

## Open questions

UNRESOLVED within I3-3: exact per-operation stored-decision versus typed
duplicate behavior, explicit observer-safe ambiguity evidence across sessions,
dependency/frontier rules for reordered traffic, and minimal real controls for
authority/provider/time/quiescent-cut rows. The accepted operation contracts
govern; no universal retry policy is selected by this report.

The parent selected LAB candidate B above for the current owner-request
duplicate seam. Oracle one-off consultation `i3-3-b-ledger-boundary` completed
normally in 11m23s with GPT-6 Pro verified (one consultation, no duplicate).
It supports B for owner RMW, non-evicting bounded tombstones, current admission
before duplicate classification, and separate original-operation versus retry-
attempt outcomes. Its advice is not Canon or executed assurance.

The parent checked that advice against the omitted full designated-consumer
contract: spec/13 and ADR-0030 already require exact designated-result retry to
return its stored typed decision without new consumption. That existing
operation-specific behavior remains required; B is not a universal policy.
The local M9-admitted administrative lifecycle has distinct, non-Surface
provenance and may withdraw existing authority; fault configuration may
schedule around that real transition but cannot manufacture a grant or revoke.
No new Surface syntax is inferred merely to represent external stimuli.

An implementation counterexample refined rejection precedence: actual M9
revocation advances generation, so an old carrier fails the existing exact
lineage-binding gate before a capability check. That fail-closed rule is
preserved; tests must not demand a later `MissingCapability` diagnostic or
widen historical-lineage admission just to reach it. Current-bound authority
revalidation and actual-network withdrawal remain separate evidence work.
Correctness, capacity, exact binding and all fault rows still require final
execution and independent review before acceptance.

Owner decisions are required only at the existing ADR-0034 reserved stops:
guarantee/North-Star weakening, unavoidable hidden retry/transaction, public
freeze, irreversible semantic tie, both retained transports failing required
criteria, or protected production/resource/security scope. Missing later
features and general proofs are not themselves stop conditions.

## Suggested next prompt

Continue active I3-3 in the original Plan 250 program: map all 20 failure
families and ordering edges to accepted runtime boundaries, resolve operation-
specific duplicate/ambiguity handling, and implement/validate bounded real fault
controls. Keep I3-4 inactive until I3-3 acceptance and remote parity close.

## Plan update status

`plan/` 更新済み: Plan 250 now records the resume cut, complete active I3-3
Goal Statement and work/evidence/decision gates. The parent goal, all failure
rows, ordering edges and later milestone order are preserved. plan/00-index
synchronization belongs to the status assignment.

## Documentation.md update status

更新済み: resume pointer now identifies I3-3 as the sole active milestone,
with the accepted I3-2 runtime retained as its input.

## docs/project-status.md update status

更新済み: owner resume、I3-3 sole active、後続 milestone inactive を同期した。
I3-2 accepted evidence と official I3 unentered を維持する。

## progress.md update status

更新済み: current I3-3 pointer, preserved three axes and startability mirror
the resume; the actual-time recent log is 2026-09-07 11:38 JST.

## tasks.md update status

更新済み: active I3-3 and inactive later packages are separated from accepted
I3-2 history. The parent-requested current-goal/direct-consumer coherence
correction is applied: I3-4 consumes I3-3 and resume is resolved history.

## samples_progress.md update status

更新済み: the sample dashboard records resumed I3-3 as active/not accepted,
with all 20 families explicitly a target. Accepted I3-2 runnable evidence
retains its existing class. No new workflow-ready or product-completion claim.

## Reviewer findings and follow-up

The parent reports completed independent Canon-first pre-edit planner review at
`648425f6bd4304d003d36bc04d346ddf0e78c058`. Its requirements are incorporated:
exact resume, complete Goal Statement, all 20 rows, preserved operation-specific
retry, request-bound ambiguity, fixed sequence and historical ADR-0039.
An independent final reviewer will assess this writer's changes; this writer
does not self-accept the milestone. Final findings and dispositions must be
recorded before acceptance.

## Skipped validations and reasons

The planning writer runs no builds or runtime tests: the assignment is limited
to two planning/report files and the parent already supplied focused baseline
results. Full I3-3 fault/order coverage, I2/M10 close regression, independent
milestone acceptance, final remote parity and any bounded model are still
unexecuted for I3-3. They are required as applicable before milestone close,
not skipped passes. General proof, WAN/production, durability and Browser/Host
product realization remain outside this bounded milestone.

## Commit / push status

No commit or push by this writer; explicitly outside the delegated assignment.
Baseline local `HEAD` and `origin/main` match the resume cut. Parent owns
accepted source/evidence pinning, integration commits, authorized pushes and
fresh remote parity before the fixed transition to I3-4.

## Sub-agent session close status

Planning writer hands both planning/report files back to the parent after
scoped validation. Source and test owners remain active. Other agents' work is
preserved; parent retains active-goal integration and final acceptance.
I3-3 remains ongoing when this writer returns; no user-task completion or
milestone close is asserted by the sub-agent handoff.
