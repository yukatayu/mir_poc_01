# Report 2600 — ALIGN-0 baseline / one goal / meta-drift alignment

Identifier: `ALIGN-0` / Mirrorea I3 Distributed Foundation

## Objective

Preserve the accepted Mirrorea I2 regression floor and establish the owner-authorized Mirrorea I3 Distributed Foundation as the sole bounded program, with one current roadmap, one active goal, explicit non-effects, and a finite direct-consumer path to ALIGN-1.

Direct consumer: ALIGN-1 project/product layer constitution.
Blocker reduced: the repository correctly had no active successor program before the 2026-09-01 owner direction, so its current pointers had become stale after that direction.
Acceptance use: Canon and LAB readers can distinguish program activation from official I3 lifecycle entry and can execute the next milestone without inventing authority or reopening accepted I2.

## Scope and assumptions

This milestone changes program authority, current-roadmap pointers, and status documentation only. It does not change accepted I2 semantics or runtime source, select a transport, enter or exit official I3, alter theory T1 or broad PHASE-I1, freeze a public surface, or define upper-product semantics.

The user-provided 2026-09-01 direction is treated as the owner-authenticated authorization required by ADR-0033 and Canon plan/05. `mirrorea_canon/` remains the normative source; Plan 250, status documents, and this report are LAB views.

## Start state / dirty state

- Pinned start revision: `ca6ffeceda6b2ed87edd2b98d6d2a6a74f61f9df`.
- `HEAD`, local `main`, and `origin/main` matched; the worktree was clean and had no untracked files.
- The owner-cited `6fa70ffd...` baseline was an ancestor. The intervening commits changed agent-orchestration guidance, not the accepted I2 semantic cuts.
- Codex CLI: `0.152.0`; effective filesystem permissions were unrestricted and no approval escalation was available or needed.
- Planner configuration retained write capability. No Git worktree was created, per owner direction.
- Resource audit: root filesystem was about 89% used with about 21 GiB free; the repository was about 28 GiB, almost all in the existing `target/`; `/mnt/mirrorea-work` was not mounted; available memory was about 8.3 GiB. No new heavy build tree was created.

## Documents consulted

Canon was read first: `README.md`, `MAP.md`, `NORTH-STAR.md`, `DESIGN-CONSTITUTION.md`, architecture 01--05, plans 00/01/05, ADR-0026 and ADR-0028--0033, and theory ledger 11. LAB/current evidence then included root `README.md`, `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, `.docs/progress-task-axes.md`, closed Plan 249, the I2 hands-on walkthrough, and only the reports directly required by those current sources.

The accepted I2 source-to-runtime path was inspected from the `mir` CLI through checked Core, SYS-3 projection, SYS-4 generated dispatch, SYS-5 workflow, and the downstream-only SYS-6 verifier. Existing unrelated alpha TCP/provider code was classified as LAB evidence, not an accepted I3 shortcut.

## Actions taken

- Recorded owner direction in PROPOSAL-037 and ADR-0034 through the Canon proposal/decision/changelog/index process.
- Created Plan 250 as the sole current roadmap with fixed ALIGN-0 through NEXT-0 sequencing and one active goal.
- Kept official I3 lifecycle inactive until I3-6; kept both transport candidates unselected.
- Added the numbered roadmap to repository validators and regenerated the Canon index.
- Synchronized current Canon/LAB navigation and status pointers.
- Preserved the accepted I2 source-first runtime as a regression floor and recorded module/risk seams without changing production source.
- Consulted an independent planner, semantic reviewer, code mapper, regression runner, status writer, and an advisory browser Oracle. The Oracle advice was checked against local Canon and owner direction rather than adopted automatically.

## Files changed

Normative records and navigation:

- `mirrorea_canon/meta/proposals/PROPOSAL-037-mirrorea-i3-distributed-foundation.md`
- `mirrorea_canon/adr/ADR-0034.md`
- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `mirrorea_canon/CHANGELOG.md`, `mirrorea_canon/INDEX.json`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/plan/README.md`, `mirrorea_canon/plan/01-phases.md`, `mirrorea_canon/plan/02-operating-model.md`, `mirrorea_canon/plan/05-i3-entry-contract.md`
- `mirrorea_canon/meta/agent-instructions.md`, `mirrorea_canon/meta/source-hierarchy.md`, `mirrorea_canon/meta/style-guide.md`

LAB/current readers and validation:

- `plan/250-mirrorea-i3-distributed-foundation-current-roadmap.md`, `plan/00-index.md`
- `AGENTS.md`, `CANON.md`, `README.md`, `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/mirrorea-project-overview.html`, `scripts/tests/test_mirrorea_project_overview_html.py`
- `scripts/check_source_hierarchy.py`, `scripts/validate_docs.py`
- `docs/reports/2600-mirrorea-i3-distributed-foundation-align0-baseline-goal-alignment.md` (this sole ALIGN-0 report)

No Rust, Lean, model, sample, or generated runtime artifact was changed.

## Commands run

- Repository/config/resource inspection: `git rev-parse`, `git status`, remote parity checks, `codex --version`, strict agent-config inspection, `df -h`, `du -sh`, `free -h`, `lsblk -f`, and `findmnt`.
- Canon index: `cd mirrorea_canon && python3 meta/build-index.py` and `python3 meta/build-index.py --check`.
- Accepted I2 workflow: the four exact `project-loci`, `run-local`, `inspect`, and `conform-i2` commands documented in `docs/hands_on/mirrorea_i2_local_toy_01.md`.
- Final milestone validators and focused test commands are recorded with their results below after the integrated cut is checked.

## Evidence / outputs / test results

The accepted I2 workflow passed at the pinned start cut:

- `project-loci`: exit 0; four loci, 13 fragments, 12 generated communication edges.
- `run-local`: exit 0; `status = ok`.
- `inspect`: exit 0; `status = ok`.
- `conform-i2`: exit 0; `status = accepted`; exactly 22/22 rows passed, including 21 runtime-monitored rows and one bounded-model row.

The four runs completed in approximately 0.48 s, 0.78 s, 0.78 s, and 1.66 s. Logs were retained outside the repository under `/tmp/mir-i2-regression-aNPnIk` for this session only. Canon index generation/check reported 193 indexed files.

Fresh close validation on the integrated candidate produced:

- `cargo test -p mir-runtime --lib sys6_i2_conformance_tests`: 25 passed, 0 failed.
- `cargo test -p mir-runtime --test sys6_i2_cli`: 8 passed, 0 failed.
- `cargo test -p mir-runtime --test m10_conformance`: 67 passed, 0 failed.
- `cargo test -p mir-runtime --test m10_cli`: 4 passed, 0 failed.
- Canon index generation/check: 193 files; index unit tests: 5 passed.
- Source hierarchy: 800 required, 800 present, 0 missing.
- `python3 scripts/validate_docs.py`: exit 0; documentation scaffold complete; 1,754 numbered reports found.
- HTML reader regression: 8 passed; its first red run detected three stale-state expectations before the test/current reader cut was corrected.
- Agent configuration validation: pass; configuration unit tests: 9 passed; strict Codex config/help: exit 0.
- `make docs`: exit 0 after agent config, Canon index, source hierarchy, and full docs validation.
- `git diff --check`: exit 0. Start-cut ancestry checks for M10 and accepted SYS-6: exit 0.
- Scoped token/private-key/webhook-pattern scan over the tracked diff and all new files: no matches.

The first authoritative docs-validator attempts rejected Report 2600's noncanonical heading/declaration and missing source-path forms. Those were corrected; only the later complete exit-0 run is counted as passing. Duplicate long validator processes accidentally started by the eval runner were terminated while one authoritative run was retained; they performed read-only scans and changed no file.

## What changed in understanding

The accepted I2 direction is aligned with the North Star: ordinary source produces checked global meaning, per-locus artifacts, generated communication, in-process dispatch, and typed devtools. The next direct technical gap is a checked network boundary, not a new semantic vocabulary.

The highest implementation concentration is `sys4_dispatch.rs`, followed by SYS-5 workflow and SYS-6 conformance. SYS-6 must remain downstream-only. Existing alpha TCP/provider paths are not on the accepted SYS-1--SYS-6 path and cannot be promoted by naming alone.

The complete ALIGN-0 drift control matrix is maintained in Plan 250. Its decisive distinctions are: Canon versus LAB authority; program activation versus lifecycle acceptance; semantic/request identity versus process/session/certificate identity; internal carrier versus private provisional encoding versus future public wire; Mir order versus stream order; typed observation versus renderer output; I3 in-flight cut safety versus deferred I4 durability; and Mirrorea versus Browser/Host, Shared-Space, upper applications, and the two satellites.

## Open questions

- OPEN-032 remains unresolved: TLS-over-TCP framed reliable stream and QUIC reliable stream remain unselected until equal executable I3-0 canaries are reviewed.
- Theory T1 and broad PHASE-I1 residuals remain unchanged.
- Package format, public wire/API/ABI, sandbox implementation, final provider ABI, Shared-Space vocabulary/governance, and Reversed Library product design remain intentionally deferred.

None of these is an ALIGN-0 blocker.

## Suggested next prompt

Continue autonomously with ALIGN-1: add the separate Canon project/product-layer map while preserving existing semantic strata and lifecycle meanings, validate that it changes no production/runtime behavior, independently review it, then close and advance to ALIGN-2.

## Plan update status

更新済み: Plan 250 is the sole current roadmap; Plan 249 remains a closed immutable baseline. `plan/00-index.md` points readers to Plan 250 without making LAB normative.

## Documentation.md update status

更新済み: current bounded program, active ALIGN-0 state, and official-I3-inactive distinction are synchronized.

## docs/project-status.md update status

更新済み: the same current frontier and non-claims are synchronized.

## progress.md update status

更新済み: the current LAB snapshot includes the three independent axes, active goal/readiness, feature evidence status, and timestamped close work log.

## tasks.md update status

更新済み: the current task map records the fixed autonomous milestone chain, delegated transport question, owner-reserved stop conditions, current promoted line, and rough phase position.

## samples_progress.md update status

更新不要: ALIGN-0 changes no runnable sample, validation command, debug surface, or sample blocker; accepted I2 sample evidence and `samples_progress.md` remain unchanged.

## Reviewer findings and follow-up

Pre-edit planner and independent semantic review found no P0 blocker. Their P1 requirements were incorporated: I3-0 needs symmetric executable two-process canaries; I3-3 binds every applicable plan/05 failure family; and I3-4 emits minimum observer-safe gate evidence before I3-5 joins it. Plan 250 permits a no-normative-source-delta audit path only when the complete required cut already exists; the current ALIGN-0 inventory instead identifies the S-axis and BND-007 gaps that ALIGN-1/2 must close. NEXT-0 keeps I4 and I5 as separate inactive contracts.

The Oracle independently reinforced the activation/lifecycle split, axis independence, early failure/order constraints, and explicit admission gates. Its suggestion to delay transport selection until after I3-6 was not adopted because it conflicts with the owner-fixed I3-0 sequence; local Canon review instead requires equal executable I3-0 canaries and reserves full C-distributed gates for I3-4. The Oracle run completed as GPT-5.6 Sol, but its metadata did not verify the UI-selected Pro setting, so no stronger claim is made.

The independent close planner initially found missing exact owner-column drift coverage, durable module/Host-View inventories, stale reader pointers, an over-restrictive milestone-addition rule, and an overbroad validation floor. The independent semantic/security reviewer found the same stale activation/lifecycle language, missing negative HTML assertions, wrong Canon-index command working directory, and P2-overstrict review gates. All P0/P1/P2 findings were corrected. Narrow re-review and final planner review both returned GO/ACCEPT with no residual P0/P1/P2.

## Skipped validations and reasons

Full workspace and exhaustive SYS-2--SYS-5 suites, Rust format/Clippy, Lean proof builds, bounded-model expansion, transport tests, fuzzing, and multi-process tests were not run because ALIGN-0 changes no production, formal, transport, model, sample, or runtime contract. The fresh SYS-6/M10 focused floor and relevant documentation/config regressions were run instead. None of the skipped suites is reported as passing.

## Commit / push status

Final validation and independent close review are complete. The candidate remains uncommitted at this report snapshot because an integration commit cannot embed its own future hash. The parent must commit with `--no-gpg-sign`, push to `origin/main`, verify parity, then update this section and advance the sole active goal before declaring ALIGN-0 closed.

## Sub-agent session close status

Completed bounded sessions: accepted-path code mapping, focused I2 regression, pre-edit planning, Plan 250 writing, scope review, status synchronization, HTML regression test authoring, command validation, Oracle consultation, independent semantic/security close review and narrow re-review, and final Canon-first planner review. All returned or were explicitly closed; no sub-agent result was treated as normative without parent integration against repository evidence.
