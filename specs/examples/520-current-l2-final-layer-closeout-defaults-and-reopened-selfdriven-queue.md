# 520 — current L2 final-layer closeout defaults and reopened self-driven queue

## 目的

`faq_009.md`、
`specs/examples/475`、
`specs/examples/519`、
2026-04-19 の final-layer closeout handoff
を前提に、

- current first line を壊さずに final-layer closeout をどう進めるか
- compare-floor を増やさずに actual package へ何を戻すか
- current self-driven queue をどこまで nonzero に保つか
- near-end success をどこで止めるか

を、current repo の source-backed default として整理する。

ここで fixed するのは
**final public language completion ではなく、final-layer closeout tranche の current package order**
である。

したがって、ここでは次を fixed しない。

- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- final public theorem / model-check production contract
- exhaustive shared-space final catalog
- installed binary / packaging / FFI / engine adapter target

## preserve する境界

current package でも、少なくとも次は崩さない。

1. `specs/` が規範正本、`plan/` が repository memory、`docs/reports/` が時系列証跡である。
2. Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform は separable subsystem として保つ。
3. `atomic_cut` は current `place` の rollback frontier だけを確定する place-local finalizing cut nucleus であり、global consistent cut / global fence / durable commit ではない。
4. verifier boundary は
   - `core_static_checker`
   - `theorem_prover_boundary`
   - `protocol_verifier_boundary`
   - `runtime_policy_boundary`
   の 4-way split を保つ。
5. source principal は high-level relation decomposition / authority-serial / witness-aware line を優先し、low-level `std::memory_order` / `std::kill_dependency` family は backend-aligned retained reference family に留める。
6. current L2 notation は companion notation であり、final grammar ではない。
7. explicit edge-row / vertical continuation を source principal に保ち、stage-block は strong secondary、`serial on ...` は authoritative-room-specific reserve sugar に留める。
8. `lambda_circle_box` は partial basis candidate に留め、guarded / MDTT / MTT / Fitch-style multimodal family を stronger family として retained する。

## current default decisions

### principal theory spine

- current theory-lab spine は **multimodal dependent core** に置く。
- `lambda_circle_box` は stage/later/always の useful partial basis だが、full spine には不足する。
- guarded lambda-calculus、MDTT、MTT、Fitch-style multimodal family は stronger candidate family として retained する。

### layered strong typing

- stronger typed surface を early source principal には上げない。
- current principal reading は、
  1. multimodal dependent core
  2. linear / affine capability
  3. row-polymorphic effects
  4. decidable refinement / contract
  5. information-flow / security label layer
  6. theorem / model-check bridge
  の layered stack に置く。
- current conceptual judgment sketch は、

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

  を reference floor として使ってよい。

### IFC / security-label line

- IFC は vague future line ではなく、current layered typing architecture の first-class package として扱ってよい。
- current minimal sketch は、

```text
LabelModel = {
  Label
  flows_to
  join
  meet
  flows_dec
}
```

```text
Labeled ℓ A
```

  に置くのが自然である。
- explicit declassification / endorsement authority と secret-key-like example は current negative/positive corpus に含めてよい。
- ただし final IFC syntax、covert-channel theorem、distributed trust finalization は still later に残す。

### proof / formalization route

- current recommendation は **Lean-first** である。
- Rocq/Coq + Iris family は runtime concurrency / separation-logic pressure が強まった場合の retained fallback とする。
- current staged roadmap は、
  1. mechanization-ready prose core
  2. Lean core
  3. checker fragment
  4. obligation / bridge layer
  5. Rust/Lean alignment
  を first line に置く。

### order / handoff syntax family

- principal syntax family:
  explicit edge-row / vertical continuation
- strong secondary:
  stage-block + witness
- reserve sugar:
  `serial on <owner>`

source principal では、
low-level `memory_order` exact surface を採らない。

### first completion scope

- theorem-first external integration target を first line に置く。
- model-check は second line とする。
- first application target は authoritative shared-space turn-based room に置く。
- first completion success は
  repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor
  で読む。
- installed binary / packaging / FFI / engine adapter / broad host integration は near-end success の必須条件にしない。

## actual package reading

### Problem 1

current first line は次である。

- checker-adjacent principal
- source-visible structural marker family first
- theorem-first external integration target
- notebook-first theorem line
- row-local model-check carrier first
- `p06` corrected prototype bridge-floor evidence

retained alternatives は次に留める。

- stronger typed surface early source-principal adoption
- final public theorem/model-check contract first
- concrete production binding first

### Problem 2

current first line は次である。

- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread/node parity:
  `thread と node は同じ causal language で書き、違いは lowering / evidence / transport / failure / durability / policy に残す`
- authoritative room first default profile
- low-level retained-later reference family

retained alternatives は次に留める。

- `snapshot_cut` / `consistent_cut` wording as comparison-only observation family
- stage-block principal promotion
- `serial on ...` principal promotion
- low-level exact source wording import

### syntax / modality

current first line は次である。

- semantic honesty > compactness
- 5 evaluation axes:
  semantic honesty / checker legibility / modal adequacy / misreading resistance / lowering friendliness
- explicit edge-row principal
- stage-block strong secondary
- `serial on ...` reserve
- `lambda_circle_box` partial basis + stronger family keep

## reopened self-driven queue

current queue は empty ではなく、次の final-layer closeout packages に reopened している。

| package | question | current first line | adequacy corpus / evidence | stop line |
|---|---|---|---|---|
| `55` final-layer closeout integration | representative theorem quartet actual Lean execution 以後の theory line をどこで再始動するか | `520` を current package anchor にして closeout queue を戻す | `faq_009`、`475`、`519`、handoff 2026-04-19 | final public completion に飛ばない |
| `56` layered strong typing / IFC first-fragment | strong typing / IFC をどこまで first checker fragment と docs corpus に入れるか | checker-adjacent principal + layered stack + `LabelModel` / `Labeled` / explicit authority | `p06`、secret-key valid/invalid sample family、negative corpus | stronger typed surface final principal / final IFC syntax は later |
| `57` Lean formal skeleton / proof obligations | mechanization-ready core をどこまで formal skeleton に落とすか | Lean-first staged roadmap | `e5 / p06 / p07 / p08` actual Lean execution floor、artifact conformance bridge、trace alignment | concrete production prover binding / final proof object contract は later |
| `58` helper / CLI hardening and broader theorem-side coverage | actual Lean execution floor を representative quartet からどう widen するか | helper/CLI hardening + broader theorem/IFC/order-handoff corpus | existing cargo/CLI regression + widened negative corpus | final public theorem contract には上げない |
| `59` near-end closeout sync | remaining mixed gate / user-spec residual をどこまで narrow に残すか | mixed gate / user-spec residual separation | snapshot + roadmap + report sync | theory solved / language complete と書かない |

## sample corpus to add or re-verify

current package では、少なくとも次の family を current self-driven evidence として扱ってよい。

1. authoritative dice handoff valid
2. missing witness static stop
3. handoff before publish static stop
4. late join visible history
5. stale reconnect fail-then-refresh
6. secret-key / IFC valid and invalid

この corpus は、
final public language sample catalog ではなく、
current final-layer closeout adequacy corpus として使う。

## actual runnable evidence

current queue reopening は、execution floor 未到達を意味しない。

already reached / machine-checked evidence として、少なくとも次がある。

- authored sixteen runner / CLI / regression floor
- corrected prototype nonet
- theorem public-seam compression
- model-check public-seam compression
- order-handoff / witness-provider final public-seam compression
- theorem toolchain probe / reopen manifest
- representative theorem quartet `e5 / p06 / p07 / p08` actual Lean execution

したがって reopened queue は
**execution floor を作るためではなく、final-layer closeout debt を減らすため**
に読む。

## stop line

current tranche は次で止める。

- final adopted calculus
- final source markers
- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- final public theorem / model-check production contract
- exhaustive fairness / replay theorem
- exhaustive shared-space final catalog

## current judgment

1. final-layer closeout handoff は、current first line を overturn するものではなく、already surviving line を actual package として再配線する current explanation source である。
2. current self-driven queue は actual Lean execution hardening だけに narrow 化したと読むのでは足りず、layered strong typing / IFC、Lean formal skeleton、negative corpus widening、near-end closeout sync を含む reopened package chain として読む方が正確である。
3. current repo はかなり手を動かせる段階に到達しているが、final public language implementation complete と書いてはならない。
4. remaining debt の中心は discovery debt より adoption / closeout debt である。
