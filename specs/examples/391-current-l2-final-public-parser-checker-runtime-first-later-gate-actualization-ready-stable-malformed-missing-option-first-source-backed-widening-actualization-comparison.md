# 391 — current L2 final-public-parser-checker-runtime-first-later-gate-actualization-ready stable-malformed-missing-option-first-source-backed-widening-actualization comparison

## 目的

`specs/examples/390-current-l2-final-public-parser-checker-runtime-first-later-gate-actualization-ready-minimal-final-public-parser-checker-runtime-first-later-gate-threshold.md`
で runtime-led thin facade の first later cut を fixed した次段として、

- `e16/e17/e18` missing-option family を source-backed sample / runner / ladder / regression line にどの最小 cut で widen するか
- helper-local missing-option compare を entry evidence に保ったまま、どこまで current authored sample に actualize するか
- public operational CLI second later gate と thin-facade later support を巻き戻さずに、Macro 4 side の widening をどこまで先に閉じるか

を比較する。

ここで固定するのは
**current L2 final-public-parser-checker-runtime-first-later-gate-actualization-ready stable-malformed-missing-option-first-source-backed-widening-actualization comparison**
であり、

- `e17` の source-authored actualization
- capability second reopen
- duplicate cluster / `TryFallback` / `AtomicCut` malformed-static family promotion
- public operational CLI actual shape

はまだ固定しない。

## scope

- current package は actual source-backed widening comparison に進む。
- widening 対象は `samples/current-l2/`、`mir-runtime` source lowerer / runner / ladder tests、repo-local regression helper に留める。
- theorem/model-check bridge や public/report shape は current fixed cut を巻き戻さない。
- `e17` は same-family staged guard として残し、family judgment 自体は triplet のまま扱う。

## current 前提

current repo では次が成立している。

1. `specs/examples/387...388` により、missing-option first reopen family は `e16/e17/e18` triplet に固定され、helper-local compare を entry evidence に再利用する current cut が固定済みである。
2. `specs/examples/389...390` により、repo-level current public pressure line は runtime-led thin facade first later cut に固定済みであり、`run_current_l2_source_sample` の public/report shape は保護されている。
3. current source lowerer は chain edge の predecessor を source row の progression から導くため、missing predecessor row を source-authored にするには special-case parser bridge か off-spec override を増やしやすい。
4. `e16` と `e18` は chain head / successor 欠落として current source surface で素直に表現でき、`fixture_static_cluster` formal-hook line に乗せやすい。

したがって current 問いは、
**`e16/e18` first cut + `e17` staged guard が、family judgment と implementation cost の両方で最小か**
である。

## 比較観点

1. helper-local compare を entry evidence に保ったまま source-backed widening を narrow に actualize できるか
2. current source lowerer / runner / ladder / regression helper の contract を unnecessary に歪めないか
3. `e17` を silent drop せず same-family staged guard として残せるか
4. 次線を public operational CLI second later gate と thin-facade later support へ clean に handoff できるか

## 比較対象

### 案 1. `e16/e18` を source-authored static-stop pair に actualize し、`e17` を staged guard に残す

#### shape

```text
stable_malformed_missing_option_source_backed_widening = {
  actualization_kind = current_l2_missing_option_source_backed_static_stop_pair,
  entry_evidence = helper_local_missing_option_compare,
  actualized_rows = [
    e16_malformed_missing_chain_head_option,
    e18_malformed_missing_successor_option
  ],
  staged_same_family_guard = [
    e17_malformed_missing_predecessor_option
  ],
  actualized_surfaces = [
    source_sample_files,
    current_l2_runner_accepted_sample_set,
    source_lowering_tests,
    source_sample_runner_tests,
    verification_ladder_tests,
    repo_local_inventory_and_regression_helper
  ]
}
```

#### 利点

- `e16/e18` は current source surface で素直に書けるため、special-case parser bridge を増やさずに widen できる。
- helper-local compare を entry evidence に保ちつつ、sample-facing authored corpus と ladder を実際に厚くできる。
- `e17` を same-family staged guard に残すことで、triplet family judgment を shrink しない。

#### 欠点

- triplet 全量の source-backed actualizationにはまだ届かない。

### 案 2. `e16` だけを source-authored に actualize する

#### 利点

- 実装は最も軽い。

#### 欠点

- first reopen family を single-row に見せやすい。
- `e18` の symmetric missing-successor case が sample-facing surface に出ない。

### 案 3. `e16/e17/e18` triplet を一括 source-authored に actualize する

#### 利点

- family judgment をそのまま authored corpus に載せられる。

#### 欠点

- `e17` のために current lowerer / parser bridge へ special-case を入れやすい。
- missing-option widening ではなく parser contract widening に話題が逸れやすい。

## current judgment

current L2 で最も自然なのは、
**案 1. `e16/e18` を source-authored static-stop pair に actualize し、`e17` を staged guard に残す**
である。

理由は次の通り。

1. current task の主眼は missing-option family judgment を維持したまま authored corpus を厚くすることであり、parser bridge special-case を足すことではない。
2. `e16` と `e18` は chain head / successor 欠落として current source lowerer で自然に表現でき、runner / ladder / regression helper への widening が straight である。
3. `e17` を staged same-family guard と明示すれば triplet family judgment を維持でき、capability second や broader malformed later line との整合も崩れない。

## current first choice details

- current authored source sample は `e16-malformed-missing-chain-head-option` と `e18-malformed-missing-successor-option` を追加し、repo-local authored corpus を decet へ widen する。
- `run_current_l2_source_sample` の accepted sample set、source lowerer tests、source sample runner tests、verification ladder tests、`scripts/current_l2_source_sample_regression.py` inventory / regression bundle を `e16/e18` まで widen する。
- `e16` / `e18` は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` に揃える。
- `e17` は current source lowerer contract では predecessor を edge row の progression から導くため、same-family staged guard に留め、silent drop しない。
- public operational CLI second later gate、thin-facade later support、capability second reopen、duplicate cluster / `TryFallback` / `AtomicCut` malformed-static family は kept-later に残す。

## next promoted line

next promoted line は、
**stable-malformed-missing-option-first-source-backed-widening-actualization-ready public-operational-cli-second-later-gate-actualization comparison**
に置く。

## open questions

- `e17` を source-authored に上げるための最小 bridge cut をどこで reopen するか
- capability second reopen を missing-option widening の直後に置くか
- public operational CLI second later gate と thin-facade later support をどの順で close するか
