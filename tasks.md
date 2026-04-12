# tasks

最終更新: 2026-04-13 08:50 JST

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
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、parser / checker-runtime / formal-hook / source-sample first packages / theorem-first review-unit pilot / plain bridge sketch first actualization / compare-ready bridge sketch second reopen / actual `e3` authored-row actualization までは current entry criteria と読んでよい。
- current macro phase としては、
  - `Macro 5` theorem / verifier bridge
  - `Macro 4` executable fixed-subset sample expansion
  - `Macro 6` shared-space docs-first boundary follow-up
  が near-term line である。
- second source-sample cluster sequencing は `specs/examples/349...350` により fixed 済みであり、post-sextet first cluster は `e21` / `e22` try-rollback locality contrast に置く。
- actual `e22` contrast-row source actualization も `specs/examples/351...352` により fixed 済みであり、current authored source sample は `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` の octet に進んだ。
- stable static malformed post-contrast sequencing は `specs/examples/353...354` により fixed 済みであり、second broader cluster は stable reason-code / fixture-static cluster に置く。
- parser / checker / runtime public surface inventory は `specs/examples/355...356` により fixed 済みであり、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split に整理した。
- model-check / public-checker second reserve inventory は `specs/examples/359...360` により fixed 済みであり、`proof_notebook_review_unit` を current first concrete pilot に維持したまま、model-check reserve と public-checker docs-only reserve を separate bucket に置く current cut を採った。
- stable-static edge-pair first reopen は `specs/examples/361...362` により fixed 済みであり、existing `e4` row と deferred `e19` row を source-backed static-stop pair に actualize した。
- public operational surface actualization gate は `specs/examples/363...364` により fixed 済みであり、already-public parser-free stack を stable bucket に据えたまま、later public pressure の first docs-only candidate を `run_current_l2_source_sample` に narrow に置いた。
- shared-space identity / auth layering reopen は `specs/examples/365...366` により fixed 済みであり、membership identity core を `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残しつつ、transport/service auth、room admission、display/projection identity を side carriers に押し分ける current cut を採った。
- model-check concrete carrier first actualization gate は `specs/examples/367...368` により fixed 済みであり、`proof_notebook_review_unit` current first pilot を保ったまま、`tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow gate を current first choice に昇格した。public-checker docs-only chain は parallel reserve に残し、model-check concrete carrier actualization、actual public-checker migration、actual emitted verifier handoff artifact、concrete tool binding は still later に残している。
- stable malformed broader follow-up inventory は `specs/examples/369...370` により fixed 済みであり、broader stable malformed next reopen order を missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`) に固定し、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残した。
- current promoted line は **public operational CLI / final public contract later gate** であり、その後に
  - shared-space admission / compile-time visibility reopen
  - shared-space authority / resource ownership reopen
  - model-check concrete carrier actualization comparison
  を置くのが自然である。

## 次に自走で進める順番

| 順番 | macro phase | feature family | current stage -> next stage | task package | rough 所要 | 自走可否 |
|---|---|---|---|---|---|---|
| 1 | `Macro 7` | public operational contract | `S3-S4 -> S4` | public operational CLI / final public contract later gate | 1〜3 task / 数日〜1週 | 一部自走可能 |
| 2 | `Macro 6` | shared-space boundary | `S3-S4 -> S4` | shared-space admission / compile-time visibility reopen | 1〜3 task / 数日〜1週 | boundary までは自走可能 |
| 3 | `Macro 6` | shared-space authority model | `S3 -> S3-S4` | shared-space authority / resource ownership reopen | 1〜3 task / 数日〜1週 | boundary までは自走可能 |
| 4 | `Macro 5` | external verifier bridge | `S4 -> S4-S5` | model-check concrete carrier actualization comparison | 1〜3 task / 数日〜1週 | 一部自走可能 |
| 5 | `Macro 4` | malformed cluster widening | `S5-S6 -> S6` | stable malformed missing-option first reopen actualization comparison | 1〜3 task / 数日〜1週 | 自走可能 |

## 自走可能な task package

### Package 1. public operational CLI / final public contract later gate

- macro phase
  - `Macro 7`
- 目的
  - public operational surface actualization gate fixed 後に、public library surface と public CLI の later reopen order を narrow に整理する。
- 完了条件
  - `run_current_l2_source_sample` first docs-only candidate を巻き戻さず、final public parser / checker / runtime API と public CLI の reopen line を separate gate として整理する。

### Package 2. shared-space admission / compile-time visibility reopen

- macro phase
  - `Macro 6`
- 目的
  - identity/auth layering fixed 後の next shared-space reopen として、compile-time over-approximation と runtime admission / activation / reconciliation の cut を narrow に整理する。
- 完了条件
  - role / capability / visibility requirement の over-approximationを compile-time に残し、actual admission / activation / late join / reconciliation を runtime control-plane に残す current cut を snapshot に昇格する。

### Package 3. shared-space authority / resource ownership reopen

- macro phase
  - `Macro 6`
- 目的
  - participant carrier、resource owner slot、delegated capability、consistency mode、fairness source をどう分けるかを docs-first に整理する。
- 完了条件
  - authoritative room / append-friendly room / relaxed projection line を潰さず、authority placement と resource ownership の first working split を current task map に整理する。

### Package 4. model-check concrete carrier actualization comparison

- macro phase
  - `Macro 5`
- 目的
  - first actualization gate fixed 後の次段として、actual emitted model-check carrier、concrete tool binding、public-checker actual migration のどれから reopen するかを narrow に整理する。
- 完了条件
  - `proof_notebook_review_unit` current first pilot と public-checker docs-only reserve line を巻き戻さず、model-check actualization 側の next compare-ready reopen order を current task map に整理する。

### Package 5. stable malformed missing-option first reopen actualization comparison

- macro phase
  - `Macro 4`
- 目的
  - broader malformed follow-up inventory fixed 後の次段として、missing-option family first reopen を helper-local compare と source-backed wideningのどの順で actualize するかを narrow に整理する。
- 完了条件
  - missing-option first / capability second の順序を巻き戻さず、`e16/e17/e18` line の first actualization cut と duplicate / try-rollback kept-later guard を current task map に整理する。

## 研究を通して見つけること

### Discovery 1. public operational CLI / final public contract later gate

- 概要
  - first docs-only candidate fixed 後に、public library surface と public CLI をどの順で reopen するか。
- 何に影響するか
  - `Macro 7`
  - Rust/Python split の外向き surface
- current recommendation
  - **`run_current_l2_source_sample` current gate を保ち、CLI と final public contract は separate later gate として narrow に reopen する**。

### Discovery 2. shared-space admission / compile-time visibility reopen

- 概要
  - identity/auth layering の次段として、declaration-side role/capability/visibility と runtime admission policy の橋をどこで切るか。
- 何に影響するか
  - `Macro 6`
  - proof / model-check 側へ送る shared-space static floor
- current recommendation
  - **declared role / capability / visibility over-approximation を compile-time に残し、actual admission / activation は runtime control-plane に残す**。

### Discovery 3. shared-space authority / resource ownership reopen

- 概要
  - authority placement、resource owner slot、delegated capability、fairness source をどの split で扱うか。
- 何に影響するか
  - `Macro 6`
  - authoritative room / append-friendly room の比較土台
- current recommendation
  - **participant carrier と authority/resource owner を同一視せず、authoritative owner slot と delegated/provider placement を別軸に比較する**。

### Discovery 4. model-check concrete carrier actualization comparison

- 概要
  - first actualization gate fixed 後に、actual emitted model-check carrier、concrete tool binding、public-checker actual migration のどれから reopen するか。
- 何に影響するか
  - `Macro 5`
  - theorem / model-check / public-checker bridge
- current recommendation
  - **current gate を compare-ready entry に保ったまま、actual emitted carrier / concrete tool binding / public-checker actual migration を同じ package に混ぜない**。

### Discovery 5. stable malformed missing-option first reopen actualization comparison

- 概要
  - broader malformed follow-up inventory fixed 後に、missing-option family first reopen を helper-local compare と source-backed wideningのどちらから actualize するか。
- 何に影響するか
  - `Macro 4`
  - malformed cluster widening speed
- current recommendation
  - **missing-option first / capability second の順序を保ったまま、duplicate cluster と try/rollback malformed-static family を later に残し、first reopen だけを separate package にする**。

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

## 現在の blocker 読み

- **current Package 1〜5 を止める immediate blocker は 0 件** と読むのが自然である。
- current blocker は主に research-discovery 側にあり、user decision は主として `Macro 6` 以降の gate にある。
