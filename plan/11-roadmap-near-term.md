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
| `58 helper/CLI surface` | `docs/reports/0807` + `specs/examples/526` | order-handoff helper CLI `surface_preview`、`p07 / p08 / p09` reached/guarded actualization、Package 58 helper hardening first slice |
| `58 negative static stop` | `docs/reports/0808` + `specs/examples/527` | order-handoff late-join visibility negative pair `p13 / p14`、helper-local static stop actualization、Package 58 negative corpus tightening |
| `58 theorem-side negative carry-over` | `docs/reports/0809` + `specs/examples/528` | order-handoff late-join visibility negative pair `p13 / p14` を representative Lean sample set / committed Lean corpus へ carry over、Package 58 broader theorem-side widening |
| `58 IFC checker-hint preview` | `docs/reports/0810` + `specs/examples/529` | source-side IFC trio `p10 / p11 / p12` を `typed_checker_hint_preview` として actualize、`family_refs[] + coverage_state` mirror、Package 58 IFC helper widening |
| `58 theorem/model-check helper preview` | `docs/reports/0811` + `specs/examples/530` | theorem result-object preview と model-check public-checker preview を `run-source-sample` helper summary へ widen し、`p08` / `p09` の reached/guard 非対称を固定、Package 58 diagnostics widening close |
| `59 near-end closeout sync` | `docs/reports/0812` + `specs/examples/531` | Package 58 close 後の queue / mixed gate / user-spec residual を再圧縮し、next self-driven queue を residual mixed-gate packages `60 / 61` へ再構成 |
| `60 theorem/model-check residual mixed-gate compression` | `docs/reports/0813` + `specs/examples/532` | theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に mirror し、`p08` / `p09` の非対称を helper-local operational summary に固定、Package 60 close |
| `61 order-handoff/shared-space residual mixed-gate compression` | `docs/reports/0814` + `specs/examples/533` | order-handoff / witness-provider public seam compression を `run-source-sample` helper summary に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に固定、Package 61 close |

## active self-driven packages

| 順番 | package | macro | question | rough weight | current exit signal |
|---|---|---|---|---|---|
| 1 | `62` typed/IFC helper-to-checker ratchet | `Macro 5/7` | sample-local `typed_checker_hint_preview` を actual checker payload family へどこまで近づけるか | `M` | final typed source principal や final public verifier contract を凍らせず、`p10 / p11 / p12` の helper-local preview を checker-adjacent payload threshold まで整理できる |

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
  `specs/examples/516...530`
- current recommendation:
  close 済み。actual Lean execution floor と committed `samples/lean/` corpus を helper / CLI hardening、broader theorem-side / diagnostics widening に使い、`p09` carry-over、`surface_preview`、`p13 / p14` static stop と theorem-side carry-over、`typed_checker_hint_preview`、theorem result-object preview helper mirror、model-check public-checker preview helper mirror まで actualize 済みと読む。
- promotion rule:
  widened corpus は mixed gate を 1 つ閉じるか、formal skeleton package を concretize する場合だけ足す。
- stop line:
  final public theorem/model-check contract
  packaging / host-facing integration

### Package 59 — near-end closeout sync

- current source:
  `specs/examples/469`
  `specs/examples/520`
  `specs/examples/530`
  `specs/examples/531`
- current recommendation:
  close 済み。Package 58 close 後の queue / mixed gate / true user-spec residual を stale wording なしで同期し、next self-driven queue を residual mixed-gate packages へ再構成した。
- required sync targets:
  `progress.md`
  `tasks.md`
  `plan/01`
  `plan/17`
  `plan/18`
  `plan/90`

### Package 60 — theorem/model-check residual mixed-gate compression

- current source:
  `specs/examples/506`
  `specs/examples/507`
  `specs/examples/530`
  `specs/examples/532`
- current recommendation:
  close 済み。theorem/model-check final public-contract reopen threshold を `run-source-sample` helper summary に narrow に mirror し、`p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を helper-local operational summary に actualize した。
- stop line:
  final public theorem/model-check contract adoption
  final public verifier contract adoption
  concrete production prover / checker binding

### Package 61 — order-handoff/shared-space residual mixed-gate compression

- current source:
  `specs/examples/505`
  `specs/examples/515`
  `specs/examples/526`
  `specs/examples/533`
- current recommendation:
  close 済み。order-handoff source wording residual / emitted-artifact residual / witness-provider public-seam residual を `run-source-sample` helper summary に narrow に mirror し、`p07 / p08` reached・`p09` guard の current cut を helper-local operational summary に actualize した。
- stop line:
  final source wording adoption
  final public witness/provider/artifact contract adoption
  final parser/public API adoption

### Package 62 — typed/IFC helper-to-checker ratchet

- current source:
  `specs/examples/522`
  `specs/examples/523`
  `specs/examples/524`
  `specs/examples/529`
- current recommendation:
  next active line。`typed_checker_hint_preview` を final typed source principal や final public verifier contract に上げず、checker-adjacent payload threshold まで narrow に ratchet する。
- stop line:
  final typed source principal
  final IFC syntax
  final public checker artifact
  final public verifier contract

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
- current self-driven queue は queue zero ではなく、order-handoff/shared-space residual compression を含む。
- authoritative-room default profile と append-friendly contrast room を shared-space current working subset に置く。
- `auditable_authority_witness` と `delegated_rng_service` は close 済み strengthening / practical actualization に移し、final public provider receipt / witness schema は mixed gate に残す。
- final parser grammar、final public API、final public verifier contract、installed binary、exhaustive catalog は near-term line に入れない。
