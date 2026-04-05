# Report 0132 — current L2 first parser subset inventory

## 1. Title and identifier

- Report 0132
- current L2 first parser subset inventory

## 2. Objective

current L2 companion notation を final grammar へ早く凍らせすぎないようにしつつ、
first parser cut に入れてよい semantic cluster を narrow に棚卸しする。

今回の goal は、

- parser に上げてよい cluster
- まだ companion notation に残す cluster

を source-backed に切り分け、
type system / theorem prover / static analysis workstream への entry criteria を一段具体化することである。

## 3. Scope and assumptions

- current L2 の core semantics、parser-free PoC、failure family は変更しない。
- final parser grammar、final reserved keyword、final punctuation は固定しない。
- 今回は docs-only inventory に留め、code / tests / fixtures は変更しない。
- `plan/` は relevant mirror を更新する。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `progress.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/09-invariants-and-constraints.md`
9. `specs/10-open-questions.md`
10. `specs/11-roadmap-and-workstreams.md`
11. `specs/12-decision-register.md`
12. `specs/examples/01-current-l2-surface-syntax-candidates.md`
13. `plan/06-surface-notation-status.md`
14. `plan/11-roadmap-near-term.md`
15. `plan/12-open-problems-and-risks.md`
16. `plan/13-heavy-future-workstreams.md`
17. `docs/reports/0114-third-remaining-problem-parser-boundary.md`

## 5. Actions taken

1. current companion notation と final grammar 未固定の範囲を再確認した。
2. first parser cut に入れてよい cluster を、current examples / parser-free PoC / heavy workstream entry criteria の 3 軸で比較した。
3. `specs/examples/29-current-l2-first-parser-subset-inventory.md` を追加し、
   - block structure
   - request head
   - statement-local `require` / `ensure`
   - option declaration core
   - option-local `admit`
   - explicit edge-row family
   を first parser cut 候補として整理した。
4. A2 / A1 の exact lexical choice、`contract` block sugar、richer predicate sublanguage、option-local outcome metadata は companion / OPEN に残すと明記した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/06`、`plan/11`、`plan/12`、`plan/13`、`progress.md` を更新し、current understanding を mirror した。

## 6. Evidence / outputs / test results

### docs / diff validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 132 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- parser 問題の current phase は final lexical surface の選択ではなく、semantic cluster の棚卸しとして切る方が自然である。
- explicit edge-row family 自体は first parser cut 候補になりうるが、A2 hanging continuation を唯一の concrete grammar として固定するのはまだ早い。
- type system / theorem prover workstream へ進む前に、parser を narrow boundary cut として整理する道筋が見えてきた。

## 8. Open questions

- first parser cut を actual parser spike に寄せるとき、A2 と A1 のどちらを accepted concrete surface にするか。
- richer predicate grammar を first checker cut とどう同期するか。
- parser cut を static elaboration helper とどう接続するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を 1 段だけ operational に寄せるか、あるいは first parser cut inventory を checker / theorem prover entry criteria に接続する docs-only step を進めるかを source-backed に比較してください。`
