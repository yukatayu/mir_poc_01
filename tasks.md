# tasks

最終更新: 2026-04-12 02:14 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half compile-ready checkpoint close fixed 後に、next reopen point をどこへ置くか** までを見通せる粒度で書く。
- ここでいう「Phase 完了」は、**現 phase の self-driven package を閉じ、残件を user spec required または heavy future workstream として明示的に切り分け終えた状態**を指す。`specs/` や `plan/13` に残す heavy future workstream まで今すぐ全部 actualize する意味ではない。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な時系列は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## 現在の読み

- **Phase 1** の current L2 semantics は、`specs/examples/291...292` により self-driven closeout fixed と読んでよい。semantic core 自体は変えず、invariants / proof-obligation wording / notation boundary の narrow closeout を source-backed に閉じ、final parser grammar / final type system / actual external schema は later に残している。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は source-backed に固定し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- **Phase 3** は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。minimal parser subset は stage 1 + stage 2 structural floor、first checker reconnect は stage 1 summary + stage 2 try/rollback structural contract までを bridge にし、stage 3 request / predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残している。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。current package は `specs/examples/121...125` を維持し、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、`delegated_provider_attestation` non-core line、control-plane separated carrier actualization、distributed fairness protocol、final operational realizationは later に残している。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。verifier handoff surface、theorem retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later line を 1 本の closeout bundle にまとめ、actual subject row、boundary-specific handoff artifact family、actual emitted verifier artifact、concrete tool binding、public checker migration、low-level memory-order family は later に残している。
- **Phase 6 front-half** は parser first tranche が `specs/examples/299...300`、checker/runtime first tranche が `specs/examples/301...302`、compile-ready verification / formal hook が `specs/examples/303...304` で fixed 済みであり、checkpoint drift suppression / mirror sweep も report-backed に close 済みである。`specs/examples/305...306` により next reopen sequencing も fixed 済みであり、`specs/examples/307...308` により parser second tranche first package も attached-slot / predicate route として actualize 済みである。`specs/examples/309...310` により reserve formal tool binding inventory も fixed 済みであり、theorem-first concrete binding は first reserve、model-check side は second reserve に整理した。`mir-ast` には stage 1 / stage 2 non-production carrier に加え、stage 3 declaration-side admit attached slot と shared isolated predicate fragment が入り、multiline attachment / request clause suite helper は support-local structural family に留めている。current next line は **Phase 6 parser-side follow-up package sequencing** である。

## PoC compile までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 前半 compile-ready minimal actual PoC | checkpoint close fixed | **96%+** | maintenance / reopen only | maintenance only | current L2 subset の compile-ready checkpoint は閉じたと読んでよい |

### 進捗 96%+ の根拠

- 進んでいるもの
  - semantic core、invariants、representative fixtures、parser-free harness、detached validation loop、Phase 3 staged parser spike evidence、minimal parser subset freeze、parser-to-checker reconnect freeze、`mir-ast` stage 1 / stage 2 non-production carrier、`mir-semantics` program-level entry、`mir-runtime` current L2 thin skeleton、selected compile/test/smoke gate、tool-neutral formal hook first tranche、checkpoint drift suppression / mirror sweep。
- まだ薄いもの
  - concrete theorem / model-check tool binding
  - final public parser / checker / runtime surface
  - fixed-subset sample/program corpus staging

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 次段 | parser-side follow-up package sequencing | shared single attachment frame を同梱するか next package に残すかを narrow に決める | 中 | 1 task / 1〜3日 | 自走可能 |
| 2 | Phase 6 次段 | parser-side follow-up package actualization | selected follow-up line の最小 cut を actualize する | 中〜重い | 1〜2 task / 3〜7日 | 自走可能 |
| 3 | Phase 6 reserve / later line | fixed-subset sample/program corpus staging | 固まった subset のサンプル群と静的検証 staging を current freeze を壊さずに整理する | 中 | 1〜2 task / 2〜5日 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 次段 parser-side follow-up package sequencing

- 目的
  - parser second tranche first package の後に shared single attachment frame を同梱するか、次 package に残すかを narrow に決める。
- 完了条件
  - attached-slot / predicate route の current stop line が snapshot に mirror されている。
  - perform head / clause suite / richer diagnostics と混線していない。

### Task 2. Phase 6 次段 parser-side follow-up package actualization

- 目的
  - selected parser-side follow-up line の最小 cut を actualize する。
- 完了条件
  - spec / report / verification が source-backed に揃っている。
  - request head / richer diagnostics bulk widen を still later に残している。

### Task 3. Phase 6 reserve / later line fixed-subset sample/program corpus staging

- 目的
  - 固まった subset に対して、単一要素から非自明例までの sample/program corpus と static / theorem / model-check verification staging を段階化する。
- 完了条件
  - current fixed subset を壊さない corpus staging の順序と guard が snapshot に mirror されている。
  - final grammar / final tool binding / broad syntax revision と混線していない。

## 方針決定が必要な blocker / open question

- **現時点で、Task 1〜3 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、current next line では parser-side follow-up sequencing と fixed-subset corpus staging の guard が効いてくる。

### Blocker 1. actual parser subset の second-tranche widen boundary

- 概要
  - compile-ready checkpoint close 後に、どこまで current L2 source surface を widen するか。
- 何に影響するか
  - `mir-ast` public-ish API
  - parser test corpus
  - checker / verifier handoff shape
- 主要な選択肢
  1. attached-slot / predicate route を next reopen の first line に置く
  2. selected stage 3 / perform head を next reopen の first line に置く
  3. request head / clause suite / richer diagnostics まで一気に広げる
- current recommendation / 見解
  - **1 を current first choice に固定し、2 と 3 は later に残す** のが自然である。
  - current checkpoint close を壊さない narrow reopen を維持する。

### Blocker 2. fixed-subset sample/program corpus の phase gate

- 概要
  - 現在固定済みの subset に対して、サンプル群・静的検証・interpreter 実行・later formal staging をどの順に接続するか。
- 何に影響するか
  - representative sample corpus
  - static / theorem / model-check staging
  - grammar widening の timing
- 主要な選択肢
  1. fixed subset の sample/program corpus と interpreter/static gate staging を先に整理する
  2. grammar / syntax widening と同時に sample corpus を太らせる
  3. concrete tool binding の actualization と同時に formal corpus を作る
- current recommendation / 見解
  - **1 を current first choice に置き、final grammar / concrete tool binding とは切り離す** のが自然である。
  - current fixed subset を壊さず、sample/program corpus は static gate / interpreter / later formal staging を段階化して増やす方が安全である。
