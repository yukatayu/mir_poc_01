# Report 2598 — Mirrorea I2 Systems Foundation SYS-6 I2 conformance closeout

Identifier: `REPORT-2598`

Date: 2026-08-28 JST

## Objective

Close SYS-6 by accepting one finite, source-first Mirrorea I2 conformance
profile over the already accepted SYS-2--SYS-5 implementation layers, classify
every assurance claim accurately, apply official I2 entry and then I2 exit
without weakening pre-existing criteria, and leave SYS-7 as the only active
milestone. The direct user-visible result is a reproducible `mir conform-i2`
command whose typed observer-safe report joins one ordinary source through
checked Core, generated per-locus artifacts and communication, actual in-
process dispatch, selected backend evidence, runtime occurrences, devtools,
save, and patch evidence.

Direct consumer: SYS-7's inactive I3 goal and entry contract.

Blocker reduced: I2 lacked one bounded source-first acceptance profile and one
authorized lifecycle decision over the actual SYS-3--SYS-5 fabric evidence.

Acceptance use: ADR-0032 evaluates the fixed 22-row profile and records official
I2 entry followed by I2 exit; SYS-7 can now describe, but not start, a future
real-transport program over the accepted internal boundary.

## Scope and assumptions

- Normative scope is the ADR-0026 SYS-6 milestone only. SYS-7 remains a
  separate active goal and is not implemented here.
- The exact accepted implementation/evidence cut is
  `5429712de89a7e41c46cfd7fb4a39c4a492864c4`.
- The primary four-locus local toy executes its complete workflow in ST. OW1
  evidence comes from the separate ordinary exactly-one-worker eligible
  source; the four-locus whole-workflow `BackendIneligible` residual remains
  explicit.
- The report producer and verifier are non-authorizing. Their lifecycle bits
  remain false; only ADR-0032 applies Canon lifecycle transitions.
- Twenty-one profile rows are `runtime-monitored`. The no-source-free-authority
  row reuses OBL-058 bounded-model evidence and is `model-checked-bounded`.
  Aggregate OBL-063 remains `runtime-monitored`.
- Broad PHASE-I1 remains unaccepted; theory remains T1; architecture/04 stays
  L2-working with OPEN-026, OPEN-027, and full internal-carrier freeze open.
- No public CLI/API/ABI/JSON/artifact/wire, real transport, I3 activation,
  production, durable distributed persistence, browser/View product,
  four-locus whole-workflow OW1, or general theorem is claimed.

Primary falsifier: a profile candidate passes despite a missing or extra
generated edge, moved owner, manual interface, direct remote store,
source-free authority/state, unexecuted evidence, wrong diagnostic, selected
ST/OW divergence, stale cut/patch mutation, relation/designated/fallback drift,
observer leak, or lifecycle overclaim.

Stop condition: close once the exact 22 rows consume executed positive and
falsifier evidence with property-specific provenance, all focused and
workspace regressions pass, an independent reviewer returns ACCEPT, official
I2 entry/exit can be applied without changing broad I1 or public boundaries,
and SYS-7 has a direct accepted consumer boundary.

## Start state / dirty state

The documentation closeout began on branch `main` with local HEAD and
`origin/main` both at the already committed and pushed production cut:

```text
5429712de89a7e41c46cfd7fb4a39c4a492864c4
feat: add I2 conformance profile
```

The worktree was clean at that cut. Existing user changes were not present and
no production source or test file was changed by this report/Canon lane. Other
agents were active in the shared repository; this writer did not revert or
overwrite their production work.

## Documents consulted

Canon was read first, following the repository hierarchy:

- `mirrorea_canon/README.md`, `MAP.md`, `NORTH-STAR.md`, and
  `DESIGN-CONSTITUTION.md`;
- `mirrorea_canon/plan/00-gates.md` and `plan/01-phases.md`;
- ADR-0025, ADR-0026, and the direct SYS-1--SYS-5 ADR chain through ADR-0031;
- architecture/03 and architecture/04;
- theory/11 proof ledger and direct SYS-3--SYS-5 specifications;
- the current Plan 249 SYS-6 Goal Statement and direct status pointers.

LAB evidence was then read only where directly required: `README.md`,
`Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
`samples_progress.md`, `plan/00-index.md`, Plan 249, the canonical SYS-5 sample
and walkthrough, and Report 2597. `docs/reports/` was not bulk-read.

## Actions taken

1. Added PROPOSAL-035, ADR-0032, and spec/15 for the finite producer/verifier,
   exact 22-row inventory, evidence/provenance requirements, typed failures,
   observer-safe output, and non-authorizing lifecycle-report boundary.
2. Added OBL-063 to the proof ledger as aggregate `runtime-monitored` evidence
   while retaining the row-local OBL-058 `model-checked-bounded` classification.
3. Updated architecture/03 and architecture/04 to record the one-way SYS-6
   evidence dependency without allowing conformance orchestration to control
   runtime architecture.
4. Applied official I2 entry and then I2 exit in plan/01. Broad PHASE-I1 and
   theory T1 remain unchanged. OPEN-032 now triggers only at a future owner-
   authorized I3 decision, not at I2 exit, and remains unresolved.
5. Regenerated Canon navigation/index material and synchronized its README,
   MAP, ADR/spec subindices, and CHANGELOG.
6. Closed SYS-6 and made SYS-7 the sole active milestone in Plan 249 and the
   current LAB snapshots.
7. Added the canonical SYS-6 sample README and extended root/sample/hands-on
   reader paths with the exact `conform-i2` workflow and lifecycle/non-claim
   explanation.
8. Updated the static project overview so readers can distinguish official I2
   exit from broad I1 residuals, I3 inactivity, and public/product non-claims.
9. A parent-managed test lane updated the overview regression markers from the
   closed SYS-5/active SYS-6 snapshot to closed SYS-6/active SYS-7, including
   ADR-0032, OBL-063, official I2 exit, and I3-inactive assertions.

Normative change: PROPOSAL-035 / ADR-0032 / spec/15 accept the finite SYS-6
profile and apply official I2 entry then I2 exit. No other lifecycle or public
contract change is implied.

## Files changed

Canon:

- `mirrorea_canon/meta/proposals/PROPOSAL-035-sys6-i2-conformance.md` (new)
- `mirrorea_canon/adr/ADR-0032.md` (new)
- `mirrorea_canon/spec/15-sys6-i2-conformance.md` (new)
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json` (regenerated)
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/spec/README.md`
- `mirrorea_canon/architecture/03-toolchain.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/plan/01-phases.md`

LAB roadmap/status/readers:

- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `plan/00-index.md`
- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/clean-near-end/README.md`
- `samples/clean-near-end/mirrorea-i2-conformance/README.md` (new)
- `docs/hands_on/README.md`
- `docs/hands_on/mirrorea_i2_local_toy_01.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py` (parent-managed test
  lane; current lifecycle/reader assertions only)
- this report.

No production Rust, Lean, generated runtime artifact, or executable script was
changed by this closeout-writing lane. This writer did not edit the docs-only
HTML regression test listed above.

## Commands run

Focused implementation and regression validation reported by the parent
integration lane for the accepted production cut:

```text
cargo test -p mir-runtime --lib sys6_i2_conformance_tests
cargo test -p mir-runtime --test sys6_i2_cli
cargo test -p mir-runtime --lib sys2_
cargo test -p mir-runtime --lib sys3_projection_tests
cargo test -p mir-runtime --lib sys4_
cargo test -p mir-runtime --lib sys5_
cargo test -p mir-runtime --test m10_conformance
cargo test -p mir-runtime --test m10_cli
cargo test --workspace --all-targets --no-fail-fast
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
git diff --check
```

Canonical user path inspected for this closeout:

```text
cargo run -q -p mir-runtime --bin mir -- conform-i2 \
  samples/clean-near-end/mirrorea-i2-local-toy/main.mir \
  --selected-ow1-source samples/clean-near-end/mirrorea-i2-conformance/ow1-selected-owner-designated.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/designated-plus-two.mir \
  --patch samples/clean-near-end/mirrorea-i2-local-toy/patches/owner-rmw-change.mir \
  --format json
```

Closeout documentation validation:

```text
cd mirrorea_canon && python3 meta/build-index.py
cd mirrorea_canon && python3 meta/build-index.py --check
python3 -m unittest scripts.tests.test_build_index -v
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html -v
make docs
git diff --check
```

The first grouped validation invocation kept its working directory inside
`mirrorea_canon/`: the Canon index check passed, then repo-root imports/paths
and `make docs` correctly failed to resolve. No document assertion failed in
that invocation. The same commands were rerun from their required directories
as shown above; the later content-validation failures precisely identified and
led to restoration of required Canon notices, report status declarations,
snapshot headings/source references, and project-status source references.
Fresh final runs are the acceptance evidence below.

Inspection included `git status --short --branch`, `git rev-parse HEAD`,
`git rev-parse origin/main`, focused `rg` stale-state scans, test inventory
listing, Canon file-size checks, and a final diff review.

## Evidence / outputs / test results

- SYS-6 library suite: **25/25 pass**.
- SYS-6 CLI suite: **8/8 pass**.
- SYS-2 selected backend/model suite: **28/28 pass**.
- SYS-3 projection suite: **28/28 pass**.
- SYS-4 dispatch suite: **104/104 pass**.
- SYS-5 local-toy/devtools suite: **62/62 pass**.
- M10 conformance and CLI regressions: **67/67 and 4/4 pass**.
- Full workspace all-target tests: pass.
- Full workspace all-target warnings-denied Clippy: pass.
- `cargo fmt --all -- --check`: pass.
- Production-cut `git diff --check`: pass.
- Canonical `conform-i2` report: `status = accepted`, exactly 22 rows, all 22
  pass; lifecycle/public flags remain false; output explicitly lists broad I1,
  I2 lifecycle self-authorization, I3, real transport, public ABI/wire,
  durable distributed save/load, general metatheory, arbitrary fairness/DAG,
  and four-locus whole-workflow OW1 as non-claims.
- Every accepted row contains executed positive and representative falsifier
  evidence plus property-appropriate provenance; observer output is redacted
  and content-bound identities are independent of host directory paths.
- Final independent implementation/assurance review reported **ACCEPT** with
  no blocking finding.
- Canon index regeneration/check: **188 files**, pass.
- Canon build-index unit tests: **5/5 pass**.
- Canon/source hierarchy: **799/799**, zero missing, pass.
- Documentation validation: pass; **1,752 numbered reports** found.
- HTML reader regression: **8/8 pass** after the parent-managed stale-marker
  test update.
- Aggregate `make docs`: pass, including agent config, 188-file Canon index,
  799/799 hierarchy, and documentation validation.
- Fresh final `git diff --check`: pass.

These are finite executable/model/runtime facts, not general proofs. The exact
classification is in OBL-063 and spec/15.

## What changed in understanding

The I2 acceptance boundary does not need a second facade that reinterprets
source or invents expected outcomes. A small producer can invoke the accepted
SYS-2--SYS-5 layers and emit typed inventories; a separate fixed verifier can
then check cross-layer joins, negative controls, and observer safety without
becoming a dependency of the semantic runtime or projection kernels.

Lifecycle evidence and lifecycle authority must remain distinct. The command
correctly reports all lifecycle bits as false because an executable evidence
producer cannot change Canon. ADR-0032 can nevertheless accept official I2
entry and exit after evaluating that evidence. Conversely, broad I1 remains
open because architecture/04's wider carrier criteria have not changed.

The accepted correspondence is also narrower than a superficial “ST and OW
both work” claim. The complete four-locus toy remains ST; OW1 correspondence is
for a separate ordinary exactly-one-worker-eligible source. Retaining the typed
four-locus `BackendIneligible` result is part of the assurance boundary.

## Open questions

- OPEN-026 and OPEN-027 plus the full internal-carrier freeze still block broad
  PHASE-I1 acceptance.
- OPEN-032 transport choice remains unresolved. Its next permissible trigger
  is a future owner-authorized I3 program/entry decision after SYS-7 records an
  inactive entry contract.
- Public CLI/API/ABI/JSON/artifact/wire compatibility remains unresolved and
  is not a hidden requirement of I2 acceptance.
- General projection, communication-completeness, concurrency, data-race,
  authority, relation-DAG, persistence, patch, fairness, and noninterference
  theorems remain intentionally deferred until they have direct consumers.

None of these residuals blocks SYS-7's documentation-only entry contract.

## Suggested next prompt

Continue autonomously with SYS-7 only: write one inactive future I3 parent goal
and entry contract over the accepted I2 per-locus artifacts and communication
plan. Compare at most two transport candidates, keep transport non-authority,
separate the internal carrier from any future public wire, include typed
disconnect/reconnect/duplicate/reorder and ordering-refinement requirements,
and do not select a transport, implement it, freeze compatibility, activate
I3, or deploy anything.

## Plan update status

更新済み: Plan 249 now records SYS-6 as completed at the exact accepted cut,
official I2 entry/exit, broad I1/T1 residuals, and SYS-7 as the sole active
goal. Its SYS-6 Goal Statement contains the user-visible outcome, invariants,
direct consumer, primary falsifier, evidence, stop/reopen conditions, and
non-claims. `plan/00-index.md` mirrors the current control state. No new
numbered plan or WRK was created.

## Documentation.md update status

更新済み: `Documentation.md` now distinguishes the completed SYS-6 finite I2 profile and official I2
exit from broad I1, I3, public compatibility, real transport, production, and
general-theory residuals. Reader commands point to the canonical source-bound
`conform-i2` path.

## docs/project-status.md update status

更新済み: `docs/project-status.md` now records the concise current status: SYS-0--SYS-6 completed, SYS-7 sole
active, official I2 entry/exit accepted by ADR-0032, theory T1 and broad I1
unchanged, exact accepted cut and validation summary, and I3 inactive.

## progress.md update status

更新済み: `progress.md` is updated as a current snapshot rather than an append-only history. Logical,
user-facing, and implementation/operations axes now reflect the finite I2
acceptance boundary; macro-phase and feature rows retain separable subsystem
status and explicit self-driven/deferred/owner-decision columns. A timestamped
2026-08-28 JST closeout log entry was added.

## tasks.md update status

更新済み: `tasks.md` is rewritten as the current task map. SYS-7 is the only self-driven package.
Future I3 authorization, transport choice, public contract freeze, and
production/publication are separated as owner-reserved decisions; broad I1
and theory/general-proof residuals are research or later-consumer work rather
than hidden SYS-7 tasks.

## samples_progress.md update status

更新済み: `samples_progress.md` is updated in place with the canonical SYS-6 conformance sample, exact validation
command, 22-row finite evidence, test counts, evidence classification, and
non-claims. `samples/README.md` and `samples/clean-near-end/README.md` were
updated in the same task. `scripts/README.md` update was unnecessary because no
script or script taxonomy changed.

## Reviewer findings and follow-up

- A Canon-first pre-edit planner review concluded that broad I1 must remain
  unaccepted, official I2 entry/exit may be accepted only after complete SYS-6
  evidence, the producer/verifier must remain non-authorizing, and OPEN-032
  must move to a future owner-authorized I3 decision rather than I2 exit. Those
  boundaries are reflected in PROPOSAL-035, ADR-0032, spec/15, and plan/01.
- The implementation/assurance lane's final independent review reported
  **ACCEPT** with no blocking finding after the 25+8 SYS-6 tests, cross-layer
  regressions, full workspace validation, and actual `conform-i2` inspection.
- The final independent closeout semantic review reported **ACCEPT**. It found
  no semantic, authority, lifecycle, evidence-classification, or owner-reserved
  boundary defect in the integrated SYS-6 decision package.
- The parent-managed HTML test lane replaced stale SYS-5/SYS-6 markers with
  SYS-6/SYS-7, ADR-0032/OBL-063, official I2 exit, and I3-inactive assertions;
  its fresh regression result is **8/8 pass**.
- The final closeout planner review found two docs-only drift items in Plan
  249: its retained alignment introduction still said SYS-3--SYS-5 were closed
  with SYS-6 as the current blocker, and the SYS-6 user-visible outcome could
  be read as claiming that runtime JSON reports the accepted Git implementation
  cut. Both are resolved: the matrix now says SYS-3--SYS-6 closed and SYS-7
  sole-active/no-implementation, while the outcome names only a bounded source
  fingerprint and safe opaque evidence references; ADR-0032, Report 2598, and
  Canon acceptance metadata pin the exact Git cut. No implementation,
  conformance row, lifecycle decision, or evidence class changed.
- The post-fix independent planner re-review returned **ACCEPT with no P0/P1**.
  It confirmed that both wording drifts are resolved, SYS-7 is sole-active and
  implementation-inactive, runtime JSON makes no Git-cut claim, and no further
  substantive closeout review is required. Commit/push remains a parent-owned
  mechanical integration step.

Reopen SYS-6 for any accepted missing/extra edge, moved owner, manual route,
direct remote mutation, source-free mint, unbound/fabricated evidence, wrong
diagnostic, selected-backend semantic divergence, stale cut/patch mutation,
observer leak, lower-layer dependency on the conformance aggregator, M10
regression, or counterexample to the unchanged I2 lifecycle criteria.

## Skipped validations and reasons

- Lean was not run for SYS-6 because no Lean file, theorem statement, proof, or
  proof dependency changed. OBL-063 deliberately claims only aggregate finite
  `runtime-monitored` evidence; the row-local bounded model reuses OBL-058.
- No new general model check was run. SYS-6 consumes the accepted finite SYS-2
  bounded model and runtime controls; it does not claim arbitrary scheduling,
  hardware memory, or general authority proof.
- Four-locus whole-workflow OW1 was not run or claimed because that topology is
  ineligible under the accepted OW1 contract. The typed nonmutating residual is
  part of the conformance evidence.
- Real transport, multi-process/network faults, durable distributed storage,
  browser rendering, public compatibility, production, performance, and I3
  execution were intentionally out of scope.
- `scripts/README.md` was not updated because no script or taxonomy changed.

## Commit / push status

The exact production implementation/evidence commit
`5429712de89a7e41c46cfd7fb4a39c4a492864c4` was committed, pushed, and locally
parity-confirmed before this closeout-writing lane began:

```text
HEAD        5429712de89a7e41c46cfd7fb4a39c4a492864c4
origin/main 5429712de89a7e41c46cfd7fb4a39c4a492864c4
```

At this report snapshot, the Canon, roadmap, status, reader, sample-index, and
Report 2598 changes are an uncommitted/unpushed same-milestone closeout diff.
This writer was explicitly delegated not to commit or push. The parent will
perform final diff review, create the milestone documentation commit with
`--no-gpg-sign`, push it, verify remote parity, and report the resulting hash.
No future commit hash, push, clean-worktree state, or remote parity is invented
inside the commit that contains this report.

## Sub-agent session close status

- SYS-6 implementation/test lanes: completed under parent orchestration; their
  accepted result is the pushed production cut above.
- Final independent implementation/assurance reviewer: completed with
  **ACCEPT** and no blocking finding, as reported by the parent integration
  lane.
- Final independent closeout semantic reviewer: completed with **ACCEPT** and
  no blocking semantic/authority/lifecycle finding.
- Final closeout planner reviewer: completed its initial review, raised two
  docs-only Plan 249 wording findings, then completed a post-fix substantive
  re-review with **ACCEPT, no P0/P1**. Both findings are resolved.
- HTML reader test-update lane: completed the current-boundary assertion
  update and reported **8/8 pass**; it did not own Canon or report text.
- SYS-6 Canon/status/reader/report writer (this lane): completed the requested
  scoped diff and local documentation validation; no production edit, commit,
  or push ownership.
- No child sub-agent was spawned by this writer. Other parent-managed agents
  remain the parent's responsibility; this report does not declare their
  sessions closed without evidence.
