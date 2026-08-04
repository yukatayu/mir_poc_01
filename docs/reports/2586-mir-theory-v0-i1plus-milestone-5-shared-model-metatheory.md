# Report 2586 — Mir Theory v0 / I1+ Milestone 5: shared formal model / modular metatheory

- Date: 2026-08-04 13:50–15:15 JST
- Author / agent: parent orchestrator with independent planner, theory/formalization, implementer, test-author, and reviewer roles
- Scope: ADR-0015 M5 only — one finite concrete shared model that integrates accepted M1--M4 meaning and establishes the M6 semantic boundary
- Decision levels touched: owner-delegated M5 Canon/implementation/proof refinement; no owner-reserved decision, final grammar, public API/ABI/wire, or deployment

## Objective

Close one non-opaque finite model in which a semantic Surface fragment elaborates deterministically to Core or a typed Diagnostic, and the same Config/Step/WellFormed/Trace universe represents the accepted M3 evaluation/materialization and M4 maintained-relation/projection rules. It supplies the semantic boundary M6 needs without preselecting M6 grammar or M8 runtime.

## Scope and assumptions

M1 Constitution, M3, and M4 are accepted inputs. Exactly two routes were compared: a fresh finite concrete universe and a prose/adapter correspondence between the separate M3/M4 models. The former was selected because the latter has no shared Config, Step, WellFormed, or save boundary. M5 retains finite cut/save and inactive-patch carriers, but does not implement M8 persistence or patch runtime. General theorems, arbitrary relation DAGs, final diagnostic identifiers, final spans, public contracts, and distributed semantics remain deferred.

## Start state / dirty state

Started clean from pushed M4 revision `a1b9d0b76553283f35f40c273929b142e95d37a0` (`HEAD == origin/main`). No user changes or untracked files were present. M4 was closed and M5 was the sole active semantic milestone in Plan 247.

## Documents consulted

Canon-first reading used `mirrorea_canon/README.md`, `MAP.md`, the Design Constitution, ADR-0015/0018/0019, theory 01--11/13/14, and current Plan 247. M3/M4 reports, Lean foundations, and `mir-semantics` modules were consulted as LAB/implementation evidence only. The independent pre-edit planner and final reviewer supplied read-only evidence. A focused ChatGPT Pro Oracle challenge was advisory only; its 9m53s response was checked against Canon and did not become normative state.

## Actions taken

1. Selected one fresh finite shared carrier rather than model-to-model adapters, retaining nominally distinct result frontier, relation activation frontier, epochs, presentation context, and atomic cut.
2. Added PROPOSAL-023, ADR-0020, theory/15, and exact OBL-040--047 ledger rows. They define `SurfaceFragment → Core | Diagnostic` and one shared Config containing occurrence/authority/observation histories plus receipt `R`, designated result `D`, relation `J`, cut/save, and inactive patch state.
3. Added the directly executable `shared_model` reference and focused tests. It runs owner RMW/receipt chains, designated result version/one-shot state, owner relation bind/publish, C-local projection, fallback/reacquire, label propagation, atomic cut, and save/restore directly from the shared types; it does not invoke M3/M4 report or fixture wrappers.
4. Added one finite shared Lean foundation and registered it in the Lean manifest/synchronizer. The evidence states only its enumerated finite profiles.
5. Applied review corrections in this report rather than opening a closeout packet: published relation is an owner-created immutable carrier; consumer projection reads it rather than mutable `J`; save/restore reconstructs and validates state; fabricated receipt insertion fails closed; designated consumption is globally one-shot; membership provenance is validated before owner or relation use.
6. A first all-authority witness comparison falsely rejected valid fresh relation witnesses. Root-cause tracing identified two lineages: membership witness matches base admission authority, while relation witness matches the exact current `J` binding authority. The final scoped check validates principal/capability/membership epoch/live exact lease for every authority, validates the membership witness against base admission, and validates the relation witness against current relation lineage.

## Files changed

- `mirrorea_canon/meta/proposals/PROPOSAL-023-shared-formal-model-metatheory.md`
- `mirrorea_canon/adr/ADR-0020.md`, `mirrorea_canon/theory/15-shared-formal-model.md`, the M5 references in Canon theory/README/MAP/changelog/index/ledger
- `samples/lean/foundations/MirTheoryV0M5SharedModel.lean` and companion, Lean manifest, and synchronizer registration
- `crates/mir-semantics/src/shared_model.rs`, its `lib.rs` export, and `crates/mir-semantics/tests/shared_model_m5.rs`
- `plan/247-mir-theory-v0-i1plus-current-roadmap.md`, `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md`
- `docs/project-status.md`
- This sole M5 report

## Commands run

- clean-start, remote-parity, Canon-first, and focused source inventory checks
- `cargo test -p mir-semantics --test shared_model_m5` (final: 13 tests)
- `cargo fmt --check`
- `cargo clippy -p mir-semantics --all-targets -- -D warnings`
- `cargo test -p mir-semantics`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M3EvaluationMaterialization.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M4MaintainedRelationProjection.lean`
- `lean --trust=0 samples/lean/foundations/MirTheoryV0M5SharedModel.lean`
- M5 `axiom`/`opaque`/`sorry`/`admit`/`:= True` scan, Lean sample synchronization, and its 21 Python tests
- `cd mirrorea_canon && python3 meta/build-index.py --check`
- focused snapshot-heading validation, `make docs`, and repeated `git diff --check`

## Evidence / outputs / test results

The direct M5 suite has 13 passing tests: deterministic Surface/Core-or-Diagnostic elaboration; M3 owner/receipt/designated behavior; owner-held relation publication; C-local projection/rejects; fallback/reacquire; bounded presentation counterexamples; authority lineage; and cut/save/restore corruption rejection. The full `mir-semantics` suite, formatting, and warnings-denied Clippy passed.

`--trust=0` compiled the M3, M4, and M5 foundations. The M5 axiom print shows only Lean standard `propext` and, where needed, `Quot.sound`; no user axiom, `sorry`, `admit`, `opaque`, or tautological theorem stub was found. Canon index check reported 153 indexed files and Lean synchronization tests reported 21 passing tests.

The correspondence is explicit: M3 Surface/evaluation intent is M5 `SurfaceFragment`/`EvalPlan`, receipt `R`, designated `D`, and owner step trace; M4 relation semantics is M5 `J`, published carrier, C-local `PresentationContext` projection, fallback/reacquire, and derived label. OBL-040--047 are `lean-proved` only for these finite constructors. The Rust tests are runtime/model evidence, not general proofs.

## What changed in understanding

One configuration can preserve the necessary distinctions without making M3 and M4 opaque adapters. The critical authority detail is that membership and relation witnesses are related but not interchangeable: the base admission witness validates membership lineage, while a fresh relation witness validates the current relation/binding lineage. Saving is meaningful only when provenance reconstructs a well-formed shared configuration before mutation.

## Open questions

No M5 blocker remains. General elaboration determinism, general preservation, arbitrary relation DAG naturality, noninterference, distributed cut/save/load, patch execution, and transport remain explicitly deferred in the ledger and later milestones.

## Suggested next prompt

No prompt is required. Continue autonomously with M6 Surface v0: bounded ordinary grammar, source spans, and total M5-aligned Core/Diagnostic classification; do not begin M7 before M6 closes.

## Plan update status

Updated Plan 247: M5 is closed, M6 is sole active, M7 is next, and the M5 finite evidence/non-claims are recorded.

## Documentation.md update status

Updated: reader-facing entry now routes current work to M6 and states M5’s finite scope without a grammar, ABI, wire, runtime, or general-theorem claim.

## docs/project-status.md update status

更新済み: current frontier is M6; M5 evidence and non-effects are synchronized.

## progress.md update status

Updated: the three-axis snapshot, milestone map, macro map, and timestamped M5 close log now identify M6 as active.

## tasks.md update status

Updated: the whole current task map has M5 closed, M6 active, and M7 next; historical plans remain repository memory.

## samples_progress.md update status

Updated: accepted M5 Lean/shared-model evidence is shown as finite evidence only, not a product/workflow completion claim.

## Reviewer findings and follow-up

The pre-edit planner rejected prose/adapter correspondence. The initial independent reviewer found missing published-relation state, incomplete save/restore reconstruction, receipt/designated gaps, and Lean/model correspondence overclaims. A re-review found that restore had to validate reconstructed configuration and that direct receipt insertion and per-consumer designated consumption were unsafe. The final review found membership capability/witness lineage incomplete. The first universal witness check exposed a valid fresh-relation-witness counterexample; the scoped lineage correction and new direct/relation/save corruption tests resolved it. The final independent re-review found no P0/P1.

## Skipped validations and reasons

M6 grammar/parser/AST, M7 checker/elaborator, M8 deterministic runtime, M9 auth/verification extensions, and M10 conformance are intentionally later milestones. No claim is made for final CLI, public ABI/wire, arbitrary DAG projection, general metatheory, distributed execution, production identity, or deployment. `make docs` passed after the closeout record and status snapshots were completed.

## Commit / push status

No M5 commit or push existed at report start. M5 is integrated in one `--no-gpg-sign` closeout commit, pushed to `origin/main`, and remote parity is checked immediately after the push.

## Sub-agent session close status

- M5 pre-edit planner: complete, read-only.
- M5 theory/formalization: complete; Canon/Lean writer.
- M5 implementer: complete; production single writer.
- M5 test author: complete; test-only writer.
- M5 final reviewer: complete, independent read-only; final result no P0/P1.
- M5 snapshot planner: complete; planning/status-only writer.
- Oracle advisory: complete; no repository write authority and no normative standing.
