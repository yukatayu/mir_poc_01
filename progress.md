# progress

最終更新: 2026-04-17 00:04 JST

## この文書について

- この文書は repo 全体の **薄い進捗 snapshot** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/`、詳細経緯は `docs/reports/` に置く。
- 進捗率は **rough estimate** であり、問題が見つかれば巻き戻る。

## current snapshot

- current execution line:
  `Macro 4 / malformed duplicate-cluster source-authored static-stop pair actualization comparison with try-rollback malformed-static kept-later inventory`
- current theory-lab line:
  `Macro 5/6 reserve checkpoint close`
  （typed/theorem/model-check second-order reserve と order/modality reserve は fixed 済み、next reopen は third-order follow-up）
- current reserve integration line:
  `Macro 6/7 reserve integration checkpoint close`
  （public operational CLI packaging reserve note と shared-space fairness / replay strengthening reserve note は fixed 済み、next reopen は later mixed gate）
- immediate blocker: `0`
- current lane を止める user decision: `0`
- current authored source sample:
  `e1 / e2 / e3 / e4 / e13 / e16 / e18 / e19 / e20 / e21 / e22 / e23`

## macro phase map

| Macro phase | 主眼 | 現在位置 | 重さ | 自走可否 |
|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 低 | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 中 | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 中 | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 中 | 着手可能 |
| `Macro 4` | executable fixed-subset sample expansion | duplicate cluster reopen sequencing へ進行中 | 重 | 着手可能 |
| `Macro 5` | typed / theorem / model-check bridge | reserve checkpoint close、next reopen は third-order follow-up | 重 | 着手可能 |
| `Macro 6` | fabric / shared-space / runtime evolution | docs-first boundary plus reserve strengthening | 重 | 着手可能（boundaryまで） |
| `Macro 7` | toolchain / backend / host-facing integration | thin facade / shell actualization 後の packaging reserve | 重 | 着手可能（boundaryまで） |
| `Macro 8` | domain / application realization | not started | とても重い | 要仕様確認 |

## 特徴機能ごとの到達度

| feature family | 現在地 | できていること | 次の意味ある一歩 |
|---|---|---|---|
| event DAG / local `try` / `atomic_cut` | `S5-S6` | parser-free / source-backed runnable evidence | duplicate cluster widening comparison |
| guarded option chain / `lease` / monotone degradation | `S6` | missing-option / capability pair まで source-authored static-stop widening | duplicate cluster の next widen cut を決める |
| contracts / static gate / formal hook | `S6` | static gate、tool-neutral formal hook、emitted artifact wiring、request / predicate / `try` cluster typed-surface reserve | typed-surface family unification keep/drop note |
| theorem-side pilot | `S5-S6` | notebook-first pilot、lemma order、lemma wording floor、bridge stop-line refresh、admissible evidence contraction | notebook-consumer threshold / discharge reserve |
| model-check line | `S5-S6` | row-local carrier、projection reserve inventory、sample-facing summary、tool-binding stop line | small-cluster projection keep/drop refresh |
| typed-core / type system | `S3-S4` | obligation owner matrix、attachment inventory、source-visible first comparison、request/predicate/`try` reserve | typed-surface family unification keep/drop note |
| multi-node / shared-space | `S3-S4` at boundary | identity / admission / authority / room-profile-host bridge note、fairness / replay reserve note | final operational catalog / fairness operational profile mixed gate |
| dynamic attach / detach / safe downstream addition | `S2-S3` | DAG discipline、overlay constraints、shared-space boundary notes | operational profile / fairness reserve は later |
| host-facing I/O / adapter boundary | `S4` | capability-scoped docs-first boundary、thin facade、Rust shell actualization、CLI packaging reserve note | adapter target / final host contract mixed gate |
| ordering / `memory_order` reinterpretation | `S4-S5` | cut-family comparison、relation decomposition、falsifier coverage、property-language bridge、emitted-artifact schema reserve | source-surface wording reserve |
| syntax / modality theory line | `S3-S4` | syntax honesty principle、stimuli comparison、modal promotion threshold、guarded-vs-MDTT/MTT reduction timing | modality internalization trigger |
| public dev surface / CLI | `S4-S5` | thin facade、later support、current-L2 scoped Rust shell actualization | installed-binary / final packaging reserve |
| executable sample corpus | `S6` | authored dozen、runner / ladder / regression helper、artifact fan-out、duplicate next-cut comparison | duplicate cluster source-authored static-stop pair actualization comparison |

## 層ごとの進捗

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手性 |
|---|---|---|---|---|
| semantic kernel | 92% | 85% | 76% | 着手可能 |
| parser-free substrate | 88% | 78% | 86% | 着手可能 |
| compile-ready minimal actualization | 84% | 73% | 83% | 着手可能 |
| fixed-subset source sample line | 83% | 78% | 83% | 着手可能 |
| typed / theorem / model-check bridge | 79% | 72% | 62% | 着手可能 |
| shared-space docs-first boundary | 57% | 48% | 27% | 着手可能 |
| host-facing integration boundary | 51% | 43% | 34% | 着手可能 |
| ordering / memory-model reinterpretation | 61% | 52% | 15% | 着手可能 |
| syntax / modality foundation comparison | 44% | 40% | 5% | 着手可能 |
| backend / public packaging | 29% | 25% | 24% | 要仕様確認 |
| application realization | 10% | 8% | 2% | 要仕様確認 |

## twin peaks

### 1. 型システム / 定理証明 / モデル検査

- 位置づけ:
  repo 全体で最も重い研究線のひとつであり、`Macro 5` の主核である。
  ただし current reading は「full implementation を急ぐ線」ではなく、**typed-core / theorem / model-check の境界と stop line を ratchet 方式で固める線**である。
- 現段階:
  `Macro 5/6` の reserve checkpoint close 後、third-order follow-up を切る段階。
  feature-stage の読みでは、typed-core は `S3-S4`、theorem/model-check boundary は `S5-S6` にあり、bridge 全体としては `boundary hardening` 段階にいる。
- rough progress:
  全体の bridge line は `79% / 72% / 62%`（論理仕様 / ユーザ向け仕様 / 実装・運用）。
  内訳の rough read は、`typed-core boundary 約60%`、`theorem pilot boundary 約74%`、`model-check boundary 約68%`。
  ここで内訳は repo の current package 群からの synthesis であり、規範値ではない。
- 採用済みの骨格:
  - typed-core は full calculus first ではなく、`checker-adjacent semantic carrier` を first attachment に置く。
  - theorem-side は syntax tree 全体ではなく、semantic-core invariant family を first pilot に置く。
  - model-check は row-local machine-facing carrier を current floor に置き、small-cluster projection を reserve に留める。
  - request / predicate / `try` cluster は grouped typed-surface reserve cue に留める。
  - admissible theorem evidence は symbolic refs only に contraction する。
  - model-check tool binding は explicit stop line を current cut にする。
  - `formal_hook`、`proof_notebook_review_unit`、`model_check_concrete_carriers`、emitted artifact wiring、sample-facing theorem/model-check summary は既に source-backed である。
- 最初の構想からの整理:
  当初の「強い型 / 定理証明 / モデル検査を早く入れる」方向は、そのまま immediate implementation line にはしていない。
  current repo では、まず
  `obligation owner`、
  `attachment cut`、
  `proof artifact と bridge stop line`、
  `projection grain と tool seam`
  を docs-first に固定し、あとから concrete tool や stronger type surface を接続できるように整理している。
- いま残っている OPEN:
  - `request / predicate / try` cluster の typed surface
  - admissible evidence contraction
  - concrete prover brand と proof-object/public-contract shape
  - model-check concrete tool binding と first settled property language
  - final typed syntax / final public verifier contract / full strong type system
- 直近の研究計画:
  1. `typed-surface family unification keep/drop note`
  2. `notebook-consumer threshold and discharge reserve note`
  3. `model-check small-cluster projection keep/drop refresh`
  4. その後に stronger typed surface / concrete theorem-model-check binding の可否を再評価する
- 自走範囲:
  current stop line までは `着手可能`。
  concrete theorem/model-check binding と stronger typed surface promotion は `mixed gate`。
  final public contract と application-level guarantee surface は `要仕様確認`。

### 2. fence / atomic / `memory_order` / authority-handoff 再構築

- 位置づけ:
  repo 全体で最も重い研究線のもうひとつであり、`Macro 5/6` theory-lab と `Macro 6` shared-space boundary の接点である。
  current reading は「low-level concurrency API を先に決める線」ではなく、**cut / order / visibility / witness / authority / replay を分解して comparison framework を固める線**である。
- 現段階:
  `Macro 5/6` reserve checkpoint close 後、source-surface wording と modality internalization trigger を切る段階。
  feature-stage の読みでは、ordering / `memory_order` 再解釈は `S4-S5`、shared-space docs-first boundary は `S3-S4` にあり、current phase は `comparison framework hardening` である。
- rough progress:
  `ordering / memory-model reinterpretation` は `61% / 52% / 15%`。
  隣接する `shared-space docs-first boundary` は `57% / 48% / 27%`。
  authority-handoff operational side は shared-space row と密接だが、current snapshot ではまだ独立の progress row を持たない。
  そのため、この重線全体に対する単一の combined percentage はまだ固定しない。
- 採用済みの骨格:
  - `atomic_cut` は current `place` の rollback frontier を確定する local finalizing cut であり、global consistent cut / durable commit / distributed sync point ではない。
  - cut family は `local finalization / ordering-only barrier / snapshot-only observation / durable commit` に分けて比較する。
  - order family は `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before` を working relation decomposition とする。
  - `authority_serial_transition_family` を first kept candidate、`witness_aware_commit_family` を second candidate に置く。
  - emitted-artifact schema は refs-only reserve schema を first cut に留める。
  - guarded と MDTT/MTT cluster は threshold-gated parallel keep に留める。
  - low-level `std::memory_order` / `kill_dependency` family は reference family として保持するが、current source surface には直接輸入しない。
  - thread と node は programming surface では同じ causal language で扱いたいが、差は lowering / evidence / failure / transport / fairness に残す。
- 最初の構想からの整理:
  当初の「fence / atomic / memory_order を再構築する」方向は、そのまま source-level API 設計にはしていない。
  current repo では、
  `atomic_cut` を narrow local nucleus に残し、
  higher-level handoff / publication / witness / fairness family を docs-first に比較し、
  adequacy corpus と falsifier loop で破綻候補を落としながら、
  low-level family は backend-aligned reference として後段に残す形へ整理している。
- いま残っている OPEN:
  - final property language
  - order / handoff emitted-artifact schema
  - final source-surface handoff syntax
  - `snapshot_cut` / `consistent_cut` の settled naming
  - low-level `memory_order` reinterpretation の final stance
  - scheduler / runtime finalization
  - shared-space final fairness / replay / authority catalog
- 直近の研究計画:
  1. `order / handoff source-surface wording reserve note`
  2. `modality internalization trigger note`
  3. その後に source-surface handoff wording と concrete tool / runtime binding の昇格可否を再評価する
- 自走範囲:
  docs-first comparison、adequacy corpus hardening、candidate reduction、shared-space boundary strengthening までは `着手可能`。
  concrete tool / runtime binding と fairness operational profile は `mixed gate`。
  final source syntax、final fairness policy、external protocol/runtime commitment は `要仕様確認`。

## research / user split

### 研究で見つけること

- typed-surface family unification keep/drop
- notebook-consumer threshold and discharge reserve
- model-check small-cluster projection keep/drop
- order / handoff source-surface wording reserve
- modality internalization trigger note

### mixed gate / later reserve

- shared-space final fairness / replay operational profile
- public operational CLI installed-binary / packaging success criteria

### user が後で決めること

- shared-space final catalog
- first external integration target
- backend / tooling success criteria
- first application target

## recent log

- 注記: この欄は recent log として保つ。詳細は `docs/reports/` を正本にする。
- 2026-04-17 00:04 JST — `docs/reports/0706` で `specs/examples/433...438` を追加し、typed/theorem/model-check second-order reserve、order/handoff emitted-artifact schema reserve、guarded-vs-MDTT/MTT reduction timing、duplicate-cluster next widening cut を docs-first に固定したうえで、theory-lab lane を reserve checkpoint close、execution lane を duplicate pair actualization comparison へ更新した。
- 2026-04-16 23:42 JST — `docs/reports/0705` で `specs/examples/431...432` を追加し、public operational CLI packaging reserve note と shared-space fairness / replay strengthening reserve note を docs-first に固定したうえで、reserve integration line を checkpoint close とし、current self-driven queue を theory-lab reserve packages と duplicate-cluster execution reopen に更新した。
- 2026-04-16 23:20 JST — `docs/reports/0704` で twin peaks 追跡用の節を `progress.md` に追加し、typed/theorem/model-check 線と order/memory-order/authority-handoff 線について、現段階・rough progress・source-backed floor・OPEN・次の research package を分離して snapshot 化した。
- 2026-04-16 22:34 JST — `docs/reports/0703` で `specs/examples/423...430` を追加し、public operational CLI current-L2 shell actualization、shared-space room-profile / host-binding bridge-only note、typed/theorem/model-check second-order docs-first follow-up、duplicate-cluster later reopen comparisonを close したうえで、current line を packaging reserve / fairness-replay reserve / typed-theorem-model-check reserve / duplicate-cluster source-sample widening comparison に更新した。
- 2026-04-16 21:55 JST — `docs/reports/0702` で `e13/e20` capability pair を source-authored static-stop pair として actualize し、sample corpus を authored dozen に widen したうえで、typed-surface comparison / theorem lemma wording hardening / model-check bridge note / order-handoff candidate reduction / modal follow-up を `specs/examples/417...422` と `plan/11` `12` `17` `18` に反映した。
- 2026-04-16 21:19 JST — `docs/reports/0701` で typed-core attachment inventory、semantic-core theorem pilot planning、model-check projection reserve inventory、order/handoff falsifier hardening を `specs/examples/413...416`、`specs/10`、`specs/11`、`plan/11` `12` `16` `17` `18` に反映した。
- 2026-04-16 19:03 JST — `docs/reports/0700` で plan/docs/snapshot を three-lane reading に再構成し、`faq_004.md` を新規追加して `progress.md` / `tasks.md` を薄い snapshot へ全面更新した。
- 2026-04-16 18:06 JST — `docs/reports/0699` で order / handoff / memory-model / syntax / modality theory line を `specs/examples/405...412` と `plan/11` `12` `13` `17` `18` に切り出し、theory-lab operating model、adequacy corpus、property-to-boundary matrix、literature-backed comparison cut を揃えた。
