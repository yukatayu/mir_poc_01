# Report 0135 — current L2 first checker cut entry criteria

## 1. Title and identifier

- Report 0135
- current L2 first checker cut entry criteria

## 2. Objective

current L2 parser-free PoC と first parser cut inventory を前提に、
first checker cut に入れてよい local / structural judgment と、
external verifier / theorem prover / model checker 側へ残すべき global property の境界を
docs-only で narrow に切り出す。

今回の goal は、

- language core judgment に寄せてよい floor
- heavy workstream 側へ残す property

を source-backed に分け、
small decidable core へ進む前提を一段具体化することである。

## 3. Scope and assumptions

- current L2 の core semantics、parser-free PoC、failure family は変更しない。
- final type system、final theorem prover relation、final model checker boundary は固定しない。
- 今回は docs-only entry criteria に留め、code / tests / fixtures は変更しない。
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
12. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
13. `plan/11-roadmap-near-term.md`
14. `plan/12-open-problems-and-risks.md`
15. `plan/13-heavy-future-workstreams.md`
16. `plan/90-source-traceability.md`
17. `docs/reports/0116-fifth-remaining-problem-static-analysis-and-proof-boundary.md`

## 5. Actions taken

1. static analysis / type / theorem prover boundary の current understanding を読み直し、first parser cut の次段として first checker cut のみを narrow scope に切る方針を再確認した。
2. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md` を追加し、
   - same-lineage static evidence floor
   - malformed / underdeclared rejection
   - minimal capability strengthening prohibition
   - request-local / option-local clause attachment
   - minimal predicate fragment well-formedness
   - `try` / rollback locality の structural floor
   を first checker cut 候補として整理した。
3. canonical normalization の一般証明、no re-promotion、rollback / cut non-interference、membership churn、scheduler などの global property は theorem prover / model checker 側へ残すと明記した。
4. `Documentation.md`、`specs/00-document-map.md`、`plan/11`、`plan/12`、`plan/13`、`progress.md`、`plan/90` を更新し、current understanding を mirror した。

## 6. Evidence / outputs / test results

### docs / diff validation

```text
python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 136 numbered report(s).
```

```text
git diff --check
```

無出力。

## 7. What changed in understanding

- type system / theorem prover 問題は、強い system を一気に language core へ入れるのではなく、first checker cut で local / structural / decidable floor だけを切る方が current repo の phase に合う。
- first parser cut inventory と first checker cut entry criteria を分けることで、parse する cluster と checker が静的に判定する property を別々に ratchet できる。
- heavy workstream を language core へ押し込まない current L2 の方針を保ったまま、checker 側へ寄せてよい最小核はかなり具体化できる。

## 8. Open questions

- first checker cut を actual checker spike に寄せるとき、artifact form を AST fixture 補助にどう接続するか。
- theorem prover 向け core relation をどの semantic layer で切り出すか。
- model checker 候補 property をどの subsystem から先に有限化するか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、aggregate export の actual narrow API cut を operational に 1 段寄せるか、あるいは first checker cut entry criteria を actual checker spike の artifact form に接続する docs-only step を進めるかを source-backed に比較してください。`
