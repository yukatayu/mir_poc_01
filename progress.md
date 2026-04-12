# progress

最終更新: 2026-04-13 02:51 JST

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
- Mirrorea / shared-space docs-first re-entry bundle も fixed 済みであり、`mirrorea_fabric_boundary + shared_space_practical_boundary` を current boundary core、Typed-Effect / Prism を adjacent track、shared-space final catalog と upper-layer app target を user-spec-required gate に置く current cut を採った。
- model-check / public-checker second reserve inventory も fixed 済みであり、current first concrete carrier は `proof_notebook_review_unit` のまま維持し、model-check concrete carrier と public-checker actual migration は second reserve / kept-later に押し分けた。
- final parser grammar、final public parser / checker / runtime API、LLVM-family backend、Mirrorea operational runtime、shared-space final catalog はまだ無い。

## Macro phase map

| Macro phase | rough % | 主眼 | 現在位置 | 自走可否 | 補足 |
|---|---:|---|---|---|---|
| `Macro 0` | 95% | repository memory / docs / traceability | maintenance | 自走可能 | report / snapshot / plan / FAQ の整流ルールはかなり安定 |
| `Macro 1` | 94% | semantic kernel / invariant stabilization | late | 自走可能 | current L2 semantics / invariant bridge はかなり安定 |
| `Macro 2` | 97% | parser-free validation substrate | late | 自走可能 | parser-free PoC と detached validation loop は runnable |
| `Macro 3` | 92% | compile-ready minimal actualization | late | 自走可能 | parser / checker / runtime / formal-hook の non-production minimal cut と public-surface inventory は揃った |
| `Macro 4` | 81% | executable fixed-subset sample expansion | active | 自走可能 | current authored octet runnable。`e21/e22` contrast と `e4/e19` static pair は fixed 済みで、broader malformed follow-up は later inventory に残る |
| `Macro 5` | 49% | static reasoning / theorem / model-check bridge | early | 一部自走可能 | proof notebook review-unit current cut を first concrete pilot に固定し、model-check / public-checker second reserve inventory も fixed 済み |
| `Macro 6` | 26% | distributed fabric / shared-space / runtime evolution | docs-first re-entry fixed | 境界までは自走可能 | Mirrorea/shared-space re-entry bundle は fixed。next docs-first reopen は identity/auth layering |
| `Macro 7` | 19% | toolchain / backend / developer surface | gate selection | 一部自走可能 | public operational surface inventory は fixed 済みで、次は first public-pressure candidate の docs-only gate |
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
| guarded option chain / `lease` / monotone degradation | `S5-S6` | semantics / invariant / fixture baseline は厚く、`e3` source-authored row まで runtime success で通る | full syntax / algebra、`Approximate` / `Compensate`、formal-hook widening | 自走可能 |
| parser-free validation loop | `S6` | interpreter / host harness / bundle / batch / detached compare loop が runnable | public exporter / retention policy / detached serialization | 自走可能 |
| parser / checker / runtime actualization | `S4-S5` | `mir-ast` / `mir-semantics` / `mir-runtime` の narrow non-production tranche が compile-ready で、public-surface inventory も fixed 済み | final grammar、public surface actualization gate、richer host interface | 自走可能 |
| source-backed sample corpus / verification ladder | `S5-S6` | current authored octet `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` が ladder に乗る。`e21/e22` runtime contrast pair と `e4/e19` static-stop pair は source-backed で閉じている | broader malformed follow-up、final CLI、public retention path | 自走可能 |
| contracts / static gate / formal hook / proof notebook first bridge | `S4-S5` | tool-neutral formal hook、review-unit pilot、plain bridge sketch first actualization、compare-ready bridge sketch second reopen、model-check/public-checker second reserve inventory がある | bless / review-session metadata、model-check concrete carrier actualization、public checker actual migration | 自走可能 |
| multi-node / routing / overlay / safe downstream addition | `S2-S3` | semantic constraintsと docs-first boundaryはある | route proof、suspended task / patch interaction、operational realization | 境界までは自走可能 |
| shared-space / membership / authority / fairness | `S3` for boundary, `S1-S2` for runtime realization | authoritative room baseline と control-plane threshold はある | final activation / authority / auth / identity / admission / consistency / fairness catalog | user spec required beyond boundary |
| type system / theorem prover / model checker full line | `S1-S2` | boundary inventory と first theorem-side pilot はある | strong type system、concrete prover/model-check binding、final external contract | heavy self-driven + later user goals |
| async-control / memory-order reconstruction | `S1` | `atomic_cut` local cut は source-backed | higher-level ordering / fairness / memory-order-like surface | heavy future research |
| backend / public dev surface / operational CLI | `S0-S1` | Rust-heavy core direction と some thin examples はある | backend IR bridge、public operational CLI、LSP / editor surface | later inventory |

## 層 / subsystem ごとの現在地

| Layer / subsystem | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 現在位置 | 補足 |
|---|---:|---:|---:|---|---|
| Mir core semantics | 93% | 89% | 81% | late | current L2 semantic floor はかなり安定 |
| parser-free PoC / detached loop | 95% | 91% | 99% | late | runnable baseline と regression policy がある |
| compile-ready parser / checker / runtime | 87% | 76% | 84% | late | non-production minimal tranche は揃った |
| fixed-subset source samples | 93% | 94% | 93% | active | authored octet runnable。`e21/e22` contrast row と `e4/e19` static pair まで source-backed に actualize 済み |
| theorem / verifier bridge | 81% | 76% | 47% | early | formal hook と proof notebook first bridge はある。proof notebook current cut を first concrete pilot に固定し、`e3` は guarded non-reached row として接続済み |
| Mirrorea fabric boundary | 67% | 55% | 8% | docs-first | re-entry bundle は fixed。route proof / suspended-task interaction / operational realizationは still later |
| shared-space boundary | 72% | 61% | 12% | docs-first | practical cut と repo-level re-entry bundleは fixed。final catalog は user spec required |
| Typed-Effect Wiring Platform | 34% | 26% | 4% | positioning only | subsystem boundary はあるが actual line は薄い |
| PrismCascade | 28% | 20% | 3% | positioning only | positioning / separation が主で、actual kernel work は later |
| domain / application layer | 12% | 9% | 1% | not started | target application と success criteria が未確定 |

## 現在の self-driven line

1. **Macro 3-7 / public operational surface actualization gate**
   - already-public parser-free stack を壊さず、current compile-ready tranche の later public pressure をどこから受けるかを narrow に決める。
2. **Macro 6 / shared-space identity/auth layering reopen**
   - Mirrorea/shared-space re-entry bundle fixed 後の next docs-first reopen として、identity core と auth / admission / projection layering の cut を narrow に保つ。
3. **Macro 5 / model-check concrete carrier first actualization gate**
   - proof-notebook first pilot を保ったまま、machine-facing actualization をどの reserve bucket から最初に reopen するかを narrow に決める。
4. **Macro 4 / stable malformed broader follow-up**
   - edge-pair close 後の broader malformed follow-up を missing-option / capability family、duplicate cluster、try/rollback malformed-static familyの順で inventory し直す。

## 研究で見つけることと、user が決めること

### 研究で見つけること

- public operational surface actualization の first sub-cut
- shared-space identity / auth layering reopen の最小 boundary cut
- model-check concrete carrier first actualization gate の最小 carrier
- stable malformed broader follow-up の sequencing

### user が決めること

- shared-space final activation / authority / auth / identity / admission / consistency / fairness catalog
- upper-layer application の concrete goal と success criteria
- domain example を何から first-class に扱うか

### current reading

- current immediate mainline を止める **user decision は 0 件** と読んでよい。
- current blocker は主に research-discovery 側であり、shared-space / app 側の user decision は later gate にある。

## 実装言語の current split

- **Rust**
  - parser carrier
  - parser-free interpreter
  - static gate
  - runtime thin bridge
  - source lowerer / sample runner
  - formal hook / review-unit emitters
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
- 2026-04-13 02:51 JST — `docs/reports/0661` と `specs/examples/361...362` で stable-static edge-pair first reopen を閉じ、existing `e4` row と deferred `e19` row を source-backed static-stop pair に actualize した。repo-level current line は public operational surface actualization gate に進んだ。
- 2026-04-13 00:37 JST — `docs/reports/0659` と `specs/examples/359...360` で model-check / public-checker second reserve inventory を閉じ、`proof_notebook_review_unit` を current first concrete pilot に維持したまま、model-check side と public-checker docs-only chain を separate reserve bucket に整理した。repo-level current line は stable-static edge-pair first reopen に進んだ。
- 2026-04-13 00:21 JST — `docs/reports/0658` と `specs/examples/357...358` で Mirrorea/shared-space docs-first re-entry bundle を閉じ、Mirrorea/shared-space を repo-level current boundary track、Typed-Effect / Prism を adjacent track、shared-space final catalog と upper-layer app target を user-spec-required gate に切り分けた。repo-level current line は model-check / public-checker second reserve inventory に進んだ。
- 2026-04-12 23:48 JST — `docs/reports/0657` で post-public-surface document consistency audit を閉じ、FAQ2/3、Phase 5 abstract、`plan/01`、`plan/12` の stale wording を修正した。reviewer 再確認は `no findings` で、repo-level current line は Mirrorea / shared-space docs-first re-entry のまま維持した。
- 2026-04-12 23:40 JST — `docs/reports/0656` と `specs/examples/355...356` で parser / checker / runtime public surface inventory を閉じ、already-public parser-free stack、crate-public but non-production tranche、repo-local helper surface の 3 bucket split を固定した。repo-level current line は Mirrorea / shared-space docs-first re-entry に進んだ。
- 2026-04-12 23:33 JST — `docs/reports/0655` と `specs/examples/353...354` で stable static malformed post-contrast sequencing を閉じ、second broader cluster を stable reason-code / fixture-static cluster、Macro 4 side の next reopen point を `e4/e19` edge-pair に固定した。repo-level current line は parser / checker / runtime public surface inventory に進んだ。
- 2026-04-12 23:13 JST — `docs/reports/0654` と `specs/examples/351...352` で actual `e22` contrast-row source actualization を閉じ、`e22-try-atomic-cut-place-mismatch` を source-authored row / runner accepted set / regression inventory / verification ladder / runtime formal-hook smoke に actualize した。current line は stable static malformed post-contrast sequencing に進んだ。
