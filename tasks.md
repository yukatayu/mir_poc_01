# tasks

最終更新: 2026-04-19 13:09 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18` に寄せる。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current status at task level

- current mapped corpus では、
  - authored sixteen
  - corrected prototype nonet
  - runner / CLI / regression / helper-local compare floor
  が already runnable である。
- したがって、「きちんと直した version が interpreter / runner で動く状態にする」こと自体は、**current mapped family では principal blocker ではない**。
- remaining work は次に寄っている。
  - actual Lean execution helper/CLI hardening と broader coverage
  - final public theorem/model-check/order-handoff/shared-space contract の mixed gate
  - packaging / FFI / broader app target の user-spec residual
- exact rough stimulus は `samples/not_implemented/` preservation bucket に残し、corrected runnable version と混同しない。

## current executable floor

- `samples/current-l2/`
  - authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている
- `samples/prototype/`
  - corrected prototype nonet は explicit path + adjacent host-plan sidecar で runnable
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview` を見せる current cut がある
- theorem / model-check / order-handoff / shared-space current floor
  - theorem-side:
    theorem-first pilot、binding preflight、Lean-stub pilot、artifact-conformance bridge、representative trace alignment、public-seam compression、toolchain probe/reopen manifest、representative theorem quartet actual Lean execution
  - model-check side:
    row-local property route、checker-artifact route、final public-contract reopen threshold、public-seam compression
  - order-handoff / shared-space side:
    authoritative-room vertical slice、surface actual adoption、source-wording route actual adoption、serial-scope reserve surface、witness/provider route/schema route actual adoption、emitted-contract trace alignment bridge、public-seam compression

## ordered self-driven packages

- current reading:
  theorem / model-check / order-handoff / shared-space の unconditional compression package は close 済みであり、current self-driven queue は actual Lean execution helper/CLI hardening と broader coverage、later mixed gate tracking に narrow 化した

## actual Lean execution hardening package

- actual Lean execution
  - current reading:
    global `elan` stable install は済みであり、representative theorem quartet `e5-underdeclared-lineage`、`p06-typed-proof-owner-handoff`、`p07-dice-late-join-visible-history`、`p08-dice-stale-reconnect-refresh` では actual Lean execution reached
  - reopen condition:
    next は helper/CLI orchestration か、broader theorem-relevant corpus への widening を narrow package として進める

## research-discovery items

| item | 何に影響するか | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持し、evidence pressure が出るまで experimental adoption を待つ |
| final modal foundation / final source marker | syntax / modality / proof spine | partial basis + stronger family keep を維持し、final adoption は mixed gate に残す |
| authoritative-room `serial` sugar admissibility | order-handoff source-facing reserve surface | room-specific reserve に留め、principal surface には上げないまま helper-local evidence を集める |

## remaining mixed gates

| topic | impact | current recommendation |
|---|---|---|
| final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract | theorem-first pilot | review-unit transport first、notebook-consumer object first、Lean-stub bridge current floor と representative theorem quartet actual Lean execution floor を維持し、final public theorem contract 群には上げない |
| first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract | model-check line | row-local property route first、checker-artifact route first、public-seam compression keep を維持する |
| final source-surface handoff wording / final emitted-artifact schema | order-handoff public surface | edge-row principal、stage-block secondary keep、thread/node same causal language keep、serial sugar reserve を維持する |
| final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | shared-space stronger public shape | claim/payload split first、route/schema route actual adoption、trace-alignment reserve を維持し、final public contract 群には上げない |
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持する |
| final modal foundation / final source marker | syntax / modality | partial basis + stronger family keep を維持する |
| final parser grammar / final public parser-checker-runtime API | public surface | this line では凍らせない |

## true user-spec residuals

| item | impact | current recommendation |
|---|---|---|
| exhaustive final shared-space catalog beyond minimal subset | room profile / authority / replay scope | minimal working subset を current default に保ち、exhaustive finalization は後段へ送る |
| installed-binary / packaging / FFI / engine adapter / host integration target | backend / toolchain / operational target | repo-local CLI + tests + emitted artifacts + compare floor を near-end success に保つ |
| upper-layer application target beyond authoritative room scenario | domain-specific acceptance criteria | authoritative shared-space turn-based roomを first target に保ち、broader target は later user choice に残す |

## self-driven macro phase reading

| macro phase | self-driven closeout 読み | 注記 |
|---|---|---|
| `Macro 0` | はい | snapshot / traceability drift suppression まで |
| `Macro 1` | はい | narrow semantic reopen の closeout まで |
| `Macro 2` | はい | current parser-free substrate closeout まで |
| `Macro 3` | はい | compile-ready minimal actualization closeout まで |
| `Macro 4` | はい | fixed-subset widening / adequacy closeout まで |
| `Macro 5` | はい | current corrected runnable floor reached。remaining work は actual-execution hardening と later mixed gate tracking |
| `Macro 6` | はい | minimal working subset default と public-seam compression closeout までは self-driven |
| `Macro 7` | はい（repo-localまで） | repo-local CLI/tests/artifacts/compare floor まで。packaging/FFI は later |
| `Macro 8` | mixed | authoritative room default scenario までは narrow self-driven、broader app target は user-spec |

## compact answer to the user's practical question

- current mapped examplesについては、**「直した version が実際に動く状態」へ行くための principal task はもう大きくは残っていない**。
- いま残っている task は、
  - actual Lean execution helper/CLI hardening と broader theorem-side coverage
  - final public contract / wording mixed gate
  - packaging / host/app target の user-spec residual
  が中心である。
- したがって、理論構築パートは「まだ何も決まっていない」段階ではなく、
  **current first line は揃っており、remaining theory work は public seams をどう止めるかの narrowing に移っている**
  と読むのが正確である。

## guard

- corrected runnable floor reached は final public language completion を意味しない
- current self-driven queue nonzero は theory solved を意味しない
- `atomic_cut` を global consistent cut / durable commit / seq_cst fence と同一視しない
- low-level exact `memory_order` family を final source wordingにしない
- final parser grammar / final public API / final verifier contract / exhaustive final shared-space catalog をこの line で凍らせない
