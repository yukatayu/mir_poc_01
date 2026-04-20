# 0860 Package 106 parser companion mapping matrix

## 1. Title and identifier

- 0860 Package 106 parser companion mapping matrix

## 2. Objective

- `p06 / p07 / p08` representative slice について、original prototype / parser companion / guided bundle / Lean artifact / anchor spec-report の対応を docs / helper / traceability で同じ読みに揃える。

## 3. Scope and assumptions

- current target は representative slice だけに限定する。
- final public parser / checker / runtime API、exhaustive sample catalog、final public tutorial surfaceは今回の scope に入れない。
- helper は repo-local / non-production のまま保つ。

## 4. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md`
- `specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md`
- `specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md`
- `specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md`
- `specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/prototype/current-l2-parser-companion/README.md`

## 5. Actions taken

1. `scripts/current_l2_guided_samples.py` に `mapping` subcommand を追加し、representative slice の mapping manifest と pretty/json renderer を actualize した。
2. `scripts/tests/test_current_l2_guided_samples.py` に mapping manifest / renderer / CLI dispatch の RED テストを追加し、失敗確認後に最小実装で通した。
3. `samples/prototype/current-l2-parser-companion/README.md` に mapping command と representative mapping 表を追加した。
4. `specs/examples/580`、snapshot docs、traceability を Package 106 closeout の reading に更新した。

## 6. Files changed

- `scripts/current_l2_guided_samples.py`
- `scripts/tests/test_current_l2_guided_samples.py`
- `samples/prototype/current-l2-parser-companion/README.md`
- `specs/examples/580-current-l2-parser-side-representative-mapping-matrix-actualization.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`

## 7. Commands run

```bash
python3 -m unittest scripts.tests.test_current_l2_guided_samples
python3 scripts/current_l2_guided_samples.py mapping
python3 scripts/current_l2_guided_samples.py mapping --format json
python3 scripts/validate_docs.py
git diff --check
```

## 8. Evidence / outputs / test results

- RED 確認:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - `build_parser_companion_mapping_manifest` / `render_parser_companion_mapping` / `mapping` subcommand 未実装で失敗した。
- GREEN:
  - `python3 -m unittest scripts.tests.test_current_l2_guided_samples`
  - mapping manifest / renderer / CLI dispatch を含む 15 tests が通過した。
- manual check:
  - `python3 scripts/current_l2_guided_samples.py mapping`
  - `p06 / p07 / p08` について prototype / parser companion / guided bundle / Lean artifact / anchor refs が 1 画面で読めることを確認した。
- manual check:
  - `python3 scripts/current_l2_guided_samples.py mapping --format json`
  - helper-local manifest が機械可読 JSON として出ることを確認した。

## 9. What changed in understanding

- parser-side tranche の remaining self-driven line は、parser carrier そのものの widening ではなく、representative slice の readable mapping を drift なく固定することだった。
- bundle helper と parser companion README を別々に読むだけでは導線が二重化しやすいため、mapping helper と README table を同時に置く cut が適切だった。

## 10. Open questions

- reserve / negative sample (`p09 / p13 / p14`) まで同じ mapping matrix を広げるかは still later。
- final public parser / checker / runtime API へどの時点で reconnect するかは still mixed gate。
- exhaustive sample catalog と final tutorial surface は user-spec / later packaging lane に残る。

## 11. Suggested next prompt

- `samples/` 以下に Problem 1 / Problem 2 の explained representative sample bundle を actualize し、current runnable line と parser-side line を日本語 guide で tighter に接続してください。
