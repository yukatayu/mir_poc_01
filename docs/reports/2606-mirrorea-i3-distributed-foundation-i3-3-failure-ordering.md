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

Crash recovery at `2026-09-07T12:32:34+09:00` (fresh `date -Iseconds`):
the resume integration is committed at
`48d98ed0279cf6fa2a0eaabd7d1ec21c926c7aec`; fresh `git ls-remote origin
refs/heads/main` matched HEAD. Uncommitted runtime, probe and test changes
survived. No previous trial/build processes remained and the old agent sessions
were unavailable; replacement specialists resumed with disjoint ownership.
Root free space was approximately 17 GiB, with no configured external mount.
No cleanup or user-change discard occurred.

The recovery test assignment then ran a package-wide `cargo test --locked
-p mir-runtime --features i3-process-test-seams --no-run` rather than the
intended narrowly selected test target. Free root space fell to 6.3 GiB;
the parent stopped additional builds. Read-only inspection found `target`
43 GiB, including `target/debug/incremental` 12 GiB, `target/debug/deps`
28 GiB and `target/debug/build` 81 MiB. The parent requested explicit owner
permission to remove only the regenerable incremental directory; no deletion
has been performed at this checkpoint. Existing compiled tests may be run
without rebuilding; source work continues under the resource guard.

Owner-authorized cleanup completed at `2026-09-07T13:25:58+09:00`
(fresh `date -Iseconds`). The parent verified that the exact
`target/debug/incremental` directory and its parents were not symlinks and
that no Cargo/rustc process remained, then removed only that regenerable
directory. Source, Git history, and existing test binaries were preserved.
`df -h .` reported 18 GiB available afterward; no other cleanup was authorized
or performed. Targeted validation resumed with `CARGO_INCREMENTAL=0`,
`CARGO_BUILD_JOBS=2`, `--locked`, and one Cargo builder.

The latest-source `mir-runtime` process-runtime integration target compiled
successfully and ran 50 tests: 49 passed and one failed. The owner-capacity
test could not emit its 65th request because the newly bounded requester
ledger returned `OutboundRequestLedgerExhausted` first. This is an unresolved
test-path interaction, not passing evidence for owner overflow, and neither
capacity guarantee is relaxed. Disk remained at 18 GiB available. The test
owner investigates the distinct boundaries while the evaluator checks the
remaining independently selected targets; I3-3 remains unaccepted.

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
- `crates/mir-runtime/src/sys5_i3_private_quic.rs` (bounded retry/session evidence)
- `crates/mirrorea-i3-probe/tests/i3_process_localnet.rs` (new fault contracts)
- `crates/mir-runtime/src/m9_auth_verification.rs`
- `crates/mir-runtime/src/sys4_dispatch.rs`
- `crates/mir-runtime/tests/i3_request_lifecycle_model.rs`
- `crates/mirrorea-i3-probe/src/i3_process_localnet.rs`
- `crates/mirrorea-i3-probe/src/i3_process_faults.rs`
- `crates/mirrorea-i3-probe/src/lib.rs`

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
No new `lean-proved` or `lean-stated` result is claimed. Following crash
recovery, the test owner compiled the standalone std-only lifecycle model with
`rustc --edition=2021 --test crates/mir-runtime/tests/i3_request_lifecycle_model.rs
-o /tmp/mir-i3-model-J2zA0R/model_test` and executed it: 2/2 tests passed,
432 states and 2328 transitions explored for the initial positive model. All three
mutants (remove dedup, clear ledger on reconnect, renew grant on reconnect)
produced their expected counterexamples. The prior missing lifetime annotation
was corrected before this successful compile. Bounds are one owner, two
semantic identities, two attempts per identity, one withdrawal, one disconnect/
reconnect and ledger capacity one. This is `model-checked-bounded` evidence
for the stated abstract model only; reconnect and withdrawal correspondence
remain implementation targets, not proven concrete runtime/network behavior.
No Canon assurance ledger promotion follows. Unexecuted/deferred checks cannot
be counted as passing rows.

Recovery execution of the newly available
`target/debug/deps/sys5_i3_process_runtime-6ed64992515d5698` binary passed the
exact duplicate rejection test and the exact 64-entry capacity/no-eviction
test, 1/1 each (48 filtered out), with `--exact --nocapture --test-threads=1`.
The Cargo-generated `i3_request_lifecycle_model-5893a4f85fa139c3` also passed
2/2 with the same finite exploration counts. The retained-publisher internal
test then exposed an incorrect test expectation: fresh binding rejects the
old lineage, whereas a deliberately retained old binding passed directly to
the pure authority helper correctly returns `MissingCapability`. The test
was corrected to distinguish both gates and assert unchanged use-audit count;
that corrected test had not been rebuilt at the disk-guard checkpoint; its
subsequent passing execution is recorded below.

The same compiled runtime integration binary subsequently passed its full
49-test inventory: exit 0, 49 passed / 0 failed, 47.70 s with `--nocapture
--test-threads=1`. This verifies that binary's ledger-era source cut, not the
later retry additions or corrected internal test. Independent review then
identified a model transition that classified an already-reserved identity as
capacity pressure. A new negative test reproduced that wrong precedence;
the model now permits capacity pressure only for absent identities and checks
duplicate precedence defensively. Fresh standalone execution passed 3/3,
432 states / 2136 transitions, preserving all three mutant witnesses.
The earlier 2328-transition exploration is superseded, not acceptance evidence
for the corrected model.

The parent independently recompiled and ran that standalone model in a fresh
`mktemp -d /tmp/mir-i3-model-checkpoint-XXXXXX` directory using `rustc
--edition=2021 --test` and `--nocapture --test-threads=1`: exit 0, 3/3 tests,
432 states / 2136 transitions and the three expected mutant traces. This
small dependency-free check does not run Cargo or bypass the main build guard.

After the explicitly authorized incremental-cache cleanup, the sole evaluator
rebuilt the latest dirty source with incremental compilation disabled and two
build jobs. The process-runtime integration target compiled and ran 49/50
successfully; the remaining owner-capacity test incorrectly retained all 64
requester pending entries by discarding real replies. The requester correctly
rejected the 65th emission before owner admission. The test-only correction
must consume checked replies to distinguish owner tombstone capacity from
requester unresolved-request capacity; neither production limit changes.

The independently selected latest-source checks then passed:

- `cargo test --locked -p mir-runtime --lib --features i3-process-test-seams i3_old_owner_carrier_after_m9_withdrawal -- --test-threads=1`: 1/1.
- The same lib target filtered to `i3_same_source_identity_with_a_different_snapshot_rejects_without_replacing_its_tombstone`: 1/1.
- `cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet -- --test-threads=1`: 17/17.

Each used `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`. These results supersede
the earlier uncompiled status for the selected authority/binding tests and
actual-process fault/audit slice, not the unexecuted retry/reconnect route or
the complete 20-family milestone. Current-slice independent review continues.

The test-only repairs subsequently passed fresh execution. The owner-capacity
test now returns each of its first 64 actual owner replies through checked
requester admission, asserts its exact local receipt and cleared pending
record, and retains every owner tombstone before testing the 65th owner
admission. A separate requester test holds 64 unresolved generated requests
and confirms the next rejection preserves pending, outbox and occurrence
snapshots. No production limit or authority rule changed. The private probe
test's array-to-iterator conversion error (`E0599`, no tests executed in that
failed run) was also corrected without changing production code.

Fresh evaluator results: exact owner-capacity 1/1; runtime integration
`i3_3_` filter 4/4; full `sys5_i3_process_runtime` integration 51/51 (44.43 s);
probe library `lifecycle_evidence_tests` 2/2; `cargo fmt --all -- --check`
exit 0. All Cargo invocations retained `--locked`, incremental disabled and
two jobs, with disk remaining at 18 GiB available. The immediately preceding
17/17 actual-process integration result remains applicable; it was not rerun
after changes confined to runtime integration tests and probe `cfg(test)`.
These are current-slice runtime results, not an I3-3 close or actual-network
retry proof. The next verification directly exercises the already-added
runtime-bound retry API before its actual two-session harness consumer.

That runtime-only retry verification subsequently passed 2/2, followed by the
full same-feature `mir-runtime --lib` suite: 285/285, exit 0, 82.33 s. The
selected retry module compiled in 28.57 s and executed in 0.67 s. An initial
test compile error (`expect_err` required `Debug` for an opaque authorization)
was repaired only in test assertions using `.err().expect(...)`; no `Debug`
implementation or carrier/token disclosure was added. Tests preserve exact
bytes/identity across two authorized attempts, reject a foreign runtime handle
and excess/incorrect attempts, retain pending state on an invalid reply, then
consume one checked receipt and reject stale replay. They do not create QUIC
sessions. Disk remained at 18 GiB. The parent's `make docs` also completed
with exit 0, including configuration, 210-file Canon index, 800/800 hierarchy
and 1760-report scaffold checks. Focused changed-crate lint and checkpoint
planning review were the next pre-commit checks. Planning review subsequently
found no P0/P1 alignment issue and requested only reconciliation of historical
uncompiled/pending statements with these fresh results. Focused lint exposed
`large_enum_variant` in the new private reply outcome; the production owner
boxed only its large receipt field without changing pending/consumption
semantics. The same lint then identified two large admitted-evidence variants
in the probe; only their request-receive fields were boxed, preserving Serde
shape. Two model-test nested conditions became equivalent Rust-2021 match
guards. No warnings were suppressed and no model transition changed.

Final checkpoint verification passed: focused `cargo clippy --locked
-p mir-runtime -p mirrorea-i3-probe --all-targets --features
mir-runtime/i3-process-test-seams -- -D warnings`; workspace formatting and
diff checks; actual probe integration 17/17; runtime retry module 2/2; private
probe lifecycle 2/2; model 3/3 with 432 states, 2136 transitions and all three
mutant witnesses. Each Cargo command used incremental disabled and two jobs.
The separately run model target emitted eight runtime `dead_code` warnings
without the private feature profile; its exit was 0, but this does not claim
all default-feature targets are warning-free. The requested two-crate lint
profile passed. Root remained at 18 GiB available. Secret-pattern screening
found no matches in the tracked diff or either new source file; this is a
bounded screening check, not a general secret-detection guarantee.

## What changed in understanding

Owner-intent checkpoint: at the owner's explicit request, the parent used a
separate read-only tab in the existing Oracle Chrome profile to read the
specified discussion's owner statements and latest analysis. No messages were
posted or edited; the temporary reading tab was closed without touching the
running independent Oracle consultation. No private transcript or conversation
URL is reproduced in this public repository.

The owner reaffirmed meaning-derived distribution, ordinary programming,
distinct lifetime/admission/patch responsibilities, Mir-owned authoritative
logic, typed replaceable external boundaries and continued checked evolution.
The long-term goal is a programmable, observable computational environment,
not merely a transport runtime or a particular VR application. Reversed Library
remains an upper application. Independent Canon-first planner review found
these intentions consistent with current Canon and the fixed Plan 250
sequence. Advice in that separate discussion about broader compositional proof
is advisory future scope, not an added I3-3 gate. Its pause-era repository
snapshot does not supersede the current ADR-0040 resume.

The concrete steering correction is to keep finite implementation assumptions
finite: the 64-entry ledger is neither a universal no-GC requirement nor a
continued-operation solution; the admitted cohort coordinator is not a global
Mir authority requirement; retry-attempt rejection does not resolve the
original uncertain operation; and fault-matrix completion cannot justify
synthetic evidence or new source meaning by itself. Plan 250 records this
containment without a new roadmap or general-theory prerequisite.

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

A second, distinct one-off Oracle consultation, `i3-3-effect-time-provider`,
completed normally in 11m24s with 6Pro selection verified. The parent read its
full answer and compared it with the mapper's accepted-path findings. The
current SurfaceV0/SYS3 path has no declared external-provider invocation;
historical Full System V1 syntax and provider-admission fixtures are not an
I3 execution path. Consequently, renaming an RMW, transport or policy error
as provider failure would invent missing effect meaning and is rejected.
The narrow proposed alternative is one checked, read-only external invocation
with its own declared failure, generated projection and actual provider
attempt. Its source/Core/admission contract remains **OPEN**; no provider
implementation or Canon adoption has been made at this checkpoint.

For the combined timeout/lease/clock family, the advisory proposes a distinct
admitted request-lifecycle deadline and actual pre-serve time check. It is not
the launcher/reaper timeout and must not silently change an existing operation
failure contract. A finite pending-request timeout would not establish lease,
fallback, synchronized-clock or arbitrary cancellation semantics. Its exact
contract is also still **OPEN**. The parent retains all 20 required families;
neither missing row is counted as covered by this design advice. The next
consumer is I3-3 boundary specification/implementation after the current
runtime slice is freshly verified, not a new Browser/Host or theory program.

An implementation counterexample refined rejection precedence: actual M9
revocation advances generation, so an old carrier fails the existing exact
lineage-binding gate before a capability check. That fail-closed rule is
preserved; tests must not demand a later `MissingCapability` diagnostic or
widen historical-lineage admission just to reach it. Current-bound authority
revalidation and actual-network withdrawal remain separate evidence work.
Correctness, capacity, exact binding and all fault rows still require final
execution and independent review before acceptance.

The subsequent retry source checkpoint retains at most 64 requester pending
records, checks capacity before source submission, and permits at most two
explicit attempts through opaque runtime-bound pending handles. Reconnect
consumes the peer-control/counter state into one second session; current
lineage/authority is revalidated, occurrence counters fail closed on overflow,
and rejected ingress exposes only run-scoped commitment/occurrence evidence.
Reply failure retains the original pending handle; one checked local receipt
consumes it. At that source handoff, these additions and their safe attempt
summaries had formatting/diff checks only. The later 2/2 runtime-only retry
and 285/285 library runs above supersede that uncompiled state; actual
reconnect execution and milestone acceptance remain pending.

The decoded/restricted child images intentionally omit the M9 successor
publisher. Therefore an integration test cannot manufacture local revocation
by exposing that publisher to a child. Canon theory/05 and theory/18 require
the retained admitted-program M9 publisher to produce successors. The parent
selected the smallest conforming direction: genuine coordinator-published
immutable successors, restricted per child and validated through independently
trusted control binding. A child-produced "monotone self-withdrawal" successor
is rejected as inconsistent with that producer rule. Exact installation and
actual-network withdrawal tests remain open; no new authority is inferred from
deployment, session or fault controls.

Owner decisions are required only at the existing ADR-0034 reserved stops:
guarantee/North-Star weakening, unavoidable hidden retry/transaction, public
freeze, irreversible semantic tie, both retained transports failing required
criteria, or protected production/resource/security scope. Missing later
features and general proofs are not themselves stop conditions.

The next actual-session consumer uses the existing generated owner carrier,
opaque original-pending handle, and consuming QUIC reconnect API. The parent
selects this smallest extension over adding a new network rejection codec:
owner-local typed duplicate rejection can be joined as actual owner evidence,
but cannot be described as a failure delivered to the requester or as completion
of the original operation. No new wire meaning is needed for these two cases.

- Before first carrier write: retain one source-emitted original request,
  complete and close the first checked session without sending its carrier,
  reconnect using consumed control on both sides, explicitly send the retained
  original on session two, and consume the actual checked reply exactly once.
- After actual owner admission before reply: send the original once, retain
  actual owner serve/write evidence while the reply is withheld by connection
  close, reconnect the same live runtimes, and explicitly send the identical
  original carrier. Owner current admission then duplicate lookup rejects it
  without a second mutation. Requester retains the original pending operation;
  the joined view separates owner-observed attempt rejection from requester
  reply uncertainty and makes no successful-recovery claim.

Implementation ownership remains probe-local scheduling/evidence in
`i3_process_localnet.rs` and `i3_process_faults.rs`, runtime/adapter enforcement
in `sys5_i3_process_runtime.rs` and `sys5_i3_private_quic.rs`, and separately
owned tests. Request identity and carrier binding stay unchanged; session and
network occurrences change. Positive/control and falsifier tests must inspect
actual child events, exact joins, pending/receipt state, unchanged owner
mutation count on duplicate, bounded reaping, and wrong-session local misuse
without peer-blame diagnostics. No identity manufacture, child reboot, ledger
reset, authority update, automatic retry, or new source operation is permitted.
This is an I3-3 direct consumer, not I3-4 activation or complete fault coverage.

The rejected-attempt join additionally requires a common adapter-derived,
run/session-bound commitment to the actual encoded frame. Sender evidence and
receiver rejection must agree on it before the supervisor attaches a duplicate
outcome. A nonempty unrelated hash is insufficient. Missing/mutated or
wrong-session commitments produce explicit evidence rejection while requester
uncertainty remains; they cannot import rejected source/Core claims. This
narrow evidence addition belongs to the next actual-session slice, not the
current checkpoint, and does not add a rejection wire message.

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

Parent review of the first two actual-process disconnect profiles found three
gaps before acceptance: normal joined tests permitted absent owner evidence;
owner evidence was not checked against the complete retained source/Core/
artifact/edge contract; and absent owner-terminal evidence escaped through a
lifecycle error before request-bound ambiguity could be retained. Separate
source/test owners are adding strict normal joins, an evidence-only provenance
falsifier, and a post-admission suppress-owner-audit control. Missing evidence
must remain unknown, not zero mutation; clean child exit is not semantic
success. These findings remain open until fresh execution and review.

Independent first-slice review found no P0 and assessed the owner ledger's
ordering/retention as coherent. It reported three P1 findings: unexpected
normal-profile owner terminal shapes silently became unknown evidence;
request-delivery validation omitted linked-identity and nonempty carrier/
network-reference checks; and the model's capacity-pressure transition
contradicted duplicate precedence. The source/test owners are correcting the
two harness issues; the model correction has the fresh 3/3 result above.
The reviewer also requested concrete retention-after-handoff-error and
same-identity/different-binding tests and stronger model-bound checks (P2).
Later in-progress retry code was explicitly excluded from this review.
No aggregate first-slice or milestone acceptance is claimed pending the fixes,
fresh compilation/execution and narrow re-review.

The narrow re-review confirmed the three original P1 mechanisms were fixed in
source, then found an incorrect lifecycle stage/count default for unexpected
owner terminals. Source now preserves post-Ready owner-start/terminal evidence
and classifies missing or unusable evidence explicitly. Post-loss starts at
`RequesterReplyOrReceiptNotObserved`; only a validated admitted-owner join may
upgrade it to `AfterRemoteAdmission`. Suppressed or provenance-rejected owner
evidence remains unknown. The separate test owner added all-terminal-shape,
zero-mutation/positive-admission and missing-evidence assertions. The parent
inspected these changes; initially only formatting/diff checks passed. The
subsequent 17/17 actual-process and 2/2 internal lifecycle tests above supersede
that uncompiled checkpoint. Concrete P2 tests now cover the
pre-handoff reservation failure and private-ledger binding mismatch; the
pre-handoff seam is not claimed as a failure after actual mutation.

The subsequent independent current-slice review found no P0/P1 production
defect and confirmed the earlier lifecycle-stage P1 resolved. It inspected
current-binding/authority-before-dedup order, exact retained snapshots,
non-eviction, opaque runtime-bound retry handles, attempt commitment before
network writes, consuming two-session control, and fail-closed rejected-input
evidence. One P2 remains: the new send-attempt guard reports
`PeerBindingRejected` when peer/preface checks succeeded but the caller uses
the wrong session generation. Its direct fix/test consumer is the actual
two-session retry slice; this local misuse must not blame the peer. The review
does not establish actual reconnect, withdrawal-between-sessions, or complete
I3-3 coverage. Runtime-only retry tests subsequently passed 2/2, with 285/285
full library regression as recorded above; actual-session evidence is next.

## Skipped validations and reasons

The planning writer runs no builds or runtime tests: the assignment is limited
to two planning/report files and the parent already supplied focused baseline
results. Full I3-3 fault/order coverage, I2/M10 close regression, independent
milestone acceptance and final remote parity are still unexecuted for I3-3.
The standalone bounded model above does not replace those checks.
They are required as applicable before milestone close,
not skipped passes. General proof, WAN/production, durability and Browser/Host
product realization remain outside this bounded milestone.

## Commit / push status

The parent committed and pushed the resume integration at
`48d98ed0279cf6fa2a0eaabd7d1ec21c926c7aec`, with fresh remote parity confirmed
during recovery. The source/evidence checkpoint containing this report pins
the verified first I3-3 slice, not milestone acceptance. The planned commit
subject is `feat: checkpoint I3 request lifecycle and fault evidence`; its
exact hash and push result will be recorded during the next slice, without a
metadata-only commit. Parent owns
accepted source/evidence pinning, integration commits, authorized pushes and
fresh remote parity before the fixed transition to I3-4.

## Sub-agent session close status

Planning writer hands both planning/report files back to the parent after
scoped validation. Source and test owners remain active. Other agents' work is
preserved; parent retains active-goal integration and final acceptance.
I3-3 remains ongoing when this writer returns; no user-task completion or
milestone close is asserted by the sub-agent handoff.

Recovery checkpoint: replacement runtime, harness, test, mapping, planner and
review specialists have handed back their bounded work; both Oracle consults
completed. Their contexts are retained for continuation, with no new feature
assignment pending. The parent stops additional build/integration acceptance
at the explicit cleanup-permission boundary, not at I3-3 completion. No
milestone was accepted, no later milestone activated, and no unvalidated
source commit/push or deletion was performed in this recovery checkpoint.

The subsequent explicit owner approval resolved that cleanup-permission
pause. The narrowly authorized cleanup and resumed validation are recorded
under Start state above. Evaluation and bounded test diagnosis are active;
no later milestone has been activated.
