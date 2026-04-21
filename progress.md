# progress

最終更新: 2026-04-21 13:46 JST

## この文書について

- この文書は repo 全体の **薄い進捗 snapshot** である。
- 規範判断の正本は `specs/`、長期参照は `plan/`、詳細経緯と実行証跡は `docs/reports/` に置く。
- 進捗率は **rough estimate** であり、current-L2 / fixed-subset / non-production / repo-local floor に scoped した読みである。
- この文書では「先の package chain」を並べず、**次にどの gate / band が見えているか** を mirror する。
- immediate な実行単位、entry command、rough estimate は `tasks.md` が正本である。

## current snapshot

- execution lane:
  `Macro 4 active on fixed authored/prototype floor`
  authored sixteen と corrected prototype set `p01 ... p16` は runnable
- theory-lab lane:
  `Macro 5 repo-local near-end`
  typed / theorem / model-check / order-handoff representative bundle、Lean foundation、reserve summary index は揃っている
- reserve integration lane:
  `Macro 6 minimal working subset default / Macro 7 mixed`
  authoritative-room default と reserve package summary はあるが、final public contract / packaging は later
- historical recovery:
  `旧資料_参考_ChatGPT_03_sync_v3/` から回収した concern は `plan/10` / `plan/12` / `plan/13` / `plan/18` に owner 別で再配置済み
  current numbered self-driven queue は reopen していない

## practical reading

- **動くもの**
  - current-L2 sample runner / CLI
  - Problem 1 representative bundle
  - Problem 2 representative bundle
  - Lean foundation proof fragment
  - generated Lean stub corpus acceptance
- **まだ later のもの**
  - final public parser / checker / runtime API
  - stronger typed source principal / full strong typed surface
  - concrete theorem / model-check production binding
  - final public verifier contract
  - low-level `memory_order` exact source surface
  - final witness / provider / emitted-artifact public contract
  - packaging / installed binary / FFI / engine adapter

## reproducibility anchors

- representative bundle:
  `python3 scripts/current_l2_guided_samples.py smoke-all --format json`
  で `problem1` / `problem2` の aggregate smoke を再確認できる
- Problem 1 typed line:
  `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample ...p10...`
  は success、
  `...p11...` / `...p12...` / `...p15...` / `...p16...`
  は Reject を返す
- Problem 1 theorem / model-check reserve:
  `python3 scripts/current_l2_guided_samples.py emit-theorem problem1`
  と
  `python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line`
  で repo-local artifact / reserve summary を再生成できる
- Problem 2 order / handoff line:
  `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample ...p07...`
  は success、
  `...p13...` / `...p14...`
  は static stop になる
- Problem 2 scenario / reserve:
  `python3 scripts/current_l2_guided_samples.py emit-scenario problem2`
  で `p07 / p08 / p09 / p13 / p14` の scenario summary を materialize できる
- Lean:
  `source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  と
  `...CurrentL2FiniteIndexFirstLayer.lean`
  は success
- Lean sync:
  `source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py`
  は foundation success、generated stub success with `warning: declaration uses 'sorry'`
- unit test:
  `python3 -m unittest scripts/tests/test_current_l2_lean_sample_sync.py`
  は `Ran 9 tests ... OK`

## current self-driven horizon

| band | macro | topic family | current reading | deeper detail owner |
|---|---|---|---|---|
| immediate self-driven | `Macro 0` | snapshot / traceability maintenance | `README.md` / `Documentation.md` / `progress.md` / `tasks.md` / `plan/01` の drift suppression | `tasks.md` |
| immediate self-driven | `Macro 5` | Problem 1 public-seam residual cluster | typed / theorem / model-check の residual boundary を current first reopen band として narrow に読む | `tasks.md` + `plan/18` |
| immediate self-driven | `Macro 6` | Problem 2 public-seam residual cluster | source wording / emitted schema / witness-provider public-shape residual を分離したまま narrow に読む | `tasks.md` + `plan/18` |
| immediate self-driven | `Macro 3 / 7` | parser-side / public-candidate residual cluster | parser companion surface、request/head/clause carrier、public parser/checker/runtime API residual を current reopen band として読む | `tasks.md` + `plan/11` |
| immediate self-driven | `Macro 5 / 6` | reserve summary upkeep | theorem-first external pilot、auditable-authority-witness、delegated-rng-service、model-check second-line を reserve summary band として保つ | `tasks.md` + `plan/13` + `plan/18` |

## future gate map

| band | macro focus | topic families | promotion 条件の読み | owner |
|---|---|---|---|---|
| immediate | `Macro 0 / 3 / 5 / 6 / 7` | summary drift suppression、public-seam residual cluster、parser-side/public-candidate residual、reserve summary upkeep | existing bundle / reserve / helper entrypoint を使い、sample corpus widening ではなく gate 整理を優先する | `tasks.md` |
| mid mixed gate | `Macro 5 / 6 / 7` | theorem result object / payload public contract、model-check property/tool seam、source wording / emitted schema、witness-provider public shape、syntax / modality final marker | final public contract を決める直前まで compare floor / helper-local actualization / residual split を narrow に保つ | `plan/12` + `plan/18` |
| reserve | `Macro 5 / 6 / 7` | theorem-first external pilot、auditable-authority-witness、delegated-rng-service、model-check second-line、proof promotion ladder memory | reserve summary / retained concern として保持し、mainline へ hidden promotion しない | `plan/01` + `plan/13` + `plan/18` |
| heavy future | `Macro 5 / 6 / 7 / 8` | full strong type system、production theorem/model-check、runtime finalization、operational shared-space、trust/audit/registry、backend/codegen、dynamic evolution choreography | current line から切り離した heavy workstream として読む | `plan/13` |
| user-spec hold line | `Macro 7 / 8` | packaging、installed binary、FFI / engine adapter、concrete application target、acceptance criteria、shared-space final catalog | user の目的・保証範囲・target profile が揃うまで本格昇格しない | `tasks.md` + `plan/13` |

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | next band | 先読み | 自走可否 |
|---|---|---:|---:|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 95% | immediate | snapshot / report / task map の drift suppression を継続する | 着手可能 |
| `Macro 1` | semantic kernel / invariant stabilization | late | 84% | mid / reserve | current invariant は揃っているが、cut-family / evolution / compatibility wording の hardening は残る | 着手可能 |
| `Macro 2` | parser-free validation substrate | late | 85% | immediate / mid | detached loop / fixture corpus / compare helper を崩さず保つ | 着手可能 |
| `Macro 3` | compile-ready minimal actualization | late | 81% | immediate | parser-side residual と public-candidate boundary の整理が next line | 着手可能 |
| `Macro 4` | executable fixed-subset sample floor | active on fixed floor | 94% | immediate maintenance | sample wideningより docs / regression / falsifier comparison を優先する | 着手可能 |
| `Macro 5` | typed / theorem / model-check / verifier bridge | repo-local near-end | 96% | immediate -> mid mixed gate | final-public seam は残るが、current first line / reserve line / stop line は source-backed に揃っている | 自走可能（public contract 以外） |
| `Macro 6` | shared-space / fabric / runtime evolution boundary | default fixed + reserve split | 75% | immediate -> mid mixed gate -> heavy runtime stressor | authoritative-room default はあるが、catalog / fairness / portal / multi-world は later | 自走可能（final profile 以外） |
| `Macro 7` | toolchain / backend / host-facing integration | mixed | 53% | immediate parser-side -> mid mixed gate -> user-spec hold | shell / facade / packaging boundary を保ちながら residual を読む段階 | repo-local まで |
| `Macro 8` | broader application realization | early | 18% | heavy future / user-spec hold | benchmark family は見えているが、concrete target と acceptance criteria は未確定 | 要仕様確認 |

## feature family snapshot

| feature family | 現在地 | できていること | next band | stop line |
|---|---|---|---|---|
| executable sample corpus | `S6` | authored sixteen、prototype set `p01 ... p16`、problem bundle smoke、negative static-stop pair | immediate maintenance | final public runtime surface |
| robustness via contracts / theorem / model-check boundary | `S6` | typed rejection pair、theorem-first emitted artifact loop、model-check reserve summary、Lean foundation | immediate / mid mixed gate | final public theorem / checker / verifier contract |
| multi-node / fabric / shared-space | `S4-S5` | authoritative-room first default、late-join / reconnect representative pair、reserve strengthening lane | immediate / heavy runtime stressor | final shared-space catalog / multi-world operational profile |
| dynamic attach / detach / DAG-safe evolution | `S3` | downstream-addition invariant、patch / lease / fallback concern の docs-first boundary、historical concern owner split | reserve / heavy future | concrete migration / finalization / revocation choreography |
| `atomic_cut` と higher-level ordering / `memory_order` family | `S4-S5` | place-local cut nucleus、relation decomposition、authority-serial family、retained-later memory-order reference | immediate / mid mixed gate / reserve | low-level exact surface / runtime semantics finalization |
| Lean foundations / proof spine | `S6` | self-contained small proofs、reusable lemma groups、guide-aligned explanations、generated stub acceptance | immediate maintenance / reserve proof hardening | final mechanized public theory |
| Typed-Effect Wiring / host integration | `S2-S3` | docs-first boundary、repo-local helper / artifact loop、host-facing residual inventory | mid mixed gate / user-spec hold | installed binary / FFI / engine adapter |
| PrismCascade / domain-kernel separation | `S1-S2` | separable subsystem boundary を specs で固定 | reserve / user-spec hold | domain-specific kernel actualization |
| broader application / benchmark family | `S0-S1` | benchmark family catalog と user-spec hold line を保持 | heavy future / user-spec hold | concrete target と acceptance criteria 合意 |

## layer / track progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | next milestone | 着手性 |
|---|---:|---:|---:|---|---|
| Mir core / current-L2 | 92% | 87% | 76% | invariant / boundary wording と public-candidate residual の整理 | 着手可能 |
| Mirrorea / fabric / shared-space | 76% | 71% | 54% | authoritative-room default と later catalog / multi-world stressor の分離維持 | final profile 以外は自走可能 |
| Typed-Effect Wiring Platform / host integration | 41% | 34% | 28% | host-facing residual と packaging residual を hidden promotion なしで整理 | repo-local まで |
| PrismCascade / domain-kernel separation | 46% | 29% | 12% | separable kernel boundary を維持しつつ actual target をまだ固定しない | 後段依存 |
| contracts / theorem / model-check bridge | 95% | 95% | 87% | Problem 1 final-public-seam cluster を narrow に reopen | final public seam 以外は自走可能 |
| ordering / handoff / cut-family boundary | 89% | 93% | 73% | Problem 2 residual cluster と `memory_order` retained-later line を分離維持 | final public wording / contract 以外は自走可能 |
| Lean foundations / proof spine | 86% | 90% | 58% | relation library / artifact conformance / compare floor の hardening | repo-local mechanization までは自走可能 |
| upper application / benchmark families | 23% | 21% | 8% | benchmark family から first concrete target を user と絞る | 要仕様確認 |

## twin peaks

### Problem 1 — 型システム / 定理証明 / モデル検査

- current first line:
  checker-adjacent first strong typing layer、notebook-first theorem line、row-local model-check carrier first
- immediate:
  `problem1-final-public-seams` を入口に、typed residual / theorem public-contract residual / model-check public-contract residual の reopen order を narrow に保つ
- mid mixed gate:
  theorem result object、consumer-shaped payload public contract、public checker artifact、verifier handoff、concrete tool seam を compare floor / residual split で整理する
- reserve / heavy:
  proof promotion ladder memory、production theorem proving、production model checking、full strong type system は `plan/13` / `plan/18` 側で読む

### Problem 2 — order / handoff / authoritative-room

- current first line:
  relation decomposition principal、authoritative-room first default、reserve strengthening split、negative static-stop pair
- immediate:
  `problem2-final-public-seams` を入口に、source wording / emitted schema と witness-provider public-shape residual を分離したまま再整理する
- mid mixed gate:
  final source wording、final emitted schema、final witness/provider public shape、syntax / modality final marker、parser-side reconnect を narrow に読む
- reserve / heavy:
  low-level `memory_order` exact surface、portal / multi-world / fairness / replay catalog、dynamic evolution choreography は retained-later / heavy future に留める

## owner map for deeper detail

| concern | primary owner |
|---|---|
| immediate executable task package / entry command / rough estimate | `tasks.md` |
| current status memory / reopen entrypoint snapshot | `plan/01-status-at-a-glance.md` |
| mixed-gate / risk / reopen trigger | `plan/12-open-problems-and-risks.md` |
| heavy future / operational / backend / application family | `plan/13-heavy-future-workstreams.md` |
| typed / theorem / model-check / order / syntax / modality theory detail | `plan/18-type-proof-modelcheck-and-ordering-research-program.md` |
| progress/tasks の軸 | `.docs/progress-task-axes.md` |

## research を通して見つけること

| topic | macro | current reading |
|---|---|---|
| low-level `memory_order` reinterpretation | `Macro 6` | retained-later reference family のままにし、high-level order / handoff line を先に維持する |
| concrete theorem / model-check tool binding | `Macro 5` | Lean-first bridge / reserve summary を維持し、public contract 化は later |
| final public verifier contract | `Macro 5 / 7` | helper preview / reserve summary を final shared contract に統合するかは still later |
| portal / multi-world / handoff stressor family | `Macro 6` | authoritative-room first default の後段 stressor として heavy line に残す |
| solver / projection / artifact-trace scaling | `Macro 5` | compare floor を超える widening 圧力が出たときに reopen する |

## user が決める必要があること

| topic | macro | current recommendation |
|---|---|---|
| broader application target | `Macro 8` | authoritative-room 延長、Typed-Effect / host integration、Prism / 上位アプリのどれを first target にするかを user 側で固定したい |
| packaging / installed binary / FFI | `Macro 7` | まだ repo-local helper floor を維持する |
| shared-space final catalog | `Macro 6 / 8` | exhaustive catalog ではなく minimal working subset を先に保つ |
| acceptance criteria for benchmark family | `Macro 8` | benchmark family はあるが committed product target は未決 |

## next reading

- concise snapshot: `Documentation.md`
- current task map: `tasks.md`
- long-lived status memory: `plan/01-status-at-a-glance.md`
- mixed-gate / risk / reopen trigger: `plan/12-open-problems-and-risks.md`
- heavy future / application family: `plan/13-heavy-future-workstreams.md`
- theory-side detail: `plan/18-type-proof-modelcheck-and-ordering-research-program.md`

## recent log

- 2026-04-21 00:48 JST — Problem 1 / Problem 2 representative bundle、typed rejection pair、order-handoff negative pair、Lean foundation、generated Lean stub corpus を再実行し、summary docs を `repo-local near-end` と `final public stop line` を分けた記述へ全面更新した。
- 2026-04-21 01:21 JST — Lean foundation に reusable lemma 群を追加して `python3 scripts/current_l2_lean_sample_sync.py` と unit test を通し、Problem 1 / Problem 2 の beginner 向け guide `docs/research_abstract/static_analysis_01.md` / `order_01.md` を追加した。
- 2026-04-21 01:36 JST — Lean beginner guide `docs/research_abstract/lean_01.md` を追加し、label-model / proof-skeleton の補題を少量補強したうえで Lean 実行・standalone success/error 例・unit test・docs validation を再確認した。
- 2026-04-21 09:30 JST — `static_analysis_01_detail.md` / `order_01_detail.md` / `lean_01_detail.md` を追加し、Problem 1 / Problem 2 / Lean foundation のコード全文・行単位解説・再現コマンド・出力の読み方を standalone な詳細版として整理した。
- 2026-04-21 09:56 JST — reviewer 指摘を取り込み、`docs/research_abstract/README.md` の stale snapshot を現行 status に合わせ、report `0898` を repo ルール準拠の章立てと exact command / output excerpt に修正した。
- 2026-04-21 13:11 JST — 旧 `sync_v3` 資料の回収価値がある論点を `plan/10` / `plan/12` / `plan/13` / `plan/18` に owner 別で再配置し、historical scope-collapse と requirement amnesia を避ける方針を明示した。current numbered self-driven queue は reopen していない。
- 2026-04-21 13:46 JST — `progress.md` を reviewer 指摘に合わせて再圧縮し、task-map と同型な記述を弱めたうえで subsystem / feature family 分解を補った。`tasks.md` も rough next order を軽く同期した。
