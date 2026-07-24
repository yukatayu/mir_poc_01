# Report 2415 - Parser-free chain closure integrity

## Objective

Determine whether the post-PROPOSAL-012 theory frontier contains a safe
autonomous package, and repair any independently found existing-lane LAB defect
without selecting a new Mir semantic family.

## Scope and assumptions

`mirrorea_canon/` is normative. Owner-reserved PROPOSAL-008, PROPOSAL-009, and
PROPOSAL-012 questions remain unselected. The implementation scope is limited
to the parser-free current-L2 fallback-chain representation and its static
gate. No Core primitive, chain algebra, grammar, event carrier, OBL, Gate,
Phase, scenario, public API, or sample workflow is selected or changed.

## Start state / dirty state

The worktree was clean at `254d06de`, equal to `origin/main`. The task-scoped
Discord baseline was recorded before substantive work. No user changes were
present.

## Documents consulted

- Canon: README, MAP, ADR-0014, theory/00 through theory/11, spec/02, spec/04,
  spec/05, plan/00-gates, plan/01-phases, and PROPOSAL-008 through
  PROPOSAL-012.
- LAB: README, Documentation.md, `docs/project-status.md`, `progress.md`,
  `tasks.md`, `samples_progress.md`, Plans 180 through 187, and Plan 184's
  reopening criteria.
- Implementation: `crates/mir-semantics/src/lib.rs`,
  `crates/mir-semantics/src/harness.rs`,
  `crates/mir-ast/src/current_l2.rs`, and current-L2 semantics tests.
- Advisory input: an independent planner, an independent code reviewer, a
  final narrow reviewer, and a temporary Oracle review.

## Actions taken

1. Re-read the Canon and current LAB control documents, then classified the
   existing owner-reserved theory boundaries without selecting any of them.
2. Screened theory/implementation seams with independent planner and reviewer
   passes.
3. Traced parser-free `ChainDecl` data from static validation through evaluator
   order construction and its fixture-bundle consumers.
4. Reproduced the disconnected-edge defect with a minimal synthetic fixture:
   `head = primary`, `edges = [mirror -> archive]`.
5. Added a red regression expecting `Malformed`; it failed because the former
   static verdict was `Valid`.
6. Added only predecessor-continuity validation after head and endpoint
   resolution, preserving existing missing-head and missing-endpoint behavior.
7. Kept both new continuity findings outside the existing stable reason-code
   carrier and regression-tested that detached artifacts omit `detached_noncore`.
8. Follow-up review found that an unresolved endpoint left the expected
   predecessor stale; added a red/green regression and advanced it from every
   raw edge successor without checking continuity until both endpoints resolve.
9. Added Plan 188 and synchronized the human control snapshots.

## Files changed

- `crates/mir-semantics/src/lib.rs`
- `crates/mir-semantics/tests/current_l2_static_gate_support.rs`
- `plan/188-parser-free-chain-closure-integrity.md`
- `plan/00-index.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `docs/reports/2415-parser-free-chain-closure-integrity.md`

## Commands run

- ordered Canon/LAB/source reads and current-frontier inventory
- focused red/green:
  `cargo test -p mir-semantics --test current_l2_static_gate_support static_gate_rejects_a_chain_whose_first_edge_does_not_start_at_its_head -- --exact`
- `cargo fmt --check`
- `cargo test -p mir-semantics --test current_l2_static_gate_support`
- `cargo test -p mir-semantics --test current_l2_minimal_interpreter`
- `cargo test -p mir-semantics`
- `cargo check --workspace`
- `cargo test --workspace`
- missing-endpoint red/green:
  `cargo test -p mir-semantics --test current_l2_static_gate_support static_gate_does_not_add_continuity_to_missing_endpoint_diagnostics -- --exact`
- final narrow diff review and `git diff --check`
- `make check`
- `python3 -m unittest scripts.tests.test_validate_docs`
- resource check: `df -h .` and `free -h`

## Evidence / outputs / test results

Before the correction, the new regression failed with `left: Valid` and
`right: Malformed`. The root cause was structural: `static_gate_program_detailed`
validated every edge independently, but `resolve_chain_order` appended each
edge successor to the head and never consumed the predecessor. A disconnected
edge could therefore pass validation and have its predecessor silently ignored
by evaluation.

The corrected gate starts with the declared head and requires every resolved
edge predecessor to equal the expected predecessor; it then advances that
expected value to the edge successor. Red/green tests cover both an initial
disconnected edge and a later disconnected edge; their detached artifacts
retain absent `detached_noncore`, so the wording does not enlarge the stable
reason-code carrier. A third regression crosses an undeclared endpoint and
confirms that the later syntactically connected edge receives no additional
continuity reason. All 11
`current_l2_static_gate_support` tests pass, all 46
`current_l2_minimal_interpreter` tests pass, and the full `mir-semantics`
suite passed before the missing-endpoint follow-up; its final workspace rerun is
pending. The first post-review documentation run passed all 87 tests in
1109.860 seconds. A later closeout `make check` correctly rejected a 189-line
`docs/project-status.md` against its 180-line concise-view budget; the new
detail was the only overage and is folded into an existing table row. A second
documentation run passed all 87 tests in 1086.601 seconds; the final rerun
after this follow-up is pending. Existing source parsing already carries the
preceding option forward; the correction protects the parser-free
representation before fixture or source-runtime consumers reach evaluation.

## What changed in understanding

The broad theory frontier remains largely owner-reserved, but the code review
found a non-theory defect with a concrete existing consumer. The LAB
representation already stores ordered edges; checking their continuity is not a
choice of a new chain model. It preserves the Canon's settled finite,
left-to-right fallback-chain form and prevents the evaluator from silently
rewriting malformed input.

## Open questions

- PROPOSAL-008 outcome-totality interpretation and future obligation placement.
- PROPOSAL-009 direct-Core versus explicit output/Core correspondence.
- PROPOSAL-012 V/R/S/A value-flow and occurrence-identity dispositions.
- Whether future chain work needs a rule for repeated options, cycles, or
  strictness of degradation. This package intentionally does not infer one.

## Suggested next prompt

Record a PROPOSAL-012 disposition only when the owner wants a later design
package to cover that selected family. Otherwise, re-screen any newly observed
existing-lane discrepancy under ADR-0014 rather than reopening frozen routes.

## Plan update status

更新済み: Plan 188 records the bounded parser-free integrity correction, and
`plan/00-index.md` indexes it. No Canon plan or Gate/Phase text changed.

## Documentation.md update status

`Documentation.md` 更新不要: the entry-point reading order and high-level
project description remain current.

## docs/project-status.md update status

更新済み: the concise human control view now records the current parser-free
chain correction and its non-claims.

## progress.md update status

更新済み: the snapshot and dated recent log distinguish this existing-lane
integrity repair from a new theory/WRK result.

## tasks.md update status

更新済み: the task map records the corrected current-L2 chain closure and that
it is neither a new owner gate nor a new WRK.

## samples_progress.md update status

`samples_progress.md` 更新不要: no active sample path, runnable command, debug
surface, or evidence classification changed.

## Reviewer findings and follow-up

The planner initially found no eligible theory package from the document
frontier. The code reviewer then found the disconnected parser-free chain
input, its evaluator consequence, and the existing bundle/source-runtime
consumer. The final narrow reviewer confirmed the continuity condition but
found three closeout gaps: a project-status heading-rule violation, two omitted
validator manifests in this report, and no direct assertion that the new
unclassified wording leaves `detached_noncore` absent. All three were corrected
and revalidated. A follow-up reviewer then found that missing endpoints did not
advance the expected predecessor, which could add a spurious later continuity
diagnostic; the third regression and raw-successor advancement correct it. The
first temporary Oracle screen did not include the
subsequently inspected static-gate implementation, so its no-candidate
conclusion was not used for this code-local discovery. The narrow Oracle retry
independently classifies predecessor continuity as direct preservation of the
settled linear chain representation, asks for the later-edge regression added
here, and confirms that cycles, repeated options, graph analysis, diagnostic
schema, and runtime behavior remain outside this package.

## Skipped validations and reasons

No Lean proof or new executable semantic lane was created because the repair
uses the existing parser-free test lane and makes no theorem claim. Focused
red/green, full parser-free semantics, full workspace, formatting, and diff
checks ran. The final workspace and documentation reruns remain pending after
the missing-endpoint correction.

## Commit / push status

The implementation package will be committed with `--no-gpg-sign` and pushed
only after the final workspace and documentation reruns pass; its closeout
commit will record the resulting commit and push status.

## Sub-agent session close status

The planner, code reviewer, and final narrow reviewer completed and were
closed. No sub-agent edited repository files. Both temporary Oracle sessions
are closed: the first attachment attempt failed before submission, and the
single retry completed successfully.
