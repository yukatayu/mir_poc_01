# plan/03 — 決定の強さと境界

## L0 / L1 / L2 / L3 の意味

| level | 意味 | 扱い方 |
|---|---|---|
| L0 | 基盤。変えると全体系へ影響する | 勝手に動かさない |
| L1 | 強い方向性。大きな設計方針 | 明示 decision なしに反転しない |
| L2 | active refinement 中の設計提案 | current repo の正本として扱うが、将来見直しうる |
| L3 | 探索段階 | 未決の比較対象として扱う |

## この `plan/` で区別する 5 種類

### 1. settled current L2

current L2 として、いまこの repo が採っている判断である。
parser grammar や production runtime までは固定していないが、current task では正本として扱う。

例:

- fallback は guarded option chain
- monotone degradation は left-to-right
- no re-promotion
- explicit edge-row family を current companion notation に維持

### 2. current parser-free PoC reading

理論本体ではなく、**current parser-free PoC 基盤で採っている最小実装 / 検証読み**である。
helper boundary、sidecar discovery、named profile catalog などがここに入る。

例:

- `.host-plan.json` sidecar discovery
- bundle / batch / selection / profile helper stack
- named profile catalog の hard-coded table

### 3. OPEN

まだ決まっていない。
文書上で convenient だからといって事実化してはいけない。

例:

- final parser grammar
- detached trace / audit serialization
- path canonicalization policy
- machine-readable catalog asset 採用

### 4. FUTURE WORKSTREAM

重いが重要な将来作業であり、現在の narrow PoC task にはまだ降ろさない。

例:

- 型システムの強さ
- 静的解析可能性
- 定理証明可能性
- 決定可能性
- external verifier / theorem prover との境界

### 5. HISTORICAL / COMPARISON

比較したが採らなかった案、または経緯把握用の材料である。
現在の repo 状態を説明するのに必要だが、いま採っている意味論そのものではない。

例:

- line-leading `>` ladder
- machine-readable named profile catalog externalization 案
- outer-longer-lifetime fallback intuition

## `plan/` におけるラベル運用

この `plan/` では次の読み分けを保つ。

- `決定`
  - `specs/` と `decision register` に anchor がある current repo の判断
- `current PoC reading`
  - code / fixture / tests まで含めて現在採っている companion / helper の読み
- `未決`
  - `OPEN QUESTION` として残すべき事項
- `比較対象`
  - 候補だが採用済みではないもの
- `履歴`
  - どういう順で現在形に至ったか

## 事実化してはいけないもの

次は `plan/` に書くときも「決定」と混ぜてはいけない。

- final parser grammar
- machine-readable catalog asset / manifest 採用
- richer host interface の concrete shape
- multi-request scheduler
- `Approximate` / `Compensate` の PoC 実装読み
- outer-longer-lifetime fallback の再導入

## この task 以後の使い方

将来 task では、まず「どの種類の更新か」を切り分ける。

| 更新種別 | どこへ反映するか |
|---|---|
| 意味論そのものの変更 | `specs/` と relevant `plan/` の両方 |
| parser-free PoC helper の変更 | code / tests と `plan/07` / `plan/09` |
| fixture 追加・更新 | fixture 実体と `plan/08` |
| roadmap / current status の変化 | `plan/01` / `plan/10` / `plan/11` / `plan/12` |
| 比較候補の整理 | relevant `specs/examples/`、report、必要に応じて `plan/06` / `plan/12` / `plan/13` |

この切り分けを崩すと、`plan/` が scratchpad になりやすい。
`plan/` は scratchpad ではなく、repo memory の整理層として維持する。
