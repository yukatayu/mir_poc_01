# 475 — current L2 principal theory spine and Lean-first proof roadmap

## 目的

`faq_008.md`、
`specs/examples/466`、`467`、`468`、`469`、
`470`、`473`、`474`
および 2026-04-18 の theory handoff を前提に、

- principal theory spine の current research-direction
- layered typing / proof architecture
- layer compatibility / obligation export の metatheory package
- Lean-first proof / formalization roadmap
- retained alternatives
- stop line

を、near-end theory closeout の次段 package として整理する。

ここで fixed するのは、
**final calculus adoption を行わずに、current repo が何を principal research-direction として進めるか**
であり、

- final parser grammar
- final adopted modal foundation
- final public parser / checker / runtime API
- final public verifier contract
- concrete theorem / model-check production contract
- exhaustive shared-space final catalog

は fixed しない。

## applied defaults

current package では、current reading と user-position default を次の形で採る。

1. ばらばらの理論候補を横並びに維持せず、principal spine を 1 本置く
2. local theory layer を module / region ごとに持つこと自体は許す
3. ただし layer compatibility / obligation export の metatheory を必須にする
4. multistage computation は core-ish concern として扱う
5. strong typing、contract-programming-like aspects、dependent-like power、user-defined security label model は desired line として扱う
6. theorem/model-check public contract の final adoption は still later に残す
7. proof assistant route は Lean-first を current recommendation に置く

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. Problem 1 current first line / actual adoption package
   - checker-adjacent semantic carrier principal
   - source-visible structural marker family first
   - theorem-first external integration target
   - notebook-first theorem line
   - row-local model-check carrier first
2. Problem 2 current first line / actual adoption package
   - relation decomposition principal
   - `authority_serial_transition_family` first
   - `witness_aware_commit_family` second
   - authoritative-room default first profile
3. syntax / modality current recommendation
   - semantic honesty principle
   - 5 evaluation axes
   - `lambda_circle_box` partial basis
   - guarded / MDTT / MTT / Fitch-style multimodal retained stronger family
4. helper-local actualization / narrowing floor
   - theorem-first pilot actualization
   - theorem-prover binding preflight
   - authoritative-room vertical-slice actualization
   - explicit edge-row principal surface
   - stage-block secondary surface

したがって current open problem は discovery ではなく、
**principal spine と proof/formalization route をどう current direction として固定するか**
に narrowed している。

## current recommendation

### principal spine

current principal research-direction は、
**multimodal dependent core**
に置く。

ここでいう principal は final adoption ではなく、
typed / theorem / model-check / order-handoff line を 1 本の spine で揃えるための current direction である。

### modal family reading

| family | current role |
|---|---|
| `lambda_circle_box` | partial basis candidate |
| guarded lambda-calculus | retained stronger family |
| MDTT | retained stronger family |
| MTT | strongest current full-spine candidate family |
| Fitch-style multimodal | retained formulation family |

current repo が fixed するのは、
**partial basis + stronger family keep**
である。

したがって、
`MTT` family を strongest current candidate と言ってよいが、
**final adopted foundation**
と書いてはならない。

### layered theory stack

current principal stack は次に置くのが自然である。

1. multimodal dependent core
2. linear / affine capability layer
3. row-polymorphic effect layer
4. decidable refinement / contract layer
5. information-flow / security label layer
6. theorem / model-check bridge

この layered reading は、

- checker floor を narrow に保つ
- residual obligation を theorem/model-check/runtime-policy boundary へ送る
- strong typing / contract / dependent-like power / IFC を一段で collapse しない

ための principal direction である。

### conceptual judgment sketch

current conceptual reference judgment は、少なくとも次の形で読める。

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

Where:

- `Ψ` = mode / world assumptions
- `Γ` = unrestricted + dependent context
- `Δ` = linear / ownership context
- `m` = mode
- `ε` = effect row
- `O` = residual obligations

final syntax や final checker API を先に凍らせず、current spine を説明するための sketch として扱う。

### IFC / label model reading

current package では、
IFC を vague future line にせず、
**layered typing architecture の first-class later package**
として置いてよい。

最小 sketch は次である。

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

ただし current package は、

- final IFC syntax
- final public label API
- covert channel theorem
- distributed trust finalization

を fixed しない。

## compatibility metatheory

local theory layer を module / region ごとに持つ current directionでは、
少なくとも次の metatheory package が必要である。

1. elaboration soundness
2. module interface well-formedness
3. cross-layer preservation
4. obligation export soundness
5. boundary checking soundness

current repo が actualize するのは、
monolithic full calculus ではなく、
**core calculus + elaborator + interface / manifest + obligation artifacts**
を first path に置く line である。

## Lean-first proof roadmap

### principal route

current principal route は、
**Lean-first**
に置く。

Use Lean-first for:

- core metatheory
- elaboration correctness
- checker fragment correctness
- obligation extraction
- modal/dependent core experiments

### retained fallback

`Rocq/Coq + Iris` は、

- runtime internals に concurrency / separation-logic pressure が強く出る
- Lean-first route だけでは reuse が弱い

場合の retained fallback に留める。

### staged roadmap

1. Stage A
   - prose + pseudocode core
   - judgmental spine
   - module/interface and obligation export shape
2. Stage B
   - Lean core
   - preservation / progress
   - linearity / ownership soundness
   - effect soundness
   - elaboration soundness
3. Stage C
   - algorithmic checker fragment
   - soundness / decidability / limited completeness
4. Stage D
   - obligation / bridge layer
   - carrier extraction soundness
   - bounded abstraction soundness
5. Stage E
   - Rust/Lean alignment
   - differential testing
   - property-based testing
   - artifact conformance
   - representative corpus trace alignment

### not required before implementation

current package は implementation を次で block しない。

- final calculus adoption
- final public parser/runtime API proof
- complete global fairness theorem
- exhaustive shared-space catalog proof

## retained alternatives

- stronger typed surface early source-principal adoption
- plugin-like staging line rather than core-ish multistage line
- `lambda_circle_box` only hold
- early reduction to guarded-only or MDTT-only
- Rocq/Iris-first proof route
- public theorem/model-check contract first
- final parser/public API proof before current implementation ratchet

## stop line

current package は次で止める。

- final adopted calculus
- final source markers
- final public parser / checker / runtime API
- final public verifier contract
- concrete theorem / model-check production contract
- exhaustive fairness / replay theorem
- exhaustive shared-space final catalog

## current judgment

1. principal theory spine を current research-direction として明示してよい。
2. その spine は multimodal dependent core に置くのが自然である。
3. `lambda_circle_box` は partial basis に留め、guarded / MDTT / MTT / Fitch-style multimodal は stronger family に retained する。
4. current layered architecture は、linear capability + effect rows + decidable refinement / contract + IFC label layer + theorem/model-check bridge として読むのが自然である。
5. local theory layer を許すなら、compatibility metatheory を mandatory にする。
6. proof/formalization route の current recommendation は Lean-first に置いてよい。
7. ここで fixed するのは current direction であり、final calculus adoption や final public contract ではない。

## next self-driven line

historical package-local follow-up chain としては、
少なくとも次が compare-anchor memory に残る。

1. reserve strengthening / practical actualization
   - `specs/examples/476`
   - `specs/examples/477`
2. model-check second-line concretization と theorem/model-check/order-handoff mixed-gate actualization
   - `specs/examples/478...507`
3. theorem actual Lean execution representative prototype widening
   - `specs/examples/519`
4. final-layer closeout defaults / reopened self-driven queue
   - `specs/examples/520`

historical remaining self-driven line として保持していた

1. layered strong typing / IFC first-fragment
2. Lean formal skeleton / proof obligations
3. helper / CLI hardening and broader coverage
4. near-end closeout sync

も package-memory の follow-up shape として読める。

ただし current repo-level queue authority は `progress.md` / `tasks.md` にあり、
2026-04-30 current-line maintenance closeout 後に
この package から追加の self-driven implementation line を promote しない。
