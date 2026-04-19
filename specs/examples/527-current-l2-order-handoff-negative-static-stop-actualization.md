# 527 — current L2 order-handoff negative static-stop actualization

## 目的

Package 58 の order-handoff negative corpus tightening として、

- `missing witness static stop`
- `handoff before publish static stop`

を compare-floor の TODO に残さず、current source sample runner / CLI で inspectable な helper-local static stop として actualize する。

ここで actualize するのは、
**authoritative-room late-join visibility line の `publish -> handoff -> observe` 前提を、`p13 / p14` に限って helper-local current-L2 source sample runner static gate へ落とす current cut**
である。

したがって、ここでは次を fixed しない。

- final parser grammar
- final source-surface handoff wording
- final emitted-handoff contract
- final public witness/provider/artifact contract
- final shared-space exhaustive catalog

## current question

`specs/examples/520` が要求した

- missing witness static stop
- handoff before publish static stop

を、new grammar や final public checker contract を導入せずに current execution floor へ下ろせるか。

## current first line

current recommendation は次である。

1. `p13-dice-late-join-missing-publication-witness` を underdeclared static stop にする
2. `p14-dice-late-join-handoff-before-publication` を malformed static stop にする
3. 判定は helper-local current-L2 source sample runner 側に留める
4. 判定対象は authoritative-room late-join visibility line の negative pair に限定する
5. CLI `run-source-sample --format json|pretty` から static verdict / reasons / preview guard を inspectable にする

## actualized negative pair

| sample | verdict | helper-local reason | current reading |
|---|---|---|---|
| `p13-dice-late-join-missing-publication-witness` | `Underdeclared` | `missing publication witness before handoff for late-join visibility` | publication witness を欠いた late-join line は current default profile の static stop negative pair にする |
| `p14-dice-late-join-handoff-before-publication` | `Malformed` | `handoff appears before publish for late-join visibility` | publish が存在しても順序が handoff より後ろなら malformed static stop にする |

## なぜこの cut でよいか

- `specs/examples/520` は、この 2 つを current final-layer closeout adequacy corpus に戻すよう要求している。
- current source principal は relation decomposition / authority-serial / witness-aware line にあり、low-level `memory_order` surface や final source wording をまだ fixed しない。
- したがって、いま必要なのは final grammar ではなく、late-join visibility line の negative pair を helper-local static stop として sample-visible にすることだと読める。
- `p07` の reached line を支える negative pair を `p13 / p14` として切ることで、current actual adoption line を壊さずに negative corpus tightening を進められる。

## evidence

- runner side helper-local static gate
  - `crates/mir-runtime/src/current_l2.rs`
- regression
  - `crates/mir-runtime/tests/current_l2_order_handoff_negative_static_stop.rs`
- corrected prototype pair
  - `samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt`
  - `samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt`
- prior positive/default floor
  - `specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md`
  - `specs/examples/471-current-l2-authoritative-room-vertical-slice-emitted-artifact-ratchet.md`
  - `specs/examples/526-current-l2-order-handoff-helper-cli-surface-preview-actualization.md`

## retained later

- final parser grammar
- final source wording
- final emitted-artifact / emitted-handoff schema
- final public witness/provider/artifact contract
- stronger fairness / replay operational profile

## stop line

この package の stop line は、

- `p13 / p14` が static stop で runtime 未到達になる
- CLI JSON から verdict / reasons / preview guard が inspectable になる
- docs / roadmap / snapshot が order-handoff negative pair actualization を current package として追える

の 3 点である。
