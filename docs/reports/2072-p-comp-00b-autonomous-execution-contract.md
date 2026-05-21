# 2072 — P-COMP-00B autonomous execution contract

## Objective

Integrate the computational-core handoff follow-up into an implementation-ready autonomous execution contract, without starting runtime implementation.

The package fixes how an agent should proceed after the user gives an execution request: docs/scaffold front half first, implementation half second, no silent final-product commitments, validation/report/commit/push at every package close.

## Scope and assumptions

- Scope is documentation, planning, source hierarchy validators, and reviewer integration only.
- Runtime implementation, new sample roots, and new helper scripts are intentionally not created in this package.
- `specs/` remain normative, `plan/` remains repository memory, and `progress.md` / `tasks.md` remain the live queue snapshot.
- The existing Product Alpha-1 and operational suite runnable workflows remain valid floor evidence, but current typed external `AddOne` remains host-boundary evidence only.

## Start state / dirty state

Initial dirty state observed at the start of this closeout:

```text
 M Documentation.md
 M README.md
 M progress.md
 M samples_progress.md
 M specs/00-document-map.md
 M specs/29-transform-posegraph-semantics.md
 M tasks.md
?? docs/hands_on/autonomous_execution_01.md
?? docs/research_abstract/autonomous_execution_01.md
?? plan/57-autonomous-computational-core-master-plan.md
?? specs/32-autonomous-execution-and-completion-contract.md
```

Resource snapshot before validation:

```text
df -h .: /dev/vda2 99G, 74G used, 21G available, 79%
free -h: 960Mi total memory, 331Mi available, 19Gi swap
```

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/28-mir-computational-core.md`
- `specs/29-transform-posegraph-semantics.md`
- `specs/30-projection-and-backend-boundary.md`
- `specs/31-engine-wasm-ffi-adapter-boundary.md`
- `plan/00-index.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/53-mir-computational-core-roadmap.md`
- `plan/54-transform-posegraph-roadmap.md`
- `plan/55-projection-backend-roadmap.md`
- `plan/56-engine-adapter-roadmap.md`
- `plan/90-source-traceability.md`
- `sub-agent-pro/mirrorea_mir_computational_core_handoff.md`
- Sub-agent reviewer findings from Wegener, Chandrasekhar, Hume, and Dalton.

## Actions taken

- Added `specs/32-autonomous-execution-and-completion-contract.md`.
- Added `plan/57-autonomous-computational-core-master-plan.md`.
- Added reader-facing autonomous execution guides under `docs/hands_on/` and `docs/research_abstract/`.
- Reordered the autonomous line into a docs/scaffold front half followed by an implementation half.
- Integrated reviewer findings:
  - `P-COMP-01` is scaffold actualization, not runtime completion.
  - `P-COMP-02` should introduce a narrow `mir-semantics` computational module before runtime event wrapping.
  - PoseGraph now distinguishes `Anchor`, `AnchorBinding`, `AnchorSwitch`, frontier ordering, and stale-anchor reacquire.
  - Projection now has a manifest/provider compatibility relation.
  - Engine/provider contracts now include rollback/replay/cut policy.
  - Backend realization, bounded native/WASM provider admission, and final engine adapter ABI remain user-spec-required or kept-later gates.
- Updated validators and documentation indexes to include `specs/32`, `plan/57`, and the new reader-facing docs.
- Updated `progress.md`, `tasks.md`, and `samples_progress.md` to keep the current queue and planned/runnable status explicit.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/29-transform-posegraph-semantics.md`
- `specs/30-projection-and-backend-boundary.md`
- `specs/31-engine-wasm-ffi-adapter-boundary.md`
- `specs/32-autonomous-execution-and-completion-contract.md`
- `plan/00-index.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/54-transform-posegraph-roadmap.md`
- `plan/55-projection-backend-roadmap.md`
- `plan/56-engine-adapter-roadmap.md`
- `plan/57-autonomous-computational-core-master-plan.md`
- `plan/90-source-traceability.md`
- `docs/hands_on/README.md`
- `docs/hands_on/autonomous_execution_01.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/autonomous_execution_01.md`
- `scripts/README.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- `scripts/tests/test_validate_docs.py`
- `docs/reports/2072-p-comp-00b-autonomous-execution-contract.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
df -h .
free -h
git status --short
rg ...
sed -n ...
git diff --stat
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

Validation passed:

```text
python3 -m unittest scripts.tests.test_validate_docs
  Ran 13 tests in 0.086s
  OK

python3 scripts/check_source_hierarchy.py
  required: 171
  present: 171
  missing: 0

python3 scripts/validate_docs.py
  Documentation scaffold looks complete.
  Found 1224 numbered report(s).

cargo fmt --check
  pass

git diff --check
  pass
```

## What changed in understanding

- The autonomous chain must not be one undifferentiated queue. It needs a named front-half closeout after `P-COMP-01`, `P-POSE-01`, `P-PROJ-01`, and `P-ENG-01`.
- `P-COMP-01` was already conceptually satisfied by docs; the next useful work is planned-only scaffold actualization with machine-readable rejected runs.
- Current executable `AddOne` remains adapter-owned. Mir-owned computation requires a new narrow computational layer, most directly under `mir-semantics`.
- PoseGraph, projection, and engine boundaries needed additional carrier/policy fields before they could guide autonomous implementation safely.

## Open questions

- Final public grammar / ABI / SDK remain unresolved.
- Backend realization beyond inventory remains unresolved.
- Bounded native/WASM provider admission remains unresolved.
- Final shared-space catalog breadth and public distribution shape remain user-spec-required.

## Suggested next prompt

Proceed with `P-COMP-01`: create the planned-only computational sample root, matrix, helper, and tests, with `run comp-02-pure-add-one` rejecting as `planned_only` until `P-COMP-02`.

## Plan update status

Updated `plan/00-index.md`, `plan/19-repository-map-and-taxonomy.md`, `plan/54-transform-posegraph-roadmap.md`, `plan/55-projection-backend-roadmap.md`, `plan/56-engine-adapter-roadmap.md`, `plan/57-autonomous-computational-core-master-plan.md`, and `plan/90-source-traceability.md`.

## Documentation.md update status

Updated. `Documentation.md` now includes the autonomous execution contract and avoids a stale fixed count in the layer-reading section.

## progress.md update status

Updated. `progress.md` now records `P-COMP-00B`, the front-half / implementation-half split, and the new backend/native/WASM user-spec-required gates.

## tasks.md update status

Updated. `tasks.md` now gives the ordered autonomous package map, stronger `P-COMP-01` / `P-COMP-02` stop lines, and explicit backend/native/WASM user-decision gates.

## samples_progress.md update status

Updated. `samples_progress.md` keeps computational and PoseGraph rows planned-only and records that no runnable roots/helpers were added.

## Reviewer findings and follow-up

- Wegener found missing front-half stop line, missing backend/native/WASM user gates, and weak `P-COMP-02` negative guard. Follow-up: added front-half closeout, user gates, and `not_mir_owned` negative evidence requirement.
- Chandrasekhar found `P-COMP-01` should be scaffold actualization and `P-COMP-02..04` need a new computational AST/evaluator under `mir-semantics`. Follow-up: updated `plan/57`, `tasks.md`, and reader docs.
- Hume found missing `Anchor`, PoseGraph plan wording drift, missing manifest/provider compatibility, missing provider rollback/cut policy, and missing anchor-switch ordering. Follow-up: updated `specs/29..31` and `plan/54..56`.
- Dalton found the master plan must not become a second queue authority and must be indexed. Follow-up: `specs/32` / `plan/57` explicitly preserve source hierarchy and are indexed in document maps and validators.

## Skipped validations and reasons

Runtime/Product Alpha commands were not rerun because this package changes docs, plans, and validators only. No runtime source, sample package, or CLI behavior was changed.

## Commit / push status

Pending at report write. The commit hash and push result are recorded in the final response because this report is included in the same commit it describes.

## Sub-agent session close status

All four reviewer sub-agent sessions were closed after completion:

- Wegener: completed and closed.
- Chandrasekhar: completed and closed.
- Hume: completed and closed.
- Dalton: completed and closed.
