# progress

最終更新: 2026-04-05（detached validation-loop actualization 時点）

## 位置づけ

- この文書は repo の**簡潔な進捗スナップショット**である。
- 規範判断の正本は `specs/`、長期参照用の repository memory は `plan/` である。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は、可能な限り次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / representative examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度

## いまどこまで来ているか

- Mir current L2 の核心意味論は、current task を回すにはかなり安定している。
- parser-free PoC 基盤は、fixture / interpreter / host harness / bundle / batch / selection / profile / catalog まで揃っている。
- detached validation loop は、docs-only judgment、bundle-first emitter、aggregate emitter、core-only diff helper、tiny loop wrapper、fixture authoring template まで入った。
- いま重いのは semantics そのものより、**fixture authoring / elaboration** と **detached validation loop の実運用面**である。
- richer host interface、final parser grammar、static analysis / type / theorem prover、multi-request scheduler はまだ後段である。
- 実装上の非本質だが忘れてはいけない制約として、**OS / hardware 非依存性** と **step 実行 / graph 可視化へ伸ばせる observability 境界** を、早期固定しすぎずに守る必要がある。

## バリデーションループ前の残課題

### 1. detached validation loop の actual narrow cut

- bundle export / compare / aggregate summary の最小手順は揃い始めた
- storage / path policy の current candidate を使った smoke を数回増やして、artifact naming / discovery を安定させる必要がある
- aggregate 側の typed count field は non-production aggregate artifact にまで進んだが、actual public API cut はまだ未決

### 2. fixture authoring / elaboration bottleneck

- 新しい fixture を 1 本足す手順は template 化された
- ただし authoring / sidecar / expected trace-audit / profile 影響確認は、まだ人手依存が大きい
- 「1 本足して detached artifact を保存し、aggregate summary も取り、既存 artifact と比べる」運用をあと数回回して固める必要がある

### 3. parser 前の boundary inventory

- current companion notation はあるが final parser grammar は未決
- parser 導入前に、何を syntax で固定し、何を current L2 companion notation に残すかの棚卸しがまだ必要

### 4. 後段に残すもの

- richer host interface の typed carrier 化
- static analysis / theorem prover 境界
- multi-request scheduler
- `Approximate` / `Compensate`
- portability / observability hook の final contract

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書・decision level・invariants | 92% | 86% | 70% | 着手可能 | repo の基礎境界はかなり揃っている |
| Mir current L2 core semantics | 82% | 72% | 68% | 着手可能 | current task を回すには十分安定、ただし final formalization はまだ先 |
| fallback / notation / representative examples | 84% | 79% | 62% | 着手可能 | drift 抑制は進んだが final parser grammar は未決 |
| parser-free PoC execution stack | 80% | 74% | 88% | 着手可能 | interpreter / host / bundle / batch / selection / profile まで揃っている |
| detached export / validation loop | 74% | 70% | 82% | 着手可能 | bundle / aggregate emitter と wrapper はあるが actual public API cut は未確定 |
| fixture authoring / elaboration 実務 | 64% | 68% | 62% | 着手可能 | template は改善されたが、追加作業の人手コストはまだ高い |
| parser / syntax finalization 準備 | 38% | 44% | 18% | 着手可能 | companion notation はあるが final grammar inventory がこれから |
| richer host interface / coverage typed 化 | 24% | 22% | 16% | 後段依存 | comparison までは進んだが implementation cut は後段 |
| aggregate export の typed actualization | 42% | 34% | 34% | 着手可能 | non-production aggregate emitter は入ったが actual API と compare 契約は未決 |
| static analysis / type / theorem prover workstream | 12% | 8% | 4% | 後段依存 | plan と entry criteria はあるが未着手に近い |
| portability / observability / debug hook 設計 | 20% | 14% | 10% | 後段依存 | HW 非依存と step / graph 可視化余地は要件化したが contract はまだない |
| Mirrorea fabric | 18% | 12% | 8% | 要仕様確認 | 境界整理はあるが current mainline 実装はまだ先 |
| Typed-Effect Wiring Platform | 12% | 8% | 6% | 要仕様確認 | 位置づけはあるが concrete architecture は後段 |
| PrismCascade / 上位空間 | 12% | 8% | 6% | 要仕様確認 | 分離方針はあるが mainline の進捗対象ではない |

## 着手可否の読み方

- `着手可能`
  - 非本質部分を先に進めても手戻りが比較的小さい。
  - current repo の mainline として、agent が narrow task を自走しやすい。
- `要仕様確認`
  - user 側の目的、保証範囲、非機能要件、上位層の具体像が不足しており、勝手に詰めると手戻りが大きい。
  - たとえば上位空間の UX や社会システム仕様はここに入る。
- `後段依存`
  - 先行 layer / 先行 decision が固まるまで、本格着手を急がない方がよい。
  - comparison、inventory、entry criteria 整理までは進めてよいが、mainline 実装にはまだ早い。

## いまから validation loop 入口まで何手か

- **detached validation loop の入口まで**: あと **1〜3** task 程度
- 主な中身:
  1. fixture authoring を detached artifact + aggregate summary loop 前提で 2〜3 回実地に回す
  2. aggregate export の compare 契約と actual narrow API cut をもう一段整理する
  3. storage / naming / compare discovery の current candidate を smoke evidence で固める

## ある程度自律的な Mir 構築ループまで何手か

- **ある程度自律的に「追加し、回し、比較し、次へ進む」状態まで**: あと **5〜8** task 程度
- 想定する中身:
  1. detached validation loop の入口を安定化する
  2. fixture authoring / elaboration の反復コストをもう一段下げる
  3. parser 前 inventory を作る
  4. richer host interface を後段に送ったまま、今必要な host coverage の cut だけ固める
  5. current L2 semantics の追加 regression を数本回して、巻き戻りが必要か確認する
  6. portability / observability の boundary を壊さない helper cut を確認する

## いまの判断

- **今の mainline は正しい**。Mir current L2 と parser-free PoC を主線に据える方針は維持でよい。
- 直近の主ボトルネックは、意味論の大変更ではなく **validation loop の運用面** と **fixture authoring** である。
- final parser grammar や richer host interface を急ぐより、まず「artifact を保存し、比較し、fixture を増やしてまた回す」ループを安定化させるべき段階にある。
- portability / observability は今すぐ runtime core に組み込む話ではないが、helper cut と将来 API の形を歪めないよう early reminder として管理する。
