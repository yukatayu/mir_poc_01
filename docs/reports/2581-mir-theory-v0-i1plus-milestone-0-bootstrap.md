# Report 2581 — Mir Theory v0 / I1+ Milestone 0 bootstrap

- Date: 2026-08-03 19:42 JST
- Author / agent: Codex orchestrator
- Scope: repository/agent/governance bootstrap for the owner-approved Mir
  Theory v0 + Mir I1+ Milestones 0--10
- Decision levels touched: owner-approved operating L0/L1 consequence through
  PROPOSAL-018 and ADR-0015. No theory rule, Core/Config/Step, SCN expectation,
  OBL status, Gate/Phase state, conformance verdict, or public contract moved.

## Objective

Make the repository capable of executing the owner-approved finite program
autonomously: record the exact start cut, reconcile Codex role permissions and
instructions, adopt one semantic frontier and one current roadmap, replace
per-task report proliferation with milestone reports, broaden bounded autonomy
without weakening the source hierarchy, and preserve official T0 while later
milestones remain unexecuted.

## Scope and assumptions

- Start from current `origin/main`, not an older local plan/report cut.
- `mirrorea_canon/` remains the only normative source; implementation,
  roadmaps, snapshots, tests, and this report are LAB.
- The user's direction is the owner disposition. This milestone applies it
  through proposal/ADR/changelog/index rather than treating the prompt itself
  as an untracked alternate Canon.
- Milestone 0 changes governance/configuration/documentation validation only.
  T0 semantic-assertion profile production and G0/T1 movement belong to M2.
- Existing v1/v2 governance artifacts, WRKs, historical plans/reports, and
  user data are preserved.

## Start state / dirty state

- `git fetch --prune origin` established
  `HEAD == origin/main == b9dcaa054c548112a7977776723418559b8ba8b2` on
  branch `main`.
- Worktree/index were clean; there were no untracked files, submodules, or
  extra Git worktrees.
- Installed Codex was `codex-cli 0.146.0`; current parent session had
  unrestricted filesystem/network and approval `Never`.
- Repository filesystem: 188 GiB total, about 21 GiB available, 89% used.
  Memory: 15 GiB total, about 7.8 GiB available at audit time. Repository was
  about 6.0 GiB, dominated by the existing `target/` directory. The documented
  `/mnt/mirrorea-work` candidate was absent/unmounted, so no heavy build was
  started.

## Documents consulted

- Canon entry/current operations: `mirrorea_canon/README.md`, `MAP.md`,
  `plan/README.md`, `plan/01-phases.md`, `plan/02-operating-model.md`,
  `plan/03-risks.md`, `meta/agent-instructions.md`,
  `meta/source-hierarchy.md`, and `meta/style-guide.md`.
- Canon decisions/process: ADR-0012 through ADR-0014, ADR index, CHANGELOG,
  Proposal 017, and current T0/G0 state referenced by Plan 01.
- Current LAB views: root `README.md`, `CANON.md`, `AGENTS.md`,
  `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `.docs/progress-task-axes.md`, Plans 246 and index,
  and directly referenced Report 2580.
- Agent/validation surfaces: `.codex/config.toml`, all `agents/*.toml`,
  `agents/README.md`, `Makefile`, `scripts/validate_docs.py`,
  `scripts/check_source_hierarchy.py`, their focused tests, and the report
  template.
- Installed Codex 0.146.0 help/doctor output and the locally fetched official
  Codex manual sections for custom agents, configuration precedence,
  sandboxing, and approvals.
- `docs/reports/` was not read as a history corpus.

## Actions taken

1. Audited remote/head/worktree/submodule/worktree/storage state and recorded
   the exact base revision.
2. Verified Codex-supported sandbox modes (`read-only`, `workspace-write`,
   `danger-full-access`), approval values, relative custom-agent config path
   behavior, parent override semantics, and lack of a dedicated dry-run command.
3. Corrected planner's `workspace-write`/do-not-edit contradiction: planner may
   apply and validate explicitly delegated roadmap/status/governance work, but
   not production source, tests, Lean, or normative semantics without exact
   delegation.
4. Made all role sandbox/approval defaults explicit, retained read-only mapping,
   documentation research, and review roles, and corrected stale agent schema
   documentation/status-reporter paths.
5. Added a deterministic repository agent-config validator through TDD,
   including sandbox and approval-policy role checks, and made it the first
   `make docs` gate.
6. Recorded the owner direction as PROPOSAL-018 and ADR-0015, with ADR-0014
   retained as the default program-external research route and L2 still
   fail-closed.
7. Recut the Canon operating model around one frontier/roadmap, one report per
   milestone, WRK admission, two-candidate limit, role ownership, proof status,
   and owner escalation conditions.
8. Designated Plan 247 as the sole current execution roadmap and synchronized
   the current LAB status views; old plans remain history, not a queue.
9. Kept samples, theory/spec/scenarios, proof ledger, T0/G0 state, and runtime
   implementation unchanged.

## Files changed

- Governance/entry: `AGENTS.md`, `CANON.md`, `README.md`, and
  `.docs/current-l2-source-sample-authoring-policy.md`.
- Agent configuration: `.codex/config.toml`, `agents/README.md`, and all eight
  registered `agents/*.toml` files.
- Canon: PROPOSAL-018, ADR-0015, bounded-successor note in ADR-0014, ADR-0012,
  ADR index, CHANGELOG, README, MAP, meta agent/source/style rules, Plan 02,
  Plan README, risks, and regenerated INDEX.
- Roadmap/status: Plan 247, plan index, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, and `tasks.md`.
- Validation: `scripts/validate_agent_configs.py`, its focused tests,
  `Makefile`, `scripts/README.md`, and Plan 247 registration in both docs/source
  hierarchy validators.
- Report: this file.

## Commands run

- Git/remote/worktree/submodule audit with `git fetch`, `git status`,
  `git rev-parse`, `git worktree list`, and `.gitmodules`/submodule checks.
- Resource audit with `df -h`, `free -h`, `du`, `lsblk -f`, and `findmnt`.
- `codex --version`, `codex --help`, `codex exec --help`, strict config help,
  and `codex doctor`/JSON diagnostics.
- Official Codex manual fetch/status and focused manual searches.
- TDD RED then GREEN runs for `scripts/tests/test_validate_agent_configs.py`.
- Canon index generation/check, agent-config validation, focused docs/source
  validators, stale-wording scans, `git diff --check`, and final milestone
  validation listed below.

## Evidence / outputs / test results

- TDD RED was observed before the validator existed: `10 failed, 3 passed`;
  failures were specifically the absent validator contract, not collection or
  syntax failures. The final independent review then exposed a missing
  `approval_policy` validation. Its regression tests produced three expected
  failures (missing policy, unsupported value, and role mismatch) before the
  corrective implementation.
- GREEN after the initial implementation: `python3 -m pytest
  scripts/tests/test_validate_agent_configs.py -q` returned `7 passed, 6
  subtests passed`. After the review correction, `python3 -m unittest
  scripts.tests.test_validate_agent_configs -v` returned `9` passing tests and
  the standalone validator returned `Agent configuration validation passed.`
- `codex --strict-config -C . --help` returned zero after the role changes.
  Codex 0.146.0 has no custom-agent config-only `--dry-run`; the strict parse,
  repository validator, and an actual planner write delegation form the
  available validation combination.
- Canon index generation/check returned 136 indexed files after adding
  PROPOSAL-018 and ADR-0015.
- Final docs/review/commit evidence is completed in the close sequence below;
  no result is treated as passed until its command exits zero.

## What changed in understanding

The previous blocker was not a missing ability to write files. Planner already
had `workspace-write`; its developer instructions and role description denied
the very planning/status writes requested by the owner. Codex also reapplies
live parent permission overrides to children, so role TOML is a declared
least-privilege default and ownership contract rather than an absolute ceiling.

Governance now has two explicit profiles. ADR-0015 governs the finite M0--M10
program and permits evidence-gated semantic/implementation progress. ADR-0014
continues to govern unrelated research and does not become an unlimited Canon
write grant. This separation preserves both the owner's new direction and the
existing trust-anchor fail-closed rule.

The official lifecycle is still T0. Milestone 0 makes the critical path
executable but does not itself satisfy the new T0 profile, accept G0-D3, exit
G0, enter T1, prove an OBL, or implement I1+.

## Open questions

No owner decision is required for Milestone 0. T0 semantic assertions, the
Constitution's exact compact placement, and later internal carriers are bounded
decisions assigned to subsequent milestones. They are not blockers for M0.

## Suggested next prompt

No prompt is required. Continue autonomously with Milestone 1: consolidate the
owner direction and current Canon into one concise Design Constitution, remove
semantic contradictions in the same milestone, review, validate, commit, and
push.

## Plan update status

`plan/` 更新済み: Plan 247 is the sole current M0--M10 roadmap and
`plan/00-index.md` registers it. Older plans are preserved as LAB memory.

## Documentation.md update status

`Documentation.md` 更新済み: its current-status/roadmap entry points now route
to Plan 247 and the M0--M10 program instead of presenting the prior plan family
as an active queue.

## docs/project-status.md update status

更新済み: official T0 is preserved while M0 and the next M1 frontier, direct
blocker, owner boundary, and single current roadmap are shown explicitly.

## progress.md update status

`progress.md` 更新済み: the derived status reflects M0 execution, official T0
non-movement, the M0--M10 critical path, and a command-derived timestamped log.

## tasks.md update status

`tasks.md` 更新済み: the previous broad queue is replaced by one ordered
M0--M10 current task map with active/next milestone and owner escalation only.

## samples_progress.md update status

`samples_progress.md` 更新不要: M0 changes no runnable sample path, command,
debug surface, evidence classification, or sample blocker.

## Reviewer findings and follow-up

- Pre-edit planner review required M0 to remain governance/bootstrap-only,
  preserve official T0 and all OBL/SCN/conformance states, distinguish planner
  write capability from authority, retain old plans/WRKs/artifacts, and use one
  final independent review.
- It suggested implementing the future T0 profile in M0. The owner milestone
  sequence places profile adoption/fresh pass/G0 closeout in M2, so M0 records
  that dependency in Plan 247 without prematurely changing ADR-0013 or phase
  state.
- Final independent reviewer found one P2 omission: the validator did not
  enforce the explicit `approval_policy` contract stated in `agents/README.md`.
  The corrective TDD cycle added supported-value and per-role checks, plus
  missing/invalid/mismatch regression cases. The reviewer found no other
  material contradiction: T0/G0/T1, OBL, SCN, conformance, and runtime state
  remain untouched; ADR-0015 remains bounded relative to ADR-0014; Plan 247 is
  the sole current roadmap; and planner write capability remains distinct from
  normative authority.

## Skipped validations and reasons

- Cargo build/test, Lean compilation, runtime replay, model checking, sample
  suites, save/load, patch, relation, designated-evaluator, auth, and release
  conformance were not run in M0 because no semantic implementation, Lean
  source, or sample behavior changed. Their required validations remain bound
  to the milestone that changes those layers.
- No heavy disposable build was run because the verified external workdir was
  absent and root storage was already 89% used.
- No Oracle consultation was used. Installed Codex behavior was answered by
  official local/manual evidence, and repository governance received separate
  planner/schema/config/final-review checks.

## Commit / push status

The substantive milestone payload was committed with `--no-gpg-sign` as
`be5928a168fd519c05867fba2746ddd833a3bde5` (`Bootstrap Mir Theory v0 I1+
governance`) and pushed to `origin/main`. A fresh `git fetch --prune origin`
confirmed `HEAD == origin/main == be5928a168fd519c05867fba2746ddd833a3bde5`.
This closeout/report-and-snapshot commit is the second and final M0 commit; it
will be pushed and parity-checked before M1 begins.

## Sub-agent session close status

- Canon pre-edit planner review: complete, read-only.
- Codex schema/manual audit: complete, read-only.
- Repository config/validation map: complete, read-only.
- Agent-config test author: complete after RED evidence.
- Agent-config implementer: complete after GREEN evidence.
- Current-roadmap planner writer: Plan 247 and the initial `tasks.md` snapshot
  were applied; following the crash its completion was safely integrated by the
  orchestrator without reopening its scope.
- Final independent reviewer: complete, read-only; its one P2 finding was
  corrected in the single permitted review follow-up cycle.
