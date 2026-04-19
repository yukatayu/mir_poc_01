# tasks

最終更新: 2026-04-19 16:58 JST

## この文書について

- この文書は repo 全体の **current task map** である。
- 規範判断の正本は `specs/`、detail-side の repository memory は `plan/11`、`plan/12`、`plan/13`、`plan/16`、`plan/17`、`plan/18` に寄せる。
- `tasks.md` は append-only ではなく、毎回 snapshot に合わせて全体を書き直す。

## current status at task level

- current mapped corpus では、
  - authored sixteen
  - corrected prototype set `p01...p12`
  - runner / CLI / regression / helper-local compare floor
  が already runnable である。
- representative Lean sample set `e5 / p06 / p10 / p11 / p12 / p07 / p08 / p09` は actual Lean execution reached であり、`samples/lean/current-l2/` に committed corpus として保存済みである。
- `samples/lean/foundations/` には
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2ProofSkeleton.lean`
  の actual small proof fragment が入った。
- したがって、remaining work の主眼は次に移っている。
  - helper / CLI hardening と broader theorem-side / IFC / order-handoff coverage
  - final public theorem/model-check/order-handoff/shared-space contract の mixed gate
  - packaging / FFI / broader app target の user-spec residual
- exact rough stimulus は `samples/not_implemented/` preservation bucket に残し、corrected runnable version と混同しない。

## current executable floor

- `samples/current-l2/`
  - authored sixteen は inventory / runner / verification ladder / emitted artifact wiring / regression helper に乗っている
- `samples/prototype/`
  - corrected prototype set `p01...p12` は explicit path + adjacent host-plan sidecar で runnable
  - helper-local `debug_outputs` / `verification_preview` / `artifact_preview` を見せる current cut がある
- `samples/lean/`
  - `foundations/` は actual small proof fragment
  - `current-l2/` は representative Lean sample set generated stub corpus
  - `manifest.json` は Lean verification summary
  - generated current-L2 theorem stubs は `sorry` を含むため、artifact well-formedness / bridge alignment evidence として読む
- theorem / model-check / order-handoff / shared-space current floor
  - theorem-side:
    theorem-first pilot、binding preflight、Lean-stub pilot、artifact-conformance bridge、representative trace alignment、public-seam compression、toolchain probe/reopen manifest、representative Lean sample set actual Lean execution
  - model-check side:
    row-local property route、checker-artifact route、final public-contract reopen threshold、public-seam compression
  - order-handoff / shared-space side:
    authoritative-room vertical slice、surface actual adoption、source-wording route actual adoption、serial-scope reserve surface、witness/provider route/schema route actual adoption、emitted-contract trace alignment bridge、public-seam compression、CLI `surface_preview`

## ordered self-driven packages

| package | question | package weight | macro phase | current recommendation | promotion criteria |
|---|---|---|---|---|---|
| `58` helper / CLI hardening and broader coverage | actual Lean execution floor と committed Lean corpus を representative sample setからどう widen するか | `M` | `Macro 5/7` | first widening slice として `p09` carry-over を actualize 済みとし、export/sync helper と `samples/lean/` committed corpus を基点に、残る broader theorem-side / IFC / order-handoff widening を narrow package で進める | widened corpus が mixed gate を 1 つ閉じるか、proof / IFC helper route を concretize する |
| `59` near-end closeout sync | mixed gate / true user-spec residual をどこまで narrow に残すか | `S-M` | `Macro 5/6/7` | final public completion を凍らせず、closeout-ready snapshot に再圧縮する | `progress.md` / `tasks.md` / `plan/` / traceability が stale wording を残さない |

## recently closed package note

### Package 57 — Lean formal skeleton / proof obligations

- current reading:
  first slice は close 済みである。
- close evidence:
  `specs/examples/521`
  `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
  `samples/lean/current-l2/`
  `crates/mir-runtime/examples/current_l2_emit_theorem_lean_bundle.rs`
  `scripts/current_l2_lean_sample_sync.py`
- kept later:
  concrete production prover binding、final proof object public contract、final public verifier contract

### Package 56 — layered strong typing / IFC first-fragment

- current reading:
  first actual adoption package として close 済みである。
- close evidence:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `samples/lean/foundations/CurrentL2LabelModel.lean`
  `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  `samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt`
  `samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt`
- kept later:
  stronger typed source principal、checker-hint / diagnostics exhaustive integration、final typed calculus、final IFC syntax、final public verifier contract

## active package notes

### Package 58 — helper / CLI hardening and broader coverage

- current reading:
  actual Lean execution reached 後の next work は helper/CLI hardening と broader theorem-side / IFC / order-handoff corpus widening であり、first widening slice として `p09-dice-delegated-rng-provider-placement` carry-over、helper hardening slice として order-handoff CLI `surface_preview` は actualize 済みである。
- evidence:
  toolchain probe / reopen manifest、representative sample set actual Lean execution、`samples/lean/` committed corpus、source-side IFC trio `p10 / p11 / p12`、`specs/examples/525`、`specs/examples/526`、`docs/reports/0806`、`docs/reports/0807`、`p09-dice-delegated-rng-provider-placement`
- stop line:
  final public theorem contract / final parser grammar / packaging には上げない

### Package 59 — near-end closeout sync

- current reading:
  final public seams と true user-spec residual を narrow に残し、theory solved と誤読されない snapshot にする。
- evidence:
  `progress.md`、`plan/11`、`plan/17`、`plan/18`、`plan/90`

## research-discovery items

| item | 何に影響するか | current recommendation |
|---|---|---|
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + structural marker family first を維持し、evidence pressure が出るまで experimental adoption を待つ |
| final modal foundation / final source marker | syntax / modality / proof spine | partial basis + stronger family keep を維持し、final adoption は mixed gate に残す |
| authoritative-room `serial` sugar admissibility | order-handoff source-facing reserve surface | room-specific reserve に留め、principal surface には上げないまま helper-local evidence を集める |
| formal skeleton artifact shape beyond first slice | proof plan / Rust-Lean alignment | public proof artifact contract へ上げず、mechanization-ready internal floor に留める |

## remaining mixed gates

| topic | impact | current recommendation |
|---|---|---|
| final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract | theorem-first pilot | review-unit transport first、notebook-consumer object first、Lean-stub bridge current floor と representative Lean sample set actual Lean execution floor を維持し、final public theorem contract 群には上げない |
| first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract | model-check line | row-local property route first、checker-artifact route first、public-seam compression keep を維持する |
| final source-surface handoff wording / final emitted-artifact schema | order-handoff public surface | edge-row principal、stage-block secondary keep、thread/node same causal language keep、serial sugar reserve を維持する |
| final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | shared-space stronger public shape | claim/payload split first、route/schema route actual adoption、trace-alignment reserve を維持し、final public contract 群には上げない |
| stronger typed-surface actual adoption | typed source principal | checker-adjacent principal + layered stack + IFC first-fragment docs を先に actualize する |
| final modal foundation / final source marker | syntax / modality | partial basis + stronger family keep を維持する |
| final parser grammar / final public parser-checker-runtime API | public surface | this line では凍らせない |

## true user-spec residuals

| item | impact | current recommendation |
|---|---|---|
| shared-space exhaustive final catalog beyond minimal working subset | shared-space / room-profile final shape | minimal working subset default を保持し、exhaustive catalog は user-spec residual に残す |
| installed-binary / packaging / FFI / engine adapter / host integration target | backend / distribution / external embedding | repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor を near-end success とし、distribution / embedding target は later に残す |
| upper-layer application target beyond authoritative-room first scenario | broader app realization | authoritative turn-based room first を保持し、それ beyond は user goal に応じて reopen する |

## next reopen order

1. Package 58 で CLI `surface_preview` の次段として order-handoff negative corpus tightening か broader theorem-side widening を narrow package で進める。
2. Package 59 で closeout-ready snapshot を再圧縮する。
