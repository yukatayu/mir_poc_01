# progress

最終更新: 2026-04-19 14:14 JST

## この文書について

- この文書は repo 全体の **薄い進捗 snapshot** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/`、詳細経緯は `docs/reports/` に置く。
- 進捗率は **rough estimate** であり、問題が見つかれば巻き戻る。
- 高い進捗率は current-L2 / fixed-subset / non-production / repo-local closeout line に scoped した reading であり、repo 全体の final public completion を意味しない。

## current snapshot

- current execution line:
  `Macro 4 active on fixed authored/prototype floor`
  （authored sixteen と corrected prototype nonet は runnable floor に乗っており、sample corpus は adequacy corpus として active に保つ）
- current theory-lab line:
  `Macro 5 final-layer closeout packages active`
  （`specs/examples/458...465` compare floor、`466...469` actual-adoption floor、`470...474` helper-local actualization / narrowing floor、`475...519` deeper theory / reserve / mixed-gate / actual-execution actualization floor は揃っている。corrected runnable floor と representative theorem quartet `e5 / p06 / p07 / p08` actual Lean execution は reached 済みであり、`specs/examples/520` により current self-driven queue は layered strong typing / IFC first-fragment、Lean formal skeleton / proof obligations、helper/CLI hardening and broader coverage、near-end closeout sync へ reopened している）
- current reserve integration line:
  `Macro 6 minimal working subset actual default / Macro 7 mixed`
  （authoritative room baseline、late join visible past、stale reconnect fail-then-refresh、witness/provider route-first line は current default に上がっているが、final public witness/provider/artifact contract、packaging、FFI、engine adapter、exhaustive catalog は still later に残る）

## practical reading

- current mapped corpus については、**「きちんと直した runnable version が interpreter / runner / CLI で動く」段階は already reached** している。
  - `samples/current-l2/` authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている。
  - `samples/prototype/` corrected prototype nonet は explicit path + adjacent host-plan sidecar で runnable である。
  - theorem/model-check/order-handoff/shared-space の compare / actualization floor も current representative corpus 上で machine-check されている。
- ただしこれは、**final public language implementation complete** を意味しない。
  - final parser grammar
  - final public parser / checker / runtime API
  - final public verifier contract
  - concrete theorem/model-check production binding
  - final source-surface handoff wording
  - final public witness/provider/artifact contract
  は still open または mixed gate に残る。
- current repo は discovery debt より adoption / closeout debt が大きい段階に入っている。
  - したがって remaining work の中心は execution floor を作ることではなく、layered strong typing / IFC、Lean formal skeleton、broader negative corpus、mixed gate compression を current package として閉じることに移っている。

## implementation / execution comparison status

- fresh validation evidence として、current snapshot は少なくとも次に支えられている。
  - `cargo test -p mir-runtime --test current_l2_operational_cli --test current_l2_source_sample_runner --test current_l2_theorem_actual_lean_execution_prototype_widening`
  - `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe`
  - `python3 scripts/current_l2_source_sample_regression.py inventory`
  - `python3 scripts/validate_docs.py`
- current execution / comparison floor の practical meaning は次である。
  - source sample runner / CLI は current authored sixteen と prototype floor を通す。
  - theorem-side は notebook-first / Lean-stub non-production bridge / representative trace-alignment / public-seam compression / toolchain probe / representative theorem quartet actual Lean execution まで repo-local actualization 済みである。
  - model-check side は row-local property route / checker-artifact route / final public-contract reopen threshold / public-seam compression まで repo-local actualization 済みである。
  - order-handoff / authoritative-room side は vertical slice、minimal companion surface、stage-block secondary surface、witness/provider route line まで repo-local actualization 済みである。
- したがって、remaining work の主眼は **実行可能化そのもの** ではなく、
  - layered strong typing / IFC first-fragment closeout
  - Lean formal skeleton / proof obligations
  - actual Lean execution helper/CLI hardening と broader theorem-side / negative corpus coverage
  - final public contract / wording mixed gate
  - packaging / host/app target の user-spec residual
  に移っている。

## macro phase map

| Macro phase | 主眼 | 現在位置 | rough progress % | 重さ | 自走可否 | current stop line |
|---|---|---:|---:|---|---|---|
| `Macro 0` | repository memory / docs / traceability | maintenance | 95% | 低 | 着手可能 | snapshot/detail-memory drift suppression |
| `Macro 1` | semantic kernel / invariant stabilization | late | 84% | 中 | 着手可能 | narrow semantic reopen が出たら再投入 |
| `Macro 2` | parser-free validation substrate | late | 85% | 中 | 着手可能 | helper/public drift suppression |
| `Macro 3` | compile-ready minimal actualization | late | 81% | 中 | 着手可能 | support-only/public-candidate split の維持 |
| `Macro 4` | executable fixed-subset sample expansion | active on fixed floor | 94% | 重 | 着手可能 | new sample family が core semantics を押し広げる地点 |
| `Macro 5` | typed / theorem / model-check / order-handoff theory line | final-layer closeout packages active | 95% | 重 | 着手可能 | final public contract / full calculus / production binding が必要になる地点 |
| `Macro 6` | shared-space / room-profile / runtime evolution | minimal working subset default fixed + public-seam compression closed | 73% | 重 | 着手可能 | final public witness/provider/artifact contract、stronger fairness profile、exhaustive catalog gate |
| `Macro 7` | toolchain / backend / host-facing integration | mixed with repo-local near-end success criteria | 50% | 重 | 着手可能（repo-localまで） | packaging / installed binary / FFI / engine adapter gate |
| `Macro 8` | domain / application realization | first authoritative-room scenario chosen | 18% | とても重い | mixed | broader app target / acceptance criteria gate |

## feature family snapshot

| feature family | 現在地 | できていること | 次の意味ある一歩 |
|---|---|---|---|
| current-L2 runner / CLI | `S6` | authored sixteen、prototype nonet、pretty/json CLI、regression floor | widened corpus は mixed gate か closeout package を 1 つ閉じる時だけ追加 |
| theorem-side pilot | `S6` | review-unit first、Lean-stub pilot、artifact-conformance bridge、representative trace alignment、public-seam compression、toolchain probe/reopen manifest、representative theorem quartet actual Lean execution | Lean formal skeleton と helper/CLI hardening |
| model-check line | `S6` on docs, `S4-S5` on implementation | row-local property route、checker-artifact route、reopen threshold、public-seam compression | first settled property language / tool brand mixed gate と negative corpus widening |
| order / handoff line | `S6` | relation decomposition principal、surface actual adoption、source-wording route、stage-block secondary、serial-scope reserve surface、public-seam compression | final source wording / emitted-artifact schema mixed gate と negative corpus tightening |
| shared-space minimal subset | `S6` | authoritative room baseline、vertical slice、witness/provider route/schema route、emitted-contract trace alignment、public-seam compression | final public contract mixed gate |
| syntax / modality | `S6` on docs, `S3-S4` on implementation | semantic honesty principle、5 axes、partial basis keep、minimal companion + stage-block secondary | final foundation / final marker mixed gate へ行く前の source package hardening |
| strong typing / IFC | `S5` on docs, `S2-S3` on implementation | layered stack、`LabelModel` / `Labeled` sketch、checker-adjacent principal | first checker fragment + secret-key valid/invalid corpus |
| proof spine / formalization | `S6` on docs, `S3-S4` on implementation | multimodal dependent core direction、Lean-first roadmap、actual Lean execution quartet floor | mechanization-ready core と proof-obligation list の actual package化 |
| backend / packaging | `S2-S3` | repo-local CLI/tests/artifacts/compare floor | packaging / installed binary / FFI later |

## layer / track progress

| layer / track | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手性 |
|---|---:|---:|---:|---|
| semantic kernel | 92% | 86% | 76% | 着手可能 |
| parser-free substrate | 89% | 79% | 87% | 着手可能 |
| compile-ready minimal actualization | 85% | 75% | 85% | 着手可能 |
| fixed-subset source sample line | 86% | 83% | 89% | 着手可能 |
| corrected prototype line | 85% | 83% | 89% | 着手可能 |
| Problem 1 typed / theorem / model-check | 94% | 91% | 81% | final public seams 以外は自走可能 |
| Problem 2 order / handoff / room default | 89% | 87% | 66% | final public wording / contract 以外は自走可能 |
| strong typing / IFC closeout | 76% | 68% | 32% | first checker fragment までは自走可能 |
| proof spine / mechanization-ready core | 82% | 73% | 39% | Lean-first skeleton までは自走可能 |
| shared-space minimal working subset | 73% | 67% | 49% | minimal subset strengthening までは自走可能 |
| syntax / modality comparison | 81% | 79% | 27% | helper-local surface narrowing までは自走可能 |
| backend / public packaging | 37% | 32% | 31% | repo-local near-end success までは自走可能 |
| broader application realization | 18% | 14% | 7% | first scenario beyond は要仕様確認 |

## twin peaks

### Problem 1 — 型システム / 定理証明 / モデル検査

- current first line:
  checker-adjacent semantic carrier principal、structural marker family first、theorem-first external integration target、notebook-first theorem line、row-local model-check carrier first
- current runnable / machine-check evidence:
  `p06`、verifier preview alignment pre-floor、theorem discharge pre-floor、model-check projection pre-floor、theorem-first pilot actualization、theorem binding preflight、theorem Lean-stub pilot、theorem artifact-conformance bridge、theorem representative trace-alignment bridge、theorem public-seam compression、theorem toolchain probe/reopen manifest、representative theorem quartet actual Lean execution、model-check row-local property route、model-check checker-artifact route、model-check public-seam compression
- current self-driven closeout package:
  layered strong typing / IFC first-fragment、Lean formal skeleton / proof obligations、broader theorem/model-check negative corpus
- remaining stop line:
  stronger typed-surface actual adoption、final public theorem result object、consumer-shaped theorem payload public contract、concrete theorem prover brand、proof object public schema、first settled property language、concrete model-check tool brand、final public checker artifact、actual public checker migration、final public verifier contract

### Problem 2 — order / handoff / `memory_order` / authority-handoff

- current first line:
  cut family decomposition、relation decomposition principal、`authority_serial_transition_family` first、thread/node parity wording default、authoritative room first actual default profile、low-level family retained-later reference
- current runnable / machine-check evidence:
  `p07 / p08 / p09`、authoritative-room vertical slice、witness/provider route actual adoption、witness/provider schema route actual adoption、witness/provider emitted-contract trace alignment bridge、order-handoff surface actual adoption、order-handoff source-wording route actual adoption、order-handoff serial-scope reserve surface、order-handoff / witness-provider public-seam compression、auditable-authority-witness strengthening actualization、delegated-rng-service practical actualization
- current self-driven closeout package:
  missing-witness / handoff-before-publish negative corpus tightening、source principal edge-row family hardening、mixed gate residual compression
- remaining stop line:
  final source-surface handoff wording、final emitted-artifact / emitted-handoff schema、final public witness/provider/artifact contract、stronger fairness / replay profile、exhaustive shared-space catalog

### syntax / modality

- current first line:
  semantic honesty > compactness、5 evaluation axes、explicit edge-row principal、stage-block secondary、`serial on ...` reserve、`lambda_circle_box` partial basis、guarded / MDTT / MTT / Fitch-style multimodal retained stronger family
- current helper evidence:
  minimal companion / experimental order-handoff surface、stage-block secondary order-handoff surface
- current self-driven closeout package:
  order/handoff source package hardening、modal markers remain non-final、stronger family comparison is kept only as long as it helps actual packages
- remaining stop line:
  final parser grammar、final source marker、final modal foundation adoption

## current self-driven queue

- `Package 55` final-layer closeout integration
  - `specs/examples/520` を current package anchor にし、representative theorem quartet actual Lean execution 以後の queue drift を解消する
- `Package 56` layered strong typing / IFC first-fragment
  - `LabelModel`、`Labeled`、explicit authority、secret-key valid/invalid corpus を current checker/theorem split へ接続する
- `Package 57` Lean formal skeleton / proof obligations
  - mechanization-ready core、proof obligations list、Rust/Lean alignment floor を current repo-local evidence に合わせて actualize する
- `Package 58` helper / CLI hardening and broader coverage
  - actual Lean execution floor を helper/CLI と broader theorem-side / IFC / order-handoff corpus へ widen する
- `Package 59` near-end closeout sync
  - mixed gate / user-spec residual を再圧縮し、snapshot / roadmap / traceability を同期する

## recent log

- 2026-04-19 11:25 JST — current runnable floor と final public completion の誤読を避けるため、`progress.md` / `tasks.md` / relevant `plan/` / docs snapshot を全面見直しし、runner / CLI / theorem bridge / room-profile representative validation を取り直したうえで queue を reserve packages と mixed/user-spec gates に分けて再記述した。
- 2026-04-19 12:00 JST — Package 46/47/48 として `serial` reserve surface、witness/provider emitted-contract trace-alignment bridge、local Lean availability probe を実装・検証し、`specs/examples/511...513` と queue snapshot を同期した。
- 2026-04-19 12:21 JST — Package 49/50 として theorem public-seam compression、order-handoff / witness-provider final public-seam compression を実装・検証し、`specs/examples/514...515` と residual-gate snapshot を同期した。
- 2026-04-19 12:58 JST — Package 51/52/53 として theorem toolchain probe/reopen manifest、model-check public-seam compression、actual Lean execution narrow probe を実装・検証し、`specs/examples/516...518` と current queue を actual-execution widening 読みに更新した。
- 2026-04-19 13:09 JST — Package 54 として `p06 / p07 / p08` actual Lean execution widening regression を追加し、`specs/examples/519` と snapshot 文書を representative theorem quartet actual-execution 読みに更新した上で、focused cargo/Python/docs validation を通した。
- 2026-04-19 14:14 JST — `specs/examples/520` と 2026-04-19 handoff を統合し、queue drift を再監査したうえで `progress.md` / `tasks.md` / `plan/` / document map を final-layer closeout packages active 読みに更新した。
