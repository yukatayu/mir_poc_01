# Report 0237 — current L2 stage1 parser spike entry criteria

- Date: 2026-04-05T21:32:30.240161Z
- Author / agent: Codex
- Scope: docs-only parser boundary refinement
- Decision levels touched: L2

## 1. Objective

`specs/examples/73-current-l2-first-parser-spike-staging.md` で current next step とした
checker-led staged spike のうち、
stage 1 の chain / declaration structural floor を actual parser spike として切るなら
どこまでを accepted parse cluster に含め、どこから先を non-goal として later stage に残すかを
source-backed に整理する。

## 2. Scope and assumptions

- current L2 の core semantics、parser-free PoC、detached validation loop は変更しない。
- final parser grammar、exact lexical choice、actual parser API は固定しない。
- 今回は docs-only boundary refinement に留め、parser / checker の actual code は変更しない。
- `plan/` は relevant mirror を更新する。

## 3. Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/examples/02-current-l2-ast-fixture-schema.md`
10. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
11. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
12. `specs/examples/73-current-l2-first-parser-spike-staging.md`
13. `plan/00-index.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/90-source-traceability.md`
17. `progress.md`
18. `docs/reports/0235-current-l2-first-parser-spike-staging.md`
19. `docs/reports/0236-review-current-l2-first-parser-spike-staging.md`
20. `crates/mir-semantics/src/lib.rs`
21. `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
22. `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`

## 4. Actions taken

1. first parser cut inventory、first checker cut entry criteria、staged spike judgment の 3 本を読み直し、
   stage 1 が declaration structural floor に限られるべきことを再確認した。
2. current source anchor として、`static_gate_detailed` が実際に見ている floor が
   - option declaration visibility
   - chain edge attachment
   - lineage assertion edge match
   - target presence / equality
   - capability strengthening prohibition
   であることを確認した。
3. current parser-free fixture carrier の `OptionDecl.target / capability / lease / admit` shape を見て、
   declaration-side guard slot を stage 1 から完全に外す案は current carrier との対応を弱めると整理した。
4. stage 1 の declaration-side guard slot について、
   - stage 1 から外す案
   - opaque attached slot として含める案
   - minimal predicate fragment として parse する案
   を比較した。
5. `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md` を追加し、
   stage 1 では declaration-side guard slot を predicate parser の入口ではなく
   opaque attached slot に留める cut を current docs-only judgment として整理した。
6. `Documentation.md`、`specs/00-document-map.md`、`specs/examples/73-current-l2-first-parser-spike-staging.md`、
   `plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`progress.md` を更新した。

## 5. Evidence / outputs / test results

- `crates/mir-semantics/src/lib.rs::static_gate_detailed` は declaration / chain structural floor を見ているが、
  predicate fragment parse 自体には依存していない。
- `crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json` のような stage 1 対象 fixture では、
  `OptionDecl.lease` は present だが `admit` は `null` であり、request/admissibility cluster を前倒ししなくても
  declaration structural floor が見える。
- `crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json` は option-local `admit` が predicate node を持つため、
  stage 1 で predicate fragment parse を入れると stage 3 cluster を先食いしやすい。
- 以上から、stage 1 の declaration-side guard slot は
  parse 境界としては残すが predicate grammar へは進めない cut が最も narrow である。
- `python3 scripts/validate_docs.py`
  - Output: `Documentation scaffold looks complete.`
  - Output: `Found 238 numbered report(s).`
- `git diff --check`
  - Output: no output

## 6. What changed in understanding

- stage 1 parser spike の争点は、option declaration core に `lease` slot を残すかどうかではなく、
  `lease` slot を **どこまで解釈するか** にある。
- current repo では `lease` slot の existence / attachment / boundary だけを stage 1 に入れ、
  guard fragment parse / well-formedness は stage 3 に残す方が、
  checker-led staged spike の意味を保ちやすい。
- stage 1 の parser boundary は、same-lineage / target / capability floor の structural handoff を先に作る task と読むのが自然である。

## 7. Open questions

- declaration-side guard slot の actual carrier 名を何にするか。
- opaque attached slot を raw token slice / raw text / opaque leaf のどれで持つのが最小か。
- stage 1 actual parser spike の smoke family を `e4/e12/e13/e16/e17/e18/e19/e20` のどこまでにするか。
- stage 3 で predicate fragment floor と option-local `admit` を同じ parser spike に載せるかどうか。

## 8. Suggested next prompt

`current L2 parser-free PoC 基盤と first parser cut inventory / checker-led staged spike を前提に、stage 1 parser spike の declaration-side guard slot を actual parser / checker handoff へ送るなら、opaque attached slot の actual carrier 名と parser-free AST fixture schema との最小 handoff cutを narrow に比較してください。`

## 9. Files changed

- `specs/examples/74-current-l2-stage1-parser-spike-entry-criteria.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## 10. Commands run

- `git status --short --branch`
- `python3 scripts/new_report.py --slug current-l2-stage1-parser-spike-entry-criteria`
- `sed -n '1,220p' README.md`
- `sed -n '1,220p' Documentation.md`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,240p' specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `sed -n '1,260p' specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `sed -n '1,260p' specs/examples/73-current-l2-first-parser-spike-staging.md`
- `sed -n '1,260p' specs/examples/02-current-l2-ast-fixture-schema.md`
- `sed -n '1,140p' crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json`
- `sed -n '1,140p' crates/mir-ast/tests/fixtures/current-l2/e3-option-admit-chain.json`
- `sed -n '1369,1474p' crates/mir-semantics/src/lib.rs`
- `python3 scripts/validate_docs.py`
- `git diff --check`
