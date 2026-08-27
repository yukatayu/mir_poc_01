# Report 2596 - Mirrorea I2 Systems Foundation SYS-4: in-process generated dispatch

- Date: 2026-08-27 JST
- Milestone: SYS-4
- Status: **completed / closed**
- Accepted implementation/evidence cut: `22196f93b0112b8fd2987ec078021c8865b71651`
- Canon decision: PROPOSAL-033 / ADR-0030 / spec/13
- Evidence class: OBL-061 `runtime-monitored`, finite internal dispatch only
- Lifecycle: theory T1; broad PHASE-I1 and official I2 entry/exit unaccepted

## Objective

Execute the SYS-3 generated per-locus artifacts as independent in-process locus
endpoints, using only checked program identity, generated communication plans,
complete sealed M9 admission, and typed initial values. The milestone closes the
finite SYS-4 dispatch path without source reparsing, fixture-name plan lookup,
manual route construction, direct cross-locus store mutation, authority/result
injection, public carrier/API/ABI/wire freeze, real transport, durable
distributed persistence, or general theorem claims.

The accepted result is a crate-private Mirrorea I2 Systems Foundation runtime
foundation cut: actual endpoint crossing, carrier-side idempotent return for
the bounded designated-result consume path, typed fail-closed dispatch faults,
typed observer snapshot failure, deterministic replay, ST whole-fabric local
cut/restore, and bounded quiescent designated-only checked patch.

## Scope and assumptions

SYS-4 consumes the accepted SYS-3 projection result and does not reinterpret
ordinary source at runtime. `LocalFabric::bootstrap` admits only exact checked
program/projection identity, complete sealed M9 authority evidence, exact
projected locus inventory, and typed initial values for already projected local
schemas. Initial values can fill owner-local projected state but cannot create
new operations, loci, edges, schemas, authority, capability, witness,
membership, Core nodes, or expected results.

The accepted ST profile is process-local and finite. OW1 is accepted only for
eligible one-owner-worker selected correspondence and observer snapshot
behavior. OW1 save/restore and checked patch remain typed
`BackendIneligible` residuals.

```text
Direct consumer: SYS-5 minimal typed devtools and local virtual-space vertical slice
Blocker reduced: accepted SYS-3 artifacts now actually cross generated locus
  endpoints and produce typed trace/failure/cut/patch evidence
Acceptance use: SYS-5 causal view and four-locus toy; SYS-6 finite I2 assurance
```

## Start state / dirty state

- Branch: `main`.
- Start baseline for SYS-4 implementation: accepted SYS-3 source/evidence cut
  `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`.
- Accepted SYS-4 implementation chain included `38e653cf` for partitioned
  fabric cuts, `61106954` for typed observer fail-closed behavior, and
  `22196f93` for bounded checked SYS-4 patch activation.
- At the closeout handoff, `HEAD == origin/main ==
  22196f93b0112b8fd2987ec078021c8865b71651`.
- Parent close-validation later found one workspace Clippy issue in
  `crates/mirrorea-cli/src/main.rs:4506`: a bool literal assertion unrelated
  to SYS-4 behavior. The parent applied the minimal hygiene change
  `assert_eq!(..., true)` to `assert!(...)` and reran format/Clippy
  successfully. This report records that source delta as validation hygiene,
  not as SYS-4 capability evidence.
- Parent HTML/reader validation also initially exposed stale proof-ledger counts
  and missing explicit OBL-061 boundary text. The parent corrected
  `model-checked-bounded = 3`, `runtime-monitored = 5`, added explicit
  `OBL-061 runtime-monitored (finite in-process dispatch only)` wording to
  HTML/Documentation, changed the Documentation OBL range to `029..061`, and
  reran the HTML unittest successfully.
- At report authoring, the worktree also held Canon/status/reader closeout
  diffs for SYS-4; this report writer touched only this report file.
- Snapshot timestamp came from `date`: `2026-08-27 21:27:25 JST`.

## Documents consulted

- Canon hierarchy: `mirrorea_canon/README.md`, `MAP.md`, `NORTH-STAR.md`,
  `DESIGN-CONSTITUTION.md`, `CANON.md`, and `AGENTS.md`.
- Authority/lifecycle: ADR-0025--0030, Canon plan/00--02, and Plan 249.
- Direct semantics and runtime boundaries: architecture/03--04, theory/11/13,
  spec/12, and spec/13.
- Direct implementation evidence: `crates/mir-runtime/src/sys4_dispatch.rs`,
  `crates/mir-runtime/src/sys4_dispatch_tests.rs`, SYS-2 backend tests, M8/M9
  runtime modules, and SYS-4 Surface fixtures at the pinned cuts.
- Current derived views: README, `Documentation.md`, `docs/project-status.md`,
  `progress.md`, `tasks.md`, `samples_progress.md`, and
  `docs/mirrorea-project-overview.html`.
- Report 2595 was read as the direct previous milestone report/style source;
  `docs/reports/` was not read in bulk.

## Actions taken

1. Bootstrapped a crate-private `LocalFabric` from generated SYS-3 artifacts
   and complete sealed M9 admission rather than from source text, fixture
   names, manual routes, or expected JSON.
2. Partitioned the runtime into independently keyed locus runtimes with local
   stores, mailboxes/endpoints, local traces, and generated communication
   records.
3. Executed owner RMW at the owner partition and designated-result delivery at
   the named consumer partition without requester-private reads or direct
   cross-locus store mutation.
4. Implemented bounded carrier-side idempotent return: the first exact
   designated-result delivery performs exactly one M8 semantic consume, while
   an exact same-consumer retry returns the stored typed decision with no new
   semantic consume row.
5. Added typed fail-closed dispatch diagnostics for route, membership,
   capability/witness, wrong-target, duplicate/stale receipt, split-frame,
   revoked-layer, patch-frontier, and provenance/cache corruption classes.
6. Made OW1 observer snapshot failure explicit as
   `ObserverSnapshotUnavailable`, distinct from genuine absence and without
   stale cached success substitution.
7. Added deterministic replay evidence over generated endpoints.
8. Added ST whole-fabric local cut/restore covering per-locus M8 cuts,
   endpoint send/receive records, pending carriers, receipts, counters,
   publication/import/consume/cache state, and patch lifecycle rows.
9. Added bounded ST checked patch activation for quiescent designated-only
   compatible patches, with rejection for stale frontier and forged lifecycle.
10. Kept OW1 cut/patch as typed `BackendIneligible`; no worker-cut protocol was
    claimed.
11. Updated Canon/status/reader documents through the delegated closeout lanes:
    PROPOSAL-033, ADR-0030, spec/13, Plan 249, phase/status snapshots, and the
    HTML reader view. This report records those updates but did not author them.
12. Recorded OBL-061 as `runtime-monitored` finite correspondence only and left
    official lifecycle state unchanged.

## Files changed

Accepted SYS-4 implementation/test/fixture delta from
`3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9` through
`22196f93b0112b8fd2987ec078021c8865b71651`:

- `crates/mir-ast/tests/fixtures/surface-v0/sys4_combined_owner_designated_owner_rmw_changed_with_auth.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys4_designated_consume_with_auth.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys4_designated_consume_with_auth_plus_two.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys4_ow1_endpoint_crossing.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys4_two_owner_four_locus_with_auth.mir`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/m8_runtime_designated_value.rs`
- `crates/mir-runtime/src/m8_runtime_local_cut.rs`
- `crates/mir-runtime/src/m8_runtime_owner_queue.rs`
- `crates/mir-runtime/src/m9_auth_verification.rs`
- `crates/mir-runtime/src/sys2_execution_backend.rs`
- `crates/mir-runtime/src/sys2_execution_backend_tests.rs`
- `crates/mir-runtime/src/sys3_projection/lowering.rs`
- `crates/mir-runtime/src/sys3_projection/mod.rs`
- `crates/mir-runtime/src/sys3_projection/model.rs`
- `crates/mir-runtime/src/sys4_dispatch.rs`
- `crates/mir-runtime/src/sys4_dispatch_tests.rs`

SYS-4 Canon/status/reader closeout delta present at report authoring:

- `mirrorea_canon/meta/proposals/PROPOSAL-033-sys4-in-process-generated-dispatch.md`
- `mirrorea_canon/adr/ADR-0030.md`
- `mirrorea_canon/spec/13-sys4-in-process-generated-dispatch.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/architecture/03-toolchain.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/spec/12-sys3-per-locus-projection.md`
- `mirrorea_canon/spec/README.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/theory/13-evaluation-materialization.md`
- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `docs/mirrorea-project-overview.html`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `plan/00-index.md`
- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `scripts/tests/test_mirrorea_project_overview_html.py`

Close-validation hygiene delta present at report authoring:

- `crates/mirrorea-cli/src/main.rs` changed only a bool literal assertion style
  required by workspace `clippy -D warnings`; it is not SYS-4 semantic evidence.

This report writer added only:

- `docs/reports/2596-mirrorea-i2-systems-foundation-sys4-in-process-generated-dispatch.md`

## Commands run

Accepted SYS-4 implementation validation executed by parent implementation/test
lanes:

```text
cargo test -p mir-runtime --lib sys4_dispatch_tests
cargo test -p mir-runtime --lib
cargo test -p mir-runtime --test m10_source_execution
cargo test -p mir-runtime --test m10_cli
cargo test -p mir-runtime --test m10_conformance
cargo fmt --all -- --check
cargo clippy -p mir-runtime --all-targets -- -D warnings
git diff --check
```

Close validation reported by the parent after the SYS-4 implementation cut:

```text
cargo fmt --all -- --check
cargo test --workspace --all-targets --no-fail-fast
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

The first workspace Clippy run failed only on the non-SYS4 bool literal
assertion in `crates/mirrorea-cli/src/main.rs:4506`; the parent applied the
minimal hygiene fix and reran the final format/Clippy pair successfully.

Report-author inspection commands:

```text
git status --short --branch
git rev-parse HEAD origin/main
git log --oneline -8
git diff --name-status 3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9..22196f93b0112b8fd2987ec078021c8865b71651
git diff --name-status
TZ=Asia/Tokyo date '+%Y-%m-%d %H:%M:%S %Z'
git diff --check
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html -v
cd mirrorea_canon && python3 meta/build-index.py --check
make docs
```

## Evidence / outputs / test results

- SYS-4 focused tests: **99/99 pass**.
- `mir-runtime` library tests: **179/179 pass**.
- M10 source regression: **2/2 pass**.
- M10 CLI regression: **4/4 pass**.
- M10 conformance regression: **67/67 pass**.
- `cargo fmt --all -- --check`: pass at accepted implementation validation and
  pass again after close-validation hygiene.
- Scoped `mir-runtime` all-target Clippy with `-D warnings`: pass at accepted
  implementation validation.
- Full workspace tests: `cargo test --workspace --all-targets --no-fail-fast`
  exit 0 after SYS-4 cut.
- Full workspace Clippy: first run failed only on the non-SYS4 bool literal
  assertion; after the minimal hygiene fix, `cargo fmt --all -- --check &&
  cargo clippy --workspace --all-targets -- -D warnings` exited 0.
- Canon index check: **182 files**, pass.
- Canon hierarchy check: **799/799**, pass.
- `make docs`: pass.
- HTML reader regression: initial RED after closeout sync exposed stale
  proof-ledger counts / missing OBL-061 boundary text; after parent corrections,
  `python3 -m unittest scripts.tests.test_mirrorea_project_overview_html -v`
  ran **8/8 pass**.
- `git diff --check`: pass at accepted implementation validation and pass again
  after report authoring.
- Independent semantic/code review after P1/P2 fixes: **ACCEPT**.
- Independent planner close review first returned **REJECT** on this report's
  command/inventory accounting: it named the nonexistent `m10_cli_acceptance`
  target instead of `m10_cli`, omitted the HTML regression test from Files
  changed, and had not yet recorded the final `make docs` run or review state.
  Those report-only defects were corrected. Narrow re-review returned
  **ACCEPT**, with no remaining P0/P1/P2.

OBL-061 is classified only as finite `runtime-monitored` evidence for the
accepted internal dispatch correspondence. It is not Lean-proved, not a general
scheduler/memory-model theorem, not arbitrary relation-DAG proof, not public
carrier/API/ABI/wire compatibility, and not real transport evidence.

## What changed in understanding

SYS-4 confirms that the accepted SYS-3 projection is executable without turning
the conformance facade into the runtime architecture. The runtime can carry a
source/Core/artifact/edge/occurrence chain across generated endpoints while
preserving owner-local mutation, designated publication/consume identity,
authority revalidation, typed failure, and observer-safe redaction.

The designated-result retry rule must live at the carrier/runtime boundary. It
is distinct from the legacy M8 same-delivery `AlreadyConsumed` regression and
does not authorize hidden retry, exactly-once transport, implicit callbacks, or
multi-consumer semantics.

The ST local cut/restore and bounded patch evidence are meaningful only inside
the accepted finite profile. OW1 requires a separate worker-cut protocol before
it can claim save/restore or patch support.

## Open questions

- SYS-5 must compose the accepted dispatch runtime into one four-locus headless
  toy and joined causal view without manual evidence joins or direct store
  bypass.
- OW1 cut/restore and patch remain typed `BackendIneligible` until a worker-cut
  protocol is specified and implemented.
- Public CLI/API/ABI/wire/carrier formats remain unfrozen.
- Real transport, durable distributed save/load, browser/View product,
  arbitrary relation-DAG theorem, arbitrary scheduler fairness, and lock-free
  runtime remain intentionally deferred.
- Broad PHASE-I1 exit and official I2 lifecycle entry/exit still require
  separate actual criteria and acceptance evidence.

## Suggested next prompt

Continue without user input to SYS-5:

```text
Implement the minimal typed devtools and local virtual-space vertical slice on
top of accepted SYS-4 dispatch evidence. Keep the toy source-driven,
four-locus, observer-safe, and non-public; do not claim browser/View,
production, public ABI/wire, or official I2 lifecycle acceptance.
```

## Plan update status

更新済み: `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md` was
updated by the delegated planner/status lane to mark SYS-4 completed, record
accepted cut `22196f93b0112b8fd2987ec078021c8865b71651`, set SYS-5 as active,
set SYS-6 as next, and state the current direct blocker. `plan/00-index.md` was
synchronized to keep Plan 249 as the sole current roadmap and Plan 247 as
historical M10 baseline.

## Documentation.md update status

更新済み: `Documentation.md` was updated by the delegated reader/status lane
with done / in-progress / blocked state, SYS-4 decisions taken, open risks, and
verification status. It records SYS-0--SYS-4 completed, SYS-5 active, SYS-6
next, lifecycle non-claims, the corrected OBL range `029..061`, and the
finite-only accepted SYS-4 / OBL-061 boundary.

## docs/project-status.md update status

更新済み: `docs/project-status.md` was updated by the delegated status lane as
the current short status snapshot. It records the accepted SYS-4 cut, SYS-5
active blocker, SYS-6 next, unchanged theory T1/lifecycle state, and evidence
limitations.

## progress.md update status

更新済み: `progress.md` was updated by the delegated status lane as the concise
progress snapshot and recent log. It records SYS-4 closed, SYS-5 active, the
accepted runtime dispatch evidence, residual OW1 cut/patch ineligibility, and
unchanged public/lifecycle non-claims.

## tasks.md update status

更新済み: `tasks.md` was updated by the delegated status lane as the current
task map. It sets the next autonomous package to SYS-5 typed devtools / local
toy vertical slice and leaves owner-reserved/public transport/lifecycle
decisions out of the active queue.

## samples_progress.md update status

更新済み: `samples_progress.md` was updated by the delegated reader/status lane
to reflect that SYS-4 supplies internal runtime evidence, while the user-visible
local toy sample remains the SYS-5 active blocker. It does not promote helper
evidence or expected-output fixtures to final workflow readiness.

## Reviewer findings and follow-up

Independent implementation and semantic reviewers raised and then accepted fixes
for the following issues:

- raw observer/debug leak and stale view risk;
- observer failure conflated with semantic absence;
- panic or wrong-locus handling at endpoint boundaries;
- exact causality/provenance gaps;
- patch rejection temporal ordering;
- M9 identity, time-of-check/time-of-use, and shared-floor concerns;
- non-designated semantics and OW1 diagnostics;
- cut consistency, publisher rebase, counters, and provenance normalization;
- stale publisher transitions; and
- forged lineage success counterexample.

Final implementation semantic/code review returned **ACCEPT** after these fixes.
Reader/HTML validation then raised stale proof-ledger counts and missing
explicit OBL-061 boundary wording; the parent corrected the reader/status docs
and the HTML regression reran **8/8 pass**.
Independent planner close review returned **REJECT** on two report-accounting
P1s: a nonexistent M10 CLI target name and an omitted HTML regression test file.
This correction records the actual `m10_cli` target, complete dirty inventory,
final `make docs`, and review state. Narrow re-review returned **ACCEPT**, with
no remaining P0/P1/P2; no Canon, runtime, lifecycle, or evidence-class defect
was found.

## Skipped validations and reasons

- Lean was not rerun for SYS-4 closeout because SYS-4 records OBL-061 as
  `runtime-monitored`; no new Lean-proved theorem is claimed.
- Model checking was not extended beyond the accepted finite SYS-2/SYS-4 runtime
  evidence; arbitrary scheduler/fairness/memory-model claims are deferred.
- Real transport, socket, WAN, public wire, durable distributed save/load,
  production deployment, browser renderer, and final View/FFI validation were
  intentionally out of scope.
- OW1 cut/restore and checked patch validation were not run because the accepted
  contract returns typed `BackendIneligible` for those paths.
- Oracle advisory attempt produced no usable output: browser-backed sessions
  `sys4-real-fabric-falsificat-20260827t0`, `sys4-real-fabric-falsificat-current`,
  and `sys4-real-fabric-falsificat-current-2` did not receive the prompt and
  ended at browser/login/picker timeout. No advisory content was mirrored and no
  repo edits came from Oracle.

## Commit / push status

Implementation cut `22196f93b0112b8fd2987ec078021c8865b71651` was committed,
pushed, and remote parity-confirmed before closeout documentation began.

At this report authoring, the Canon/status/reader/report closeout diff and the
parent's non-SYS4 Clippy hygiene source diff were still pending parent
integration. This report does not claim that the closeout documentation commit
has been made or pushed.

## Sub-agent session close status

Delegated lanes active or completed around SYS-4 closeout:

- Planner/status writer: updated Plan 249, plan/00, project status, progress,
  and tasks snapshots.
- Canon closeout writer: updated PROPOSAL-033, ADR-0030, spec/13, Canon index,
  changelog, architecture/spec/theory references, and lifecycle non-claims.
- Reader/status writer: updated README, Documentation, samples progress, and
  HTML overview.
- HTML test author: synchronized lifecycle/evidence assertions, producing a
  useful OBL-061 RED; the parent corrected reader proof counts/boundary wording
  and reran the test 8/8 green.
- Report writer: added this report only; the parent corrected the final
  command/inventory/review accounting after planner review.
- Independent final planner: returned report-accounting P1s without finding a
  Canon/lifecycle/runtime-evidence defect; after the report-only correction,
  narrow re-review returned **ACCEPT** with no remaining P0/P1/P2.
- Oracle advisory: unavailable, with no prompt result and no repo edits.

No sub-agent result is treated as evidence without parent-side validation.
