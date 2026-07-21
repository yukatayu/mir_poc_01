# Report 2297 - minimal runnable Mir core feasibility

- Date: 2026-07-21 18:21 JST
- Author / agent: Codex
- Scope: Determine whether an actually runnable, minimally computational and type-checked Mir core can be advanced autonomously, without conflating LAB evidence with Canon implementation status.
- Decision levels touched: None. This is an evidence/readout report; it does not create a WRK, change L0/L1/L2/L3, move an OBL, exit a Gate/Phase, or promote implementation status.

## Objective

Separate technical feasibility of a minimal textual `.mir -> parse -> check -> run` path from the governance conditions for an official Mir implementation or I1 reference implementation.

## Scope and assumptions

`mirrorea_canon/` is authoritative. Existing Rust crates, CLI, scripts, `.mir` samples, and legacy plans are LAB evidence. A successful LAB command is not treated as C-static/C-runtime conformance, a proof, or a phase transition.

## Start state / dirty state

Started from pushed, clean `main` at `cf65821f`. The prior cleanup had removed generated Rust output. Read-only/sample validation recreated ignored `target/` output, measured at 764 MiB; no tracked source was changed.

## Documents consulted

- `CANON.md`, `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and `mirrorea_canon/plan/01-phases.md` / `02-operating-model.md`.
- `mirrorea_canon/architecture/03-toolchain.md`, `04-carriers.md`, and `mirrorea_canon/spec/02-surface-grammar.md` through `06-conformance.md`.
- `mirrorea_canon/working/README.md`, `progress.md`, `tasks.md`, `docs/project-status.md`, and `samples_progress.md`.
- LAB `specs/28-mir-computational-core.md`, `specs/33-full-system-v1-scope.md`, and `plan/53`, `plan/57`, `plan/58`, and `plan/59`.
- Oracle temporary review `mir-minimal-core-feasibilit-20260721` (advisory only; no external transcript is committed).

## Actions taken

- Compared the normative T0/I1 boundaries with the existing Rust parser, semantics, runtime, CLI, Full System V1, and computational evidence lanes.
- Executed the build-independent computational matrix and current textual/Surface sample checks.
- Inspected the runner paths and Rust module layout to distinguish hardcoded helper rows from the source-first parser/checker/interpreter path.
- Requested an independent Oracle review of technical feasibility, governance blockers, smallest honest scope, and a minimal staged plan.

## Files changed

- `docs/reports/2297-minimal-runnable-mir-core-feasibility.md`

## Commands run

- `python3 scripts/mir_computational_samples.py check-all --format json`.
- `python3 scripts/textual_mir_samples.py check-all --format json`.
- `python3 scripts/surface_mir_samples.py check-all --format json`.
- Read-only source/module and plan inspection with `rg` / `sed`.
- `df -h`, `du -sh target /tmp`, and Git status.
- One Oracle temporary consultation with the relevant Canon and LAB documents attached.

## Evidence / outputs / test results

- The computational matrix passed all 15 rows: 7 accepted, 5 expected runtime rejections, and 3 expected check rejections. Its own output says `workflow_ready: false`; it is bounded LAB evidence.
- The current textual Mir and Surface sample checks exited successfully. Their runners invoke Cargo-backed parser/checker paths; the validation recreated 764 MiB of ignored `target/` output. No direct broad Cargo suite was rerun after the storage cleanup.
- Existing Rust modules include `mir-ast::textual_alpha`, `mir-semantics::full_system_v1::{checker, typed_ir, interpreter}`, `mir-runtime::full_system_v1_session`, and the parser-free computational-core modules. The Full System V1 route is source-first LAB evidence, not Canon implementation state.
- Canon states that the official implementation phase is T0, with T0--T2 implementation freeze except scoped LAB research artifacts / bounded implementation validation. I1 requires its own C-static/C-runtime conformance criteria.
- Oracle's advisory conclusion agrees with the local evidence: a narrow textual research core is technically feasible, but an official Mir implementation, I1 entry, phase movement, or conformance claim is not autonomously available under current Canon.

## What changed in understanding

The project is not technically waiting for a greenfield parser, typechecker, or interpreter. A bounded source-first LAB implementation already exists and provides a credible substrate. The missing integration is not simply more code: a minimal profile must use only the intersection of Canon syntax/semantics and existing implementation, preserve checker-before-run and span diagnostics, and retain explicit non-claims.

The smallest autonomous target is therefore a LAB-only, deterministic, single-locus, pure textual profile with a positive arithmetic/branching program and parse/name/type negative programs. It may not silently import LAB-only `fn`, `mut`, loop, array, enum, entry, result, host-I/O, Core IR, or ABI choices into Canon.

## Open questions

- Whether the owner wants the next autonomous package to prioritize this executable research-core integration over the currently listed OBL-021 eligibility assessment.
- Whether a future owner/canon action should define a bootstrap route by which validated pre-I1 LAB implementation evidence can be accepted or reclassified toward I1.
- Which, if any, computational surface features beyond the Canon/existing-code intersection are desired before a future official language claim.
- L2 promotion remains fail-closed without an owner-authenticated trust anchor.

## Suggested next prompt

Authorize the minimal textual research-core package, or keep the current OBL-021 package first. If authorized, first perform standing-eligibility triage and commit a separate L3 WRK pre-registration before editing source or running outcome commands.

## Plan update status

更新不要: no new owner decision, research outcome, or roadmap selection was made.

## Documentation.md update status

更新不要: the reader route remains accurate.

## docs/project-status.md update status

更新不要: its distinction between bounded runnable LAB evidence and T0/G0 official status agrees with this readout.

## progress.md update status

更新不要: feasibility was clarified, but workflow readiness and the current selected package did not change.

## tasks.md update status

更新不要: OBL-021 eligibility remains the selected next package until the owner changes priority.

## samples_progress.md update status

更新不要: no sample status or validation contract changed.

## Reviewer findings and follow-up

Oracle was used as an advisory architecture reviewer. Its key finding was a bootstrap-policy tension: LAB implementation evidence may be created in T0, but accepting it as an I1 mainline implementation requires owner/canon action. The advisory result is consistent with local Canon and source evidence; it does not decide any owner-reserved item.

## Skipped validations and reasons

No broad Cargo test suite, direct CLI runtime command, or clean-worktree test was rerun after the prior `cargo clean`. The focused runners already recreated 764 MiB of build output while root capacity is 12 GiB and the configured external workdir remains unmounted. Further heavy validation is deferred until a specifically selected implementation/research package warrants it.

## Commit / push status

This readout is committed and pushed after documentation validation as the task closeout.

## Sub-agent session close status

No sub-agent was used. One temporary Oracle consultation completed and is distilled above; its local temporary output is not repository state.
