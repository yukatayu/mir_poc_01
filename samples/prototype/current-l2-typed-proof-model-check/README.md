# current-l2 typed / proof / model-check サンプル

## 要約

- Problem 1 の current first line を追うための prototype 群。
- 入口は `p06-typed-proof-owner-handoff` とし、typed marker / theorem-first / row-local model-check carrier の current cut をまとめて確認する。

## まず見るサンプル

### `p06-typed-proof-owner-handoff`

- 役割:
  typed marker、proof notebook review-unit、model-check concrete carrier を 1 本の corrected prototype で見るための入口。
- ここで確認したいこと:
  - typed marker family は source principal ではなく helper-local preview に留まっている
  - theorem line は notebook-first / review-unit first の current cutに留まっている
  - model-check line は row-local carrier first の current cutに留まっている

実行例:

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  run-source-sample \
  samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt \
  --format pretty
```

matrix で representative と residual bridge-floor をまとめて見る例:

```bash
python3 scripts/current_l2_guided_samples.py matrix problem1
```

- `p06` は theorem/model-check public seam preview / reopen threshold が reached の representative sample として出る。
- `p10 / p11 / p12 / p15 / p16` は `typed_checker_hint_preview` が reached、theorem/model-check public seam は `bridge-only(...)` の residual bundle として出る。

bundle として docs / Lean artifact / residual matrix まで一本道で見る例:

```bash
python3 scripts/current_l2_guided_samples.py bundle problem1
```

- `p06` の prototype、`samples/lean/current-l2/p06-typed-proof-owner-handoff/`、anchor になる spec/report を 1 画面で辿れる。
- `samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt` も同じ bundle から辿れ、parser-side companion surface の first slice を確認できる。
- `p10 / p11 / p12 / p15 / p16` は補助サンプルとして同じ bundle 内で Lean artifact と一緒に確認できる。

## あわせて見る補助サンプル

### `p10-typed-authorized-fingerprint-declassification`

- authority 付き declassification が current IFC floor でどう見えるかを確認する。

### `p11-typed-unauthorized-fingerprint-release`

- authority 欠如で release が止まる negative を確認する。

### `p12-typed-classified-fingerprint-publication-block`

- label-flow mismatch による publication block を確認する。

### `p15-typed-capture-escape-rejected`

- capture / lifetime line の negative を確認する。

### `p16-typed-remote-call-budget-exceeded`

- simple cost bound line の negative を確認する。

## 読み方

- `verification_preview`
  - theorem / model-check bridge の helper-local preview を示す。
- `artifact_preview`
  - proof notebook review unit と model-check concrete carrier の repo-local emitted artifact preview を示す。
- `typed_checker_hint_preview`
  - checker-adjacent first layer の current cut を示す。
- `python3 scripts/current_l2_guided_samples.py matrix problem1`
  - representative public-seam sample と checker-adjacent bridge-floor sample を 1 表で見せる。

## 注意

- これらは **corrected prototype** であり、final typed calculus / final public verifier contract / final public property language を意味しない。
- current first line は、full dependent core ではなく finite decidable index fragment + IFC / taint + capture / lifetime + simple cost を checker-adjacent first layer として扱う読みである。
