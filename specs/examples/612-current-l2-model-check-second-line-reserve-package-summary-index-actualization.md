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

ここで保持するのは、
**`scripts/current_l2_guided_samples.py emit-reserve model-check-second-line` を使って
row-local property carrier second-line を repo-local output dir に narrow materialize していた historical helper-local summary index memory**
であり、current active compatibility front door は clean-near-end `list / smoke-all / closeout` に置く。

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
   - `p06` historical theorem-model-check bridge anchor
   - `p10 / p11 / p12 / p15 / p16` checker-adjacent bridge-floor widening
   を separate に保っている
3. closeout 後の reserve reopen line は `specs/examples/606` で historical helper / snapshot memory に同期済みである

したがって current open problem は、
model-check second-line の判断自体を増やすことではなく、
**この package の historical reserve-summary memory を current compatibility front door と混同せずに保持すること**
である。

## current actualization cut

current package では次を採る。

1. `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
   は 2026-04-22 clean-sample migration 前の historical reserve package entrypoint として扱い、
   current active compatibility front door には戻さない
2. current active compatibility front door は
   `python3 scripts/current_l2_guided_samples.py list`
   `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
   `python3 scripts/current_l2_guided_samples.py closeout --format json`
   に置く
3. historical output dir は
   `target/current-l2-guided/reserve-packages/model-check-second-line`
   に固定する
4. 同 dir には
   - `package-summary.md`
   - `package-summary.json`
   が materialize されていた
5. sample set は
   - `p06` historical theorem-model-check bridge anchor
   - `p10` authority release positive carrier
   - `p11 / p12 / p15 / p16` bad pattern rejection pair
   として narrow に保つ
6. historical summary index には
   - `typed_hint_status`
   - `theorem_preview_status`（historical bridge anchor status）
   - `model_check_preview_status`
   - `model_check_reopen_status`
   をそのまま残し、historical theorem bridge anchor と current clean-near-end theorem/model-check live floor を collapse しない

## current evidence

| evidence | current reading |
|---|---|
| `python3 scripts/current_l2_guided_samples.py list` | current active compatibility front door が clean-near-end accepted sample setのみを列挙することを示す |
| `python3 scripts/current_l2_guided_samples.py smoke-all --format json` | current active compatibility front door が active clean-near-end suite を実行することを示す |
| `python3 scripts/current_l2_guided_samples.py closeout --format json` | current active compatibility front door が canonical current inventory を返すことを示す |
| `specs/examples/606` | reserve integration entrypoint summary は historical helper memory として保持する |
| `target/current-l2-guided/reserve-packages/model-check-second-line/package-summary.md` | historical human-facing summary index path |
| `target/current-l2-guided/reserve-packages/model-check-second-line/package-summary.json` | historical machine-readable summary index path |

## current recommendation

1. `model-check-second-line` reserve package は、
   reserve list の中に置くだけでなく、
   historical helper-local summary index memory として保持してよい。ただし current active command へ戻さない。
2. current cut は
   `p06 / p10 / p11 / p12 / p15 / p16`
   の 6 本を保ったまま、
   row-local property carrier second-line を
   helper-local / non-production summary index に留めるのが自然である。ここでの `p06` は current theorem live floor ではなく historical bridge anchor として読む。
3. historical `matrix problem1` と historical `bundle problem1` は Problem 1 全体の residual / doc bundle memory、
   historical `emit-reserve model-check-second-line` は retired reopen memory、
   current compatibility front door は `list / smoke-all / closeout`
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
