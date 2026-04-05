# Report 0243 — current L2 stage 1 parser smoke family working set

- Date: 2026-04-05T22:20:01Z
- Author / agent: Codex
- Scope: docs-only comparison for stage 1 parser smoke family working set
- Decision levels touched: L2

## 1. Title and identifier

- Report 0243
- current L2 stage 1 parser smoke family working set

## 2. Objective

`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md` の次段として、
actual parser spike の stage 1 smoke family をどの fixture pair / trio で始めるのが最小かを比較し、
working set を narrow に固定する。

## 3. Scope and assumptions

- current L2 の core semantics は変更しない。
- stage 1 は chain / declaration structural floor に留める。
- declaration-side guard slot は opaque attached slot のままにする。
- `admit` predicate parse / request-local clause parse は stage 1 に入れない。
- `plan/` は今回更新対象である。

## 4. Documents consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/75-current-l2-stage1-parser-guard-slot-handoff.md`
- `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `plan/00-index.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- fixture `crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

## 5. Actions taken

1. spec 76 の naming / bridge judgment を読み直し、actual parser spike の next narrow step が smoke family 選定であることを確認した。
2. `e3`、`e4`、`e7` の役割を fixture corpus 上で比較した。
3. 次の 3 working-set 案を比較した。
   - `e4` 単独
   - `e4` + `e7`
   - `e4` + `e7` + `e3`
4. `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md` を追加し、`e4` + `e7` を最小 working set、`e3` を later-stage contrast reference に残す current judgment を整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を mirror 更新した。
6. reviewer follow-up として、`specs/examples/02-current-l2-ast-fixture-schema.md` の fixture catalog wording を current parser-boundary reading に合わせ、`progress.md` の簡潔ログを時系列順へ整えた。

## 6. Evidence / outputs / test results

### Commands run

- `git status --short --branch`
- `python3 scripts/new_report.py --slug current-l2-stage1-parser-smoke-family-working-set`
- `sed -n '1,240p' specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- `sed -n '1,160p' crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `sed -n '1,160p' crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `sed -n '1,180p' crates/mir-ast/tests/fixtures/current-l2/e7-write-fallback-after-expiry.json`

### 読み取り evidence

- `e4` は declaration structural floor と malformed static stop の最小 contrast を与える。
- `e7` は valid chain declaration と mixed `lease` carrier を与える。
- `e3` は `admit` predicate node を含み、stage 1 non-goal を active smoke に引き込みやすい。
- したがって `e4` + `e7` で malformed / valid と mixed `lease` anchor の contrast は足りる一方、`e3` は later-stage contrast reference として残す方が current stage split に合う。

## 7. What changed in understanding

- stage 1 smoke family では malformed / valid と `lease` anchor の contrast が要るが、`admit` contrast までは active smoke に入れない方がよい。
- `e4` + `e7` は declaration structural floor と `decl_guard_slot -> OptionDecl.lease` bridge を確認する working set として最小である。
- `e3` は stage 1 active smoke に上げると request / admissibility cluster を先食いしやすく、current phase では contrast reference に留めるのが自然である。

## 8. Open questions

- actual parser spike で smoke を parser-side AST compare にするか fixture-side lowering compare にするか。
- stage 1 private module / helper の配置をどこに置くか。
- `e3` を active smoke に昇格させる条件を何にするか。

## 9. Suggested next prompt

`specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md` を前提に、stage 1 actual parser spike を置く private module / helper の最小配置と compare surface を narrow に比較してください。

## 10. Files changed

- Added: `specs/examples/77-current-l2-stage1-parser-smoke-family-working-set.md`
- Modified: `Documentation.md`
- Modified: `specs/00-document-map.md`
- Modified: `specs/examples/02-current-l2-ast-fixture-schema.md`
- Modified: `specs/examples/76-current-l2-stage1-parser-opaque-slot-carrier-and-bridge-api.md`
- Modified: `plan/11-roadmap-near-term.md`
- Modified: `plan/12-open-problems-and-risks.md`
- Modified: `plan/90-source-traceability.md`
- Modified: `progress.md`

## 11. plan/ update note

- `plan/` 更新あり: `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`
