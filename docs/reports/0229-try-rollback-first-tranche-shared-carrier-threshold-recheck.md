# 0229 — try-rollback first-tranche shared carrier threshold recheck

## Objective

`TryFallback` / `AtomicCut` dedicated AST structural helper first tranche actualization と
wording stability judgment を前提に、saved artifact compare need の観点で
shared detached carrier threshold が本当に満たされたかを source-backed に再比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- current task は docs-only comparison と mirror 更新だけを扱う。
- shared detached carrier の actual 実装や reference-only mirror actualization は行わない。
- current helper-local checker が `fixture_path` と `artifact_path` を直接受けることを actual evidence に使う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/60-current-l2-try-rollback-ast-helper-shared-carrier-threshold.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/69-current-l2-try-rollback-second-malformed-static-tranche-comparison.md`
- `specs/examples/70-current-l2-try-rollback-first-tranche-wording-stability.md`
- `plan/00-index.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `docs/reports/0222-try-rollback-ast-helper-first-tranche-actualization.md`
- `docs/reports/0226-try-rollback-first-tranche-wording-stability.md`
- `crates/mir-semantics/src/lib.rs`
- `scripts/current_l2_try_rollback_structural_checker.py`
- `scripts/current_l2_detached_loop.py`

## Actions taken

1. old threshold doc (`specs/examples/60`) の前提と、current actualized first tranche の差分を棚卸しした。
2. shared detached carrier へ上げる案、helper-local dedicated contract に留める案、reference-only mirror を足す案を比較した。
3. `scripts/current_l2_try_rollback_structural_checker.py` が saved artifact path を直接 compare できることを actual evidence として確認した。
4. `specs/examples/71-current-l2-try-rollback-first-tranche-shared-carrier-threshold-recheck.md` を追加し、current phase では threshold 未充足と読む judgment を整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/07-parser-free-poc-stack.md`、`plan/09-helper-stack-and-responsibility-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/71-current-l2-try-rollback-first-tranche-shared-carrier-threshold-recheck.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 scripts/current_l2_try_rollback_structural_checker.py crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json target/current-l2-detached/static-gates/wording-stability-e23/e23-malformed-try-fallback-missing-fallback-body.static-gate.json
python3 scripts/current_l2_try_rollback_structural_checker.py crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json target/current-l2-detached/static-gates/wording-stability-e24/e24-malformed-atomic-cut-fallback-placement.static-gate.json
python3 scripts/validate_docs.py
git diff --check
git status --short --branch
```

## Evidence / outputs / test results

- direct saved artifact compare for `e23`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- direct saved artifact compare for `e24`

```text
status: matched
fixture structural verdict: findings_present
actual structural verdict: findings_present
```

- local inspection
  - current helper-local checker は `fixture_path` と emitted static gate `artifact_path` を直接受ける
  - したがって saved artifact compare need の narrow version は、shared detached carrier を増やさなくても current helper-local contract で満たせる

- `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
```

- `git diff --check`

```text
<no output>
```

## What changed in understanding

- `specs/examples/60` が想定していた threshold のうち、first 4 条件は current first tranche で概ね満たした。
- ただし 5 番目の detached compare need は、「shared detached carrier が無いと困る need」まではまだ source-backed でない。
- current actual need は helper-local checker の `fixture + saved artifact` compare で十分なので、threshold はまだ未充足と読む方が自然である。

## Open questions

- artifact-to-artifact diff や shared consumer が later に必要になるか。
- generic structural checker family / public checker API comparison の前提条件をどこまで narrow に切るか。

## Suggested next prompt

current first-tranche shared carrier threshold recheck を前提に、`TryFallback` / `AtomicCut` dedicated AST structural helper family を generic structural checker family / public checker API comparison に載せる前提条件を、first-tranche actual state から narrow に整理してください。
