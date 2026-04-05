# progress

最終更新: 2026-04-05

## 位置づけ

- この文書は repo の**簡潔な進捗スナップショット**である。
- 規範判断の正本は `specs/`、長期参照用の repository memory は `plan/` である。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。

## いまどこまで来ているか

- Mir current L2 の核心意味論は、current task を回すにはかなり安定している。
- parser-free PoC 基盤は、fixture / interpreter / host harness / bundle / batch / selection / profile / catalog まで揃っている。
- detached validation loop は、docs-only judgment、bundle-first emitter、core-only diff helper、tiny loop wrapper、fixture authoring template まで入った。
- いま重いのは semantics そのものより、**fixture authoring / elaboration** と **detached validation loop の実運用面**である。
- richer host interface、final parser grammar、static analysis / type / theorem prover、multi-request scheduler はまだ後段である。

## バリデーションループ前の残課題

### 1. detached validation loop の actual narrow cut

- bundle export / compare / aggregate summary の実運用手順をもう少し揃える
- storage / path policy の current candidate を使った smoke を増やす
- aggregate 側の typed count field は docs-only judgment までで、actual API cut はまだ未決

### 2. fixture authoring / elaboration bottleneck

- 新しい fixture を 1 本足す手順は template 化された
- ただし authoring / sidecar / expected trace-audit / profile 影響確認は、まだ人手依存が大きい
- 「1 本足して detached artifact を保存し、既存 artifact と比べる」運用をあと数回回して固める必要がある

### 3. parser 前の boundary inventory

- current companion notation はあるが final parser grammar は未決
- parser 導入前に、何を syntax で固定し、何を current L2 companion notation に残すかの棚卸しがまだ必要

### 4. 後段に残すもの

- richer host interface の typed carrier 化
- static analysis / theorem prover 境界
- multi-request scheduler
- `Approximate` / `Compensate`

## 章別 rough progress

| 章 / 層 | 概算進捗 | 補足 |
|---|---:|---|
| 基礎文書・decision level・invariants | 90% | repo の基礎境界はかなり揃っている |
| Mir current L2 core semantics | 82% | current task を回すには十分安定、ただし final formalization はまだ先 |
| fallback / notation / representative examples | 78% | drift 抑制は進んだが final parser grammar は未決 |
| parser-free PoC execution stack | 88% | interpreter / host / bundle / batch / selection / profile まで揃っている |
| detached export / validation loop | 72% | bundle-first emitter / diff / wrapper はあるが actual narrow API は未確定 |
| fixture authoring / elaboration 実務 | 58% | template はできたが、追加作業の人手コストはまだ高い |
| parser / syntax finalization 準備 | 30% | companion notation はあるが final grammar inventory がこれから |
| richer host interface / coverage typed 化 | 20% | comparison までは進んだが implementation cut は後段 |
| aggregate export の typed actualization | 25% | docs-only naming / migration cut はあるが actual API は未決 |
| static analysis / type / theorem prover workstream | 8% | plan と entry criteria はあるが未着手に近い |
| Mirrorea fabric | 15% | 境界整理はあるが current mainline 実装はまだ先 |
| Typed-Effect Wiring Platform | 10% | 位置づけはあるが concrete architecture は後段 |
| PrismCascade / 上位空間 | 10% | 分離方針はあるが mainline の進捗対象ではない |

## いまから validation loop 入口まで何手か

- **detached validation loop の入口まで**: あと **2〜4** task 程度
- 主な中身:
  1. fixture authoring を detached artifact loop 前提で 2〜3 回実地に回す
  2. bundle export / compare / aggregate summary の actual narrow cut をもう一段整理する
  3. storage / naming / compare discovery の current candidate を smoke evidence で固める

## ある程度自律的な Mir 構築ループまで何手か

- **ある程度自律的に「追加し、回し、比較し、次へ進む」状態まで**: あと **6〜10** task 程度
- 想定する中身:
  1. detached validation loop の入口を安定化する
  2. fixture authoring / elaboration の反復コストをもう一段下げる
  3. parser 前 inventory を作る
  4. richer host interface を後段に送ったまま、今必要な host coverage の cut だけ固める
  5. current L2 semantics の追加 regression を数本回して、巻き戻りが必要か確認する

## いまの判断

- **今の mainline は正しい**。Mir current L2 と parser-free PoC を主線に据える方針は維持でよい。
- 直近の主ボトルネックは、意味論の大変更ではなく **validation loop の運用面** と **fixture authoring** である。
- final parser grammar や richer host interface を急ぐより、まず「artifact を保存し、比較し、fixture を増やしてまた回す」ループを安定化させるべき段階にある。
