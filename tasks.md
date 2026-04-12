# tasks

最終更新: 2026-04-12 16:49 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half compile-ready checkpoint close fixed 後に、fixed-subset source sample をどの順で widen し、theorem-side bridge sketch reopen をどこで挟むか** を見通せる粒度で書く。
- ここでいう「Phase 完了」は、**現 phase の self-driven package を閉じ、残件を user-spec-required / heavy future workstream / reserve reopen line として切り分け終えた状態**を指す。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な時系列は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## 現在の読み

- **Phase 1** は `specs/examples/291...292` により self-driven closeout fixed と読んでよい。semantic core は変えず、invariants / proof-obligation wording / notation boundary の narrow closeout を source-backed に閉じ、final parser grammar / final type system / actual external schema は later に残している。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は source-backed に固定し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- **Phase 3** は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。minimal parser subset は stage 1 + stage 2 structural floor、first checker reconnect は stage 1 summary + stage 2 try/rollback structural contract までを bridge にし、stage 3 request / predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残している。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。current package は `specs/examples/121...125` を維持し、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、`delegated_provider_attestation` non-core line、control-plane separated carrier actualization、distributed fairness protocol、final operational realizationは later に残している。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。verifier handoff surface、theorem retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later line を 1 本の closeout bundle にまとめ、actual subject row、boundary-specific handoff artifact family、actual emitted verifier artifact、concrete tool binding、public checker migration、low-level memory-order family は later に残している。
- **Phase 6 front-half compile-ready checkpoint** は `specs/examples/299...314` により fixed 済みである。`mir-ast` stage 1 / stage 2 carrier、stage 3 declaration-side admit attached slot、shared isolated predicate fragment、shared single attachment frame extraction bridge、`mir-semantics` program-level entry、`mir-runtime` current L2 thin skeleton、tool-neutral formal hook first tranche、reserve formal tool binding inventory までは narrow actual evidence がある。
- source-sample sideでは、`specs/examples/315...316` により source corpus scope / layout、`317...318` により representative / fixture / source mapping matrix、`319...320` により actual parser-to-`Program` lowering first cut、`321...322` により syntax-backed sample runner first cut、`323...324` により verification ladder wiring、`325...326` により source-sample authoring / bless / regression policy、`327...328` により theorem-first concrete tool pilot が fixed 済みである。
- **deferred authored-row widen sequencing** は `specs/examples/329...330` により fixed 済みであり、current first choice は `e1 -> e21 -> e3` である。`e1` / `e21` は current runtime formal-hook family の内側で先に扱い、`e3` は admit-family / theorem-side guard を伴う third slot に残す。
- したがって current immediate line は **Phase 6 proof-notebook bridge-sketch reopen ordering** である。ここでは review-unit current cut を保ったまま、authored-row widen line の後で docs-only bridge sketch をどこで reopen するかを narrow に固定する。

## 具体的な sample code 検証段階までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 front-half compile-ready minimal actual PoC | checkpoint close fixed | 96%+ | maintenance / reopen only | maintenance only | current L2 subset の compile-ready checkpoint は閉じたと読んでよい |
| fixed-subset syntax-backed sample verification milestone | scope/layout、mapping、lowering、runner、ladder、authoring policy、theorem-first pilot、widen sequencing fixed | **91%前後** | 3〜5 package | 数日〜2週 | source text sample を parse / lower / static / interpreter / formal hook と repo-local policy、proof-notebook review-unit pilotまで接続済み。残りは bridge-sketch ordering、follow-up maintenance、`e1 -> e21 -> e3` actualization line |
| proof-notebook bridge-sketch reopen path | review-unit pilot fixed、reopen order未固定 | 15%前後 | 1〜3 package | 1〜3週 | review-unit current cut の後で docs-only bridge sketch をどこで reopen するかが残る |

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 theorem-side reserve reopen | proof-notebook bridge-sketch reopen ordering | review-unit current cut の後で theorem-side bridge sketch をいつ reopen するかを比較する | 中 | 1〜2 task / 数日 | 自走可能 |
| 2 | Phase 0 / 6 maintenance | mirror sweep follow-up maintenance | current promoted line と snapshot 文書のずれを継続的に抑える | 低〜中 | 継続 | 自走可能 |
| 3 | Phase 6 sample path | first widened authored row actualization (`e1`) | `e1` を source row / runner accepted set / regression helper へ narrow actualize する | 中 | 1〜2 task / 数日 | 自走可能 |
| 4 | Phase 6 sample path | second widened authored row actualization (`e21`) | `e21` を `e1` の後段 widen として narrow actualize する | 中 | 1〜2 task / 数日 | 自走可能 |
| 5 | Phase 6 sample path / theorem guard | third widened row guard comparison (`e3`) | `e3` widening を current formal-hook / theorem-side guard とどう接続するかを比較する | 中 | 1〜2 task / 数日 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 theorem-side reserve reopen proof-notebook bridge-sketch reopen ordering

- 目的
  - `proof_notebook_review_unit` current cut を壊さずに、bridge sketch をどの package で reopen するかを narrow に整理する。
- 完了条件
  - review-unit current shape と source-sample path current cut を保持し、compare-bless metadata や concrete theorem/model-check tool binding を premature に混ぜない。
  - theorem-side later reopen line が docs / plan / abstract で一貫している。

### Task 2. Phase 0 / 6 maintenance mirror sweep follow-up maintenance

- 目的
  - current promoted line と snapshot 文書のずれを継続的に抑える。
- 完了条件
  - current line / next line / retained-later line が `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、abstract、report で一致している。
  - historical log を壊さずに snapshot だけを更新できている。

### Task 3. Phase 6 sample path first widened authored row actualization (`e1`)

- 目的
  - current widen sequence の first slot `e1-place-atomic-cut` を actual source row / runner accepted set / regression helper / README ladder に反映する。
- 完了条件
  - current tool-neutral formal hook top と theorem-first review-unit pilot guard を壊さない。
  - source text / fixture mapping / runner accepted set / regression helper / README ladder / snapshot docs が同じ task で同期している。

### Task 4. Phase 6 sample path second widened authored row actualization (`e21`)

- 目的
  - `e21-try-atomic-cut-frontier` を current widen sequence の second slot として actualize する。
- 完了条件
  - `e1` widen 後の runner / regression / README matrix と整合し、`E21` / `E22` contrast を premature に public line へ混ぜない。

### Task 5. Phase 6 sample path / theorem guard third widened row guard comparison (`e3`)

- 目的
  - `e3-option-admit-chain` widening を、current formal-hook / theorem-side family の外でどう扱うかを narrow に比較する。
- 完了条件
  - `e3` を runtime sample として widen する場合の theorem-side / formal-hook guard が明示され、`admit` family widening を silent に既成事実化しない。

## 方針決定が必要な blocker / open question

- **現時点で、current Task 1〜3 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、current mainline では bridge-sketch timing、`e3` theorem-side guard、formal/back-end timing の guard が効く。

### Blocker 1. proof-notebook bridge-sketch reopen ordering

- 概要
  - review-unit current cut の後で theorem-side bridge sketch をどの順で reopen するか。
- 何に影響するか
  - theorem-side handoff shape
  - source-sample policy との handoff
- 主要な選択肢
  1. authored-row actualization より前に bridge sketch を reopen する
  2. authored-row actualization line の後で plain docs-only bridge sketch を reopen する
  3. compare-bless metadata や concrete theorem/model-check tool binding と同時に reopen する
- current recommendation / 見解
  - **2 を current first choice** に置くのが自然である。
  - plain docs-only bridge sketch を先に reopen し、compare metadata はその後段、concrete tool binding はさらに後段に残す。

### Blocker 2. `e3` widening と theorem-side / formal-hook family guard

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

### Blocker 3. concrete formal tool binding と backend/codegen timing

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

### Blocker 4. `atomic_cut` sample 拡張と higher-level async-control / low-level memory-order family の関係

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
