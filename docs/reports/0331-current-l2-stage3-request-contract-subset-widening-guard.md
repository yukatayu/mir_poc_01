# Report 0331 — current L2 stage3 request contract subset widening guard

- Date: 2026-04-08T08:48:00Z
- Author / agent: Codex
- Scope: spec109 actualization 後に request head metadata を still later に残したまま contract-only compare surface をどこまで widening してよいかの docs-only comparison
- Decision levels touched: L2

## 1. Objective

`Stage3RequestContractSubset` first tranche が通った後に、current compare family を row-list へ広げるべきか、それとも 0-or-1 guard を維持するべきかを source-backed に整理する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/02-current-l2-ast-fixture-schema.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md`
- `specs/examples/108-current-l2-stage3-request-contract-subset-compare-cut.md`
- `specs/examples/109-current-l2-stage3-request-contract-subset-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `crates/mir-ast/tests/fixtures/current-l2/`

## 3. Actions taken

1. fixture corpus 全体を scan し、`contract.require` / `contract.ensure` array に 2 個以上の predicate row を持つ current fixture が無いことを確認した。
2. source-side suite bridge / fixture-side contract subset / row-list widening の 3 案を比較した。
3. current next cut は row-list widening ではなく、0-or-1 guard 維持だと `specs/examples/110...` に整理した。

## 4. Files changed

- `specs/examples/110-current-l2-stage3-request-contract-subset-widening-guard.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0331-current-l2-stage3-request-contract-subset-widening-guard.md`

## 5. Commands run and exact outputs

```bash
python3 - <<'PY'
import json, pathlib
root = pathlib.Path('crates/mir-ast/tests/fixtures/current-l2')
for path in sorted(root.glob('*.json')):
    if path.name.endswith('.host-plan.json'):
        continue
    data = json.loads(path.read_text())
    found=[]
    def walk(node):
        if isinstance(node, dict):
            kind=node.get('kind')
            if kind in ('PerformOn','PerformVia'):
                contract=node.get('contract',{})
                for clause in ('require','ensure'):
                    arr=contract.get(clause)
                    if isinstance(arr,list) and len(arr)>1:
                        found.append((kind, clause, len(arr)))
            for v in node.values():
                walk(v)
        elif isinstance(node, list):
            for v in node: walk(v)
    walk(data.get('program'))
    if found:
        print(path.name, found)
PY
```

主要出力:

```text
<no output>
```

## 6. Evidence / findings

- current fixture corpus には multi-row clause array anchor が無い。
- source-side suite bridge は still fixed two-slot / one-fragment-per-slot shape である。
- row-list widening を今ここで入れると、fixture-sideだけ先に太るか、source-side family 自体を早く崩すかのどちらかになりやすい。

## 7. Changes in understanding

- current compare family は still 0-or-1 guard に留める方が staged line に合う。
- explicit `and` が already あるため、multi-row clause array を later に送ることは current L2 の表現力の直ちの欠損ではない。

## 8. Open questions

- next は source-side suite bridge 側の widening entry criteria を比べるか、それともこの family を一旦 freeze して別 Phase 3 subline に戻るか。

## 9. Suggested next prompt

`specs/examples/110-current-l2-stage3-request-contract-subset-widening-guard.md` を前提に、source-side suite bridge widening の entry criteria を先に比較するか、それとも request contract subset family を一旦 freeze して別 Phase 3 subline に戻るかを narrow に比較してください。
