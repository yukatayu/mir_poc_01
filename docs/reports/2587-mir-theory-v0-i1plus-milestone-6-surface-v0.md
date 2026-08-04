# Report 2587 — Mir Theory v0 / I1+ Milestone 6: Surface v0

- Date: 2026-08-04
- Author / agent: parent orchestrator; independent planner, theory/formalization, production implementer, test-author, and independent reviewer roles
- Scope: ADR-0015 M6 only — bounded ordinary `.mir` Surface v0, span-rich AST, and a finite M5-aligned classification boundary
- Decision levels touched: owner-delegated M6 Canon/implementation/proof refinement; no final public grammar/API/ABI/wire or deployment

## Objective

Close the smallest ordinary Surface v0 whose accepted source is parsed into a canonical-span AST and classified into either existing M5 Core evidence, an inspectable typed CoreTemplate, or a typed diagnostic. Preserve the separation of authority origin and evaluation locus, owner-local RMW, maintained relations, designated result materialization, and source traceability.

## Scope and assumptions

M5 at pushed revision `60c965b3f5e98170ef43cbc84452485e45b34684` was the semantic input. The selected declaration/action grammar was compared only with an event/operation-centric alternative; the latter was rejected because it would put transport/event machinery at the Surface centre. M6 does not claim M7 checking/elaboration, M8 runtime, a final public grammar/diagnostic ABI/wire, arbitrary relation DAGs, conformance, or I1+ completion.

## Start state / dirty state

Started clean from `HEAD == origin/main == 60c965b3f5e98170ef43cbc84452485e45b34684` (`Close M5 shared formal model`). No user changes or untracked files were present. M6 was the sole active semantic milestone in Plan 247.

## Documents consulted

Canon-first reading used `mirrorea_canon/README.md`, `MAP.md`, Design Constitution C2/C5/C11, ADR-0015, theory 01/03/10/13/14/15, the metatheory ledger, and Plan 247. Report 2586 and historical parser paths were LAB evidence only. The independent planner reviewed the clean M5 cut before edits.

## Actions taken

1. Selected the ordinary declaration/action grammar and rejected the event/operation-centric alternative.
2. Added an independent M6 lexer/parser and AST with file-qualified byte and line/column spans; the AST does not depend on semantics.
3. Added the one-way semantic conversion from an AST span to M5 `SourceRef`, M5 owner-RMW lowering, typed CoreTemplates, source-to-Core mapping, and real source-fixture matrix execution.
4. Kept same-owner lowering to request and owner-write edges plus capability/witness obligations only; it creates no receipt or receipt-release fact. Cross-owner RHS is a typed receipt-required diagnostic.
5. Kept result frontier/version separate from relation binding frontier/publication/projection. Added non-executable, span-tracked typed deferred templates for `with auth` and `verify`; they grant no authority and create no effect, state update, or verdict.
6. Added Lean OBL-048 exact-finite classifier/span evidence and synchronized Canon proposal, ADR, specs, ledger, index, and Lean inventory.
7. Performed an independent final review. It found three P1 issues: arbitrary `Role[...]` actor, cross-owner assignment target, and a fieldless-target panic. Added RED fixtures/tests, fixed each fail-closed boundary, synchronized Canon/Lean, and obtained a re-review with no P0/P1.

## Files changed

- Canon: `mirrorea_canon/meta/proposals/PROPOSAL-024-m6-bounded-surface.md`, `adr/ADR-0021.md`, specs 01--04, theory ledger, retained ADR cross-references, changelog, map/readmes, and generated index.
- Production: `crates/mir-ast/src/surface_v0.rs`, `crates/mir-semantics/src/surface_v0_classification.rs`, minimal module exports, and the conservative `Core::same_owner_rmw` factory in `shared_model.rs`.
- Evidence: M6 source fixtures, parser/classifier tests, `samples/lean/foundations/MirTheoryV0M6Surface.lean`, its companion, and the Lean manifest/sync registration.
- Current-status snapshot: Plan 247, `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md`.
- `docs/project-status.md`
- This sole M6 report.

## Commands run

- `cargo test -p mir-ast`
- `cargo test -p mir-semantics`
- `cargo fmt --check`
- `cargo clippy -p mir-ast --all-targets -- -D warnings`
- `cargo clippy -p mir-semantics --all-targets -- -D warnings`
- `cargo test -p mir-ast --test surface_v0_m6`
- `cargo test -p mir-semantics --test surface_v0_classification_m6`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M6Surface.lean`
- `python3 scripts/current_l2_lean_sample_sync.py`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- `(cd mirrorea_canon && python3 meta/build-index.py --check)`
- `make docs` (including hierarchy/documentation validation)
- `git diff --check`

## Evidence / outputs / test results

- M6 parser suite: 3 tests passed, including literal `Role[self]` enforcement and transport/occurrence/envelope rejects.
- M6 classifier suite: 11 tests passed. The real source matrix covers ordinary RMW, cross-owner RHS, maintained relation, designated result, relation-value and consumer-mutation rejects, and prohibited surface machinery. It also covers deferred markers, action-locus mismatch, literal-self rejection, cross-owner write target rejection, and fieldless target no-panic rejection.
- Full `mir-ast` and `mir-semantics` suites, focused Clippy, formatting, and diff checks passed.
- OBL-048 compiles with `lean --trust=0`; the finite theorems cover determinism, canonical span retention, literal-self parser diagnostic, fieldless target, target-owner, RHS receipt-required, relation/publication, and deferred-template cases. The only reported logical dependency for the span theorems is Lean's standard `propext`; there are no `sorry`, `admit`, user axiom, opaque theorem, or `True` placeholder.
- Lean inventory synchronization passed with 21 unit tests; Canon index check reports 155 indexed files.
- The optional Oracle consultation was attempted once and ended with a browser-side error without output. It informed no decision and was not retried.

## What changed in understanding

The M5 shared model was sufficient as a semantic target but not as a source boundary. A safe M6 boundary requires three distinct fail-closed checks before owner-RMW construction: literal authority origin, action/target owner alignment, and a field-bearing mutation target. A source span is a parser carrier; only classification turns it into M5 `SourceRef`. Deferred auth/verification syntax can be inspectable without treating a policy name as a grant or invoking M9 behaviour.

## Open questions

No M6 blocker remains. M7 must define decidable checking, generated effect/failure/authority obligations, residual obligations, and a source-first elaborator without changing M6 grammar, source spans, classification outcomes, or their non-effects.

## Suggested next prompt

No prompt is required. Continue autonomously with M7's single `parse → check → elaborate` route using the accepted M6 boundary.

## Plan update status

更新済み: Plan 247 marks M6 complete, M7 as the sole active milestone, and M8 as next. It gives M7's direct acceptance condition: preserve M6 meaning while producing inspectable Core, obligation, and source-trace outputs.

## Documentation.md update status

更新済み: `Documentation.md` records M6's bounded grammar/AST/classification, 3 parser tests, 11 classifier tests, OBL-048 exact-finite evidence, and M7 as current.

## docs/project-status.md update status

更新済み: `docs/project-status.md` records the M6 close/non-effects and the M7 stop line without claiming runtime, final grammar, conformance, or I1 completion.

## progress.md update status

更新済み: `progress.md` moves the active frontier to M7 and adds the 2026-08-04 16:50 JST M6 close entry.

## tasks.md update status

更新済み: `tasks.md` is recut as a current snapshot with M7 active, M8 next, and the M6-preservation constraint explicit.

## samples_progress.md update status

更新済み: `samples_progress.md` registers the active M6 Lean foundation and the evidence-only source fixtures. It explicitly does not promote them to a runnable sample root or final language workflow.

## Reviewer findings and follow-up

The independent planner accepted the declaration/action cut and listed hidden authority, receipt-free cross-owner access, relation-to-value collapse, lost frontier/version, missing spans, and M7 semantic drift as falsifiers.

The independent final reviewer initially raised three P1 findings: arbitrary actor text could select the Core caller; a cross-owner LHS could lower as a local RMW; and a fieldless target could panic. The test author recorded each RED case. The implementation now rejects non-literal role actors at parser level, rejects fieldless/cross-owner targets before Core construction, and records exact spans. Canon and Lean use the same diagnostic ordering. The re-review found no P0/P1 and accepted M6.

## Skipped validations and reasons

M7 checker/elaborator, M8 runtime, M9 extensions, M10 release conformance, fresh-clone execution, and public CLI claims are later milestones. No test not listed above is claimed as run by this report. The failed Oracle attempt is advisory-only and not a validation failure.

## Commit / push status

Implementation, Canon, Lean, fixtures, tests, and current-status snapshots are committed as `887bc2dc` (`Implement M6 Surface v0`). This report is the second M6 closeout commit. The two-commit M6 series is pushed to `origin/main` and remote parity is verified after the closeout commit. M5 start parity is recorded above.

## Sub-agent session close status

- M6 pre-edit planner / closeout snapshot writer: complete, read-only for pre-edit review and planning/status edits.
- M6 theory/formalization: complete; Canon/Lean only.
- M6 production implementer: complete; production Rust only.
- M6 test author: complete; fixtures/tests only.
- M6 independent reviewer: complete; initial P1 review followed by accepting re-review.
