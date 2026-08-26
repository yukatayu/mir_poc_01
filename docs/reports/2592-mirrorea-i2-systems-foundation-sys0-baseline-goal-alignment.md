# Report 2592 — Mirrorea I2 Systems Foundation SYS-0: baseline and goal alignment

- Date: 2026-08-26 JST
- Milestone: SYS-0
- Status: final independent close review ACCEPT with no P0/P1/P2; SYS-0 remains
  active/closing until parent integration, commit, push, and remote parity
- Decision levels touched: operational L0 owner authority (ADR-0026), L1
  program direction/operating model, LAB roadmap/status

## Objective

Preserve the accepted M10 finite reference baseline while recording the
owner-authorized Mirrorea I2 Systems Foundation program, one sole current
roadmap, complete goal-driven SYS-0--SYS-7 control statements, the exact
lifecycle/non-effect boundary, and SYS-1 as the next goal after SYS-0 closes.

## Scope and assumptions

`mirrorea_canon/` remains the sole normative source. This milestone may change
governance/plan/status pointers and the focused primary-reader regression test,
but does not change Rust, Lean proofs, runtime semantics, the proof ledger,
frozen SCN expectations, or accepted M10 implementation behavior. It records a
new bounded implementation-program authority through normal proposal/ADR/
changelog/index procedure.

Program activation is distinct from official lifecycle acceptance. Theory
remains T1; broad PHASE-I1 exit, I2 lifecycle entry, and I2 exit are unclaimed
at SYS-0. Public grammar/API/ABI/wire, real transport, durable distributed
persistence, production, browser/View renderer, general metatheory, and I3
implementation remain non-effects.

## Start state / dirty state

- Branch: `main`.
- Baseline `HEAD`:
  `49e6845ada990a1c9d9944896a5ff1754994a1b3` (`docs: refresh Mir project overview`).
- `origin/main` matched that revision and the worktree was clean at the task
  baseline.
- Accepted immutable M10 implementation/validation cut:
  `23f5a8130334bf0c8516d51e9dcea38b92f50db1`; it is an ancestor of baseline
  HEAD. Only three documentation/closeout commits followed it.
- Resource floor before substantial work: root filesystem had about 70 GiB
  free; available memory was about 11 GiB.
- No pre-existing uncommitted user change was present at baseline. Later
  shared-worktree changes belong to their delegated writers and must not be
  reverted.

## Documents consulted

- Canon entry/direction: `mirrorea_canon/README.md`, `NORTH-STAR.md`,
  `DESIGN-CONSTITUTION.md`, `MAP.md`.
- Canon execution/lifecycle: `mirrorea_canon/plan/00-gates.md`,
  `plan/README.md`, `plan/01-phases.md`, `plan/02-operating-model.md`,
  `meta/source-hierarchy.md`, `meta/agent-instructions.md`, and root
  `CANON.md`.
- Authority/history: ADR-0014, ADR-0015, ADR-0025, PROPOSAL-018, the ADR index,
  and Canon changelog.
- Runtime/projection boundary: `mirrorea_canon/architecture/01-strata.md`
  through `05-satellites.md`, especially `04-runtime-carriers.md` OPEN-030.
- Task-relevant theory/spec: ordering/cuts, authority, existence/fallback,
  observation, patch, two-layer time, evaluation/materialization, maintained
  relation/projection, M5--M9 finite boundaries, Core/runtime/conformance and
  M7--M10 specifications.
- LAB reader/status sources: `README.md`, `Documentation.md`,
  `docs/project-status.md`, `docs/mirrorea-project-overview.html`, its focused
  regression test, `progress.md`, `tasks.md`, `samples_progress.md`,
  `.docs/progress-task-axes.md`, and `plan/00-index.md`.
- Direct historical records only: closed Plan 247, M10 Report 2591, and the
  prechange independent SYS-0 review. `docs/reports/` was not read in bulk.

## Actions taken

1. Recorded the 2026-08-26 owner disposition in PROPOSAL-029 and adopted the
   new bounded authority in ADR-0026 without reopening ADR-0015/Plan 247.
2. Created Plan 249 as the sole current roadmap with the program parent goal,
   completed/active/next goals, blockers, evidence, deferred scope, fixed
   SYS-0--SYS-7 sequence, and complete Goal Statements for every milestone.
3. Added the 15-row meta-alignment matrix connecting owner intent, Canon,
   implementation evidence, current gap, and direct program consumer.
4. Canonized the direct-consumer rule, candidate limit, finite stop rule,
   one-report policy, evidence/review discipline, allowed change surface, and
   owner-reserved stop line.
5. Synchronized Canon and LAB reader/status pointers from “no active roadmap /
   owner direction required” to the active ADR-0026 program. Until integration
   commit/push/parity, SYS-0 remains active/closing and SYS-1 remains next, not
   active; official T1 and broad I1/I2 non-acceptance remain unchanged.
6. Left `samples_progress.md` unchanged because SYS-0 changed no runnable
   sample, validation command, debug surface, or sample blocker.
7. Registered Plan 249 in both documentation-validator required-path lists.
   `scripts/check_source_hierarchy.py` required the same one-line registration
   because its numbered-plan completeness test mirrors
   `scripts/validate_docs.py`; no validation behavior changed beyond admitting
   the newly authorized current roadmap.
8. After the first close review returned REJECT without a P0, addressed its
   authority-entry, stop-line, HTML, README, current-state, and report-command
   findings in the first correction cycle. A residual stale status/order
   pointer and scan gap were later found by the second review.
9. After the second narrow review returned REJECT, corrected the remaining
   Plan 249 pointer and recommended-action transition, strengthened the
   stale-state scan across line breaks, and separated parent baseline local-ref
   evidence from the reviewer's remote-head query.

## Files changed

Created:

- `mirrorea_canon/meta/proposals/PROPOSAL-029-mirrorea-i2-systems-foundation.md`
- `mirrorea_canon/adr/ADR-0026.md`
- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md`

Updated within the delegated planning/governance/status surface:

- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/plan/01-phases.md`
- `mirrorea_canon/plan/README.md`
- `mirrorea_canon/plan/02-operating-model.md`
- `mirrorea_canon/meta/source-hierarchy.md`
- `mirrorea_canon/meta/agent-instructions.md`
- `mirrorea_canon/INDEX.json` (generated)
- `CANON.md`
- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `docs/project-status.md`
- `docs/mirrorea-project-overview.html`
- `plan/00-index.md`
- `progress.md`
- `tasks.md`
- `scripts/tests/test_mirrorea_project_overview_html.py`
- `scripts/validate_docs.py` (one required-path registration)
- `scripts/check_source_hierarchy.py` (matching one required-path registration)

`samples_progress.md` was inspected but not changed.

## Commands run

The baseline/resource invocation contained these exact commands:

```text
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
git rev-parse HEAD
git rev-parse --abbrev-ref HEAD
git rev-parse origin/main
git remote -v
git worktree list --porcelain
git ls-files --others --exclude-standard
find . -name AGENTS.md -print
df -h .
free -h
git log --oneline --decorate --graph 23f5a813..HEAD --max-count=80
git diff --stat 23f5a813..HEAD
git diff --name-status 23f5a813..HEAD
```

Those parent baseline commands compared the checked-out commit with the local
`origin/main` tracking ref. They did not run or claim a baseline `git fetch`.
The close reviewer separately checked the remote branch head with this exact
command and result (also rechecked during this correction):

```text
git ls-remote --heads origin refs/heads/main
49e6845ada990a1c9d9944896a5ff1754994a1b3	refs/heads/main
```

The five exact M10 regression invocations were:

```text
cargo test -p mir-runtime --test m10_conformance -- --nocapture
cargo test -p mir-runtime --test m10_source_execution -- --nocapture
cargo test -p mir-runtime --test m10_cli -- --nocapture
cargo test -p mir-ast --test surface_v0_m10_visibility -- --nocapture
cargo test -p mir-semantics --test surface_v0_pipeline_m10_visibility -- --nocapture
```

The exact agent/config probes were:

```text
python3 scripts/validate_agent_configs.py
python3 -m unittest scripts.tests.test_validate_agent_configs
codex --strict-config -C . --help >/tmp/mirrorea-codex-help.txt && wc -l /tmp/mirrorea-codex-help.txt
```

The Oracle wrapper invocation is retained by session
`sys1-kernel-carrier-review`. The literal prompt is stored in
`/home/codex/.oracle/sessions/sys1-kernel-carrier-review/meta.json`; it is not
duplicated here. With that literal substituted, the exact attachment/output
shape was:

```text
/home/codex/.local/bin/ask-chatgpt-pro-temp \
  --slug sys1-kernel-carrier-review \
  --write-output /tmp/sys1-kernel-carrier-review.md \
  -p '<prompt recorded verbatim in session meta.json>' \
  --file mirrorea_canon/NORTH-STAR.md \
  --file mirrorea_canon/DESIGN-CONSTITUTION.md \
  --file mirrorea_canon/architecture/02-boundary-contracts.md \
  --file mirrorea_canon/architecture/03-toolchain.md \
  --file mirrorea_canon/architecture/04-runtime-carriers.md \
  --file mirrorea_canon/theory/13-evaluation-materialization.md \
  --file mirrorea_canon/adr/ADR-0025.md \
  --file crates/mir-runtime/src/m10_reference_system.rs \
  --file crates/mir-runtime/src/lib.rs \
  --file crates/mir-runtime/src/m8_runtime_owner_queue.rs \
  --file crates/mir-runtime/src/m8_runtime_relation_projection.rs \
  --file crates/mir-runtime/src/m8_runtime_designated_value.rs \
  --file crates/mir-runtime/src/m8_runtime_authority.rs \
  --file crates/mir-runtime/src/m8_runtime_admission.rs \
  --file crates/mir-runtime/src/m8_runtime_local_cut.rs \
  --file crates/mir-runtime/src/m8_runtime_patch.rs \
  --file crates/mir-runtime/src/m8_runtime_observer.rs \
  --file crates/mir-runtime/src/m9_auth_verification.rs
oracle status --hours 24 --limit 20
oracle session sys1-kernel-carrier-review
```

The intended output path `/tmp/sys1-kernel-carrier-review.md` was not created
because browser login failed before prompt submission. Session inspection in
the correction cycle used:

```text
oracle session sys1-kernel-carrier-review
find /home/codex/.oracle/sessions/sys1-kernel-carrier-review -maxdepth 2 -type f -print | sort
sed -n '1,260p' /home/codex/.oracle/sessions/sys1-kernel-carrier-review/meta.json
sed -n '1,220p' /home/codex/.oracle/sessions/sys1-kernel-carrier-review/output.log
```

The prechange and close reviews were delegated through the repository's agent
orchestrator rather than a shell command.

Writer/correction validation commands were:

```text
cd mirrorea_canon && python3 meta/build-index.py
make docs
git diff --check
python3 -m unittest -v scripts.tests.test_validate_docs.ValidateDocsTests.test_all_repo_numbered_plan_files_are_registered
python3 -m unittest -v scripts.tests.test_mirrorea_project_overview_html
```

The HTML command was deliberately run once after the test update but before
the HTML/documentation fix (3 failures, expected red), then after the fix (8/8
pass). Final fresh results are recorded below.

Focused stale-state and diff inspection used:

```text
rg -n 'active roadmap なし|新 owner direction が必要|owner direction 待ち|successor を開くか pause するか|I2 は未開始|新 owner roadmap|post-program scope を owner が選ぶ地点|closed M10 baseline の次に、どの direct consumer と acceptance profile を開くかが未選択' docs/mirrorea-project-overview.html README.md Documentation.md docs/project-status.md progress.md tasks.md plan/00-index.md plan/249-mirrorea-i2-systems-foundation-current-roadmap.md
rg -n -i 'SYS-1.*active|active.*SYS-1|SYS-0.*completed|completed.*SYS-0' README.md Documentation.md docs/project-status.md docs/mirrorea-project-overview.html progress.md tasks.md plan/00-index.md plan/249-mirrorea-i2-systems-foundation-current-roadmap.md docs/reports/2592-mirrorea-i2-systems-foundation-sys0-baseline-goal-alignment.md
python3 - <<'PY'
import re
from pathlib import Path

text = re.sub(r"\s+", " ", Path("plan/00-index.md").read_text())
stale = re.compile(
    r"SYS-0.{0,120}(?:をclose|closed|completed).{0,180}"
    r"SYS-1.{0,120}(?:がactive|is active)",
    re.IGNORECASE,
)
current = re.compile(
    r"SYS-0.{0,120}active / closing.{0,180}"
    r"SYS-1.{0,120}next / not active",
    re.IGNORECASE,
)
assert stale.search(text) is None
assert current.search(text) is not None
print("Plan 249 pointer: SYS-0 active/closing; SYS-1 next/not active")
PY
git diff --stat
git status --short
git diff -- CANON.md README.md Documentation.md docs/project-status.md docs/mirrorea-project-overview.html mirrorea_canon/meta/source-hierarchy.md mirrorea_canon/meta/agent-instructions.md mirrorea_canon/plan/README.md mirrorea_canon/plan/01-phases.md progress.md tasks.md scripts/tests/test_mirrorea_project_overview_html.py
```

## Evidence / outputs / test results

- Parent baseline local-ref evidence: `HEAD == origin/main ==
  49e6845ada990a1c9d9944896a5ff1754994a1b3`; clean worktree at task start. No
  baseline fetch is claimed.
- Reviewer-run remote-head evidence:
  `git ls-remote --heads origin refs/heads/main` returned
  `49e6845ada990a1c9d9944896a5ff1754994a1b3 refs/heads/main`, agreeing with
  baseline HEAD and the local `origin/main` ref.
- M10 ancestry: accepted cut `23f5a813...` is retained; three later commits
  are documentation/closeout/reader updates only.
- M10 focused regression: all five groups passed, exactly 67 + 2 + 4 + 3 + 5,
  zero failures.
- Agent configuration validator passed; focused config unit tests passed 9/9;
  strict-config help exited 0. The planner can write its delegated planning,
  status, roadmap, governance-proposal, and agent-config surfaces.
- Oracle consultation was attempted once. The browser profile was logged out,
  so the attempt failed before prompt submission and produced no advice. It
  was not retried without concrete new failure evidence.
- Prechange independent planner review disposition: GO; no North Star
  contradiction, mandatory stop condition, or P0 finding.
- Post-edit Canon index regeneration/check passed: `ok: 172 files indexed`.
- The first documentation validation correctly rejected unregistered Plan 249;
  after the two matching one-line registry additions, its focused numbered-plan
  completeness unit test passed 1/1.
- A subsequent documentation validation correctly rejected the owner-decision
  status section until it cited repository-relative authority/roadmap sources;
  the corrected final `make docs` passed. Its exact summary was agent config
  validation passed, Canon index `172`, source hierarchy `799/799` with zero
  missing paths, and documentation scaffold complete with 1,746 numbered
  reports.
- Post-edit `git diff --check` passed with no whitespace errors.
- Focused primary-reader regression demonstrated the correction: after the
  new current-state assertions were written, the unchanged HTML/Documentation
  failed 3 of 8 tests; after correction, all 8 passed.
- First/second correction-cycle Canon index regeneration/check passed at 172 files. Fresh
  `make docs` passed with agent configuration validation, source hierarchy
  799/799 and zero missing paths, and documentation scaffold completeness over
  1,746 numbered reports. The focused numbered-plan registry test passed 1/1,
  the final HTML suite passed 8/8, and fresh `git diff --check` passed.
- The independent close-review sequence and forward correction history are
  recorded below. Final review accepted the listed validation/evidence with
  **no P0/P1/P2**; SYS-0 integration, commit, push, and parity remain pending.

No proof-ledger, SCN expectation, Rust behavior, Lean source, or runnable sample
changed in this planning cut. One focused HTML reader regression test changed.

## What changed in understanding

The accepted M10 system is a finite executable reference laboratory, not yet
the North Star's generated per-locus fabric. The next direct capability is not
real networking or more general theory: it is to separate the semantic kernel
from conformance/release orchestration, fix a narrow internal semantic carrier,
map concurrency before threading, generate locus artifacts/communication, and
actually dispatch them in-process.

The new owner direction satisfies ADR-0025's requirement for a successor
program and narrow I2 entry contract. That authority lets implementation work
start, but it does not itself satisfy the pre-existing broad I1/I2 lifecycle
exit criteria.

Milestone state changes only after its full close gate. Creating the authority,
roadmap, and local validation did not complete SYS-0: repeat independent review,
integration commit, push, and remote parity remain. Therefore SYS-0 is
active/closing and SYS-1 is next, not active.

## Open questions

- SYS-1 must decide the smallest internal request/reply/receipt carrier and
  whether its evidence satisfies broad I1 carrier-freeze criteria. If not, it
  records the exact residual and still provides the narrow internal boundary
  needed by SYS-2/3.
- SYS-2 must select the exact safe OW primitives and finite abstraction without
  importing low-level memory ordering into ordinary Surface.
- SYS-3 must choose a conservative finite-DAG extension boundary without
  claiming an arbitrary DAG theorem.
- Final CLI spelling remains provisional/internal until the implementation
  follows existing conventions.

None of these is an owner-reserved stop condition at SYS-0.

## Suggested next prompt

Repeat the independent SYS-0 close review against this correction diff. If it
accepts, integrate/commit/push, verify a clean worktree and remote parity,
record the exact cut in this report, then and only then transition Plan 249 and
current views from SYS-0 closing to SYS-1 active.

## Plan update status

更新済み: Plan 249 is the sole current execution roadmap with complete
SYS-0--SYS-7 Goal Statements and the meta-alignment matrix. Plan 247 remains an
unchanged closed record/regression baseline. `plan/00-index.md` points to both
with distinct current/historical roles. SYS-0 is active/closing and SYS-1 is
next until review/commit/push/parity close.

## Documentation.md update status

更新済み: the reader entry now identifies ADR-0026/Plan 249, SYS-0 closing,
and SYS-1 next while retaining the M10 baseline and lifecycle/public non-claims.

## docs/project-status.md update status

更新済み: the concise LAB control view records SYS-0 closing, SYS-1 next,
current program authority, exact official T1/non-acceptance boundary, close
blocker, and following technical blocker.

## progress.md update status

更新済み: the three axes, macro map, feature rows, startability, SYS-0/SYS-1
timing, and recent log were synchronized. The correction log uses the actual
`TZ=Asia/Tokyo date` value `2026-08-26 19:31 JST`.

## tasks.md update status

更新済み: rewritten as a current snapshot with ordered SYS packages,
macro/rough estimates, self-driven versus research-discovery items, and the
complete owner-reserved stop-condition set.

## samples_progress.md update status

`samples_progress.md 更新不要`: SYS-0 changed no runnable sample path,
validation command, debug surface, sample classification, or sample blocker.
The accepted M10 dashboard remains accurate.

## Reviewer findings and follow-up

Prechange independent planner review: GO. It recommended PROPOSAL-029,
ADR-0026, Plan 249, retention of Plan 247 as a closed baseline, no new WRK,
and explicit separation of program activation from lifecycle acceptance. It
identified six drift risks reflected in this cut: lifecycle overclaim, closed-
record rewrite, internal/public carrier collapse, I4/I5 overclaim, treating the
M10 facade as systems architecture, and leaking memory-order vocabulary into
Surface.

First independent close review: **REJECT, P0 none**. It found:

1. `meta/source-hierarchy`, Canon `plan/README`, and root `CANON.md` still
   described ADR-0015 as the active program and did not distinguish active
   ADR-0026 from outside-program ADR-0014.
2. `meta/agent-instructions` omitted hidden multi-owner transaction and real-
   transport selection/implementation from the ADR-0026 stop line.
3. The primary HTML reader still said no active roadmap/owner direction wait;
   README still presented Plan 149's pre-T0/G0-exit reading as current.
4. Current views advanced SYS-1 before SYS-0 review/commit/push/parity close.
5. This report summarized commands rather than recording exact invocations and
   lacked exact Oracle session/attachments/output disposition.
6. `tasks.md` did not expose the complete stop-condition set.

These six findings were addressed in the first forward correction cycle. The
HTML change followed a focused red→green test. The second review later found a
residual stale status/order pointer and insufficient multiline scan, so the
first cycle was not itself final acceptance.

Second independent narrow close review: **REJECT**. It found:

1. `plan/00-index.md` still said SYS-0 had closed and SYS-1 was active.
2. Plan 249's recommended action told the runner to execute SYS-1 before
   completing SYS-0 review/integration/push/parity.
3. This report's stale-state scan omitted `plan/00-index.md` and used only
   line-oriented patterns that missed the split-line transition.
4. Remote parity evidence did not identify the reviewer's exact
   `git ls-remote` command/result or distinguish it from the parent's baseline
   local-ref probes.

All four findings were corrected: Plan 249 pointers now say SYS-0
active/closing and SYS-1 next/not active; the recommended action closes SYS-0
first; a whitespace-normalized multiline pointer assertion complements `rg`;
and the reviewer-run remote-head query is recorded separately without
inventing a baseline fetch. Another independent close review is **pending**;
no acceptance is pre-claimed.

Third independent close review: **REJECT, P0/P1 none, one P2**. Its sole
finding was that Action 8 and the first-cycle summary still said every/all
first-review finding had been corrected, even though the forward history
records residual stale status/order evidence found by the second review. The
wording now says that the first-cycle findings were addressed there and that
the second review later found the residual state/order and scan gaps. Final
independent acceptance was not pre-claimed at that correction point.

Final independent close review: **ACCEPT — no P0/P1/P2**. It accepted the
truthful forward correction history and the validation/evidence listed in this
report. Plan 247 remains the closed immutable baseline and ADR-0025 remains
unchanged. This acceptance closes the independent review gate only: SYS-0
remains active/closing until the parent integrates, commits, pushes, verifies
remote parity, and records that evidence.

## Skipped validations and reasons

- No Rust/Lean/runtime source changed in this delegated planning cut. The parent
  already ran the five focused M10 regression groups before editing; this
  writer did not rerun full workspace Cargo, Clippy, formatting, Lean, or
  model-check suites because they are implementation/formal close validations,
  not evidence created by these documentation/reader-test changes.
- No second Oracle attempt was made: the first failed before submission due to
  a logged-out browser and supplied no advisory result; duplicate retry lacked
  concrete recovery evidence.
- Integration, commit, push, clean final worktree, and remote parity are
  pending parent orchestration and are not claimed as passed.

## Commit / push status

Pending. The planning/status writer was explicitly instructed not to commit or
push. The parent orchestrator must integrate the independently accepted cut,
then record the exact commit, push result, and `HEAD == origin/main` parity
here.

## Sub-agent session close status

The config audit, Canon/code mapping, prechange planner review, Oracle attempt,
all correction reviewers, and the final accepting reviewer returned. The
planning/status writer session is complete and returns the accepted diff to the
parent without commit/push. All delegated writer/reviewer sessions are closed;
parent-owned integration, commit, push, and remote parity remain pending.
