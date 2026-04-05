# 0226 — try-rollback first-tranche wording stability

## Objective

`TryFallback` / `AtomicCut` dedicated AST structural helper first tranche actualization と
second malformed static tranche comparison close を前提に、
`missing_fallback_body` / `disallowed_fallback_placement` の wording と helper-local row family を
current next narrow step でどこまで固定してよいかを source-backed に整理する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- current task は docs-only comparison と mirror 更新だけを扱う。
- new fixture / new `finding_kind` / helper rename / alias 追加は行わない。
- current first tranche の actual contract は `e23` / `e24` / helper-local compare / static gate smoke path とする。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/67-current-l2-try-rollback-malformed-pattern-slot-selection.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `docs/reports/0220-try-rollback-malformed-pattern-slot-selection.md`
- `docs/reports/0222-try-rollback-ast-helper-first-tranche-actualization.md`
- `docs/reports/0224-try-rollback-second-malformed-static-tranche-comparison.md`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/current_l2_detached_loop.py`

## Actions taken

1. current first-tranche wording / row family の source-backed anchor を code, fixture, spec, plan mirror から棚卸しした。
2. next narrow step の選択肢を
   - current exact wording / row family を固定
   - helper-local `finding_kind` だけ先に generic 化
   - alias / synonym layer を先に導入
   の 3 案で比較した。
3. `specs/examples/70-current-l2-try-rollback-first-tranche-wording-stability.md` を追加し、current next phase では exact wording / row family を fixed working set に保つ judgment を整理した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/15-current-l2-fixture-authoring-template.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/70-current-l2-try-rollback-first-tranche-wording-stability.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/15-current-l2-fixture-authoring-template.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json --run-label wording-stability-e23 --overwrite
python3 scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json --run-label wording-stability-e24 --overwrite
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- `smoke-try-rollback-structural-checker` for `e23`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- `smoke-try-rollback-structural-checker` for `e24`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- emitted static gate artifact local inspection
  - `e23` artifact は `checker_core.reasons = [\"try fallback body must not be empty\"]`
  - `e24` artifact は `checker_core.reasons = [\"atomic cut may not appear inside fallback body\"]`
  - helper-local checker は current reason wording を `TryFallback` / `missing_fallback_body` と `AtomicCut` / `disallowed_fallback_placement` に写し、fixture-side expected row と一致した

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
```

- `git diff --check`

```text
<no output>
```

## What changed in understanding

- current first tranche では wording と helper-local row family を今すぐ generic 化したり alias で吸収したりするより、current exact pair を fixed working set として 1 段維持する方が自然である。
- 理由は、current actual contract が `e23` / `e24`、helper bridge、fixture-side expected row、docs mirror の 4 点セットで揃っており、rename や alias の導入は new evidence なしに drift surface だけを増やすからである。

## Open questions

- shared detached carrier threshold の再比較で、本当に saved artifact compare need が満たされるか。
- later generic structural checker family comparison で `disallowed_fallback_placement` をどう generalize するか。

## Suggested next prompt

current first-tranche wording / row family stability judgment を前提に、`TryFallback` / `AtomicCut` dedicated AST structural helper first tranche が saved artifact compare need の観点で shared detached carrier threshold を本当に満たしたかを narrow に再比較してください。
