# 612. current-l2 model-check-second-line reserve package summary index actualization

## 目的

`specs/examples/478`、
`568`、
`606`
と
`p06-typed-proof-owner-handoff`、
`p10-typed-authorized-fingerprint-declassification`、
`p11-typed-unauthorized-fingerprint-release`、
`p12-typed-classified-fingerprint-publication-block`、
`p15-typed-capture-escape-rejected`、
`p16-typed-remote-call-budget-exceeded`
を前提に、

- `model-check-second-line` reserve package の単独 entrypoint
- repo-local emitted summary index
- representative bridge / positive carrier / bad pattern rejection pair
- current recommendation
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**`scripts/current_l2_guided_samples.py emit-reserve model-check-second-line` を使って、
row-local property carrier second-line を repo-local output dir に narrow materialize する helper-local summary index**
であり、

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- production checker/runtime-policy contract
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. model-check second-line concretization 自体は `specs/examples/478` で close 済みである
2. theorem/model-check bridge reconnect は `specs/examples/568` で
   - `p06` representative bridge
   - `p10 / p11 / p12 / p15 / p16` checker-adjacent bridge-floor widening
   を separate に保っている
3. closeout 後の reserve reopen line は `specs/examples/606` で helper / snapshot に同期済みである

したがって current open problem は、
model-check second-line の判断自体を増やすことではなく、
**この package を単独 command と repo-local summary index で再実行可能にすること**
である。

## current actualization cut

current package では次を採る。

1. `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
   を reserve package の単独 entrypoint に置く
2. output dir は
   `target/current-l2-guided/reserve-packages/model-check-second-line`
   に固定する
3. 同 dir に
   - `package-summary.md`
   - `package-summary.json`
   を置く
4. sample set は
   - `p06` representative theorem-model-check bridge
   - `p10` authority release positive carrier
   - `p11 / p12 / p15 / p16` bad pattern rejection pair
   として narrow に保つ
5. summary index には
   - `typed_hint_status`
   - `theorem_preview_status`
   - `model_check_preview_status`
   - `model_check_reopen_status`
   をそのまま残し、theorem-first pilot と model-check second-line を collapse しない

## actual runnable evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line` | reserve package 単独の repo-local summary index |
| `target/current-l2-guided/reserve-packages/model-check-second-line/package-summary.md` | human-facing summary index |
| `target/current-l2-guided/reserve-packages/model-check-second-line/package-summary.json` | machine-readable summary index |
| `p06-typed-proof-owner-handoff.run.json` | representative theorem-model-check bridge |
| `p10-typed-authorized-fingerprint-declassification.run.json` | authority release positive carrier |
| `p11-typed-unauthorized-fingerprint-release.run.json` | authority-miss rejection |
| `p12-typed-classified-fingerprint-publication-block.run.json` | label-flow rejection |
| `p15-typed-capture-escape-rejected.run.json` | capture/lifetime rejection |
| `p16-typed-remote-call-budget-exceeded.run.json` | simple cost rejection |

## current recommendation

1. `model-check-second-line` reserve package は、
   reserve list の中に置くだけでなく、
   単独 command と summary index まで actualize してよい。
2. current cut は
   `p06 / p10 / p11 / p12 / p15 / p16`
   の 6 本を保ったまま、
   row-local property carrier second-line を
   helper-local / non-production summary index に留めるのが自然である。
3. `matrix problem1` と `bundle problem1` は Problem 1 全体の residual / doc bundle、
   `emit-reserve model-check-second-line` は model-check second-line package 単独の reopen entrypoint
   として読み分ける。

## retained alternatives

- `matrix problem1` だけで reserve package detail を兼ねる
- public checker artifact preview を先に materialize する
- theorem-first pilot と model-check second-line を 1 本の helper に collapse する

## stop line

current package は次で止める。

- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- production checker/runtime-policy contract
- final public verifier contract

## next self-driven line

current package を close した後の next reopen line は、

1. later mixed gate lane
2. true user-spec hold line

に移るのが自然である。
