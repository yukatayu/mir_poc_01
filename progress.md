# progress

最終更新: 2026-04-17 13:58 JST

## この文書について

- この文書は repo 全体の **薄い進捗 snapshot** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/`、詳細経緯は `docs/reports/` に置く。
- 進捗率は **rough estimate** であり、問題が見つかれば巻き戻る。

## current snapshot

- current execution line:
  `Macro 0〜4 closeout fixed`
  （duplicate pair `e14/e15` actualized、broader try-rollback malformed-static family は kept-later inventory）
- current theory-lab line:
  `Macro 5 boundary / pilot / framing closeout fixed`
  （modality internalization trigger、stronger typed-surface threshold framing、theorem discharge transport / public-contract later-gate framing、model-check property-language / tool-binding later-gate framing は fixed 済み、remaining topics は mixed gate only）
- current reserve integration line:
  `Macro 6/7 mixed-gate boundary fixed`
  （shared-space fairness / replay mixed-gate boundary と public operational CLI installed-binary / packaging success-criteria mixed-gate boundaryは fixed 済み、remaining topics は mixed gate / user-spec-required）
- current self-driven queue:
  `2 package`
- immediate blocker:
  `0`
- current lane を止める user decision:
  `0`
- current authored source sample:
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- current runnable prototype sample:
  `p01 / p02 / p03 / p04 / p05`
  （helper-local `debug_outputs` / `verification_preview` / `artifact_preview` 付き）
- exact rough stimulus preservation bucket:
  `samples/not_implemented/`
- underdeclared source omission actualization:
  `e5 / e12`
  （current source parser / lowerer convenience cut に actualize 済み、helper-local `verification_preview` / `artifact_preview` でも `fixture_static_cluster` route reached）

## macro phase map

この map は repo 全体の大局を示す。
current macro phase reading は、`Macro 0` から `Macro 5` までは closeout まで self-driven に持って行ける読み、`Macro 6` と `Macro 7` は mixed-gate boundary まで、`Macro 8` は user specification 必須、という切り方で読む。
`Macro 0〜8` は repo 全体の **top-level macro axis** を全部並べたものであり、現時点では `Macro 9` 以降を置く想定はない。
後続の課題は「`Macro 8` の先に続く」のではなく、`Macro 0〜7` の深化として戻るか、application / domain realization なら `Macro 8` に入る。
したがって `Macro 8` は「それ以降全部」の catch-all ではなく、**application-specific target / product realization** 専用の phase として読む。
ここでの `rough progress %` は、repo の current-L2 / docs-first / non-production line に scoped した rough estimate であり、最終製品や上位アプリまで含めた全体野心の達成率ではない。

| Macro phase | 主眼 | 現在位置 | rough progress % | 重さ | 自走可否 | 自走 closeout 読み |
|---|---|---|---|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 94% | 低 | 着手可能 | maintenance closeout まで self-driven |
| `Macro 1` | semantic kernel / invariant stabilization | late | 84% | 中 | 着手可能 | narrow semantic reopen の closeout まで self-driven |
| `Macro 2` | parser-free validation substrate | late | 84% | 中 | 着手可能 | current substrate closeout まで self-driven |
| `Macro 3` | compile-ready minimal actualization | late | 80% | 中 | 着手可能 | current tranche closeout まで self-driven |
| `Macro 4` | executable fixed-subset sample expansion | current scoped closeout fixed + prototype second tranche actualized | 98% | 重 | 着手可能 | current fixed-subset widening closeout まで self-driven |
| `Macro 5` | typed / theorem / model-check bridge | current scoped closeout fixed | 80% | 重 | 着手可能 | current boundary / pilot / framing closeout まで self-driven |
| `Macro 6` | fabric / shared-space / runtime evolution | mixed-gate boundary fixed | 47% | 重 | 着手可能（boundaryまで） | docs-first boundary / mixed-gate boundary まで self-driven |
| `Macro 7` | toolchain / backend / host-facing integration | mixed-gate boundary fixed | 39% | 重 | 着手可能（boundaryまで） | thin facade / shell / mixed-gate boundary まで self-driven |
| `Macro 8` | domain / application realization | application-specific target not started | 7% | とても重い | 要仕様確認 | self-driven closeout はしない |

## 特徴機能ごとの到達度

| feature family | 現在地 | できていること | 次の意味ある一歩 |
|---|---|---|---|
| event DAG / local `try` / `atomic_cut` | `S5-S6` | parser-free / source-backed runnable evidence、duplicate pair まで actualized | broader try-rollback malformed-static family は kept-later inventory に保つ |
| guarded option chain / `lease` / monotone degradation | `S6` | missing-option / capability / duplicate pair まで source-authored static-stop widening | broader malformed-static family を hidden promotion なしで reserve に保つ |
| contracts / static gate / formal hook | `S6` | static gate、tool-neutral formal hook、emitted artifact wiring、typed/theorem/model-check bridge floor | stronger typed surface と discharge/public-contract は mixed gate に保つ |
| theorem-side pilot | `S6` | notebook-first pilot、lemma order、lemma wording floor、abstract discharge-entry reserve、transport/public-contract later-gate framing | concrete discharge transport / public theorem contract は mixed gate |
| model-check line | `S6` | row-local carrier、projection reserve inventory、small-cluster projection keep/drop、property-language/tool-binding later-gate framing | settled property language / concrete tool seam は mixed gate |
| typed-core / type system | `S4-S5` | obligation owner matrix、attachment inventory、source-visible first comparison、typed family split、stronger typed-surface threshold framing | stronger typed surface adoption は mixed gate |
| multi-node / shared-space | `S4` at boundary | identity / admission / authority / room-profile / host-binding bridge、fairness/replay mixed-gate boundary | final fairness / replay operational profile は mixed gate / user spec |
| dynamic attach / detach / safe downstream addition | `S2-S3` | DAG discipline、overlay constraints、shared-space boundary notes | operational profile / target-specific attach-detach policy は later |
| host-facing I/O / adapter boundary | `S4-S5` | capability-scoped docs-first boundary、thin facade、Rust shell actualization、installed-binary/packaging success mixed-gate boundary | installed binary promotion / final packaging success criteria は mixed gate |
| ordering / `memory_order` reinterpretation | `S5` | cut-family comparison、relation decomposition、falsifier coverage、property-language bridge、source-surface wording reserve、modality trigger | final property language / source surface / emitted schema / low-level reinterpretation stanceは mixed gate |
| syntax / modality theory line | `S4-S5` | syntax honesty principle、stimuli comparison、modal promotion threshold、guarded-vs-MDTT/MTT reduction timing、modality internalization trigger | final foundation adoption / final syntax marker は mixed gate |
| public dev surface / CLI | `S5` | thin facade、later support、current-L2 scoped Rust shell actualization、installed-binary/packaging mixed-gate boundary | installed binary promotion / hierarchy / success criteria は mixed gate |
| executable sample corpus | `S6` | authored sixteen、runner / ladder / regression helper、artifact fan-out、duplicate pair actualization、underdeclared source omission actualization、prototype runnable quintet、rough-stimulus preservation bucket、helper-local `debug_outputs` / `verification_preview` / `artifact_preview` | corrected prototype third tranche と broader try-rollback malformed-static family comparisonを続ける |

## 層ごとの進捗

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手性 |
|---|---|---|---|---|
| semantic kernel | 92% | 85% | 76% | 着手可能 |
| parser-free substrate | 88% | 78% | 86% | 着手可能 |
| compile-ready minimal actualization | 84% | 73% | 83% | 着手可能 |
| fixed-subset source sample line | 85% | 80% | 85% | 着手可能 |
| typed / theorem / model-check bridge | 84% | 79% | 64% | mixed gate までは整理済み |
| shared-space docs-first boundary | 59% | 50% | 29% | mixed gate までは整理済み |
| host-facing integration boundary | 54% | 47% | 37% | mixed gate までは整理済み |
| ordering / memory-model reinterpretation | 66% | 58% | 17% | mixed gate までは整理済み |
| syntax / modality foundation comparison | 50% | 46% | 6% | mixed gate までは整理済み |
| backend / public packaging | 32% | 29% | 27% | 要仕様確認 |
| application realization | 10% | 8% | 2% | 要仕様確認 |

## twin peaks

### 1. 型システム / 定理証明 / モデル検査

- 位置づけ:
  repo 全体で最も重い研究線のひとつであり、`Macro 5` の主核である。
  current reading は **full 実装を急ぐ線**ではなく、typed-core / theorem / model-check の boundary、pilot、framing、stop line を ratchet 方式で固める線である。
- 現段階:
  `Macro 5 boundary / pilot / framing closeout fixed`
  feature-stage の読みでは、typed-core は `S4-S5`、theorem/model-check boundary は `S6` にあり、bridge 全体としては `mixed-gate promotion pending` 段階にいる。
- rough progress:
  全体 bridge line は `84% / 79% / 64%`（論理仕様 / ユーザ向け仕様 / 実装・運用）。
  内訳の rough read は、`typed-core boundary 約68%`、`theorem pilot boundary 約82%`、`model-check boundary 約76%`。
  ここで内訳は repo の current package 群からの synthesis であり、規範値ではない。
- 採用済みの骨格:
  - typed-core は full calculus first ではなく、`checker-adjacent semantic carrier` を first attachment に置く。
  - source-visible typed line は structural marker family first、request / predicate / `try` cluster は reserve cluster に留める。
  - theorem-side は syntax tree 全体ではなく、semantic-core invariant family を first pilot に置く。
  - notebook-first review pressure と abstract discharge-entry reserve を theorem floor に置く。
  - theorem discharge transport seam と public-contract seam は coupled だが distinct な later gate として扱う。
  - model-check は row-local carrier first、same-subject stage-local small-cluster projection reserve を first keep line に置く。
  - first settled property language と concrete tool seam は adjacent だが distinct な later gate として扱う。
  - final public verifier contract、full strong type system、concrete tool binding はまだ adopted ではない。
- 最初の構想からの整理:
  当初の「強い型 / 証明 / モデル検査を早く入れる」方向は残しているが、
  current repo では
  `full system first` ではなく
  `boundary / pilot / framing / stop line first`
  に整理した。
  その結果、最初の構想は
  - obligation allocation
  - attachment candidate
  - theorem lemma family
  - review artifact と discharge の stop line
  - row-local carrier
  - small-cluster projection
  - later-gate framing
  へ分解されて仕様に取り込まれた。
- いま残っている OPEN:
  - stronger typed surface promotion
  - theorem discharge transport concretization
  - public theorem/public verifier contract
  - model-check first settled property language
  - concrete theorem prover / model-check tool binding
  - full strong type system / final public verifier contract
- 自走範囲:
  current self-driven queue は small reopened state にあり、corrected prototype / sample-visible comparison までは自律的に進められる。
  mixed-gate framing / threshold / stop-line の再整理も引き続き自律的に進められる。
  actual adoption と external binding は mixed gate か user 仕様確認に入る。

### 2. order / fence / atomic / `memory_order` / authority-handoff

- 位置づけ:
  repo 全体で最も重い研究線のもうひとつであり、`Macro 5` theory-lab と `Macro 6` shared-space boundary の接点である。
  current reading は **low-level concurrency API を先に決める線**ではなく、cut / order / visibility / witness / authority / replay を relation decomposition と boundary package で再構成する線である。
- 現段階:
  `Macro 5 boundary / pilot / framing closeout fixed` と `Macro 6/7 mixed-gate boundary fixed` の接続部。
  feature-stage の読みでは、ordering / `memory_order` 再解釈は `S5`、shared-space docs-first boundary は `S4` にあり、current phase は `comparison framework hardening complete, adoption pending` である。
- rough progress:
  `ordering / memory-model reinterpretation` は `66% / 58% / 17%`。
  隣接する `shared-space docs-first boundary` は `59% / 50% / 29%`。
  authority-handoff operational side は shared-space row と密接だが、current snapshot では still mixed-gate reserve を含むため単一 combined percentage は固定しない。
- 採用済みの骨格:
  - `atomic_cut` は current `place` の rollback frontier を確定する local finalizing cut であり、global consistent cut / durable commit / distributed sync point ではない。
  - cut family は `local finalization / ordering-only barrier / snapshot-only observation / durable commit` に分けて比較する。
  - order family は `program_order / dependency_order / publication_order / observation_order / witness_order / finalization_order / scoped_happens_before` を working relation decomposition とする。
  - `authority_serial_transition_family` を first kept candidate、`witness_aware_commit_family` を second candidate に置く。
  - low-level `std::memory_order` / `kill_dependency` family は reference family として保持するが、current source surface には直接輸入しない。
  - snake_case relation family 名を principal wording に保ち、平易な日本語を explanation layer に重ねる。
  - shared-space fairness / replay と public operational CLI packaging は current docs-only boundary を越えず、mixed-gate boundary で止める。
- 最初の構想からの整理:
  当初の「fence / atomic / `memory_order` を再構築したい」という方向は捨てていない。
  ただし current repo では、
  `atomic_cut` を narrow local nucleus に残し、
  higher-level handoff / publication / witness / fairness family を docs-first に比較し、
  adequacy corpus と falsifier loop で破綻候補を落としながら、
  low-level family は backend-aligned reference として後段に残す形へ整理している。
- いま残っている OPEN:
  - final property language
  - final source-surface handoff wording
  - final emitted artifact schema
  - `snapshot_cut` / `consistent_cut` の settled naming
  - low-level `memory_order` reinterpretation の final public stance
  - shared-space final fairness / replay / authority catalog
  - installed binary / packaging success criteria
- 自走範囲:
  docs-first comparison、adequacy corpus hardening、candidate reduction、mixed-gate boundary整理までは `着手可能`。
  fairness operational profile、external protocol/runtime commitment、installed-binary promotion、final source syntax は `mixed gate` か `要仕様確認` に入る。

## research / user split

### promoted self-driven package

- current promoted self-driven package は 2 本である。
  - typed/theorem/model-check sample-visible corrected prototype tranche
  - order/handoff corrected prototype third tranche
- `Macro 0〜5` の current scoped closeout 自体は fixed と読む。

### mixed gate / later reserve

- stronger typed surface の実昇格
- theorem discharge transport / public-contract の具体 contract 化
- model-check first settled property language / concrete tool seam の確定
- shared-space final fairness / replay operational profile
- public operational CLI installed-binary promotion / packaging success criteria

### user が後で決めること

- shared-space final catalog
- first external integration target
- backend / tooling success criteria
- first application target

## recent log

- 注記: この欄は recent log として保つ。詳細は `docs/reports/` を正本にする。
- 2026-04-17 09:34 JST — `docs/reports/0715` で `progress.md` / `tasks.md` / `Documentation.md` / relevant `plan/` / `faq_005.md` / `specs/10` / `specs/00-document-map.md` を current snapshot に揃え、`Macro 5 boundary / pilot / framing closeout fixed` と `Macro 6/7 mixed-gate boundary fixed` の読みへ更新した。
- 2026-04-17 10:50 JST — `docs/reports/0718` で `samples/prototype/` runnable trio、`samples/not_implemented/` rough-stimulus preservation bucket、leading `#` comment support、prototype direct-path runner support、adjacent host-plan auto resolution を追加し、sample bucket policy を `specs/examples/451` と snapshot docs に同期した。
- 2026-04-17 11:35 JST — `docs/reports/0721` で helper-local `debug_outputs` preview cut、`specs/examples/452`、prototype sidecar debug record、`research_abstract` 全面書き直しを追加し、大局 phase の読みを簡潔化した。
- 2026-04-17 12:00 JST — `docs/reports/0722` で helper-local `verification_preview` cut、prototype second tranche `p04/p05`、runtime/static/guarded の sample-visible comparison を追加した。
- 2026-04-17 12:49 JST — `docs/reports/0723` で helper-local `artifact_preview` cut と prototype / sample の theorem-model-check row preview を current CLI summary へ追加した。
- 2026-04-17 13:24 JST — `docs/reports/0724` で `e5/e12` underdeclared omission family を source-authored current-l2 corpusへ actualize し、helper-local `verification_preview` / `artifact_preview` を underdeclared static cluster reached へ widen、authored sixteen inventory と regression bundle へ同期した。
- 2026-04-17 09:34 JST — `docs/reports/0714` で `specs/examples/444...450` を追加し、modality internalization trigger、stronger typed-surface threshold framing、theorem discharge transport/public-contract later-gate framing、model-check property-language/tool-binding later-gate framing、shared-space fairness/replay mixed-gate boundary、public operational CLI installed-binary/packaging mixed-gate boundary、Macro 5 closeout threshold を docs-first に固定した。
- 2026-04-17 09:12 JST — `docs/reports/0713` で `e14/e15` duplicate pair を source-authored static-stop pair として actualize し、runner / ladder / emitted artifact wiring / regression inventory / sample docs を authored fourteen へ widen したうえで、execution lane を `Macro 0〜4 closeout fixed` に更新した。
- 2026-04-17 08:41 JST — `docs/reports/0712` で `macro phase map` に rough progress `%` 列を追加し、`Macro 0〜8` が repo 全体の top-level axis であり `Macro 8` は application-specific realization 専用であることを明記したうえで、repo-scoped progress estimate を再配置した。
- 2026-04-17 08:00 JST — `docs/reports/0710` で macro phase map を見直し、`Macro 0〜5` は closeout まで self-driven、`Macro 6〜7` は boundary-prep まで、`Macro 8` は user-spec-required だと読み分けたうえで、`tasks.md` に current self-driven macro-phase closeout reading を追加した。
- 2026-04-17 07:22 JST — `docs/reports/0708` で `specs/examples/441...442` を追加し、model-check small-cluster projection keep/drop と order/handoff source-surface wording reserve を docs-first に固定したうえで、`faq_005.md` を literature mapping と dice-owner handoff explanation 付きの current explanation へ更新した。
- 2026-04-17 00:28 JST — `docs/reports/0707` で `specs/examples/439...440` を追加し、typed family split と notebook-consumer threshold / discharge reserve を docs-first に固定した。
- 2026-04-17 00:04 JST — `docs/reports/0706` で `specs/examples/433...438` を追加し、typed/theorem/model-check second-order reserve、order/handoff emitted-artifact schema reserve、guarded-vs-MDTT/MTT reduction timing、duplicate-cluster next widening cut を docs-first に固定した。
- 2026-04-16 23:42 JST — `docs/reports/0705` で `specs/examples/431...432` を追加し、public operational CLI packaging reserve note と shared-space fairness / replay strengthening reserve note を docs-first に固定した。
- 2026-04-16 23:20 JST — `docs/reports/0704` で twin peaks 追跡用の節を `progress.md` に追加し、typed/theorem/model-check 線と order/memory-order/authority-handoff 線について、現段階・rough progress・source-backed floor・OPEN・次の research package を分離して snapshot 化した。
