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

Provider Stage 2c starts from accepted inactive Stage 2b
`7142b205b4e2805d50ce78156de99e6feb1db37a`, observed clean with live remote
parity at `2026-09-08T19:22:07+09:00`. All subsequent source/test/LAB changes
belong to this continuation. Parent fetches `origin/main` again before
integration and observes zero ahead/behind; no user source edits are discarded.
Owner housekeeping increases available storage; the parent performs no further
cleanup beyond the separately authorized five old rlibs recorded below.

Provider Stage 2b starts from static cut
`3862b1688821c8fb243f8f148f54cd313ad3b163`. Parent verifies HEAD, origin/main
and live remote equality plus clean worktree at `2026-09-08T15:41:54+09:00`.
All subsequent initial LAB changes belong to this continuation; no user
changes are overwritten. Final Stage 2a resource gate retained about 11.03
GiB free, with no cleanup. The source/guard predecessor is `9e8d674a`.

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

Provider Stage 2c starts from accepted inactive cut `7142b205`. Parent selects
the private mixed-image variant in the existing SYS4/SYS5 pipeline after the
bounded Oracle advisory and local seam inspection. Ordinary evidence remains
ordinary; no reconstructed final-verdict input is permitted for the composite.
The existing finite-local grant issuer and active-snapshot inventory are shared,
not replaced by a second authority system. Actual composite/ContractUpdate
facts are retained at sealing, then restricted into inactive child candidates.
This is a direct consumer of Stage 2b and supplier to actual host/runtime work,
not a new milestone or provider invocation claim. Parent assigns one coupled
production writer, a separate private test author and one evaluator. The first
gate is a fail-closed scaffold's actual positive assertion RED. Exact packet:
`/tmp/i3-provider-stage2c-parent-packet-20260908.md` (working input, not Canon).

Owner resume observed `2026-09-08T18:43:01+09:00`: the owner approves the
previously listed five old regenerable runtime-library artifacts and again
requests execution through full I3-3 close, then stop. Parent rechecks exact
regular files/sizes/link counts and absence of active Cargo/rustc, deletes only
those five explicit paths, and observes 12,267,094,016 bytes free afterward.
The files are recoverable by rebuilding; no source, Git history, evidence log
or other cache is removed. The 10 GiB pre-command guard remains in force.
Product goal remains paused; manual authorized execution resumes without
changing the sole roadmap/milestone or activating I3-4. Test and implementation
specialists resume only the already-reviewed Stage 2b constructor negatives
and minimal hardening before its integration.

While Stage 2b production/tests proceed, a read-only planner check confirms
the later row-20 direct consumer: actual process-owned quiescent cut admission
followed by a legal transition, with retained-ingress/in-flight rejection and
late decided-traffic non-resurrection. Parent verifies this against Canon
plan/05 (failure matrix and ordering), spec/17 and Plan 250. Existing local
SYS4 save evidence alone is insufficient; process/runtime and adapter
obligations must participate. The supported positive may be admission-only,
without a complete process snapshot, durability or live distributed patch.
Provider-bearing unsupported export may still reject, but cannot replace the
supported profile's positive. This is remaining-scope clarification, not new
implementation/evidence, a second active frontier or an I4 entry.

Stage 2b parent integration additionally checks two concrete consumer gaps.
The generic static finite checker validly permits additional preconditions;
the new runtime may not accept arbitrary same-source evidence by identity/
coverage alone. It compares the supplied intact discharge to the exact
canonical MembershipAuth-strengthened discharge, preserving missing-input
rejection and without a raw inner-proof accessor. A genuine stronger static
discharge supplies the runtime-negative case, not a Stage 2a defect.

The scaffold's process-local counter is inadequate as an eventual transferable
resource incarnation. Parent selects an independently OS-random 32-byte T0
incarnation over extending that counter with PID/path/session identity.
`getrandom` 0.4.3 is already present in Cargo.lock and the local registry;
parent verifies `fill(&mut [u8]) -> Result<(), Error>` in that exact source and
the [official API documentation](https://docs.rs/getrandom/latest/getrandom/fn.fill.html).
Only its existing dependency edge is added directly to mir-runtime, with no
version upgrade or new package nodes. Entropy failure rejects setup without
fallback. Randomness is not permission or a general uniqueness/security proof.
Actual M9 membership, policy, binding lifetime and exact-coordinate checks
remain necessary. Future B-only transfer uses the existing separately trusted
FD-3 control seam, not tainted images or QUIC; that transfer is not implemented
by this component and private namespace material cannot enter observers.

Stage 2b consumes the reviewed static cut. Parent accepts reuse of the real
M9/M8 machinery but rejects an initial proposed convenience API that omitted
explicit bootstrap/auth, intact verification and separate policy inputs.
Correct checked source cannot synthesize these approvals. The refined route
requires genuine setup binding, exact finite-local bootstrap validation and
membership authentication, full composite discharge, and separate fixed
effect policy before a provider-specific capability/witness. The capability
scope cannot be constructed through an unguarded generic grant route.
Production lifetime retirement/Drop invalidates binding; a test-only stale
flag is insufficient. Ordinary M8 exports must reject scoped component loss,
and mechanically necessary caller changes must preserve legacy behavior.
This is implementation boundary clarification within spec/17, not a new
normative contract, authority shortcut or provider execution claim.

After the pushed Stage 1 cut, parent adopts the smallest activation-pending
composite implementation route within spec/17. One temporary Oracle consult
(`i3-provider-composite-handoff-20260908`, actual 6Pro picker verification)
returns advisory candidate A; full local output is
`/tmp/i3-provider-composite-handoff-20260908.md`. Independent planner checks
the real M8-before-M9 ordering and incomplete mixed materialization hazard.
Parent retains static projection before actual admission/verification, rather
than making pure projection depend on a finite verifier verdict. It also
requires exact provider-effect retention/source-label binding in the composite
checker, without changing generic legacy refinement/redaction. Exact Stage 2
sequence and disjoint ownership are recorded in Plan 250; no new Canon rule,
milestone, authority issuer or transport is introduced by this coordination.

Provider Stage 1 implements the selected source-only profile and rejects all
legacy execution/export paths lacking a dedicated provider runtime. Independent
review exposed identifier restoration, constructor visibility and M8 snapshot
lowering-tag gaps; genuine failing tests preceded the bounded fixes. No provider
call, grant, generated network edge or new normative rule belongs to this slice.
Parent synchronizes current LAB snapshots after final source/runtime regression
and quality checks. A single asynchronous Oracle consultation examines the next
mixed M8/M9 coverage handoff; its advice is not authority or executed evidence.

Provider contract checkpoint after the time/reply integration: parent wrote
PROPOSAL-045/spec/17 candidates, using the existing verified
Oracle advice as advisory input and a fresh Canon-first planner review. The
smallest path keeps a distinct source effect/grant/result at the already
remote locus and shares selected QUIC framing. No extra provider process or
I5 registry is required. Actual bounded T0/T4 fixture I/O, exact typed failure,
retention and current-binding consumption remain required. The Oracle's
terminal-ambiguity suggestion is not adopted; lost results remain pending and
remote-unknown. Independent contract review found five P1 gaps: failure
taxonomy, call-start/revocation ordering, provider incarnation, resource
activation/accounting and observation separation. Parent corrected all five
plus role/cardinality/decimal/allowance P2 wording; narrow re-review returned
P0/P1/P2 zero. Parent adopts the finite contract through ADR-0042 under
ADR-0034, without inventing a new owner message. No provider code/execution
or milestone acceptance is claimed. This is an explicit normative addition,
not a LAB reinterpretation of accepted owner RMW or authentication.

The actual I3 localnet constructor at
`crates/mirrorea-i3-probe/src/i3_process_localnet.rs` uses a 15-second deadline
and 1-second reaper allowance. The reviewer also cited the separate canary's
`process_harness.rs` defaults; that is not substituted for actual I3 child
evidence. Spec/17 binds these finite operational maxima, preserving the existing
non-hard-real-time/host-suspension non-claim rather than promising a new clock
or CPU scheduling theorem. Source/test implementation starts after this
docs-only integration and a behavioral RED test.

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

Provider Stage 2a changes semantics finite-refinement/coverage and the existing
checked identity query; SYS3 pure lowering/model, new static provider projector
and private static snapshot; SYS4/SYS5 explicit unsupported boundaries; and
SYS6 metadata-name arms only. Separate source tests are
`crates/mir-semantics/tests/i3_provider_composite.rs`; the runtime cfg-test
module is `crates/mir-runtime/src/sys3_provider_effect_tests.rs`, outside the
production projection directory. Plan 250, Report 2606, Documentation,
project-status, progress, tasks and samples_progress are synchronized. No
Canon, sample-root, script taxonomy, wire/runtime activation or lifecycle
change belongs to this component.

Provider Stage 1 changes AST surface/parser support; semantics classification,
pipeline, private snapshot and static M9 refinement; runtime M8/M9/M10/kernel,
SYS3 projection/private snapshot and SYS5 unsupported guards. It adds the
read-only provider AST/Core/snapshot modules, source test target, runtime guard
test target and ordinary source-only provider sample; M8 private snapshot tests
gain two negatives. Plan 250, this report, Documentation, project-status,
progress, tasks, samples_progress, samples/README and scripts/README are
synchronized. Canon is unchanged from the adopted contract cut `985ee179`.

Provider contract integration adds Canon PROPOSAL-045, ADR-0042 and spec/17;
updates Canon README/MAP/spec and ADR navigation, CHANGELOG, plan/05 and the
generated INDEX; and synchronizes Plan 250, this report, Documentation,
project-status, progress, tasks and samples_progress. No Rust source, test,
sample root or artifact changes belong to this docs-only checkpoint.

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

Stage C additionally changes `mir-runtime/src/lib.rs`,
`m8_owner_admission_gate.rs` (new private linear handoff),
`m8_runtime_owner_queue.rs`, `m8_runtime_local_cut.rs`,
`sys5_i3_owner_admission_tests.rs` (new source-derived unit tests), and
`sys4_dispatch_tests.rs` (genuine queued-request falsifier). The later
`30429d5` integration adds the ordinary budgeted source at
`samples/clean-near-end/mirrorea-i3-owner-admission/main.mir` and its reader
entry; the current reply-replay delta reuses it without adding another sample
or public control surface. The replay delta touches the two existing SYS5 I3
runtime/adapter files, the three existing probe production files and the probe
integration test, plus Plan 250, this report and five current LAB snapshots.
It changes no Canon rule, theory ledger or official lifecycle.

## Commands run

Provider Stage 1: sole evaluator uses `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`,
locked dependencies and serial tests. Commands include source
`cargo test --locked -p mir-semantics --test i3_provider_effect`, the constructor
privacy doctest, runtime `--lib` with `i3-private-quic,i3-process-test-seams`,
and integration targets `i3_provider_effect_guards`, `sys5_i3_process_runtime`,
`m8_runtime_admission`, `m9_external_boundary`, `sys5_local_slice`, `sys6_i2_cli`
and `m10_conformance` using that same feature union. Exact invocations are kept
with the individual evaluator logs below. Quality commands are
`cargo clippy --locked -p mir-ast -p mir-semantics -p mir-runtime --features
mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams --all-targets --
-D warnings`, `cargo fmt --all -- --check`, and the all-workspace `cargo check`
recorded under final integration. Test results, compilation and static checks
remain distinct evidence classes.

Provider contract integration: read-only git/disk/time/source checks;
`python3 meta/build-index.py` (216 files indexed); `git diff --check`;
docs-only evaluator `make docs` and changed-text credential-pattern scan
(exact results recorded below when returned). Discord progress was sent at
the natural contract checkpoint; no complete notification or user-task stop.

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

Initial Stage C behavioral falsifier, after pushed `050f5067`:

```bash
CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-runtime --test sys5_i3_process_runtime --features i3-process-test-seams i3_owner_admission_budget_sys5_source_request_stages_without_owner_effects_and_keeps_the_unannotated_control -- --test-threads=1
```

## Evidence / outputs / test results

Stage 2c parent integration: the first `make docs` stops at the report's
project-status update section because the validator forbids the literal
`pending` there even when referring to Git integration rather than an
unfinished document update. The section now states the current integration
stage; no milestone or Git success is substituted. The complete docs command
is rerun after this wording correction. Staged added-line credential-pattern
scan finds zero matches in the sixteen owned files; this is a limited pattern
scan, not a general proof of secret absence. Staged whitespace check passes.

Stage 2c post-review frozen packet is complete under
`/tmp/c3-stage2c-final-packet-20260908.gTUvUc/`. Parent reads the actual logs:
focused13/13 (9.65 seconds, included in the library total), scoped three-crate
Clippy exit0 (22.06 seconds), workspace format exit0, runtime library375/375
(88.17 seconds), process63/63 (49.19 seconds), M10 67/67 (10.35 seconds),
I2 local5/5 (0.05 seconds), I2 CLI8/8 (10.47 seconds), and public provider
guards5/5 (0.34 seconds). Logs are respectively `01-provider-module.log`,
`02-clippy.log`, `03-fmt-check.log`, `04-runtime-lib.log`, `05-process.log`,
`06-legacy-regressions.log` (three distinct targets), and
`07-provider-guards.log`. Actual ordinary localnet regression then passes
41/41 in 59.92 seconds (`08-probe-localnet.log`), and workspace all-target
check passes in 17.41 seconds (`09-workspace-check.log`). They emit no
warnings or failures. Every command passes the pre-run 10 GiB disk guard;
final free space is 29,851,475,968 bytes. The workspace check compiles the
runtime, probe and CLI but is not workspace test execution or their complete
Clippy coverage. No probe-all-tests, actual provider invocation, whole-I3-3
matrix/order or public/production acceptance claim follows.

Stage 2c reviewer falsifiers execute: 11/13 pass, 362 filtered, exit 101,
compile 28.78 seconds and tests 9.99 seconds, no warnings. Log:
`/tmp/c3-stage2c-review-falsifiers-red-20260908.53sBo2/01-provider-module.log`.
Both proposed P1 cases are reached: changed endpoint metadata with a refreshed
candidate digest is accepted, and actual trusted setup retirement between
structural validation and receipt construction is accepted. The latter uses
capacity-one channels and bounded waits, not fabricated currentness flags.
The new decoded ordinary-start rejection passes. The coverage table now
distinguishes a retained non-provider M8 row mutation from removal of one
actual excluded provider source/Core/ordinal association; both reject.
Parent reads the full output and authorizes exact parent-commitment equality
plus one final parent-local currentness point, with receipt facts captured
there rather than later contradictory reloads. The stale scaffold comment is
also assigned for correction. Post-run free space is 30,797,791,232 bytes.
These findings and repairs do not change the contract into a continuous or
cross-process authority guarantee.

Stage 2c pre-review-repair regressions pass runtime feature-union library
372/372 in 86.90 seconds and SYS5 process integration 63/63 in 49.82 seconds,
with no warnings (`03-runtime-lib.log`, `04-process.log` under
`/tmp/c3-stage2c-regression-final-20260908.21X7Qr/`). The parent stops the
remaining sequence after the running command finishes when independent
review identifies two expected-start defects: a candidate's recomputed image
digest is not compared with the separately retained parent commitment, and
mid-validation setup retirement can return an `Ok` receipt whose currentness
flag is false. These are acceptance blockers despite the passing existing
tests. New deterministic metadata-tamper and real-retirement falsifiers are
being authored; a bounded test-only synchronization handshake avoids hanging
on an unexpected earlier rejection. M10/I2, provider guards, actual ordinary
localnet and workspace check are not run in this interrupted packet. The
pre-repair passes are not final-cut evidence.

Stage 2c final-quality recheck passes after the exact lint-only removal:
`cargo clippy --locked -p mir-ast -p mir-semantics -p mir-runtime --features
mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams --all-targets
-- -D warnings` exits 0 in 22.60 seconds, and
`cargo fmt --all -- --check` exits 0. Logs:
`/tmp/c3-stage2c-regression-final-20260908.21X7Qr/01-clippy.log` and
`02-fmt-check.log`. No warnings are emitted. Post-format free space is
30,824,005,632 bytes. Frozen runtime/process/I2/probe regression and independent
source review follow; this quality checkpoint alone does not close Stage 2c.

Stage 2c Debug repair passes all ten focused tests, 362 filtered, 7.57 seconds
after 28.13 seconds compilation, no emitted warnings
(`/tmp/c3-stage2c-debug-quality-green-20260908.ZgIdD0/01-provider-module.log`).
Expected-start Debug now emits only slot, assigned loci and safe parent/image
references; private snapshots remain available for exact trusted comparison,
not formatting. Scoped Clippy then reports four unfulfilled narrow lint
expectations (`02-clippy.log`); parent authorizes only their removal, retaining
the needed inactive impl-root annotations. Independent source reviews and
frozen regression are pending. The read-only planning review gives conditional
GO with the original component gates, followed by actual provider runtime and
remaining I3-3 obligations. Neither review planning nor focused tests establish
component or milestone acceptance.

Stage 2c expected-start Debug privacy falsifier executes after bounded layout
and dead-code cleanup: 9/10 pass, 362 filtered, exit 101, compile 32.67 seconds
and tests 8.17 seconds, with no emitted warnings. Log:
`/tmp/c3-stage2c-provider-module-debug-red-20260908.GsB89Q/01-provider-module.log`.
The public expected-start type's derived Debug traverses the newly retained
private component/authority DTOs. The test examines genuine formatted data
locally and emits only a static failure message, not the private rendering.
Parent reads the actual failure and authorizes a reference-only custom Debug
implementation and correction of its retained-data documentation. This is
neither a host call nor a trusted-start admission bypass. Post-run free space
is 30,835,892,224 bytes; final Clippy/regression gates are still pending.

Stage 2c focused behavior first turns GREEN: 9/9, 362 filtered, 7.77 seconds
after 31.19 seconds compilation
(`/tmp/c3-stage2c-module-quality-20260908.5W4LFu/01-provider-module.log`).
The new nested M8 private snapshot is version 2 with a required scope tag;
ordinary restore rejects the provider component, and missing/unknown tags
cannot default into ordinary scope. The existing deployment validator is
shared, and exact legacy inventory queries derive from the actual M9
restriction/filter/snapshot comparison rather than a nonempty digest.
These are private/provisional implementation changes, not compatibility claims.
The following scoped three-crate all-target Clippy run with warnings denied
fails on inactive-only dead-code and two large-enum layouts
(`02-clippy.log` in the same directory); workspace format is not run after
that failure. Parent reads both logs and delegates bounded quality repairs.
Separately, a new expected-start Debug privacy falsifier is authored against
genuine retained data; its execution is still pending at this checkpoint.
No component acceptance, actual host call or whole-I3-3 acceptance follows.

Stage 2c extended falsifiers reach their actual boundaries after the identity
repair: nine module tests yield 7 passed / 2 failed / 362 filtered, exit 101,
compile 38.52 seconds and tests 8.74 seconds. Log:
`/tmp/c3-stage2c-provider-module-nine-rerun-20260908.sgX2US/01-provider-module.log`.
First, extracting the component DTO from genuine codec-produced image bytes
permits ordinary M8 restoration because the nested snapshot loses its scope.
Second, the new inactive handoff accepts one slot containing all four loci,
unlike the existing two-or-more-slot deployment validator. These are reached
behavior failures, not claims of an external trusted-start bypass. Parent
reads both failures and authorizes scope-preserving private serialization and
reuse of the existing deployment validator. All prior seven checks, including
foreign checked identity, now pass. Five warning groups remain, and post-run
free space is 30,845,276,160 bytes. A preceding import typo caused E0432/E0425
and no test execution
(`/tmp/c3-stage2c-provider-module-two-reds-20260908.zCtJCh/01-provider-module.log`);
the exact import-only repair precedes this behavioral run.

Stage 2c incremental repair checkpoints (not component acceptance): the
bounded compiler repair passes the same library check in 12.77 seconds, with
eight warning groups and no tests
(`/tmp/c3-stage2c-runtime-lib-check-rerun-20260908.0l1roX/01-runtime-lib-check.log`).
The subsequent seven-test module run compiles but all seven fail at genuine
handoff with `InactiveProcessHandoffRejected`, before codec or intended
negative boundaries
(`/tmp/c3-stage2c-provider-module-red-20260908.QavNlg/01-provider-module.log`).
The existing exact M9 restriction correctly rejects the missing retained
relation-reacquire association: issuance was retained, but the already
validated lifecycle facts were discarded. The bounded repair retains those
facts and binds the existing translated authority; it does not issue new
authority or synthesize ordinary final evidence.

After that repair, the same focused command
`cargo test --locked -p mir-runtime --features
i3-private-quic,i3-process-test-seams --lib sys5_i3_provider_composite_tests
-- --test-threads=1` (incremental off, two build jobs) reaches 6 passed / 1
failed, 362 filtered, compile 36.50 seconds and tests 7.70 seconds. Actual log:
`/tmp/c3-stage2c-provider-module-rerun-20260908.D45gJY/01-provider-module.log`.
The genuine A/B codec, scope, trusted-binding, retirement and ordinary-start
guards pass. The remaining reached behavioral falsifier shows that an image
inspection query incorrectly accepts a different valid checked identity
(test line 346). Parent reads the actual failure and authorizes a bounded
query-association repair. Four warning groups remain; post-run free space is
30,849,011,712 bytes. These are inactive-image checks, not provider calls,
host reads, cross-process revocation or I3-3 acceptance.

Stage 2c coherent-source incremental check:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo check --locked -p mir-runtime
--features i3-private-quic,i3-process-test-seams --lib` exits 101 with 18
compiler errors and no test execution. Exact log:
`/tmp/c3-stage2c-runtime-lib-check-20260908.U6XuHx/01-runtime-lib-check.log`.
Errors are actual shared-liveness-cell type mismatch, moved M9 base/coverage,
misplaced relation-reacquire fields, Vec/set deployment drift, missed ordinary
seed-field migrations, a missing expected-binding tag and a moved locus set.
Parent reads all diagnostics and delegates bounded compile repairs without
default authority, copied liveness flags or discarded tag checks. Pre/post
free space is 30,877,523,968 / 30,877,278,208 bytes. This is a compile
checkpoint failure, not a test failure or accepted inactive-image result.

Stage 2c first behavior RED, inspected `2026-09-08T19:32:46+09:00`:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-runtime
--features i3-private-quic,i3-process-test-seams --lib
i3_provider_composite_validates_exact_all_locus_inactive_images_after_codec_roundtrip
-- --test-threads=1` exits 101. One actual test body runs: 0 passed, 1 failed,
363 filtered; after genuine source/setup/M9/policy sealing the positive
expectation fails at `InactiveProcessHandoffPending` (test line 181). This is
not a missing-symbol/compiler-only RED. Compile takes 27.71 seconds, test
0.03 seconds; the uninhabited scaffold yields nine unreachable/unused
warnings, not final quality evidence. Log:
`/tmp/c3-stage2c-focused-red-20260908.Ezxnpb/01-focused-red.log`.
Pre/post free bytes: 30,893,686,784 / 30,893,654,016. Parent reads the actual
output and authorizes behavior plus separate falsifier implementation.
No Stage 2c successful image, authority activation or runtime claim follows.

Stage 2b final frozen regression, observed `2026-09-08T19:07:58+09:00`,
supersedes intermediate results below. Parent reads the final logs in
`/tmp/c3-stage2b-regression-packet-20260908.thVbcd/`. Commands use
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, locked dependencies, feature union
`i3-private-quic,i3-process-test-seams`, package `mir-runtime` and serial tests:

| Executed target | Actual result | Log |
| --- | --- | --- |
| `cargo test --locked -p mir-runtime --features i3-private-quic,i3-process-test-seams --lib -- --test-threads=1` | 362/362, 83.93 seconds | `01-runtime-lib.log` |
| Same package/features, `--test sys5_i3_process_runtime` | 63/63, 53.08 seconds | `02-process.log` |
| Same package/features, `--test m10_conformance --test sys5_local_slice --test sys6_i2_cli` | separate targets 67/67, 5/5, 8/8 | `03-legacy-regressions.log` |
| Same package/features, `--test i3_provider_effect_guards` | 5/5, 0.36 seconds | `04-provider-guards.log` |
| `cargo check --locked --workspace --all-targets --features mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams` | exit 0, 17.14 seconds; compile coverage only | `05-workspace-check.log` |

The final provider filter26 is included in library362, not additive. No
failures/warnings are emitted in this frozen regression packet. Final resource
reading is 30,906,707,968 bytes free, with each command above the 10 GiB guard.
The scoped three-crate Clippy and workspace format packet is recorded below.
Workspace tests, complete probe/fault/order coverage, real provider use and
whole-I3-3 acceptance are not supplied by these component results.

Stage 2b initial API/compiler RED at `3862b168`:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-runtime
--features i3-private-quic,i3-process-test-seams --lib
i3_provider_composite_trusted_binding_yields_only_inactive_scoped_component
-- --test-threads=1` exits 101 with exactly E0432: unresolved
`crate::i3_read_only_provider_composite` in the new test module.
Log: `/tmp/c3-stage2b-composite-api-red-20260908.gyaEqh/01-cargo-test.log`.
No test body executes. Parent inspects the output and authorizes only a
fallible, non-authorizing fail-closed scaffold before the behavioral RED.
Actual setup/binding operations must return typed failure rather than panic;
absence of the declared target remains a valid binding layout. No fake
membership, verification, policy seal, M8 materialization or host-call audit
is supplied by the scaffold. Free disk is 11,821,633,536 bytes; no cleanup.

The same command then reaches actual behavioral RED in
`/tmp/c3-stage2b-composite-binding-behavior-red-20260908.2CbCfn/01-cargo-test.log`:
compilation succeeds, one test body runs and fails with
`PendingGenuineInactiveAdmission` at its genuine candidate expectation
(0 passed, 1 failed, 345 filtered). It has already prepared the full checked
source/static plan, exact composite discharge and real setup-owned binding.
Two nonfatal dead-code warning groups remain in the scaffold. No M9 authority,
provider invocation or successful component is returned. Parent inspects the
log and authorizes the full bounded Stage 2b implementation with separate
test ownership. Postflight free disk is 11,802,308,608 bytes; no cleanup.

The first two-policy falsifier attempt uses the same environment/package/
features with `--lib i3_provider_composite_rejects_policy_ -- --test-threads=1`.
It is compile-blocked, not behavioral evidence: E0308 at
`sys4_dispatch.rs:1285` exposes one remaining successor-snapshot caller after
ordinary M8 export became fallible. No test body runs. Log:
`/tmp/c3-stage2b-policy-behavior-red-20260908.v5Kz9w/01-cargo-test.log`.
Parent authorizes only typed error propagation through the existing SYS4/
SYS5 snapshot chain before rerunning; no panic, default or lost scope.
Postflight free disk is 11,792,039,936 bytes; no cleanup. The genuine planned
falsifiers are retirement after policy issuance before sealing, and a policy
for another checked operation within the same trusted setup. Neither is yet
claimed reproduced by this compiler failure.

After the compile-only propagation repair, the identical policy command runs
both bodies and reproduces both wrong successful seals (0 passed, 2 failed,
350 filtered; compile 31.23 seconds). The actual retired binding still returns
an inactive component, as does a distinct checked operation using another
operation's policy from the same setup. Parent reads
`/tmp/c3-stage2b-policy-behavior-rerun-20260908.pYY6zQ/01-cargo-test.log`
and authorizes current-binding and exact-policy repairs. Eleven dead-code
warning groups remain in the evolving implementation/test helpers; this is
not a lint pass. Free disk is 11,562,471,424 bytes; no cleanup. Scope binding,
exact four-row M8 exclusion and lower-issuer policy enforcement remain part of
the same component acceptance, not later waived requirements.

The scoped-snapshot consumer first produces compiler-only API RED (two E0599,
no test body) in
`/tmp/c3-stage2b-scoped-snapshot-api-red-20260908.mAkvXk/01-cargo-test.log`.
Parent then authorizes a fail-closed consumer scaffold and actual internal
lowering-inventory inspection, without component acceptance. The complete
module filter `--lib i3_read_only_provider_composite_tests` subsequently
compiles in 33.64 seconds and runs 10 tests: 7 pass, 3 fail, 345 filtered.
Log: `/tmp/c3-stage2b-composite-module-behavior-20260908.HnuhNJ/01-cargo-test.log`.
Both policy regressions now pass. Remaining body REDs show a second seal
accepted in the same setup, provider ordinals 15--18 still present instead of
exact exclusion, and matching-context consumption rejected by the scaffold.
Parent reads the log and authorizes those three bounded repairs together,
including binding preservation inside the M8 scope and a precise private
`AdmissionAlreadyUsed` diagnostic. Six dead-code warning groups remain;
this is not component close or a quality-gate pass. Free disk is
11,559,141,376 bytes, without cleanup. Inventory inspection is T0-internal,
not an unconditional observer authorization claim.

Stage 2b regression observation, `2026-09-08T17:13:31+09:00`: the three
intermediate failures above have been repaired. The dedicated module runs
14 passed, 0 failed, 345 filtered in
`/tmp/c3-stage2b-composite-module-final-20260908.n50zoS/01-cargo-test.log`.
After removal of two unused implementation remnants, the full feature-union
runtime library runs 359 passed, 0 failed, 0 filtered (87.60 seconds) in
`/tmp/c3-stage2b-runtime-full-regression-20260908.qeqIRk/01-cargo-test.log`.
The same package/features with `--test sys5_i3_process_runtime` runs
63 passed, 0 failed (55.72 seconds) in
`/tmp/c3-stage2b-process-integration-regression-20260908.hHm28Q/01-cargo-test.log`.
All use locked dependencies, incremental disabled, two build jobs and serial
tests. The 14 are included in 359, not additive. Parent reads both broad logs.
Normal-library compilation for process integration emits 34 dead-code warnings;
this is functional regression evidence, not a Clippy/quality pass. Independent
Stage 2b specification review is pending; neither this component nor row 18 nor
I3-3 is accepted. There is still no provider invocation/network availability.

The process command began above the 10 GiB resource floor and finished below
it: 10,361,020,416 bytes available versus 10,737,418,240 required. The evaluator
therefore does not start the public guard target or further Cargo commands.
No old build artifacts are deleted without new owner approval; the earlier
incremental-directory cleanup permission is not broadened. Read-only review
and lightweight documentation validation may continue. Required quality and
remaining integration gates are not waived or recorded as passing.

After the later explicitly approved five-file cleanup, the review-negative
filter runs with the same locked feature-union environment and
`--lib i3_provider_ -- --test-threads=1`: actual 26 tests, 25 passed,
1 failed, 336 filtered. Log:
`/tmp/c3-stage2b-time-network-provider-filter-red-20260908.HnIIfA/01-cargo-test.log`.
Both new existing-boundary guards pass (cross-checked bootstrap at actual M9
verification and provider-scoped base at ordinary M9 runtime admission).
The sole behavioral RED is direct generic M8 materialization returning a bare
instance whose `is_runtime_admitted()` is true before composite sealing.
Parent reads the log and authorizes typed rejection in that generic materializer,
minimal fallible private M9/M10 call propagation, and migration of the same
negative assertion to the exact existing M8 error. This is observed wrong
success, not a compiler-only API absence or an ordinary snapshot counterexample.
The internal constructor test uses a fixed test nonce, not claimed as actual
trusted-fixture provisioning evidence. Postflight free space is 12,256,542,720
bytes. No additional cleanup or acceptance claim follows.

The first post-repair provider filter is compiler RED, not test evidence:
`/tmp/c3-stage2b-provider-green-clippy-20260908.16SCVC/01-provider-filter.log`
exits 101 before any body. Four errors identify three remaining M9 callers
that used `map` with the now-fallible private execution seam, leaving nested
Results (two existing String diagnostic boundaries and finite-local relation
setup). Parent reads the exact E0308/E0599 output and authorizes only missing
flattening/error propagation in those existing callers. No tests or semantic
guarantees are weakened. Clippy is not run after this failed prerequisite.
Postflight free space is 12,249,833,472 bytes.

After the three missing Result flattenings, the same provider filter passes
26/26 (336 filtered, 0.59 seconds), including the exact typed generic-M8
rejection, in
`/tmp/c3-stage2b-provider-green-clippy-rerun-20260908.DMte8T/01-provider-filter.log`.
The subsequent three-crate feature-union all-target Clippy with `-D warnings`
fails; `02-clippy.log` records 34 dead-code diagnostics from the still-inactive
composite/M8 extension, two collapsible conditionals and two oversized enum
variants. Parent reads the full output and authorizes only bounded lint repair:
preserve all predicates and exact scope equality, indirect large private
coverage payloads, and narrowly document unconsumed next-component roots.
No activation or blanket existing-module warning waiver is introduced. The
lib-test four-error summary overlaps lint output; it is not behavioral evidence.
Available space is now 31,054,176,256 bytes following concurrent owner
housekeeping; agents remove nothing beyond the five explicitly approved files.

The next Clippy run reaches only seven unfulfilled M8 dead-code expectations:
`/tmp/c3-stage2b-clippy-quality-rerun-20260908.FmD8fI/01-clippy.log`.
The new inactive-composite module root already makes those referenced M8
extensions live for lint analysis. Parent reads the output and authorizes
removal of exactly the redundant expectations, not disabling the unfulfilled
expectation lint. The private-payload size and conditional warnings are gone;
no test bodies or subsequent format gate run after this failed prerequisite.

The frozen final quality packet succeeds in
`/tmp/c3-stage2b-quality-final-20260908.uI6Zpn/`:

- `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo clippy --locked -p mir-ast
  -p mir-semantics -p mir-runtime --features
  mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams --all-targets
  -- -D warnings`: exit 0, 21.62 seconds, `01-clippy.log`. These are the
  actual three packages, not CLI/probe or a workspace-Clippy claim.
- The same runtime provider filter passes 26/26, 336 filtered, 0.57 seconds
  in `02-provider-filter.log` after the final private-layout/lint edits.
- `cargo fmt --all -- --check`: exit 0, `03-fmt-check.log`.

Postflight space is 31,544,897,536 bytes. No warnings are reported in final
Clippy output. Full frozen library/process/I2/M10/guard regression and
workspace all-target compilation follow as separate gates, not assumed passes.

Final frozen Stage 2a packet, `2026-09-08T15:23:52+09:00`, supersedes the
earlier static RED/intermediate GREEN checkpoints below for the current delta
over `9e8d674a`. All commands use `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`,
locked dependencies and serial tests; one evaluator runs Cargo.

| Executed gate | Actual result | Exact log |
| --- | --- | --- |
| `cargo test --locked -p mir-semantics --test i3_provider_composite --test i3_provider_effect --test m9_finite_refinement -- --test-threads=1` | 6/6, 17/17, 8/8 separately | `/tmp/c3-stage2a-final-semantics-targets-20260908.JmMezY/01-cargo-test.log` |
| `cargo test --locked -p mir-semantics --doc M9CompositeFiniteRefinementDischarge` | selected 1/1; 2 filtered | `/tmp/c3-stage2a-final-semantics-doctest-20260908.2oTpSU/01-cargo-test.log` |
| `cargo test --locked -p mir-runtime --features i3-private-quic,i3-process-test-seams --lib sys3_projection::provider_effect_tests -- --test-threads=1` | 11/11; 334 filtered | `/tmp/c3-stage2a-final-runtime-staticmodule-20260908.dzny2B/01-cargo-test.log` |
| Same runtime command without module filter | 345/345; 83.64 seconds | `/tmp/c3-stage2a-final-runtime-featureunion-full-20260908.V2cVMH/01-cargo-test.log` |

The static 11 are included in the full 345, not additive. Source identity
tests also distinguish genuine provider source from genuine I2 source whose
ordinary operation is named exactly `ReadOnlyProviderEffect`. The full runtime
run includes the unchanged production-projection dependency scanner after
test relocation. Free disk is 11,855,839,232 bytes; no cleanup or alternate
feature build occurs. Parent inspects the evidence. No effect grant, actual
binding, host read, provider network execution or I3-3 acceptance follows.
Independent spec review is clear; separate final quality/regression/Git gates
remain in progress at this checkpoint.

Stage 2a integration regression then passes process integration 63/63
(`/tmp/c3-stage2a-process-integration-20260908.rJ7XlC/01-cargo-test.log`),
M10 67/67 plus I2 local 5/5 and CLI 8/8
(`/tmp/c3-stage2a-i2-m10-regression-20260908.Ddz3QT/01-cargo-test.log`), and
public provider guards 5/5
(`/tmp/c3-stage2a-runtime-public-guards-20260908.ARycgG/01-cargo-test.log`).
These use `cargo test --locked -p mir-runtime` with the same feature union,
respectively targets `sys5_i3_process_runtime`, `m10_conformance` /
`sys5_local_slice` / `sys6_i2_cli`, and `i3_provider_effect_guards`, with
serial tests. Unlike the cfg-test library packet, ordinary library compilation
emits ten dead-code warnings for the intentionally inactive static snapshot.

`cargo clippy --locked -p mir-semantics -p mir-runtime -p mirrorea-cli
--features mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams
--all-targets -- -D warnings` then exits 101 in
`/tmp/c3-stage2a-threecrate-clippy-20260908.CmrcWi/01-cargo-clippy.log`.
It reports those ten unused static snapshot items and six needless explicit
lifetimes in the relocated test helpers. The evaluator stops: workspace fmt
and all-target check have not run in this packet. Parent inspects the log,
the existing static-module lint pattern and actual absent production consumer;
authorizes only targeted non-test dead-code annotations with a pending-consumer
reason and the exact six lifetime elisions. No file-wide warning waiver,
semantic activation or schema change is authorized. Free disk is
11,846,680,576 bytes. Final lint and affected-test reruns remain required.

The first annotation rerun also exits 101:
`/tmp/c3-stage2a-threecrate-clippy-rerun-20260908.uPqjjv/01-cargo-clippy.log`.
Original dead-code and lifetime diagnostics are absent, but nine transitive
snapshot helper annotations report unfulfilled lint expectations. The root
static-projection snapshot-method impl is the one fulfilled expectation;
marking that root makes its referenced helper items live for lint analysis.
Parent selects removal of the nine redundant expectations, retaining only the
non-test root expectation with its exact inactive-consumer reason. No general
unfulfilled-lint waiver or executable use is added. Fmt, workspace check and
the post-lifetime static rerun are still unexecuted at this intermediate RED.

Final quality packet at `2026-09-08T15:35:01+09:00` supersedes both lint REDs:

- Same three-crate all-target Clippy command: exit 0, 27.36 seconds,
  `/tmp/c3-stage2a-threecrate-clippy-final-20260908.6bDyl6/01-cargo-clippy.log`.
- `cargo fmt --all -- --check`: exit 0,
  `/tmp/c3-stage2a-fmt-final-20260908.Fz8R9x/01-cargo-fmt.log`.
- `cargo check --locked --workspace --all-targets --features mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams`:
  exit 0, 26.92 seconds,
  `/tmp/c3-stage2a-workspace-alltargets-check-20260908.2aX6sG/01-cargo-check.log`.
  This is all-workspace compilation, not all-workspace test execution.
- Same runtime static-module command: fresh 11/11, 334 filtered,
  `/tmp/c3-stage2a-runtime-static11-final-20260908.uyFLiA/01-cargo-test.log`.

Parent inspects final outputs; no warnings remain in these runs. Final free
disk is 11,840,618,496 bytes. The only delta after the full 345/process 63/
I2-M10 behavior gates is the exact annotation/lifetime repair; those full
behavior suites are not relabeled as rerun after it. `make docs` also passes
agent configs, 216-entry Canon index, all 800 hierarchy paths and the scaffold
containing 1760 reports. Final LAB synchronization/documentation rerun and
scoped Git integration follow; no effect activation or I3-3 close is claimed.

Full static module at the typed-marker repair executes 11 tests: 10 pass and
the new ViewerC restriction test fails in
`/tmp/c3-stage2a-static-projection-module-red-20260908.EvAf9C/01-cargo-test.log`.
Before the failure it verifies equality with the original full checked
provider identity and absence of local provider fragments/edges. It then
observes successful ordinary snapshot export and panics; the subsequent SYS4
assertion is not reached in this RED. Parent inspected both log and test order
and authorizes the source-owned identity profile query. The preceding ten
tests include the eleven typed-marker mutation categories, direct carrier
adapter/fingerprint refusal and genuine provider static SYS4 refusal. These
are separate cases within the reported test bodies, not additive test counts.
No provider invocation is performed. Free disk is 11,859,234,816 bytes.

The two ordinary legacy snapshot mutations reach assertion RED (0/2,
340 filtered, exit 101) in
`/tmp/c3-stage2a-ordinary-snapshot-red-20260908.3A0UKC/01-cargo-test.log`.
Their unchanged legacy snapshots restore positively before the mutation;
the requested typed unsupported rejection is absent for a provider invocation
effect row and a nested provider fragment identity. The log records failed
rejection assertions; parent additionally traces the remaining budget/path
checks and ordinary preflight before authorizing the bounded typed-marker fix.
The relocated cfg-test module preserves the original architecture scanner:
its regression passes 1/1 in
`/tmp/c3-stage2a-projection-architecture-regression-20260908.hxPzW6/01-cargo-test.log`.
Both commands use the same feature union, locked dependencies and serial
tests. Free disk after the second command is 11,872,235,520 bytes.

After the bounded compile repairs, the final-cut runtime static target passes
6/6 in `/tmp/c3-stage2a-runtime-static6-finalcut-20260908.95zTPv/01-cargo-test.log`.
The full feature-union library command then executes 340 tests, with 339 passed
and one failed in
`/tmp/c3-stage2a-runtime-featureunion-full-20260908.0MVbcZ/01-cargo-test.log`.
The failed architectural scan finds `serde` in the new cfg-test file under the
production projection directory. Parent preserves that scanner and relocates
the test module outside the production directory with an explicit cfg-test
path, rather than excluding a new production subtree. This is a full-suite
RED, not a pass. A later focused rerun must bind the final relocated source.

Stage 2a source review repair GREEN (2026-09-08):
`/tmp/c3-stage2a-semantics-greenpacket-20260908.aXQXGs/01-cargo-test.log`
records composite 6/6, provider source 17/17 and legacy finite refinement 8/8
as separate targets. The composite getter-removal doctest, moved to the
composite struct, passes 1/1 in
`/tmp/c3-stage2a-composite-doctest-green-20260908.GgrkuE/01-cargo-test.log`.
Parent inspected both outputs. Source A cannot verify source B after either
candidate transformation; the ordinary discharge cannot be extracted.
The opaque API no longer permits directly replacing the source-contract half
of a composite candidate; its earlier constructible mutation is replaced by
the genuine cross-source identity falsifier, not by a raw test-only constructor.

The next runtime static test command does not yet pass. Its first integration
attempt exits 101 before any test body in
`/tmp/c3-stage2a-runtime-static-projection-green-20260908.Ij3sDk/01-cargo-test.log`
(directory name is only an intended gate, not its result). Three test-source
borrow/type errors and seven missing exhaustive match arms are captured.
Repairs are scoped to test borrows, preserved existing relation-consumer
handling and explicit new-provider metadata labels/refusals in SYS4/SYS5/SYS6.
No conformance acceptance or provider execution is added to repair compilation.
Postflight free disk is 11,880,099,840 bytes; no cleanup was performed.

Stage 2a initial interface/compiler RED at pushed `9e8d674a` is captured in
`/tmp/c3-stage2a-provider-composite-red-20260908.JlkxvC/01-cargo-test.log`:
`cargo test --locked -p mir-semantics --test i3_provider_composite --
--test-threads=1` exits 101 with E0432 for the two newly required coverage/
composite-checker symbols. No test body executes; this is not behavioral RED.
Parent inspects the output and authorizes a minimal fail-closed interface
scaffold only, followed by actual assertion RED before behavior completion.
Free disk remains 11.14 GiB; no cleanup or parallel Cargo is performed.

Stage 2a behavioral RED follows in
`/tmp/c3-stage2a-provider-composite-behavior-red-20260908.pXUxn1/01-cargo-test.log`:
the same command compiles, executes exactly one test and fails at the composite
verdict expectation with typed `ReadOnlyProviderEffectRequiresDedicatedRuntime`.
The scaffold derives real static coverage, but no verification success,
authority or execution follows. Parent inspects the failure and authorizes the
shared full-source finite checks plus exact selected-profile retention. Final
tests will separately assert static coverage so their checks do not remain
behind the intentionally failing composite expectation. Free disk is 11.09 GiB.

Stage 2a semantics is GREEN in
`/tmp/c3-stage2a-semantics-composite-stage1-20260908.M5eGjx/01-cargo-test.log`:
composite target 5/5 and retained provider source target 17/17. Actual
source-derived candidate negatives cover source mismatch, baseline failure/
capability/observation/effect constraints, each provider effect omission and
missing/changed exact source-label row. Existing finite-refinement regression
is 8/8 in
`/tmp/c3-stage2a-m9-finite-refinement-regression-20260908.70ICZH/01-cargo-test.log`.
Parent inspects both outputs. These are finite runtime test results, not a new
Lean proof, permission, invocation or full Stage 2 integration. Static SYS3
lowering and its actual-plan/snapshot falsifiers remain the direct consumer.

Runtime static-projection behavioral RED is captured in
`/tmp/c3-stage2a-runtime-static-projection-red-20260908.oyavwJ/01-cargo-test.log`:
feature-union library command with the exact new test filter compiles in
36.66 seconds, executes one test and fails at the dedicated projection with
typed `UnsupportedReadOnlyProviderEffectProfile` (334 existing tests filtered).
One non-fatal unused projection accessor warning belongs to this scaffold,
not a passing warnings-denied gate. The success type has no construction path;
its temporary unreachable borrowed-view panic must be removed before positive
implementation acceptance. Parent verifies the log and authorizes actual
static role/edge lowering plus exact snapshot validation; a separate writer
owns only downstream explicit non-execution guards. No provider authority,
handler activation, actual host I/O or network dispatch follows from this RED.

Provider Stage 1 final integration, over `985ee179` (2026-09-08):

Initial docs-only gate in
`/tmp/c3-provider-stage1-docs-final-20260908.MiX9U0/` passes agent configs,
Canon index 216 and hierarchy 800/800, then fails the literal Canon-notice
lint in rewritten tasks.md. Parent checks the validator's required phrases
and restores backticks/lowercase wording; no semantics or validator is changed.
Diff check passes and bounded credential-pattern scan has zero matching files
across 30 dirty paths, including exactly six expected new source/test/sample
files and no generated artifacts. Final docs rerun is tracked below.

The next docs rerun in
`/tmp/c3-provider-stage1-docs-rerun-20260908.Mn5TnO/` clears the notice lint but
finds project-status at 190 lines against its 180-line concise-view budget.
Parent replaces old repeated checkpoint counts with a short Report 2606
history pointer and corrects stale current disk wording. No validator or
acceptance limit is weakened; diff and 30-path credential scan still pass.
Parent also restores exact Canon/Plan paths in tasks.md's current-position
section; the validator's focused `snapshot_position_source_errors()` returns
an empty mapping before the final full rerun completes.

Final docs rerun is GREEN in
`/tmp/c3-provider-stage1-docs-final-rerun-20260908.gl8MRa/`: agent configuration,
Canon index 216, hierarchy 800/800 and documentation scaffold 1760 reports
pass. Diff check passes; bounded 30-file credential-pattern scan has zero
matching filenames, with six expected new files and no generated artifacts.
Free disk is 11.14 GiB, available memory 11 GiB. Parent inspected the actual
logs; source/tests remain the same frozen reviewed delta.

- `/tmp/c3-provider-stage1-runtime-regression-20260908.N53EQJ/`:
  feature-union runtime library 334/334, process integration 63/63, M8 admission
  7/7, M9 external boundary 1/1, SYS5 local slice 5/5, I2 CLI 8/8 and M10 67/67.
  These are separately scoped targets, not an additive test total.
- `/tmp/c3-provider-stage1-quality-20260908.ad6Slv/`:
  AST/semantics/runtime all-target Clippy with warnings denied and runtime
  `i3-private-quic,i3-process-test-seams` features passes; workspace format passes.
- `/tmp/c3-provider-stage1-workspace-check-20260908.5Ya3m6/`:
  `cargo check --locked --workspace --all-targets --features
  mir-runtime/i3-private-quic,mir-runtime/i3-process-test-seams` passes across
  all 14 members. This is compilation, not a full workspace test run.
- Final free space is 11.15 GiB; incremental builds remain disabled, jobs 2,
  locked dependencies and serial tests. No cleanup or external workdir fallback
  was performed. Full I3-3 validation is still required after actual effects and
  remaining fault/cut/order work; no provider execution is inferred here.

Provider Stage 1 intermediate validation (uncommitted source delta over
`985ee179`): `cargo check --locked -p mir-semantics --lib` passes without
warnings, log `/tmp/c3-provider-stage1-structural-20260908.pjeM9T/01-mir-semantics-lib-check.log`.
The complete new semantics target then executes 16 bodies: 13 pass and three
test expectations fail, log
`/tmp/c3-provider-stage1-tests-20260908.vMpdjK/01-semantics-provider-effect.log`.
The parent checked the actual identity wrapper and M9 normalized failure row,
and the test author verified the old generated-row order against `985ee179`.
Only those expectations were corrected; neither duplicate identity entries nor
an unrelated production failure-order change was introduced to satisfy tests.
The runtime guard target is separately **compile-blocked**, with 14 exhaustive
match errors and no executed test body, log
`/tmp/c3-provider-stage1-runtime-guard-20260908.d7QVCg/01-runtime-provider-guards.log`.
It authorizes the already planned explicit unsupported-profile guards and
lossless metadata mirrors, not provider execution or a passing runtime claim.
Both runs used `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, `--locked` and
serial test execution; final free disk after them was 12.48 GiB.

The semantics-only rerun passes all 16 provider tests without warnings:
`/tmp/c3-provider-stage1-semantics-rerun-20260908.LQ3ugU/01-semantics-provider-effect.log`.
The focused unchanged-source regression packet also passes: AST parser 17/17
and M6 10/10 (`/tmp/c3-provider-legacy-regression-20260908.n4EgtG/01-ast-m6.log`),
and owner-admission budget 14/14, M6 classification 13/13 and M7 pipeline 27/27
(`04-semantics-m6-m7-time.log` in the same directory). Commands select these
five named targets in `mir-ast`/`mir-semantics`; no runtime or workspace test
is included in these counts. Parent read the executed logs/results. Runtime
unsupported-profile implementation and its tests remain the next evidence gate.

Stage 1 independent review found three bounded construction/restore defects,
not failures of the selected provider contract. First, private provider Core
restoration accepted source-inexpressible identifiers; a real test body failed
in `/tmp/c3-provider-source-identifier-red-20260908.WgAXFD/01-source-identifier.log`.
The private constructor now checks the complete Surface lexer identifier form,
and the rerun passes provider semantics 17/17 plus public runtime guards 5/5
(`/tmp/c3-provider-stage1-p1-rerun-20260908.UHDJSy/01-semantics-provider-effect.log`
and `04-runtime-provider-guards.log`). The public M10 test exercises its existing
String rejection channel before execution; no new M10 diagnostic/profile is
introduced. Second, the public M6-template constructor bypassed M7; the external
compile-fail doctest actually compiled, producing RED in
`/tmp/c3-provider-privacy-snapshot-red-20260908.aQaVqq/01-provider-core-doctest.log`.
It is now crate-private, with the sole checked pipeline caller preserved.
Third, new provider lowering tags could be restored into a legacy M8 instance
without a provider Core/handler. The genuine-owner-snapshot tamper test reached
that defect in
`/tmp/c3-provider-privacy-snapshot-runtime-red-20260908.UV5RoH/01-private-snapshot-provider-kind.log`.
An early four-tag guard now rejects before identity/admission/plan restoration.
The normal enclosing child-image byte binding is a separate safeguard; this
is a component integrity repair, not evidence of a network cryptographic bypass.
Final green reruns and independent sign-off remain required for these last
two repairs; no Stage 1 or I3-3 acceptance is inferred from the edits alone.

Final P1 rerun packet is green in
`/tmp/c3-provider-p1-green-20260908.hPcdw3/`: privacy doctest 1/1
(`01-doctest.log`), source suite 17/17 (`04-semantics-provider-effect.log`),
private runtime guards 2/2 (`07-runtime-provider-effect-lib.log`), and public
guards 5/5 (`10-runtime-provider-guards.log`). Source/spec and runtime quality
review independently report no remaining P0/P1/P2. The complete feature-union
runtime library then passes 334/334 in
`/tmp/c3-provider-stage1-runtime-regression-20260908.N53EQJ/01-runtime-feature-union-lib.log`.
These nested/overlapping filters are not additive test totals. This supports
the checked-source/unsupported-handoff component only; remaining integration,
lint/docs gates and component commit/push are tracked separately.

Provider Stage 1 behavioral RED, before production edits:
`CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 cargo test --locked -p mir-semantics
--test i3_provider_effect ordinary_provider_source_is_checked --
--test-threads=1` executes one test body and fails its checked-source assertion
with `UnexpectedSyntax` at the new source's `effect` (line 57, columns 1--7).
This is the missing parser feature, not an undefined API/compile failure.
Log `/tmp/c3-provider-effect-initial-red-20260908.BgbxVG/01-provider-effect.log`
was read by the parent. Exit 101, wall 0.25s, RSS 228944 KiB; post-run free disk
12.19 GiB. Parent then authorized AST/M6/M7/snapshot/M9-contract production
work and separately assigned typed tests, without enabling provider execution.

Provider contract docs-only gate passes, logs
`/tmp/c3-provider-contract-validation-20260908.GDxYjC/`: `01-make-docs.log`
records agent-config validation, Canon index 216, hierarchy 800/800 and complete
scaffold with 1760 numbered reports; `02-git-diff-check.log` is clean;
`03-scanned-paths.txt`/`04-credential-pattern-matches.txt`/`05-summary.txt`
record a bounded 17-dirty-text-path scan with zero filename matches. No secret
contents or entropy scan were used. Final free disk is 12.23 GiB. Parent read
the summary and actual docs log. No provider implementation/runtime test is
claimed. This adopted contract is ready for its scoped commit/push before the
behavioral source RED gate; exact cut/parity is recorded by the next component.

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

Stage C begins from the source-based SYS5 default-guard control. Its expected
behavior now deliberately advances from unsupported rejection to `Ok(None)`
staging, retaining every zero-effect/requester-pending and unannotated control.
The first run fails to compile because `assert_eq!(opaque Option, None)`
requires an unavailable message `PartialEq`; it is not behavioral evidence.
The test-only `.is_none()` correction exposes the actual intended RED:
0/1, 62 filtered, 0.02 s execution, `CarrierAdmissionRejected` at the staging
expectation. Only then does the parent authorize the bounded clock/gate/permit
implementation. Eight existing dead-code warnings remain; root has 14 GiB
free. No passing staging, permit, expiry or actual-network result is claimed.

Initial Stage C results (`2026-09-07T21:56+09:00`): after compile-only repairs
to the condition import/collection type and existing post-dequeue test seam,
the staging target passes 1/1 (0.04 s). The feature-enabled library filter
`i3_owner_admission_budget_` passes 15/15 (22.55 s build, 0.16 s tests), and
`i3_reserved_owner_admission_refuses_before_insertion_when_a_genuine_prior_owner_request_is_queued`
passes 1/1. These use the same locked two-job, incremental-disabled Cargo
environment and `--test-threads=1`. The library commands add
`--lib --features i3-process-test-seams`; no actual network execution follows
from these results. Root remains at 14 GiB free.

The integration target emits 16 dead-code warnings while the later private
consumer is not connected. The library emits two warnings: three existing
peer-preface helper items, and one ignored test-hook `Result`. The test owner
now checks that hook result; this test-only change has not yet been rerun.
Two tool launches returned no output/session despite an active Cargo process;
the evaluator waited for its exit and confirmed no remaining Cargo process
before obtaining a fresh captured result. No status was inferred for a lost
run. Parent `make docs` also passes, including the 213-entry Canon index,
800 required hierarchy paths and 1760-report scaffold.

This is not C1/C2 acceptance. Review found a separate staged-G1/current-G2
issuance mismatch when exact current owner authority remains valid. Parent and
read-only planner select the existing spec/16 rule: compare every immutable
staged binding field exactly, derive permit generation from freshly revalidated
current authority, and require full generation equality at later handoff.
Ordinary carrier admission and requester outcome checks are unchanged.
The paired genuine-M9 falsifier and narrow repair are next; capacity,
independent progress, multiple owner clocks and final independent review also
remain. Provider, typed expiry transport and the full I3-3 matrix remain open.

Stage C review repair checkpoint (`2026-09-07T22:28:09+09:00` observation):
the parent-authorized frozen pre-repair budget filter records 16 passes and
4 failures (289 filtered, 0.94 s). Three failures expose retained-state
defects: substituted genuine carrier consumes `Awaiting`, late selected-owner
revocation overwrites the winning resolution tick, and successful handoff
remains `ServeReserved`. The fourth is a G2 fixture failure: the source
requires a caller argument, whereas the SYS5 emission API supplies none.
It is not evidence of a remaining generation defect. The separate genuine
ordinary SYS4 absent-target test fails at the contextual trace assumption in
`m8_runtime_local_cut.rs`, completing executable reproduction of the four
independent P1 findings. Logs are local disposable evidence at
`/tmp/i3-p1-budget-8Ipxtg.log` and `/tmp/i3-p1-absent-target-IUzuPg.log`.

The production owner has repaired wrong-binding nonmutation, separated later
handoff revalidation from original decision provenance, made trace-less enqueue
rejection typed/unobserved, and shared normal/gated record-derived finalization.
The test owner has replaced the G2 fixture with genuine no-argument A/S/T
source operations and a G1 control; the unused V locus is absent from this
finite fixture, not silently authorized by a production membership change.
Additional tests cover retained 64-record capacity, independent eligible
progress, separate owner clocks and dropped reservation non-reissuance.
Both writers report static checks and freeze; joint execution and narrow
independent re-review are now in progress. No repaired-green claim, C1/C2
acceptance, typed transported expiry or I3-3 close follows yet.

One intermediate post-generation-repair run was triggered by a producer's
direct evaluator message while the test owner was still editing. Its captured
G2 handoff rejection is diagnostic only, not frozen-cut evidence. The parent
has restored a single scheduling rule: only root authorizes Cargo after both
production and test owners freeze. The 16/4 and separate absent-target results
above were captured under that corrected rule. No concurrent Cargo build or
unrecorded inferred success is used as evidence.

The next jointly frozen C1/C2 run records 13/14 in the complete
`sys5_i3_owner_admission_tests` module (300 filtered, 23.51 s compile,
1.32 s tests; one existing peer-preface dead-code warning). The G1 no-argument
control and binding/provenance/success/capacity/progress/multiple-clock/drop
cases pass. The corrected genuine G2 distinct-owner case still rejects at
handoff with `CarrierAdmissionRejected`; this is now an actual remaining
boundary failure, no longer the missing-argument fixture failure. The sole
evaluator stopped as instructed, so absent-target, queued-owner and staging
filters were not rerun. Source owner diagnoses the exact handoff boundary;
test owner independently checks the current-authority fixture. Log:
`/tmp/i3-c12-owner-tests-XNZRsz.log`. Root remains at 14 GiB free. No aggregate
C1/C2 acceptance is claimed from the passing subset.

Independent reviewer, test owner and parent traced the remaining failure to
the final ordinary SYS4 lineage equality, not permit issuance. M9's opaque
`owner_lineage_ref` incorporates the global generation reference, so genuine
G2 changes that reference even when the held operation's exact underlying
membership/capability/witness remain valid. The selected repair is confined to
the exact current-generation verified handoff: it must preserve the sealed
staged envelope while using current M9 authority for M8. The ordinary carrier
path and requester historical-outcome checks keep their exact current-lineage
rule. This is the existing spec/16 current-authority contract, not general
historical-generation reuse or a new migration policy.

The repaired jointly frozen run is GREEN (`2026-09-07T22:39:42+09:00`
observation): complete owner-admission module 14/14 (24.20 s compile,
1.35 s tests), absent-target 1/1 and queued-owner 1/1 (each 0.02 s cached
compile, 0.00 s tests), and SYS5 integration staging 1/1 (24.87 s compile,
0.04 s tests). All use `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2`, locked
`mir-runtime`, `--features i3-process-test-seams`, and one test thread;
the first three use `--lib`, the last `--test sys5_i3_process_runtime`.
Each library filter retains one existing peer-preface dead-code warning;
integration retains 16 unused-path warnings pending the C3 consumer. Root
remains at 14 GiB free, RAM 10 GiB available. Logs:
`/tmp/i3-c12-rerun-owner-tests-onEZEc.log`,
`/tmp/i3-c12-absent-target-K0uCXB.log`,
`/tmp/i3-c12-queued-owner-0IeIJf.log`, and
`/tmp/i3-c12-sys5-staging-EPkfpL.log`.
Parent `make docs` also passes (213 Canon entries, 800 hierarchy paths,
1760 reports). This closes the bounded C1/C2 repair loop, not a milestone.
The uncommitted delta now has C3 typed reply/requester retention as its direct
consumer; no actual annotated QUIC, provider or whole-I3-3 acceptance follows.

C3 implementation boundary: replace silent expiry `None` with a private
resolution sum (one-use serve reservation or genuine declared-failure reply).
The gate consumes its retained issuance into an opaque expiry decision; only
that decision may construct the generated SYS4 reply. The success/failure sum
belongs inside the existing SYS4 owner-reply payload and private snapshot,
retaining the shared envelope/codec/admission path. A separate SYS5 failure
sidecar bypassing that path is not selected. Requester terminal retention is
distinct from a successful receipt and precedes pending deletion. The first
test must not assert pending removal before delivery: initially it falsifies
the absence of a deliverable expiry outcome; the completed positive must
actually encode, decode, admit and consume the gate-produced reply.
Requester consumption must also return a distinct local typed terminal
result, not silent `Ok(None)`: the existing QUIC caller interprets absent
local output as rejection and would otherwise return an invalid pending
handle after the runtime had deleted it. The receipt-only success predicate
must remain unchanged. This direct-consumer check prevents locally passing
terminal retention from breaking the next actual transport stage.

C3 availability behavioral RED is captured on the unchanged C1/C2 production
cut: exactly 1 selected test, 0/1, 314 filtered, 23.84 s compile and 0.05 s
execution. The filter is
`i3_owner_admission_budget_expiry_exposes_a_sendable_declared_outcome_before_requester_terminal_consumption`
under the same locked two-job, incremental-disabled library/test-seam command.
It fails because expiry returns `Ok(None)` rather than an available outcome;
this is not a compile failure or evidence of delivered/consumed failure.
Log: `/tmp/i3-c3-availability-red-RhjxiL.log`. One existing peer-preface warning
remains; root retains 14 GiB free and 10 GiB available RAM. Only after this
result does the parent authorize the adjusted C3 production packet and paired
test implementation. Actual QUIC remains the later direct consumer.

## What changed in understanding

Stage 2c scope uses the existing SYS4 artifact/incident-edge restriction, not
naive authority-locus equality. A requester image legitimately needs the target
owner's operation lineage; a designated remote-input closure is retained at
both incident ends. This preserves assigned execution without deleting required
remote authorization. Parent also catches and corrects an interim draft that
issued legacy capabilities before separate policy and lacked the actual
restricted translated inventory. These are implementation corrections within
the selected contract, not new Canon decisions or accepted test evidence.

One bounded temporary Oracle consult (`i3-provider-stage2c-join-20260908`)
reviews the next mixed-image join while Stage 2b repair proceeds. Direct
consumer: the next sealed SYS4/SYS5 composite/image component. Blocker reduced:
avoiding an ordinary inner admission that erases provider obligations or a
second execution stack. Acceptance use: exact full identity, independent
effect authority, four-row scope, incarnation and trusted child-start
association tests. Parent reads the complete answer at
`/tmp/i3-provider-stage2c-oracle-answer-20260908.md` and checks it against
spec/17 and mapped code. The advisory favors a private checked composite
variant through existing admission/image/initial-state seams; a delegating
wrapper is equivalent only if every usable route preserves the same boundary.
Parent retains that smallest join direction, not a parallel runtime/transport.
Actual supervisor, host invocation, effect lifecycle and network evidence
remain required downstream; an inactive image cannot discharge those duties.
The consult is completed with captured output; a subsequent browser-unreachable
message is not retried. Oracle supplies no Canon or implementation acceptance.

The provider static boundary must preserve both positive complete coverage
and negative whole-program requirements after local restriction. Outer enum
tags alone do not detect hidden typed provider fields, and local inventory
alone does not detect a requirement erased by restriction. The existing exact
checked identity plus a shared typed predicate supplies the bounded guard.
Static coverage, full finite-verification evidence and actual effect authority
remain separate; a raw legacy discharge or unscoped M8 component cannot stand
in for complete mixed-program admission. This narrows the next implementation
consumer without adding a general verifier or new trust theory.

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

Continue active I3-3 after the green inactive Stage 2c integration:
consume its actual retained composite/M9 evidence and restricted images in
Stage 3's actual provider runtime, bounded host call and distinct QUIC
request/result flow. Reuse existing SYS4/SYS5 and supervisor/FD3 mechanisms.
Ordinary entry points remain rejectors; no synthetic ordinary verdict,
scope erasure or second runtime/transport. This is an executable direct
consumer, not another inactive-only handoff.
ADR-0042/spec/17 is selected; invocation is not yet implemented. Preserve all
20 failure families and ordering requirements, including remaining actual
membership, redaction and in-flight cut boundaries. After full I3-3 acceptance,
validation, review, commit/push and remote
parity, honor the latest owner-requested pause. I3-4 requires explicit resume;
the retained program is not blocked, stale, or completed by that pause.

## Plan update status

Provider Stage 2c: Plan 250 records actual inactive restriction/codec and
parent-held validation, repaired falsifiers, final frozen runtime/regression
and review evidence. Actual Stage 3 provider execution is the next direct
consumer; the sole milestone and owner stop boundary remain unchanged.

Provider Stage 2b: Plan 250 records actual binding/verification/policy/scoped
M8 evidence, the fresh final regression/review packet and Stage 2c as next
consumer. This remains one I3-3 milestone, not a new program or lifecycle gate.

Provider Stage 2a: Plan 250 records the selected opaque composite path, exact
static coverage/identity repairs, test results and trusted binding -> actual
M9 policy/verification -> inactive scoped M8 consumer. No new goal is opened.

Provider Stage 1: completed static/guard/review checkboxes and exact evidence
are recorded in Plan 250. Next consumer is explicit composite coverage,
projection and independent provider permission; I3-4 stays inactive.

`plan/` 更新済み: Plan 250 now records the resume cut, complete active I3-3
Goal Statement and work/evidence/decision gates. The parent goal, all failure
rows, ordering edges and later milestone order are preserved. plan/00-index
synchronization belongs to the status assignment.

## Documentation.md update status

Provider Stage 2c: final inactive packet and actual Stage 3 consumer mirrored;
ordinary localnet regression is not labelled provider execution.

Provider Stage 2b: refreshed current 362/26/63/67/5/8/5 evidence, with filtered
tests explicitly non-additive and runtime activation still pending.

Provider Stage 2a: fresh static test evidence and the remaining actual
authorization/runtime consumer replace the prior source-only current pointer.

Provider Stage 1: current pointer now distinguishes verified source/guard
evidence from still-pending provider execution and milestone acceptance.

更新済み: resume pointer now identifies I3-3 as the sole active milestone,
with the accepted I3-2 runtime retained as its input.
Delivery checkpoint: `Documentation.md` 更新不要; its current entry pointers,
owner-pause boundary and provisional workflow remain accurate. No CLI or
sample-root change is introduced here.
Time slice: updated the reader pointer from the prior successor checkpoint to
the selected contract and validated static slice, explicitly retaining pending
generated-contract/runtime enforcement and the unchanged workflow boundary.

## docs/project-status.md update status

Provider Stage 2c: final frozen packet, zero remaining review findings and
the current component integration stage are synchronized within the 180-line bound.

Provider Stage 2b: synchronized current evidence/review, capacity and next
mixed-image boundary; six accepted milestones and lifecycle remain unchanged.

Provider Stage 2a: fresh versus retained results, current integration gates,
six-of-eleven milestone count and unchanged lifecycle/control are synchronized;
the reader snapshot remains 168 lines, within its 180-line bound.

Provider Stage 1: current evidence and next consumer synchronized without
changing official lifecycle or the stop-after-I3-3 instruction.

更新済み: owner resume、I3-3 sole active、後続 milestone inactive を同期した。
I3-2 accepted evidence と official I3 unentered を維持する。

## progress.md update status

Provider Stage 2c: current evidence and actual-timestamp recent log separate
inactive validation from still-unimplemented host execution. No percentage
increase or official lifecycle change is inferred from the test count.

Provider Stage 2b: current snapshot and actual-timestamp recent log updated;
retained Stage 1/2a results are distinguished from final Stage 2b reruns.

Provider Stage 2a: 15:23 JST snapshot and recent log record fresh static
verification, separate pending authority/runtime and unchanged three axes.

Provider Stage 1: dated 2026-09-08 13:42 JST snapshot/recent log replaces stale
current counts, separates three axes and compresses older validation details
into this report. No workload-weighted percentage or completion claim is added.

更新済み: current I3-3 pointer, preserved three axes and startability mirror
the resume; the actual-time recent log is 2026-09-07 11:38 JST.
The current delivery checkpoint also synchronizes Plan 250, progress/tasks,
project status and sample dashboard with fresh versus prior validation and
time-then-provider as the next still-OPEN contract consumer.
The subsequent time slice supersedes that pointer: ADR-0041/spec/16 is
selected, static tests/review pass, generated contracts/runtime gating remain
pending and provider alone remains an OPEN contract. The 19:56 JST snapshot
and recent log separate this evidence from whole I3-3 acceptance.
The 22:39 JST snapshot supersedes those implementation pointers: C1/C2 local
gate/permit checks and narrow review pass; C3 typed reply/requester retention
is active. All later network, provider and full-matrix requirements remain.

## tasks.md update status

Provider Stage 2c: current snapshot names actual provider execution as next
consumer and retains all remaining I3-3 gates and stop-after-I3-3 control.

Provider Stage 2b: current task map separates component integration, inactive
mixed images, actual provider execution and remaining failure/order gates.

Provider Stage 2a: current snapshot is revised around the fresh static packet,
remaining integration gates, binding/auth/policy/scoped-M8 sequence and all
remaining I3-3 obligations. No remaining-time countdown is manufactured.

Provider Stage 1: the whole current snapshot is rewritten at 13:38 JST around
the unchanged 11-milestone sequence, verified static guards, next composite
admission consumer and retained final gates. Historical estimates are labelled
as dated estimates, not a current countdown.

更新済み: active I3-3 and inactive later packages are separated from accepted
I3-2 history. The parent-requested current-goal/direct-consumer coherence
correction is applied: I3-4 consumes I3-3 and resume is resolved history.
The 22:42 JST maintenance records C1/C2 as an uncommitted delta over pushed
`050f5067`, separates fresh 14+1+1+1 from prior regression counts, and names C3
as the direct consumer. No new roadmap or queue is created.

## samples_progress.md update status

Provider Stage 2c: inactive-image evidence is updated; the provider source is
still not an operational network sample. Sample/script taxonomy is unchanged,
so samples/README and scripts/README updates are unnecessary in this component.

Provider Stage 2b: current evidence row updated; provider sample remains
non-operational, with no new sample root, public workflow or Lean registration.

Provider Stage 2a: new test evidence is classified static-only, not a runnable
provider network sample. No sample or script taxonomy change; samples/README
and scripts/README need no update for this component.

Provider Stage 1: adds an explicitly source-checking-only provider row and
command. samples/README and scripts/README mirror that taxonomy. The new source
is not registered in legacy M5/M10/SYS5 sample execution and is not claimed to
run over the network. No new script or active sample root is introduced.

更新済み: the sample dashboard records resumed I3-3 as active/not accepted,
with all 20 families explicitly a target. Accepted I3-2 runnable evidence
retains its existing class. No new workflow-ready or product-completion claim.
The time slice adds the 14-test static command as evidence only; its embedded
source strings are not a new active sample root or network workflow.
The C1/C2 update now points to its actual library test command and preserves
the no-new-network-sample/non-workflow classification.

## Reviewer findings and follow-up

Stage 2c final independent specification and quality reviews each report
P0=0/P1=0/P2=0 after the narrow repairs. Both first identify two P1 blockers:
the separately held exact image commitment was unused, and real retirement
during structural validation could produce inconsistent successful receipt
facts. The executed 11/13 RED and post-repair 13/13 GREEN establish both
falsifiers and repairs. Final validation compares the expected commitment and
checks retained M9 seal then monotone setup currentness before capturing the
historical receipt facts. This does not assert continuing or cross-process
authority. P2 follow-up also adds decoded generic-start refusal, distinguishes
retained M8-row mutation from actual excluded-provider-row removal, and removes
the obsolete no-cohort scaffold comment. Both reviewers inspect the full
Stage 2c diff plus these narrow changes without running Cargo. Their clearance
is conditional on the remaining frozen-cut regression/LAB/Git gates and is
strictly inactive-component-only, not provider activation or I3-3 acceptance.

Stage 2c pre-edit/sequence planner review gives GO for the bounded test-first
packet: exact all-locus A/B inactive images and separately trusted validation
are a sufficient direct consumer without starting endpoints or providers.
It confirms no synthetic ordinary verification, no decode-time issuance,
ordinary/composite distinction through restriction/restore and explicit
parent-local versus later cross-process liveness claims. Parent marks the
older Stage 2a ownership paragraph historical. This is a plan review, not a
Stage 2c implementation or milestone acceptance review.

Independent Canon-first planner pre-close review gives GO for inactive-only
Stage 2b component integration after the executed gates above. Parent accepts
that bounded source contract subject to LAB validation and actual commit/push
parity. The next consumer is a private inactive mixed-image variant through
existing SYS4/SYS5, not a second runtime, launcher or transport. Stage 2b
consumes the intact composite discharge but is not already a complete
proof-retaining serialized image: Stage 2c must retain the actual evidence and
needed legacy M9 facts at sealing and preserve them through restriction,
restore and trusted start. No recomputed pass flag or provider-blind ordinary
seed may substitute. Six/eleven accepted milestones and the I3-3 stop boundary
are unchanged.

Final constructor-repair follow-ups supersede the earlier pending findings:
independent specification and quality reviewers each report P0=0, P1=0,
P2=0. Both inspect the direct generic M8 refusal, ordinary M9 guard, fallible
private M9/M10 propagation, three direct negatives, private scope boxing and
unchanged exact equality. The inactive module-root expectation is narrow;
redundant M8 expectations are removed. Readiness is conditional on the frozen
regression/quality gates below and limited to the inactive Stage 2b component.
It is not provider activation, row-18 or I3-3 acceptance.

Final Stage 2b independent quality disposition, observed
`2026-09-08T17:26:31+09:00`: P0=0, P1=0. The generic M8 constructor gap
below is P2 but explicitly blocks component integration until direct refusal
and its falsifier pass. The zero-call evidence clarification is P2 wording
only and is incorporated below. No further bounded correctness/concurrency/
authority or source-identity defect is found. Specification and quality reviews
are complete, but their repair/test closure and Rust quality gates are not.
Parent preserves the dirty work and requests the scoped cache-cleanup approval
needed to resume safe build validation; no milestone acceptance or goal-status
change is made at this operational hold.

Stage 2b specification review over `3862b168` plus the frozen dirty source
finds no P0/P1 defect. It identifies P2 stale module-header claims and two
missing direct negatives: same-setup bootstrap from another checked operation
must fail actual M9 authentication, and ordinary `admit_runtime` must reject a
provider-scoped base. Parent checks both production guards before accepting
the test recommendations; additional tests are not yet run. The production
writer corrects only the stale module header, with scoped format/diff checks;
no implementation behavior changes follow the broad regression. A separate quality
review is requested after specification review, without Cargo below the disk
guard. The reviewer confirms that full non-provider lowering inventory after
locus restriction is provenance, not child-local executable ownership; the
next SYS4/SYS5 consumer must preserve that distinction. Generic materialization
is not a reachable ordinary escape through the current guarded call graph.
No provider activation, host/network execution or milestone acceptance follows.

Quality review additionally identifies a constructor-level hardening gap:
`materialize_m9_resolved_base` can accept a provider-scoped deferred base and
return a bare M8 instance. Parent verifies that the only current production
caller is behind the ordinary M9 provider-scope rejection, so this is not
evidence of an externally reachable bypass. Nevertheless, the component's
constructor invariant should reject this generic materialization directly.
A direct negative and the smallest typed/unrepresentable-boundary repair
remain required before Stage 2b acceptance; neither is run or implemented
while build capacity is held. No weaker authority contract is selected.

Current `provider_call_count() == 0` assertions describe structural inactive
objects; they are constants, not actual adapter invocation instrumentation.
Source inspection finds no provider lookup/read/invocation in this component.
Actual call/host crossing counters and their falsifiers belong to the dependent
runtime component, not evidence claimed from these local preparation tests.

The independent planner approves the 17:13 snapshot maintenance: fresh module
14/14 is included in runtime 359/359, separate process 63/63; quality and
component acceptance remain pending, and capacity does not waive later gates.
Parent retains six of eleven accepted milestones and the fixed stop boundary.

An independent read-only planner approves the intermediate Stage 2b snapshot
correction only: local tests now produce actual effect authorization/inactive
components, but 7/10 is incomplete and unaccepted; 345 is filtered in this
run, while the earlier full 345/process 63 belong to accepted Stage 2a. The
three reproduced blockers and later sealed SYS4/SYS5 consumer remain explicit.
This is status review, not semantic/quality acceptance of the current code.
Parent retains six of eleven accepted milestones, sole-active I3-3 and the
owner's stop after full I3-3 acceptance. Historical 334/current-source-only
snapshot wording is marked retained rather than allowed to describe this cut.
Parent log/diff review rejects a draft snapshot that misread 7 passed / 3
failed as 3 passed / 4 filtered, and corrects retained-cut attribution and
reversed next-consumer ordering before integration. That dated intermediate
7/3/345 result remains history; no draft count is acceptance evidence.

Final Stage 2a independent spec and code-quality/security dispositions both
report P0=0, P1=0, P2=0. The quality reviewer verifies exact source-bound
opaque construction, pure static projection, exact checked-source/topology
snapshot rederivation, centralized typed/identity refusal and unchanged
ordinary entry guards. The final lint delta is annotation/lifetime-only.
Its last written observation predates completion of the post-lint static run;
the parent independently checks the final 11/11 log before integration.

Reopen this static handoff if a later change exposes the ordinary inner
discharge, turns static evidence into authority, changes restricted identity
to erase a provider requirement, admits provider markers through ordinary
snapshot/SYS4/SYS5, or restores the static snapshot without exact rederivation.
Actual permission/binding/M8 materialization/host/network/ledger/cut and all
I3-3 acceptance remain outside this static review's completion claim.

A separate Stage 2a review finding concerns total local omission: the raw
static projection can be restricted to ViewerC, dropping provider-local rows
while retaining the original full program identity. Parent confirms that
restriction preserves identity and adds a genuine restriction falsifier before
repair. Candidate A uses a source-owned query on the existing canonical
structural identity entry: record tag `evaluation`, exact evaluation-kind
field `ReadOnlyProviderEffect`, and Core discriminant `read-only-provider-effect`.
Candidate B introduces separate provider profile/origin state. Independent
review agrees A is smaller and sound for this finite exact-identity boundary;
checked identifiers cannot contain the record separator. A is selected as the
bounded repair direction, not arbitrary substring scanning. Deleting or
changing the identity record changes the identity, whose expected-source/image
binding remains the authority boundary; this is not snapshot authentication.
The new restriction test subsequently reaches the ordinary-export RED as
recorded above. The final frozen 11/11 and 345/345 packet validates the repair,
including ordinary export and SYS4 refusal after genuine restriction.

Stage 2a runtime review identifies a further ordinary-carrier integrity gap:
an otherwise legacy snapshot can encode a provider-only effect in its carrier
effect row or a provider kind in a nested checked identity without changing
the outer legacy kind. Existing outer-kind/placement/detail guards do not
cover those fields. The parent confirms the decoder/preflight path by source
inspection and requests genuine legacy-snapshot mutation tests before repair.
This is not a claim of actual host invocation or bypass of every enclosing
authenticated image check. A bounded typed-marker predicate, not arbitrary
string/failure-name scanning or general snapshot redesign, is the selected
repair direction. The actual 0/2 RED and final 11/11 plus 345/345 GREEN above
establish the bounded repair; eleven typed-marker categories share the guard.

Provider Stage 2a source review (2026-09-08): two P1 interface findings were
independently reported and locally reproduced before production repair.
The new composite checker accepted a source-A normalized candidate against
independently checked source B with a changed resource; the generic candidate
did not retain exact source identity. Its returned composite also exposed
the cloneable ordinary finite discharge through a public getter, allowing
coverage/pending context to be discarded. Existing runtime guards still
prevent provider invocation; neither test demonstrates an actual unauthorized
call. Parent selected an opaque source-bound composite candidate (without
changing the legacy generic candidate) and removal of raw inner discharge
access. Source repair tests subsequently pass as recorded above; independent
source-only narrow re-review reports no remaining P0/P1/P2. Final Stage 2a
spec review, after the nested-marker and omission repairs and final test
packet, closes all four findings with no remaining P0/P1/P2. The dedicated
projector remains pure and full-source-bound; SYS5 remains rejecting and SYS6
changes only metadata-name exhaustiveness. Independent quality review is
 separate and still pending here. The next direct consumer is genuine trusted
binding -> actual M9 composite authentication/verification and separate policy
-> inactive component-scoped M8, not invocation or milestone acceptance.

Actual RED evidence, each one selected test and exit 101:
`/tmp/c3-stage2a-cross-source-composite-red-20260908.NQzZG1/01-cargo-test.log`
records the behavioral `expect_err` receiving a composite discharge;
`/tmp/c3-stage2a-finite-discharge-doctest-red-20260908.atBxcA/01-cargo-test.log`
records the compile-fail snippet compiling successfully. Commands respectively:
`cargo test --locked -p mir-semantics --test i3_provider_composite composite_checker_rejects_a_candidate_derived_from_a_different_checked_resource_source -- --test-threads=1`
and `cargo test --locked -p mir-semantics --doc finite_discharge`, both with
incremental disabled and two jobs. These are distinct behavioral/privacy
REDs, not passing conformance. Postflight free disk was 11,886,387,200 bytes;
no runtime command, cleanup or provider invocation occurred in this gate.

Provider Stage 1: independent source/spec review and runtime quality review
both close with P0/P1/P2 zero after the three construction/restore repairs
recorded above. The private M8 snapshot check is a component integrity
requirement, not a claim that enclosing authenticated image binding failed.
The public M10 guard deliberately retains its existing String error interface;
no provider M10 profile or public diagnostic contract is introduced.
Planner's read-only handoff review retains all 20 fault families, single I3-3
frontier and the owner pause; its sample-classification and final-evidence
notes are incorporated during the component's status synchronization.
Direct SYS3/kernel guards are source-inspected common-entry correspondence,
not falsely counted as independent public test executions. Source M5 export,
private M8 deferred-base and restored-image negatives have their own tests.
Reopen if any legacy path admits provider execution, a provider-tagged legacy
image restores, or a checked provider Core can bypass M7/validated restoration.

Provider contract review: read-only `c3_spec_review` returned five initial P1
and the P2 precision items recorded above. Parent checked against Constitution,
BND-014/016, architecture/08 and actual supervisor defaults before integrating;
final narrow review reports P0/P1/P2 zero. Read-only `i3_remaining_plan` confirms
bounded scope, direct consumers, stage gates and full-I3-3 stop. Its three stale
current pending-integration pointers in Documentation/sample dashboard/Plan250
were corrected to the actual `55f1fd7f` pushed checkpoint. No source or runtime
review is claimed by these contract reviews.

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

Stage C narrow independent re-review finds no remaining P0/P1 in C1/C2
source after the four repairs and verified-handoff G2 correction. Exact
envelope identity is now included in the immutable permit binding. The
reviewer confirms ordinary current-lineage rejection remains unchanged and
that a permit issued under G1 still cannot cross into G2. The shared finalizer
does not expose a reachable post-M8 extraction gap under the present private,
exclusive in-process boundary; no artificial extraction fault API is added.
This is source-review evidence, not a Cargo pass, C3 acceptance or full
I3-3 acceptance. Fresh frozen evaluation is recorded separately.

C3 independent core review reports no P0 and three P1s awaiting concrete
falsifier capture: failure replies omit the shared exact-current pending
lineage check; nonempty decision commitment/occurrence strings can lose their
canonical correlation; and terminal capacity is checked after SYS4 reply
dequeue. The selected narrow repairs preserve the original requester policy,
check canonical decision references without claiming Byzantine authentication,
and preflight capacity before carrier consumption. A separate QUIC component
test will cover the production-used post-admission classifier; no new TLS
fixture/dependencies or fake network evidence are introduced. Production-only
compile checking may run while test files change because those files are not
compiled; behavioral tests still require all compiled inputs frozen and root
authorization. C3 and whole I3-3 remain unaccepted.

At the owner-requested continuation on 2026-09-08, fresh local C3 evaluation
over dirty `050f5067` compiles and runs 16 passed / 3 failed. Two failures
reproduce invalid decision provenance and unbudgeted-source expiry admission.
The third is a fixture defect, not capacity counterevidence: the helper always
advances to tick 1, so its second staged request legitimately remains eligible
at start tick 1 / deadline 2. The test owner changes only the trusted test
clock sequence to 1..64 (the independent second owner starts at 0). All full
requester-state nonmutation assertions remain. The lower SYS4 genuine
G1-pending/G2-successor test separately runs 0 passed / 1 failed by accepting
the declared failure. Logs are `/tmp/c3-1-owner-admission-20260908.log` and
`/tmp/c3-2-requester-lineage-20260908.log`.

The private QUIC classifier test does not run: its intended function/type are
absent, and its direct private message-field assertions also fail compilation.
The latter are test-only repairs using existing predicates, not justification
to widen production visibility. Log:
`/tmp/c3-3-private-quic-classifier-20260908.log`. No result here establishes
actual network time behavior. Source inspection confirms the additional
receiver P1: failure admission must require the exact checked nonempty budget
and declared failure containment at both binder and pending-validation seams;
the producer-side gate alone cannot enforce that receiver invariant.

The read-only independent planner retains C3 -> actual time/QUIC -> distinct
provider contract -> remaining membership/policy/visibility/cut and ordering
evidence -> final regression/review/acceptance. Local save snapshots do not
establish network quiescence; existing RMW records do not establish relation
publication/fallback ordering. No new roadmap, milestone, general theorem,
I3-4 activation or mandatory owner escalation follows. Fresh remote main still
matches `050f5067`; the dirty delta is not committed or accepted.

The repaired test fixture and first three receiver guards subsequently pass
19/19 owner tests and the 1/1 genuine generation test. The capacity result was
not sufficient: trace/store/inbox snapshots omit SYS4's causality graph, and
enqueue followed by dequeue leaves the inbox empty. Adding an assertion using
the existing read-only causality accessor reproduces one phantom dequeue edge
on the 65th terminal-capacity refusal (18 passed / 1 failed). No testing API or
production visibility is added. The implementation then moves the validated
failure's duplicate/capacity preflight before SYS4 admission. Its final rerun
is pending here. The actual-used QUIC post-admission classifier now separately
returns terminal consumption, never a success receipt or a now-invalid pending
handle; its genuine runtime component test passes 1/1. These local logs are
under `/tmp/c3-runtime-batch-20260908.fyXq93/`, not actual network evidence.

One asynchronous Temporary Chat Oracle consult, `i3-provider-minimum-20260908`,
finishes in 9m42s with verified model selection. It receives a bounded context
packet and the Constitution, BND contracts and plan/05; no implicit repository
context is assumed. Its advisory answer agrees that a distinct checked
read-only external-effect operation, generated path and effect-only grant are
unavoidable; neither owner RMW nor the expiry producer is a substitute. It
suggests one bounded task-local integer-file read through a trusted T0 adapter
to the T4 host service, with genuine missing-entry failure, separate typed
result/authority validation, bounded retention and no T2 sandbox claim. The
parent retains this as design input only: the provider contract remains
unselected until its explicit Canon proposal/review. In particular, advisory
terminal-ambiguity wording is not adopted over the existing pending/unknown
requester contract. Output remains local at
`/tmp/i3-provider-minimum-20260908.md`; no new theory, I3-3 acceptance or I3-4
activation follows.

The planner subsequently clarifies the ordering/frontier distinction against
plan/05's "for each accepted operation" and Plan 250's explicit I3-4 pressure
contracts: I3-3 does not need to move relation/designated cross-process
realization forward from I3-4. Keep every ordering edge in the inventory,
execute and classify local producer/positive/falsifier preservation evidence,
and exercise dependency-preserving admission for the network operations I3-3
actually admits. Unsupported-carrier rejection alone is not positive ordering
evidence. No local result becomes distributed relation/designated evidence,
and none of the 20 required failure families is deferred by this distinction.
The parent adopts this narrower sequencing interpretation; the earlier advice
to introduce those crossings during I3-3 is not followed.

The frozen predecessor C3 evaluation passes owner tests 19/19, genuine-generation test
1/1, private QUIC module 3/3 and process-runtime integration 63/63. Two old
integration negatives initially used the pre-sum receipt JSON pointer; only
their pointer is updated to `payload/fields/outcome/fields/receipt/request_id`,
preserving the same request-linkage/replay falsifiers. The independent C3 spec
review finds no P0/P1 source defect but requests focused genuine G1-produced
expiry-after-G2 and dropped-expiry/duplicate-retention coverage. Code-quality
review and those additional tests remain pending at this checkpoint.

The new ordinary-source two-process test executes and fails (0/1); both child
PIDs exit and are reaped, with no orphan. The returned audit labels this
`LifecycleRejected / BeforeOwnerStart` and contains zero counters, but the
parent identifies that the generic child wrapper emits exactly that record
for any inner error, including errors after startup or network admission.
Consequently these labels do not establish the actual failure point or zero
semantic activity. Diagnose the genuine transition and preserve unknown
state instead of adopting the wrapper's default zeroes. The current B loop
also requires `Some(reply)` immediately after admission; a valid source-budget
request can instead be Awaiting. Neither a QUIC handshake nor its absence is
claimed from this failing audit. Logs:
`/tmp/c3-full-validation-20260908.APoPwq/` and
`/tmp/c3-runtime-probe-rerun-20260908.3anlLA/`.

At `2026-09-08T09:11:05+09:00`, the independent quality reviewer withdraws
an initially proposed P1 requiring receiver-verifiable expiry attestation.
Canon architecture/08 explicitly assumes T0 checker/policy/runtime integrity;
architecture/10 uses exact selected-peer transport integrity separately from
receiver-local M9/pending semantic authority. Coherently replacing both a
decision commitment and its occurrence reference requires the raw test seam
or a compromised/alternative trusted peer in this profile; ordinary packages
cannot construct the private runtime message. The parent checks these sources
and agrees: the present negative proves canonical shape and correlation, not
Byzantine T0 resistance or a signed gate proof. No new cryptographic authority
scheme or weakening of the existing trust boundary is adopted.

The two additional reviewer-requested tests are not yet green: the first
focused build reports missing test imports for `Sys4I3ValidatedOwnerReply`
and `Sys4I3PrivateProcessCarrierSnapshot`; its body and the second test have
not executed. The earlier 19/1/3/63 results therefore describe their frozen
predecessor, not this newer test snapshot. The test author owns the import
repair. Log: `/tmp/c3-focused-runtime-20260908.qsiMMN/`.

The fresh follow-up executes the genuine G1-byte test and observes an earlier
fail-closed `CarrierProvenanceMismatch` at the current-G2 carrier binder, not
at its later pending validator. Its test expectation is corrected to that
actual boundary, with pending/state/trace/causality/endpoint/mailbox snapshots
captured before attempted binding. The authentic G1 producer and real M9
successor are preserved; no production admission rule is loosened. The dropped
expiry/exact-request replay and prior-queued-request handoff falsifiers both
pass 1/1. Log: `/tmp/c3-focused-rerun-20260908.vU9Sgy/`. The broader current
local rerun is pending. The quality review closes with no P0/P1 in this bounded
local slice; its two P2 dispositions are the explicit non-Byzantine/shape-only
wording above and fresh coverage of the new tests. Its final repetition of
generic prestart/zero activity is corrected by the parent: the failed probe
does not establish those facts.

The corrected current local test snapshot then passes the owner-admission
filter (31/31) and the `i3_` library filter (51/51), including the authentic
G1-bytes/current-G2 binder rejection, dropped-expiry exact replay and prior
queued-request refusal. These sets overlap and are not summed as unique
tests. Both use locked serial tests with the existing process-test seam
feature, `CARGO_INCREMENTAL=0` and `CARGO_BUILD_JOBS=2`; the two known
dead-code warning categories remain, so this is not a warning-free clippy
claim. Logs: `/tmp/c3-complete-focused-20260908.BPNWXy/`. Available disk remains
13.39 GiB. Production ownership is now separated for genuinely independent
time-consumer work: the opaque host driver in the runtime file, and truthful
child/supervisor diagnostics in the probe file. The sole evaluator still
waits for the relevant compiled inputs to freeze.

The corrected diagnostic probe executes the existing budgeted positive test
again (0/1, expected behavioral RED). It now reports actual B-local
`OwnerAdmissionAwaiting`: verified peer/preface, one handshake, one admitted
request, Awaiting 1, serve/write 0, and B natural zero exit. A contributes an
explicit unknown child failure rather than synthetic zero semantic evidence.
The aggregate cause is `OwnerAdmissionDriveRequired` at
`LifecycleEvidenceRejected`; both actual child PIDs are reaped and the
supervised observation is 416.455 ms within its 21 s envelope. This proves the
missing owner-driver continuation after real selected-transport admission,
not successful resolution or a delivered expiry. Parent diff review also
corrected the generic post-Ready fallback to change its failure stage as well
as retain its observed start. Log:
`/tmp/c3-probe-diagnostic-20260908.Llvtgv/01-budget-positive-probe.log`.

The current documentation check passes agent configuration validation, Canon
index (213 files), source hierarchy (800 required/present), and documentation
scaffold validation. The subsequent sample/runner navigation additions
explicitly classify the ordinary budget source as current test input with
network validation in progress; they add no script or public workflow.

The opaque host-driver tests first fail to compile solely because the two
intended runtime methods are absent (14 `E0599` uses across five tests).
After the source owner adds the non-Clone/private-field driver and the
optional monotonic-tick/one-Awaiting continuation, all five tests pass and
the broad `i3_` library filter passes 56/56. The API accepts no request ID,
carrier, permit or expected outcome; it validates the exact runtime/owner
driver even for an empty queue and invokes only the retained gate/handoff.
Tests cover current-tick serve, deadline expiry/terminal consumption, foreign
and backward input nonmutation, empty/settled non-reissue, and absence of
drivers for unbudgeted source. These are local runtime tests, not the pending
probe continuation. Logs: `/tmp/c3-host-driver-red-20260908.zYxHM6/` and
`/tmp/c3-host-driver-green-20260908.ZwyanW/`.

Formatting coordination correction: the runtime writer's direct `rustfmt`
followed child modules and mechanically formatted the test-author-owned
untracked module before the green run. That tested formatting is preserved;
no untracked prior state is reconstructed. Its unrelated snapshot import
reordering is restored with an exact `apply_patch` of the writer's own delta.
Subsequent scoped formatting uses `skip_children=true`. There is no semantic
test change claimed from formatting and no concurrent Cargo writer.

The first default probe wiring used the wrong post-handoff summary: it
expected an outstanding `ServeReserved`, although the genuine successful
finalizer transitions the retained tombstone to `Received`. Parent source
inspection identifies this mismatch after a run reports two unknown child
errors; that diagnostic alone cannot establish execution or nonexecution.
The probe guard is corrected to zero outstanding reservations, one retained
tombstone and the unchanged actual serve/write counts of one.

The next run returns a successful process result and passes the two-child,
serve/write/reply/receipt and four-delivery assertions, but fails the later
source-reference comparison: the new sample still receives the generic
`i3-2-private-input.mir` alias. This is not a pre-start failure. The writer
adds only the known LAB sample's safe logical alias alongside the existing I2
allowlist; all arbitrary paths retain the private alias and no lineage test
is weakened. Full probe regression awaits the corrected rerun. Logs:
`/tmp/c3-probe-green-20260908.P21PZi/` and
`/tmp/c3-probe-guard-rerun-20260908.IoHczL/`.

The corrected source-first budgeted round trip passes 1/1; the entire probe
integration target then passes 33/33. The new positive checks two actual
exec/reaped processes, request/serve/write/reply/receipt each one, and all four
distinct observer-safe request/reply delivery records against exact checked
source/Core/artifact/edge/request contracts. Existing diagnostics remain
green. This is actual default-time network execution, not delivered-expiry
or full row-17/I3-3 acceptance. Logs:
`/tmp/c3-probe-alias-rerun-20260908.9fCbJY/`; final available disk 13.37 GiB.

Next, a private producer-return wrapper retains the already generated expiry
commitment and occurrence in the existing owner terminal ledger, only after
successful message construction. A read-only observer projection cannot
reconstruct these refs from counts, transport data or caller strings. The
neutral finite host schedule advances tick one after actual Awaiting and
branches on the gate's real outcome. Live expiry requires all four network
records plus actual owner production and requester terminal consumption, and
both natural zero exits/reaps even though the typed command outcome is an
error. The existing post-admission reconnect runner is reused with distinct
retained Expired evidence; no second retry runner or schedule-minted expiry
is selected. Tests precede these source changes.

The producer-ref boundary first fails at its intended absent APIs, then the
two new runtime tests pass: retained refs equal the actual serialized
gate-produced reply and requester terminal; a genuine M9 revocation before
resolution produces no expiry record. The broad `i3_` library filter passes
58/58. The three new probe cases then fail to compile only at their intended
new drive-profile/audit/stage interfaces (nine missing APIs), establishing
the next implementation consumer rather than network evidence. Logs:
`/tmp/c3-expiry-decision-red-20260908.XRbMVT/` and
`/tmp/c3-expiry-runtime-probe-batch-20260908.31qZhM/`.

While that probe-only implementation proceeds, an independent test-only
assignment exercises the already accepted `prepare_finite_admission`
missing/unknown auth-discharge boundary for row 15. It introduces no new
semantic frontier, provider contract, dynamic policy-revocation claim or
network claim; the runtime production files stay frozen. The parent schedules
all Cargo commands and keeps compiled-input ownership disjoint.

The new public-method admission-policy integration target passes 2/2:
missing and unknown actual auth-discharge inputs reject before prepared/live
runtime handles, preserve checked project metadata, and permit a subsequent
genuine admission/start. This is local pre-activation evidence, not dynamic
revocation or network/provider execution. Log:
`/tmp/c3-new-admission-policy-20260908.GQ89cJ/`.

Scoped `cargo clippy --locked -p mir-runtime --all-targets -- -D warnings`
is RED, not a validation pass. Eleven new unit-test calls incorrectly depend
on the feature-gated public decoded-ingress test seam under the default
feature command; the unit test owner switches those calls to the existing
crate-private shared admission body, preserving decoded candidates and every
assertion without exposing another production API. Clippy also reports nine
dead-code groups and the large admission-resolution enum. A bounded source
owner audit separates baseline/feature-dependent warnings from this delta
before repair. Log: `/tmp/c3-clippy-runtime-20260908.LbZYWv/`.

Parent source inspection additionally corrects a prior review premise:
`mirrorea-i3-probe` enables `i3-process-test-seams` for its negative bootstrap
controls. Consequently absence of that compiled API from the probe cannot be
claimed. The actual normal child path still uses verified QUIC followed by
the shared receiver checks; the T0-integrity/non-Byzantine boundary, not a
false compile-absence claim, governs the correlation-only assurance. Narrow
review is requested on this factual correction; no raw Mir/native ingress
route or new authority scheme is introduced.

After the unit-only repair, the exact default-feature `cargo test --locked
-p mir-runtime --lib i3_ -- --test-threads=1` passes 57/57 (not the
feature-enabled 58-test set). Three configuration-dependent dead-code groups
remain. Log: `/tmp/c3-default-i3-20260908.1QuedH/`.
The narrow independent trust review confirms no P0/P1 change: the compiled
test seam is callable by trusted probe code, but repository call inspection
finds no current probe/Mir-source/native-FFI caller bypassing verified QUIC.
Its presence is an in-process accidental-regression surface, not a present
remote entry. The stale absence/unlinkability source comment is a P2 to
correct. No cryptographic gate-attestation claim follows.

Actual-time network checkpoint, `2026-09-08T10:15:31+09:00`: the four-case
ordinary budget-source integration filter executes 3 passing cases and one
behavioral failure. Delivered tick-one expiry is consumed as a terminal
failure, expiry followed by a lost reply rejects the exact reconnect duplicate,
and tick-zero serve passes. The served-but-lost-reply case validates actual
owner serve/write and source-bound evidence, then fails the requester pending
retention audit assertion. The full 36-test target is not run after that RED.
The source owner is investigating the actual child-to-supervisor pending
projection; no profile-derived pending or remote-nonexecution fact is admitted.
Log: `/tmp/c3-time-probe-final-rerun-20260908.nqVyY1/`.
Earlier attempts stopped at compile errors (borrowed counter comparisons,
then two old exhaustive reconnect-result matches); those were repaired
without weakening the old non-expiry assertions. Independent review of the
actual-time delta is in progress. This is not row-17 or I3-3 acceptance.

The pending projection repair then passes all four actual owner-budget cases
and the full probe integration target 36/36. The matching runtime feature
union (`i3-private-quic,i3-process-test-seams`) passes library `i3_` 61/61
and all-target deny-warnings Clippy. The prior 58 and default-feature 57
counts are different feature selections, not results to add together.
Clippy's four test-only `err_expect` sites were replaced with explicit matches
without exposing Debug/Clone on linear success values. Private resolution
payloads are boxed without changing one-use ownership; the expiry producer
now checks its exact private binding explicitly, and a field-scoped lint
expectation documents retention of the exact requester commitment.
Logs: `/tmp/c3-runtime-clippy-rerun-20260908.tph1dS/` and
`/tmp/c3-clippy-probe-final-20260908.sUzTbQ/`. No matching child executable
remains after the probe runs; root has 13.14 GiB free.

Independent actual-time review finds no current P0/P1 but identifies an
unexecuted required condition: physical peer close is not requester-local wait
expiry. The parent selects an explicit local monotonic wait over the retained
request after the existing causal owner-serve/withheld-reply/close path.
Before/after pending and receipt/terminal counts plus actual elapsed evidence
must preserve remote uncertainty and perform no new send. A keepalive route
with an extra causal barrier is the one considered larger alternative; it
adds no stronger required semantic fact. This implementation remains within
spec/16's T0 host boundary, not a new timeout/provider contract. Actual reply
replay/staleness across sessions remains separate required evidence. These
gaps prevent time-row or I3-3 acceptance despite the current 36/36 result.

Sample-taxonomy spot check: `samples/clean-near-end/README.md` now explicitly
distinguishes the budget source's probe-only execution from unsupported
`run-local`, and distinguishes program activation from official I3 lifecycle.
The optional HTML-reader inventory test
`python3 -m unittest scripts.tests.test_mir_hilight_html.MirHilightHtmlTests.test_embeds_every_active_clean_near_end_mir_sample`
fails first on the already committed
`i1plus-reference/scn-01/negative-missing-visibility-denied.mir` entry, not the
new budget file. `git ls-tree HEAD` confirms that predecessor file and
`git diff -- mir_hilight.html` is empty. The reader therefore has a preexisting
stale embedded sample catalog, also not updated for this new sample. It is
not a passing check or an I3 runtime failure; broad HTML catalog regeneration
is not opened as a new semantic frontier. No reader output is Canon evidence.

Local-wait completion checkpoint (`2026-09-08T10:46:24+09:00`): an initial
compile RED captured the absent profile/falsifier/audit API in
`/tmp/c3-localwait-red-20260908.rvPI1o/`. The implemented probe-only wait
then passes six named budget tests (seven behavioral cases, including two
observer-only corruptions) and full probe integration 38/38. The real
monotonic wait is fixed at 20 ms, follows the selected actual loss path, and
retains pending `1 -> 1`, receipt `0 -> 0`, and terminal failure `0 -> 0`.
Invalid elapsed/pending evidence produces lifecycle-evidence rejection, no
accepted fault audit and no promoted pending fact. It is not an adapter
timeout, remote expiry or retry. Logs:
`/tmp/c3-probe-localwait-20260908.WrlwI3/` and
`/tmp/c3-probe-localwait-full-20260908.xCtTQc/`.

Fresh accepted-regression floor passes SYS5 5/5, SYS6 CLI 8/8 and M10
conformance 67/67 in `/tmp/c3-runtime-regression-floor-20260908.L0a44m/`.
That no-feature runtime build retains seven preexisting retry-related
dead-code warning groups, distinct from the selected feature-union gate.
The first two-crate deny-warnings Clippy attempt rejects two probe style
lints (eight named audit arguments and a manual Option map). The source owner
applies a documented constructor-scoped expectation and an equivalent map;
quality rerun and final narrow review are pending at this record. No full
milestone acceptance follows from these component results.

Canon-first independent planner review approves the row-11 evidence plan:
actual successor-session duplicates of both successful and expiry replies,
plus the existing local genuine G1-outcome/G2-requester binder falsifier and
verification that actual QUIC consumes through that binder. Only B currently
has the live successor control; installing B-G2 while A remains G1 would not
prove requester-generation staleness and is deliberately omitted. The
adapter-only replay token is not yet implemented or accepted. It must retain
only the exact successfully sent generated reply, bind the permitted peer/
cohort/successor session, permit one explicit replay attempt and preserve the
first requester decision. No second Quinn harness is added merely for unit
tests: existing component binding tests remain, and the actual two-process
probe is the direct producer/falsifier consumer. Plan 250 records this finite
classification without reducing any required family or opening I3-4.

Parent source inspection at the pinned `30429d5` cut confirms the production
correspondence: `receive_and_admit_generated_message` completes bounded frame
decode after verified peer/preface checks, then calls
`admit_decoded_process_message`; that function checks cohort and invokes
`bind_i3_untrusted_process_carrier` before `accept_inbound`. The binder compares
reply M9 lineage with the current owner generation. The existing genuine
G1-expiry/G2-requester test reaches this same binder, not a substitute validator.
Row-11 replay must use this ordinary receive path; a new pre-receive pending
guard would not establish its rejection. Candidate commitments include the
session generation: compare sender/receiver within each attempt, not equality
across sessions. The retained exact body and token-derived original successful
send binding establish replay correspondence. This is source inspection for
the row-11 direct consumer, not unexecuted network acceptance.

The three actual-process replay tests are authored first (successful reply,
genuine expiry reply, and initial-session prewrite rejection). Their initial
command, `cargo test --locked -p mirrorea-i3-probe --test
i3_process_localnet reply_replay -- --test-threads=1`, with incremental builds
disabled and two build jobs, exits 101 on the missing planned replay API;
no test body executes. Log: `/tmp/c3-row11-initial-red-20260908.mv524A/`.
This is API-availability RED, not behavioral evidence. Parent also catches
three test calls to nonexistent `failure_stage()`; tests use the existing
`stage()` instead of adding a redundant production alias. The adapter and
probe writers then begin disjoint production changes; runtime/acceptance
results are still pending. Free disk at this gate is 12.02 GiB.

Row-11 implementation now executes the three actual-process cases. The first
integrated build exposes two missing read-only preface cohort accessors, not
behavioral failures; the bounded repair adds that diagnostic getter without
changing a field, codec or grant. The rerun passes all three tests, with 38
filtered, in `/tmp/c3-row11-replay-rerun-20260908.1qkaqU/`; no orphan probe
child is observed. Two unused new session-plan fields are then removed, not
suppressed. Full probe regression, deny-warnings Clippy, format and independent
time/reply review follow this freeze; those results are not yet claimed.

Parent integration also repairs three evidence issues before that green run:
the initial-session falsifier takes A's actual final runtime snapshot rather
than cloning its first snapshot or claiming an observation after B's later
refusal; expiry evidence joins the exact actual B receive occurrence; and
replay acceptance checks the outer terminal's request identity, mutation count
and absence of conflicting evidence. On the accepted successor path, A's
first consume precedes its close/reconnect, and the actual replay is rejected
by the ordinary receiver. Peer close alone is not a consumption proof: the
supervisor additionally requires A's independent known-result record. The
wrong-initial-session branch creates no second session or replay delivery.
The adapter-only regression at the earlier freeze passes three existing
private-QUIC component tests in
`/tmp/c3-runtime-private-quic-20260908.SvT8pR/`; it is not the actual replay test.

Final reviewed time/reply disposition (`2026-09-08T12:04:34+09:00`): full probe
integration passes 41/41 in `/tmp/c3-final-slice-20260908.at7tQr/`.
Deny-warnings Clippy first finds a large observer-only enum and the existing
private error constructor's eighth named argument. Boxing the delivery record
and a documented constructor-scoped arity expectation preserve semantics;
the rerun is clean in `/tmp/c3-row11-quality-repair-20260908.iN8Leo/`.
The subsequent workspace format failure is a scope/style configuration
mismatch in the earlier local formatter invocation. Direct Rustfmt with
explicit `edition=2024` and `style_edition=2024` repairs only the owned file.
Workspace format and the focused actual replay 3/3 then pass in
`/tmp/c3-row11-format-rerun-20260908.KJQvAz/`; no orphan children remain.

Final independent semantic/security/concurrency review records P0/P1/P2 zero;
Canon-first planner review confirms the same bounded consumer disposition.
After documentation and commit/parity integration, row 11 and the spec/16
time/reply consumer can close. Reopen for cloneable/raw-exportable replay
tokens, issuance before complete write, wrong control/generation use, a
prewrite refusal reserving an occurrence, replaced/duplicated decisions,
missing child joins or bypass of the current-generation production binder.
Evidence is finite `runtime-monitored`, with the G1→G2 case explicitly local;
no new Lean/general proof, network requester-generation update, provider
support, whole I3-3 acceptance or public compatibility claim follows.
Baseline runtime 332/process 63/I2/M10 5/8/67 are retained `30429d5` evidence,
not reruns of this replay delta. Provider contract and implementation, the
remaining matrix/order and full milestone acceptance still follow.

Final documentation gate (`2026-09-08T12:15:50+09:00`) passes `make docs`:
agent configuration, Canon index 213 files, hierarchy 800/800 and scaffold
1760 reports. `git diff --check` is clean. The bounded credential-pattern
scan checks 13 changed paths with zero matches and prints no secret contents;
no entropy scanner is used or implied. Logs:
`/tmp/c3-doc-validation-20260908.paufCw/`. Independent planner's final seven-file
documentation-diff check finds no material P0/P1 or incoherent current claim.
Root free space remains about 12.00 GiB; no cleanup is performed. Parent now
integrates this reviewed source/evidence checkpoint before the provider contract.

The independent local-wait re-review closes its previous P2 with P0/P1 zero
and no new P2. A reports missing reply generically (`Err(_)`), not a separately
identified QUIC peer-close error: the physical close belongs to the trusted
selected B serve/withhold/close path. Its local elapsed observation and the
independent same-request B record are the accepted finite evidence. Full
row-11, time-matrix and I3-3 acceptance remain open.
The two-crate Clippy rerun is warning-free; the subsequent all-workspace
format check finds one test-only panic wrapping, repaired exactly without
behavior changes. Whole-runtime and format reruns remain pending at this
record. A changed-file credential-pattern scan covers 27 files and finds no
private-key header, GitHub token, AWS access-key or project API-key pattern.
No dedicated entropy scanner is installed; this is a bounded scan, not a
general secret-freedom proof.

Final bounded checkpoint gates (`2026-09-08T10:55:34+09:00`) pass: workspace
format; feature-union whole runtime library 332/332; feature-union process
integration 63/63; two-crate all-target deny-warnings Clippy (only the exact
verified whitespace change followed that run); and `make docs` (agent config,
213 Canon indexed files, 800/800 hierarchy paths, scaffold with 1760 reports).
Runtime logs: `/tmp/c3-runtime-feature-union-20260908.0HlZJi/`.
Full probe 38/38 and I2/M10 5/8/67 are the fresh results above; no source
behavior changed afterward. Final snapshot synchronization corrects stale
counts, the ambiguous shorthand "6/7", and a reused log timestamp: the actual
meaning is all six named tests/seven behavioral cases passing. Local G1→G2
binder evidence is already green, not a missing implementation; actual replay
and its receive-path correspondence are the next consumer. Root free space
is 12.03 GiB, with no cleanup performed. The optional stale HTML-reader test
remains the explicitly reported failure, not an I3 runtime or passing gate.

## Skipped validations and reasons

Stage 2c: no new Lean/general model, provider activation/host call, full
workspace tests or probe-all-tests run is claimed. The component changes a
private inactive image/validation boundary and its shared ordinary helpers;
the final frozen focused/runtime/process/I2/localnet/quality/check packet above
is its bounded regression evidence. Actual provider runtime and remaining
fault/order/model/full-workspace gates remain required inside I3-3. The nested
M8 private DTO version change is not a public or durable format migration.

Stage 2b current resource hold: after fresh library 359/359 and process 63/63,
available root space is about 9.65 GiB, below the 10 GiB pre-command guard.
The planned public guard rerun, warning cleanup/Clippy, extra review negatives
and further integration builds are not run. No new cleanup permission has
been received; prior one-directory cleanup authority is not expanded. The
full-milestone obligations remain open. Parent's lightweight `make docs` passes
(agent configuration, 216 Canon entries, 800/800 hierarchy, 1760-report scaffold)
before the final snapshot synchronization. The post-update `make docs` also
passes all four checks; parent diff whitespace checks pass. Neither command
replaces the unexecuted Rust quality or review-negative gates.

Read-only cleanup candidate inventory is rechecked at
`2026-09-08T17:18:24+09:00`. These five regular single-link regenerable files
under `target/debug/deps/` total 1,885,726,492 bytes (about 1.76 GiB):

- `libmir_runtime-82ee8eb6f7dfa021.rlib` (380,289,468 bytes)
- `libmir_runtime-093be29134d3830d.rlib` (378,739,914 bytes)
- `libmir_runtime-9343d5e60c3bd8a8.rlib` (376,320,432 bytes)
- `libmir_runtime-b74aa02dad80a2cd.rlib` (375,311,936 bytes)
- `libmir_runtime-7fa5e776cb77eefa.rlib` (375,064,742 bytes)

This is an approval request inventory, not deletion evidence. Source, Git
history, report/test logs and existing process-evidence binaries are excluded.
No build-cache deletion occurs before owner approval; capacity must be checked
again afterward, and this finite reclaim is not a whole-milestone space promise.

Provider Stage 2a: no new Lean/model run, provider invocation or full I3-3
matrix/acceptance is claimed. The static component's fresh tests and completed
regression/quality gates are recorded above. Full workspace tests and full
probe/matrix gates remain required at I3-3 close. The existing unrelated
optional HTML sample-catalog failure is not rerun or counted as a pass here.


Provider Stage 1: full workspace tests, full probe rerun and new Lean/model
runs are not performed for this static/guard component; relevant source,
runtime/process/I2/M10 regression, all-workspace compilation and quality gates
above are executed. Full final I3-3 gates remain required. The pre-existing
optional embedded-HTML sample-catalog failure is retained as a known unrelated
UI failure, not repaired or counted as passing by this slice.

Provider contract checkpoint changes only Canon/LAB documents and generated
index. Rust/Lean/runtime/network tests and Clippy/format are not rerun for this
docs-only decision; earlier exact-cut results remain retained evidence. The
provider's behavioral RED and implementation validation follow this contract
integration. This does not waive the final full-I3-3 gates.

The planning writer runs no builds or runtime tests: the assignment is limited
to two planning/report files and the parent already supplied focused baseline
results. Full I3-3 fault/order coverage, I2/M10 close regression, independent
milestone acceptance and final remote parity are still unexecuted for I3-3.
The standalone bounded model above does not replace those checks.
They are required as applicable before milestone close,
not skipped passes. General proof, WAN/production, durability and Browser/Host
product realization remain outside this bounded milestone.

## Commit / push status

Stage 2c parent final integration gates: `make docs` rerun exits zero,
observed `2026-09-08T21:23:31+09:00` (agent config, 216 Canon entries,
800/800 hierarchy, 1760-report scaffold). Report-section checks and staged
whitespace checks pass. Sixteen-file added-line credential-pattern scan has
zero matches; this is not a general secret audit. Parent accepts this bounded
inactive handoff for the actual Stage 3 consumer and performs its scoped
commit/push next. Actual resulting identity/parity remains an observed
post-commit fact to record with that next in-scope work, not a future hash.

Provider Stage 2c's frozen source/test packet, independent reviews and Rust
gates are green over `7142b205`. Parent is synchronizing LAB records and
performing final documentation/diff/changed-line credential checks before its
scoped commit/push. No future self-referential hash or remote success is
asserted. The actual observed cut/parity will be recorded with the next
in-scope provider-runtime work. This integration is inactive-component-only:
I3-3 remains active and the owner-requested stop applies to its full close.

Provider Stage 2b is committed/pushed as
`7142b205b4e2805d50ce78156de99e6feb1db37a`
(`feat: seal inactive provider composite with scoped authority`). Parent
observes clean HEAD/origin/main/live remote parity at
`2026-09-08T19:22:07+09:00`. This accepts the inactive component only;
I3-3 remains active/unaccepted and I3-4 inactive. The next in-scope Stage 2c
starts from that cut and retains the owner-requested I3-3 stopping point.

The final Stage 2b source/test packet and independent reviews are green and
ready for parent's scoped integration over `3862b168`. Parent synchronized
LAB snapshots and observed `make docs` exit zero at 19:21 JST (agent config,
216 Canon entries, 800/800 hierarchy and 1760-report scaffold). Staged
whitespace and 19-file added-line credential-pattern checks pass with zero
matches; the latter is not a general secret audit. The final report-only
metadata edit is checked with the section validators and staged diff check.
Actual commit/push/parity follow; no prospective self-referential hash is
asserted. Record the exact observed cut with the next Stage 2c work. This
supersedes the earlier capacity-held Git state below without accepting I3-3.

Historical capacity-held checkpoint (superseded by the final packet above):
Stage 2b changes remained uncommitted over
`3862b1688821c8fb243f8f148f54cd313ad3b163`; local HEAD and origin/main still
matched that accepted Stage 2a cut. Required Stage 2b review/quality gates were
not complete, so no new integration commit or push was claimed. Existing dirty
implementation, tests, status and evidence are preserved. I3-3 remains active
and unaccepted; the eventual owner-requested pause is still after full I3-3.

Provider Stage 2a is committed/pushed as
`3862b1688821c8fb243f8f148f54cd313ad3b163`
(`feat: preserve static provider coverage and projection boundaries`). Parent
verifies HEAD/origin/main/live remote parity and clean worktree at
`2026-09-08T15:41:54+09:00`. This supersedes the pending Git record below.
The complete static handoff is accepted as a component; I3-3 remains active,
not accepted, and I3-4 remains inactive. Stage 2b is the next in-scope work.

Provider Stage 2a production and tests are frozen over `9e8d674a` with
independent spec/quality reviews and executed code gates green. Parent final
`make docs` passes again (agent config, 216 Canon entries, 800/800 hierarchy,
1760-report scaffold). Staged whitespace checks and a 20-file added-line
credential-pattern scan pass with zero matches; this is not a general secret
audit. The last report-only metadata addition is checked with the repository
section validators and diff inspection. Scoped commit/push/parity follow. Record the
resulting exact cut with the next in-scope Stage 2b work, not a predicted
self-referential hash. This closes only the static handoff after those gates;
it is not I3-3 acceptance, a lifecycle change or the owner's stopping point.

Provider Stage 1 is committed/pushed as
`9e8d674a29b724d3947c8fccf1cf78e62a491206`
(`feat: check source-declared read-only provider effects`). HEAD, origin/main
and live remote match with clean worktree at `2026-09-08T14:05:06+09:00`.
Stage 2 static work starts from this exact cut. This is not provider execution,
I3-3 acceptance, an official lifecycle transition or the requested stopping point.

Provider Stage 1 production/tests are frozen with independent reviews and
executed code/documentation checks green over `985ee179`; parent scoped
commit/push/parity are pending. Exact resulting cut will be recorded
with the next in-scope component, not predicted as a self-referential hash.
This checkpoint is not I3-3 acceptance or the requested stopping point.

Provider contract integration is committed/pushed as
`985ee179b76d8e1a9e72571ef5a275e190ae4cf9`
(`docs: select bounded source provider effect contract`). Fresh live remote,
HEAD and origin/main match with a clean worktree at
`2026-09-08T12:50:23+09:00`. Stage 1 source/test work begins at that cut;
no provider execution or I3-3 acceptance follows from the contract commit.

The reviewed row-11/spec/16 time-and-reply checkpoint is committed and pushed:
`55f1fd7f76b86a2fc6a846c0133d55d5c8213831`
(`feat: reject cross-session replay of known owner replies`). HEAD, main,
origin/main and live remote matched with a clean worktree at
`2026-09-08T12:16:45+09:00`. Provider contract work starts from that exact cut.
This is a component checkpoint only; I3-3 remains active and unaccepted.

The reviewed row-11/spec/16 time-and-reply integration over `30429d5` is
prepared for the parent's scoped commit/push after LAB synchronization and
documentation validation. Its exact hash and remote parity will be recorded
with the next in-scope provider work rather than predicting a self-referential
commit hash here. This component checkpoint does not trigger the owner pause;
only full I3-3 acceptance does.

The C3/time-runtime checkpoint is committed and pushed as
`30429d5d0521d4ad03fd0500ad49092c89042caf`
(`feat: enforce owner admission budgets across process transport`). At
`2026-09-08T10:58:12+09:00`, fresh remote lookup, HEAD and origin/main agree
and `git status --porcelain=v1` is empty. Its 27-file integration contains
the clock/gate/one-use handoff, genuine expiry transport/consumption, local
wait evidence, tests, source fixture and LAB synchronization. This is a
validated component checkpoint only. I3-3 remains active and resumes with
actual reply replay; I3-4 remains inactive until full acceptance and owner resume.

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

That integration is committed and pushed as
`050f5067c5f63384abd7b8a6389158ddc76187da`
(`feat: preserve and guard source owner admission budgets`). Fresh remote
lookup matched HEAD/main/origin/main at `2026-09-07T21:09:01+09:00`; the
worktree was clean before the next authorized Stage C test change.
This is the fifth source checkpoint inside I3-3, not milestone acceptance.

## Sub-agent session close status

Current provider Stage 2c: the production writer, separate test author, sole
Cargo evaluator, independent specification/quality reviewers and read-only
planner have returned their bounded results. The exact post-review source is
frozen with all required component Rust gates green. The five-file LAB writer
has completed snapshot synchronization; parent completed report/roadmap/docs
integration and owns acceptance/Git. The mapper has returned a read-only map of the next
existing runtime/carrier/FD3 consumer; no Stage 3 implementation has started.
Contexts are retained for the same I3-3 continuation, not competing goals.

Historical provider Stage 2b: production/test writers, the sole evaluator,
independent specification/quality reviewers, planner and five-file LAB writer
have returned their bounded results. Final source/test behavior is frozen;
the repaired constructor and full current regression/quality packet are green.
Parent owns integration, Canon/status and Git. The production writer has only
a read-only Stage 2c planning task until this component is committed/pushed.
All contexts are retained for the same I3-3 continuation, not competing
implementations or I3-4.

Current provider Stage 2a: both production writers, separate source/runtime
test authors, sole evaluator and independent spec/quality reviewers have
returned their bounded results. Contexts are retained for continuation.
The next coupled M9/M8 implementer has a read-only Stage 2b API preparation
assignment; production waits for this static cut's commit/push/parity. Parent
owns acceptance/Git and continues I3-3. No I3-4 work or final complete notice
is issued. This supersedes older active-assignment snapshots below.

Provider Stage 1: source/runtime and both test writers are frozen; independent
source/spec and runtime quality reviewers have returned no remaining findings.
The sole evaluator completed focused, regression, quality and workspace
compilation checks. Parent owns LAB synchronization and Git integration. Mapper
returned the exact Stage 2 coupling packet; the planner monitors one bounded
Oracle handoff consult. All contexts are retained for the same I3-3 consumer.

Provider contract: mapper, source implementation planner, acceptance-command
evaluator planning, independent planner and contract reviewer returned their
bounded read-only packets. The final contract reviewer reports P0/P1/P2 zero;
the evaluator now owns only docs validation. All contexts remain retained for
source/test implementation, no production writer has started, and I3-4 is inactive.

At the reviewed replay checkpoint, adapter/probe/test/evaluation writers,
independent quality reviewer and read-only planner have returned their bounded
results. The five-file LAB writer hands back its files; parent corrects stale
pending-replay/review wording, separates retained `30429d5` results from fresh
41/3/Clippy/format evidence, and restores the prior recent-log timestamp before
adding a genuinely new log entry. Parent owns the final synchronized snapshot.
All contexts are retained for the provider direct consumer;
no I3-4 assignment is active.

At the C3/time-runtime checkpoint, source/test/evaluation/status writers and
the narrow reviewer have completed their bounded assignments and remain idle
with context retained for the next I3-3 reply-replay consumer. Parent owns
integration; no I3-4 work is authorized or started.

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

Current Stage C handoff: C1/C2 source, tests, evaluation and independent review
have returned bounded green results. The read-only planner confirms the
existing dependent sequence; the scoped status writer has returned its four
snapshots, followed by parent correction of cut/command anchors. C3 production
and tests are active after the captured availability RED; sole evaluator is
idle until the parent receives both freezes. QUIC mapping is advisory and its
implementation remains dependency-gated. Agent contexts are retained; no
whole-I3-3 acceptance, requested pause, or I3-4 activation has occurred.
