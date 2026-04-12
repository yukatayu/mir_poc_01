# plan/01 — 現況サマリ

## project 全体の主眼

- 主眼は **Mir** の意味論基盤である。
- 現在の主要作業は、Phase 1〜5 の self-driven closeout / freeze と Phase 6 front-half compile-ready checkpoint close を entry criteria にしつつ、parser-side follow-up package と **syntax-backed fixed-subset sample verification path** を narrow に整理することである。
- PrismCascade は重要な比較対象・将来統合対象だが、現時点では **optional / side-track に近い独立 kernel** として扱う。

## いまの主フェーズ

現在は次のフェーズにある。

1. current L2 semantics / parser-free PoC / proof boundary の fixed entry criteria を維持する
2. `mir-ast` / `mir-semantics` / `mir-runtime` の front-half actual code path を narrow に保つ
3. compile-ready verification / formal hook first tranche を fixed entry criteria として扱う
4. parser-side first package fixed 後の reserve formal tool binding inventory、parser-side follow-up sequencing、shared single attachment frame actualization、source corpus scope / layout、representative / fixture / source mapping matrix、lowering、runner、verification ladder、source-sample policy、theorem-first concrete tool pilot、authored-row widen sequencing、bridge-sketch reopen ordering、maintenance close を fixed 済み entry criteria とし、current line を first widened row `e1` actualization に置く
5. fixed-subset source corpus / mapping / lowering / runner / verification ladder / authoring policy と proof-notebook review-unit pilot を維持しつつ、widened authored-row order `e1 -> e21 -> e3`、theorem-side plain bridge sketch first / compare-ready bridge sketch second の順を final grammar や backend 固定に逆流させず段階 actualize する

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
- Phase 6 front-half では stage 1 / stage 2 parser carrier、checker/runtime thin skeleton、theorem-line整合の tool-neutral formal hook first trancheまで fixed 済みである
- first authored source sample trio `e4` / `e2` / `e23` の verification ladder も fixed 済みであり、`e2` は `static gate -> interpreter -> runtime_try_cut_cluster formal hook`、`e4` / `e23` は `static gate -> fixture_static_cluster formal hook` まで current reached と読む
- theorem-first concrete consumer も first pilot までは fixed 済みであり、tool-neutral formal hook artifact を入力にする non-production `proof_notebook_review_unit` を current cut に置く

### まだ未決のもの

- final parser grammar
- actual parser/runtime sample path の widen boundary
- concrete theorem / model-check tool binding
- LLVM-family backend / external codegen binding の timing
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate` の PoC 実装読み
- machine-readable catalog asset / manifest の採否

## parser-free PoC と compile-ready gate の状態

current repo は少なくとも次まで到達している。

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
13. detached bundle / aggregate / static gate emitter
14. detached loop compare / smoke helper
15. `mir-ast` stage 1 / stage 2 non-production carrier
16. `mir-semantics` program-level entry
17. `mir-runtime` current L2 thin skeleton
18. tool-neutral formal hook emitter と formal-hook smoke
19. helper-local source sample runner first cut

runtime fixture は `e1` / `e2` / `e21` / `e22` / `e3` / `e6` / `e7` / `e8` / `e9` / `e10` / `e11`、static-only fixture は `e4` / `e23` / `e5` / `e12` / `e13` / `e14` / `e15` / `e16` / `e17` / `e18` / `e19` / `e20` で揃っている。

## 直近の next steps

- first widened authored row `e1` actualization へ進む
- second widened authored row `e21` actualization へ進む
- third widened row `e3` theorem-side / formal-hook guard comparison を比較する
- final parser grammar をまだ固定せず、notation / examples / fixture / helper を壊さない範囲で syntax workstream を進める

## 主要な blocker / risk

| 項目 | いまの状態 |
|---|---|
| fallback 直感 drift | outer/inner wrapper 読みが残りやすい。explicit edge-row と prose で抑制中 |
| final parser grammar 未固定 | companion notation はあるが grammar はまだ固定しない |
| concrete formal tool binding 未選定 | tool-neutral formal hook は fixed 済みだが、concrete theorem/model-check tool は still later |
| source-sample path は sequencing まで fixed 済みだが widened authored row は未整備 | representative prose / fixture corpus / source target path の 3 層対応、helper-local lowering、file-path-backed runner、first-trio reached-stage inventory、repo-local policy helper、`e1 -> e21 -> e3` widen order は揃ったが、actual widened row は still later |
| theorem-first bridge sketch / concrete tool handoff は ordering まで fixed 済みだが actual line は未整備 | compile-ready checkpoint、source-to-`Program` lowerer、helper-local runner、first-trio ladder、source sample policy、proof notebook review unit pilot、bridge sketch reopen ordering はあるが、plain bridge sketch actual package / concrete theorem-model-check tool handoff はまだ薄い |
| backend timing | LLVM-family backend や external codegen を今つなぐと syntax / lowering が早期固定しやすい |
| helper / snapshot drift | docs / tests / code の mirror 境界を狭めてきたが、checkpoint close 後も sample-path 再分解に合わせた継続整流が必要 |
| heavy future workstream 未着手 | 型・静的解析・定理証明・決定可能性はまだ本格着手前 |

## rough step estimate

これは **rough estimate** であり、決定ではない。
current L2 / PoC の narrow task を積みながら見直す前提で読む。

| 目標 | rough estimate |
|---|---|
| first widened row `e1` actualization + second widened row `e21` actualization | 2〜4 task |
| plain bridge sketch actualization | 1〜2 task |
| first widened authored row `e1` actualization | 1〜2 task |
| richer runtime / host interface / static analysis の入口を切る | 6〜12 task |
| 型システム・定理証明可能性まで含む重い workstream に実装的に着手する | 15 task 以上。現時点では荒い |

## 読むべき次の文書

- current L2 semantics の整理なら `plan/04-core-semantics-current-l2.md`
- fallback / `lease` の核心なら `plan/05-fallback-lease-and-chain-semantics.md`
- PoC stack を追うなら `plan/07-parser-free-poc-stack.md`
- 次 task を考えるなら `plan/11-roadmap-near-term.md`
