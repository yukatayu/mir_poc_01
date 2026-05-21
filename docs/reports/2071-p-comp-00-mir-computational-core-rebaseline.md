# Report 2071 — P-COMP-00 Mir computational core recognition rebaseline

## Objective

Read `sub-agent-pro/mirrorea_mir_computational_core_handoff.md` carefully, analyze the drift in target definition, and rewrite specs / plans / snapshot docs without implementing runtime behavior.

## Scope and assumptions

Scope was docs/spec/planning only. Product Alpha-1 operational workflow remains preserved. Current typed external `AddOne` is treated as host-boundary evidence, not Mir-owned computation. Planned computational, PoseGraph, projection, and engine-adapter samples remain planned-only until a later implementation package adds real sample roots, helpers, and positive / negative evidence.

## Start state / dirty state

Started on `main` tracking `origin/main`. Initial dirty state was one untracked user handoff file: `sub-agent-pro/mirrorea_mir_computational_core_handoff.md`.

## Documents consulted

Read the named handoff first, then the repository standard docs and relevant subsystem docs: `README.md`, `Documentation.md`, `AGENTS.md`, `progress.md`, `.docs/progress-task-axes.md`, `tasks.md`, `samples_progress.md`, `specs/00..03`, `specs/04`, `specs/05`, `specs/07`, `specs/09`, `specs/18..27`, `plan/00`, `plan/20`, `plan/23`, `plan/25`, `plan/44..52`, docs/hands_on and docs/research_abstract indexes, samples and scripts indexes, and validation scripts.

## Actions taken

Added normative specs for Mir computational core, Transform / PoseGraph, projection/backend boundary, and engine/WASM/FFI adapter boundary. Added roadmap memory for `P-COMP`, `P-POSE`, `P-PROJ`, and `P-ENG`. Reframed front-door and snapshot docs so Product Alpha-1 remains runnable but no longer stands in for Mir-owned computation. Reconciled AddOne wording across host-I/O docs as adapter-boundary evidence. Added docs-first hands-on and research summaries. Updated validators so the new hierarchy is checked.

## Files changed

Added `specs/28..31`, `plan/53..56`, `docs/hands_on/mir_computational_core_01.md`, `docs/hands_on/transform_posegraph_01.md`, and `docs/research_abstract/mir_computational_core_01.md`.

Updated root snapshots, indexes, and consistency docs: `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `specs/00`, `specs/03`, `specs/20`, `specs/22`, `specs/23`, `specs/24`, `specs/26`, `plan/00`, `plan/44`, `plan/45`, `plan/47`, `plan/48`, `plan/49`, `plan/51`, `plan/52`, `samples/README.md`, `scripts/README.md`, docs indexes, and validator scripts/tests. Added the user handoff file to the checked hierarchy.

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
date '+%Y-%m-%d %H:%M %Z'
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

Sub-agent review commands were run through the agent interface and all six assigned reviewers returned.

## Evidence / outputs / test results

`python3 -m unittest scripts.tests.test_validate_docs` passed: 13 tests.

`python3 scripts/check_source_hierarchy.py` passed: 167 required paths present, 0 missing.

`python3 scripts/validate_docs.py` passed before report addition and after report addition. Final rerun reported documentation scaffold complete and 1223 numbered reports.

`cargo fmt --check` passed.

`git diff --check` passed.

After adding this report, the full validation floor listed above was rerun before commit.

## What changed in understanding

The main drift was architectural: Product Alpha-1 is usable as a controlled alpha workflow, but it does not prove Mir-owned computation. The current `AddOne` lane is a typed external adapter lane. The next self-driven docs/spec line is therefore `P-COMP-01`, not broader distribution hardening. Broader distribution and final catalog breadth remain user-spec-required gates, but they are no longer the only repo-wide next reopen point.

## Open questions

Exact final textual `.mir` grammar remains open. Exact computational sample root shape remains planned. PoseGraph runtime carrier, projection inventory helper, and engine adapter validation helper remain future implementation work. Final distribution / final shared-space catalog breadth remains a user decision.

## Suggested next prompt

Proceed with `P-COMP-01`: create the planned computational sample matrix and scaffold, still without overclaiming execution, then prepare `P-COMP-02` for pure AddOne in Mir.

## Plan update status

Updated `plan/00`, `plan/44`, `plan/45`, `plan/47`, `plan/48`, `plan/49`, `plan/51`, and `plan/52`. Added `plan/53`, `plan/54`, `plan/55`, and `plan/56`.

## Documentation.md update status

Updated. It now distinguishes current operational floor, computational-core rebaseline, and non-claims.

## progress.md update status

Updated. Current promoted reopen point is now `P-COMP-01`; Product Alpha-1 remains controlled-alpha usable; computational / PoseGraph samples are planned-only.

## tasks.md update status

Updated. The task map now lists `P-COMP-01..04`, `P-POSE-01..02`, `P-PROJ-01`, and `P-ENG-01` as the self-driven docs/spec and later implementation line, while final distribution remains a user decision item.

## samples_progress.md update status

Updated. Added boundary-fixed / planned-only rows for computational core, PoseGraph, projection/backend, and engine adapter. No new runnable sample root or helper was claimed.

## Reviewer findings and follow-up

Theory reviewer: required explicit failure row, no global layer renumbering, AddOne non-claim, pure/effect split, array bounds split, continuation stop line, and save/load carrier integration. Reflected in `specs/28`, `specs/03`, `specs/20`, `specs/23`, `plan/53`.

Runtime/toolchain reviewer: Product Alpha floor stays preserved, but promoted docs/spec line moves to computational core. Reflected in `README.md`, `Documentation.md`, `progress.md`, `tasks.md`, `plan/51`.

Projection/backend reviewer: projection/backend docs must be inventory/boundary, not codegen. Reflected in `specs/30`, `specs/31`, `plan/55`, `plan/56`.

PoseGraph reviewer: pose snapshot, anchor graph, fallback admissibility, save/load, and devtools hooks need explicit future carriers. Reflected in `specs/29`, `plan/54`, `specs/20`, `specs/22`, `plan/47`.

Docs reviewer: source hierarchy needed `specs/28..31`, `plan/53..56`, docs indexes, and AddOne wording reconciliation. Reflected across indexes and validators.

Completion-gate reviewer: future packages need positive and negative evidence and must remain planned-only until real helpers exist. Reflected in `plan/53..56` and `samples_progress.md`.

## Skipped validations and reasons

Runtime/product validation commands were not rerun because this package made no implementation, sample, or CLI behavior changes. Future planned commands for `mir_computational_samples.py`, `posegraph_samples.py`, `projection_boundary_samples.py`, and `engine_adapter_boundary_samples.py` were not run because those helpers do not exist yet.

## Commit / push status

Pending at report authoring time. This report and related docs are intended to be committed with `git commit --no-gpg-sign` and pushed from `main` after final validation.

## Sub-agent session close status

Six sub-agent reviewers completed and were closed: theory/core invariants, Product Alpha/runtime architecture, projection/backend boundary, Transform/PoseGraph, docs/source hierarchy, and completion gate/package line.
