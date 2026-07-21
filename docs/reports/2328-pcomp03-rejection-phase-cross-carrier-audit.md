# Report 2328 - P-COMP-03 rejection-phase cross-carrier audit

- Date: 2026-07-22 04:29 JST
- Author / agent: Codex
- Scope: existing P-COMP-03 helper, semantic registry, and Product Alpha
  rejection-phase evidence classification
- Decision levels touched: none; Canon and implementation are read-only

## Objective

Correct the phase classification of P-COMP-03 negative computational evidence
without widening its helper, package schema, runtime, public interface, or
Canon meaning.

## Scope and assumptions

`mirrorea_canon/` is normative. In particular, Canon theory says static errors
precede dynamic failure and must not be folded into `Reject`. This package
audits only current LAB evidence. It does not assert that the Product Alpha
error carrier conforms to Canon, and it does not choose a final diagnostic,
failure, effect, or package interface.

The standing boundary in ADR-0014 allows existing-lane scoped research and
bounded validation. The package is documentation and evidence classification
only; it adds no helper, schema, CI, Make target, runtime behavior, or public
surface.

## Start state / dirty state

Started from clean pushed `main` at `0cb6d615` after report 2327's directness
audit. No user changes were present, reverted, or overwritten. The initial
red documentation-registration test was introduced solely by this package's
new numbered `plan/167` file.

## Documents consulted

- Canon README/MAP, theory/01, theory/02, spec/03, ADR-0014, and plan/01/02.
- `specs/28`, `plan/53`, `plan/166`, the computational matrix, package
  manifests, and Product Alpha sample documentation.
- `scripts/mir_computational_samples.py`, computational semantic-core source
  and tests, Product Alpha session source and tests.
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, `plan/00-index.md`, and Oracle operating notes.

## Actions taken

1. Replayed the matrix and the full helper check, then ran focused semantic and
   Product Alpha runtime suites through a read-only validation sub-agent.
2. Mapped each negative module from checked-in helper fixture to the closed
   Rust semantic registry and Product Alpha error wrapper using a separate
   read-only source sub-agent.
3. Verified that P-COMP-03 fixture execution calls the Python dispatcher
   directly, while constructed valid Product Alpha packages reach the Rust
   typechecker/evaluator route.
4. Recorded the four-static / one-evaluation split and explicitly superseded
   the over-broad report-2327 phrase through this successor record.
5. Registered `plan/167` in both documentation guard lists and synchronized
   the current LAB snapshots.

## Files changed

- `plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`
- `plan/00-index.md`
- `plan/53-mir-computational-core-roadmap.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- this report

## Commands run

- `df -h .` and `free -h`
- `python3 scripts/mir_computational_samples.py matrix --format json`
- `python3 scripts/mir_computational_samples.py check-all --format json`
- `cargo test -p mir-semantics --test mir_computational_core -- --nocapture`
- focused positive and negative P-COMP-03 Product Alpha runtime test commands
- `python3 -m unittest scripts.tests.test_validate_docs.ValidateDocsTests.test_all_repo_numbered_plan_files_are_registered`
  before and after registration
- final focused documentation and source-hierarchy checks listed below

## Evidence / outputs / test results

The matrix remained 15 executable rows: 7 accepted, 5 helper-labelled
`runtime_rejection`, and 3 expected package-check rejections. `check-all` had
no failures. The semantic suite passed 4 tests; focused Product Alpha runtime
tests separately passed the five-positive and five-negative constructed-package
registry cases.

The source path establishes the following split:

- scope/use-before-declare: `UnboundVariable` from typechecking;
- Vec3 unknown field: `UnknownField` from typechecking;
- non-Bool condition: `TypeMismatch` from typechecking;
- missing import/function: `UnknownFunction` from typechecking; and
- array bounds: `OutOfBounds` from evaluator execution after typechecking.

Product Alpha first invokes `typecheck_module`, then `eval_function`; the
latter typechecks again before `eval_function_impl`. Thus the four static
cases do not enter evaluator execution on the Product Alpha route, while the
array case does. Product Alpha wraps all five errors as `MirCompute` at
`<runtime_input.mir_compute>`. The checked-in helper fixture route does not
invoke that Rust path, so its common `runtime_rejection` category is not a
phase result.

The initial registration test failed exactly because plan/167 was absent from
the two required registries. It passes after the matching entries were added.

## What changed in understanding

Report 2327's separation of helper fixtures from constructed-package runtime
tests remains correct. Its assertion that every negative P-COMP-03 module was
rejected before evaluator execution was not. The accurate LAB description is a
four-static / one-evaluation split, carried without a public phase distinction
by the current Product Alpha error wrapper.

This does not prove Canon conformance: current helper labels, computational
core errors, and Product Alpha `MirCompute` are implementation evidence, not
the Canon static-diagnostic or dynamic-failure contract.

## Open questions

- Whether a future LAB fixture should expose a rejection-phase field, and if
  so which existing carrier should own it, remains unselected.
- How a future Mir implementation should present static diagnostics versus
  structured dynamic failures remains a Canon/toolchain design question.

## Suggested next prompt

Continue autonomous research by selecting the next non-reserved question with
new discriminating evidence, while treating a public or fixture-visible
rejection-phase carrier as a separate design package.

## Plan update status

`plan/` 更新済み: added and indexed plan/167, and clarified the phase split in
the existing computational roadmap plan/53.

## Documentation.md update status

`Documentation.md` 更新済み: distinguishes helper comparison labels from the
closed-registry static/evaluation split.

## docs/project-status.md update status

更新済み: the current computational evidence row now states the four-static /
one-evaluation result and its carrier limit.

## progress.md update status

`progress.md` 更新済み: adds the exact LAB correction without changing macro
phase, workflow readiness, or implementation status.

## tasks.md update status

`tasks.md` 更新済み: keeps the directness audit closed while recording the
phase distinction and its non-claim.

## samples_progress.md update status

`samples_progress.md` 更新済み: updates the computational dashboard and recent
validation log with the carrier-aware phase classification.

## Reviewer findings and follow-up

Read-only code mapper `Socrates` found the decisive correction: the five
checked-in fixture paths are helper-only, but the closed Rust registry has four
typecheck rejections and one evaluator `OutOfBounds` rejection. Read-only
validation runner `Hilbert` reproduced the matrix, helper check, semantic
suite, and focused positive/negative Product Alpha runtime tests successfully.
A prior temporary Oracle consultation had recommended this phase-and-provenance
audit; its advisory output is not committed. Final reviewer `Bohr` found one
P1 documentation defect: `docs/project-status.md` and `tasks.md` still carried
the previous snapshot timestamp. All four current snapshot headers and their
two affected dated log entries (`progress.md` and `samples_progress.md`) were
synchronized to the observed task-close timestamp. Its narrow re-review found
no remaining issue.

## Skipped validations and reasons

No release workflow, Docker flow, workspace-wide suite, Lean replay, or heavy
backend build was rerun. This package changes no runtime behavior and the
focused computational paths plus full documentation/source-hierarchy checks
are the relevant evidence. No new Oracle request was started because the prior
temporary recommendation is being directly checked against local source and
test evidence.

## Commit / push status

Pending at report write. The complete documentation package will be committed
with `git commit --no-gpg-sign` and pushed after final validation and review.

## Sub-agent session close status

`019f8619-c587-7632-ae66-eb8d17029f82` (`Socrates`) and
`019f8619-c611-7183-98fd-c02c33e5eaaa` (`Hilbert`) completed read-only work;
both were closed after their findings were incorporated. `Bohr` completed the
final review and narrow timestamp re-review; it will be closed at package
close. No sub-agent edited the workspace.
