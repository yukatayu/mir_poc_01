# plan/01 — 現況サマリ

## project 全体の主眼

- 主眼は **Mir** の意味論基盤である。
- 現在の主要作業は、Mir current L2 の意味整理と、それを parser-free PoC で machine-check できる最小基盤の整備である。
- PrismCascade は重要な比較対象・将来統合対象だが、現時点では **optional / side-track に近い独立 kernel** として扱う。

## いまの主フェーズ

現在は次のフェーズにある。

1. current L2 の companion semantics を固める
2. parser-free PoC で representative examples を machine-check する
3. docs / tests / helper layer の責務境界を狭く整える
4. final parser grammar や richer runtime にはまだ進み切らない

## current L2 の状態

### すでに current L2 として揃っているもの

- fallback は **guarded option chain**
- canonical chain は **left-to-right monotone degradation**
- **earlier option への再昇格禁止**
- write-after-expiry は later write-capable option を試し、成立しなければ request-level `Reject`
- rollback / `atomic_cut` は degradation order を巻き戻さない
- option-local `admit` miss / `lease` expiry は non-admissible metadata に留める
- capability mismatch は formal subreason にせず narrative explanation に留める
- compact syntax 比較の結果、**explicit edge-row family** を current L2 companion notation として維持する

### まだ未決のもの

- final parser grammar
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate` の PoC 実装読み
- machine-readable catalog asset / manifest の採否

## parser-free PoC 基盤の状態

current L2 parser-free PoC stack は、少なくとも次まで到達している。

1. AST fixture schema
2. evaluation state schema
3. step semantics
4. oracle API
5. parser-free minimal interpreter
6. host harness
7. host plan sidecar loader
8. bundle loader
9. batch runner
10. selection helper
11. selection profile helper
12. named profile catalog

runtime fixture は `e1` / `e2` / `e3` / `e6` / `e7` / `e8` / `e9` / `e10` / `e11`、static-only fixture は `e4` / `e5` / `e12` / `e13` / `e14` / `e15` / `e16` / `e17` / `e18` で揃っている。

## 直近の next steps

- `plan/` を今後の repo memory として維持し、PoC task ごとに relevant file を更新する
- current L2 と parser-free PoC のあいだで未決の点を narrow-scope task に分解し続ける
- final parser grammar をまだ固定せず、notation / examples / fixture / helper を壊さない範囲で syntax workstream を進める
- 将来の heavy workstream である型・静的解析・定理証明・決定可能性を、今の PoC stack と切り分けた上で entry criteria を整える

## 主要な blocker / risk

| 項目 | いまの状態 |
|---|---|
| fallback 直感 drift | outer/inner wrapper 読みが残りやすい。explicit edge-row と prose で抑制中 |
| final parser grammar 未固定 | companion notation はあるが grammar はまだ固定しない |
| helper stack の drift | docs / tests / code の mirror 境界を狭めてきたが、今後も継続的な整流が必要 |
| review infrastructure の返答遅延 | reviewer completion が遅いことがあり、report で local evidence を補う運用が必要 |
| heavy future workstream 未着手 | 型・静的解析・定理証明・決定可能性はまだ本格着手前 |

## rough step estimate

これは **rough estimate** であり、決定ではない。
current L2 / PoC の narrow task を積みながら見直す前提で読む。

| 目標 | rough estimate |
|---|---|
| current PoC 検証ループを壊さずに継続できる状態を保つ | 2〜4 task |
| syntax / fixture / helper の境界を詰めて parser 準備段階へ進む | 5〜8 task |
| richer runtime / host interface / static analysis の入口を切る | 8〜15 task |
| 型システム・定理証明可能性まで含む重い workstream に実装的に着手する | 15 task 以上。現時点では荒い |

## 読むべき次の文書

- current L2 semantics の整理なら `plan/04-core-semantics-current-l2.md`
- fallback / `lease` の核心なら `plan/05-fallback-lease-and-chain-semantics.md`
- PoC stack を追うなら `plan/07-parser-free-poc-stack.md`
- 次 task を考えるなら `plan/11-roadmap-near-term.md`
