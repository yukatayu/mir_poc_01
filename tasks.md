# tasks

最終更新: 2026-04-12 00:42 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half parser + checker/runtime first tranche fixed 後に、compile-ready checkpoint close を残り 2 package で詰める** ところまでを見通せる粒度で書く。
- ここでいう「Phase 完了」は、**現 phase の self-driven package を閉じ、残件を user spec required または heavy future workstream として明示的に切り分け終えた状態**を指す。`specs/` や `plan/13` に残す heavy future workstream まで今すぐ全部 actualize する意味ではない。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な時系列は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## 現在の読み

- **Phase 1** の current L2 semantics は、`specs/examples/291...292` により self-driven closeout fixed と読んでよい。semantic core 自体は変えず、invariants / proof-obligation wording / notation boundary の narrow closeout を source-backed に閉じ、final parser grammar / final type system / actual external schema は later に残している。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は source-backed に固定し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- **Phase 3** は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。minimal parser subset は stage 1 + stage 2 structural floor、first checker reconnect は stage 1 summary + stage 2 try/rollback structural contract までを bridge にし、stage 3 request / predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残している。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。current package は `specs/examples/121...125` を維持し、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、`delegated_provider_attestation` non-core line、control-plane separated carrier actualization、distributed fairness protocol、final operational realizationは later に残している。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。verifier handoff surface、theorem retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later line を 1 本の closeout bundle にまとめ、actual subject row、boundary-specific handoff artifact family、actual emitted verifier artifact、concrete tool binding、public checker migration、low-level memory-order family は later に残している。
- **Phase 6 front-half** は parser first tranche が `specs/examples/299...300`、checker/runtime first tranche が `specs/examples/301...302` で fixed 済みである。`mir-ast` には stage 1 / stage 2 non-production carrier、`mir-semantics` には program-level static gate / evaluator / host-runner entry、`mir-runtime` には semantic `Program` と optional parser bridge input を受ける thin `current_l2` skeleton が actualize 済みである。残る主線は compile-ready verification / formal hook と checkpoint sweep である。

## PoC compile までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 前半 compile-ready minimal actual PoC | parser + checker/runtime first tranche は fixed 済みで、formal hook と sweep が主残量 | **82% 前後** | 下の Task 1〜2 | **1〜2 package 前後 / 4〜8日** | current L2 subset に scope を絞った rough estimate |

### 進捗 82% 前後の根拠

- 進んでいるもの
  - semantic core、invariants、representative fixtures、parser-free harness、detached validation loop、Phase 3 staged parser spike evidence、minimal parser subset freeze、parser-to-checker reconnect freeze、`mir-ast` stage 1 / stage 2 non-production carrier、`mir-semantics` program-level entry、`mir-runtime` current L2 thin skeleton。
- まだ薄いもの
  - theorem / model-check formal hook first tranche
  - compile-ready checkpoint の selected cargo / smoke gate wording
  - final mirror / traceability sweep

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 前半 | compile-ready verification and formal hook | cargo gate、smoke、tool-neutral formal hook first tranche を揃える | 重い | 1 task / 3〜6日 | 自走可能 |
| 2 | Phase 6 前半 | compile-ready checkpoint drift suppression / mirror sweep | specs / plan / progress / tasks / abstract の wording と current line を揃える | 中 | 1 task / 1〜2日 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 前半 compile-ready verification and formal hook

- 目的
  - compile-ready PoC を checkpoint close 扱いできる verification package を揃える。
- 完了条件
  - selected `cargo test` / `cargo check` / smoke command が docs に固定されている。
  - representative parser / checker / runtime path が local gate で通る。
  - theorem-side relation と model-check side relation の **first tranche** が少なくとも tool-neutral export か narrow concrete tool first cut のどちらかで用意されている。
  - report / progress / tasks / plan が compile-ready milestone を mirror している。

### Task 2. Phase 6 前半 compile-ready checkpoint drift suppression / mirror sweep

- 目的
  - compile-ready checkpoint close 後の snapshot drift を suppress し、normative docs / abstract / snapshot / plan の current wording を揃える。
- 完了条件
  - `Documentation.md`、`progress.md`、`tasks.md`、`docs/research_abstract/`、relevant `plan/` に stale wording が残っていない。
  - `specs/00-document-map.md` と `plan/90-source-traceability.md` が current line を追えている。
  - validation / audit command が report に固定されている。

## 方針決定が必要な blocker / open question

- **現時点で、Task 1〜2 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、compile-ready checkpoint を閉じた後では次の decision point が効いてくる。

### Blocker 1. actual parser subset の second-tranche widen boundary

- 概要
  - Phase 6 parser + checker/runtime first tranche の後で、どこまで current L2 source surface を widen するか。
- 何に影響するか
  - `mir-ast` public-ish API
  - parser test corpus
  - checker / verifier handoff shape
- 主要な選択肢
  1. stage 1 / 2 carrier + current checker/runtime skeleton を compile-ready checkpoint minimum として維持し、stage 3 reconnect は second tranche に回す
  2. selected stage 3 / perform head を compile-ready checkpoint の中で同時に widen する
  3. request head / clause suite / richer diagnostics まで一気に広げる
- current recommendation / 見解
  - **1 を推奨**
  - current first tranche を fixed entry criteria にし、compile-ready checkpoint を閉じてから second tranche widen を比較する方が自然である。

### Blocker 2. theorem / model-check formal tool binding

- 概要
  - compile-ready checkpoint でどの formal tool に first tranche を置くか。
- 何に影響するか
  - proof artifact の保存場所
  - compile / CI gate
  - finite-state 化の前提
- 主要な選択肢
  1. tool-neutral relation export で compile-ready checkpoint を先に閉じる
  2. theorem side を先に concrete tool へ結ぶ
  3. model-check side を先に concrete tool へ結ぶ
  4. theorem / model-check の 2 本を同時に concrete tool へ結ぶ
- current recommendation / 見解
  - **Task 1 では 1 を第一候補にする** のを推奨。
  - current checker/runtime surface はまだ intentionally thin なので、formal hook でも surface が逆流しにくい tool-neutral first cut が自然である。
