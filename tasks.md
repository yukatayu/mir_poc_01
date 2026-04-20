# tasks

最終更新: 2026-04-21 00:48 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、長期参照は `plan/`、詳細経緯は `docs/reports/` に置く。
- `tasks.md` は履歴ではなく snapshot である。

## current status at task level

- current-L2 authored sixteen と corrected prototype set `p01 ... p16` は runnable
- Problem 1 representative bundle は executable
  - typed success / rejection pair
  - theorem-first emitted artifact loop
  - model-check second-line reserve summary
  - Lean foundation / generated stub acceptance
- Problem 2 representative bundle は executable
  - representative pair `p07 / p08`
  - reserve route `p09`
  - negative static-stop pair `p13 / p14`
  - witness / delegated RNG reserve summary
- numbered self-driven queue は closed
- 現在の次段は **mixed gate reopen** と **true user-spec residual** をどう切るかである

## current executable floor

### Problem 1

- primary:
  `p06-typed-proof-owner-handoff`
- supporting typed set:
  `p10 / p11 / p12 / p15 / p16`
- helper entrypoints:
  - `python3 scripts/current_l2_guided_samples.py bundle problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
- Lean:
  - `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  - `samples/lean/current-l2/`

### Problem 2

- representative pair:
  `p07 / p08`
- reserve / negative:
  `p09 / p13 / p14`
- helper entrypoints:
  - `python3 scripts/current_l2_guided_samples.py bundle problem2`
  - `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness`
  - `python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service`

## 自走可能な task package

### 1. Problem 1 final-public-seam reopen

- phase:
  Macro 5 mixed-gate reopen
- 目的:
  typed residual、theorem public-contract residual、model-check public-contract residual の reopen order を narrow に保ったまま整理する
- 入口:
  `python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams`
- 何に効くか:
  Problem 1 を `repo-local near-end` から `final public seam decision` 手前まで綺麗に分解して読める
- rough estimate:
  中
- current recommendation:
  sample corpus を増やさず、existing bundle / matrix / reserve helper を使う

### 2. Problem 2 final-public-seam reopen

- phase:
  Macro 6 mixed-gate reopen
- 目的:
  source wording / emitted schema と witness-provider public-shape を分けたまま整理する
- 入口:
  `python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams`
- 何に効くか:
  authoritative-room first line を壊さずに final public contract 残件だけを見直せる
- rough estimate:
  中
- current recommendation:
  `p07 / p08 / p09 / p13 / p14` の役割差を崩さない

### 3. parser-side residual lane

- phase:
  Macro 3 / Macro 7 mixed-gate reopen
- 目的:
  parser companion surface、request/head/clause bundle carrier、public parser API 残件を narrow に読む
- 入口:
  `python3 scripts/current_l2_guided_samples.py lane parser-side-residual`
- 何に効くか:
  current parser-side carrier を final grammar claim と混ぜない
- rough estimate:
  中
- current recommendation:
  stage3 carrier と current guided bundle の対応を保つ

### 4. reserve package hardening

- phase:
  Macro 5 / Macro 6 reserve integration
- 目的:
  theorem-first external pilot、auditable-authority-witness、delegated-rng-service、model-check second-line を reserve package として磨く
- 入口:
  `python3 scripts/current_l2_guided_samples.py reserve`
- 何に効くか:
  first line と later mixed gate の間の帯域を読みやすく保てる
- rough estimate:
  小〜中
- current recommendation:
  final public contract claim に踏み込まず summary index を改善する

### 5. summary / traceability maintenance

- phase:
  Macro 0 maintenance
- 目的:
  `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`plan/01` を stale wording なしで保つ
- 入口:
  `python3 scripts/validate_docs.py`
- 何に効くか:
  agent が current state を誤読しにくくなる
- rough estimate:
  小
- current recommendation:
  representative command と stop line を必ず対で書く

## research を通して見つけること

### low-level `memory_order` reinterpretation

- 概要:
  低レベル `memory_order` family を current public line に採るかどうか
- 影響:
  Problem 2 wording、artifact contract、shared-space reasoning
- 主要な選択肢:
  - order / handoff high-level line を維持する
  - low-level exact surface を別 layer として reopen する
- current recommendation:
  今は retained-later のままにする

### concrete theorem / model-check tool binding

- 概要:
  theorem/model-check を concrete tool contract に進めるか
- 影響:
  Problem 1 public seam、artifact schema、verifier contract
- 主要な選択肢:
  - Lean-first bridge / reserve summary を維持する
  - concrete tool brand と public checker artifact を導入する
- current recommendation:
  まだ bridge / reserve floor に留める

### final public verifier contract

- 概要:
  helper preview / reserve summary を final shared verifier contract に統合するか
- 影響:
  Problem 1、Problem 2、parser-side、formal hook 全体
- 主要な選択肢:
  - tool-neutral formal-hook floor を維持
  - final public verifier contract を定義
- current recommendation:
  まだ later

## user が決める必要があること

### broader application target

- 概要:
  repo の次の大きな適用先をどこに置くか
- 影響:
  Macro 8、host integration、packaging、acceptance criteria
- 主要な選択肢:
  - authoritative-room first scenario の延長
  - Typed-Effect / host integration 側の具体化
  - Prism / 上位アプリ側の具体化
- current recommendation:
  目的と保証範囲を user 側で先に固定したい

### packaging / installed binary / FFI

- 概要:
  repo-local helper floor から配布可能 surface へ進むか
- 影響:
  Macro 7、CI、artifact retention、host-facing contract
- 主要な選択肢:
  - repo-local helper floor を維持
  - installed binary / packaging を first-class target にする
  - FFI / engine adapter を先に切る
- current recommendation:
  まだ repo-local floor を維持

## rough next order

1. summary / traceability maintenance
   Macro 0 maintenance。短く終わるが drift 防止効果が高い
2. Problem 1 final-public-seam reopen
   Macro 5 mixed-gate front half
3. Problem 2 final-public-seam reopen
   Macro 6 mixed-gate front half
4. parser-side residual lane
   Macro 3 / 7 reserve path
5. user-spec hold line beyond repo-local near-end
   Macro 7 / 8。ここから先は user 決定が濃くなる
