# 401 — current L2 public-operational-cli-concrete-shell-naming-ready stable-malformed-capability-second-source-backed-widening-actualization comparison

## 目的

`specs/examples/400-current-l2-public-operational-cli-concrete-shell-naming-ready-minimal-public-operational-cli-concrete-shell-naming-threshold.md`
で current-L2 scoped shell naming の minimum を fixed した次段として、

- capability family (`e13/e20`) の source-backed widening actualization をどの cut から narrow に始めるか
- helper-local capability compare、stage1 reconnect widen、fixture-static capability rows をどこまで entry evidence に再利用するか
- missing-option first source-backed widening close を巻き戻さずに、capability family の actualized surface をどこまで pair cut に留めるか

を比較する。

ここで固定するのは
**current L2 public-operational-cli-concrete-shell-naming-ready stable-malformed-capability-second-source-backed-widening-actualization comparison**
であり、

- public operational CLI actual shell
- duplicate cluster actualization
- `TryFallback` / `AtomicCut` malformed-static broader reopen
- theorem/model-check/public-checker widening

はまだ固定しない。

## scope

- current package は docs-only actualization comparison に留める。
- `specs/examples/397...398` の `e13/e20` pair judgment と source-backed widening first readingを entry criteria に使う。
- `specs/examples/391...392` の missing-option first source-backed widening actualization を closest precedent に使う。
- existing public/report shape `run_current_l2_source_sample` / `CurrentL2SourceSampleRunReport` と current formal-hook top は巻き戻さない。

## current 前提

current repo では次が成立している。

1. `specs/examples/397...398` により、capability family current judgment は `e13/e20` pair に維持し、next malformed-side actualization mode は source-backed widening first に固定済みである。
2. `specs/examples/391...392` により、same Macro 4 precedent として missing-option family は `e16/e18` source-authored static-stop pair を current first cut に actualize 済みである。
3. `scripts/current_l2_capability_checker.py`、`crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`、fixture-static rows により、`e13/e20` は helper-local / parser-side / fixture-static の 3 方向で entry evidence を already 持つ。
4. current authored source sample corpus と runner accepted set には、`e13/e20` はまだ入っていない。
5. duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は broader follow-up inventory で still later に残している。

したがって current 問いは、
**`e13/e20` pair judgment を保ったまま、source-backed widening actualization の current first cut を source-authored static-stop pair に置くのが最小か**
である。

## 比較観点

1. existing `e13/e20` pair judgment を shrink せずに済むか
2. helper-local capability compare と stage1 reconnect widen を actual source widening の entry evidence に再利用できるか
3. missing-option first source-backed widening precedentと対称な malformed-side cut を採れるか
4. duplicate cluster と malformed-static broader family を later に残したまま next package を public operational CLI actual shell 側へ handoff できるか

## 比較対象

### 案 1. `e13/e20` source-authored static-stop pair を current first cut に置く

#### shape

```text
stable_malformed_capability_second_source_backed_widening = {
  actualization_kind = current_l2_capability_source_authored_static_stop_pair,
  chosen_family = [e13, e20],
  entry_evidence = [
    helper_local_capability_compare,
    stage1_parser_reconnect_widening,
    fixture_static_gate_capability_rows,
    stable_malformed_capability_second_reopen_actualization
  ],
  actualized_surface = [
    source_samples,
    source_lowering_tests,
    source_sample_runner_tests,
    verification_ladder_static_formal_hook_route,
    regression_inventory_and_regression_bundle
  ],
  staging_note = [
    e13_lead_allowed_only_as_implementation_order
  ]
}
```

#### 利点

- `e13/e20` pair judgment を維持したまま、next malformed-side actualization cut を明示できる。
- missing-option first source-backed widening と対称な source-authored static-stop pair cut を採れる。
- helper-local capability compare、stage1 reconnect、fixture-static rows を current entry evidence として再利用できる。

#### 欠点

- actual source row / runner / regression widening は次段実装に残る。

### 案 2. `e13` lead だけを先に source-authored row にし、`e20` は staged guard に残す

#### 利点

- implementation cut は最も薄い。

#### 欠点

- current family judgment の `e13/e20` pair を actualization threshold 側で弱めやすい。
- missing-option line の pair actualization precedent との対称性が崩れやすい。

### 案 3. helper-local capability compare を current stop line とみなし、source-backed widening 自体を later に送る

#### 利点

- code widening を先送りできる。

#### 欠点

- malformed-side source-backed corpus expansion が capability familyだけ未接続のまま残る。
- `e13/e20` source text evidence を持つ current repo stateを十分活かせない。

## current judgment

current L2 で最も自然なのは、
**案 1. `e13/e20` source-authored static-stop pair を current first cut に置く**
である。

理由は次の通り。

1. current package の主眼は actualization comparison を narrow に閉じることであり、reopen comparison で fixed 済みの pair judgment を actual row family へ素直に送る方が自然である。
2. `e13/e20` は helper-local capability compare、stage1 source-text evidence、fixture-static capability rowsの 3 方向で already 揃っており、single-row に縮める理由が薄い。
3. missing-option family は pair actualization first cut を already 持つため、capability family も source-authored static-stop pair を current first cut に置く方が repo-level symmetry を保ちやすい。

## current first choice details

- actualization family は `e13-malformed-capability-strengthening` と `e20-malformed-late-capability-strengthening` の source-authored static-stop pair に置く。
- entry evidence は helper-local capability compare、stage1 reconnect widen、fixture-static capability rows、`specs/examples/397...398` の pair judgment を再利用する。
- current actualized surface は source sample file、source lowerer / runner / ladder、repo-local regression helper inventory / regression bundleまでに留める。
- current formal-hook top は `fixture_static_cluster` のまま保ち、theorem/model-check/public-checker wideningは混ぜない。
- implementation order を narrower に取る場合でも `e13` lead は staging note に留め、family judgment 自体は `e13/e20` pair のまま保つ。
- duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残す。

## next promoted line

next promoted line は、
**stable-malformed-capability-second-source-backed-widening-actualization-ready public-operational-cli-concrete-shell-actualization comparison**
に置く。

## open questions

- `e13/e20` source-authored static-stop pair を 1 commit で同時に閉じるか、implementation order だけを `e13` lead にするか
- capability source-backed widening close 後に duplicate cluster と malformed-static broader line をどう reopen するか
- capability source-backed widening actualization と public operational CLI concrete shell actualization のどちらを先に actual code にするか
