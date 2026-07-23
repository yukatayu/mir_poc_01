# Report 2414 - Surface-to-Core form coverage recheck

## Objective

Determine whether Surface `let`, `if`, ordinary expressions, and compound
assignment expose a new autonomous theory candidate, or only restate the
existing value-flow, receipt, conditional-normalization, and outcome-totality
boundaries.

## Scope and assumptions

`mirrorea_canon/` is normative. This began as a read-only source recheck. Final
review found bounded LAB parser integrity defects, so the package also adds
fail-closed rejections for compound assignment, `let` / `if`, and equality. It
does not select a Core binder, substitution strategy, evaluation context, result
carrier, conditional normalization policy, remote receipt mapping, totality
reading, compound operation, operational rule, OBL, Gate, Phase, execution
semantics, or public contract.

## Start state / dirty state

The worktree was clean at `e6b7706d`, equal to `origin/main`. The task-scoped
Discord baseline was recorded before substantive work. No pre-existing user
changes were present.

## Documents consulted

- Canon: README, MAP, ADR-0014, spec/02, theory/01 through theory/03,
  PROPOSAL-008, PROPOSAL-009, and PROPOSAL-012.
- LAB: Plans 73, 145, 180, 182, 184, 186, and 187; `Documentation.md`,
  `docs/project-status.md`, `progress.md`, and `tasks.md`.
- Bounded implementation evidence: `crates/mir-ast/src/surface_alpha.rs`,
  `crates/mir-semantics/src/surface_to_core_elaboration.rs`, and
  `crates/mir-runtime/src/surface_source_patch_hotplug.rs`.
- Advisory input: one code mapper and one planner. Oracle was not used because
  the independent source reviews found the proposed audit duplicative before a
  further external consultation was warranted.

## Actions taken

1. Compared every named Surface expression/binder form with the stated Core
   grammar and key elaboration sketches.
2. Traced the bounded LAB Surface parser, elaborator, and patch consumer to
   distinguish unsupported LAB forms from Canon semantics.
3. Separated runtime-value flow from pure-expression lowering, successful read
   receipt, OPEN-012 join normalization, and PROPOSAL-008 outcome totality.
4. Rejected a new Plan, proposal, WRK, Lean model, or runtime experiment because
   no new non-reserved relation, permitted evidence path, and live binary
   consumer were established.
5. Updated the current task and progress snapshots with the concise disposition.
6. Investigated the review findings end to end: lexer tokens, assignment branch,
   brace-aware target collection, expression collection, elaboration entry point,
   and patch admission path.
7. Added red regressions for `+=` / `-=`, braced and nested compound target/RHS
   contexts, `let`, `if`, and direct/RHS/target `==`; each initially demonstrated
   the stated incorrect acceptance.
8. Made the parser reject each unlowered form before it can enter a checker or
   elaborator, and added a source-patch regression showing parse rejection stops
   typecheck, elaboration, admission, mutation, and activation.

## Files changed

- `tasks.md`
- `progress.md`
- `crates/mir-ast/src/surface_alpha.rs`
- `crates/mir-ast/tests/surface_mir_parser.rs`
- `crates/mir-runtime/tests/source_patch_hotplug.rs`
- `docs/reports/2414-surface-to-core-form-coverage-recheck.md`

## Commands run

- ordered Canon/LAB/source reads and line-numbered source extraction
- bounded Surface parser/elaborator/consumer source trace
- independent code-mapper and planner reviews
- focused parser red/green regressions for compound, statement, and equality
  dispatch
- `cargo test -p mir-ast --test surface_mir_parser`
- `cargo test -p mir-semantics --test surface_to_core_elaboration`
- `cargo test -p mir-runtime --test source_patch_hotplug`
- `cargo fmt --check`
- `cargo check --workspace`
- `cargo test --workspace`
- `make check`
- `python3 -m unittest scripts.tests.test_validate_docs`
- resource check: `df -h .` and `free -h`

## Evidence / outputs / test results

`theory/01` and Canon spec/02 name Surface `let`, `if`, expressions, equality,
and compound assignment, but give no dedicated current-LAB lowering for them.
They do state local/cross-locus reads and plain assignments. The bounded LAB
elaborator rejects actual `Raw` statements rather than silently dropping them,
but the parser had four defects:

- `+=` / `-=` could be collected into an ordinary assignment target;
- a brace-containing index could hide the outer assignment delimiter;
- `let` / `if` could be lexed as identifiers and reach assignment/raw dispatch;
- two `=` tokens could be read as assignment plus an equals-prefixed RHS.

The parser now rejects compound tokens in direct, braced, and nested target/RHS
contexts with `compound_assignment_not_supported`; `let` / `if` with
`unsupported_surface_statement`; and `==` in assignment target/RHS context with
`unsupported_surface_expression_operator`. The red tests each failed before
their corresponding correction and are green afterward. The complete `mir-ast`
parser suite (17 tests), `mir-semantics` elaboration suite (36 tests), and
source-patch suite (4 tests) pass. The source-patch row proves that a braced
compound form fails at parse and therefore has false typecheck, elaboration, and
admission stages, no runtime mutation, and no activation cut.

`cargo test --workspace` also passed across the repository's Rust unit,
integration, and documentation-test targets, including the existing ordinary
RHS dependency fixtures. Resource state immediately before that run was 71 GiB
free on the repository filesystem and 8.8 GiB available memory.

This changes the observable LAB rejection report for previously accepted invalid
source. It does not introduce compound-assignment evaluation order, atomicity,
binding, conditional, equality, result materialization, or Core lowering.

This is not a new semantic contradiction. Runtime-result use by an assignment,
`let`, or conditional guard needs the V/R boundary already isolated by Plans
186/187 and PROPOSAL-012. OPEN-012 remains only the finite-checker conditional
join-normalization question. PROPOSAL-008 remains the sole owner decision for
BND-001 outcome totality. Pure-expression lowering and compound assignment
cannot be resolved without selecting a Core/elaboration relation.

The final `make check` passed: the Canon index found 104 files, source hierarchy
found 737/737 required paths, and documentation validation found 1,568 numbered
reports. Two captured post-correction documentation-suite runs both passed 87
tests: 948.032 seconds before final review close, and 952.202 seconds after it.
The first documentation run correctly rejected one report status declaration
whose prefix did not match the required template; the one-line wording
correction then passed the earlier validation.

## What changed in understanding

The missing concrete lowerings are a coverage reminder for a later V/R-authorized
design package, not evidence that MirCore needs a general `let`, a fused
read-modify-write primitive, or an immediate expansion of OPEN-012. The bounded
parser's executable scope is now explicit: only its existing plain-assignment
and dependency-recording path proceeds; compound, `let`, `if`, and equality
forms fail closed. Existing ordinary assignment RHS dependency evidence is not
an evaluator or pure-expression semantics claim. Any later design must cover
assignment RHS values, `let` RHS/use, conditional guards, equality, and compound
operations without allowing these examples to choose the representation
prematurely.

## Open questions

- PROPOSAL-012 V: whether and how a runtime read result reaches dependent Core
  computation.
- PROPOSAL-012 R: successful cross-locus read receipt when such a result is
  needed.
- OPEN-012: finite-checker conditional join normalization only.
- PROPOSAL-008: BND-001 outcome-totality interpretation and placement.
- Compound assignment: whether and how a later selected lowering relates reads,
  writes, atomicity, failures, and result flow. It is not decided here.
- `let`, `if`, and equality: whether and how a later selected lowering supplies
  values, scope, control joins, and comparison semantics. They are not decided
  here.

## Suggested next prompt

Record the PROPOSAL-012 V disposition, and record R as well when cross-locus
read return is in scope. A subsequent bounded design package can then determine
which Surface forms it covers without adding a general continuation or binding
primitive by default.

## Plan update status

`plan/` 更新不要: the source recheck remains duplicative of Plans 186/187; the
parser correction makes the already documented bounded plain-assignment LAB
scope fail-closed and creates no new long-lived research artifact.

## Documentation.md update status

`Documentation.md` 更新不要: the high-level owner boundary and project status
did not change.

## docs/project-status.md update status

更新不要: no lifecycle, readiness, or owner-decision surface changed.

## progress.md update status

更新済み: the dated recent log records both the no-new-artifact disposition and
the bounded-LAB fail-closed correction for all reviewed forms.

## tasks.md update status

更新済み: the PROPOSAL-012 summary prevents a separate `let`/`if` decision from
being inferred and records that the bounded LAB parser now rejects unlowered
compound, statement, and equality forms.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample, validation command, debug
surface, or evidence classification changed.

## Reviewer findings and follow-up

The code mapper identified literal lowerings absent from the current sketches
and bounded LAB path, but classified runtime-value use as V/R and pure lowering
as a reserved Core/elaboration choice. The planner independently found the
candidate non-duplicative only at the textual level, not at the required
consumer/evidence level, and recommended no new artifact. The final focused
review found the compound collapse plus `let` / `if` / `==` assignment-dispatch
defects, incomplete braced/nested coverage, and over-broad report wording. The
follow-up added targeted red/green parser and source-patch tests, tightened the
fail-closed implementation, and corrected the report. The re-review then caught
the missing equality-in-target case and validation-timing wording; both were
corrected. The final narrow re-review found no remaining issue in the completed
diff, existing RHS dependency behavior, or report claim boundary.

## Skipped validations and reasons

No Lean model or executable semantic sample was added or run. Such an artifact
would choose a binder, evaluator, result carrier, or compound-operation behavior
that this source recheck is not authorized to select. Full repository and
documentation validation passed.

## Commit / push status

Pending final validation, focused review, commit with `--no-gpg-sign`, and
push to `origin/main`.

## Sub-agent session close status

The code mapper, planner, and three read-only reviewers completed their work
and are closed. No sub-agent edited repository files.
