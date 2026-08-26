# Report 2593 — Mirrorea I2 Systems Foundation SYS-1: runtime kernel and internal carrier

- Date: 2026-08-26 JST
- Milestone: SYS-1
- Status: **completed / closed** at accepted source/evidence cut
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`; SYS-2 active, SYS-3 next
- Evidence class: `runtime-monitored` only
- Lifecycle: theory T1; broad PHASE-I1 and official I2 entry/exit remain
  unaccepted

## Objective

Separate the semantic runtime kernel used by ordinary source execution and
generic checked owner events from M10 conformance/release/profile/CLI
orchestration, and fix the smallest typed I2-internal owner-request and
designated-remote-input lifecycle that SYS-2 and SYS-3 can consume without
freezing a public contract.

## Scope and assumptions

The selected production claim is deliberately narrow. A crate-private
`SemanticRuntimeKernel` consumes one exact checked program and sealed M9
execution seam, owns the admitted M8 runtime, and supports its extraction.
Ordinary `run_source` and generic checked `OwnerEvent` use it. Specialized
historical M10 SCN-04, SCN-09, SCN-10, and route-patch runners remain their
preserved regression-only paths and are not SYS-1 kernel evidence.

The bounded carrier covers owner request → serve → reply → receive/receipt and
designated remote-input request → source-owner serve → reply → receive/receipt
→ evaluator consume. It does not create a generic provider registry. Receipt
does not transfer authority; queue position is not identity; transport does
not mint authority; retry/exactly-once and public API/ABI/wire remain open.

## Start state / dirty state

- Branch: `main`.
- Initial `HEAD == origin/main`:
  `94e3707c7bc98d4a0764c51f13a12b1dae1968c6`
  (`feat(runtime): add SYS-1 semantic kernel boundary`).
- Worktree was clean at this writer's start; no uncommitted user change was
  present.
- The source cut changes only the delegated runtime/test implementation
  package. This report/Canon/status closeout is its successor docs change.
- Resource recheck during closeout: root filesystem 188 GiB total, 58 GiB
  available; memory 15 GiB total, about 12 GiB available; no heavy artifact
  was added by the docs writer.

## Documents consulted

- Canon entry and direction: `mirrorea_canon/README.md`, `NORTH-STAR.md`,
  `DESIGN-CONSTITUTION.md`, `MAP.md`.
- Authority/lifecycle: ADR-0025, ADR-0026, Canon plan/00-gates,
  plan/01-phases, architecture/04-runtime-carriers.
- Semantic/runtime boundary: theory/13-evaluation-materialization and
  spec/05-runtime-semantics.
- Sole current execution control: LAB Plan 249.
- Source evidence: `semantic_runtime_kernel.rs`, M9 execution seam changes,
  `m10_reference_system.rs`, the crate module boundary, SYS-1 focused tests,
  and the ordinary-source M10 integration tests at cut `94e3707c...`.
- Current reader/status surfaces: root README, `Documentation.md`,
  `docs/project-status.md`, primary HTML reader and its focused test,
  `progress.md`, `tasks.md`, and `samples_progress.md`.
- Only directly relevant Report 2592 was used for the predecessor state;
  `docs/reports/` was not read in bulk.

## Actions taken

1. Verified the source cut's dependency direction: checked Core + sealed M9
   seam → crate-private kernel → owned/extractable M8 runtime, with no M10
   profile/verifier/release/CLI import into the kernel.
2. Distinguished ordinary source/generic checked-owner production evidence
   from specialized legacy M10 scenario runners.
3. Recorded PROPOSAL-030 and ADR-0027 through the normal Canon process.
4. Refined architecture/04, theory/13, and spec/05 with the bounded internal
   lifecycle, field/invariant requirements, fail-closed pre-admission, and
   non-effects.
5. Closed OPEN-030 only for the I2-internal owner/designated-input contract;
   kept architecture/04 L2-working and broad I1 unaccepted because OPEN-026,
   OPEN-027, and full carrier freeze remain.
6. Recorded the immutable-M9-snapshot revoke-after-enqueue/serve visibility
   gap as the direct SYS-2 blocker/reopen boundary.
7. Moved the sole roadmap/current snapshots to SYS-1 completed, SYS-2 active,
   SYS-3 next without changing theory T1 or broad lifecycle state.
8. Updated the primary HTML regression expectation first, observed the
   expected red failure against old SYS-1 pointers, then synchronized the
   reader view and obtained 8/8 green.
9. Regenerated the Canon index and ran the full documentation validator. Its
   first two passes exposed missing machine-readable report update declarations
   and a missing source path in the project-status stop section; both were
   corrected at their source before the final full pass.

## Files changed

Created:

- `mirrorea_canon/meta/proposals/PROPOSAL-030-sys1-runtime-kernel-internal-carrier.md`
- `mirrorea_canon/adr/ADR-0027.md`
- `docs/reports/2593-mirrorea-i2-systems-foundation-sys1-runtime-kernel-carrier.md`

Updated:

- `mirrorea_canon/README.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/theory/13-evaluation-materialization.md`
- `mirrorea_canon/spec/05-runtime-semantics.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/INDEX.json` (generated)
- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `plan/00-index.md`
- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `docs/mirrorea-project-overview.html`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- `progress.md`
- `tasks.md`

`samples_progress.md` was inspected and remains unchanged.

## Commands run

Baseline and resource inspection by this writer included:

```text
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
git show --stat --oneline --decorate 94e3707c
df -h .
free -h
TZ=Asia/Tokyo date '+%Y-%m-%d %H:%M:%S %Z'
```

The parent supplied fresh source-cut close evidence from these exact commands:

```text
cargo fmt --all -- --check
cargo clippy -p mir-runtime --all-targets -- -D warnings
cargo test -p mir-runtime --lib sys1_runtime_kernel_tests -- --nocapture
cargo test -p mir-runtime --lib
cargo test -p mir-runtime --test m10_source_execution -- --nocapture
cargo test -p mir-runtime --test m10_cli -- --nocapture
cargo test -p mir-runtime --test m10_conformance -- --nocapture
cargo test --workspace
git diff --check
```

A secret-pattern scan over the six changed runtime/test files returned no
matches. Its exact shell spelling was not retained in the docs handoff and is
not invented here.

The Oracle wrapper was attempted once, but the private browser profile was
logged out before prompt submission. It produced no advisory response. No
second attempt was made without new failure evidence.

The HTML test-first red check was:

```text
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html
```

It ran 8 tests and failed 3 current-pointer assertions because the old HTML
and Documentation still said SYS-1 active/SYS-2 next. This was the expected
red reason. The post-edit commands were:

```text
cd mirrorea_canon && python3 meta/build-index.py
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html
make docs
git diff --check
```

The first `make docs` invocation rejected Report 2593 because the
six update-status sections lacked the required exact `更新済み:` / `更新不要:`
declaration. The second advanced and rejected the `現在の停止線` section
because it had no repo-relative source path. After the two minimal corrections,
the third full invocation passed. A focused `rg` assertion then checked every
current pointer for SYS-1 completed / SYS-2 active / SYS-3 next and rejected
premature SYS-1-active/SYS-2-next wording in non-historical current views.
Subsequent fresh full invocations after that assertion also passed.

## Evidence / outputs / test results

- Focused SYS-1 kernel tests: **13/13 pass**.
- `mir-runtime` library: **25/25 pass**.
- M10 ordinary-source integration: **2/2 pass**.
- M10 CLI integration: **4/4 pass**.
- M10 conformance regression: **67/67 pass**.
- Full workspace: `cargo test --workspace` exited 0.
- Formatting and changed-crate all-targets warnings-denied Clippy exited 0.
- Parent source diff check and six-file secret-pattern scan passed.
- Independent semantics review: **ACCEPT**.
- Independent code-quality review: **ACCEPT**.
- The focused implementation tests cover exact provenance and occurrences,
  FIFO identity/result alignment, typed declared failure, invalid/duplicate/
  stale/wrong target/source/origin, restricted visibility, producer release vs
  evaluator authority, no-occurrence/no-M8-enqueue before admission, and
  malformed factory diagnostics rather than panic.
- Canon index regeneration/check: **174 files indexed**, pass.
- Focused HTML/Documentation reader test: **8/8 pass** after the recorded
  3-failure red check.
- Final `make docs`: agent configuration validation pass; Canon index check
  pass; source hierarchy **799/799**, missing 0; documentation scaffold pass;
  **1747 numbered reports** discovered.
- Current-state assertion: pass for SYS-1 completed, SYS-2 active, SYS-3 next,
  with no stale active-goal pointer in the checked current views.
- `git diff --check`: pass.
- New Proposal/ADR and the task-specific architecture/theory/spec records are
  below 15 KiB. The pre-existing aggregate `MAP.md` (23,049 bytes) and
  `CHANGELOG.md` (17,019 bytes) remain above the style-guide ceiling; this
  milestone updated their required registry lines but did not open an
  unrelated document-splitting project.

Evidence classification is `runtime-monitored`. No Lean theorem, model check,
general kernel/concurrency correspondence, or arbitrary scheduler claim is
part of SYS-1.

## What changed in understanding

The reusable production boundary is smaller than the M10 facade and smaller
than a generic effect runtime. The direct SYS-2/3 consumer is the exact
ordinary-source/generic-owner kernel plus two typed lifecycles. Specialized
scenario runners, legacy M8 receipt fixtures, release identities, and
correspondence predicates remain outside that claim.

OPEN-030 can close internally without freezing public transport or moving the
broad lifecycle. The remaining immediate systems problem is not carrier field
invention: it is whether ST and OW executions preserve abstract Mir order and
make revocation/publication/activation/cut changes visible at the required
edge despite the current immutable M9 admission snapshot.

## Open questions

- Which smallest safe mailbox/worker design implements OW while preserving
  owner seriality? Compare the current design and at most one viable
  alternative.
- What are the exact linearization points and backend visibility mappings for
  owner and designated-input operations?
- What finite abstraction detects revoke-after-enqueue/serve, publication,
  patch, save, relation epoch, and witness/capability ordering faults?
- OPEN-026, OPEN-027, full carrier freeze, public encoding, real transport,
  retry/exactly-once, and general fairness/memory proofs remain later scope.

## Suggested next prompt

Execute Plan 249 SYS-2: define the ST/OW backend contract over ADR-0027's
kernel/carrier, add edge-removal litmus/model falsifiers first, implement the
smallest safe one-owner-worker profile, verify selected ST/OW correspondence
and owner data-race freedom, and keep ordinary Surface, public wire, real
transport, lock-free work, and general memory theory unchanged.

## Plan update status

更新済み: Plan 249 records SYS-1 completed at exact cut `94e3707c...`,
SYS-2 active, SYS-3 next, accepted evidence, direct blocker, residual, and
reopen triggers. Plan 00 index matches the sole current roadmap.

## Documentation.md update status

更新済み: Documentation.md now says SYS-0/SYS-1 completed, SYS-2 active, SYS-3 next,
with OPEN-030 narrow internal resolution and broad lifecycle non-claims.

## docs/project-status.md update status

更新済み: `docs/project-status.md` records the SYS-1 source/evidence cut, validation
counts, active SYS-2 blocker, and unchanged theory T1/broad I1/I2 state.

## progress.md update status

更新済み: `progress.md` synchronizes the three axes, milestone/macro/feature rows,
startability, and the timestamped recent log.

## tasks.md update status

更新済み: `tasks.md` is rewritten as the current SYS-2 snapshot with ordered packages,
active execution order, research-discovery items, complete owner-reserved
decision boundary, estimates, and macro position.

## samples_progress.md update status

更新不要: `samples_progress.md` remains accurate because SYS-1 changed no runnable sample path,
validation command, user-visible debug surface, or sample blocker.
Conclusion: `samples_progress.md 更新不要`.

## Reviewer findings and follow-up

The independent semantics reviewer accepted the exact SYS-1 source cut. The
independent code-quality reviewer also accepted it. The closeout preserves the
review-bounded scope by explicitly excluding specialized M10 scenario paths,
generic provider-registry claims, broad lifecycle movement, and public
compatibility.

The independent Canon-first close planner review returned **ACCEPT — no
remaining P0/P1/P2**. It found one P2 current-index mismatch during review:
`mirrorea_canon/README.md` still counted 26 ADRs after ADR-0027 was added. The
count was corrected to 27 and this report's file inventory was updated before
the reviewer issued final acceptance. No other closeout correction remained.

## Skipped validations and reasons

- Lean `--trust=0` was not run because SYS-1 changed no Lean source, theorem,
  proof ledger status, or proof claim.
- Model checking was not run because SYS-1 adds no bounded model or concurrency
  refinement claim; those are SYS-2 direct evidence.
- Fresh-clone M10 release reproduction was not rerun because release/profile
  identity is preserved regression evidence, not the SYS-1 kernel identity.
- The pre-existing oversized aggregate MAP/changelog registers were not split;
  that restructuring has no SYS-2/3 direct consumer and the normal docs
  validator accepts their current structure.
- Real transport, browser renderer, durable persistence, performance, and
  deployment checks are outside this milestone.

## Commit / push status

The accepted implementation/evidence cut
`94e3707c7bc98d4a0764c51f13a12b1dae1968c6` is the pinned SYS-1 source cut and
was the clean `HEAD == origin/main` baseline for this closeout. This
Canon/report/status diff is not committed or pushed by the planning writer.
The parent owns its integration commit, push, clean-worktree check, and remote
parity before SYS-2 source work proceeds.

That immediate successor commit cannot embed its own hash inside the report
without creating a new successor. The report therefore pins the exact source
cut; the parent records the docs closeout commit/push/parity after integration.

## Sub-agent session close status

The handed-off implementation, semantics-review, and code-quality-review work
is complete. This planning/status writer spawned no additional sub-agent.
Oracle produced no advisory response. Parent integration/commit/push/parity
remains open; no session is represented as committed or pushed by this writer.
