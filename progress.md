# progress

最終更新: 2026-04-13 16:28 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- ここに書く進捗率は **rough estimate** であり、問題が見つかれば巻き戻る。
- 高い進捗率は、特に断りがない限り **current-L2 fixed-subset / non-production line に scoped した reading** である。
- この文書では、old `Phase 1..7` checkpoint label に加えて、`.docs/progress-task-axes.md` で定義した
  - **macro phase**
  - **feature maturity stage**
  の 2 軸で大局を整理する。

## 現在の capability snapshot

- legacy checkpoint としては、Phase 1〜5 closeout / freeze は fixed 済みである。
- legacy Phase 6 front-half compile-ready checkpoint も fixed 済みであり、`mir-ast` / `mir-semantics` / `mir-runtime` の narrow actual path、tool-neutral formal hook、proof-notebook first pilot、plain bridge sketch first actualization、compare-ready bridge sketch second reopen までは source-backed である。
- parser-free current L2 PoC は runnable であり、compile / test / detached-loop baseline がある。
- syntax-backed fixed-subset sample path も runnable であり、現在 authored 済みなのは
  - `e1`
  - `e2`
  - `e3`
  - `e4`
  - `e19`
  - `e21`
  - `e22`
  - `e23`
  の octet である。
- `e3` は source row / runner / inventory / ladder まで actualize 済みであり、`admit-miss` を non-admissible skip に留めた runtime success まで reached している。一方で current theorem-side consumer `proof_notebook_review_unit` と current formal-hook top `runtime_try_cut_cluster` はそのまま保ち、formal hook stage は `not reached (guarded)` に留めている。
- tool-neutral formal hook から `proof_notebook_review_unit` を作る first theorem-side pilot と、`comparison_basis_refs` までを持つ compare-ready bridge sketch second reopen はあるが、bless / review-session metadata、concrete theorem / model-check binding はまだ later である。
- second source-sample cluster sequencing と actual `e22` contrast-row source actualization も fixed 済みであり、first post-sextet runtime contrast pair は `e21` / `e22` に置く。`e22` は source row / runner / regression inventory / verification ladder / runtime formal-hook smoke まで actualize 済みである。
- stable static malformed post-contrast sequencing も fixed 済みであり、second broader cluster は stable reason-code / fixture-static cluster に置いている。
- stable-static edge-pair first reopen も fixed 済みであり、existing `e4` row と deferred `e19` row を source-backed static-stop pair へ actualize した。`e4` / `e19` は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` に揃っている。
- parser / checker / runtime public surface inventory も fixed 済みであり、already-public parser-free helper stack、crate-public but non-production compile-ready tranche、repo-local helper / example emitter surface の 3 bucket split を current reading に置く。
- public operational surface actualization gate も fixed 済みであり、already-public parser-free stack を stable bucket に据えたまま、later public pressure の first docs-only candidate を `run_current_l2_source_sample` に narrow に置き、`run_current_l2_runtime_skeleton` / `lower_current_l2_fixed_source_text` は tranche-internal support、`resolve_current_l2_source_sample_path` と repo-local script/example surface は excluded bucket に留めている。
- Mirrorea / shared-space docs-first re-entry bundle も fixed 済みであり、`mirrorea_fabric_boundary + shared_space_practical_boundary` を current boundary core、Typed-Effect / Prism を adjacent track、shared-space final catalog と upper-layer app target を user-spec-required gate に置く current cut を採った。
- shared-space identity / auth layering reopen も fixed 済みであり、membership identity core を `member_ref + principal_ref + member_incarnation + activation_state` に narrow に残しつつ、transport/service auth、room admission、display/projection identity は side carriers に押し分ける current cut を採った。
- shared-space admission / compile-time visibility reopen も fixed 済みであり、role / capability / visibility / notify path requirement の over-approximation だけを compile-time に残し、actual admission / activation / active member set / reconciliation は runtime control-plane に残す current cut を採った。
- shared-space authority / resource ownership reopen も fixed 済みであり、participant carrier を membership / activation に留めたまま、resource owner slot、delegated capability、consistency mode、fairness source を separate family に置き、authority placement は `single room authority` first、`replicated authority` next candidate、`relaxed projection authority` future comparison に残す current cut を採った。
- model-check / public-checker second reserve inventory も fixed 済みであり、current first concrete carrier は `proof_notebook_review_unit` のまま維持し、model-check concrete carrier と public-checker actual migration は second reserve / kept-later に押し分けた。
- model-check concrete carrier first actualization gate も fixed 済みであり、`proof_notebook_review_unit` current first pilot を保ったまま、`tool_neutral_formal_hook_only_input + compare_ready_docs_only_bridge_sketch` を entry にする narrow gate を current first choice に昇格した。public-checker docs-only chain は parallel reserve に留め、model-check concrete carrier actualization、actual public-checker migration、actual emitted verifier handoff artifact、concrete tool binding は still later に残している。
- model-check concrete carrier actualization comparison も fixed 済みであり、sample-visible theorem/model-check line の next reopen order は actual model-check carrier first、source-sample emitted verification artifact wiring second、sample-facing theorem/model-check evidence summary and bless/review flow third に固定した。
- model-check concrete carrier first actualization も fixed 済みであり、tool-neutral formal hook only hard input から `schema_version + artifact_kind + subject_kind + subject_ref + case(obligation_kind + evidence_refs)` の row-local machine-facing carrier list を actualize した。`proof_notebook_review_unit` current first theorem-side pilot は維持し、source-sample emitted verification artifact wiring を next package に送っている。
- source-sample emitted verification artifact wiring も fixed 済みであり、`run_current_l2_source_sample` と `CurrentL2SourceSampleRunReport` の public/report shape を変えず、runtime test/support helper-local route として `source report -> formal hook reached/guarded split -> proof_notebook_review_units / model_check_concrete_carriers` fan-out を actualize した。runtime row / static row は reached route を持ち、`e3` は `GuardedNotReached` + empty followup artifact list のまま維持している。
- stable malformed broader follow-up inventory も fixed 済みであり、broader stable malformed next reopen order は missing-option family first (`e16/e17/e18`)、capability family second (`e13/e20`) に置き、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は kept-later に残している。
- public operational CLI / final public contract later gate も fixed 済みであり、`run_current_l2_source_sample` current gate を保ったまま、first later gate を final public parser / checker / runtime API、public operational CLI を second later gate に置き、repo layout / accepted-set / repo-local helper surface は current final contract の外に残している。
- next near-term path は、sample-facing theorem/model-check evidence summary and bless/review flow を current line に置き、その次段で `docs-first I/O / host-facing port boundary`（working label）を整理する line に置く。
- final parser grammar、final public parser / checker / runtime API、LLVM-family backend、Mirrorea operational runtime、shared-space final catalog はまだ無い。

## Macro phase map

| Macro phase | rough % | 主眼 | 現在位置 | 自走可否 | 補足 |
|---|---:|---|---|---|---|
| `Macro 0` | 95% | repository memory / docs / traceability | maintenance | 自走可能 | report / snapshot / plan / FAQ の整流ルールはかなり安定 |
| `Macro 1` | 94% | semantic kernel / invariant stabilization | late | 自走可能 | current L2 semantics / invariant bridge はかなり安定 |
| `Macro 2` | 97% | parser-free validation substrate | late | 自走可能 | parser-free PoC と detached validation loop は runnable |
| `Macro 3` | 93% | compile-ready minimal actualization | late | 自走可能 | parser / checker / runtime / formal-hook の non-production minimal cut と public-pressure gate は揃った |
| `Macro 4` | 84% | executable fixed-subset sample expansion | active | 自走可能 | current authored octet runnable。next malformed widening は reserve に下げつつ保持 |
| `Macro 5` | 68% | static reasoning / theorem / model-check bridge | early-active | 一部自走可能 | proof notebook review-unit current cut、first actualization gate、actualization sequencing comparison、first actual carrier、emitted artifact wiring は fixed 済み。current line は sample-facing summary |
| `Macro 6` | 46% | distributed fabric / shared-space / runtime evolution | docs-first checkpoint fixed | 境界までは自走可能 | Mirrorea/shared-space re-entry bundle、identity/auth layering、admission/compile-time visibility、authority/resource ownership split は fixed。final catalog と operational realizationは later |
| `Macro 7` | 30% | toolchain / backend / developer surface | later-gate order fixed | 一部自走可能 | public operational surface inventory と library-before-CLI later orderingは fixed。nextは docs-first I/O / adapter boundary |
| `Macro 8` | 5% | domain / application realization | not started | 要仕様確認 | 上位アプリの concrete goal が未確定 |

## Feature maturity stage legend

| Stage | 意味 |
|---|---|
| `S0` | 要求探索 |
| `S1` | 理論骨格 |
| `S2` | 論理学 / 計算機科学 / invariant 整理 |
| `S3` | current spec / boundary convergence |
| `S4` | narrow implementation |
| `S5` | executable / validation ratchet |
| `S6` | sample / human docs integration |

## 特徴機能ごとの進捗

| Feature family | current stage | 現在できていること | 主な残件 | 進め方 |
|---|---|---|---|---|
| `event DAG` / `place` / local `try` / `atomic_cut` | `S5-S6` | fixed subset では runnable。parser-free fixture と source sample の双方で evidence がある | `durable_cut`、distributed finalization、final syntax | 自走可能 |
| guarded option chain / `lease` / monotone degradation | `S5-S6` | semantics / invariant / fixture baseline は厚く、`e3` source-authored row まで runtime success で通る | full syntax / algebra、`Approximate` / `Compensate`、formal-hook widening beyond current guard | 自走可能 |
| parser-free validation loop | `S6` | interpreter / host harness / bundle / batch / detached compare loop が runnable | public exporter / retention policy / detached serialization | 自走可能 |
| parser / checker / runtime actualization | `S4-S5` | `mir-ast` / `mir-semantics` / `mir-runtime` の narrow non-production tranche が compile-ready で、public-pressure first gate も fixed 済み | final grammar、final public contract、richer host interface actualization | 自走可能 |
| source-backed sample corpus / verification ladder | `S5-S6` | current authored octet `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` が ladder に乗る。`e21/e22` runtime contrast pair と `e4/e19` static-stop pair は source-backed で閉じ、next line は emitted verification artifact wiring に進んだ | sample-facing evidence summary、missing-option widen reserve | 自走可能 |
| contracts / static gate / formal hook / proof notebook first bridge | `S5-S6` | tool-neutral formal hook、review-unit pilot、plain bridge sketch first actualization、compare-ready bridge sketch second reopen、model-check/public-checker second reserve inventory、model-check concrete carrier first actualization gate、actualization sequencing comparison、first actual model-check carrier、source-sample emitted route がある | sample-facing theorem/model-check evidence summary、bless/review metadata、public checker actual migration | 自走可能 |
| public operational surface | `S3-S4` | inventory 3 bucket split、first docs-only candidate、library-before-CLI later orderingが fixed 済み | final public parser / checker / runtime API actualization、public CLI second gate、layout/host-plan decoupling | 一部自走可能 |
| host-facing I/O / adapter / visualizer / FFI / engine boundary | `S1-S2` | Typed-Effect Wiring Platform、`richer host interface` risk、`C ABI / engine adapters` guidance という anchor はある | docs-first capability-scoped port boundary、visualizer/substrate adapter cut、FFI / engine adapter sequencing | docs-first boundary までは自走可能 |
| multi-node / routing / overlay / safe downstream addition | `S2-S3` | semantic constraints と docs-first boundary はある | route proof、suspended task / patch interaction、operational realization | 境界までは自走可能 |
| shared-space / membership / authority / fairness | `S3-S4` for docs-first boundary, `S1-S2` for runtime realization | authoritative room baseline、control-plane threshold、Mirrorea/shared-space re-entry bundle、identity/auth layering cut、admission/compile-time visibility cut、authority/resource ownership splitがある | final activation / authority / auth / identity / admission / consistency / fairness catalog | user spec required beyond boundary |
| type system / theorem prover / model checker full line | `S1-S2` | boundary inventory と first theorem-side pilot はある | sample-visible narrow type/contract surface、concrete prover/model-check binding、full stronger type system | heavy self-driven + later user goals |
| async-control / memory-order reconstruction | `S1` | `atomic_cut` local cut は source-backed | higher-level ordering / fairness / memory-order-like surface | heavy future research |
| backend / public dev surface / operational CLI | `S0-S1` | Rust-heavy core direction と public-pressure gate はある | backend IR bridge、public operational CLI、LSP / editor surface | later inventory |

## 層 / subsystem ごとの現在地

| Layer / subsystem | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在位置 | 補足 |
|---|---:|---:|---:|---|---|
| Mir core semantics | 93% | 89% | 81% | late | current L2 semantic floor はかなり安定 |
| parser-free PoC / detached loop | 95% | 91% | 99% | late | runnable baseline と regression policy がある |
| compile-ready parser / checker / runtime | 88% | 78% | 85% | late | non-production minimal tranche と public-pressure gate は揃った |
| fixed-subset source samples | 93% | 94% | 93% | active | authored octet runnable。`e21/e22` contrast row と `e4/e19` static pair まで source-backed に actualize 済み |
| theorem / verifier bridge | 89% | 84% | 66% | early-active | formal hook、proof notebook first bridge、first actual model-check carrier、source-sample emitted route はある。nextは sample-facing summary |
| Mirrorea fabric boundary | 71% | 60% | 10% | docs-first checkpoint fixed | re-entry bundle、identity/auth layering、admission/compile-time visibility cut、authority/resource ownership split は fixed。final catalog と operational realizationは later |
| shared-space boundary | 79% | 68% | 16% | docs-first checkpoint fixed | practical cut、repo-level re-entry bundle、admission/visibility bridge、authority/resource split は fixed。final catalog は user spec required |
| Typed-Effect Wiring Platform | 35% | 27% | 4% | positioning only | host-facing I/O / adapter boundary の high-level anchor として参照できる |
| PrismCascade | 28% | 20% | 3% | positioning only | positioning / separation が主で、actual kernel work は later |
| domain / application layer | 12% | 9% | 1% | not started | target application と success criteria が未確定 |

## 現在の self-driven line

1. **Macro 5 / sample-facing theorem/model-check evidence summary and bless/review flow**
   - sample code から見える theorem/model-check evidence と current bless/review flow を narrow に揃える。
2. **Macro 7 / docs-first I/O / host-facing port boundary**
   - capability-scoped port / adapter boundary を first docs-only cut として整理する。
3. **Macro 4 / stable malformed missing-option first reopen**
   - sample-visible milestone の後段 reserve として missing-option family widen を進める。
4. **Macro 7 / final public parser/checker/runtime first later gate**
   - library-before-CLI later orderingを保ったまま final public library contract actualization を後段 reserve として進める。

## 研究で見つけることと、user が決めること

### 研究で見つけること

- docs-first I/O / host-facing port boundary の最小 adapter cut
- stable malformed missing-option first reopen の最小 actualization cut
- final public parser / checker / runtime API first later gate の最小 actualization cut

### user が決めること

- shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog
- upper-layer application の concrete goal と success criteria
- backend / editor / operational tooling の final success criteria
- visualizer / host substrate / FFI / game engine adapter の first target profile

### current reading

- current immediate mainline を止める **user decision は 0 件** と読んでよい。
- current blocker は主に research-discovery 側であり、shared-space / app / integration 側の user decision は later gate にある。

## 実装言語の current split

- **Rust**
  - parser carrier
  - parser-free interpreter
  - static gate
  - runtime thin bridge
  - source lowerer / sample runner
  - formal hook / review-unit / model-check-carrier emitters
- **Python**
  - docs validation
  - report scaffolding
  - detached-loop orchestration
  - source-sample regression orchestration
  - compare/checker helper

current reading は、**Rust-heavy core + mixed-tool helper workflow** である。
「repo 全体を最終的に全部 Rust へ寄せる」とまでは、現時点の docs からは読まない。

## recent log

- 注記: この欄は recent log として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-13 16:28 JST — `docs/reports/0677` と `specs/examples/381...382` で source-sample emitted verification artifact wiring を閉じ、`run_current_l2_source_sample` の public/report shape を変えずに runtime test/support helper-local emitted route と `proof_notebook_review_unit` / model-check carrier fan-out を actualize した。repo-level current line は sample-facing theorem/model-check evidence summary and bless/review flow に進んだ。
- 2026-04-13 16:10 JST — `docs/reports/0676` で post-model-check-carrier document consistency audit を閉じ、`Documentation.md`、`plan/01`、Phase 6 abstract、`faq_003.md` の stale wording を reviewer finding に従って修正した。repo-level current line は source-sample emitted verification artifact wiring のまま維持した。
- 2026-04-13 15:53 JST — `docs/reports/0675` と `specs/examples/379...380` で model-check concrete carrier first actualization を閉じ、tool-neutral formal hook only hard input から row-local machine-facing carrier list を actualize した。repo-level current line は source-sample emitted verification artifact wiring に進んだ。
- 2026-04-13 15:35 JST — `docs/reports/0674` と `specs/examples/377...378` で model-check concrete carrier actualization comparison を閉じ、sample-visible theorem/model-check line の順序を actual carrier first / artifact wiring second / sample-facing summary third に固定した。repo-level current line は model-check concrete carrier first actualization に進んだ。
- 2026-04-13 15:10 JST — `docs/reports/0672` で post-package document consistency audit を閉じ、reviewer finding は empty audit report の completion のみに narrow だったことを確認した。`specs/examples/375...376` close 後の snapshot docs は current line `model-check concrete carrier actualization comparison` に整合している。
- 2026-04-13 15:00 JST — `docs/reports/0673` と `specs/examples/375...376` で shared-space authority / resource ownership reopen を閉じ、participant carrier を membership / activation に留めたまま、resource owner slot、delegated capability、consistency mode、fairness source を separate family に置く current cut を fixed した。repo-level current line は model-check concrete carrier actualization comparison に進んだ。
- 2026-04-13 14:31 JST — `docs/reports/0671` と `specs/examples/373...374` で shared-space admission / compile-time visibility reopen を閉じ、role / capability / visibility / notify path requirement の over-approximationだけを compile-time に残し、actual admission / activation / active member set / reconciliation は runtime control-plane に残す current cut を fixed した。repo-level current line は shared-space authority / resource ownership reopen に進んだ。
- 2026-04-13 09:09 JST — `docs/reports/0669` で post-later-gates document consistency audit を閉じ、`.docs/current-l2-source-sample-authoring-policy.md`、`samples/current-l2/README.md`、`faq_003.md`、Phase 5 abstract の stale current-line wording を current snapshot に揃えた。reviewer 再確認は `No additional findings.` で、repo-level current line は shared-space admission / compile-time visibility reopen のまま維持した。
- 2026-04-13 09:00 JST — `docs/reports/0668` と `specs/examples/371...372` で public operational CLI / final public contract later gate を閉じ、public-side later ordering を final public parser/checker/runtime API first / public operational CLI second に固定した。repo-level current line は shared-space admission / compile-time visibility reopen に進んだ。
- 2026-04-13 08:50 JST — `docs/reports/0667` と `specs/examples/369...370` で stable malformed broader follow-up inventory を閉じ、broader stable malformed next reopen order を missing-option first / capability second に固定した。repo-level current line は public operational CLI / final public contract later gate に進んだ。
- 2026-04-13 08:12 JST — `docs/reports/0666` で post-model-check-gate document consistency audit を閉じ、reviewer 指摘だった post-`stable malformed broader follow-up inventory` の順序 drift と `Mirrorea fabric boundary` row の stale wording を修正した。reviewer 再確認は `No findings.` で、repo-level current line は stable malformed broader follow-up inventory のまま維持した。
- 2026-04-13 07:46 JST — `docs/reports/0665` と `specs/examples/367...368` で model-check concrete carrier first actualization gate を閉じ、`proof_notebook_review_unit` current first pilot を保ったまま compare-ready bridge entry の narrow gate を fixed した。repo-level current line は stable malformed broader follow-up inventory に進んだ。
- 2026-04-13 07:28 JST — `docs/reports/0664` と `specs/examples/365...366` で shared-space identity / auth layering reopen を閉じ、membership identity core と auth/admission/projection side carriers の split を fixed した。repo-level current line は model-check concrete carrier first actualization gate に進んだ。
- 2026-04-13 03:19 JST — `docs/reports/0663` で post-package document consistency audit を閉じ、FAQ2/3、`plan/08`、Phase 6 abstract の stale `septet` / `current line` wording を修正した。reviewer 再確認と docs validator を通し、repo-level current line は shared-space identity / auth layering reopen のまま維持した。
