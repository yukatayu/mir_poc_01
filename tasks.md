# tasks

最終更新: 2026-04-13 14:31 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- この snapshot では、`.docs/progress-task-axes.md` に従い、
  - **legacy checkpoint**
  - **macro phase**
  - **feature maturity stage**
  を分けて扱う。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- `tasks.md` は append-only の履歴ではない。**毎回 current snapshot に合わせて全体を書き直す。**

## current checkpoint

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、parser / checker-runtime / formal-hook / source-sample first packages / theorem-first review-unit pilot / bridge sketch first packages / actual `e1` / `e21` / `e3` / `e22` authored-row line / stable-static edge pair / public-surface inventory / shared-space identity-auth layering / model-check concrete carrier first actualization gate までは current entry criteria と読んでよい。
- current macro phase としては、
  - `Macro 6` shared-space docs-first boundary follow-up
  - `Macro 5` sample-visible theorem / model-check milestone
  - `Macro 7` host-facing I/O / adapter boundary inventory
  が near-term line である。
- second source-sample cluster sequencing は `specs/examples/349...350` により fixed 済みであり、post-sextet first cluster は `e21` / `e22` try-rollback locality contrast に置く。
- actual `e22` contrast-row source actualization も `specs/examples/351...352` により fixed 済みであり、current authored source sample は `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` の octet に進んだ。
- stable static malformed post-contrast sequencing は `specs/examples/353...354` により fixed 済みであり、second broader cluster は stable reason-code / fixture-static cluster に置く。
- parser / checker / runtime public surface inventory は `specs/examples/355...356` により fixed 済みであり、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split に整理した。
- Mirrorea/shared-space docs-first re-entry は `specs/examples/357...358` により fixed 済みであり、`mirrorea_fabric_boundary + shared_space_practical_boundary` を current boundary core、Typed-Effect / Prism を adjacent track、shared-space final catalog と upper-layer app target を user-spec-required gate に置く current cut を採った。
- model-check / public-checker second reserve inventory は `specs/examples/359...360` により fixed 済みであり、`proof_notebook_review_unit` を current first concrete pilot に維持したまま、model-check reserve と public-checker docs-only reserve を separate bucket に置く current cut を採った。
- stable-static edge-pair first reopen は `specs/examples/361...362` により fixed 済みであり、existing `e4` row と deferred `e19` row を source-backed static-stop pair に actualize した。
- public operational surface actualization gate は `specs/examples/363...364` により fixed 済みであり、already-public parser-free stack を stable bucket に据えたまま、later public pressure の first docs-only candidate を `run_current_l2_source_sample` に narrow に置いた。
- shared-space identity / auth layering reopen は `specs/examples/365...366` により fixed 済みであり、membership identity core を `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残しつつ、transport/service auth、room admission、display/projection identity を side carriers に押し分ける current cut を採った。
- model-check concrete carrier first actualization gate は `specs/examples/367...368` により fixed 済みであり、`proof_notebook_review_unit` current first pilot を保ったまま、`tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow gate を current first choice に昇格した。
- stable malformed broader follow-up inventory は `specs/examples/369...370` により fixed 済みであり、broader stable malformed next reopen order を missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`) に固定し、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残した。
- public operational CLI / final public contract later gate は `specs/examples/371...372` により fixed 済みであり、`run_current_l2_source_sample` current gate を保ったまま、first later gate を final public parser / checker / runtime API、second later gate を public operational CLI に固定し、repo layout / accepted-set / repo-local helper surface は current final contract の外に残した。
- shared-space admission / compile-time visibility reopen は `specs/examples/373...374` により fixed 済みであり、role / capability / visibility / notify path requirement の over-approximation だけを compile-time に残し、actual admission / activation / active member set / reconciliation は runtime control-plane に残す current cut を採った。
- current promoted line は **shared-space authority / resource ownership reopen** であり、その後に
  - model-check concrete carrier actualization comparison
  - model-check concrete carrier first actualization
  - source-sample emitted verification artifact wiring
  - sample-facing theorem / model-check evidence summary and bless/review flow
  - later `docs-first I/O / host-facing port boundary comparison`（working label）
  を置くのが自然である。
- `stable malformed missing-option first reopen actualization comparison` と `final public parser / checker / runtime API first later gate actualization comparison` は near-term reserve に下げ、sample-visible theorem/model-check line（comparison + 3 package）の後段に残す。

## 次に自走で進める順番

| 順番 | macro phase | feature family | current stage -> next stage | task package | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | `Macro 6` | shared-space authority model | `S3 -> S3-S4` | shared-space authority / resource ownership reopen | 1〜3 task / 数日〜1週 | boundary までは自走可能 |
| 2 | `Macro 5` | external verifier bridge | `S4 -> S4-S5` | model-check concrete carrier actualization comparison | 1〜3 task / 数日〜1週 | 一部自走可能 |
| 3 | `Macro 5` | model-check pilot | `S4-S5 -> S5` | model-check concrete carrier first actualization | 1〜3 task / 数日〜1週 | 一部自走可能 |
| 4 | `Macro 5` | source-backed verification artifacts | `S5 -> S5-S6` | source-sample emitted verification artifact wiring | 1〜3 task / 数日〜1週 | 自走可能 |
| 5 | `Macro 5` | theorem/model-check sample visibility | `S5-S6 -> S6` | sample-facing theorem / model-check evidence summary and bless/review flow | 1〜3 task / 数日〜1週 | 一部自走可能 |
| 6 | `Macro 7` | host-facing integration boundary | `S1-S2 -> S2-S3` | docs-first I/O / host-facing port boundary comparison | 1〜3 task / 数日〜1週 | docs-first boundary までは自走可能 |
| 7 | `Macro 4` | malformed cluster widening | `S5-S6 -> S6` | stable malformed missing-option first reopen actualization comparison | 1〜3 task / 数日〜1週 | 自走可能 |
| 8 | `Macro 7` | public operational contract | `S4 -> S4-S5` | final public parser/checker/runtime first later gate actualization comparison | 1〜3 task / 数日〜1週 | 一部自走可能 |

## 自走可能な task package

### Package 1. shared-space authority / resource ownership reopen

- macro phase
  - `Macro 6`
- 目的
  - participant carrier、resource owner slot、delegated capability、consistency mode、fairness source をどう分けるかを docs-first に整理する。
- 完了条件
  - authoritative room / append-friendly room / relaxed projection line を潰さず、authority placement と resource ownership の first working split を current task map に整理する。

### Package 2. model-check concrete carrier actualization comparison

- macro phase
  - `Macro 5`
- 目的
  - first actualization gate fixed 後の次段として、actual emitted model-check carrier、sample runner に乗る emitted verification artifact wiring、sample-facing theorem/model-check evidence summary のどれから reopen するかを narrow に整理する。
- 完了条件
  - `proof_notebook_review_unit` current first pilot と public-checker docs-only reserve line を巻き戻さず、sample-visible theorem/model-check line（comparison + 3 package）の reopen order を current task map に整理する。

### Package 3. model-check concrete carrier first actualization

- macro phase
  - `Macro 5`
- 目的
  - first actualization gate fixed 後の次段として、actual emitted model-check carrier、sample runner に乗る emitted verification artifact wiring、sample-facing theorem/model-check evidence summary のどれから reopen するかを narrow に整理する。
- 完了条件
  - `proof_notebook_review_unit` current first pilot と public-checker docs-only reserve line を巻き戻さず、sample-visible theorem/model-check line（comparison + 3 package）の reopen order を current task map に整理する。

### Package 4. source-sample emitted verification artifact wiring

- macro phase
  - `Macro 5`
- 目的
  - compare-ready gate の直後に置く narrow non-production actual model-check carrier を、tool-neutral formal hook / compare-ready bridge との整合を保ったまま docs-first + helper-local に actualize する。
- 完了条件
  - `proof_notebook_review_unit` current first pilot を保持しつつ、model-check side の first actual carrier shape、入力境界、fail-closed 条件、kept-later line を fixed する。

### Package 5. sample-facing theorem / model-check evidence summary and bless/review flow

- macro phase
  - `Macro 5`
- 目的
  - current authored sample octet と verification ladder の reached stage を、theorem/model-check side に見せる emitted verification artifact route へ narrow に接続する。
- 完了条件
  - current guarded row (`e3`) と current runtime/static cluster split を壊さず、sample runner / ladder / emitted artifact の接点を helper-local actual path として整理する。

### Package 6. docs-first I/O / host-facing port boundary comparison

- macro phase
  - `Macro 5`
- 目的
  - sample code を読んだ人間が theorem/model-check evidence をどこで確認し、どの review / bless flow を current repo で踏むのかを narrow に整理する。
- 完了条件
  - concrete external tool binding を still later に残したまま、sample-facing evidence summary、review-unit / bridge sketch の見せ方、current bless/review flow を current task map に整理する。

### Package 7. stable malformed missing-option first reopen actualization comparison

- macro phase
  - `Macro 7`
- 目的
  - host-facing I/O、visualizer / substrate adapter、FFI、game engine adapter を current runtime semantics と混ぜずに、docs-first boundary としてどこで切るかを整理する。
- 完了条件
  - `stdin/stdout` を privileged primitive にせず、capability-scoped port / adapter boundary、visualizer / host substrate、FFI / engine adapter を別 gate として比較する current cut を整理する。
- 注記
  - `host-facing port` は working label であり、final terminology は OPEN である。

### Package 8. final public parser/checker/runtime first later gate actualization comparison

- macro phase
  - `Macro 4`
- 目的
  - broader malformed follow-up inventory fixed 後の次段として、missing-option family first reopen を helper-local compare と source-backed wideningのどの順で actualize するかを narrow に整理する。
- 完了条件
  - missing-option first / capability second の順序を巻き戻さず、`e16/e17/e18` line の first actualization cut と duplicate / try-rollback kept-later guard を current task map に整理する。

## 研究を通して見つけること

### Discovery 1. shared-space authority / resource ownership reopen

- 概要
  - authority placement、resource owner slot、delegated capability、fairness source をどの split で扱うか。
- 何に影響するか
  - `Macro 6`
  - authoritative room / append-friendly room の比較土台
- current recommendation
  - **participant carrier と authority/resource owner を同一視せず、authoritative owner slot と delegated/provider placement を別軸に比較する**。

### Discovery 2. model-check concrete carrier actualization sequencing

- 概要
  - current first gate の次段として、actual model-check carrier / emitted verification artifact wiring / sample-facing evidence summary をどう並べるか。
- 何に影響するか
  - `Macro 5`
  - theorem / model-check / public-checker bridge
- current recommendation
  - **current gate を compare-ready entry に保ったまま、actual carrier・artifact wiring・sample summary を separate package として narrow に切る**。

### Discovery 3. docs-first I/O / host-facing port boundary

- 概要
  - host-facing I/O、visualizer / substrate adapter、FFI、game engine adapter をどの capability / adapter boundary で切るか。
- 何に影響するか
  - `Macro 7`
  - future public operational surface と external integration
- current recommendation
  - **`stdin/stdout` を language core の privileged primitive にせず、capability-scoped port / adapter boundary を first docs-only cut にする**。

### Discovery 4. stable malformed missing-option first reopen actualization comparison

- 概要
  - broader malformed follow-up inventory fixed 後に、missing-option family first reopen を helper-local compare と source-backed wideningのどちらから actualize するか。
- 何に影響するか
  - `Macro 4`
  - malformed cluster widening speed
- current recommendation
  - **missing-option first / capability second の順序を保ったまま、duplicate cluster と try/rollback malformed-static family を later に残し、first reopen だけを separate package にする**。

### Discovery 5. final public parser/checker/runtime first later gate actualization comparison

- 概要
  - public operational later ordering fixed 後に、library-side final public contract を symbol-level でどこから narrow に actualize するか。
- 何に影響するか
  - `Macro 7`
  - Rust/Python split の外向き surface
- current recommendation
  - **`run_current_l2_source_sample` current gate と library-before-CLI later orderingを保ち、final public library contract actualization と public operational CLI second gate を同じ package に混ぜない**。

## user が決める必要があること

### Decision 1. shared-space final catalog

- 概要
  - activation / authority / auth / identity / admission / consistency / fairness の final catalog。
- 何に影響するか
  - `Macro 6` 全体
  - upper-layer runtime / governance semantics
- 主要な選択肢
  - authoritative 寄り
  - append-friendly 寄り
  - hybrid / policy overlay
- current recommendation
  - **current repo では docs-first boundary までに留め、final catalog は user が決める**。

### Decision 2. upper-layer application target

- 概要
  - first-class に何を application goal とするか。
- 何に影響するか
  - `Macro 8`
  - sample / demo / acceptance criteria
- 主要な選択肢
  - synchronized shared-space
  - collaborative editing
  - virtual-world / avatar-oriented example
  - other domain target
- current recommendation
  - **current mainline を止める immediate blocker ではないが、Macro 8 へ進む前に必要**。

### Decision 3. backend / tooling success criteria

- 概要
  - backend / editor / operational CLI に何を first success criterion とするか。
- 何に影響するか
  - `Macro 7`
  - Rust/Python split の将来比率
- current recommendation
  - **今は inventory と narrow gate だけに留め、final success criteria は later で user と合わせる**。

### Decision 4. external integration target profile

- 概要
  - visualizer / host substrate / FFI / game engine adapter のどれを first-class external integration target にするか。
- 何に影響するか
  - `Macro 7`
  - `Macro 8`
  - future I/O boundary と demo path
- 主要な選択肢
  - visualizer / inspectable host substrate
  - system integration / FFI
  - game engine adapter
  - hybrid staged rollout
- current recommendation
  - **まず docs-first I/O / adapter boundary を切ってから、first target profile は user と合わせる**。

## 現在の blocker 読み

- **current Package 1〜9 を止める immediate blocker は 0 件** と読むのが自然である。
- current blocker は主に research-discovery 側にあり、user decision は shared-space final catalog、external integration target、upper-layer app target の later gate にある。
