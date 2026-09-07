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

Latest owner execution control (recorded at `2026-09-07T18:13:00+09:00`):
complete I3-3 with its full evidence, independent review, commit/push and
remote parity, then stop and report. I3-4 must not activate without explicit
owner resume. This supersedes automatic continuation only, not the original
program scope or the 20-family/order gates. I3-3 remains active now; after
acceptance retain Plan 250 with no active semantic milestone. The user controls
the goal pause; do not represent this as blocked, stale or whole-program
completion. Independent pre-edit planner review confirms the existing
ADR-0040/PROPOSAL-043/operating-model pause exception permits this control.
The I3-3 acceptance record will carry the effective pause; historical ADRs are
not rewritten now.

Independent final control-diff review finds no P0/P1 scope inconsistency.
Fresh `make docs` passes (agent configuration, 210 Canon files, 800 hierarchy
paths, 1760 reports), as does `git diff --check`. The material owner-stop
instruction is committed separately from the in-progress adapter delivery
implementation; that documentation commit does not claim those new tests pass
or pause the currently active I3-3 work.

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

Post-repair delivery checkpoint commands (all executed with
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`; results and source scope below):

```bash
cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet i3_3_malformed_owner_audit_contract_is_rejected_without_validated_remote_evidence -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet i3_3_disconnect_after_remote_admission_is_request_bound_ambiguity_not_false_success -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet -- --test-threads=1
cargo test --locked -p mirrorea-i3-probe --lib -- --test-threads=1
cargo clippy --locked -p mir-runtime -p mirrorea-i3-probe --all-targets --features mir-runtime/i3-process-test-seams -- -D warnings
cargo test --locked -p mir-runtime --lib --features i3-process-test-seams,i3-private-quic sys5_i3_private_quic_tests -- --nocapture --test-threads=1
cargo test --locked -p mir-runtime --test sys5_i3_process_runtime --features i3-process-test-seams -- --nocapture --test-threads=1
cargo test --locked -p mir-runtime --test sys5_local_slice --test sys6_i2_cli -- --test-threads=1
```

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

During the next probe implementation, the independent runtime-only compile
check with `--features i3-private-quic,i3-process-test-seams` passed for the
new commitment evidence API; the local-attempt guard still intentionally had
its old behavior. Direct accepted-I2 regression also passed using only
`--test sys5_local_slice --test sys6_i2_cli`: 5/5 and 8/8 respectively, exit 0
(0.60 s build; 0.05 s and 11.28 s execution). That default-feature run emitted
the same eight `dead_code` warnings; it is not warning-free evidence. Neither
command compiled the in-progress probe or established actual reconnect. Root
remained at 18 GiB and no further cleanup occurred.

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

The next within-I3-3 authority/late-ingress slice starts from verified reconnect
cut `7580e3a1298dd39bc26ef1979c5033d485e92de5`. Its direct consumer is the
required control/data ordering and stale/revoked-authority fault boundary;
I3-4 remains inactive. The first bounded case pre-stages one genuine M9
owner-capability revocation before any owner semantic admission, avoiding a
new observation-counter synchronization protocol. The exact successor must
preserve the admitted program, advance one generation, tombstone only the
selected existing owner lineage, introduce no grants/lineages and retain
unrelated inventory and prior tombstones. A child receives a tainted restricted
candidate plus an independently trusted run/program/projection/cohort/slot/
closure/generation/kind/digest binding, never a publisher or self-withdrawal
constructor. A separate checked install and exact owner acknowledgement gate
coordinator publication; lost acknowledgement remains incomplete/unknown, not
a rollback or distributed-durability guarantee.

The adapter's next direct seam is opaque complete-frame pending ingress:
verified network receive may precede a genuine authority successor, while
semantic admission still revalidates the current owner state. The positive
control holds an old-session frame across a second verified session without
an authority update and admits once. The falsifier installs the genuine
revocation first and then admits the retained old frame: the existing current-
binding precedence must yield `CarrierAdmissionRejected` with zero serve/write,
not an invented later `MissingCapability` diagnostic. The lifecycle stimulus
is explicitly `M9AdmittedLifecycle`, not source-derived `SourceAction` or
transport authority. No general administration, issuer-in-child, hidden retry,
membership reacquisition, provider/time, durability or later-milestone scope is
opened by this case. Concrete APIs and tests are being bounded before edits.

The parent reviewed the complete bounded ACK advisory from temporary Oracle
session `i3-3-ack-publicatio-boundary` (5m44s;
SHA-256 `07d8d32bc4604ecf2ddd648c1428e29430c922b5e7d17ce370d538087bb4a8d8`)
against theory/18's install/ack/publication order. It selects A: reuse the
trusted compiled kernel and isolated owner-child control route, with no new
challenge/cryptographic management protocol. Only completed installation and
live-floor advancement may privately construct an installed receipt; a
kernel-only ACK emitter consumes that receipt. The parent reader associates
the exact received record with the registered owner-child instance/descriptor,
not an owner label in JSON or a detachable origin token. Decoding remains
tainted; exact retained pending binding and one-use parent completion gate
publication. Candidate integrity, matching references, `NonClone`, transport
identity and observer logs do not establish installation.

The direct falsifiers are a correct-field candidate-derived ACK through an
untrusted/requester/observer path, failure before the install/floor
postcondition, wrong actual origin or binding, replay, and lost ACK. A test
that replaces the trusted owner emitter with a privileged lying writer on its
genuine descriptor is outside this honest-kernel/OS boundary; it must not be
reported as a rejected forgery. Lost ACK may leave B at G2 while parent
publication remains incomplete, without rollback or hidden retry. This is a
bounded implementation decision and test contract, not executed evidence,
Byzantine-kernel resistance, public protocol, durability, or a new theorem.

Current coverage reconciliation at `7580e3a` separates already executed slices
from candidate tests and missing interactions. The row numbers follow Canon
plan/05's 20 rows; this table does not accept a row or substitute test existence
for execution. Its direct consumer is the remaining I3-3 test assignment.

| Rows | Executed checkpoint / reusable boundary | Remaining claim before full matrix acceptance |
|---|---|---|
| 1 route/partition/endpoint | Before-write disconnect is executed, but is not absent/refused endpoint evidence | bounded actual route/endpoint failure and no invented route |
| 2 handshake/peer | Actual peer/preface rejection is in the 22-test process floor | bind exact positive/falsifier and non-authority evidence to row |
| 3 target; 4 provenance | Retained runtime/private-codec target/lineage tests use the production receiver binder | exact test-to-row inventory and representative actual-ingress correspondence |
| 5 partial/split; 6 malformed/limits | Private process codec has bounded complete-frame negatives; older static-adapter chunk tests are a different profile | actual selected process adapter split/truncation behavior; no partial admission |
| 7 before-admission loss; 8 after-admission loss | Actual unavailable versus request-bound ambiguity paths and rejected/missing observer joins are executed | final exact ordering/evidence classification |
| 9 reconnect; 10 duplicate | Two actual sessions, one original identity, owner duplicate rejection and retained requester ambiguity are executed | current-authority/late-traffic interaction; no migration or exactly-once claim |
| 11 result/receipt replay | Retained process-runtime checked reply/replay boundary | actual late/duplicate reply schedule and current pending binding |
| 12 control/data order | Session/attempt distinctions are executed | current retained-ingress/successor slice |
| 13 membership/epoch; 14 capability/witness | Retained M9 withdrawal test distinguishes current binding from pure authority revalidation | actual genuine successor and old-use rejection; capability-only revocation must not be mislabeled membership retirement |
| 15 auth/policy | Existing M9 admission tests are candidate evidence, not actual I3 policy execution | exact admitted-policy failure and pre-use ordering |
| 16 capacity | Retained 51-test floor includes bounded requester/owner ledger capacity | exact positive/negative source/runtime binding; no new network-flow saturation requirement inferred |
| 17 external time | Lifecycle reaper deadline is not semantic request expiry | OPEN request-bound pre-serve time contract and actual check |
| 18 provider | Ordinary owner RMW is not a provider invocation | OPEN declared generic effect, generated execution and actual typed provider result/failure |
| 19 visibility/redaction | Source/observer-safe joins are executed; local/static mismatch tests are candidate evidence | exact production admission mismatch and redacted actual diagnostic |
| 20 cut/patch/save | Accepted local quiescence rules are input evidence | actual child quiescent control plus in-flight rejection/late-traffic interaction, without durability |

The same production boundary may supply multiple finite negative inputs; each
parser variant need not create another subprocess profile. Network-dependent
claims still need actual selected adapter/process observations. No row is
removed, broadened into a new general theorem, or silently closed by this
reconciliation.

The separate test owner then added the first prestaged-lifecycle runtime test
group. The sole evaluator ran
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-runtime
--test sys5_i3_process_runtime --features i3-process-test-seams i3_3_prestaged
-- --nocapture --test-threads=1`: exit 101 with 24 missing-API/cascading type
errors. The binary did not execute; this is API/compile RED, not a behavioral
falsifier or passing evidence. The production owner may now implement that
bounded surface. The fourth tainted-ACK filter and new process tests were not
run. Eight dead-code warnings were also emitted; no warning-free or independently
established pre-existing-warning claim is made. Root free space was 16 GiB,
above the 10 GiB build-stop guard; no further cleanup was performed.

The first two actual late-ingress test bodies were then added separately.
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked
-p mirrorea-i3-probe --test i3_process_localnet i3_3_late_session_one_ingress
-- --nocapture --test-threads=1` exited 101 with seven compile errors: missing
late-ingress exports/builders/audit APIs plus a helper lifetime and an inferred
type error. No test executed. The test owner must correct the helper lifetime
rather than treating every error as an absent-production-API cascade. This
opens the probe implementation gate only; disk remained 16 GiB and no cleanup
was performed.

During the bounded implementation, the parent found an exact-delta gap in the
initial predicate draft: preserving M9 maps and checking the selected M8
capability/witness alone would not exclude an added or changed unrelated M8
record. The source owner must compare the complete M8 state against the prior
state with only the selected capability/witness retired; the test owner adds
an independent unrelated-M8-state falsifier. The parent extended source
ownership narrowly to `m8_runtime_authority.rs` for pure crate-private checks
and private test seams, not a public authority API or broader M8 redesign.
A genuine prior-tombstone predicate test uses two actual M9 revocations, not
an empty-G1 set mislabeled as prior-tombstone evidence. These are identified
implementation/test requirements, not yet executed results.

The parent also rejected an intermediate cross-crate ACK signature that let a
cohort-issued reader/token consume a caller-supplied tainted ACK. Without
actually owning and reading the registered output stream, the method name
"from registered owner FD" cannot establish record origin: a caller could
combine the token with correct candidate-derived bytes before installation.
The required replacement owns the actual registered child-output stream and
performs the framed read/decode itself; it must not expose completion from
arbitrary decoded records. Existing compiled-kernel/OS trust remains explicit.
This corrects a proposed API boundary before acceptance, not an executed
security test or a reason to add a cryptographic management protocol.

The concrete bounded replacement uses one dedicated inherited B-to-parent
UnixStream ACK descriptor. The alternative was ownership of the existing
multiplexed `ChildStdout`, but its probe-local event enum would invert the
runtime/probe dependency. Only the actual B spawn path registers the dedicated
route. Its reader owns the parent stream plus the independent cohort binding,
performs bounded framed reads internally, and returns an immutable opaque
completion; the cohort separately consumes that completion for one pending
publication. There is no completion API accepting caller-supplied bytes or
tainted records. The ordinary stdout parser remains unchanged. Descriptor
duplication/close-on-exec, writer-end closure, deadline and reap behavior must
preserve the existing bounded process lifecycle. This private local ACK route
does not grant semantic authority, introduce a general management protocol, or
close the separate semantic external-time row.

At the first frozen runtime-source handoff, the same focused prestaged test
command stopped at compilation with six errors: owned-string/hash component
types, incorrect assumed `LocalFabric` identity/fingerprint/summary accessors,
and an immutable coordinator admission binding. No test body executed; ACK,
install-failure and private M9 predicate targets were not run. The source owner
corrected the existing program accessors and types without inventing a shadow
summary, then re-froze after formatting/diff checks for another evaluation.
Disk ended at 17 GiB free; no deletion occurred.

The parent also authorized the private probe dependency to enable the existing
`mir-runtime/i3-process-test-seams` feature alongside `i3-private-quic`, solely
to consume the genuine post-stage image-wrapper falsifier. Runtime default and
workspace features are unchanged. This negative alters B's tainted lifecycle
wrapper after genuine prestaging and retained control binding; the separate
M9 predicate negatives reject in the parent and cannot stand in for actual
child bootstrap rejection. No positive path gains an authority constructor.

The post-type-fix runtime build compiled, then executed four prestaged tests:
one passed and three normal staging paths failed with
`LifecyclePrestageRejected` (1.65 s; build 33.69 s). The targeted private M9
module subsequently ran two tests, both failing at their genuine G1-to-G2 or
G2-to-G3 positive predicates before the intended tombstone/unrelated-state
falsifiers (0.01 s; build 30.61 s). These are behavioral RED results, not
successful falsifier execution or upper image-restriction evidence.

The source owner localized a representation mismatch, which the parent
verified against committed `7580e3a` translation code: M9 retains inactive
capabilities, stale dependent witnesses and revocation evidence, but its M8
execution projection includes only active capabilities/live witnesses. Thus
the genuine projected successor omits the selected retired records rather
than retaining explicit inactive/stale M8 records. The parent rejected a
proposed change to the accepted M9-to-M8 producer merely to satisfy the new
I3 predicate. The new predicate must instead compare the entire M8 projection
to the prior projection minus exactly the selected capability and dependent
witness rows, retaining every unrelated row and the exact M9 successor,
lineage and monotone tombstone conditions. Test-only reanimation restores
only the corresponding genuine prior record. This corrects the new
representation assumption; it does not weaken logical revocation or change
accepted I2 authority semantics. Fresh positive/falsifier execution is pending.

The subsequent focused M9 rerun remains RED (0/2): genuine G1-to-G2 now
passes, but both tests stop at genuine G2-to-G3 before their falsifiers.
Static diagnosis localizes a second representation question: the existing
successor constructor restores only the newly revoked owner-use/lineage
entries after active-only translation, while retaining all earlier
tombstones. Earlier retired owner-use/lineage entries therefore disappear.
The parent requested a bounded independent comparison of retaining those
historical maps in the producer versus recognizing their exact permitted
omission in the new predicate. Neither a producer change nor a relaxed
predicate is accepted merely to make the positive tests pass. This run does
not establish actual QUIC or process behavior; root free space remains 16 GiB.

The independent bounded review recommends retaining both historical M9
owner-use and kernel-lineage maps in the existing revocation-successor
constructor. The parent accepts this narrow correction: these paired maps
are sealed binding/history, already retained for the most recently revoked
owner, whereas M8 remains the active-only authority projection. Allowing their
omission only in the new predicate would also break exact per-process
restriction when that scope still contains the earlier-retired owner operation,
and would remove that owner's current-generation lineage binding.
The correction preserves the exact tombstone set and M8 omission, adds no
grant, and must be tested with both retired owners denied without new
validation observations, as well as map/tombstone/unrelated-state falsifiers.
This decision is not yet a passing implementation or milestone acceptance.

With that narrow correction, the focused private M9 module passes 2/2
(build 29.07 s, one dead-code warning). The subsequent exact
`i3_3_prestaged` runtime integration filter still fails: 1/4 passes and three
normal stages return `LifecyclePrestageRejected` before child bootstrap
(build 31.53 s, tests 1.64 s; eight dead-code warnings). Full runtime
integration, registered-ACK and actual late-ingress execution therefore have
not run on this cut. The remaining diagnosis is the process/coordinator
prestaging boundary, not a reason to weaken the now-tested M9 predicate.

The runtime owner localized that rejection before M9 restriction: the new
prestage gate incorrectly required reply-only linked-request and typed-outcome
flags from an owner-request contract. The parent verified SYS-3's existing
separate request/reply constructors and authorized removing only those two
requirements from the new gate. Exact owner-request kind, checked identity,
full retained-contract match, target and M9 checks remain. The existing
genuine-request positive and wrong-reply-edge falsifier are its direct tests;
the accepted carrier definitions do not change.

After that correction, the prestaged filter passes 4/4 (build 32.65 s,
tests 3.26 s). The full runtime integration target then executes 59 tests:
58 pass, while the cross-cohort completion test fails during its originating
cohort's genuine ACK acceptance, before the intended foreign-publication
falsifier (49.65 s). Static binding propagation is unchanged end-to-end; the
test registers its new absolute 250 ms deadline before image bootstrap and
M9 installation. The next narrow diagnostic measures that setup duration.
No production deadline or binding relaxation is authorized. Actual-process
late ingress remains unexecuted on this cut.

The narrow rerun measures 364.342423 ms from reader registration to the
genuine ACK rejection, exceeding that fixture's 250 ms absolute budget
(0/1 passes; 0.98 s test, 58 filtered). The parent authorizes a finite 5 s
budget only for the cross-cohort provenance test's valid setup. The separate
250 ms fragmented-reader deadline test and production deadline remain
unchanged. This is a measured fixture correction, not a timeout waiver.

The corrected full runtime integration target passes 59/59 (build 0.87 s,
tests 49.40 s; eight default-feature dead-code warnings). The first actual
late-ingress probe filter then stops at compilation with 13 errors and two
warnings: enum variant field visibility, missing serialization derives for
bounded audit enums, and an immutable lifecycle-summary borrow held across
mutable runtime admission. No probe test executes. The probe owner must
correct these local Rust issues without duplicating receipts/authority or
changing tested runtime semantics. Disk remains 16 GiB free.

After the probe compile repairs, the focused actual-process late-ingress
filter executes: unchanged-G1 control passes; the G2 case returns
`LifecycleRejected` instead of the expected joined `AmbiguousDelivery`
(1/2 passes; build 4.67 s, tests 2.39 s, 25 filtered). The full 27-test
probe target has not run. The parent requests observer-safe failure-stage
diagnosis; neither a profile-selected expected outcome nor unvalidated owner
evidence may override the actual failure. Root free space ends at 15 GiB.
Parent inspection also finds the G2 install currently follows session-two
establishment; the approved schedule requires install before that reconnect,
so the actual test must establish reconnect-after-withdrawal, not merely
withdrawal-before-eventual-admission. The probe owner must restore that order
without cloning a receipt or changing runtime admission.

The safe diagnostic identifies B's actual handled carrier rejection (exit 0,
zero admission/write) followed by A's generic rejection (exit 1), with both
children reaped. B immediately closes the QUIC connection after writing its
session-two preface, which need not yet have reached A; A may therefore fail
preface validation before reaching its pending-reply boundary. The authorized
probe correction installs G2 before closing session one, and after the actual
late rejection finishes only B's send half and waits for A's real peer close.
A must validate the preface, observe EOF as pending, report that fact and
close. No sleep, retry, semantic reply or receipt is added. B's local peer
validation precedes its admission attempt; A's validation precedes its pending
observation. No additional global application-level handshake barrier is
claimed. Outer lifecycle/reaper deadlines remain unchanged.

After the bounded schedule/EOF correction, actual G1/G2 late-ingress passes
2/2 (build 5.16 s, tests 2.45 s). Full probe integration then passes 27/27
(48.14 s, cached build 0.04 s). This establishes the executed G1/G2 paths,
not the validity of every named falsifier. The independent reviewer confirms
a P1 evidence defect: five ACK profiles named A-route, observer-route and
wrong run/reference/slot only suppress B's valid ACK, then classify the same
reader failure from the selected profile. They inject no corresponding
candidate/frame. Those passes are only repeated dropped-ACK executions and
cannot close origin or wrong-binding falsifiers. Acceptance is withheld until
the actual inputs are exercised or the names/claims are consolidated with
genuine production-boundary evidence. Normal post-install ACK gating and
full runtime 59/59 remain separate evidence, not a waiver of this finding.

The independent current-slice review reports three P1 findings and no P0 or
additional independent P2: the ACK-input defect above; missing session-owned
one-shot acquisition/consumption guards for retained ingress; and G2 joining
selected counters without validating the complete per-slot terminal contract.
The latter can accept contradictory control/transport/unauthenticated-count
observations. All three are in-scope repairs, not milestone or owner-reserved
stops. The approved repair consolidates the false ACK profiles into real drop,
replay and one actual A-stdout tainted-input path, plus production-reader
component field-mismatch tests; adds a private one-shot permit reserved before
I/O and consumed before semantic handoff; and shares exact terminal validation
between the full join and independently retained A-pending evidence.

Before those repairs, full runtime library regression with
`i3-process-test-seams,i3-private-quic` passes 288/288 (build 30.33 s,
tests 82.79 s), including the control-binding component test. Focused
two-crate deny-warnings Clippy fails on the large accepted late-ingress audit
enum variant and the eight-argument server helper. Boxing the audit payload
and a small server invocation aggregate are authorized local fixes, not a
general harness framework. Those pre-repair green counts are its regression
floor, not evidence that the findings were resolved.

The one-shot permit tests first fail to compile with three missing-type errors
(no test execution), then pass 2/2 against the production-used private state
machine (build 29.85 s). The separate terminal-corruption probe tests initially
fail to compile with five missing-API errors, again not a behavioral falsifier.
After the runtime repair, the exact runtime integration target passes 61/61
(build 1.00 s, tests 54.90 s), including genuine ACK-v2 field mutation through
the registered reader and wrong-slot registration preserving the valid B
registration. The eight known default-feature dead-code warnings remain;
their pre-existing status was not independently established by a pinned-cut
comparison. Root free space remains 15 GiB, above the 10 GiB build-stop guard.

At 2026-09-07 17:47 JST the probe source/test owners freeze the repair cut for
fresh actual-process evaluation and narrow independent re-review. The probe
now routes a tainted candidate through actual A stdout, retains real dropped
and replayed B ACK controls, validates complete per-slot terminal observations,
and exercises a second retained-ingress acquisition before completing the
original G1 request. These are implemented claims awaiting fresh process
results, not acceptance. Runtime and probe writers are distinct from the
test author, sole command evaluator and independent reviewer. I3-3 remains
the sole active milestone.

Fresh actual-process evaluation of that frozen repair passes each of the four
focused tests 1/1: terminal validation (2.49 s), second retained acquisition
(1.24 s), actual A-stdout/dropped/replayed ACK inputs (3.74 s), and the G2
late-admission join (1.24 s). Full probe integration passes 29/29 in 46.25 s;
full runtime feature library passes 289/289 in 81.32 s. The exact commands,
with `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, are:

```bash
cargo test --locked -p mirrorea-i3-probe --test i3_process_localnet -- --test-threads=1
cargo test --locked -p mir-runtime --lib --features i3-process-test-seams,i3-private-quic -- --nocapture --test-threads=1
cargo clippy --locked -p mir-runtime -p mirrorea-i3-probe --all-targets --features mir-runtime/i3-process-test-seams -- -D warnings
```

The focused Clippy gate passes in 30.90 s with no warnings. Parent format and
diff checks also pass. These results apply to HEAD `7580e3a` **plus the frozen
dirty successor/repair changes**, not to that committed revision alone.
Independent narrow review then confirms no remaining P0/P1 in the three
repaired mechanisms: actual tainted A input cannot produce registered-B
completion; the session-owned permit transfers and is consumed before semantic
admission; and exact terminal validation gates both the joined owner conclusion
and independently retained requester uncertainty. The reviewer ran no Cargo;
the results above come from the sole evaluator. Optional P2 strengthening is
an actual partial-prefix/cancelled receive; the next selected-adapter framing
package is its direct consumer, not a new general framework or current
acceptance blocker. This disposition assumes the stated honest compiled
kernel/control plane and OS descriptor provenance, not Byzantine attestation.
Applicable I2/M10 regression then passes with the same bounded build settings:

```bash
cargo test --locked -p mir-runtime --test sys5_local_slice --test sys6_i2_cli -- --test-threads=1
cargo test --locked -p mir-runtime --test m10_conformance -- --test-threads=1
```

SYS-5 passes 5/5 (0.05 s), SYS-6 CLI 8/8 (12.32 s), and M10 conformance
67/67 (13.59 s); builds take 37.64 s and 1.87 s respectively. These test
commands emit the eight known default-feature dead-code warnings, distinct
from the passing seam-feature deny-warnings Clippy gate. The evaluator's
read-only process check finds no `mirrorea-i3-probe` children; none was
signalled or killed. Disk remains 15 GiB free. No new Lean or bounded-model
run, complete I3-3 matrix, or milestone acceptance is claimed.

Parent integration checks pass: `cargo fmt --all -- --check`,
`git diff --check`, and `make docs` (agent configuration, 210 Canon index
files, 800/800 hierarchy paths, 1760 reports). A scoped diff plus the two new
private test files has no match for the checked private-key, webhook,
private-chat-link and common API-key patterns; this is a limited pattern
screen, not a general secret-freedom proof. The parent corrects snapshot
wording that had mislabeled the older probe lifecycle 2/2 as fresh, omitted
the current runtime 61/61 and library 289/289, or left successor installation
as the next consumer. No code changes result from that synchronization.

Independent planner advice selects the smallest next package: actual endpoint
unavailability, controlled writes of one complete generated frame, and actual
truncated-frame rejection through the existing selected adapter and child
lifecycle. Production decoder variants remain component evidence rather than
each receiving a new process harness. Provider/time contract specification
follows that bounded package, so easy transport cases do not indefinitely
postpone those two OPEN dependencies. All 20 rows and ordering obligations
remain required. Direct consumer: I3-3's missing network-interaction evidence;
blocker reduced: selected-adapter delivery behavior; acceptance use: exact
positive/falsifier observations, nonmutation and bounded cleanup. No milestone
or additional semantic frontier is activated.

The independent final snapshot planner review finds no P0/P1 acceptance or
scope issue: fresh and retained evidence are separated, only I3-3 is active,
and the next bounded package does not remove the provider/time dependencies
or complete failure/order inventory. This review is read-only and does not
rerun execution evidence. The parent may pin this reviewed checkpoint and
continue without a program pause.

The next bounded delivery assignment starts from clean committed cut
`cb050be23be6bc2cbc420b24f8d8c583503d6fff`. It reuses the existing source
request, selected QUIC blob codec, child lifecycle and observer joins. Complete
fragmented writes remain ordinary success; strict truncation must produce an
actual receiver frame failure before decode/admission, with no receiver source
lineage invented from incomplete bytes. A truncated sender has attempted a
write and cannot be labeled `RequestCarrierWriteNotAttempted`. Endpoint
unavailability must reach actual connection failure and known owner-side
nonadmission, not infer nonmutation from a missing child or count the parent
reaper deadline as the semantic timeout family. The separate source and test
owners first agree minimal private controls, then the sole evaluator captures
RED before production implementation. No new Core, public wire, retry policy,
general control protocol or network-packet fragmentation claim is introduced.

The selected control design is one private adapter-delivery profile, mutually
exclusive with existing disconnect/retry/late-ingress schedules. Folding a
successful fragmented send into the older fault enum is rejected because it
would blur normal completion and the distinct connection/frame failures.
`EndpointClosedBeforeConnect` names the actual order: B closes its bound
endpoint before A connects to the subsequently reported address. Its bounded
adapter connection outcome is not the still-OPEN semantic time contract.
The main review requires an actual runtime-produced write observation for the
complete-frame positive: using the normal whole-frame path must not leave
the fragmentation test green. Only completed controlled writes may supply
that observation; normal delivery has none. This adds no bytes, dynamic frame
length, packet-boundary or semantic-authority claim to observer output.

The three new probe tests then produce compile-only RED: the focused
`i3_3_complete_generated_request_in_two_application_writes_preserves_the_exact_source_bound_round_trip`
filter on the existing probe integration target exits 101 with 17 missing-API
errors (two exports, profile builder and observer/terminal getters). No test
executes. This opens the bounded production implementation gate; it is not
runtime fault evidence. Disk remains 15 GiB free. Runtime QUIC and probe
production writers retain disjoint ownership, separate from the test author
and sole Cargo evaluator.

The first actual delivery run passes endpoint-closed 1/1 (build 36.12 s,
test 2.56 s). Complete two-write then fails with `LifecycleRejected` /
`LifecycleEvidenceRejected` after 1.29 s: both children actually complete,
both sessions validate, the owner serves/writes once, and natural reaping
finishes within the bound. Truncation/full probe/Clippy are not run after this
failure. The cause is the probe's early adapter-failure join accepting all
three controls, while its classifier correctly returns no failure for the
complete-frame positive. The independent reviewer reproduces this P1 in
source. The bounded correction restricts that early branch to endpoint and
truncation negatives so the complete case reaches the normal join and exact
sender write-observation check. A missing write observation on B's receive
record is expected: it must not receive A's sender-only observation. No
runtime or test assertion is weakened; fresh rerun is required.

After that branch correction, complete two-write passes 1/1 (build 6.43 s,
test 1.34 s), truncation passes 1/1 (1.24 s), and full probe integration
passes 32/32 (54.45 s). The all-target Clippy command stops before lint
evaluation on one old inline-test call missing the new optional delivery-record
argument. This is an additional compile surface, not a passing Clippy gate.

Independent review then finds a second P1: the new rejection-audit accessor
projects a raw B terminal's delivery record even when the existing validated
fault join rejects that same record as `ProvenanceMismatch`. The selected
repair deletes the raw-terminal projection and derives any returned record
only from an already validated fault/retry admission join. Other failures
remain `None` (unknown, not proof of nonadmission). The separate test owner
adds both malformed-record absence and genuine validated-record presence
assertions, so returning constant `None` cannot satisfy the repair. Acceptance
is withheld pending behavioral RED, implementation, fresh execution and narrow
review disposition.

The malformed-owner-audit test then compiles and fails behaviorally at its
new `core-ref` corruption assertion (build 5.53 s, test 1.17 s). The invalid
record is still visible through the new accessor, confirming the falsifier
without an API error. The minimal repair removes the raw supervisor accessor
and clones only `request_receive` from validated fault/retry `Admitted`
evidence. Independent narrow source review finds both P1s resolved; final
post-repair execution remains evaluator-owned. Optional direct mutations of
each adapter-terminal field and an additional durable peer-close event are
P2 test hardening, not new current correctness findings. No Cargo pass is
inferred from this static review.

Final post-repair delivery validation applies to `386d5f09` **plus the five
source/test files in this delivery delta**, not that docs-only committed cut
in isolation. The malformed-audit negative passes 1/1 (build 5.19 s, test
4.60 s), and genuine post-admission lost-reply control passes 1/1 (1.21 s).
Full probe integration passes 32/32 (52.62 s), full probe library 3/3 (build
2.78 s), private QUIC unit module 2/2 (build 31.10 s), runtime I3 integration
61/61 (55.23 s), and I2 local slice/CLI 5/5 (0.05 s) + 8/8 (11.60 s).
Focused two-crate all-target deny-warnings Clippy passes (5.73 s). Runtime
integration and I2 emit eight dead-code warnings; this does not claim those
commands warning-free or independently establish that the warnings predate
the delta. Root disk remains 15 GiB free. Feature-library 289/289, M10 67/67
and bounded model 3/3 remain prior checkpoint evidence, not fresh runs.
The independent reviewer finds no remaining P0/P1 in this delivery delta.
This closes only the bounded delivery/observer-repair checkpoint, not any
whole failure family, the full matrix, or I3-3. No new Lean evidence is claimed.

Parent formatting and diff checks pass, and the bounded source-diff
private-key/token/private-chat pattern screen finds no match. The first
`make docs` attempt passes agent configuration, Canon index (210 files) and
hierarchy (800 paths), then rejects a stale progress header: 18:49 was older
than its newly appended 18:52 log. The header is corrected using the actual
18:58 JST clock; full documentation validation is rerun, not assumed passed.
The rerun passes all four checks, including the 1760-report scaffold. Final
read-only planner review clears the snapshot delta: prior executions remain
historical, the time/provider contracts stay OPEN, and the owner pause after
full I3-3 acceptance is preserved. The status writer's stale next-package text
and accidental historical-log rewrite were corrected by the parent before
that review; the 17:52 successor log retains its actual 29/289/M10 results.

A further temporary Oracle preservation review completed in 8m42s; the parent
read the complete answer, SHA-256
`2c958bda69926ea31d97aa98ba203c9f97ec047510e247491378121deed01b44`.
Direct consumer: the still-OPEN I3-3 time/provider contracts after the delivery
checkpoint. Blocker reduced: failure containment and authority-preserving
representation choices. Acceptance use: proposed positive/falsifier contracts,
not evidence for the current implementation. Its recommendation is a separate
explicitly updated admission-lifecycle expiry contract, leaving the checked
Core failure row unchanged, and a distinct checked external invocation whose
effect-specific authorization may share an owner-local inventory. Confirmed
unserved expiry requires a serialized owner gate that excludes later serve;
requester-local expiry after an unknown remote disposition cannot assert
nonexecution. An RMW grant cannot authorize the new effect, nor can an effect
grant authorize a store. The advice remains non-normative pending Canon-first
review and the bounded proposal/decision route; no source language, failure
row, M9 authority or time/provider implementation is changed by the consult.

Subsequent mapping changes the time recommendation, without silently adopting
the Oracle answer. Restricted SYS5/LocalFabric cannot reach the existing M9
`apply_contract_update`; that operation supports authorized layer Attach/Remove,
not a request-expiry contract. A new opaque bridge would still need a supported
semantic authority rule. The parent and planner therefore prefer retained
alternative B: an explicit opt-in source owner-admission budget with its own
generated declared failure, preserving old sources and failure rows. The
finite clock origin/comparison, owner gate, retained expiry decision and
validated terminal failure remain under design review before Canon adoption.
The existing owner reply is success-only: a `FabricReceipt` or `fault_id`
cannot be relabeled as a declared deadline failure. A distinct typed terminal
outcome must preserve exact request/source/owner bindings and remain separate
from successful receipt and unresolved remote disposition. This needed path
also has the next provider-failure row as a direct consumer. Neither candidate
opens general clock/lease theory or a new control protocol.

The parent subsequently selects B under ADR-0034 through PROPOSAL-044 /
ADR-0041 / spec/16, after independent exact-diff review. This is a normative
contract change before implementation, not runtime or milestone acceptance.
Review required exact u64 clock-domain identity and arithmetic; atomic
clock/revoke/resolve order; `ServeReserved` distinct from actual M8 success;
retained authority rejection and immutable wrong-handle behavior; a sealed
one-use request/runtime/source/M9-bound serve permit; atomic requester terminal
retention; and record-derived expiry/delivery/consume versus success counters.
All P1s were repaired in the reviewed contract. The final P2 permit-reuse,
foreign-runtime/request, wrong-handle and downstream-failure falsifiers are
also written into its acceptance matrix.

Mapping found concrete lower-entry bypasses that the new contract must close:
SYS4 bootstraps M8 directly, and an M10 typed handoff schedule also bypasses
the semantic-kernel wrapper. A CLI-only or Kernel-only budget check therefore
cannot suffice. Every private M9 execution translation retains the condition;
unimplemented execution entrances reject annotated requests before mutation.
The complete bare-M5 export must not erase it. An existing owner grant is not
the additional sealed admission permit, nor does that permit replace M9 use
validation. No broad M9 layer/clock framework or hidden ContractUpdate is
introduced. Production implementation has not begun at this decision point;
the separate static implementer and test author first agree exact metadata
APIs and RED tests. Current provider remains OPEN and I3-4 remains inactive.

Final normative integration review found one remaining proposal sentence
promising actual serve before expiry or guaranteed requester delivery. The
parent replaced it with the exact gate reservation/expiry decision and
validated-delivery knowledge boundary; narrow independent confirmation has
no remaining P0/P1. The read-only planner approves a snapshot-only next-step
update: static preservation, projection/non-bypass enforcement, then actual
QUIC failure evidence, all within I3-3. No additional roadmap is opened.

The sole evaluator captures behavioral RED on the first new source test at
`31be54b9` plus the new test/Canon delta:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-semantics
--test i3_owner_admission_budget
explicit_owner_budget_with_declared_deadline_failure_reaches_m6_and_m7
-- --exact`. Compilation succeeds (0.23 s), then 0/1 passes, exit 101:
`UnexpectedSyntax` at bytes 287--293, line 14 columns 125--131. M6/M7 are not
reached; this is not an API compile failure or passing implementation evidence.
Disk remains 15 GiB free. The parent then authorizes only the AST/M6/M7/private
snapshot production slice, with a separate single-file static test owner.
The agreed metadata API retains exact budget, owner, typed clock domain,
source span/ref; absent annotations retain old identity, while complete M5
export of an annotated template rejects instead of discarding its condition.
Runtime/projection/provider implementation is outside this static assignment.

The four production files are then frozen for evaluation. Existing static
regressions pass: M6 classification 13/13 and M7 pipeline 27/27 (shared
9.10 s build), AST Surface M6 10/10 (0.21 s build), no warnings, disk 15 GiB
free. Commands use `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, `--locked`,
`cargo test -p mir-semantics --test surface_v0_classification_m6
--test surface_v0_pipeline_m7 -- --test-threads=1` and
`cargo test -p mir-ast --test surface_v0_m6 -- --test-threads=1`.
These old-source regressions do not establish new annotated runtime behavior.
The new test target is held for its separately authored private-snapshot
round-trip and malformed-binding checks; no passing new-target result exists
at this point.

Parent index invocation initially used the repository root, which the script
does not accept (`canon root not found`). Rerunning from `mirrorea_canon/`
regenerates and validates 213 entries. The first full docs check then rejects
the expanded status view at 184 lines against its 180-line limit. The parent
condenses only that current-view paragraph; the full `make docs` rerun passes
agent config, 213-file index, 800 hierarchy paths and 1760-report scaffold.
No semantics or test assertion is changed for these documentation repairs.

The new static target then runs 14 tests: 10 pass and four fail (0.39 s
build, 0.01 s tests, no warnings, disk 15 GiB). Three implementation defects
are exposed: the broad owner-expression collector swallowed a second
assignment, and the new nested span decoder accepted unknown fields and
inverted byte ranges. The repair recognizes subsequent assignment heads only
for annotated handlers, preserving the old unannotated collector, and uses
strict structural span decoding. The fourth assertion overclaimed component
restoration: a still-ordered changed byte offset cannot be checked against
source text absent that text or an independently expected artifact. Parent
and independent reviewer require it to restore as a distinct component, just
like a valid changed budget; the later expected-artifact/projection binder must
reject the mismatch. Wrong source-ref coherence, inverted ranges and unknown
fields remain rejection tests. No duplicate self-authenticating hash is added.

After the bounded repairs, new static tests pass 14/14 (8.78 s build), M6
13/13 and M7 27/27 (0.44 s build), and AST 10/10 (0.21 s build), with no
warnings and disk still 15 GiB. These passes do not resolve the independent
review's remaining P1: the public inspection accessor returns `&Core`, whose
`.cloned()` recreates the complete raw M5 export that the annotated conversion
rejects. The parent selects a non-executable inspection summary with no raw
Core conversion. A separately supplied compile-fail doctest is added before
the API repair to demonstrate the current escape. Static-slice acceptance
remains withheld until that falsifier and the repaired regression pass.

The doctest produces the intended RED: `cargo test --locked -p mir-semantics
--doc m5_owner_rmw_subcomponent -- --test-threads=1` reports 0/1 because the
`compile_fail` example actually compiles (6.20 s build). The repaired accessor
returns only an owned non-executable summary containing copied source-ref and
operation count, with no Core or conversion escape. Narrow independent
review resolves the P1. Final tests pass: new static 14/14, M6 13/13 and M7
27/27 (combined 6.82 s build), AST 10/10, and the same compile-fail doctest
1/1. Clippy first reports a test-only `single_match`; the test owner changes
it to equivalent `if let` without changing assertions. New static tests then
pass again 14/14 (0.44 s build, 0.01 s tests) and two-crate all-target
deny-warnings Clippy passes (2.63 s). Disk remains 15 GiB free.
These checks close the static implementation/review slice only, not a safe
annotated execution checkpoint: generated-contract preservation and every
runtime non-bypass path remain the immediate required consumers before any
source acceptance commit/push. Existing static proof claims are unchanged.

For that runtime consumer, the parent selects central M8 enqueue rejection
for unsupported annotated execution over introducing a new serve diagnostic
algebra immediately. Existing `M8ServeDiagnostics` always carries a terminal
outcome, so a new local rejection must not be represented as success or a
fabricated `RouteUnavailable`. The admitted owner plan must retain the exact
checked condition independently of any caller marker; ordinary enqueue then
rejects before occurrence allocation or mutation. A later private one-use
authorization path is separately required for actual gate-produced execution.

At `2026-09-07T20:26+09:00`, generated owner request/reply contracts and
private projection/SYS5 adapter snapshots retain the exact condition. The
first projection tests exposed an assertion expecting a later structural
diagnostic although the unchanged edge validator correctly rejects the
non-derived edge first; only that exact expected diagnostic was corrected.
The existing restricted-image regression then exposed a real defect: a
requester-only image was incorrectly required to carry the remote owner's
Core. Restoration now requires the exact local Core when the owner artifact
is retained, but does not invent/copy it into a requester-only image. Such a
restored image remains a candidate authenticated by the independently retained
expected-start binding, not by its own recomputed digest. Projection-budget
tests pass 8/8 and existing private projection snapshots 6/6. Independent
projection review reports no remaining P0/P1.

The separate M8 behavioral RED was 1/2: the unannotated failure-name-padding
control served `100 -> 90`, while the annotated request was wrongly accepted
by ordinary enqueue. The admitted M8 plan and its private snapshot now retain
the checked condition, and central ordinary enqueue returns the distinct
`OwnerAdmissionAuthorizationRequired` diagnostic before allocating request
occurrences, recording trace, queueing, or mutation. Repaired tests pass 2/2
(23.86 s build). Existing M8 admission/local-cut/owner-queue/patch targets pass
7/11/7/8, respectively (33 total; 1.26 s common build); projection-budget
8/8 (22.89 s build) and private projection snapshot 6/6 (0.25 s) pass again.
Commands use `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked
-p mir-runtime`, the named integration targets and `--lib
i3_owner_admission_budget_` / `--lib private_projection_snapshot`, with
`--test-threads=1`. Default runtime targets emit eight dead-code warnings;
library guards emit two, so these runs are not warning-free. Root free space
ends at 14 GiB; no cleanup is performed.

This is default fail-closed enforcement, not a running clock gate, one-use
permit, transported expiry, or I3-3 acceptance. Actual alternative workflow
negatives, private M8 image condition retention, broader regressions and the
independent M8 review remain required before the source checkpoint is committed.

Current-delta broader regressions subsequently pass: I2 local 5/5 (0.05 s)
and CLI 8/8 (9.73 s, 0.54 s build), I3 process-runtime integration 61/61
(47.74 s, 25.24 s build), and M10 conformance 67/67 (9.73 s, 1.15 s build).
The report-pinned `cargo clippy --locked -p mir-runtime -p mirrorea-i3-probe
--all-targets --features mir-runtime/i3-process-test-seams -- -D warnings`
then fails on three style lints: one production `collapsible_if` in the
projection budget check and two test-only `obfuscated_if_else` clause builders.
Their respective source/test owners receive bounded syntax-only repairs;
no successful rerun is claimed here. Disk remains 14 GiB free.

The production style repair initially introduces E0507 by calling consuming
`is_some_and` on a borrowed optional condition; adding `as_ref()` repairs the
borrow without cloning or changing the predicate. No B3 tests ran on that
compile failure. The test-only module hook and private-image JSON pointer were
also corrected by inspection before the subsequent run.

The next B3 feature-enabled SYS5 filter compiles (24.60 s) and runs 2 tests:
the changed private M8 budget component is rejected by the internal sealed-image
decode binding, but the actual default owner entry panics at
`m8_runtime_local_cut.rs`'s `owner_context_row`. This is a genuine integration
defect: contextual execution assumes every enqueue rejection has an M8 trace
row, whereas the new authorization guard intentionally rejects before trace
or occurrence allocation. The test is retained unchanged. The parent
authorizes the smallest typed pre-enqueue/unobserved error through local M8,
the OW1 contextual response and SYS4, preserving ordinary observed failures.
It must not fabricate a trace, successful outcome, or `RouteUnavailable`.
Private-M8 and M10 diagnostic tests have not yet run at this point. Eight
runtime dead-code warnings are emitted; root free space is 15 GiB. This
counterexample reopens the default-execution integration checkpoint despite
the earlier bounded two-file review; no commit/acceptance follows its green
component tests alone.

The contextual-result repair's first build exposes E0308 in the unchanged
serve-error return arm, which still returned the old boxed observation type.
Wrapping that actual observation in the new `Observed` variant repairs the
type mismatch without changing its semantics. No tests ran on this compile
failure. The repaired chain is frozen for renewed B3 tests; the separate kernel
diagnostic change still waits for its own behavioral RED.

After that repair, the feature-enabled SYS5 budget filter passes 2/2
(24.93 s build), and the private-M8 snapshot filter passes 2/2 (23.06 s
build). These execute genuine source-derived default owner entry and its
unannotated control, internal image-component mismatch rejection, exact
restricted-plan round-trip, and malformed owner/clock/span rejection. They
are runtime/private-codec evidence, not new OS-process or expiry evidence.
The image tamper negative is an internal sealed-image decode check; its
positive start uses a separately held coordinator binding, but the negative
does not claim to have reached that outer start validator.

The M10 source-execution diagnostic test then produces the intended behavioral
RED, 0/1 (24.56 s build): the lower authorization rejection is incorrectly
reported as `KernelDiagnosticKind::RouteUnavailable`. The parent authorizes
mapping only that new M8 guard to a distinct private
`KernelDiagnosticKind::OwnerAdmissionAuthorizationRequired` in both ST and
OW1 enqueue paths; every previous mapping remains unchanged. The Reference
backend is only an internal test backend or an unobservable initializer that
production M9 construction replaces before returning, not a supported annotated
execution path. The source repair is frozen for fresh validation. The test
owner also adds actual OW1 contextual-worker rejection, exact trace/state
nonmutation and clean shutdown, without inventing an OS/network claim.

Final default-guard integration results (`2026-09-07T21:00+09:00`):

| Gate on the frozen source delta | Actual result |
|---|---|
| `--test m10_source_execution` | 3/3, 24.74 s build; new diagnostic plus existing source controls |
| `--lib i3_owner_admission_budget_` | 11/11, 22.58 s build; projection, private M8 and actual OW1 worker |
| `--test i3_owner_admission_budget_runtime` | 2/2; final syntax-only test repair rerun builds in 0.38 s |
| `--test sys5_i3_process_runtime --features i3-process-test-seams` | 63/63, 25.31 s build, 47.97 s tests |
| `--lib --features i3-process-test-seams` | 298/298, 22.87 s build, 72.87 s tests |
| M8 admission/local-cut/owner-queue/patch targets | 33/33; 1.21 s build, owner-queue 9.42 s |
| `--test m10_conformance` | 67/67, 1.14 s build, 9.65 s tests |
| I2 local slice and CLI | 5/5 and 8/8; 0.53 s build, 0.04/10.03 s tests |
| `mirrorea-i3-probe --test i3_process_localnet` | 32/32, 32.25 s build, 46.62 s tests |
| `--lib --features i3-process-test-seams,i3-private-quic sys5_i3_private_quic_tests` | 2/2, 31.68 s build, 0.00 s tests |
| report-pinned two-crate all-target Clippy `-D warnings` | pass, final 0.12 s |
| `cargo fmt --all -- --check`; `git diff --check` | parent rerun passes after workspace formatter repair |

Cargo tests above use `cargo test --locked -p mir-runtime` unless the probe
package is named, `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, and
`--test-threads=1`. Default runtime builds emit eight dead-code warnings;
the budget library emits two, and the full test-seam library emits one warning
for three peer-preface test helpers. Do not call those test builds warning-free
or confuse the 298-test feature profile with a different earlier library run.
The final Clippy attempt first detects one additional test-only
`obfuscated_if_else`; the author changes only its clause builder to `if/else`.
The next Clippy passes, but workspace format detects two files previously
formatted under a different style. The source owner runs the workspace
formatter; only private projection/SYS5 import grouping and assertion wrapping
change. The parent's full format/diff check then passes. No assertion or
production behavior is weakened for lint/format acceptance. Root free space
remains about 14 GiB; no additional cleanup occurs.

Independent spec-first and code-quality integration review reports no P0/P1.
It confirms real endpoint dequeue/quarantine without an invented M8 observation,
exact guard propagation, unchanged old failure paths, and live OW1 rejection.
The recommended worker snapshot/shutdown and private-M8 round-trip/restriction
tests are now executed. Additional direct kernel-counter/SYS4-diagnostic-context
and coherent full-image binding falsifiers remain useful Stage C consumers;
they do not supply clock/permit/expiry evidence by implication. Read-only planner
review confirms the same consumer and whole-I3-3 pause boundary. This closes
the finite default-guard slice only: actual clock staging/resolution, one-use
permit, retained typed expiry/requester consumption and selected-QUIC delivery
remain next, with provider and every other required I3-3 family/order still
in scope. No new Lean/model run, theorem/ledger/lifecycle acceptance or public
compatibility claim follows.

Parent final documentation validation passes `make docs`: agent config,
213-entry Canon index, 800 required hierarchy paths and the 1760-report
scaffold. Staged whitespace validation passes. A targeted added-line scan
finds no private-key PEM, GitHub/OpenAI token or Discord webhook pattern;
this is a bounded credential-pattern check, not a general security proof.
Existing patch quiescence and whole-owner cut restoration must preserve that
condition; no new general patch/clock framework is authorized by this choice.

Two further ACK checks are required before acceptance: the completion must
carry the complete independent run/cohort/slot/program/projection/closure/
generation/kind/candidate binding through to cohort publication, not just
three possibly reusable SYS-4 references; and framed reads must consume one
absolute lifecycle budget, not reset a socket inactivity timeout on each
partial read. Cross-cohort completion and fragmented-reader deadline tests
are direct consumers. Opaque pending-ingress implementation ownership moves
to the probe implementer for `sys5_i3_private_quic.rs` only, independently of
the runtime owner's M9/SYS-4/process-image/ACK corrections.

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

The next slice starts from pushed checkpoint `87ee2418`. Four separately
authored integration tests cover pre-write reconnect success, post-admission
duplicate/uncertainty, modified or missing duplicate-commitment evidence, and
verified first-session retry misuse. The first focused Cargo invocation exited
101 with 16 compile diagnostics for the absent agreed retry APIs and related
type inference. No tests executed: this is intentional missing-API RED, not a
runtime counterexample. Runtime/adapter and probe owners then started disjoint
implementation. The `LocalAttemptRejected` declaration may be added to permit
compilation, but its generation guard must remain unchanged until the actual
verified-session test demonstrates the existing incorrect peer-blame outcome.

Parent test review caught an impossible evidence requirement in that last
negative: first-session misuse occurs before reconnect, so it cannot carry
the two-session retry audit's mandatory second delivery. The test and producer
contract now require a local rejection audit with actual first-session and
pending/nonmutation evidence, and no retry audit. The two genuine reconnect
profiles retain their mandatory second-session evidence. No fictitious
connection, delivery or owner outcome may be inserted to satisfy an audit type.

Session generation and begun runtime-attempt generation are separate: the
pre-write profile sends attempt 1 on session 2, whereas the post-admission
profile sends attempt 2 on session 2. The probe must capture the runtime's
actual source-requester binding, reason and attempt summary before a successful
receipt removes its pending record. Neither generation nor retry initiator
may be inferred from a profile label or minted by observer output.

In-progress parent flow review at `2026-09-07T14:30:56+09:00` identified two
schedule/evidence hazards before execution. The post-admission requester must
wait for the actual owner-close reply outcome and retain the returned opaque
pending handle, rather than close immediately after sending and race owner
admission. The verified-session misuse owner must check the actual receive
failure and zero serve/write summary, rather than discard a receive result and
report profile-selected nonmutation. The production owner corrected both in
source; these are not yet executed passes. The successful reconnect reply also
needs the existing peer-close lifecycle discipline so stream `finish` is not
mistaken for requester receipt before closing the owner connection.

The next exact reconnect-filter compilation stopped with four probe errors:
two mismatched local variable names, one unresolved delivery tuple type, and
use of the retry audit after consuming its enclosing failure value. Exit was
101 after approximately 27 seconds; no tests executed and the separate local
misuse behavioral RED was not run. Disk remained 18 GiB free, with no competing
Cargo/rustc processes. The production owner received only these bounded fixes;
test expectations and semantic behavior are unchanged.

After those compile fixes, the same source-bound actual-process reconnect
filter passed 3/3 (18 filtered; build 4.75 seconds, tests 6.03 seconds). The
separate verified-first-session misuse test then produced the intended
behavioral counterexample: required `Some(LocalAttemptRejected)`, actual
`Some(PeerBindingRejected)`, exit 101, 0/1 passed, 1.19 seconds. Unlike the
earlier missing-API compilation failure, this test executed the verified QUIC
session and exposed the incorrect local-error classification. The runtime
owner then split peer/preface validation from the local generation guard,
preserving genuine peer failure while returning the zero-payload local error
before runtime authorization. Fresh post-fix verification remains pending.
The two unread intermediate probe verification fields were redundant with
the actual child-rejection-to-public-audit path and were removed, rather than
suppressing the compiler warning or adding profile-derived evidence.

Post-fix execution supersedes that pending check: the verified-session misuse
filter passed 1/1 (33.80-second build, 1.15-second test), and the complete
`i3_process_localnet` integration target passed 21/21 in 32.17 seconds. The
subsequent focused two-crate all-target Clippy command exited 101 on three
probe style diagnostics: two excessive-argument signatures and one nonminimal
boolean. These are not a behavioral failure, but deny-warnings validation is
not yet passing for this slice. The owner is making bounded signature/boolean
changes; independent review is in progress. Disk remained 18 GiB free and no
full-workspace validation or cleanup was performed.

The independent retry-slice review then found a P1 accounting defect: the
post-admission retry early return omitted the previously observed owner
`Ready` fact, so the public rejection audit reported zero owner starts while
validated admission and mutation counts were one. The test owner added exact
owner-start assertions to the normal path and all three commitment-falsifier
cases. The focused normal path reproduced `0 != 1` at test line 1162 (exit
101, 0/1, build 4.84 seconds, test 1.35 seconds). The production owner is
retaining the actual observed-start fact on that early return; no admission,
mutation, or successful-recovery fact is inferred from the retry profile.

That one-line correction passed the full actual-process target again: 21/21,
build 4.60 seconds and tests 32.02 seconds. The next all-target lint invocation
exposed a cfg(test)-only call lacking the newly added optional retry-audit
argument; the test owner supplied `None` for that existing non-retry helper.
The lifecycle unit target was not run after the failed compile, so it remains
pending until the final rerun.

The independent reviewer completed this slice with no further P0/P1 and two
P2 audit repairs, both accepted by the parent. First, fields sourced only from
a `Rejected` child event cannot report `false/0/0` when handled-fault events
provide no such observation. The smallest correction is explicit optional
control/certificate/handshake observations, not synthesizing counts from the
retry profile; actual retry-session facts remain in their child audit. Second,
the two runtime attempt summaries must carry the same exact requester binding,
not merely two nonempty strings. An observer-only changed reconnect binding
must reject the join without altering the actual request, owner mutation or
retained uncertainty. The test owner is adding these narrow regressions before
production changes. These repairs do not authorize further profile or framework
expansion, and are not complete I3-3 evidence.

The final P2 test-first command produced 10 compile diagnostics (nine
scalar-versus-optional observation assertions and the absent binding-falsifier
variant), exit 101, with no tests executed. Production now reports those
rejection-event-only observations as `Some(actual)` or `None`, and requires
equal requester bindings across the two attempts. The binding falsifier changes
only the emitted observer summary after the actual retry, not carrier bytes or
runtime state. Rejected joins preserve the independently source-bound requester
pending observation and actual terminal admission/mutation counts, but publish
neither an accepted retry audit nor a duplicate owner conclusion. Accepted
post-admission joins also propagate the validated pending fact into the generic
error audit. These changes await the final full-target run and narrow review.

Final actual-reconnect checkpoint, `2026-09-07T15:00:30+09:00`: the full probe
integration target passed **22/22** (build 5.08 seconds, tests 34.74 seconds),
focused two-crate all-target Clippy with the selected private feature and
`-D warnings` passed (6.82 seconds), and probe lifecycle unit tests passed
**2/2** (one filtered; build 2.42 seconds). All Cargo commands used
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2` and `--locked`; disk remained 18 GiB
free throughout. Fresh formatting and diff checks passed. These results
supersede the pending/error checkpoints above for this source delta.

The narrow independent re-review resolved both P2 mechanisms and retained the
P1 disposition, with no remaining finding in the reviewed slice. It confirmed
unknown versus observed transport facts, exact attempt-binding equality, the
observer-only falsifier, and conservative requester-only pending retention on
join rejection. No actual-reconnect issue remains open in this bounded slice;
authority withdrawal between sessions, late replies/order, provider/time/cut,
the full 20-family matrix and I3-3 acceptance remain open. Prior 51/51 runtime,
285/285 library and bounded-model evidence is retained at checkpoint
`87ee2418`, not relabeled as a new full-suite execution. The current source
delta only refines the private QUIC adapter and probe execution/evidence path.
The main agent also ran `make docs` successfully: agent configuration, the
210-file Canon index, all 800 hierarchy paths, and the 1760-report scaffold
passed. A bounded private-key/token pattern screen of the source diff found no
match. These checks do not establish a general secret/noninterference proof.

The advisory planner review recommends completing this slice before adding
the remaining authority/order controls. Same-production-boundary codec,
target/provenance, authority-precedence, capacity and result-admission tests may
be reused with exact source/runtime correspondence; actual process controls
must establish their network-dependent claims. Every family remains required,
but each parser negative need not become a new subprocess profile. Process
startup/reaping and exact joins should be reused when a direct consumer repeats
them. No broad harness framework refactor or weaker evidence gate is authorized
by this advice; provider/time remain OPEN and I3-4 remains inactive.

## Suggested next prompt

Continue active I3-3 in the original Plan 250 program: map all 20 failure
families and ordering edges to accepted runtime boundaries, resolve operation-
specific duplicate/ambiguity handling, and implement/validate bounded real fault
controls. After full I3-3 acceptance, validation, review, commit/push and remote
parity, honor the latest owner-requested pause. I3-4 requires explicit resume;
the retained program is not blocked, stale, or completed by that pause.

## Plan update status

`plan/` 更新済み: Plan 250 now records the resume cut, complete active I3-3
Goal Statement and work/evidence/decision gates. The parent goal, all failure
rows, ordering edges and later milestone order are preserved. plan/00-index
synchronization belongs to the status assignment.

## Documentation.md update status

更新済み: resume pointer now identifies I3-3 as the sole active milestone,
with the accepted I3-2 runtime retained as its input.
Delivery checkpoint: `Documentation.md` 更新不要; its current entry pointers,
owner-pause boundary and provisional workflow remain accurate. No CLI or
sample-root change is introduced here.
Time slice: updated the reader pointer from the prior successor checkpoint to
the selected contract and validated static slice, explicitly retaining pending
generated-contract/runtime enforcement and the unchanged workflow boundary.

## docs/project-status.md update status

更新済み: owner resume、I3-3 sole active、後続 milestone inactive を同期した。
I3-2 accepted evidence と official I3 unentered を維持する。

## progress.md update status

更新済み: current I3-3 pointer, preserved three axes and startability mirror
the resume; the actual-time recent log is 2026-09-07 11:38 JST.
The current delivery checkpoint also synchronizes Plan 250, progress/tasks,
project status and sample dashboard with fresh versus prior validation and
time-then-provider as the next still-OPEN contract consumer.
The subsequent time slice supersedes that pointer: ADR-0041/spec/16 is
selected, static tests/review pass, generated contracts/runtime gating remain
pending and provider alone remains an OPEN contract. The 19:56 JST snapshot
and recent log separate this evidence from whole I3-3 acceptance.

## tasks.md update status

更新済み: active I3-3 and inactive later packages are separated from accepted
I3-2 history. The parent-requested current-goal/direct-consumer coherence
correction is applied: I3-4 consumes I3-3 and resume is resolved history.

## samples_progress.md update status

更新済み: the sample dashboard records resumed I3-3 as active/not accepted,
with all 20 families explicitly a target. Accepted I3-2 runnable evidence
retains its existing class. No new workflow-ready or product-completion claim.
The time slice adds the 14-test static command as evidence only; its embedded
source strings are not a new active sample root or network workflow.

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
the verified first I3-3 slice, not milestone acceptance. It was committed as
`87ee241898b58c40bb5fcafa98df39a815d57498`
(`feat: checkpoint I3 request lifecycle and fault evidence`) and pushed to
`origin/main`; fresh `git ls-remote origin refs/heads/main` matched HEAD and
the worktree was clean before the next actual-session test assignment. This
update accompanies that next slice, not a metadata-only commit. Parent owns
accepted source/evidence pinning, integration commits, authorized pushes and
fresh remote parity before the fixed transition to I3-4.

The actual-reconnect slice was subsequently committed and pushed as
`7580e3a1298dd39bc26ef1979c5033d485e92de5`
(`feat: exercise I3 actual-session reconnect and retry`). Fresh remote lookup
matched HEAD and the worktree was clean before this successor-slice update.
This is another source/evidence checkpoint within active I3-3, not milestone
acceptance or I3-4 activation.

The genuine-revocation/retained-ingress slice and its reviewed repairs are
committed and pushed as `cb050be23be6bc2cbc420b24f8d8c583503d6fff`
(`feat: revalidate retained I3 ingress after genuine revocation`). Fresh
`git ls-remote origin refs/heads/main` matched HEAD at
`2026-09-07T18:00:05+09:00`, and the worktree was clean before the next
delivery assignment. This is an I3-3 checkpoint, not milestone acceptance.

The separate docs-only owner-control commit
`386d5f09d779fd327ecdc616234cbb34d4f75241` schedules a pause after full I3-3
acceptance; its push/parity was confirmed before the delivery checkpoint.
The current delivery delta has passed source/test review and the fresh gates
above and is ready for the parent's scoped commit/push. Its exact committed
cut and parity will be recorded with the next in-scope I3-3 work; this report
does not predict a commit hash or treat the whole milestone as accepted.

That delivery checkpoint is now committed and pushed as
`31be54b9ec83b3a75a56f6d71a5523674237b035`
(`feat: exercise I3 endpoint and generated-frame delivery faults`). Fresh
remote lookup matches HEAD at `2026-09-07T19:04:33+09:00`, with a clean
worktree before the next contract-first assignment. I3-3 remains active and
unaccepted; the next work does not activate I3-4.

The owner-admission contract/static/default-guard integration is prepared as
the next scoped source checkpoint over `31be54b9`, after the executed repair
and regression gates above. Parent updates this report, Plan 250,
Documentation, project status, progress, tasks and sample dashboard together;
no new runnable network sample is promoted. Its commit/push and exact parity
will be recorded with the next in-scope Stage C work, rather than predicting
a hash here. This checkpoint does not trigger the scheduled whole-I3-3 pause.

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

At the default-guard checkpoint, static, projection, runtime, test, evaluation,
independent integration review and read-only planner assignments have returned
their bounded results. Contexts are retained for Stage C; production source
and tests are frozen until the parent finishes this checkpoint's commit/push.
The parent continues I3-3 and does not send task completion or activate I3-4.
