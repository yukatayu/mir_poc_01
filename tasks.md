# tasks

最終更新: 2026-04-11 22:24 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 3 reconnect freeze fixed 後に、Phase 1〜5 の self-driven / current-recommendation scope を閉じ、Phase 6 前半の compile-ready minimal PoC へ入る** ところまでを見通せる粒度で書く。
- ここでいう「Phase 完了」は、**現 phase の self-driven package を閉じ、残件を user spec required または heavy future workstream として明示的に切り分け終えた状態**を指す。`specs/` や `plan/13` に残す heavy future workstream まで今すぐ全部 actualize する意味ではない。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な時系列は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## 現在の読み

- **Phase 1** の current L2 semantics は、`specs/examples/291...292` により self-driven closeout fixed と読んでよい。semantic core 自体は変えず、invariants / proof-obligation wording / notation boundary の narrow closeout を source-backed に閉じ、final parser grammar / final type system / actual external schema は later に残している。
- **Phase 2** の parser-free PoC と detached validation loop は、`mir-semantics` 側で compile / test 可能な主線をすでに持っている。残りは bless / retention / compile gate のような closeout package である。
- **Phase 3** は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。minimal parser subset は stage 1 + stage 2 structural floor、first checker reconnect は stage 1 summary + stage 2 try/rollback structural contract までを bridge にし、stage 3 request / predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残している。
- **Phase 4** は `specs/examples/121...125` までで current package close だが、Phase 完了と呼ぶには「current recommendation で self-driven に閉じる範囲」と「user spec required な final catalog」を tasks / progress / plan でさらに明確化する closeout package が要る。
- **Phase 5** は `specs/examples/126...286` までで current package close である。verifier handoff gate は `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` docs-only mixed-row bridge まで fixed したが、proof / protocol / model-check handoff closeout は still later に残る。
- **実装面の現在地** は uneven である。`mir-semantics` には parser-free current L2 minimal interpreter と harness があり、`cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --list` で 46 tests が列挙できる。一方で `mir-ast` / `mir-runtime` / `mir-lsp` の public `src/lib.rs` はまだ placeholder skeleton であり、actual parser evidence は `mir-ast/tests/support/current_l2_stage*` の test-only helper に留まる。

## PoC compile までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 前半 compile-ready minimal actual PoC | docs / test-only spike / parser-free harness / reconnect freeze と Phase 1 closeout までは揃っているが、public crates は未 actualize | **35% 前後** | 下の Task 1〜6 | **6 package 前後 / 3〜6週** | current L2 subset に scope を絞った rough estimate |

### 進捗 35% 前後の根拠

- 進んでいるもの
  - semantic core、invariants、representative fixtures、parser-free harness、detached validation loop、Phase 3 staged parser spike の private evidence、minimal parser subset freeze、parser-to-checker reconnect freeze。
- まだ薄いもの
  - `mir-ast` public parser carrier
  - `mir-runtime` actual runtime bridge
  - parser -> checker -> runtime の public compile path
  - proof / model-check handoff の tool-bound first tranche

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 2 closeout | parser-free PoC / detached loop closeout | compile gate、retention / bless policy、authoring flow の current rule を固定 | 中 | 1〜2 task / 2〜4日 | 自走可能 |
| 2 | Phase 4 closeout | shared-space self-driven closeout | current recommendation package を close し、user-spec-required final catalog を明示分離 | 重め | 2〜4 task / 4〜8日 | 一部自走可能 |
| 3 | Phase 5 closeout | proof / protocol / runtime-policy handoff closeout | theorem / model-check / runtime-policy 側へ送る relation slice と stop line を固定 | 重い | 2〜4 task / 4〜8日 | 自走可能 |
| 4 | Phase 6 前半 | actual parser / AST carrier first tranche | `mir-ast` に non-production minimal parser carrier を actualize し compile させる | 重い | 2〜4 task / 4〜7日 | 後段依存 |
| 5 | Phase 6 前半 | actual checker / runtime skeleton first tranche | `mir-semantics` / `mir-runtime` をまたぐ minimal compile path を actualize する | 重い | 2〜4 task / 4〜7日 | 後段依存 |
| 6 | Phase 6 前半 | compile-ready PoC verification and formal hook | cargo gate、smoke、proof/model-check first tranche を揃えて checkpoint close | 重い | 1〜3 task / 3〜6日 | 後段依存 |

## 自走で進める task package

### Task 1. Phase 2 parser-free PoC / detached loop closeout

- 目的
  - parser-free PoC を「mainline companion baseline」として close する。
- 完了条件
  - compile / test / smoke entrypoint が current snapshot として整理されている。
  - bless / retention / reference-update の current rule が narrow に明示されている。
  - `mir-semantics` の parser-free minimal interpreter と detached loop の責務境界が plan/docs/report で揃っている。

### Task 2. Phase 4 shared-space self-driven closeout

- 目的
  - current recommendation を前提に、Phase 4 の self-driven portion を close する。
- 完了条件
  - authoritative room baseline、working subset、minimal witness core、delegated RNG provider placement、control-plane threshold が final snapshot として読める。
  - final activation / authority / auth / consistency / fairness catalog は **user spec required** として tasks / progress / plan/12 に明示されている。
  - Phase 6 current L2 compile-ready PoC に不要な shared-space finalization を immediate blocker にしていない。

### Task 3. Phase 5 proof / protocol / runtime-policy handoff closeout

- 目的
  - theorem prover / protocol verifier / runtime policy へ送る relation slice と stop line を Phase 5 closeout package として固定する。
- 完了条件
  - local / structural / decidable floor と global invariant proof の分界が明示されている。
  - verifier handoff surface、theorem-side retained bridge、protocol/model-check side inventory が 1 本の sequencing として読める。
  - actual external schema / actual emitter / low-level memory-order family は still later と明示されている。

### Task 4. Phase 6 前半 actual parser / AST carrier first tranche

- 目的
  - `mir-ast` public crate を placeholder から一段 actualize し、minimal parser carrier を compile-ready にする。
- 完了条件
  - selected current L2 subset を parse / retain する non-production public API または crate-local API が追加されている。
  - existing stage 1 / 2 / 3 test-only evidence を壊さず、representative fixtures か inline source smoke が通る。
  - generic parser、span-rich diagnostics、final grammar、full request head parse は still later として切れている。

### Task 5. Phase 6 前半 actual checker / runtime skeleton first tranche

- 目的
  - `mir-semantics` / `mir-runtime` をまたいで、parsed subset -> checker floor -> runtime skeleton の compile path を成立させる。
- 完了条件
  - actual parser output を checker floor へつなぐ minimal bridge がある。
  - `mir-runtime` が placeholder ではなく、current L2 subset を受ける non-production skeleton と smoke entry を持つ。
  - parser-free interpreter と actual path の boundary が明記され、両者を混同していない。

### Task 6. Phase 6 前半 compile-ready PoC verification and formal hook

- 目的
  - compile-ready PoC を checkpoint close 扱いできる verification package を揃える。
- 完了条件
  - selected `cargo test` / `cargo check` / smoke command が docs に固定されている。
  - representative parser / checker / runtime path が CI 相当の local gate で通る。
  - theorem-side relation と model-check side relation の **first tranche** が少なくとも tool-neutral export か concrete tool first cut のどちらかで用意されている。
  - report / progress / tasks / plan が compile-ready milestone を mirror している。

## 方針決定が必要な blocker / open question

- **現時点で、Task 1〜3 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、Task 4〜6 へ進む時点では次の decision point が効いてくる。

### Blocker 1. actual parser subset の public boundary

- 概要
  - Phase 6 前半 actual parser に、どこまで current L2 source surface を入れるか。
- 何に影響するか
  - `mir-ast` public API
  - parser test corpus
  - checker / verifier handoff shape
- 主要な選択肢
  1. stage 1 / 2 / selected stage 3 subset だけを先に public-ish へ上げる
  2. request head / clause suite / richer diagnostics まで一気に広げる
  3. さらに docs-only を延長し actual parser をまだ書かない
- current recommendation / 見解
  - **1 を推奨**
  - current repo の evidence と placeholder 状態を両立させる最小 cut である。

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
  - **Task 1〜5 の後で 1 か 2+3 を narrow に選ぶ** のを推奨。
  - 現時点では public boundary がまだ薄いため、先に tool を固定すると surface が逆流しやすい。
