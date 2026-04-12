# tasks

最終更新: 2026-04-12 12:17 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 今回の snapshot は、**Phase 1〜5 の self-driven closeout / freeze と、Phase 6 front-half compile-ready checkpoint close fixed 後に、具体的な source text sample と verification ladder へどう到達するか** を見通せる粒度で書く。
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
- ただし、**「compile-ready checkpoint が fixed」であることと、「source text sample を fixed subset で持ち、static / interpreter / formal staging を sample ごとに通せる」ことは別**である。前者は 96%+ と読める一方、後者へ進むための task chain は、旧 snapshot では `fixed-subset sample/program corpus staging` の 1 項目に圧縮されすぎていた。
- current immediate line は **Phase 6 sample path actual parser-to-`Program` lowering first cut** である。`specs/examples/315...316` により、source corpus は repo-root `samples/current-l2/` flat `.txt` layer に置き、sample stem は fixture stem aligned naming、initial cluster は `e1` / `e2` / `e3` / `e4` / `e21` / `e23` に留める current cut まで fixed 済みである。さらに `specs/examples/317...318` により、representative prose / fixture corpus / source target path の mapping matrix も fixed 済みであり、`e3` は `E3-variant` row、`e23` は unresolved representative anchor として mirror する current cut まで揃った。その後は、**actual parser-to-`Program` lowering → syntax-backed sample runner → verification ladder → authoring/bless policy** の順に進めるのが自然である。
- fixed subset の executable sample を増やすこと自体は current line に整合する。よいのは **既存 settled subset を `static gate` / `interpreter` / `tool-neutral formal hook` に段階接続すること**であり、low-level memory-order-like surface や higher-level async-control family を executable core surface として同時に太らせることではない。

## 具体的な sample code 検証段階までの rough estimate

| 目標 | 現在地 | rough 進捗 | 追加で必要な package | rough 所要 | 注記 |
|---|---|---:|---|---|---|
| parser-free current L2 PoC | 実在し、compile / test evidence あり | 90%+ | maintenance closeout だけ | 1〜2 package / 2〜4日 | `mir-semantics` 主線はすでに回る |
| Phase 6 front-half compile-ready minimal actual PoC | checkpoint close fixed | 96%+ | maintenance / reopen only | maintenance only | current L2 subset の compile-ready checkpoint は閉じたと読んでよい |
| fixed-subset syntax-backed sample verification milestone | source corpus scope/layout と mapping matrix fixed | **56%前後** | 2〜4 package | 2〜5週 | source text sample を parse / lower / static / interpreter / formal hook まで段階接続する |
| first theorem-first concrete tool pilot | reserve inventory fixed | 8%前後 | 2〜4 package | 1〜3週 | source-sample ladder の後で narrow proof consumer pressure を受ける |

### `48%前後` の根拠

- 進んでいるもの
  - representative programs の prose
  - machine-check される fixture corpus
  - parser-free interpreter / detached loop / static gate / formal hook
  - stage 1 / stage 2 carrier と stage 3 admit attached slot / predicate fragment の narrow parser evidence
- まだ薄いもの
  - source text sample corpus 自体
  - representative / fixture / source の対応表
  - actual parser-to-`Program` lowering
  - syntax-backed sample runner
  - sample ごとの static / interpreter / formal ladder
  - source-sample bless / regression policy

## 次に着手すべき順番と rough estimate

| 順番 | phase | task package | 完了条件の要点 | rough weight | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | Phase 6 sample path | actual parser-to-`Program` lowering first cut | fixed subset の parse result を semantic `Program` へ fail-closed に落とす | 重い | 1〜2 task / 3〜7日 | 自走可能 |
| 2 | Phase 6 sample path | syntax-backed sample runner first cut | source sample を parse / lower / static gate / interpreter へ流す narrow runner を作る | 重い | 1〜2 task / 3〜7日 | 自走可能 |
| 3 | Phase 6 sample path | verification ladder wiring | sample ごとに `static gate` / `interpreter` / `formal hook` の到達段を揃える | 中〜重い | 1〜2 task / 2〜6日 | 自走可能 |
| 4 | Phase 6 sample path | source-sample authoring / bless / regression policy | source sample の更新手順、reference 更新、drift suppression を固定する | 中 | 1 task / 1〜3日 | 自走可能 |

## 自走で進める task package

### Task 1. Phase 6 sample path actual parser-to-`Program` lowering first cut

- 目的
  - fixed subset の parse result を semantic `Program` へ narrow / fail-closed に落とす。
- 完了条件
  - lowering failure が explicit で、source / parser / semantic mismatch を silent repair しない。
  - host/runtime/public API の final shape を先取りしない。

### Task 2. Phase 6 sample path syntax-backed sample runner first cut

- 目的
  - source sample から parse / lower / static gate / interpreter を最小経路で回す。
- 完了条件
  - parser-free fixture runner と責務が分離されている。
  - helper-local thin wrapper で止まり、public CLI / final exporter に誤昇格していない。

### Task 3. Phase 6 sample path verification ladder wiring

- 目的
  - sample ごとに `static gate` / `interpreter` / `formal hook` のどこまで machine-check するかを段階化する。
- 完了条件
  - sample ID ごとに current reached stage が明示されている。
  - theorem/model-check concrete tool binding を早く混ぜず、tool-neutral formal hook を current top として扱う。

### Task 4. Phase 6 sample path source-sample authoring / bless / regression policy

- 目的
  - source sample の更新手順、fixture との対応維持、reference 更新、drift suppression を repo-local に固定する。
- 完了条件
  - source sample を増やしても representative prose / fixture corpus / detached loop の責務が崩れない。
  - 長期的な bless/update が docs-only でなく実務フローとして読める。

## 方針決定が必要な blocker / open question

- **現時点で、current Task 1〜4 を止める immediate blocker は 0 件** と読むのが自然である。
- ただし、current mainline では parser-side widen boundary、source corpus layering、lowering cut、formal/back-end timing の guard が効く。

### Blocker 1. actual parser-to-`Program` lowering の first cut

- 概要
  - parse result を semantic `Program` にどう落とすか。
- 何に影響するか
  - lowering coverage
  - syntax-backed runner
  - interpreter / formal hook への接続
  - fail-closed policy
- 主要な選択肢
  1. fixed subset だけを direct lower する narrow bridge を先に入れる
  2. richer host/runtime/public surface と同時に lowerer を作る
  3. lowerer を作らず source sample から fixture へ変換するだけに留める
- current recommendation / 見解
  - **1 を current first choice** に置き、bridge mismatch は fail-closed に止めるのが自然である。

### Blocker 2. concrete formal tool binding と backend/codegen timing

- 概要
  - theorem/model-check concrete tool や LLVM-family backend / external codegen をいつつなぐか。
- 何に影響するか
  - parser / lowering / runtime boundary
  - sample corpus の syntax fix pressure
  - long-term implementation flexibility
- 主要な選択肢
  1. source corpus / lowering / runner より前に backend or tool binding を入れる
  2. fixed-subset source corpus と verification ladder を先に揃え、その後 theorem-first concrete tool pilot を行う
  3. final grammar / public API closeout まで backend/tool binding を遅らせる
- current recommendation / 見解
  - **2 を current first choice** に置くのが自然である。
  - LLVM-family backend や external codegen を今すぐ主線に置くのは早い。syntax-backed fixed subset、lowering、runner、tool-neutral formal hook ladder を先に安定化した方が柔軟性を失いにくい。

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
