# 397 — current L2 final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready stable-malformed-capability-second-reopen-actualization comparison

## 目的

`specs/examples/396-current-l2-final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready-minimal-final-public-parser-checker-runtime-thin-facade-later-support-threshold.md`
で thin-facade later-support cut の minimum を fixed した次段として、

- missing-option first source-backed widening close を巻き戻さずに、capability family (`e13/e20`) をどの reopen cut から next malformed-side actualization へ送るか
- helper-local capability compare、stage1 reconnect widen、fixture-static evidence をどこまで entry evidence に再利用するか
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family を later に残したまま、current family judgment をどこまで narrow に保つか

を比較する。

ここで固定するのは
**current L2 final-public-parser-checker-runtime-thin-facade-later-support-actualization-ready stable-malformed-capability-second-reopen-actualization comparison**
であり、

- actual source-backed widening 実装
- public operational CLI concrete shell actualization
- duplicate cluster actualization
- `TryFallback` / `AtomicCut` malformed-static broader promotion

はまだ固定しない。

## scope

- current package は docs-only comparison に留める。
- `specs/examples/369...370` の broader malformed inventory、`specs/examples/48` の capability third checker spike、stage1 parser reconnect widen evidence、fixture-static detached artifact evidenceを entry に使う。
- new sample / code / fixture は増やさない。
- missing-option first / capability second の sequencing は巻き戻さない。

## current 前提

current repo では次が成立している。

1. `specs/examples/369...370` により、broader stable malformed next reopen order は missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`)、duplicate later、`TryFallback` / `AtomicCut` malformed-static later に固定済みである。
2. `specs/examples/48` と `scripts/current_l2_capability_checker.py` により、capability strengthening floor は helper-local third checker spike として actual evidence を already 持つ。
3. `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` により、`e13` / `e20` の source-text form と stage1 reconnect summary は current parser-side evidence として already ある。
4. fixture-side と static gate detached artifact 側では、`e13` / `e20` ともに `capability_strengthens` row を stable actual wording として already 持つ。
5. current authored source sample corpus と runner accepted set には `e13` / `e20` はまだ入っていない。

したがって current 問いは、
**helper-local evidence と stage1 reconnect widen を捨てずに、capability family を `e13/e20` pair のまま next source-backed widening actualization へ送るのが最小か**
である。

## 比較観点

1. existing helper-local capability compare を entry evidence に再利用できるか
2. current family judgment を `e13/e20` pair のまま保てるか
3. stage1 reconnect widen evidence を actual source-backed widening への bridge として使えるか
4. missing-option first / capability second / duplicate later / try-rollback later の ordering を保てるか

## 比較対象

### 案 1. helper-local capability compare を entry evidence に使いつつ、family judgment は `e13/e20` pair のまま source-backed widening first に送る

#### shape

```text
stable_malformed_capability_second_reopen = {
  actualization_kind = current_l2_capability_source_backed_second_reopen,
  chosen_reopen_family = capability_strengthening_floor,
  chosen_rows = [e13, e20],
  entry_evidence = [
    helper_local_capability_compare,
    stage1_parser_reconnect_widening,
    fixture_static_gate_capability_rows,
    stable_malformed_broader_followup_inventory
  ],
  actualization_mode = source_backed_widening_first,
  staging_note = [
    e13_lead_allowed_only_as_implementation_cut
  ]
}
```

#### 利点

- capability family を missing-option first reopen と同じ docs-first -> source-backed widening line に接続できる。
- `e13/e20` pair judgment を shrink せずに済む。
- parser-side special-case を増やさず、current source-backed widening の next package へ handoff しやすい。

#### 欠点

- actual source sample / runner / ladder widening は次段実装に残る。

### 案 2. helper-local capability compare を current stop line とみなし、source-backed widening はさらに later に送る

#### 利点

- 追加 actualization を急がずに済む。

#### 欠点

- capability family だけ source-backed widening line に未接続のまま残る。
- missing-option line に対して malformed widening の symmetry が弱い。

### 案 3. current family judgment を `e13` single-row lead へ narrow に縮める

#### 利点

- 次段実装 cut は最も狭い。

#### 欠点

- `e20` late-edge variant を family core から外しやすい。
- broader follow-up inventory が残した `e13/e20` pair judgment を threshold 側で弱めやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. helper-local capability compare を entry evidence に使いつつ、family judgment は `e13/e20` pair のまま source-backed widening first に送る**
である。

理由は次の通り。

1. current package は actualization comparison であり、helper-local stop ではなく next malformed-side actualization cut を narrow に選ぶ方が自然である。
2. `e13` / `e20` は fixture-static evidence、detached capability checker evidence、stage1 source-text evidenceの 3 方向で already 揃っており、single-row に縮める理由が薄い。
3. missing-option first / capability second / duplicate later / try-rollback later の ordering を保つには、capability family も separate pair judgment のまま扱う方が drift が少ない。

## current first choice details

- existing helper-local capability compare は entry evidence として再利用する。
- current family judgment は `e13-malformed-capability-strengthening` と `e20-malformed-late-capability-strengthening` の pair に置く。
- current next actualization mode は source-backed widening first に置く。
- implementation cut を narrower に取る場合でも `e13` lead は staging note に留め、family judgment 自体は `e13/e20` pair のまま保つ。
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残す。
- public operational CLI concrete shell naming comparison は repo-level next package として先に close してよいが、capability malformed-side next reopen point は source-backed widening actualization に置く。

## next promoted line

next promoted line は、
**stable-malformed-capability-second-reopen-actualization-ready public-operational-cli-concrete-shell-naming comparison**
に置く。

## open questions

- `e13/e20` source-backed widening actualization を pair 一括で行うか、`e13` lead staging を採るか
- capability second source-backed widening を close した後に duplicate cluster と malformed-static broader line をどの順で reopen するか
- capability family widening と public operational CLI actual shell をどちらから actualize するか
