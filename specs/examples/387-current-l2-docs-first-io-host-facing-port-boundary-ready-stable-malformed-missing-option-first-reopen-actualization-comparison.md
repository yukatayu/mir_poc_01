# 387 — current L2 docs-first-io-host-facing-port-boundary-ready stable-malformed-missing-option-first-reopen-actualization comparison

## 目的

`specs/examples/386-current-l2-docs-first-io-host-facing-port-boundary-ready-minimal-docs-first-io-host-facing-port-boundary-threshold.md`
で docs-first I/O / host-facing port boundary の minimum を fixed した次段として、

- missing-option family first reopen を helper-local compare に留めるか、source-backed widening first へ進めるか
- `e16/e17/e18` triplet を first reopen family として扱うか、さらに narrower な single-row lead で固定するか
- capability second、duplicate later、`TryFallback` / `AtomicCut` malformed-static later の順序を崩さずに current first actualization cut をどこへ置くか

を比較する。

ここで固定するのは
**current L2 docs-first-io-host-facing-port-boundary-ready stable-malformed-missing-option-first-reopen-actualization comparison**
であり、

- actual source-backed widening 実装
- capability second reopen actualization
- duplicate cluster / try-rollback malformed-static family の actual promotion

はまだ固定しない。

## scope

- current package は docs-only actualization comparison に留める。
- `specs/examples/369...370` の broader malformed inventory、`specs/examples/47` の helper-local checker spike、current source-sample runner / ladder / inventory を entry evidence として使う。
- new sample / code / fixture は増やさない。

## current 前提

current repo では次が成立している。

1. `specs/examples/369...370` により、stable malformed broader follow-up inventory は missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`)、duplicate later、`TryFallback` / `AtomicCut` malformed-static family later に固定済みである。
2. `specs/examples/47` により、missing-option structure floor は helper-local second checker spike として actual evidence を持つ。
3. `scripts/current_l2_missing_option_checker.py` と `scripts/current_l2_detached_loop.py smoke-missing-option-checker` により、missing-option helper-local compare は static gate artifact に対する non-production evidence として再利用できる。
4. current source-sample corpus は authored octet `e1/e2/e3/e4/e19/e21/e22/e23` に留まり、`e16/e17/e18` はまだ source-backed wideningされていない。

したがって current 問いは、
**missing-option family first reopen の actualization を helper-local compare だけで止めず、source-backed widening first へ進める current cut をどこで narrow に固定するのが最小か**
である。

## 比較観点

1. existing helper-local compare を entry evidence として再利用できるか
2. first reopen family を `e16/e17/e18` triplet として残せるか
3. implementation lead を必要なら `e16` に寄せつつ、family judgment を narrow に保てるか
4. capability second、duplicate later、try-rollback malformed-static later を崩さずに済むか

## 比較対象

### 案 1. helper-local compare を entry evidence に使いつつ、first actualization target を source-backed widening first に置く

#### shape

```text
stable_malformed_missing_option_first_reopen = {
  actualization_kind = current_l2_missing_option_source_backed_first_reopen,
  chosen_reopen_family = missing_option_structure_floor,
  chosen_rows = [e16, e17, e18],
  actualization_mode = source_backed_widening_first,
  entry_evidence = [
    helper_local_missing_option_compare,
    stable_malformed_broader_followup_inventory
  ],
  staging_note = [
    e16_lead_implementation_cut_allowed,
    capability_second_kept_later
  ]
}
```

#### 利点

- helper-local compare を捨てずに next actualization line へ handoff できる。
- `e16/e17/e18` triplet を first reopen family として保てる。
- source-sample runner / inventory / ladder widening を next package の主眼にできる。

#### 欠点

- actual widening は次段実装に残る。

### 案 2. helper-local compare を current actualization とみなし、source-backed widening をさらに later に送る

#### 利点

- 現状の helper evidence だけで閉じられる。

#### 欠点

- `specs/examples/369...370` が残した first reopen family を source-backed line へ接続できない。
- sample-visible malformed widening が進まない。

### 案 3. first reopen family を `e16` single row に縮める

#### 利点

- 実装 cut は狭い。

#### 欠点

- broader follow-up inventory が残した triplet family judgment を threshold で縮退させやすい。
- `e17/e18` の family continuity が薄くなる。

### 案 4. capability family を同じ package で actualization へ混ぜる

#### 利点

- malformed widening は一見速い。

#### 欠点

- missing-option first / capability second の順序が崩れる。
- duplicate / try-rollback later guard も薄くなる。

## current judgment

current L2 で最も自然なのは、
**案 1. helper-local compare を entry evidence に使いつつ、first actualization target を source-backed widening first に置く**
である。

理由は次の通り。

1. helper-local missing-option compare は already-supported evidence であり、comparison 本体をそこに留めるより source-backed widening line へ handoff する方が current roadmap と整合する。
2. `specs/examples/369...370` が残した first reopen family は `e16/e17/e18` triplet なので、family judgment は維持したまま actualization mode だけを source-backed widening first に固定するのが最小である。
3. capability second、duplicate later、try-rollback malformed-static later の split を保つには、missing-option family を separate package として扱い続ける方が drift が少ない。

## current first choice details

- existing helper-local missing-option compare は entry evidence として再利用する。
- first reopen family は `e16-malformed-missing-chain-head-option` / `e17-malformed-missing-predecessor-option` / `e18-malformed-missing-successor-option` の triplet に置く。
- current actualization mode は source-backed widening first に置く。
- implementation cut を narrower に取る場合でも `e16` lead を許すに留め、family judgment 自体は triplet のまま保つ。
- capability family second (`e13/e20`) は別 package に残す。
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残す。

## next promoted line

next promoted line は、
**stable-malformed-missing-option-first-reopen-actualization-ready final-public-parser-checker-runtime-first-later-gate-actualization comparison**
に置く。

## open questions

- source-backed widening 実装を triplet 一括で行うか、`e16` lead で段階化するか
- `e17` の parser-side anchor を widen 実装時にどこまで補強するか
- capability second reopen を missing-option widen の直後に置くか
- duplicate cluster later gate を malformed family 側と別 line に置き続けるか
