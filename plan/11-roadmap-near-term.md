# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 package 先までの near-term execution order を示す。
step 数や task 数は厳密な約束ではなく、**rough estimate** である。

execution lane、theory-lab lane、reserve integration lane は分けて書く。

## current reading

- current execution line は `Macro 4 active on fixed authored/prototype floor` である。
- current theory-lab line は `Macro 5 final-layer closeout packages active` である。
- current reserve integration line は `Macro 6 minimal working subset actual default / Macro 7 mixed` である。
- corrected runnable floor は current mapped corpus で already reached しており、next packages はその floor を作るためではなく closeout / mixed-gate narrowing のために置いている。
- current defaults:
  - theorem-first external integration target
  - repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor = near-end success
  - first application target = authoritative shared-space turn-based room
  - shared-space scope = minimal working subset, not exhaustive final catalog
  - first room default = authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh / replay invalidation

## recently closed packages

| package band | close evidence | current reading |
|---|---|---|
| `0...10` | `docs/reports/0740...0751` + `specs/examples/466...475` | actual adoption package、syntax/modality convergence、theory spine / Lean-first proof roadmap |
| `11...19` | `docs/reports/0752...0761` + `specs/examples/476...484` | reserve strengthening / practical actualization、model-check second line、mixed-gate threshold defaults |
| `20...35` | `docs/reports/0762...0778` + `specs/examples/485...500` | theorem/model-check/order-handoff/shared-space coupled-later / actual-adoption packages |
| `36...42` | `docs/reports/0779...0785` + `specs/examples/501...507` | checker-artifact / witness-provider / theorem/model-check final-public-contract reopen thresholds |
| `43...50` | `docs/reports/0786...0795` + `specs/examples/508...515` | Lean-stub pilot、artifact-conformance、trace alignment、public-seam compression、serial reserve surface |
| `51...54` | `docs/reports/0796...0799` + `specs/examples/516...519` | toolchain probe / reopen manifest、model-check public-seam compression、representative Lean sample set actual Lean execution |
| `55` | `docs/reports/0801` + `specs/examples/520` | final-layer closeout defaults and reopened self-driven queue |
| `57 first slice` | `docs/reports/0802` + `specs/examples/521` | committed Lean sample corpus、IFC first fragment、proof-skeleton / proof-obligation first foundations |
| `56 concrete slice` | `docs/reports/0803` + `specs/examples/522` | IFC secret valid/invalid concrete example、`samples/lean/` 日本語 explanation sync、Package 56 narrowed queue |
| `56 source pair` | `docs/reports/0804` + `specs/examples/523` | source-side IFC explicit authority pair、representative Lean sample set widening、Package 56 source-side evidence close |
| `56 label-flow close` | `docs/reports/0805` + `specs/examples/524` | source-side IFC label-flow negative、representative Lean sample set widening、Package 56 actual package close |
| `58 first widening slice` | `docs/reports/0806` + `specs/examples/525` | delegated RNG provider placement carry-over、representative Lean sample set widening、Package 58 started |

## active self-driven packages

| 順番 | package | macro | question | rough weight | current exit signal |
|---|---|---|---|---|---|
| 1 | `58` helper / CLI hardening and broader coverage | `Macro 5/7` | actual Lean execution floor を representative sample setからどう widen し、helper/CLI 側をどこまで硬くするか | `M` | `samples/lean/current-l2/` と sync helper を基点に、broader theorem-side / IFC / order-handoff corpus widening が narrow package で通る |
| 2 | `59` near-end closeout sync | `Macro 5/6/7` | mixed gate / user-spec residual をどこまで narrow に残すか | `S-M` | snapshot / roadmap / traceability が stale wording を残さない |

## package detail

### Package 56 — layered strong typing / IFC first-fragment

- current source:
  `specs/examples/475`
  `specs/examples/520`
  `specs/examples/521`
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
- current recommendation:
  close 済み。checker-adjacent principal + layered stack を維持し、stronger typed surface early principal promotion はしない。`CurrentL2LabelModel.lean` と `CurrentL2IfcSecretExamples.lean` により Lean-side first fragment と secret valid/invalid concrete example は actualize 済みであり、`p10 / p11 / p12` により source-side authority success / authority-miss negative / label-flow negative trio も corrected prototype として actualize 済みと読む。
- target corpus:
  `p06-typed-proof-owner-handoff`
  `p10-typed-authorized-fingerprint-declassification`
  `p11-typed-unauthorized-fingerprint-release`
  `p12-typed-classified-fingerprint-publication-block`
- stop line:
  final typed source principal
  final IFC syntax
  final public label API

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
  concrete production prover binding
  final proof object public contract
  final public verifier contract

### Package 58 — helper / CLI hardening and broader coverage

- current source:
  `specs/examples/516...525`
- current recommendation:
  actual Lean execution floor と committed `samples/lean/` corpus を helper / CLI hardening、broader theorem-side / IFC / order-handoff negative corpus wideningに使う。first widening slice として `p09-dice-delegated-rng-provider-placement` を representative Lean sample set へ carry over してよい。
- promotion rule:
  widened corpus は mixed gate を 1 つ閉じるか、formal skeleton package を concretize する場合だけ足す。
- stop line:
  final public theorem/model-check contract
  packaging / host-facing integration

### Package 59 — near-end closeout sync

- current source:
  `specs/examples/469`
  `specs/examples/520`
  `specs/examples/521`
- current recommendation:
  final public completion を凍らせず、mixed gate / true user-spec residual を narrow に残す。
- required sync targets:
  `progress.md`
  `tasks.md`
  `plan/01`
  `plan/17`
  `plan/18`
  `plan/90`

## later mixed-gate topics

| 順番 | lane | macro | topic | next gate |
|---|---|---|---|---|
| 5 | theory-lab | `Macro 5/7` | final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema | mixed gate |
| 6 | theory-lab | `Macro 5/7` | first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact | mixed gate |
| 7 | theory-lab | `Macro 5/6` | final source-surface handoff wording / emitted-artifact schema | mixed gate |
| 8 | theory-lab | `Macro 5/6` | final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract | mixed gate |
| 9 | theory-lab | `Macro 5` | final modal foundation / source marker | mixed gate |
| 10 | reserve integration | `Macro 6/7` | exhaustive shared-space catalog / packaging / FFI targets | mixed + user-spec residual |

## current recommendation

- comparison debt より adoption debt を優先する。
- `specs/examples/458...465` は integrator / compare floor として保持するが、current line の principal anchor は `466...469`、`475`、`520` に移った。
- representative Lean sample set actual Lean execution reached は current queue zero を意味しない。
- current self-driven queue は actual Lean execution hardening だけではなく、strong typing / IFC first-fragment、Lean formal skeleton、helper/CLI hardening、near-end closeout sync を含む。
- authoritative-room default profile と append-friendly contrast room を shared-space current working subset に置く。
- `auditable_authority_witness` と `delegated_rng_service` は close 済み strengthening / practical actualization に移し、final public provider receipt / witness schema は mixed gate に残す。
- final parser grammar、final public API、final public verifier contract、installed binary、exhaustive catalog は near-term line に入れない。
