# Problem 1 sample bundle

## この bundle の目的

- Problem 1 の current first line である
  checker-adjacent first layer / notebook-first theorem line / row-local model-check carrier first
  を、representative sample `p06` を中心に確認する。
- full dependent core や final public verifier contract までをここで確定するものではない。

## まず見るサンプル

### `p06-typed-proof-owner-handoff`

- current representative sample:
  `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
- parser companion:
  `samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt`
- Lean artifact:
  `samples/lean/current-l2/p06-typed-proof-owner-handoff/`

この sample では次を同時に見る。

- typed marker family は source principal ではなく checker-adjacent preview に留める
- theorem line は notebook-first / review-unit first に留める
- model-check line は row-local carrier first に留める

## 最短 quickstart

### 1. `smoke problem1` で representative line を一度に確認する

```bash
python3 scripts/current_l2_guided_samples.py smoke problem1
```

見るべき結果:

- `p06` の runtime / `matrix problem1` / `bundle problem1` / parser companion inspector / `mapping`
  がこの順に通る。
- representative sample bundle の主要 command 群が drift していないことを 1 本で確認できる。

### 2. `matrix problem1` で representative と補助 sample の役割差を見る

```bash
python3 scripts/current_l2_guided_samples.py matrix problem1
```

見るべき結果:

- `p06` が public-seam representative として先頭に出る。
- `p10 / p11 / p12 / p15 / p16` は checker-adjacent bridge-floor 補助 sample として残る。

### 3. `bundle problem1` で docs / Lean artifact / anchor spec-report まで一本道で辿る

```bash
python3 scripts/current_l2_guided_samples.py bundle problem1
```

見るべき結果:

- representative sample path、Lean artifact path、parser companion path、
  anchor spec / report が 1 画面で読める。
- final public theorem contract や final public verifier contract には上げていない stop line も同時に確認できる。

### 4. parser companion inspector で request/head/clause bundle を直接見る

```bash
cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- \
  samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt \
  --format pretty
```

見るべき結果:

- `p06` companion surface が repo-local parser-side carrier に戻っていることが分かる。
- surface を final grammar に昇格せず、thin experimental companion として保っている current cut を追える。

## 実行の順番

1. representative sample をそのまま流す

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt \
  --format pretty
```

2. representative と residual bridge-floor をまとめて見る

```bash
python3 scripts/current_l2_guided_samples.py matrix problem1
```

3. docs / Lean artifact / anchor spec-report まで一本道で辿る

```bash
python3 scripts/current_l2_guided_samples.py bundle problem1
```

4. parser-side companion / mapping まで同じ読みに揃える

```bash
python3 scripts/current_l2_guided_samples.py mapping
```

## 補助サンプル

- `p10-typed-authorized-fingerprint-declassification`
  - authority 付き declassification success
- `p11-typed-unauthorized-fingerprint-release`
  - authority 欠如 negative
- `p12-typed-classified-fingerprint-publication-block`
  - label-flow mismatch negative
- `p15-typed-capture-escape-rejected`
  - capture / lifetime negative
- `p16-typed-remote-call-budget-exceeded`
  - simple cost negative

これらは representative `p06` を補う corrected prototype であり、
full dependent type、general theorem proving in compiler、final public checker artifact までは意味しない。

## 読み方の要点

- `verification_preview`
  - theorem / model-check bridge の helper-local preview
- `artifact_preview`
  - proof notebook review unit と row-local model-check carrier の emitted artifact preview
- `typed_checker_hint_preview`
  - checker-adjacent finite-index first layer の current cut

## 現在の mixed gate 再開点

- `stronger typed-surface actual adoption`
  - `python3 scripts/current_l2_guided_samples.py matrix problem1`
    で `p06` と `p10 / p11 / p12 / p15 / p16` の役割差を見て、
    typed source principal を premature に上げない current cut を再確認する。
- `final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract`
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
    と `samples/lean/current-l2/p06-typed-proof-owner-handoff/`
    を起点に、theorem-first pilot artifact と notebook-first transport floor を辿る。
- `first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract`
  - `python3 scripts/current_l2_guided_samples.py matrix problem1`
    と `python3 scripts/current_l2_guided_samples.py bundle problem1`
    を合わせて見て、row-local carrier first の current cut を越えて final public checker 契約へ飛ばないことを確認する。
- global true user-spec residual は
  `python3 scripts/current_l2_guided_samples.py reopen-map`
  から、packaging / host integration / upper-layer application target をまとめて見直す。

## split package status

- `typed source principal split`
  - close 済み。`python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal`
    から入り、`p06` representative と `p10 / p11 / p12 / p15 / p16` 補助 sample の役割差を保ったまま
    typed source principal の reopen point だけを独立 package として読める。
- `theorem public-contract split`
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
    と `samples/lean/current-l2/p06-typed-proof-owner-handoff/`
    から入り、review-unit first / notebook-consumer first のまま theorem public-contract residual を切り出す。
- `model-check public-contract split`
  - `python3 scripts/current_l2_guided_samples.py matrix problem1`
    と `python3 scripts/current_l2_guided_samples.py bundle problem1`
    を合わせて見て、row-local property route first / checker-artifact route first を保ったまま
    model-check public-contract residual を切り出す。

## typed source principal split の入口

```bash
python3 scripts/current_l2_guided_samples.py split problem1 typed-source-principal
```

見るべき結果:

- `p06` representative と `p10 / p11 / p12 / p15 / p16` 補助 sample が、
  typed source principal residual の supporting set としてまとまって見える。
- `theorem public-contract split` と `model-check public-contract split` が
  kept separate として表示され、typed residual だけを narrow に読む current cut を確認できる。
- stop line が `final typed source principal` / `final typed calculus` /
  `final public verifier contract` に留まり、theorem/model-check の public contract と混ざらないことを確認できる。

## stop line

- stronger typed source principal promotion
- final public theorem result object
- consumer-shaped theorem payload public contract
- final public checker artifact
- final public verifier contract
