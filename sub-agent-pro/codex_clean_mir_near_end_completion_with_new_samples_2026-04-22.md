
# Mir clean near-end completion handoff: typed-index core, order/handoff, model-checking, and new samples

**Audience:** CodeX / GPT-5.4 xhigh + sub-agents
**Purpose:** Replace the old illustrative samples with a clean, principled sample suite and implementation plan.
**Strict instruction:** Do not reuse the old sample code as the new canonical code. Treat the old samples only as semantic stimuli. Extract their intent, then define new samples from first principles.

---

## 0. Executive summary

The user’s latest clarification changes the priority of this layer.

Earlier samples such as `declassify_authority(player_a)` were useful as helper-local scaffolding, but they are no longer acceptable as the principal design. They make it look as if domain predicates are silently built in. That is not the intended Mir design.

The intended direction is:

```text
No hidden domain primitives.
No built-in declassify_authority.
No magic observer_role.
No magic fingerprint_visible.

Instead:
  user-defined finite index theories
  + source-level policy declarations
  + typed constraint generation
  + finite static solving
  + residual theorem/model-check/runtime obligations where necessary
```

The clean design must keep the Mir principles:

- event DAG / `place`
- first-class effects and contracts
- explicit `require` / `ensure`
- guarded option chains
- `lease`
- local `atomic_cut`
- no hidden rollback across cut
- fallback as monotone degradation
- no re-promotion
- theorem/model-check/runtime-policy boundaries
- thread/node parity at causal-language level
- low-level memory-order family as backend/reference, not source principal

The “once-through completion” target is:

```text
A repo-local, runnable, non-final-public language layer that demonstrates:
  1. user-defined authority and security lattices
  2. type-level finite preorder/lattice checks
  3. information-flow rejection
  4. capture/lifetime rejection
  5. simple cost-bound rejection
  6. theorem-obligation emission
  7. model-check second-line properties
  8. order/handoff/memory-order reinterpretation
  9. authoritative-room first scenario
  10. docs/specs/plan consistency
```

Final public grammar/API/verifier/toolchain may remain future work.

---

## 1. Non-negotiable design constraints

### 1.1 No hidden domain primitives

The following are **not** language built-ins:

```text
declassify_authority
observer_role
fingerprint_bound
fingerprint_visible
room_members
authority_rng
dice_owner
draw_slot
```

They may appear in examples only after being defined by:

- `index theory`
- `policy`
- `principal`
- `resource`
- `effect`
- `contract`
- `module signature`
- `host interface`
- `room profile`

If a sample uses a domain word that is not defined in the sample or imported from a declared module/policy, the sample should fail.

### 1.2 Minimal structural built-ins

The language may have structural syntax. These are not domain primitives; they are the small Mir core surface.

Proposed structural keywords:

```text
module
index
policy
principal
resource
effect
place
option
chain
fallback
lineage
perform
via
require
ensure
atomic_cut
transition
stage
publish
observe
handoff
witness
model
property
```

These are still **current proposal / companion surface**, not final grammar. CodeX must not claim final parser grammar.

### 1.3 User-defined type-level partial orders are principal

The new samples must define authority and labels in code.

Example:

```mir
index authority FingerprintAuthority {
  element Observer
  element Holder
  element Releaser
  element Admin

  order Observer <= Holder <= Releaser <= Admin
  law preorder
}
```

Example:

```mir
index label SecurityLabel {
  element Public
  element UserSecret
  element KeyMaterial

  order Public <= UserSecret <= KeyMaterial
  join
  meet
  law finite_lattice
}
```

No sample should rely on an undeclared authority hierarchy.

### 1.4 Full dependent type theory is not required for first completion

The first strong typing layer is:

```text
finite decidable index fragment
+ linear/capability context
+ effect rows
+ refinement-like contracts
+ user-defined finite labels/policies
+ residual proof obligations
```

The conceptual typing judgement is:

```text
Σ ; Ψ ; Γ ; Δ ⊢ e : A @ μ ! ε ⇝ C ; O
```

Where:

```text
Σ = user-defined index theories / policy environment
Ψ = mode, stage, place, world, visibility environment
Γ = ordinary unrestricted typing context
Δ = linear / affine / capability context
A = ordinary type
μ = mode, e.g. local, staged, published, witnessed, durable
ε = effect row
C = compile-time constraints
O = residual theorem/model-check/runtime obligations
```

### 1.5 Static where possible, residual where necessary

Compiler/checker should solve:

- finite authority preorder constraints
- finite security label flow constraints
- finite capture-set inclusion
- lifetime/region preorder constraints
- simple cost/resource bounds
- effect-row inclusion
- structural option-chain and lineage checks
- missing witness in the same transition
- handoff-before-publication in the same transition

The checker should not pretend to solve:

- arbitrary theorem proving
- covert/timing-channel security
- probabilistic information flow
- liveness/fairness theorem
- distributed consensus correctness
- production packaging/host guarantees

Those produce residual obligations for theorem/model-check/runtime-policy.

---

## 2. Built-in syntax vs user-defined terms

This section must be reflected in the documentation. It prevents the exact misunderstanding that motivated this handoff.

### 2.1 Structural syntax controlled by the language

| Word | Layer | Meaning |
|---|---|---|
| `module` | syntax organization | defines a compilation unit |
| `index` | type/index layer | declares finite preorder/lattice/etc. |
| `policy` | type/index layer | declares derived rules from index theories |
| `principal` | static + runtime boundary | declares an actor/principal and its static attributes |
| `resource` | type/ownership | declares a resource value or capability |
| `effect` | effect layer | declares an external or semantic effect |
| `place` | core semantics | event/state/rollback/ownership locus |
| `option` | guarded option chain | declares an access candidate |
| `chain` | guarded option chain | names an option chain |
| `fallback` | guarded option chain | declares fallback edge |
| `lineage` | static evidence | same semantic lineage evidence for fallback edge |
| `perform` | effect request | invokes an effect through target/chain |
| `via` | effect request | selects chain/reference |
| `require` | contract | precondition / requirement |
| `ensure` | contract | postcondition / guarantee |
| `atomic_cut` | core semantics | local rollback frontier cut only |
| `transition` | order/handoff surface | structured ordered transition |
| `stage` | order/handoff surface | named step in transition |
| `publish` | order relation | publication event |
| `observe` | order relation | observation event |
| `handoff` | order relation | authority/resource handoff |
| `witness` | evidence relation | proof/receipt/reference to prior event |
| `model` | model-check layer | finite-state model |
| `property` | model-check layer | property to verify |

### 2.2 User-defined in the examples

The following are not built-ins and must be declared in code:

```text
FingerprintAuthority
SecurityLabel
CostBudget
CaptureScope
Observer
Holder
Releaser
Admin
Public
UserSecret
KeyMaterial
Alice
Bob
Carol
SecretKey
Fingerprint
RoomDigest
dice_owner
draw
authority_rng
room_history
```

---

## 3. Clean sample suite overview

The old samples are not reused. The following new sample suite should be created, for example under:

```text
samples/clean-near-end/
```

Suggested files:

```text
00_index_theories.mir
01_authorized_declassification_ok.mir
02_unauthorized_declassification_rejected.mir
03_label_flow_rejected.mir
04_capture_escape_rejected.mir
05_cost_bound_rejected.mir
06_order_handoff_ok.mir
07_order_handoff_missing_witness_rejected.mir
08_order_handoff_before_publish_rejected.mir
09_model_mutex_sc_ok.mir
10_model_mutex_relaxed_bad.mir
11_model_authoritative_room_properties.mir
12_memory_order_reinterpretation_ok.mir
13_memory_order_reinterpretation_bad.mir
```

Each sample must have:

- source code
- expected static verdict
- expected runtime/prototype verdict
- expected theorem/model-check carrier, if applicable
- explanation of which terms are built-in vs user-defined
- negative sample showing a plausible but invalid pattern

---

## 4. Shared index/policy definitions

This file defines the reusable user-level theories. These are not built-ins.

### File: `00_index_theories.mir`

```mir
module CleanNearEnd.IndexTheories

// Security labels are user-defined.
// The compiler only knows how to solve finite_lattice constraints once this is declared.
index label SecurityLabel {
  element Public
  element UserSecret
  element KeyMaterial

  order Public <= UserSecret
  order UserSecret <= KeyMaterial

  join
  meet
  law finite_lattice
}

// Authority levels are user-defined.
// This is a preorder/chain for the sample. A different application may define another one.
index authority FingerprintAuthority {
  element Observer
  element Holder
  element Releaser
  element Admin

  order Observer <= Holder
  order Holder <= Releaser
  order Releaser <= Admin

  law finite_preorder
}

// Capture scopes are user-defined finite sets.
// The checker can solve subset constraints.
index capture CaptureScope {
  element RoomHistory
  element SessionToken
  element SecretKeyStore

  law finite_powerset
}

// Lifetime regions are user-defined preorder elements.
// The names are sample-specific.
index lifetime Region {
  element Step
  element Turn
  element Session

  order Step <= Turn
  order Turn <= Session

  law finite_preorder
}

// Cost is a resource-bound index.
// First cut supports only simple counters.
index cost CostBudget {
  counter cpu_steps
  counter remote_calls
  counter writes

  law pointwise_natural_bound
}

// Policy is user-defined.
// It connects SecurityLabel and FingerprintAuthority.
policy FingerprintReleasePolicy {
  import SecurityLabel
  import FingerprintAuthority

  permit declassify KeyMaterial -> Public
    when authority >= Releaser

  permit ordinary_flow from_label -> to_label
    when from_label <= to_label
}
```

### Meaning

The compiler should not know `KeyMaterial`, `Releaser`, or `Public` beforehand. It only knows:

- how to parse an `index`
- how to build a finite preorder/lattice
- how to solve declared constraints
- how to generate residual obligations when constraints exceed the decidable fragment

### Expected check

```json
{
  "static_verdict": "valid",
  "index_theories": [
    {"name": "SecurityLabel", "kind": "finite_lattice"},
    {"name": "FingerprintAuthority", "kind": "finite_preorder"},
    {"name": "CaptureScope", "kind": "finite_powerset"},
    {"name": "Region", "kind": "finite_preorder"},
    {"name": "CostBudget", "kind": "pointwise_natural_bound"}
  ],
  "policy": "FingerprintReleasePolicy"
}
```

---

## 5. Problem 1 samples: type checking, theorem bridge, model-check reserve

### 5.1 Authorized declassification succeeds

#### File: `01_authorized_declassification_ok.mir`

```mir
module CleanNearEnd.AuthorizedDeclassification

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Releaser

resource secret_key : PrivateKey
  label KeyMaterial
  linear
  capture SecretKeyStore
  lifetime Session

resource public_board : Board
  label Public
  lifetime Session

effect derive_fingerprint
  input key : PrivateKey @ KeyMaterial
  output fp : Fingerprint @ KeyMaterial
  cost <= { cpu_steps: 20, remote_calls: 0, writes: 0 }

effect publish_fingerprint
  input fp : Fingerprint @ Public
  output unit : Unit
  effect publish
  cost <= { cpu_steps: 5, remote_calls: 0, writes: 1 }

place root {
  place room {
    place verifier_boundary {

      option release_path
        on public_board
        capability write
        lease live
        admit authority(Alice) >= FingerprintAuthority.Releaser

      chain fingerprint_release = release_path

      perform derive_fingerprint via fingerprint_release
        require write
        ensure result_label = SecurityLabel.KeyMaterial

      atomic_cut

      perform publish_fingerprint via fingerprint_release
        require write
        require can_declassify(SecurityLabel.KeyMaterial, SecurityLabel.Public)
        ensure visible_to(public_board, room_members)
    }
  }
}
```

#### Expected static result

```json
{
  "sample": "01_authorized_declassification_ok",
  "static_verdict": "valid",
  "constraints_solved": [
    "authority(Alice) >= FingerprintAuthority.Releaser",
    "can_declassify(KeyMaterial, Public)",
    "cost(derive_fingerprint) <= declared_bound",
    "cost(publish_fingerprint) <= declared_bound",
    "secret_key is linear and not exported"
  ],
  "residual_obligations": []
}
```

#### Expected runtime/prototype result

```json
{
  "entered_evaluation": true,
  "terminal_outcome": "success"
}
```

#### Why this sample matters

This sample demonstrates that declassification is not magic. It succeeds because:

1. `SecurityLabel` is defined.
2. `FingerprintAuthority` is defined.
3. `FingerprintReleasePolicy` defines the permission.
4. Alice is declared as `Releaser`.
5. The checker solves `authority(Alice) >= Releaser`.

No domain predicate is secretly built in.

---

### 5.2 Unauthorized declassification is rejected

#### File: `02_unauthorized_declassification_rejected.mir`

```mir
module CleanNearEnd.UnauthorizedDeclassification

use CleanNearEnd.IndexTheories

principal Bob : FingerprintAuthority.Holder

resource secret_key : PrivateKey
  label KeyMaterial
  linear
  capture SecretKeyStore
  lifetime Session

resource public_board : Board
  label Public
  lifetime Session

effect derive_fingerprint
  input key : PrivateKey @ KeyMaterial
  output fp : Fingerprint @ KeyMaterial

effect publish_fingerprint
  input fp : Fingerprint @ Public
  output unit : Unit
  effect publish

place root {
  place room {
    place verifier_boundary {
      option holder_path
        on public_board
        capability write
        lease live
        admit authority(Bob) >= FingerprintAuthority.Holder

      chain fingerprint_release = holder_path

      perform derive_fingerprint via fingerprint_release
        require write
        ensure result_label = SecurityLabel.KeyMaterial

      atomic_cut

      perform publish_fingerprint via fingerprint_release
        require write
        require can_declassify(SecurityLabel.KeyMaterial, SecurityLabel.Public)
        ensure visible_to(public_board, room_members)
    }
  }
}
```

#### Expected static result

```json
{
  "sample": "02_unauthorized_declassification_rejected",
  "static_verdict": "invalid",
  "reason_family": "authority_preorder_constraint_failed",
  "constraints_failed": [
    "authority(Bob) >= FingerprintAuthority.Releaser"
  ],
  "entered_evaluation": false
}
```

#### Why this sample matters

Bob is a `Holder`, not a `Releaser`. The sample proves that holding a value and having authority to declassify it are different.

This replaces the old symbolic predicate style:

```text
declassify_authority(player_a)
```

with a user-defined preorder constraint.

---

### 5.3 Direct high-to-low flow is rejected

#### File: `03_label_flow_rejected.mir`

```mir
module CleanNearEnd.LabelFlowRejected

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Releaser

resource fp : Fingerprint
  label KeyMaterial
  lifetime Session

resource public_board : Board
  label Public
  lifetime Session

effect write_public_board
  input x : Fingerprint @ Public
  output unit : Unit
  effect publish

place root {
  place room {
    perform write_public_board
      require input_label(fp) <= SecurityLabel.Public
      ensure visible_to(public_board, room_members)
  }
}
```

#### Expected static result

```json
{
  "sample": "03_label_flow_rejected",
  "static_verdict": "invalid",
  "reason_family": "label_flow_constraint_failed",
  "constraints_failed": [
    "KeyMaterial <= Public"
  ],
  "entered_evaluation": false
}
```

#### Why this sample matters

This rejects a common mistake:

```text
“I have the data, therefore I can publish it.”
```

The checker requires either:

- ordinary flow `KeyMaterial <= Public`, which is false, or
- explicit declassification under policy, which is absent.

---

### 5.4 Capture escape is rejected

#### File: `04_capture_escape_rejected.mir`

```mir
module CleanNearEnd.CaptureEscapeRejected

use CleanNearEnd.IndexTheories

resource session_token : Token
  label UserSecret
  capture SessionToken
  lifetime Step
  linear

effect make_room_callback
  input token : Token @ UserSecret
  output callback : Callback
  captures SessionToken
  lifetime Session

place root {
  place room {
    perform make_room_callback
      require captures(result) <= CaptureScope.RoomHistory
      ensure callback_registered(result)
  }
}
```

#### Expected static result

```json
{
  "sample": "04_capture_escape_rejected",
  "static_verdict": "invalid",
  "reason_family": "capture_escape",
  "constraints_failed": [
    "SessionToken <= RoomHistory"
  ],
  "entered_evaluation": false
}
```

#### Why this sample matters

This is stronger than ordinary taint alone. It rejects the pattern:

```text
ephemeral token captured by a callback that outlives its scope
```

The sample shows that capture/lifetime is part of the type/index layer, not a runtime-only policy.

---

### 5.5 Simple cost bound is rejected

#### File: `05_cost_bound_rejected.mir`

```mir
module CleanNearEnd.CostBoundRejected

use CleanNearEnd.IndexTheories

effect local_update
  output unit : Unit
  cost <= { cpu_steps: 10, remote_calls: 0, writes: 1 }

effect fetch_remote_bonus
  output bonus : Bonus
  effect remote_call
  cost <= { cpu_steps: 20, remote_calls: 1, writes: 0 }

place root {
  place room {
    perform local_update
      require total_cost <= { cpu_steps: 40, remote_calls: 0, writes: 2 }

    perform fetch_remote_bonus
      require total_cost <= { cpu_steps: 40, remote_calls: 0, writes: 2 }
  }
}
```

#### Expected static result

```json
{
  "sample": "05_cost_bound_rejected",
  "static_verdict": "invalid",
  "reason_family": "cost_bound_constraint_failed",
  "constraints_failed": [
    "remote_calls: 1 <= 0"
  ],
  "entered_evaluation": false
}
```

#### Why this sample matters

It demonstrates a practical compile-time resource guarantee:

```text
This code path must not call remote services.
```

The checker should reject code that violates it.

---

## 6. Problem 2 samples: order/handoff/memory-order reinterpretation

### 6.1 Valid edge-row handoff

#### File: `06_order_handoff_ok.mir`

```mir
module CleanNearEnd.OrderHandoffOk

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

effect authority_rng_roll
  output draw : DiceDraw
  effect rng
  cost <= { cpu_steps: 5, remote_calls: 0, writes: 0 }

place root {
  place room {
    transition turn_handoff(owner = Alice) {

      stage roll:
        draw <- perform authority_rng_roll

      stage publish:
        pub <- publish draw
          scope room
          ensure published(draw)

      stage handoff:
        handoff dice_owner Alice -> Bob
          after publish(draw)
          requires witness(pub)
          ensure owner(dice_owner) = Bob
    }
  }
}
```

#### Expected static result

```json
{
  "sample": "06_order_handoff_ok",
  "static_verdict": "valid",
  "relations": [
    ["program_order", "roll", "publish"],
    ["publication_order", "roll.draw", "publish.pub"],
    ["witness_order", "publish.pub", "handoff"],
    ["scoped_happens_before", "roll.draw", "handoff"]
  ],
  "constraints_solved": [
    "handoff has witness(pub)",
    "handoff occurs after publish(draw)",
    "owner authority present"
  ]
}
```

#### Expected runtime/prototype result

```json
{
  "terminal_outcome": "success",
  "current_owner": "Bob",
  "visible_history": ["draw", "publish(draw)", "handoff(Alice -> Bob)"]
}
```

---

### 6.2 Missing witness is rejected

#### File: `07_order_handoff_missing_witness_rejected.mir`

```mir
module CleanNearEnd.OrderHandoffMissingWitness

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

effect authority_rng_roll
  output draw : DiceDraw
  effect rng

place root {
  place room {
    transition turn_handoff(owner = Alice) {

      stage roll:
        draw <- perform authority_rng_roll

      stage publish:
        pub <- publish draw
          scope room

      stage handoff:
        handoff dice_owner Alice -> Bob
          after publish(draw)
          ensure owner(dice_owner) = Bob
    }
  }
}
```

#### Expected static result

```json
{
  "sample": "07_order_handoff_missing_witness_rejected",
  "static_verdict": "invalid",
  "reason_family": "missing_handoff_witness",
  "constraints_failed": [
    "requires witness(pub)"
  ],
  "entered_evaluation": false
}
```

---

### 6.3 Handoff before publication is rejected

#### File: `08_order_handoff_before_publish_rejected.mir`

```mir
module CleanNearEnd.OrderHandoffBeforePublish

use CleanNearEnd.IndexTheories

principal Alice : FingerprintAuthority.Admin
principal Bob   : FingerprintAuthority.Holder

resource dice_owner : Player
  authority_scope room
  lifetime Session

effect authority_rng_roll
  output draw : DiceDraw
  effect rng

place root {
  place room {
    transition turn_handoff(owner = Alice) {

      stage roll:
        draw <- perform authority_rng_roll

      stage handoff:
        handoff dice_owner Alice -> Bob
          requires witness(pub)
          ensure owner(dice_owner) = Bob

      stage publish:
        pub <- publish draw
          scope room
    }
  }
}
```

#### Expected static result

```json
{
  "sample": "08_order_handoff_before_publish_rejected",
  "static_verdict": "invalid",
  "reason_family": "handoff_before_publication",
  "constraints_failed": [
    "publish(draw) must precede handoff that requires witness(pub)"
  ],
  "entered_evaluation": false
}
```

---

## 7. Model checking: mutex / memory-order family

The image supplied by the user shows a classic class of problem: a mutex algorithm can look correct locally but fail or deadlock under certain interleavings or memory-order assumptions.

The key lesson:

```text
This is not primarily a type-checking problem.
It is a protocol/interleaving/memory-model problem.
It belongs to model-check second line.
```

### 7.1 Sequentially consistent Peterson-style model

#### File: `09_model_mutex_sc_ok.mir`

```mir
module CleanNearEnd.MutexPetersonSC

model PetersonSC {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true
    turn <- B
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true
    turn <- A
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model sequential_consistency

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

#### Expected model-check result

```json
{
  "model": "PetersonSC",
  "property": "mutual_exclusion",
  "result": "pass",
  "checked_under": "sequential_consistency"
}
```

### 7.2 Relaxed memory without publication/observation edges

#### File: `10_model_mutex_relaxed_bad.mir`

```mir
module CleanNearEnd.MutexRelaxedBad

model PetersonRelaxedNoPublication {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    flag[A] <- true        // write may remain unobserved by B
    turn <- B              // write may be reordered/observed late
    wait while flag[B] == true and turn == B
    critical A
    flag[A] <- false
  }

  process B {
    flag[B] <- true        // write may remain unobserved by A
    turn <- A              // write may be reordered/observed late
    wait while flag[A] == true and turn == A
    critical B
    flag[B] <- false
  }

  memory_model relaxed_without_publication_observation

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

#### Expected model-check result

```json
{
  "model": "PetersonRelaxedNoPublication",
  "property": "mutual_exclusion",
  "result": "fail",
  "counterexample": [
    "A buffers flag[A] <- true",
    "B buffers flag[B] <- true",
    "A observes flag[B] == false",
    "B observes flag[A] == false",
    "A enters critical",
    "B enters critical"
  ]
}
```

### 7.3 Mir reinterpretation with publish/observe

#### File: `12_memory_order_reinterpretation_ok.mir`

```mir
module CleanNearEnd.MutexMirPublishObserve

model MutexWithPublication {
  actor A
  actor B

  shared flag[A] : Bool = false
  shared flag[B] : Bool = false
  shared turn : Actor = A

  process A {
    publish flag[A] <- true
    publish turn <- B
    observe flag[B]
    observe turn
    wait while flag[B] == true and turn == B
    critical A
    publish flag[A] <- false
  }

  process B {
    publish flag[B] <- true
    publish turn <- A
    observe flag[A]
    observe turn
    wait while flag[A] == true and turn == A
    critical B
    publish flag[B] <- false
  }

  property mutual_exclusion:
    never (in_critical(A) and in_critical(B))
}
```

#### Expected result

```json
{
  "model": "MutexWithPublication",
  "property": "mutual_exclusion",
  "result": "pass",
  "relations_used": [
    "publication_order",
    "observation_order",
    "scoped_happens_before"
  ]
}
```

This sample is not claiming that the source language should expose C++ `memory_order` tokens. It shows how the same idea is represented using Mir’s high-level relation family.

---

## 8. Reference sanity harness

This section is a small Python sanity harness used to demonstrate the intended checks independently of the repo. CodeX should not treat it as the real implementation. It is a reference explanation only.

### Output from the reference harness

```text
## TYPE_IFC_FINITE_INDEX
PASS          type-valid-public-flow :: flow ok: Public <= Public
FAIL_EXPECTED type-invalid-secret-to-public :: flow denied: Secret !<= Public
FAIL_EXPECTED type-invalid-keymaterial-to-secret-without-declass :: flow denied: KeyMaterial !<= Secret
FAIL_EXPECTED type-invalid-declassify-without-authority :: missing authority for declassification: KeyMaterial -> Public
PASS          type-valid-declassify-with-authority :: explicit declassification ok: KeyMaterial -> Public
PASS          type-valid-cost-bound :: cost ok: 42 <= 100
FAIL_EXPECTED type-invalid-cost-bound :: cost bound exceeded: 130 > 100
FAIL_EXPECTED type-invalid-capture-escape :: capture escape denied: ['secret_key'] not allowed
PASS          type-valid-capture :: capture ok: ['db_conn'] <= ['db_conn']
SUMMARY passed=4 rejected=5 total=9

## ORDER_HANDOFF_MEMORY_REINTERPRETATION
PASS          order-valid-handoff-after-witness :: handoff has prior publish/witness path
FAIL_EXPECTED order-invalid-handoff-without-witness :: handoff lacks prior publish/witness path
FAIL_EXPECTED order-invalid-handoff-without-witness :: handoff lacks prior publish/witness path
SUMMARY passed=1 rejected=2 total=3

## MODEL_CHECK_SECOND_LINE
PASS          mc-valid-no-double-owner :: single owner slot preserved
FAIL_EXPECTED mc-invalid-double-owner :: double owner at ['bad']
PASS          mc-valid-late-join-sees-published-history :: late join sees draw#1=4
FAIL_EXPECTED mc-invalid-late-join-missing-history :: late join missing draw#1=4
PASS          mc-valid-stale-reconnect-fail-then-refresh :: stale reconnect rejected and refresh required
FAIL_EXPECTED mc-invalid-stale-reconnect-silent-merge :: stale reconnect silently accepted
PASS          mc-valid-provider-no-room-commit :: provider returned draw/receipt only
FAIL_EXPECTED mc-invalid-provider-committed-room-state :: RNG provider mutated/committed room state
SUMMARY passed=4 rejected=4 total=8

## THEOREM_PREFLIGHT
PASS          theorem-preflight-1 :: obligation accepted for notebook/preflight: handoff_requires_witness
PASS          theorem-preflight-2 :: obligation accepted for notebook/preflight: ifc_no_high_to_low
FAIL_EXPECTED theorem-preflight-3 :: missing evidence_refs
FAIL_EXPECTED theorem-preflight-4 :: unsupported obligation kind: unknown_global_magic
SUMMARY passed=2 rejected=2 total=4

```

The important point is that each negative sample is rejected for a different reason:

- high-to-low flow
- missing declassification authority
- cost bound exceeded
- capture escape
- missing handoff witness
- handoff before publication
- stale reconnect silently accepted
- RNG provider committing room state
- theorem obligation with missing evidence

---

## 9. Implementation sequence for CodeX

1. Add clean sample suite without reusing old source code.
2. Add user-defined finite index theory parser/helper carrier.
3. Add finite solver for:
   - preorder
   - lattice
   - powerset inclusion
   - lifetime preorder
   - simple cost bounds
4. Add constraint generation from:
   - resource declarations
   - principal declarations
   - policy declarations
   - `perform`
   - `require`
   - `ensure`
   - `handoff`
   - `witness`
5. Add typed sample checks for samples 01–05.
6. Add order/handoff checker for samples 06–08.
7. Add model-check carrier for samples 09–13.
8. Add Lean foundation / proof skeleton updates.
9. Add docs explaining which words are built-in and which are user-defined.
10. Run validation and write report.

---

## 10. Validation commands for CodeX

Run actual repo commands if available.

```bash
python3 scripts/current_l2_guided_samples.py list
python3 scripts/current_l2_guided_samples.py smoke-all
python3 scripts/current_l2_guided_samples.py closeout
```

Run source sample checks for the new suite, e.g.:

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample samples/clean-near-end/01_authorized_declassification_ok.mir --format json
```

Run negative cases:

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- \
  check-source-sample samples/clean-near-end/02_unauthorized_declassification_rejected.mir --format json
```

Run model-check second-line:

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

Run Lean if available:

```bash
source "$HOME/.elan/env"
lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

Do not fake output. If a command does not exist, create or update the correct helper, or report the exact blocker.

---

## 11. Final docs requirements

Update or create:

- `specs/10-open-questions.md`
- relevant `specs/examples/*`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `progress.md`
- `tasks.md`
- human-facing docs under `docs/research_abstract/`

The human-facing docs must explicitly say:

```text
Old symbolic predicates were placeholders.
The new clean samples define authority, label, capture, lifetime, and cost theories in code.
No domain authority is secretly built in.
```

---

## 12. Completion criterion

This layer is complete when:

1. New clean samples pass/fail as expected.
2. Old samples may be obsolete; that is allowed.
3. Every domain word in the clean samples is either defined or imported.
4. Static type/index checks reject the invalid security/capture/cost examples.
5. Order/handoff checks reject missing witness and handoff-before-publication.
6. Model-check second-line catches the mutex/memory-order counterexample and room protocol counterexamples.
7. Lean foundation checks at least the first finite-index facts.
8. Docs explain built-in vs user-defined vocabulary.
9. No final public grammar/API/verifier claim is made.
10. CodeX report gives validation results and remaining gates.

---

## 13. Final response format required from CodeX

Return:

1. files added
2. files changed
3. clean sample suite list
4. built-in vocabulary list
5. user-defined vocabulary list
6. type/index theory implementation status
7. finite solver status
8. successful sample outputs
9. rejected sample outputs
10. model-check outputs
11. Lean proof/skeleton status
12. docs updated
13. tests run
14. remaining blockers
15. report path
