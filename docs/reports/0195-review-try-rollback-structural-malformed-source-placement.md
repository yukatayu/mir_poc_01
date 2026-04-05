# 0195 — review trail for try-rollback structural malformed source placement

## Objective

`0194` の docs-only comparison に対して review trail を残し、malformed source placement judgment が parser / loader / static gate の責務境界を壊していないかを確認する。

## Scope and assumptions

- review 対象は `specs/examples/54` と mirror 更新だけである。
- current tool surface では stable reviewer handle が無いため、local evidence fallback を採る。

## Documents consulted

- `docs/reports/0194-try-rollback-structural-malformed-source-placement.md`
- `specs/examples/29-current-l2-first-parser-subset-inventory.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/53-current-l2-try-rollback-ast-structural-helper-entry-criteria.md`
- `specs/examples/54-current-l2-try-rollback-structural-malformed-source-placement.md`

## Actions taken

1. current tool surface では stable reviewer handle を起動して completion を待てる callable path が見当たらないことを確認した。
2. local diff inspection で、`specs/examples/54` が concrete syntax malformed / carrier malformed / semantic structural malformed を分けていることを確認した。
3. parser-free current phase では semantic structural malformed を static gate / dedicated helper 側へ置き、loader は schema/decode failure に留める judgment が `specs/examples/30` / `53` と整合していることを確認した。
4. fresh docs validation と whitespace check を review fallback evidence に採用した。

## Evidence / outputs / test results

- current tool surface では usable reviewer handle を確認できなかったため、review trail は local evidence fallback で閉じる。
- local diff inspection では次が揃っている。
  - current parser-free phase で text parser source を前提にしていない
  - loader と static malformed を同じ層へ押し込んでいない
  - dedicated AST structural helper の entry criteria と矛盾しない
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
- `git diff --check`
  - 無出力

## What changed in understanding

- `TryFallback` / `AtomicCut` dedicated helper を actualize する前に、malformed source placement を parser / loader / static gate で分けて固定する価値があることがはっきりした。
- current phase では、semantic structural malformed を parser や loader に premature に押し込まない方が、future parser handoff と checker boundary の両方を保ちやすい。

## Open questions

- malformed static family を actual corpus に入れる密度が本当にあるか。
- future parser 導入時に loader error wording と static malformed wording をどう handoff するか。

## Suggested next prompt

current runtime representative `E2` / `E21` / `E22` と parser/checker boundary inventory を前提に、`TryFallback` / `AtomicCut` の malformed static family を actual corpus に追加する価値があるかを narrow に比較してください。
