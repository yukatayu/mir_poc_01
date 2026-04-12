# tasks

最終更新: 2026-04-12 20:10 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half compile-ready checkpoint close fixed 後に、fixed-subset source sample の actual widened row line と theorem-side bridge sketch reopen lineへどう接続するか** を見通せる粒度で書く。
- ここでいう「Phase 完了」は、**現 phase の self-driven package を閉じ、残件を user-spec-required / heavy future workstream / reserve reopen line として切り分け終えた状態**を指す。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な時系列は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## 現在の読み

- **Phase 1** は `specs/examples/291...292` により self-driven closeout fixed と読んでよい。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。
- **Phase 3** は `specs/examples/287...290` により self-driven freeze fixed と読んでよい。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。
- **Phase 6 front-half compile-ready checkpoint** は `specs/examples/299...314` により fixed 済みである。`mir-ast` stage 1 / stage 2 carrier、stage 3 declaration-side admit attached slot、shared isolated predicate fragment、shared single attachment frame extraction bridge、`mir-semantics` program-level entry、`mir-runtime` current L2 thin skeleton、tool-neutral formal hook first tranche、reserve formal tool binding inventory までは narrow actual evidence がある。
- source-sample sideでは、`specs/examples/315...316` により source corpus scope / layout、`317...318` により representative / fixture / source mapping matrix、`319...320` により actual parser-to-`Program` lowering first cut、`321...322` により syntax-backed sample runner first cut、`323...324` により verification ladder wiring、`325...326` により source-sample authoring / bless / regression policy、`327...328` により theorem-first concrete tool pilot が fixed 済みである。
- `specs/examples/329...330` により deferred authored-row widen sequencing も fixed 済みであり、current first choice は `e1 -> e21 -> e3` である。`e1` / `e21` は current runtime formal-hook family の内側で先に扱い、`e3` は admit-family / theorem-side guard を伴う third slot に残す。
- `specs/examples/331...332` により proof-notebook bridge-sketch reopen ordering も fixed 済みであり、current first choice は **plain docs-only bridge sketch first / compare-ready bridge sketch second** である。authored-row actualization line を先に閉じ、theorem-side reopen はその後に置く。
- `specs/examples/333...334` により、first widened authored row `e1` actualization も fixed 済みである。`e1-place-atomic-cut` は helper-compatible single-line `ensure` source row、runner accepted set、regression helper current authored inventory、README ladder に narrow actualize 済みであり、current runtime formal-hook top `runtime_try_cut_cluster` の内側で reached row と読んでよい。
- `specs/examples/335...336` により、second widened authored row `e21` actualization も fixed 済みである。`e21-try-atomic-cut-frontier` は helper-compatible source row、runner accepted set、regression helper current authored inventory、README ladder に narrow actualize 済みであり、`E21` / `E22` contrast は still later に残す。
- `specs/examples/337...338` により、third widened row `e3` theorem-side / formal-hook guard comparison も fixed 済みである。`e3-option-admit-chain` は current authored row に昇格させず、current theorem-side consumer を row-local `proof_notebook_review_unit`、current formal-hook top を `runtime_try_cut_cluster` に保ったまま、plain bridge sketch actualization の後段 reopen に残す。
- `specs/examples/339...340` により、plain proof-notebook bridge sketch actualization も fixed 済みである。old theorem-line `specs/examples/140` の docs-only bridge sketch shape (`bridge_subject_ref + review_units + bridge_goal_text`) を current Phase 6 theorem-side first actualization として再利用し、compare-ready metadata / helper-emitter / concrete tool binding は still later に残す。
- follow-up maintenance は report-backed に閉じた。したがって current immediate line は **Phase 6 theorem-side docs-first reopen compare-ready bridge sketch second reopen** である。

## 具体的な sample code 検証段階までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 front-half compile-ready minimal actual PoC | checkpoint close fixed | 96%+ | maintenance / reopen only | maintenance only | current L2 subset の compile-ready checkpoint は閉じたと読んでよい |
| fixed-subset syntax-backed sample verification milestone | scope/layout、mapping、lowering、runner、ladder、authoring policy、theorem-first pilot、widen sequencing、bridge-sketch ordering、`e1` / `e21` actualization、`e3` guard comparison、plain bridge sketch actualization、maintenance close fixed | **97%前後** | 1〜2 package | 数日〜2週 | 残りは compare-ready bridge sketch second reopen と、その後段の `e3` actualization reopen timing である |
| plain bridge-sketch actualization path | ordering fixed、guard comparison fixed、actual package fixed | 68%前後 | 1 package | 数日 | plain docs-only bridge sketch の current docs-first actualization cutは fixed 済みであり、next は compare-ready second reopen である |

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 theorem-side docs-first reopen | compare-ready bridge sketch second reopen | plain bridge sketch actualization の後段として compare-ready bridge sketch threshold を narrow reopen する | 中 | 1〜2 task / 数日 | 自走可能 |
| 2 | Phase 6 sample path / theorem guard reserve reopen | deferred `e3` actualization reopen timing | theorem-side line の後で `e3` widen をどこへ戻すかを narrow に比較する | 中 | 1〜2 task / 数日 | 自走可能 |
| 3 | Phase 6 sample path / theorem guard reserve reopen | actual `e3` authored-row package | reopen timing fixed 後に `e3-option-admit-chain` を actual authored row に上げるかを narrow に扱う | 中 | 1〜2 task / 数日 | 自走可能 |
| 4 | Phase 6 theorem-side reserve reopen | proof / model-check handoff first concrete tool cut | `e3` authored-row reopen の後段として first concrete theorem/model-check tool cut を narrow reopen する | 中 | 1〜3 task / 1〜2週 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 theorem-side docs-first reopen compare-ready bridge sketch second reopen

- 目的
  - plain bridge sketch actualization の後段として compare-ready bridge sketch threshold を reopen し、compare-bless metadata と concrete theorem/model-check binding の still-later guard を narrow に再確認する。
- 完了条件
  - plain bridge sketch current cut を壊さず、compare-bless metadata や concrete theorem/model-check binding を still later に残す。

### Task 2. Phase 6 sample path / theorem guard reserve reopen deferred `e3` actualization reopen timing

- 目的
  - theorem-side bridge line の後で `e3-option-admit-chain` widening をどこへ戻すかを narrow に比較する。
- 完了条件
  - `e3` actualization を premature に current authored inventory へ混ぜず、theorem-side / formal-hook guard comparison fixed の意味を保つ。

### Task 3. Phase 6 sample path / theorem guard reserve reopen actual `e3` authored-row package

- 目的
  - reopen timing fixed 後に `e3-option-admit-chain` を actual authored row に上げるかを narrow に扱う。
- 完了条件
  - theorem-side bridge line と formal-hook family guard を壊さず、`e3` actualization を minimal reopened package に留める。

### Task 4. Phase 6 theorem-side reserve reopen proof / model-check handoff first concrete tool cut

- 目的
  - `e3` authored-row reopen の後段として、proof notebook / model-check side の first concrete tool cut を narrow reopen する。
- 完了条件
  - `e3` authored-row reopen と compare-ready bridge sketch current cut を壊さず、theorem-first reserve / model-check second reserve の順を保つ。

## 方針決定が必要な blocker / open question

- **現時点で、current Task 1〜4 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、compare-ready bridge sketch の minimum、concrete tool timing、deferred `e3` actualization timing、higher-level control family の guard は効いている。

### Blocker 1. compare-ready bridge sketch second reopen の minimum

- 概要
  - plain bridge sketch actualization 自体は fixed 済みだが、compare-ready second reopen で `comparison_basis_refs` 以外をどこまで入れるかは still later である。
- 何に影響するか
  - theorem-side bridge line の太さ
  - concrete tool cut と deferred `e3` line の順序
- 主要な選択肢
  1. `comparison_basis_refs` だけを compare-ready minimum に足す
  2. bless / review-session metadata まで先に足す
  3. compare-ready second reopen を飛ばし concrete tool cut へ進む
- current recommendation / 見解
  - **1 を current first choice** に置くのが自然である。
  - plain bridge sketch actualization を先に閉じた以上、next は compare-ready second reopen を最小に保つ方が staged line に合う。

### Blocker 2. concrete formal tool binding と backend/codegen timing

- 概要
  - theorem/model-check concrete tool や LLVM-family backend / external codegen をいつつなぐか。
- 何に影響するか
  - parser / lowering / runtime boundary
  - sample corpus の syntax fix pressure
  - long-term implementation flexibility
- 主要な選択肢
  1. source corpus / lowering / runner より前に backend or tool binding を入れる
  2. fixed-subset source corpus、verification ladder / authoring policy、review-unit pilot、authored-row actualization lineを先に揃え、その後 bridge sketch / concrete tool binding を開く
  3. final grammar / public API closeout まで backend/tool binding を遅らせる
- current recommendation / 見解
  - **2 を current first choice** に置くのが自然である。
  - LLVM-family backend や external codegen を今すぐ主線に置くのは早い。syntax-backed fixed subset、lowering、runner、tool-neutral formal hook ladder、authoring policy、authored-row actualization lineを先に安定化した方が柔軟性を失いにくい。

### Blocker 3. `atomic_cut` sample 拡張と higher-level async-control / low-level memory-order family の関係

- 概要
  - `atomic_cut` を含む runnable sample 拡張と、より重い制御構造 family を同時に進めるべきか。
- 何に影響するか
  - sample corpus の意味論安定性
  - proof / runtime boundary
  - later async-control workstream
- 主要な選択肢
  1. settled subset の `atomic_cut` sample を増やす
  2. higher-level async-control family を sample core surface に入れながら進める
  3. low-level memory-order-like vocabulary を先に current language surface に入れる
- current recommendation / 見解
  - **1 を current first choice** に置き、2 と 3 は heavy future workstream に残すのが自然である。
  - settled subset の runnable sample を増やすのはよいが、memory-order-like surface や higher-level async-control family を今の executable core に混ぜるのは premature である。
