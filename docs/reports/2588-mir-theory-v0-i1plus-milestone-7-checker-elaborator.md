# Report 2588 — Mir Theory v0 / I1+ Milestone 7: Checker / elaborator

- Date: 2026-08-04 17:19--19:05 JST
- Author / agent: parent orchestrator with independent planner, theory/formalization, production implementer, test-author, and reviewer roles
- Scope: ADR-0015 M7 only — one authoritative source-first `parse → M6 classify → M7 check → elaborate` pipeline over accepted M6 syntax
- Start revision: `51ca463f3d929911ed5c9df0802c0acb1cb6c15f`
- Payload revision: `f631f6436bfa69d0b34b6b0b186bde442b100e81`

## Objective

Implement the finite decidable M7 checker and deterministic elaborator that consumes ordinary `.mir` source, retains the complete accepted M6 classification, and emits M8-consumable typed Core, generated effect/failure/authority obligations, residual obligations, stable source mapping, and typed diagnostics without changing M6 meaning or admitting unresolved evidence.

## Scope and assumptions

M6 is the fixed input: broad `M6ExprToken` collection with spans, AST/name classification, literal `self`, owner-target checks, maintained-relation/designated distinctions, and non-executable auth/verify templates. M7 selects a finite executable expression fragment, checks declaration/type/failure-row consistency, and elaborates accepted forms. M7 does not implement runtime queues, save/load, patching, transport, M9 auth/verifier discharge, official SCN conformance, or a final public grammar/API/ABI/wire format.

## Start state / dirty state

The milestone started clean with `HEAD == origin/main == 51ca463f3d929911ed5c9df0802c0acb1cb6c15f`, no user modifications, and M7 as the sole active semantic frontier. The root filesystem had about 7 GiB available during final validation; existing build artifacts were reused and no cleanup or destructive operation was performed.

## Documents consulted

Canon-first reading covered `mirrorea_canon/README.md`, `MAP.md`, `root/DESIGN-CONSTITUTION.md`, ADR-0015, ADR-0021, M6 specs 01--04, theories 03/10/13/14/15, the proof ledger, and the M6 close report. Plan 247, `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`, and `samples_progress.md` were read as LAB/current snapshots. Historical Surface-alpha/current-L2/elaboration code was implementation evidence only; `docs/reports/` was not bulk-read.

## Actions taken

1. Added a single source-first M7 route and a typed checked artifact retaining the exact successful `SurfaceV0Classification`.
2. Added finite declaration, type, row, and expression checks; M6 diagnostics run first and preserve their source span.
3. Added ordered, left-associated typed expression trees with operand/operator spans and `Int` arithmetic enforcement.
4. Added owner RMW, maintained relation, and designated-value checked Core; enumerable source-bound effect and generated-obligation rows; typed residual obligations; deterministic source mapping; and explicit execution admission.
5. Preserved the full M6 token collector. Expressions outside the finite M7 fragment reach M7 and return `UnsupportedExpression` rather than becoming parse failures.
6. Added positive and adverse `.mir` fixtures, RED-first regression tests, a finite Lean model, Canon proposal/ADR/spec/theory, OBL-049, and sample registration.
7. Applied two correction cycles driven by material independent counterevidence, then recut the sole active roadmap from M7 to M8.

## Files changed

- `crates/mir-ast/src/surface_v0.rs`, `crates/mir-ast/tests/surface_v0_m6.rs`, and M7/reject fixtures under `crates/mir-ast/tests/fixtures/surface-v0/`: broad token retention, ordered expression trees, typed parse negatives, and spans.
- `crates/mir-semantics/src/lib.rs`, `crates/mir-semantics/src/surface_v0_classification.rs`, `crates/mir-semantics/src/surface_v0_pipeline.rs`, and `crates/mir-semantics/tests/surface_v0_pipeline_m7.rs`: M7 checker/elaborator, complete M6 evidence retention, typed Core/rows/residuals/maps, diagnostics, and tests.
- `mirrorea_canon/meta/proposals/PROPOSAL-025-m7-checked-elaboration.md`, `mirrorea_canon/adr/ADR-0022.md`, `mirrorea_canon/spec/08-m7-checked-elaboration.md`, and `mirrorea_canon/theory/16-m7-checked-elaboration.md`: normative M7 decision and finite formal boundary.
- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `mirrorea_canon/CHANGELOG.md`, `mirrorea_canon/INDEX.json`, ADR/spec/theory indexes, ADR-0021, specs 03/04, and theory/11: registration, correspondence wording, and OBL-049.
- `samples/lean/foundations/MirTheoryV0M7CheckedElaboration.lean`, its companion Markdown, `samples/lean/manifest.json`, and `scripts/current_l2_lean_sample_sync.py`: trusted finite proof evidence and registration.
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`: M7 close evidence and M8 sole-active recut.
- `Documentation.md`: concise current entry moved from M7 to M8.
- `docs/project-status.md`
  M7 accepted evidence and M8 active status.
- `progress.md`: M7 close, M8 active position, evidence classification, and timestamped recent log.
- `tasks.md`: current task map rewritten around the M8 direct blocker and M9 next step.
- `samples_progress.md`: M7 Lean/source fixture evidence row, explicitly not a runnable root or official conformance result.
- `docs/reports/2588-mir-theory-v0-i1plus-milestone-7-checker-elaborator.md`: this sole milestone report.

## Commands run

- RED/GREEN cycles: `cargo test -p mir-ast --test surface_v0_m6`; `cargo test -p mir-semantics --test surface_v0_pipeline_m7`.
- Focused final checks: `cargo test -p mir-semantics --test surface_v0_classification_m6`; the M6 parser, classifier, and M7 pipeline suites.
- Full Rust checks: `cargo fmt --check`; `cargo test -p mir-ast`; `cargo test -p mir-semantics`; Clippy for both crates with `--all-targets -- -D warnings`.
- Formal checks: `lean --trust=0 samples/lean/foundations/MirTheoryV0M7CheckedElaboration.lean`; no-stub/axiom scan; `python3 scripts/current_l2_lean_sample_sync.py --check`; `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`.
- Repository checks: `make docs`; `git diff --check`; staged secret-pattern scan; `git fetch origin main`; commit/push and remote-parity checks.

## Evidence / outputs / test results

- Final M6 parser suite: 8/8 pass.
- Existing M6 classification suite: 11/11 pass.
- Final M7 source-first pipeline suite: 22/22 pass, including complete M6 artifact equality, diagnostic precedence, full-token-to-M7 rejection, ordered tree/Int typing, source-bound rows, owner/relation/designated checked Core, deterministic elaboration, residual non-admission, and the ten-row fixture matrix.
- Full `mir-ast` and `mir-semantics` test suites pass; both crates pass warnings-as-errors Clippy and format checks.
- Lean: 16 printed theorem inventories compile with `--trust=0`; each reports no axiom dependency. The scan finds no `sorry`, `admit`, user `axiom`, unsafe/partial escape, or `True`-only theorem stub.
- Lean sample sync unit tests: 21/21 pass.
- Canon index: 159 indexed files; source hierarchy: 798/798 paths; docs validation reports 1742 numbered reports and a complete scaffold.
- OBL-049 is `lean-proved` only for the exact finite M7 carrier/profile. The M7 ten-row matrix is a fixture matrix, not SCN-01..10 official conformance.

## What changed in understanding

The first implementation passing 9 M7 tests was not an acceptable vertical boundary: it dropped relation/designated meaning and Lean did not compile. A corrected 16-test version retained those forms but independent final review found four P1 defects: M6 classification ran too late and was discarded; M6 grammar was narrowed; expression order/type evidence was lost; effect/obligation provenance did not match Lean. RED-first corrections produced the final 22-test cut. A residual review then found that retaining only a classification summary and only part of the M6 token set was still insufficient. The accepted cut therefore retains the complete M6 artifact directly and lets M7, not the parser, reject unsupported finite expressions. This is the minimal M8-consumable boundary.

## Open questions

No owner decision blocks M8. M8 must consume this checked artifact without reparsing source or bypassing residual admission, and must add deterministic runtime state/trace behavior. Runtime save/load, bounded patch, occurrence DAGs, version/receipt state, and observer-safe trace remain M8 work. M9 owns auth/verification discharge; M10 owns official SCN/release conformance.

## Suggested next prompt

No prompt is required. Continue autonomously with M8 deterministic single-process logical multi-locus runtime, using M7 checked Core and obligations as its only source-program input.

## Plan update status

更新済み: Plan 247 records M7 as completed with finite evidence, M8 deterministic runtime as the sole active semantic frontier, and M9 as next. The direct blocker is bypass-free runtime consumption of M7 checked Core, obligations, residuals, and source map.

## Documentation.md update status

更新済み: M7 is identified as accepted finite checker/elaborator evidence and M8 as current, without implying runtime or official conformance.

## docs/project-status.md update status

更新済み: M7 acceptance, OBL-049 classification, non-claims, and M8 active status are synchronized.

## progress.md update status

更新済み: The three-axis status, macro/current position, feature evidence, and timestamped work log now record M7 close and M8 entry.

## tasks.md update status

更新済み: The current task map is rewritten around M8 with M9 next, no owner blocker, exact direct consumer, acceptance evidence, and reopen/stop conditions.

## samples_progress.md update status

更新済み: M7 adds a registered Lean foundation and source-first fixture command. The row is evidence-only, not an active runnable sample root, product workflow, or official SCN conformance claim.

## Reviewer findings and follow-up

The pre-edit planner rejected the initial 9-test cut because relation/designated semantics and source/Core evidence were lost and Lean failed. The independent final reviewer then reported four P1 findings: wrong M6/M7 ordering and discarded classification, M6 grammar narrowing, lossy/untyped expression structure, and private/unspanned effect/obligation evidence with Lean drift. All received RED tests and production/Canon/Lean corrections. A narrow re-review found two incomplete repairs—summary-only M6 retention and incomplete punctuation collection—which received a second RED/GREEN correction. Final narrow reviews mark both Rust residuals resolved and separately confirm Lean carrier alignment, 16 axiom-free theorem inventories, and no proof stub. No P0/P1 remains.

## Skipped validations and reasons

No applicable M7 validation was skipped. M8 runtime replay/save-load/patch tests, M9 auth/verifier tests, M10 official SCN/release conformance, fresh-clone final release reproduction, real transport, and final CLI/API/ABI/wire validation were not run because their implementations are later milestones. They are not claimed by M7.

## Commit / push status

Payload commit `f631f6436bfa69d0b34b6b0b186bde442b100e81` (`feat(mir): close M7 checked elaboration payload`) was pushed to `origin/main`; `HEAD == origin/main` was verified immediately afterward. This report and the six closeout snapshots form the pending second M7 closeout commit; the parent will push it and report final remote parity in the milestone update.

## Sub-agent session close status

- M7 pre-edit planner / snapshot writer: complete.
- M7 production implementer: complete; no commit made by the sub-agent.
- M7 test author: complete; RED evidence and final formatting/focused GREEN recorded.
- M7 theory/formalization: complete; Canon/Lean/index/docs checks GREEN.
- Independent reviewer: complete; initial P1 findings and both narrow residual reviews closed.
- No M7 sub-agent remains necessary for the closed milestone.
