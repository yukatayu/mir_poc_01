# tasks

最終更新: 2026-04-12 17:10 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half compile-ready checkpoint close fixed 後に、fixed-subset source sample の actual widened row line と theorem-side plain bridge sketch reopen lineへどう接続するか** を見通せる粒度で書く。
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
- follow-up maintenance は report-backed に閉じた。したがって current immediate line は **Phase 6 sample path first widened authored row actualization (`e1`)** である。

## 具体的な sample code 検証段階までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 front-half compile-ready minimal actual PoC | checkpoint close fixed | 96%+ | maintenance / reopen only | maintenance only | current L2 subset の compile-ready checkpoint は閉じたと読んでよい |
| fixed-subset syntax-backed sample verification milestone | scope/layout、mapping、lowering、runner、ladder、authoring policy、theorem-first pilot、widen sequencing、bridge-sketch ordering、maintenance close fixed | **93%前後** | 3〜4 package | 数日〜2週 | 残りは `e1 -> e21 -> e3` actualization line と plain bridge sketch actualization |
| plain bridge-sketch actualization path | ordering fixed、actual package未着手 | 18%前後 | 1〜3 package | 1〜3週 | plain docs-only bridge sketch をどこで actualize するかは、authored-row line の後に残る |

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 sample path | first widened authored row actualization (`e1`) | `e1` を source row / runner accepted set / regression helper へ narrow actualize する | 中 | 1〜2 task / 数日 | 自走可能 |
| 2 | Phase 6 sample path | second widened authored row actualization (`e21`) | `e21` を `e1` の後段 widen として narrow actualize する | 中 | 1〜2 task / 数日 | 自走可能 |
| 3 | Phase 6 sample path / theorem guard | third widened row guard comparison (`e3`) | `e3` widening を current formal-hook / theorem-side guard とどう接続するかを比較する | 中 | 1〜2 task / 数日 | 自走可能 |
| 4 | Phase 6 theorem-side docs-first reopen | plain bridge sketch actualization | current order fixed 済みの plain docs-only bridge sketch を narrow actualize する | 中 | 1〜2 task / 数日 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 sample path first widened authored row actualization (`e1`)

- 目的
  - current widen sequence の first slot `e1-place-atomic-cut` を actual source row / runner accepted set / regression helper / README ladder に反映する。
- 完了条件
  - current tool-neutral formal hook top と theorem-first review-unit pilot guard を壊さない。
  - source text / fixture mapping / runner accepted set / regression helper / README ladder / snapshot docs が同じ task で同期している。

### Task 2. Phase 6 sample path second widened authored row actualization (`e21`)

- 目的
  - `e21-try-atomic-cut-frontier` を current widen sequence の second slot として actualize する。
- 完了条件
  - `e1` widen 後の runner / regression / README matrix と整合し、`E21` / `E22` contrast を premature に public line へ混ぜない。

### Task 3. Phase 6 sample path / theorem guard third widened row guard comparison (`e3`)

- 目的
  - `e3-option-admit-chain` widening を、current formal-hook / theorem-side family の外でどう扱うかを narrow に比較する。
- 完了条件
  - `e3` を runtime sample として widen する場合の theorem-side / formal-hook guard が明示され、`admit` family widening を silent に既成事実化しない。

### Task 4. Phase 6 theorem-side docs-first reopen plain bridge sketch actualization

- 目的
  - current order fixed 済みの plain docs-only bridge sketch (`specs/examples/140` line) を narrow actual package に落とす。
- 完了条件
  - review-unit current cut と compare-ready bridge sketch second reopen を保持し、compare-bless metadata や concrete theorem/model-check binding を premature に混ぜない。

## 方針決定が必要な blocker / open question

- **現時点で、current Task 1〜3 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、`e3` theorem-side / formal-hook guard、concrete tool timing、higher-level control family の guard は効いている。

### Blocker 1. `e3` widening と theorem-side / formal-hook family guard

- 概要
  - `e3-option-admit-chain` は current runtime formal-hook top `runtime_try_cut_cluster` にそのままは乗らない。
- 何に影響するか
  - widened authored-row ladder shape
  - theorem-first review-unit pilot の current family
- 主要な選択肢
  1. `e3` widening 前に theorem-side / formal-hook family compare を 1 package 挟む
  2. `e3` widening を formal hook not reached row として扱う
  3. current theorem-side family を widen してから `e3` を authored row に入れる
- current recommendation / 見解
  - **1 を current first choice** に置くのが自然である。
  - `e1` / `e21` actualization line を先に閉じ、`e3` は admit-family / theorem-side guard comparison の後に actualize する方が staged line に合う。

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
