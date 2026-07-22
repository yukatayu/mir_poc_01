# Report 2394 - Full System V1 release revalidation and hierarchy sync

- Date: 2026-07-23 07:22 JST
- Author / agent: Codex
- Scope: existing Full System V1 execution audit and numbered-plan validator
  registration repair
- Decision levels touched: none; LAB validation and maintenance only

## Objective

Reproduce the existing bounded textual Mir and Full System V1 workflows on the
current repository state, diagnose any release-check failure, and repair only
the validation registration defect required for the documented hierarchy.

## Scope and assumptions

`mirrorea_canon/` remains normative. This package does not select grammar,
Core semantics, an OBL/THM status, a Gate/Phase, a contract, or a public API.
The Full System V1 workflow remains bounded LAB evidence; its existing
non-claims remain in force.

## Start state / dirty state

Started clean and synchronized at `0d438a40`. The root filesystem had 6.8 GiB
free (97% used), with an existing 2.7 GiB repository `target/` directory.
The external workdir was unavailable, so release bundles were placed in two
explicitly named `/tmp` directories and not committed.

## Documents consulted

Read Canon README and MAP; `plan/158`, `plan/161`, and `plan/166`; the root
README; `docs/hands_on/mir_computational_core_01.md`; and
`docs/hands_on/full_system_v1_roadmap_01.md`. Read the Makefile, workspace and
core crate manifests, Full System V1 release-check implementation, the
documentation validator, source-hierarchy checker, and their validator tests.

## Actions taken

1. Audited storage and the Rust workspace before running existing workflows.
2. Reproduced the repository check, textual parser suite, and Full System V1
   checker/runtime/operational suite.
3. Ran the full 29-command release workflow into a named temporary bundle.
4. Diagnosed its 26/29 failure as a mismatch between the two existing
   numbered-plan registration lists.
5. Added the missing `plan/181` source-hierarchy entry, ran targeted equality
   tests and documentation validation, then reran the full release workflow
   into a fresh temporary bundle.

## Files changed

- `scripts/check_source_hierarchy.py`
- `progress.md`
- this report

## Commands run

- `make check`
- `python3 scripts/textual_mir_samples.py check-all --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- `python3 scripts/full_system_v1_release_check.py --format json check-all`
  twice, each with a different named `/tmp` output directory
- `python3 -m unittest -v scripts.tests.test_validate_docs`
- focused numbered-plan equality unit tests
- `make docs`, `git diff --check`, and targeted storage/process inspections

## Evidence / outputs / test results

`make check` passed the 97-file Canon index, the source hierarchy, the
documentation scaffold, and `cargo check`. The textual-Mir alpha suite passed
10/10: two accepted source files and eight expected parser/checker failures
with stable diagnostics. The Full System V1 suite passed 50/50 rows: 21 typed
checker rows, 17 runtime rows, and 12 source-first operational rows.

The first release bundle ran 29 commands and passed 26. Its only root failure
was `validation:test-validate-docs`; this made the nested Product Alpha and
operational compatibility checks fail. The relevant test compares the numbered
plan set in `scripts/validate_docs.py` with the parallel
`scripts/check_source_hierarchy.py` list. `plan/181` existed only in the former.

After the one-line synchronization, both equality tests and `make docs` passed
with 731 required source-hierarchy paths. The fresh release bundle at
`/tmp/mirrorea-full-v1-release-fixed-20260723` reports `accepted`, 29 planned
commands, 29 passed commands, no failures, compatibility floor preserved, and
a non-final static HTML viewer with 29 per-command report files.

This validates the bounded source-first workflow only. It does not establish a
final public grammar or typed-IR/runtime API, C-distributed conformance, real
multi-process transport, final packet/FFI semantics, arbitrary native/WASM
execution, a final provider SDK, a final public devtools family, WAN/federation,
or durable distributed save/load.

## What changed in understanding

The repository already has a materially runnable bounded layer: textual `.mir`
parsing with source spans and expected negatives, typed checking, bounded
runtime execution, and source-first operational cases. The important limit is
not absence of a working parser/checker/runtime floor, but that these are
explicitly non-final LAB workflows. The release workflow also showed that
numbered plans are governed by two maintained lists; changing one must update
both or the unit suite correctly fails.

## Open questions

- The release workflow takes roughly 50 minutes because nested compatibility
  checks each run the extensive validator suite. Its cost is observed evidence,
  not a new performance defect or a reason to reduce validation coverage.
- Temporary release bundles now occupy about 246 MiB in `/tmp`. They are known
  disposable output, but no cleanup was performed in this package.
- The owner/canon decisions recorded in `tasks.md` remain unchanged.

## Suggested next prompt

Treat the accepted Full System V1 bundle as the present execution baseline.
Continue autonomous work by selecting a non-reserved gap that moves a bounded
workflow forward, while leaving final grammar, public APIs, transport, and
conformance decisions to the Canon process.

## Plan update status

`plan/` 更新不要: no semantics, sample classification, helper stack, roadmap,
or promoted workflow claim changed; this report records fresh bounded evidence
and the validator registration repair.

## Documentation.md update status

`Documentation.md` 更新不要: the reader-facing map already describes the
bounded Full System V1 surface accurately.

## docs/project-status.md update status

更新不要: the Canon lifecycle and compact status classification did not change.

## progress.md update status

更新済み: the recent log records the failed first run, root cause, repair, and
29/29 bounded release revalidation without widening the status claim.

## tasks.md update status

`tasks.md` 更新不要: no self-driven package, discovery item, or owner decision
changed.

## samples_progress.md update status

`samples_progress.md` 更新不要: no sample source, validation command, or
dashboard classification changed.

## Reviewer findings and follow-up

Local root-cause tracing identified the exact missing list entry. The initial
release failure was not treated as a runtime regression. The focused equality
tests passed after the one-line repair, and the fresh complete bundle supplied
the integration evidence. A temporary Oracle final review found no must-fix
scope, factual, authority, validation, or report issue. It confirmed the two
validator lists contain the same plan 181 entry, while correctly noting that
its review did not independently receive the generated bundle; the 29/29 claim
therefore remains grounded in the locally inspected `bundle.json` and
per-command reports.

## Skipped validations and reasons

No new feature implementation, grammar change, Lean experiment, distributed
runtime run, or final-conformance test was attempted because this package
validates and repairs the existing bounded workflow only. The full release
workflow, repository check, parser/checker/runtime/operational suites, targeted
unit tests, and documentation validation were run.

## Commit / push status

Pending at report write. The hierarchy repair, progress log, and report will
be validated, committed with `--no-gpg-sign`, and pushed.

## Sub-agent session close status

No new task-scoped sub-agent session was started. The temporary Oracle final
review completed and was recorded only as advisory review evidence; it did not
independently execute or receive the generated release bundle.
