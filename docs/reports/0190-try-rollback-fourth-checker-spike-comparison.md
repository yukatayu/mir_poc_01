# 0190 — try-rollback fourth checker spike comparison

## Objective

`TryFallback` / `AtomicCut` の structural floor を current checker helper family の fourth spike として今 actualize すべきか、それとも current phase では docs/runtime representative に留めるべきかを source-backed に比較する。

## Scope and assumptions

- current L2 parser-free PoC を前提にする。
- same-lineage / missing-option / capability の既存 three spikes は維持する。
- new typed reason family や new helper 実装は、必要性があると判断できた場合にだけ次段で行う。
- 今回は comparison と docs/plan mirror 更新だけを扱う。

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/46-current-l2-same-lineage-first-checker-spike.md`
- `specs/examples/47-current-l2-missing-option-second-checker-spike.md`
- `specs/examples/48-current-l2-capability-third-checker-spike.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `scripts/current_l2_family_checker_support.py`
- `scripts/current_l2_same_lineage_checker.py`
- `scripts/current_l2_missing_option_checker.py`
- `scripts/current_l2_capability_checker.py`
- `scripts/current_l2_reason_code_readiness.py`
- `crates/mir-semantics/src/lib.rs`

## Actions taken

1. existing three checker spikes が fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` の row family compare に依存していることを再確認した。
2. `TryFallback` / `AtomicCut` については、current docs で structural floor と runtime/proof boundary が整理済みである一方、対応する `StaticReasonCodeRow` family と static malformed cluster はまだ無いことを確認した。
3. fourth spike の候補を
   - existing reason-row family helper へ載せる案
   - dedicated AST structural helper を別に切る案
   - current phase では docs/runtime representative に留める案
   の 3 案で比較した。
4. current judgment として、fourth spike actualization はまだ切らず、`specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md` を追加して docs/runtime representative に留める方針を固定した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/11-roadmap-near-term.md`、`plan/12-open-problems-and-risks.md`、`plan/90-source-traceability.md`、`progress.md` を更新した。

## Files changed

- `specs/examples/52-current-l2-try-rollback-fourth-checker-spike-comparison.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`

## Commands run

```bash
python3 scripts/validate_docs.py
git diff --check
```

## Evidence / outputs / test results

- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - numbered reports count が current chain に追従している
- `git diff --check`
  - 無出力
- local inspection で確認したこと
  - existing three spikes はすべて `checked_reason_codes` / detached `reason_codes` row family compare に依存している
  - current `StaticReasonCodeRow` inventory に `try` / rollback locality family は存在しない
  - `E2` / `E21` / `E22` は runtime valid fixture であり、static malformed / underdeclared cluster と同じ compare pattern にまだ載らない

## What changed in understanding

- `TryFallback` / `AtomicCut` の structural floor は first checker cut 候補 cluster ではあるが、current phase では existing three spikes と同じ reason-row family helper に載せるより、docs/runtime representative に留める方が自然である。
- future fourth spike を切るなら、existing row-family helper の延長ではなく dedicated AST structural helper として比較し直す方が筋が良い。

## Open questions

- parser boundary と first checker API cut がもう一段見えた時点で、dedicated AST structural helper を本当に切るべきか。
- `TryFallback` / `AtomicCut` の static malformed family を増やす必要があるか。
- rollback locality を theorem prover / model checker 側へ送る時の最小 carrier は何か。

## Suggested next prompt

current parser boundary inventory と first checker API cut を前提に、`TryFallback` / `AtomicCut` structural floor を dedicated AST structural helper として切るべき entry criteria を narrow に整理してください。
