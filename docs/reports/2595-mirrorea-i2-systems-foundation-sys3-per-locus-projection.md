# Report 2595 — Mirrorea I2 Systems Foundation SYS-3: checked-Core per-locus projection

- Date: 2026-08-27 JST
- Milestone: SYS-3
- Status: **completed / closed**
- Accepted source/evidence cut: `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`
- Evidence class: OBL-060 `runtime-monitored`, static finite compiler/projector only
- Lifecycle: theory T1; broad PHASE-I1 and official I2 entry/exit unaccepted

## Objective

From one `CheckedSurfaceV0` plus its identity-bound logical-locus inventory,
deterministically produce owned per-locus executable plans and generated
communication/effect/observation/persistence/source-map plans without a
handwritten interface, semantic reconstruction, hidden authority, or hidden
failure. Preserve enough checked meaning for SYS-4 to execute the artifacts
without reparsing source.

The accepted seam includes the bounded internal, non-final Surface-v0 clause
`designated consume E.result at C`. It preserves a distinct AST/M6/M7 consume
edge, names exactly one consumer in source, and generates evaluator-to-consumer
artifacts and delivery. Topology, schedule, deployment, and relations cannot
invent the consumer.

## Scope and assumptions

The projector is crate-private and pure. It consumes checked artifacts and an
exact locus inventory bound to the same program identity. It does not consume
M10 profile/release machinery, runtime state, source text, handwritten schemas,
expected results, or deployment-host choices.

Production relation lowering remains the checked two-anchor primary-to-
fallback shape. A same-program finite acyclic test seam supplies one deeper/
shared pressure case only; it is not ordinary-source nested relation semantics
or an arbitrary-DAG theorem.

`ReturnExistingNoNewConsumption` is a static source/Core identity and SYS-4
endpoint-refinement requirement, not current M8/M10 runtime evidence. Legacy
M8 returns `AlreadyConsumed` for the same delivery id and may consume a
different id; accepted M10 behavior is unchanged. SYS-4 must implement a
source/Core-bound carrier-side return/wrapper, call M8 exactly once on the
accepted semantic path, and test first/retry/competing-consumer endpoints.

```text
Direct consumer: SYS-4 in-process generated dispatch
Blocker reduced: checked Core now contains every accepted placement and
  communication fact, including the source-named result consumer
Acceptance use: SYS-4 dispatch, SYS-5 causal devtools, SYS-6 correspondence
```

## Start state / dirty state

- Branch: `main`.
- Initial complete-projector candidate `ded622fef91bab2cadc571ba944e5ee2c69a7b63`
  followed RED commits `c10a1bce`, `dae31bbe`, `db4358d1`, `cd98d81f`, and
  `e8c9570f`.
- Close review found its missing ordinary-source/Core E-CONSUME consumer path;
  SYS-3 reopened and that cut became partial regression evidence.
- Corrected sequence: `b39f3e76` (source-bound consume), `f37be73c` (M6
  metadata), `27e42658` (missing producer), `30be30bb` (ambiguous signature),
  then final cut `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9`.
- At final docs closeout start, `HEAD == origin/main == 3013e7fe...`; source was
  pushed and parity-confirmed. The worktree held the accumulating docs/Canon
  diff and an independently owned HTML test change. This writer edited neither
  production source nor tests.
- Snapshot timestamp came from `date`: `2026-08-27 07:07:55 JST`.

## Documents consulted

- Canon hierarchy: `mirrorea_canon/README.md`, `MAP.md`, `NORTH-STAR.md`,
  `DESIGN-CONSTITUTION.md`, `CANON.md`, and `AGENTS.md`.
- Authority/lifecycle: ADR-0025--0029, Canon plan/00--02, and Plan 249.
- Direct semantics: architecture/03--04, theory/11/13/14/16, and spec/02/04/08/12.
- Direct implementation evidence: bounded Surface/M6/M7 consume paths and
  `crates/mir-runtime/src/sys3_projection*` at the pinned cuts.
- Current derived views: README, `Documentation.md`, project status, HTML
  overview, `progress.md`, `tasks.md`, plan/00, and `samples_progress.md`.
- Report 2594 and only directly relevant earlier closeout conventions were
  inspected; `docs/reports/` was not read in bulk.

## Actions taken

1. Selected a pure checked-Core projector over Full System V1's manual
   request-manifest IR.
2. Restricted topology to an exact checked-identity locus inventory; it
   supplies no edge, authority, failure, handler, schema, result, or host.
3. Kept same-owner RMW owner-local, relations as owner publication plus
   consumer-local projection, and designated input as source-owner service plus
   evaluator consumption.
4. Generated fragment-bound carrier/plans and joined source/Core/artifact/edge
   correspondence with typed diagnostics and deterministic recomputation.
5. Made observation finalization deduplicate only full semantic-row equality.
6. Reopened after the missing E-CONSUME P1 and added only bounded internal
   `designated consume E.result at C`; no topology inference or public freeze.
7. Separated theory/13 retry requirements from legacy M8/M10 evidence.
8. Fixed M6 metadata P1, missing-producer P2, and silent-signature-shadow P1;
   final semantic/code-quality reviews accepted the corrected cut.
9. Applied PROPOSAL-032 / ADR-0029, spec/12, OBL-060, Plan 249, this report, and
   status/reader synchronization. No WRK or new plan was opened.
10. Closed SYS-3, made SYS-4 sole active and SYS-5 next, without moving theory
    T1 or broad PHASE-I1/I2 lifecycle state.
11. Applied final planner-review corrections: separated OBL-049 from OBL-060,
    moved report/WRK routing to active SYS-4, corrected accepted-report status,
    completed source/command/reviewer accounting, and qualified the future I2
    boundary. Narrow planner re-review remains the closeout gate.

## Files changed

Accepted SYS-3 production/test/fixture delta from
`e3d071b340b8c513c6e4804b91f9f1dddf3461cf` through `3013e7fe...`:

- `crates/mir-ast/src/surface_v0.rs`
- `crates/mir-ast/tests/fixtures/surface-v0/designated_result_consume_competing_consumer.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/designated_result_consume_missing_producer.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/designated_result_consume_three_locus.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/designated_result_consume_unknown_consumer.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys3_projection_four_locus.mir`
- `crates/mir-ast/tests/fixtures/surface-v0/sys3_projection_relation_extension_pressure.mir`
- `crates/mir-ast/tests/surface_v0_m6.rs`
- `crates/mir-runtime/src/lib.rs`
- `crates/mir-runtime/src/m8_runtime_admission.rs`
- `crates/mir-runtime/src/sys3_projection/lowering.rs`
- `crates/mir-runtime/src/sys3_projection/mod.rs`
- `crates/mir-runtime/src/sys3_projection/model.rs`
- `crates/mir-runtime/src/sys3_projection/validate.rs`
- `crates/mir-runtime/src/sys3_projection_tests.rs`
- `crates/mir-semantics/src/evaluation_materialization.rs`
- `crates/mir-semantics/src/m9_finite_refinement.rs`
- `crates/mir-semantics/src/surface_v0_classification.rs`
- `crates/mir-semantics/src/surface_v0_pipeline.rs`
- `crates/mir-semantics/tests/surface_v0_classification_m6.rs`
- `crates/mir-semantics/tests/surface_v0_pipeline_m7.rs`

SYS-3 Canon/closeout delta:

- `mirrorea_canon/meta/proposals/PROPOSAL-032-sys3-checked-core-per-locus-projection.md`
- `mirrorea_canon/adr/ADR-0029.md`
- `mirrorea_canon/spec/12-sys3-per-locus-projection.md`
- `docs/reports/2595-mirrorea-i2-systems-foundation-sys3-per-locus-projection.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/CHANGELOG.md`
- `mirrorea_canon/INDEX.json`
- `mirrorea_canon/adr/README.md`
- `mirrorea_canon/architecture/03-toolchain.md`
- `mirrorea_canon/architecture/04-runtime-carriers.md`
- `mirrorea_canon/spec/02-surface-grammar.md`
- `mirrorea_canon/spec/04-core-ir.md`
- `mirrorea_canon/spec/08-m7-checked-elaboration.md`
- `mirrorea_canon/spec/README.md`
- `mirrorea_canon/theory/11-metatheory-ledger.md`
- `mirrorea_canon/theory/13-evaluation-materialization.md`
- `mirrorea_canon/theory/16-m7-checked-elaboration.md`
- `mirrorea_canon/plan/01-phases.md`
- `Documentation.md`
- `README.md`
- `docs/project-status.md`
- `docs/mirrorea-project-overview.html`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/249-mirrorea-i2-systems-foundation-current-roadmap.md`
- `scripts/tests/test_mirrorea_project_overview_html.py` (independent test-author lane)

Production/test files through `3013e7fe...` belong to implementation/test
lanes. The modified HTML regression test in the worktree is independently
owned and was not edited here. `samples_progress.md` remains unchanged.

## Commands run

Accepted-cut source validation executed by the parent implementation/test
lanes:

```text
cargo test -p mir-ast --test surface_v0_m6
cargo test -p mir-semantics --test surface_v0_classification_m6
cargo test -p mir-semantics --test surface_v0_pipeline_m7
cargo test -p mir-semantics --test m9_finite_refinement
cargo test -p mir-runtime --lib sys3_projection_tests
cargo test -p mir-runtime --test m8_runtime_admission
cargo test -p mir-runtime --test m10_source_execution
cargo test -p mir-runtime --test m10_conformance
cargo test -p mir-runtime --no-fail-fast
cargo test --workspace --no-fail-fast
cargo fmt --all -- --check
cargo clippy -p mir-ast --all-targets -- -D warnings
cargo clippy -p mir-semantics --all-targets -- -D warnings
cargo clippy -p mir-runtime --all-targets -- -D warnings
git diff --check
```

Closeout inspection and validation executed by this writer:

```text
git status --short --branch
git rev-parse HEAD
git rev-parse origin/main
git log --oneline -12
TZ=Asia/Tokyo date '+%Y-%m-%d %H:%M:%S %Z'
git diff --name-only e3d071b340b8c513c6e4804b91f9f1dddf3461cf..3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9
cd mirrorea_canon && python3 meta/build-index.py
cd mirrorea_canon && python3 meta/build-index.py --check
make docs
python3 -m unittest scripts.tests.test_mirrorea_project_overview_html
git diff --check
```

## Evidence / outputs / test results

- AST Surface M6 **9/9**, M6 classification **13/13**, M7 pipeline **25/25**.
- M9 **8/8**, SYS-3 **27/27**, M8 admission **7/7**.
- M10 source **2/2**, M10 conformance **67/67**.
- Full `mir-runtime` tests and full workspace tests: exit 0.
- Format, scoped `mir-ast`/`mir-semantics`/`mir-runtime` Clippy with
  `-D warnings`, and source diff check: exit 0.
- Final semantic review: **ACCEPT**. Final code-quality review: **ACCEPT**.
- Canon index build/check: **179 files**, pass.
- `make docs`: pass after its first two runs correctly rejected Report 2595's
  initially malformed update declaration (`更新済み。` rather than
  `更新済み:`) and then its missing exact `docs/project-status.md` Files-changed
  bullet. Final run passed agent configuration, Canon index, source hierarchy
  **799/799**, documentation scaffold, and **1,749 numbered reports**.
- The planner-correction validation rerun first detected the expected stale
  Canon index after spec/08 changed. Regenerating it produced **179 files**;
  the subsequent full `make docs` rerun passed the same 799/799 and 1,749-report
  checks.
- `git diff --check`: exit 0.
- Focused HTML regression: final **8/8 pass**. The first final-close run was
  **8 run, 3 failures / 5 pass** because the independently owned test still
  required the superseded reopened SYS-3/SYS-4-next state. The test author
  synchronized lifecycle, stale-state, and linked-Documentation assertions to
  SYS-3 closed / SYS-4 active / SYS-5 next; this writer did not edit the test.
  The final rerun and `git diff --check` both passed.

OBL-060 is `runtime-monitored` only for static finite projection: deterministic
identity-bound placement, generated edge completeness for accepted operations,
owner preservation, exactly-one source-named consumer, correspondence,
observation/persistence requirements, falsifiers, two-anchor agreement, and
test-only extension pressure. It is not Lean/general proof, runtime dispatch,
runtime retry-return, production relation-DAG, or public compatibility evidence.

## What changed in understanding

Checked Core plus an exact locus inventory suffices only when both endpoints
are explicit in source-derived Core. A projector must reject a missing consumer
rather than infer one. The smallest correction was the bounded consume clause,
not a handwritten target or schedule field.

The theory/13 retry rule and legacy M8 delivery-id API are separate contracts.
Static projection can carry identity and refinement requirements; SYS-4 alone
can establish retained-result return and exactly one accepted M8 call.

Static observation/persistence plans are responsibility maps, not evidence that
runtime occurrences, cuts, saves, restores, or patches occurred.

## Open questions

- Smallest independent locus store/queue/endpoint abstraction without a global
  mutable-map bypass.
- Shared ST/OW1 artifact/endpoint abstraction while concrete worker layout
  remains provisional.
- Exact runtime occurrence identities for source/Core/artifact/edge binding.
- Minimal process-local whole-fabric cut for artifacts, carriers, queues,
  relations, designated state, authority references, and patch state.
- OPEN-026/027, public encoding, real transport, production nested relations,
  and general theory remain later scope.

## Suggested next prompt

Continue Plan 249 SYS-4. Define RED endpoint contracts, then implement the
smallest independent `LocusRuntime` shell over accepted SYS-3 artifacts. Run
only generated routes under ST and eligible OW1; forbid source reparse,
handwritten edges, direct remote-store mutation, and authority minting. Add the
source/Core-bound retry wrapper with actual first/retry/competing tests and one
accepted M8 consume, preserving M8/M10. Then add typed failures, replay, local
whole-fabric cut/save/restore/patch, correspondence, review, and closeout.

## Plan update status

更新済み: Plan 249 records SYS-3 closed at `3013e7fe...`, retains `ded622fe...`
as partial history, classifies OBL-060 static-only `runtime-monitored`, makes
SYS-4 sole active and SYS-5 next, and assigns runtime retry to SYS-4. plan/00
matches. No new plan or WRK was created.

## Documentation.md update status

更新済み: SYS-0--SYS-3 completed, SYS-4 active, SYS-5 next, the accepted cut,
bounded consume clause, static/runtime retry split, and lifecycle/public
non-claims are synchronized.

## docs/project-status.md update status

更新済み: The current blocker is actual generated endpoint dispatch, ST/OW1,
retry wrapper, fail-closed paths, replay, and local cut/save/restore/patch; SYS-5
is the direct consumer.

## progress.md update status

更新済み: The three axes, milestone/macro/feature rows, startability, OBL-060,
blocker, and recent log reflect `3013e7fe...`. The timestamp came from `date`.

## tasks.md update status

更新済み: The snapshot promotes SYS-4, orders its endpoint/ST/OW1/retry/failure/
replay/cut packages, separates research from owner gates, and preserves the
static/runtime evidence boundary.

## samples_progress.md update status

更新不要: No runnable sample path, command, debug surface, readiness, or sample
blocker changed. Generated artifacts have no local-run endpoint yet; SYS-5 is
the first new runnable toy consumer.

## Reviewer findings and follow-up

- Covered-fragment semantic review: ACCEPT after fragment/provenance/span/
  relation placement corrections.
- Code-quality review: ACCEPT after observation full-row deduplication P2.
- Reader review: stale HTML timing-table P1 corrected and independently tested.
- Semantic close review: missing E-CONSUME P1; reopened and demoted `ded622fe`.
- Retry-boundary P1: corrected; retry is SYS-4 refinement, not M8/M10 evidence.
- Corrected path fixed M6 metadata P1, missing producer P2, and silent signature
  shadow P1.
- Final corrected-cut semantic and code-quality reviews: **ACCEPT**, with no
  remaining P0/P1/P2 in the selected finite fragment.
- Final planner closeout review initially returned **REJECT** on the docs cut:
  P1 evidence conflation in spec/08, P1 stale SYS-3 report routing in
  `tasks.md`, P2 accumulating-report wording in project status, P2 incomplete
  production/fixture and command/reviewer accounting in this report, and P2
  premature `I2 accepted cut` wording in the HTML reader.
- This correction separates OBL-049 Lean evidence from OBL-060 static evidence,
  routes future WRK admission through active SYS-4 and its future single Report
  2596 without creating it, records Reports 2592--2595 as accepted evidence,
  enumerates the complete source delta and exact commands, and qualifies the
  I2 boundary as future post-SYS-6. Narrow planner re-review returned
  **ACCEPT**, with no remaining P0/P1/P2 and no collateral semantic or
  governance drift.
- No reviewer accepted a general theorem, production relation DAG, runtime
  dispatch, multi-consumer semantics, or public compatibility claim.

## Skipped validations and reasons

- Lean `--trust=0`: not run; no Lean/general claim changed.
- Full workspace Clippy: **not claimed**. Only scoped `mir-ast`,
  `mir-semantics`, and `mir-runtime` warnings-denied Clippy is evidence.
- Fresh-clone M10 reproduction: not rerun; M10 identity is a regression baseline,
  not SYS-3 artifact identity.
- Dispatch, sockets, multi-process transport, durable persistence, browser,
  production relations/deployment, and performance are outside SYS-3.
- This docs writer did not rerun Rust suites or edit source/tests; exact parent-
  supplied results and pinned cut are recorded. Docs validators run locally.

## Commit / push status

Production cut `3013e7fe075a7605a1ffe01e0b14f4a0856eaeb9` was committed, pushed,
and confirmed at `HEAD == origin/main`; its chain is `b39f3e76`, `f37be73c`,
`27e42658`, `30be30bb`, and `3013e7fe`.

This Canon/Plan/report/status/reader diff is uncommitted and unpushed at handoff.
The parent owns its integration commit, push, clean-worktree check, and parity;
none is claimed for the pending docs diff.

## Sub-agent session close status

Implementation/test/review lanes completed the accepted cut and reviews. This
writer changed only authorized Canon/roadmap/report/status/reader surfaces,
spawned no sub-agent, and changed no production/test/Lean file. The independently
modified HTML test remains outside this writer's ownership. Final validation
results and exact changed files are handed to the parent; docs commit/push is
parent-owned.
