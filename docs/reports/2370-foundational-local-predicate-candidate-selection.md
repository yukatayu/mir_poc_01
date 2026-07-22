# Report 2370 - foundational local-predicate candidate selection

- Date: 2026-07-22 22:54 JST
- Author / agent: Codex
- Scope: LAB candidate revalidation and pre-registration planning
- Decision levels touched: none; no Canon theory, ledger, gate, phase, or L3 record change

## Objective

Select one non-duplicative, existing-lane foundational research candidate without turning a local helper proof into a MirCore or OBL result.

## Scope and assumptions

`mirrorea_canon/` remains normative. This package records selection only; it does not add a working record, edit Lean source, or run a candidate outcome command. Oracle and sub-agent conclusions are advisory evidence checked against the repository sources.

## Start state / dirty state

`main...origin/main` was clean at `3990dc18d9a2e9a2b37dc9997e795a3b579dba1c`, matching `origin/main`. The Discord task baseline had already been recorded.

## Documents consulted

Read Canon README/MAP, ADR-0014, working README, theory/01, theory/02, theory/06, theory/11, WRK-0001 and WRK-0014; LAB `plan/156`, `plan/158`, Report 2262, Report 2263, Report 2265, Report 2275, Report 2278, the finite index/label Lean foundations, their explanations, current snapshots, validator history, and Lean sample catalog.

## Actions taken

Mapped OBL-003..019 and OBL-026 against active Lean and prior LAB evidence. Compared OBL-005, OBL-015, and local predicate constructivity. Used an independent feasibility planner, a foundation mapper, an adversarial reviewer, and temporary Oracle review. Chose only the `captureSubset` all-input decision term as a future L3 experiment; retained `outlives` and `remoteCallAllowed` as controls and rejected the other dossiers.

## Files changed

- `plan/173-local-predicate-constructive-decidability-selection.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- this report

## Commands run

- targeted `git`, `rg`, `sed`, `nl`, and validator-history inspections
- temporary Oracle review `obl003-vs-obl005-l3-selection-20260722`
- read-only sub-agent source mapping, feasibility, and adversarial review
- `make docs` (Canon index, source hierarchy, and documentation validation)
- `git diff --check` and report-heading / project-status line-cap probe
- `python3 -m unittest scripts.tests.test_validate_docs`

## Evidence / outputs / test results

Report 2275 bounds only its positive LAB result to a finite failure-row subcheck and leaves the complete unified-judgment decidability boundary open, while WRK-0001 already reproduces the exact finite-index helper shapes. Report 2262 already supplies the structural OBL-005 output result. OBL-015's current Boolean helper has no identified consumer that treats it as Canon grant-lineage evidence. The selected `captureSubset` question is potentially distinct because it asks for a uniform constructive term over arbitrary functions on the exact two-constructor LAB carrier. No candidate Lean command ran in this package.

`make docs` passed: Canon index reported 94 files, source hierarchy reported
723 / 723 required paths, and documentation validation found 1,524 numbered
reports. The first full validator-test run found that the hierarchy registry
already omitted registered plan 172 and also omitted new plan 173. Adding both
paths to the same source-hierarchy scaffold fixed the root cause: the focused
two tests and a fresh full run both passed, with 87 tests in 586.016 seconds.
`git diff --check` and the explicit report-heading / 180-line project-status
probe also passed.

## What changed in understanding

The useful distinction is not simply whether a local theorem is new. A bounded experiment needs a distinct adversarial condition and a future decision it can constrain. `captureSubset` can test explicit closed-carrier elimination against an opaque-domain control; the other candidates either repeat recorded evidence or risk attaching a helper fact to an unrelated Canon obligation.

## Open questions

- Does the registered constructivity experiment compile without forbidden machinery or a new helper/API?
- If it does, can its evidence be retained without changing the active sample catalog or treating it as a runnable workflow?
- The validator lane-catalog correspondence remains unresolved but is not touched because `samples/lean` is already admitted.

## Suggested next prompt

Create and push a fresh bounded L3 pre-registration for local `captureSubset` constructive decidability, then execute only its registered evidence plan.

## Plan update status

Updated: plan 173 records sources, comparison, selected narrow question, falsifiers, non-claims, and pre-registration sequence; the plan index links it.

## Documentation.md update status

Updated: the reader map and current-position section link the selection and its non-OBL boundary.

## docs/project-status.md update status

更新済み: the control view separates this admitted Lean candidate from the unresolved lane catalog and from OBL-003 progress.

## progress.md update status

Updated: the logical readiness row and package 42 record the selected L3 route.

## tasks.md update status

Updated: package 42 is the current closed selection and states its stop line.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no sample, command, debug surface, or runnable workflow changed.

## Reviewer findings and follow-up

The adversarial reviewer found that a three-predicate OBL-003 framing would be duplicative and misleading. Oracle recommended proceeding only with `captureSubset`, non-instance evidence, positive controls, and an opaque-domain adverse probe. The narrower selection adopts both constraints. Final review found four correct issues: `plan` had to join `samples/lean` as a permitted location, retained evidence needed mandatory append-only manifestation, Report 2275 had to remain a bounded LAB result rather than an OBL-003 bound, and the project-status timestamp had to advance. All four were corrected; narrow re-review found no findings. Subsequent full-test evidence found and corrected the mechanically separate hierarchy scaffold omission for plans 172 and 173.

## Skipped validations and reasons

No new Lean theorem or outcome command ran because immutable pre-registration must precede evidence. Runtime, distributed, and product checks do not apply to this documentation/selection package. Documentation/source-hierarchy validation, the full validator unit suite, local diff review, final independent review, and narrow re-review passed.

## Commit / push status

Pending at report write. This package will be committed with `--no-gpg-sign` and pushed immediately after validation and review.

## Sub-agent session close status

The foundation mapper, OBL-005 feasibility planner, adversarial reviewer, final reviewer, and narrow re-reviewer completed read-only work and were closed. One interim retry reviewer was shut down without a result after the original final reviewer returned. The Oracle temporary chat completed; its external transcript remains outside repository state.
