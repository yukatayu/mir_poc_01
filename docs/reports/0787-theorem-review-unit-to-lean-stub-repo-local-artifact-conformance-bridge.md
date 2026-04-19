# 0787 — theorem review-unit to Lean-stub repo-local artifact-conformance bridge

## Objective

theorem Lean-first non-production stub pilot を単発 helper に留めず、
authored current-L2 representative source sample から
formal hook / proof notebook review unit / Lean stub artifact の pair alignment を
repo-local に再現し、
regression bundle に組み込まれた reproducible compare floor として固定する。

## Scope and assumptions

- `specs/` を規範正本として扱う。
- actual Lean tool execution は要求しない。
- current package は review-unit first / Lean-first non-production stub only を保つ。
- representative coverage は runtime `e2-try-fallback` と static `e5-underdeclared-lineage` に置く。
- `p06 / p07 / p08` 全面 trace alignment や public theorem contract や proof object public schema は今回 fixed しない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/475-current-l2-principal-theory-spine-and-lean-first-proof-roadmap.md`
- `specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `sub-agent-pro/codex_theory_handoff_2026-04-18.md`

## Actions taken

1. 既存の RED test `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py` を起点に、missing module error を再確認した。
2. `scripts/current_l2_theorem_lean_stub_pipeline.py` を追加し、sample inventory / formal-hook smoke / proof-notebook review-unit emit / Lean stub emit / pair conformance validation を一体化した repo-local helper を実装した。
3. same `(subject_ref, obligation_kind)` pair、`tool_family = lean-first`、non-empty `source_text` を conformance predicate として固定した。
4. `scripts/current_l2_source_sample_regression.py` に theorem artifact-conformance bridge commands を追加し、`e2` と `e5` representative coverage を regression bundle に統合した。
5. `scripts/tests/test_current_l2_source_sample_regression.py` を更新し、new regression commands と CLI planning を固定した。
6. `specs/examples/509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md` を新設し、current recommendation / retained alternatives / stop line を source-backed に固定した。
7. relevant `Documentation.md` / `tasks.md` / `progress.md` / `plan/` / `specs/` / traceability docs を package 44 前提に同期した。

## Evidence / outputs / test results

- RED:
  - `python3 -m unittest scripts.tests.test_current_l2_theorem_lean_stub_pipeline`
  - `ModuleNotFoundError: No module named 'current_l2_theorem_lean_stub_pipeline'`
- focused GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_theorem_lean_stub_pipeline scripts.tests.test_current_l2_source_sample_regression`
  - `18 tests` passed
- representative runtime sample:
  - `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e2-try-fallback --artifact-root target/current-l2-theorem-lean-stub --run-label package44-e2`
  - review unit `1`
  - Lean stub `1`
  - matched pairs `1`
- representative static sample:
  - `python3 scripts/current_l2_theorem_lean_stub_pipeline.py e5-underdeclared-lineage --artifact-root target/current-l2-theorem-lean-stub --run-label package44-e5`
  - review unit `2`
  - Lean stub `2`
  - matched pairs `2`
- regression integration:
  - `python3 scripts/current_l2_source_sample_regression.py regression --artifact-root target/current-l2-source-sample-regression --run-label package44-regression`
  - `22/22` commands passed

## What changed in understanding

- theorem artifact-conformance bridge は actual Lean tool execution を待たずに repo-local compare floor へ actualize できる。
- review-unit first / Lean-first non-production stub only を保ったまま、runtime/static representative sample coverage を regression bundle に組み込める。
- unresolved なのは artifact conformance そのものではなく、prototype-wide trace alignment、actual Lean execution、public theorem contract、proof object public schema の順である。

## Open questions

- `p06 / p07 / p08` を含む representative prototype trace alignment bridge をどこで close するか。
- actual Lean tool execution を local tool availability / reproducibility とどう結びつけて reopen するか。
- repo-local pair conformance を cross-tool public artifact-conformance contract にいつ昇格させるか。
- theorem result object / payload / proof object schema / verifier contract の remaining mixed gate をどの順に reopen するか。

## Suggested next prompt

`theorem artifact-conformance bridge の次として、representative prototype trace alignment bridge を close するか、それとも order-handoff serial-scope sugar の helper-local reserve surface を actualize するかを source-backed に選んでください。`
