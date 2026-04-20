# Codex repo-local near-end completion handoff after FAQ 11

**Target reading:** 2026-04-20 current state after `faq_011.md`  
**Audience:** CodeX / GPT-5.4 xhigh + sub-agents  
**Purpose:** Give CodeX enough precise, source-aware, self-contained information to drive the current Mir language layer to **repo-local near-end completion**: executable representative corpus, first strong typing finite-index layer, theorem-first Lean skeleton/preflight, model-check second-line closeout, order/handoff helper-local surfaces, authoritative-room practical scenario, validation commands, expected output shapes, documentation consistency, and remaining public gates.

---

## 0. Executive summary

This handoff supersedes the operational direction of the previous handoff only where FAQ 11 is fresher. It does **not** erase prior context; Appendix B preserves the previous FAQ 10 once-completion handoff in full, and Appendix C preserves FAQ 11 in full.

The goal is **repo-local near-end completion**, not full final-public completion.

```text
Repo-local near-end completion =
  current L2 runnable corpus passes
  + corrected prototype corpus p01...p16 passes
  + guided samples pass
  + emitted artifacts can be regenerated and compared
  + theorem-first / Lean representative floor remains valid
  + model-check second-line carrier is closed or narrowed to one explicit blocker
  + order/handoff helper-local source surfaces demonstrate valid and invalid cases
  + first strong typing finite-index layer demonstrates IFC / taint / capture / lifetime / simple cost checks
  + authoritative-room first scenario runs through CLI/test/artifact floor
  + docs/specs/plan/progress/tasks/report are consistent
  + final public grammar/API/verifier/tooling gates are explicitly deferred
```

Do **not** claim these as complete unless a later task proves them:

- final parser grammar
- final public parser API
- final public checker API
- final public runtime API
- final public verifier contract
- production theorem prover binding
- production model-check binding
- installed binary / packaging
- FFI / game-engine adapter
- exhaustive shared-space catalog
- broader upper-layer application target beyond authoritative-room first scenario

The correct current statement is:

```text
The repo is past abstract research. It has runnable and machine-checkable floors.
It is not final-public complete.
The remaining active self-driven line is model-check-second-line plus closeout consistency.
This task should close repo-local near-end, not final public language/product.
```

---

## 1. Repository discipline and authority hierarchy

1. `specs/` is normative.
2. `plan/` is repository memory.
3. `docs/reports/` is historical evidence.
4. `progress.md` and `tasks.md` are current snapshots.
5. FAQ files are current explanations, not normative specs.
6. Never factify `OPEN` / `FUTURE WORKSTREAM` / `COMPARISON`.
7. Every non-trivial task must write a report.
8. If a relevant mirror is not updated, the report must say why.
9. `plan/91-maintenance-rules.md` must be respected.

Current foundational constraints to preserve:

- Mir is the main semantic core.
- Mirrorea / Typed-Effect Wiring Platform / PrismCascade / shared-space remain separable tracks.
- Mir-0 keeps `atomic_cut` as **place-local finalizing cut** only.
- `atomic_cut` is not global consistent cut, not `seq_cst`, not durable commit.
- `barrier` and `durable_cut` remain later cut families.
- fallback is a guarded option chain.
- same-lineage fallback chain is left-to-right monotone degradation.
- no re-promotion.
- explicit edge-row family remains the current companion notation style.
- final grammar and final public API remain open.

---

## 2. FAQ 11 answer fully integrated

### 2.1 One-paragraph answer

FAQ 11 says the repo has **genuinely progressed** since FAQ 10. The reserve integration lane is now more isolated: `auditable_authority_witness` and `delegated_rng_service` can be emitted / materialized as independent repo-local summary packages. The live self-driven queue has narrowed: the active self-driven package is effectively `model-check-second-line`. The repo is no longer docs-only; it has parser-free current L2, compile-ready minimal actualization, authored sixteen, corrected prototypes `p01...p16`, theorem/model-check/order-handoff/shared-space helper-local bridges, representative Lean sample set, and phase6 parser-side narrow carrier actualization.

But FAQ 11 also says the repo is **not final-public complete**. The missing items remain final parser grammar, final public parser/checker/runtime APIs, final public verifier contract, production theorem/model-check binding, final public witness/provider/artifact schema, packaging / host target, and exhaustive shared-space catalog.

### 2.2 What genuinely progressed after FAQ 10

FAQ 11 identifies three actual changes, not mere wording changes:

1. **Reserve integration lane materialized further**
   - `auditable_authority_witness` reserve package summary index actualization
   - `delegated_rng_service` reserve package summary index actualization
   - `emit-reserve auditable-authority-witness`
   - `emit-reserve delegated-rng-service`

2. **Current live queue narrowed**
   - active self-driven package = `model-check-second-line`
   - reserve integration reopen / later mixed gate / true user-spec hold remain, but are no longer the active self-driven core

3. **Problem 2 first line and reserve strengthening line are less entangled**
   - `p07/p08` = first actual adoption evidence
   - `p09` = delegated RNG reserve
   - `p13/p14` = negative static-stop pair
   - reserve packages now have separate summary indices

### 2.3 What is already done

The following floors are reached or largely closed:

| Floor | Current state | Meaning |
|---|---|---|
| Compare floor | closed | current first line / retained alternatives / stop lines are compared |
| Actual adoption floor | closed | recommendations exist as actual packages |
| Helper-local actualization floor | largely closed | theorem/model-check/order-handoff/shared-space/parser-side bridges exist |
| Representative runnable floor | reached | authored sixteen + prototypes + guided samples + smoke / CLI / regression |
| Representative Lean floor | reached | small proof fragment + generated theorem stub corpus |
| Final public language/tool/app floor | not closed | final grammar/API/verifier/tooling/product remains deferred |

### 2.4 Current evidence families

Current runnable / machine-check evidence includes:

- `samples/current-l2/` authored sixteen
- `samples/prototype/` corrected prototypes `p01...p16`
- `scripts/current_l2_guided_samples.py`
  - `list`
  - `show problem1`
  - `run problem1`
  - `matrix problem1`
  - `matrix problem2`
  - `emit-theorem problem1`
  - `emit-scenario problem2`
  - `emit-reserve auditable-authority-witness`
  - `emit-reserve delegated-rng-service`
  - `smoke problem1`
  - `smoke problem2`
  - `smoke-all`
  - `closeout`
- `samples/lean/foundations/`
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2FiniteIndexFirstLayer.lean`
  - `CurrentL2ProofSkeleton.lean`
- `samples/lean/current-l2/` generated theorem stub corpus

---

## 3. Current scope: what this task must complete

### 3.1 In scope

This task should close **repo-local near-end completion**. In practical terms:

1. Close or narrow `model-check-second-line` to one explicit remaining blocker.
2. Ensure guided samples run and are documented.
3. Ensure reserve package summary emitters are reproducible.
4. Ensure theorem-first / Lean representative floor remains valid.
5. Ensure first strong typing finite-index layer has practical examples and negative checks.
6. Ensure memory-order reinterpretation / order-handoff examples include valid and invalid cases.
7. Ensure emitted artifacts are generated and compared.
8. Ensure `docs/`, `specs/`, `plan/`, `progress.md`, `tasks.md`, and reports are consistent.

### 3.2 Out of scope

Do not try to finish:

- final public grammar
- final public parser API
- final public checker API
- final public runtime API
- final public verifier contract
- production theorem/model-check binding
- installed binary / packaging
- FFI / engine adapter
- broad app target
- exhaustive shared-space catalog

### 3.3 Completion statement allowed after this task

Allowed if evidence supports:

```text
Repo-local near-end completion is achieved for current mapped corpus.
Current L2 runnable/prototype/theorem/order-handoff/model-check helper-local floors are consistent.
Final public language/tool/product gates remain deferred.
```

Not allowed:

```text
The language is finished.
The theory is fully solved.
The public verifier is complete.
The app/system is complete.
```

---

## 4. User-authorized defaults for remaining decisions

Use these defaults unless they conflict with stronger source evidence.

### 4.1 Theory spine

- Principal spine = **multimodal dependent core**.
- `λ◯□` = partial basis candidate for stage/later/stable, not full foundation.
- Guarded λ-calculus / MDTT / MTT / Fitch-style multimodal = stronger candidate family.
- Final full calculus public adoption is not required in this tranche.

Recommended conceptual judgement:

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C
```

Where:

```text
Ψ  = mode / index / order theory environment
Γ  = unrestricted / dependent context
Δ  = linear / affine / capability context
A  = ordinary type
m  = mode, e.g. place, stage, visibility, witness
ε  = effect row
C  = generated constraints / proof obligations
```

### 4.2 Strong typing / finite-index layer

- Full dependent type is not required for first public core.
- Type-level partial orders / finite lattices / capture sets / lifetime preorder / simple cost bounds are principal targets.
- First implementation target:
  - finite label lattice
  - capture-set inclusion
  - lifetime / outlives preorder
  - simple resource index / bounded cost
  - effect rows
  - generated proof obligations
- Arbitrary type-level functions, full dependent pattern matching, general theorem proving in the compiler, and full asymptotic inference are out of first cut.

### 4.3 Security / IFC / taint

- Security label model is first-class target.
- `Labeled ℓ A` or equivalent should be supported in prototype / helper-local surface.
- `flows_to` relation must be decidable in first cut.
- pc-label / control-flow influence should be represented at least in examples / checker model.
- Explicit declassification requires authority/capability.
- Covert channels, timing leakage, probabilistic leakage, richer distributed leakage are theorem/model-check/runtime-policy later lines.

### 4.4 Proof assistant and theorem line

- Lean-first.
- Rocq/Iris reserve only if concurrency separation logic becomes necessary.
- TLA+/Apalache-style model checking acceptable for protocol/model-check second line.
- Theorem-first concrete binding may be actualized early in experimental / non-production form.
- Final public verifier contract remains deferred.

### 4.5 Order / handoff / memory-order

- Source principal = high-level relation decomposition.
- Low-level `std::memory_order` / `std::kill_dependency` remains backend/reference family.
- Do not expose C++ memory-order exact surface as primary source syntax.
- Use relation family:

```text
program_order
dependency_order
publication_order
observation_order
witness_order
finalization_order
scoped_happens_before
```

- `authority_serial_transition_family` first.
- `witness_aware_commit_family` second.
- Thread/node parity wording:

```text
thread と node は同じ causal language で書く。
違いは lowering / evidence / transport / failure / durability / policy に残す。
```

### 4.6 Syntax

- Theory-first.
- Principal syntax family = explicit edge-row / vertical continuation.
- Secondary = `stage` / `after` / `witness` stage-block.
- Reserve = authoritative-room `serial` sugar.
- Do not privilege compactness over semantic honesty.

Principal order/handoff companion example:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Stage-block secondary:

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      after publish(draw)
      requires witness(draw)
}
```

Serial sugar reserve:

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

### 4.7 First completion target

- First completion = authoritative-room first scenario + repo-local evidence.
- Packaging / installed binary / FFI / engine adapter not required.
- Broader application target not required.
- Final public parser/checker/runtime API not required.

Acceptance scenario:

```text
Dice / owner handoff / late join visible history / stale reconnect refresh
+ auditable-authority-witness reserve summary
+ delegated-rng-service reserve summary
+ model-check second-line properties
+ theorem-first / Lean representative floor
+ first finite-index typing checks
```

---

## 5. Model-check second-line package

This is the remaining active self-driven package.

### 5.1 Carrier shape

Use existing carrier if compatible; otherwise implement this repo-local shape.

```text
ModelCheckSecondLineCase = {
  case_id: string,
  subject_ref: string,
  property_kind:
    | no_handoff_without_witnessed_publication
    | late_join_sees_published_history
    | stale_reconnect_fails_then_refreshes
    | no_double_owner_in_authoritative_turn
    | delegated_rng_provider_does_not_commit_room_state
    | auditable_authority_witness_minimal_core,
  evidence_refs: [string],
  expected_result: pass | static_stop | fail_closed,
  scenario_ref?: string,
  notes?: string
}
```

### 5.2 Minimal property set

| Property | Meaning | Expected examples |
|---|---|---|
| `no_handoff_without_witnessed_publication` | handoff requires prior publish/witness | `p13`, `p14` fail / static-stop |
| `late_join_sees_published_history` | late join sees published history as past | `p07` pass |
| `stale_reconnect_fails_then_refreshes` | stale reconnect cannot silently merge | `p08` pass |
| `no_double_owner_in_authoritative_turn` | owner slot not duplicated | authoritative-room scenario pass |
| `delegated_rng_provider_does_not_commit_room_state` | provider returns draw/receipt but does not mutate room | `p09` pass |
| `auditable_authority_witness_minimal_core` | witness has required core fields | reserve package pass |

### 5.3 Sample mapping

```text
p07:
  property = late_join_sees_published_history
  expected = pass

p08:
  property = stale_reconnect_fails_then_refreshes
  expected = pass

p09:
  property = delegated_rng_provider_does_not_commit_room_state
  expected = pass

p13:
  property = no_handoff_without_witnessed_publication
  expected = static_stop or fail_closed

p14:
  property = handoff_before_publication rejected
  expected = static_stop or fail_closed

auditable_authority_witness reserve:
  property = auditable_authority_witness_minimal_core
  expected = pass

delegated_rng_service reserve:
  property = delegated_rng_provider_does_not_commit_room_state
  expected = pass
```

### 5.4 Expected emitted artifact shape

Illustrative repo-local JSON shape:

```json
{
  "schema_version": "current-l2-model-check-second-line-v0",
  "cases": [
    {
      "case_id": "p13-no-handoff-without-witnessed-publication",
      "subject_ref": "samples/prototype/p13-dice-late-join-missing-publication-witness.txt",
      "property_kind": "no_handoff_without_witnessed_publication",
      "evidence_refs": ["p13.order_handoff.static_stop"],
      "expected_result": "static_stop",
      "scenario_ref": "authoritative-dice-room",
      "notes": "handoff lacks witnessed publication; must not be accepted"
    }
  ]
}
```

### 5.5 Expected model-check output shape

CodeX must run the actual command and replace this illustrative output with real output in its report.

```text
$ python3 scripts/current_l2_guided_samples.py matrix problem2
problem2 matrix: 6 cases
  p07 late_join_sees_published_history: PASS
  p08 stale_reconnect_fails_then_refreshes: PASS
  p09 delegated_rng_provider_does_not_commit_room_state: PASS
  p13 no_handoff_without_witnessed_publication: STATIC_STOP
  p14 handoff_before_publication: STATIC_STOP
  reserve auditable_authority_witness_minimal_core: PASS
summary: passed=4 static_stop=2 failed=0
```

---

## 6. Problem 1 sample set: type checking / theorem proving / model checking

The user explicitly wants non-trivial and practical examples that demonstrate:

- type checking catches invalid patterns
- theorem proving has meaningful obligations
- model checking catches temporal/protocol invalid patterns
- examples are executable in repo-local helper/CLI form

If existing `p10...p16` cover these, use them. If not, add minimal helper-local prototypes without claiming final public grammar.

### 6.1 Type checking sample A: secret key cannot flow to public

#### Source sample: valid key use

```text
label Public
label UserSecret
label KeyMaterial

order Public <= UserSecret <= KeyMaterial

fn derive_public_key(key: PrivateKey @ KeyMaterial)
  -> PublicKey @ Public
  requires authority(declassify_key_material)
{
  perform derive_public_key on key
    ensure result.label == Public
}
```

Expected: accepted.

#### Source sample: invalid leak

```text
label Public
label UserSecret
label KeyMaterial

order Public <= UserSecret <= KeyMaterial

fn leak_key(key: PrivateKey @ KeyMaterial)
  -> Bytes @ Public
{
  return key.bytes
}
```

Expected: static error.

Expected error shape:

```text
error[ifc_flow_violation]: KeyMaterial does not flow to Public
  source: leak_key.return
  required: KeyMaterial <= Public
  note: explicit declassification with authority(declassify_key_material) is required
```

#### CLI target

If helper exists:

```bash
python3 scripts/current_l2_guided_samples.py run problem1 --case ifc-secret-key-valid
python3 scripts/current_l2_guided_samples.py run problem1 --case ifc-secret-key-leak
```

Expected output shape:

```text
ifc-secret-key-valid: PASS
ifc-secret-key-leak: STATIC_STOP ifc_flow_violation
```

### 6.2 Type checking sample B: taint/sanitization

#### Invalid pattern

```text
label Clean
label TaintedNetwork

order Clean <= TaintedNetwork

fn sql_query(q: Sql @ Clean) -> Rows

fn handle_request(body: Bytes @ TaintedNetwork) -> Rows {
  let q = parse_sql(body)       // q : Sql @ TaintedNetwork
  return sql_query(q)           // invalid
}
```

Expected:

```text
error[taint_flow_violation]: TaintedNetwork does not flow to Clean
  source: sql_query(q)
  suggestion: validate_or_sanitize body before constructing Sql @ Clean
```

#### Valid pattern

```text
fn validate_sql(body: Bytes @ TaintedNetwork)
  -> Sql @ Clean
  requires validator(sql_safe)

fn handle_request(body: Bytes @ TaintedNetwork) -> Rows {
  let q = validate_sql(body)
  return sql_query(q)
}
```

Expected: accepted.

### 6.3 Type checking sample C: capture / capability escape

#### Invalid pattern

```text
capability SecretKeyAccess

fn make_exported_callback(key: PrivateKey @ KeyMaterial)
  -> Fn[Bytes -> Signature] @ Public
{
  return fn(msg: Bytes @ Public) {
    sign(msg, key)    // captures key
  }
}
```

Expected:

```text
error[capture_escape]: closure captures PrivateKey @ KeyMaterial but result scope is Public
  captured: key
  required: captures <= PublicAllowedCaptures
```

#### Valid pattern

```text
fn make_local_signer(key: PrivateKey @ KeyMaterial)
  -> Fn[Bytes -> Signature] captures { key }
{
  return fn(msg: Bytes @ Public) {
    sign(msg, key)
  }
}
```

Expected: accepted if capture set remains local / non-exported.

### 6.4 Type checking sample D: lifetime / lease outlives

Invalid:

```text
fn return_borrowed<'short>(x: Ref<'short, Bytes>) -> Ref<'static, Bytes> {
  return x
}
```

Expected:

```text
error[lifetime_outlives_violation]: 'short does not outlive 'static
```

Valid:

```text
fn use_within_scope<'r>(x: Ref<'r, Bytes>) -> Bytes
  requires current_scope <= 'r
{
  read(x)
}
```

Expected: accepted.

### 6.5 Type checking sample E: simple cost upper bound

Invalid:

```text
fn update_presence(user: User)
  cost <= { remote_calls: 0 }
{
  perform remote_notify on user
}
```

Expected:

```text
error[cost_bound_violation]: remote_calls actual 1 exceeds bound 0
```

Valid:

```text
fn local_update_presence(user: User)
  cost <= { remote_calls: 0 }
{
  update_local_presence_cache(user)
}
```

Expected: accepted.

### 6.6 Theorem proving sample: no rollback across atomic_cut

Use a Lean skeleton and generated theorem stub, not final proof contract.

Core theorem target:

```lean
namespace Mir.CurrentL2

structure Event where
  id : Nat
  kind : String

structure Trace where
  events : List Event

constant HasAtomicCut : Trace -> Prop
constant RollbackCrossesAtomicCut : Trace -> Prop

axiom no_rollback_across_atomic_cut :
  ∀ t : Trace, HasAtomicCut t -> ¬ RollbackCrossesAtomicCut t

end Mir.CurrentL2
```

Expected Lean floor:

```text
Lean skeleton builds with theorem statements / admitted placeholders.
No claim of complete proof unless `sorry`-free proof exists.
```

If negative proof sample is included, use a separate expected-failure test. Do not break normal `lake build`.

Expected report wording:

```text
Lean representative floor: PASS (skeleton/stub corpus builds)
Proof completeness: NOT CLAIMED; theorem statements are placeholders or partial proofs where marked.
```

### 6.7 Theorem proving sample: handoff implies witnessed publication

Theorem statement target:

```lean
namespace Mir.CurrentL2

constant Scenario : Type
constant Published : Scenario -> String -> Prop
constant Witnessed : Scenario -> String -> Prop
constant Handoff : Scenario -> String -> String -> Prop
constant HappensBefore : Scenario -> String -> String -> Prop

axiom handoff_requires_witnessed_publication :
  ∀ s draw owner_to,
    Handoff s draw owner_to ->
    Published s draw ∧ Witnessed s draw

end Mir.CurrentL2
```

Expected use:

- p07 and p08 should generate theorem obligations compatible with this statement family.
- p13/p14 should fail before theorem discharge or produce unsatisfied obligation.

---

## 7. Problem 2 sample set: memory-order reinterpretation / order-handoff

### 7.1 Valid authoritative roll → publish → handoff

Principal source:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Expected relation extraction:

```json
{
  "events": [
    { "id": "roll.draw", "kind": "perform", "provider": "authority_rng" },
    { "id": "publish.draw", "kind": "publish", "after": ["roll.draw"] },
    { "id": "handoff.dice_owner.A_to_B", "kind": "handoff", "after": ["publish.draw"], "requires_witness": ["draw"] }
  ],
  "relations": [
    ["program_order", "roll.draw", "publish.draw"],
    ["publication_order", "roll.draw", "publish.draw"],
    ["witness_order", "publish.draw", "handoff.dice_owner.A_to_B"],
    ["scoped_happens_before", "roll.draw", "handoff.dice_owner.A_to_B"]
  ]
}
```

Expected helper output:

```text
order-handoff-valid: PASS
  extracted events: 3
  witness checks: PASS
  handoff preconditions: PASS
```

### 7.2 Invalid: handoff before publication

```text
roll draw via authority_rng
handoff dice_owner A -> B
  requires witness(draw)
publish draw
```

Expected:

```text
error[handoff_before_publication]: handoff requires witness(draw) but publish(draw) is not in prior scope
  handoff: handoff dice_owner A -> B
  missing prior event: publish(draw)
```

### 7.3 Invalid: publication without witness used for handoff

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
```

Expected depends on chosen first line. Recommended first line requires explicit witness for handoff:

```text
error[missing_handoff_witness]: handoff requires witness(draw)
  suggestion: add `requires witness(draw)`
```

If the implementation treats `publish draw` as creating a default witness, then this case should be moved to a different negative sample:

```text
publish draw as local_only
handoff dice_owner A -> B
  after publish(draw)
```

Expected:

```text
error[visibility_scope_insufficient]: local_only publication cannot justify cross-owner handoff
```

### 7.4 Dependency / consume analogue

This demonstrates `memory_order_consume` reinterpretation without exposing C++ syntax.

Valid dependency-preserving use:

```text
observe owner_ticket from handoff_channel
let owner = owner_ticket.owner
read board_state[owner]
  depends_on owner_ticket
```

Expected relation:

```text
dependency_order(owner_ticket -> board_state[owner])
observation_order(handoff_channel -> owner_ticket)
```

Invalid dependency kill analogue:

```text
observe owner_ticket from handoff_channel
let owner = kill_dependency(owner_ticket.owner)
read board_state[owner]
  depends_on owner_ticket
```

Expected:

```text
error[dependency_killed]: dependency on owner_ticket was explicitly killed before dependent read
  note: add explicit witness/observe barrier if this read must rely on handoff visibility
```

Important: `kill_dependency` here is a conceptual backend/reference analogy. Do not expose C++ exact API as final source surface unless a later public-language decision adopts it.

### 7.5 Stage-block valid sample

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      after publish(draw)
      requires witness(draw)
}
```

Expected:

```text
stage-block-handoff-turn: PASS
  stages: roll -> publish -> handoff
  extracted obligations: 2
    after publish(draw): PASS
    witness(draw): PASS
```

### 7.6 Stage-block invalid sample

```text
transition handoff_turn(owner = A) {
  stage handoff:
    change_owner dice_owner to B
      requires witness(draw)

  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw
}
```

Expected:

```text
error[stage_order_violation]: handoff stage references draw before roll/publish stage
```

---

## 8. Authoritative-room first scenario

Use authoritative-room first scenario as first completion target.

### 8.1 Scenario: valid dice handoff with late join

Pseudo-source:

```text
room DiceRoom profile authoritative {
  authority room_authority
  state dice_owner : Player = A
  state history : PublishedHistory

  transition roll_and_handoff(owner = A, next = B) {
    stage roll:
      draw <- perform via authority_rng
        ensure draw in 1..6

    stage publish:
      publish draw to history

    stage handoff:
      change_owner dice_owner to B
        after publish(draw)
        requires witness(draw)
  }

  on late_join(C) {
    observe history
    require sees_published_history(C)
  }
}
```

Expected:

```text
DiceRoom.valid_roll_handoff_late_join: PASS
  roll published before handoff
  handoff witnessed
  late join sees draw in published history
```

### 8.2 Invalid scenario: stale reconnect silent merge

Pseudo-source:

```text
room DiceRoom profile authoritative {
  stale_reconnect(B, old_epoch)
  merge_without_refresh(B)
}
```

Expected:

```text
error[stale_reconnect_requires_refresh]: stale reconnect cannot silently merge
  action: fail_then_refresh
```

### 8.3 Delegated RNG reserve scenario

```text
transition roll_with_delegated_rng(owner = A) {
  stage request:
    request draw from delegated_rng_service

  stage receipt:
    receive draw_result, provider_receipt

  stage commit:
    authority_commit draw_result
      requires receipt(provider_receipt)

  stage publish:
    publish draw_result
}
```

Expected:

```text
delegated_rng_service: PASS
  provider returned draw_result
  provider did not mutate room_state
  authority committed and published
```

Invalid:

```text
stage provider_mutation:
  delegated_rng_service.commit room_state
```

Expected:

```text
error[provider_commits_room_state]: delegated RNG provider must not mutate authoritative room state
```

### 8.4 Auditable authority witness reserve scenario

Valid witness core:

```json
{
  "witness_kind": "authority_draw",
  "action_ref": "roll_and_handoff#42",
  "draw_slot": "dice_roll",
  "draw_result": 4,
  "authority_ref": "room_authority"
}
```

Expected:

```text
auditable_authority_witness_minimal_core: PASS
```

Invalid missing core field:

```json
{
  "witness_kind": "authority_draw",
  "action_ref": "roll_and_handoff#42",
  "draw_slot": "dice_roll"
}
```

Expected:

```text
error[witness_core_missing_field]: draw_result is required
```

---

## 9. Validation command suite

CodeX must run actual commands and report actual results. Do not fake.

### 9.1 Guided sample commands

```bash
python3 scripts/current_l2_guided_samples.py list
python3 scripts/current_l2_guided_samples.py show problem1
python3 scripts/current_l2_guided_samples.py run problem1
python3 scripts/current_l2_guided_samples.py matrix problem1
python3 scripts/current_l2_guided_samples.py matrix problem2
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
python3 scripts/current_l2_guided_samples.py smoke problem1
python3 scripts/current_l2_guided_samples.py smoke problem2
python3 scripts/current_l2_guided_samples.py smoke-all
python3 scripts/current_l2_guided_samples.py closeout
```

### 9.2 Current L2 runtime / CLI / regression

```bash
cargo test -p mir-runtime --test current_l2_source_sample_runner
cargo test -p mir-runtime --test current_l2_operational_cli
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression
```

### 9.3 Expected additional test families

Run any existing equivalents of:

```bash
cargo test -p mir-runtime --test current_l2_verifier_preview_alignment
cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor
cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor
cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization
cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight
cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring
cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization
cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface
cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface
```

### 9.4 Lean validation

If Lean/Lake setup exists:

```bash
cd samples/lean
lake build
```

or repo-specific command. If unavailable:

```text
Lean toolchain unavailable; skeleton unvalidated in this environment.
```

### 9.5 Expected closeout output shape

CodeX must replace this illustrative block with actual command output.

```text
closeout summary:
  guided samples: PASS
  problem1 matrix: PASS
  problem2 matrix: PASS
  emitted theorem artifacts: PASS
  emitted scenario artifacts: PASS
  reserve auditable-authority-witness: PASS
  reserve delegated-rng-service: PASS
  model-check second-line: PASS
  cargo source sample runner: PASS
  cargo operational CLI: PASS
  regression inventory: PASS
  regression: PASS
  Lean representative floor: PASS or UNAVAILABLE with reason
```

---

## 10. Docs / plan / report updates required

Update as relevant:

- `specs/00-document-map.md`
- relevant `specs/examples/*`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md` only if evidence supports a real decision row
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `Documentation.md`
- relevant `docs/research_abstract/*`
- new `docs/reports/<next>-repo-local-near-end-completion-after-faq011.md`

Rules:

- `specs/` remains normative.
- `plan/` remains repository memory.
- FAQ remains current explanation.
- Do not make final public grammar/API/verifier claims.
- If a file is not updated, say why in the report.

---

## 11. Final report requirements for CodeX

Final CodeX response/report must contain:

1. updated files
2. new files
3. unchanged relevant files with reason
4. validation commands and actual results
5. model-check second-line status
6. guided sample closeout status
7. reserve package closeout status
8. Problem 1 status
9. Problem 2 status
10. first strong typing finite-index status
11. Lean / theorem-first status
12. memory-order reinterpretation / order-handoff status
13. remaining mixed gates
14. remaining true user-spec gates
15. whether repo-local near-end completion is achieved
16. whether final public completion remains deferred
17. report path

Allowed final wording if successful:

```text
Repo-local near-end completion for the current mapped layer is achieved.
Final public language/tool/product completion remains explicitly deferred.
```

---

## 12. Do-not-do list

Do not:

- claim full language completion
- claim final public verifier completion
- claim final public grammar
- claim production theorem/model-check binding
- claim installed packaging / host integration completion
- collapse `atomic_cut` into global consistent cut / `seq_cst` / durable commit
- expose C++ memory-order exact surface as source principal
- treat `λ◯□` as full foundation
- merge all candidate theories flatly
- remove separation between checker / theorem / model-check / runtime policy
- let compare-floor continue when model-check-second-line can close
- leave `tasks.md` or `progress.md` stale

---

## Appendix A. Current prompt to give CodeX

Copy the following prompt to CodeX if a direct instruction block is needed.

````text
You are the repo-local near-end completion lead.

Use FAQ 11 as current explanation delta.
The goal is repo-local near-end completion, not final public completion.
Close model-check-second-line, guided sample closeout, reserve package closeout, first strong typing finite-index sample verification, theorem-first/Lean floor, order-handoff helper-local samples, and docs/plan/progress/tasks/report consistency.

Preserve:
- specs are normative
- plan is repository memory
- FAQ is explanation only
- atomic_cut is local place finalization only
- memory_order family is backend/reference only
- relation decomposition principal
- authority-serial first, witness-aware second
- explicit edge-row / vertical continuation principal syntax
- stage-block secondary
- serial sugar reserve
- Lean-first theorem skeleton/preflight
- model-check second-line as remaining active self-driven package

Do not freeze final public grammar/API/verifier/tool/product.

Implement or update:
- model-check second-line carrier
- properties:
  - no_handoff_without_witnessed_publication
  - late_join_sees_published_history
  - stale_reconnect_fails_then_refreshes
  - no_double_owner_in_authoritative_turn
  - delegated_rng_provider_does_not_commit_room_state
  - auditable_authority_witness_minimal_core
- sample mappings p07/p08/p09/p13/p14/reserve packages
- guided sample closeout commands
- finite-index type samples for IFC / taint / capture / lifetime / cost
- theorem-first Lean representative floor
- order-handoff valid/invalid samples
- docs/specs/plan/progress/tasks/report consistency

Run validation. Report actual output. Do not fake.
````

---

## Appendix B. Previous once-completion handoff after FAQ 10

The following is preserved for continuity. FAQ 11 supersedes it where fresher, but this appendix retains earlier detail.

# Codex once-through completion handoff after FAQ 10

**Target reading:** 2026-04-20 current state  
**Audience:** CodeX / GPT-5.4 xhigh + sub-agents  
**Purpose:** Provide all information needed to push the current Mir language layer to a repo-local once-through completion: phase6 parser-side closeout, first strong typing finite-index layer, Lean-first theorem bridge, model-check second-line carrier, order/handoff helper-local surfaces, authoritative-room first scenario, and full docs/specs/plan/report consistency.

---

## 0. Executive directive

This handoff supersedes the operational direction of earlier handoff files where FAQ 10 has fresher information. It does not erase earlier context; Appendix B preserves the previous final-layer closeout handoff.

The goal is **not** full final public language/product completion. The goal is **repo-local near-end completion**:

```text
Repo-local near-end completion =
  current first lines are adopted into actual packages
  + phase6 parser-side narrow carrier closeout is complete enough for the fixed subset
  + first strong typing finite-index layer is specified and implemented/prototyped enough for current samples
  + Lean-first theorem skeleton/preflight is present and representative samples run
  + model-check second-line carrier is specified enough to continue self-driven work
  + order/handoff helper-local source surfaces are implemented for current examples
  + authoritative-room first scenario runs through current CLI/test/artifact floor
  + docs/specs/plan/progress/tasks/reports are consistent
  + remaining work is explicitly partitioned into mixed gate / true user-spec gate / reserve integration
```

Do **not** claim the following as complete unless a later task explicitly proves it:

- final parser grammar
- final public parser/checker/runtime API
- final public verifier contract
- production theorem/model-check binding
- installed binary / packaging
- FFI / game-engine adapter
- exhaustive shared-space catalog
- broader application target beyond authoritative-room first scenario

---

## 1. Repository discipline

1. `specs/` is normative.
2. `plan/` is repository memory.
3. `docs/reports/` is historical evidence.
4. `progress.md` and `tasks.md` are current snapshots.
5. FAQ files are current explanations, not specs.
6. Never factify OPEN / FUTURE / COMPARISON.
7. Keep stop lines visible.
8. Every non-trivial task needs a report.
9. If a mirror document is not updated, explicitly report why.

---

## 2. FAQ 10 answer fully integrated

### 2.1 Current state in one paragraph

FAQ 10 says the repo is **substantially advanced** but **not final-public complete**. It already has parser-free current L2, compile-ready minimal actualization, authored sixteen, corrected prototype set `p01...p14`, theorem/model-check/order-handoff/shared-space helper-local bridges, representative Lean sample execution, and phase6 parser-side narrow carrier actualization. But the two big problems are not completely solved in final-public sense, and the final parser grammar / final public API / final verifier contract / concrete production theorem-model-check binding remain open.

### 2.2 Genuine progress after FAQ 09

FAQ 10 reports actual progress, not just restatement:

- phase6 actual parser / AST carrier first tranche
- phase6 actual checker/runtime first tranche
- phase6 compile-ready verification / formal hook threshold
- phase6 next-reopen sequencing
- phase6 parser second tranche attached-slot / predicate-fragment first package
- phase6 reserve formal tool binding inventory
- phase6 parser-side follow-up package sequencing
- phase6 parser second-tranche shared single attachment frame first package
- fixed-subset source-sample corpus scope / file-layout threshold
- request clause suite publicization
- perform head structural carrier

Interpretation:

```text
The repo has moved from docs-only comparison into narrow non-production code carriers in mir-ast / mir-runtime.
```

### 2.3 First strong typing layer current default

FAQ 10 clarifies the first strong typing default:

- full dependent core is not required for the first public core
- finite decidable index fragment is the principal target
- conceptual spine:

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C
```

- IFC / taint / capture / lifetime / simple cost are first-class targets

Interpretation:

```text
Problem 1 is no longer primarily “what theory could work?”
Problem 1 is now “implement the finite-index strong typing layer without prematurely adopting final typed calculus.”
```

### 2.4 Current queue

Current self-driven queue:

```text
Package 91 = phase6 perform-head-request-clause-bundle-attachment comparison
```

It is an implementation-side narrowing task, not foundation search. It compares how to combine:

- perform head structural carrier
- request clause suite carrier

without freezing final grammar or final public parser API.

### 2.5 Floors

| Floor | State | Meaning |
|---|---|---|
| compare floor | close | current first line / retained alternatives / stop line are organized |
| actual adoption floor | close | Problem 1 / Problem 2 / syntax-modality / near-end closeout are current package judgments |
| helper-local actualization / narrowing floor | largely close on current tranche | theorem/model-check/order-handoff/shared-space and phase6 parser-side line have narrow actualization |
| representative execution floor | reached | authored sixteen, `p01...p14`, representative Lean samples, CLI / runner / static gate are machine-checked |
| final public language/tool/app floor | not close | parser grammar, public API, verifier contract, production binding, exhaustive catalog, packaging are not closed |

### 2.6 Current evidence

Current runnable/machine-check evidence includes:

- authored sixteen:
  - `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype set:
  - `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08 / p09 / p10 / p11 / p12 / p13 / p14`
- theorem-side:
  - review-unit first line
  - Lean-stub pilot
  - artifact-conformance bridge
  - representative trace alignment
  - public-seam compression
  - representative Lean sample set actual Lean execution
- model-check side:
  - row-local property route
  - checker-boundary contract route
  - checker-artifact route
  - public-seam compression
- order-handoff/shared-space:
  - authoritative-room vertical slice
  - minimal companion surface
  - stage-block secondary surface
  - witness/provider route
  - emitted-contract trace alignment
  - late-join negative static stop

### 2.7 Problem 1 current state

Closed or substantially advanced:

- checker-adjacent semantic carrier principal
- finite decidable index fragment principal
- conceptual spine `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C`
- security label / taint / capture / lifetime / simple cost first-class target
- theorem-first external integration target
- notebook-first theorem line
- row-local model-check carrier first
- representative source evidence:
  - `p06-typed-proof-owner-handoff`
  - `p10-typed-authorized-fingerprint-declassification`
  - `p11-typed-unauthorized-fingerprint-release`
  - `p12-typed-classified-fingerprint-publication-block`
- Lean-side first fragment:
  - `samples/lean/foundations/CurrentL2LabelModel.lean`
  - `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - `samples/lean/foundations/CurrentL2ProofSkeleton.lean`

Not final yet:

- stronger typed surface actual adoption
- final typed source principal
- final public theorem result object
- consumer-shaped theorem payload public contract
- proof object public schema
- first settled property language
- concrete theorem prover brand / production binding
- concrete model-check tool brand / production binding
- final public checker artifact
- actual public checker migration
- final public verifier contract

### 2.8 Problem 2 current state

Closed or substantially advanced:

- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- thread/node parity wording default
- authoritative room first actual default profile
- low-level family retained-later reference
- representative source evidence:
  - `p07-dice-late-join-visible-history`
  - `p08-dice-stale-reconnect-refresh`
  - `p09-dice-delegated-rng-provider-placement`
  - `p13-dice-late-join-missing-publication-witness`
  - `p14-dice-late-join-handoff-before-publication`
- operational actualization:
  - authoritative-room vertical slice
  - minimal companion surface
  - stage-block secondary surface
  - serial-scope reserve surface
  - witness/provider route actual adoption
  - witness/provider schema route actual adoption
  - emitted-contract trace alignment
  - order-handoff/shared-space public-seam helper mirror
  - CLI `surface_preview`
  - negative static stop

Not final yet:

- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- combined public witness/provider contract
- final emitted-handoff contract
- stronger fairness / replay profile
- exhaustive shared-space final catalog

### 2.9 FAQ 10 fresh validation anchors

Run at minimum:

```bash
cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact
cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact
python3 scripts/validate_docs.py
```

---

## 3. User-authorized defaults for this tranche

### 3.1 Main theory spine

```text
Principal = multimodal dependent core + layered finite-index strong typing
```

Do not flatten all candidate theories. Use one spine plus layers.

### 3.2 First strong typing layer

First target:

- finite posets
- finite lattices
- powerset lattices
- lifetime/region preorder
- capture-set inclusion
- simple numeric resource bounds / ordered monoids
- simple cost counters
- IFC label constraints
- taint as label/powerset specialization

Do not include in first cut:

- arbitrary user-defined type-level functions
- full dependent pattern matching
- general theorem proving inside compiler
- full asymptotic inference
- probabilistic IFC
- covert-channel noninterference

### 3.3 Proof assistant defaults

- Lean-first for reference semantics, first checker fragment, and proof skeleton.
- Rocq/Iris reserve for concurrency separation logic if needed.
- Apalache/TLA+ or equivalent reserve for model-check second line.
- Do not require production binding for this tranche.

### 3.4 Order/handoff defaults

Use high-level relation decomposition as principal:

```text
program_order
dependency_order
publication_order
observation_order
witness_order
finalization_order
scoped_happens_before
```

Use:

```text
authority_serial_transition_family = first
witness_aware_commit_family = second
low-level std::memory_order / std::kill_dependency = retained-later backend/reference family
```

### 3.5 Syntax defaults

```text
principal = explicit edge-row / vertical continuation
secondary = stage / after / witness stage-block
reserve = authoritative-room serial sugar
```

---

## 4. Package plan

### Package 91 — perform-head / request-clause bundle attachment comparison

Goal: close current queue.

Recommended combined carrier:

```text
Stage3RequestHeadClauseBundle {
  perform_head: Stage3PerformHead,
  clause_suite: Stage3RequestClauseSuite,
  attachment_frame_kind: "request-local-two-slot-suite",
}
```

Non-goals:

- final parser grammar
- final public parser API
- full Program lowering
- richer predicate grammar

Acceptance:

- helper-local / test-only carrier exists
- perform head minimum cut preserved
- request clause suite fixed two-slot cut preserved
- malformed attachment remains fail-closed
- tests pass
- docs/report/progress/tasks updated

### Package 92 — first strong typing finite-index layer

Deliver:

1. Spec/example doc for finite-index spine.
2. Prototype/static checker carrier for label lattice, capture set, lifetime preorder, simple cost counter.
3. Representative samples:
   - authorized fingerprint declassification valid
   - unauthorized fingerprint release invalid
   - classified fingerprint publication blocked
   - capture escape rejected
   - cost bound exceeded rejected
4. Docs updates.

### Package 93 — Lean-first formal skeleton

Ensure or create:

```text
samples/lean/foundations/CurrentL2LabelModel.lean
samples/lean/foundations/CurrentL2IfcSecretExamples.lean
samples/lean/foundations/CurrentL2ProofSkeleton.lean
```

Optional formal tree:

```text
formal/lean/Mir/Core.lean
formal/lean/Mir/Modes.lean
formal/lean/Mir/Effects.lean
formal/lean/Mir/Typing.lean
formal/lean/Mir/Obligations.lean
```

Required theorem placeholders:

```text
preservation
progress_or_explicit_failure
linearity_preservation
no_rollback_across_atomic_cut
no_repromotion
checker_soundness_fragment
finite_index_constraint_soundness
ifc_no_illicit_explicit_flow_fragment
elaboration_soundness_surface_to_core
obligation_extraction_soundness
```

### Package 94 — theorem-first and model-check second-line

Theorem-first:

- notebook-first
- Lean-first experimental binding
- symbolic evidence refs
- proof review unit / theorem result object preview

Model-check second-line:

- row-local carrier
- property set:
  - late join sees published history
  - stale reconnect fails then refreshes
  - no handoff without witnessed publish
  - no double owner in authoritative turn transition

### Package 95 — order/handoff source surface and artifacts

Implement/verify:

- explicit edge-row principal surface
- stage-block secondary surface
- serial reserve surface if available
- missing publication witness static stop
- handoff before publication static stop
- stale reconnect refresh

### Package 96 — authoritative-room first scenario

First room default profile:

```text
activation = authority-ack
authority placement = single room authority
consistency = authoritative serial transition
RNG = authority_rng
late join = published history is visible as past
stale reconnect = fail then refresh
replay = stale/incompatible replay invalidated rather than silently merged
fairness = no distributed fairness theorem required in first line
```

### Package 97 — reserve strengthening

Order:

1. `auditable_authority_witness`
2. `delegated_rng_service`
3. model-check second-line concretization

Keep `distributed_randomness_provider` future comparison.

### Package 98 — documentation/report closeout

Update relevant specs, plan, progress, tasks, documentation, traceability, and create a report.

---

## 5. Strong typing examples and expected outcomes

### 5.1 Valid declassification

```text
label Public <= UserSecret <= KeyMaterial

fn fingerprint(key: PrivateKey @ KeyMaterial)
  -> Fingerprint @ Public
  requires authority(declassify_key_material)
  ensures key not exported
```

Expected:

```json
{
  "static_verdict": "valid",
  "constraints": [
    "authority(declassify_key_material) present",
    "KeyMaterial declassification path allowed for Fingerprint",
    "key capability not exported"
  ],
  "terminal_outcome": "success"
}
```

### 5.2 Unauthorized secret release

```text
label Public <= UserSecret <= KeyMaterial

fn leak(key: PrivateKey @ KeyMaterial)
  -> Bytes @ Public {
  return key.bytes
}
```

Expected:

```json
{
  "static_verdict": "malformed",
  "reason_family": "ifc_flow_violation",
  "constraints_failed": ["KeyMaterial <= Public"],
  "entered_evaluation": false
}
```

### 5.3 Capture escape rejected

```text
fn make_signer(key: PrivateKey @ KeyMaterial)
  -> Fn[Bytes -> Signature] @ Public {
  return fn(msg) { sign(msg, key) }
}
```

Expected:

```json
{
  "static_verdict": "malformed",
  "reason_family": "capture_escape",
  "constraints_failed": [
    "captures(returned_fn) <= allowed_public_captures",
    "key not allowed to escape current authority scope"
  ]
}
```

### 5.4 Cost bound exceeded

```text
fn fetch_and_move(p: Player) -> Board
  cost <= { remote_calls: 0 } {
  perform fetch_remote_bonus on bonus_service
  move_player(p, 3)
}
```

Expected:

```json
{
  "static_verdict": "malformed",
  "reason_family": "cost_bound_violation",
  "constraints_failed": ["remote_calls <= 0"]
}
```

---

## 6. Order/handoff examples and expected outcomes

### 6.1 Valid edge-row handoff

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Expected semantic carrier:

```json
{
  "events": [
    {"id": "roll", "kind": "roll", "provider": "authority_rng"},
    {"id": "publish_draw", "kind": "publish", "subject": "draw"},
    {"id": "handoff_owner", "kind": "handoff", "from": "A", "to": "B"}
  ],
  "relations": [
    ["program_order", "roll", "publish_draw"],
    ["publication_order", "roll", "publish_draw"],
    ["witness_order", "publish_draw", "handoff_owner"],
    ["scoped_happens_before", "roll", "handoff_owner"]
  ],
  "static_verdict": "valid",
  "terminal_outcome": "success"
}
```

### 6.2 Missing witness

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
```

Expected:

```json
{
  "static_verdict": "malformed",
  "reason_family": "missing_handoff_witness",
  "entered_evaluation": false
}
```

### 6.3 Handoff before publication

```text
roll draw via authority_rng
handoff dice_owner A -> B
  requires witness(draw)
publish draw
```

Expected:

```json
{
  "static_verdict": "malformed",
  "reason_family": "handoff_before_publication",
  "entered_evaluation": false
}
```

### 6.4 Stage-block handoff

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      after publish(draw)
      requires witness(draw)
}
```

Expected:

```json
{
  "static_verdict": "valid",
  "stage_sequence": ["roll", "publish", "handoff"],
  "terminal_outcome": "success"
}
```

---

## 7. Package 91 parser-side examples

### 7.1 Perform head

```text
perform roll via dice_rng
```

Expected:

```json
{
  "Stage3PerformHead": {
    "operation": "roll",
    "target_ref": {"kind": "chain", "name": "dice_rng"}
  }
}
```

### 7.2 Clause suite

```text
require player_active(A)
ensure result_in_range(draw, 1, 6)
```

Expected:

```json
{
  "Stage3RequestClauseSuite": {
    "require_fragment_text": "player_active(A)",
    "ensure_fragment_text": "result_in_range(draw, 1, 6)"
  }
}
```

### 7.3 Combined bundle

```text
perform roll via dice_rng
  require player_active(A)
  ensure result_in_range(draw, 1, 6)
```

Expected:

```json
{
  "Stage3RequestHeadClauseBundle": {
    "perform_head": {
      "operation": "roll",
      "target_ref": {"kind": "chain", "name": "dice_rng"}
    },
    "clause_suite": {
      "require_fragment_text": "player_active(A)",
      "ensure_fragment_text": "result_in_range(draw, 1, 6)"
    },
    "attachment_frame_kind": "request-local-two-slot-suite"
  }
}
```

---

## 8. Validation commands

Minimum FAQ 10 validation:

```bash
cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact
cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact
cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact
python3 scripts/validate_docs.py
```

Broader representative suite if available:

```bash
cargo test -p mir-runtime --test current_l2_source_sample_runner
cargo test -p mir-runtime --test current_l2_operational_cli
cargo test -p mir-runtime --test current_l2_verifier_preview_alignment
cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor
cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor
cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization
cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight
cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring
cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization
cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface
cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression
```

Lean validation if available:

```bash
lake build
```

If unavailable, report exactly:

```text
Lean toolchain unavailable; skeleton not mechanically checked in this run.
```

---

## 9. Required final CodeX response

CodeX must return:

1. updated files
2. new files
3. unchanged files with reason
4. Package 91 status
5. first strong typing finite-index layer status
6. Lean/formalization status
7. Problem 1 closeout status
8. Problem 2 closeout status
9. syntax/source-surface status
10. shared-space first scenario status
11. validation results
12. remaining mixed gates
13. remaining true user-spec gates
14. next self-driven packages
15. report path

---

# Appendix A — FAQ 10 verbatim

# FAQ 10

## この文書について

この文書は、2026-04-20 時点の current reading に基づいて、

- 現状でどこまで何が終わっているか
- もう二大問題を完全に解決して、言語側の実装まで終わっていると言えるか
- その段階へ持っていく全体像に対して今どこにいるか
- そこへ持っていくために、何がまだ必要か
- これらに答えれば repo がどこまで自走できるか

を、`faq_009.md` の次段として整理する FAQ である。

規範判断の正本は `specs/`、長期の repository memory は `plan/`、薄い snapshot は `progress.md` と `tasks.md` にある。
この FAQ 自体は **current explanation** であり、新しい規範判断を作る文書ではない。

今回の FAQ は、特に次を踏まえている。

- `specs/examples/552`
  - phase6 actual parser / AST carrier first tranche threshold helper mirror
- `specs/examples/553`
  - phase6 actual checker/runtime first tranche threshold helper mirror
- `specs/examples/554`
  - phase6 compile-ready verification / formal hook threshold helper mirror
- `specs/examples/555`
  - phase6 next-reopen sequencing threshold helper mirror
- `specs/examples/556`
  - phase6 parser second tranche attached-slot / predicate-fragment first package
- `specs/examples/557`
  - first strong typing layer finite-index spine default
- `specs/examples/558`
  - phase6 reserve formal tool binding inventory threshold helper mirror
- `specs/examples/559`
  - phase6 parser-side follow-up package sequencing threshold helper mirror
- `specs/examples/560`
  - phase6 parser second-tranche shared single attachment frame first package
- `specs/examples/561`
  - fixed-subset source-sample corpus scope-and-file-layout threshold helper mirror
- `specs/examples/562`
  - phase6 request-clause-suite publicization threshold helper mirror
- `specs/examples/563`
  - phase6 perform-head structural carrier threshold helper mirror
- 2026-04-20 の fresh validation
  - `mir-ast` parser-side tranche test
  - representative CLI prototype test
  - docs validation

---

## 1. 先に短く答えるとどうか

短く言うと、次の読みが現在もっとも正確である。

1. **かなり進んでいる。**
   current repo は、
   - parser-free current L2
   - compile-ready minimal actualization
   - authored sixteen
   - corrected prototype set `p01...p14`
   - theorem/model-check/order-handoff/shared-space の helper-local bridge
   - representative Lean sample set actual Lean execution
   - phase6 parser-side narrow carrier actualization
   を already 持つ。
2. **しかし、二大問題を completely solved と呼ぶのはまだ正確ではない。**
   current repo が close したのは、主として
   - current first line
   - retained alternatives
   - actual adoption package
   - helper-local actualization floor
   - stop line
   - mixed gate
   - true user-spec gate
   の切り分けである。
3. **言語側の実装まで終わっているともまだ言えない。**
   いま実装されているのは、
   - parser-free validation substrate
   - compile-ready minimal actualization
   - runner / CLI / regression
   - narrow non-production parser carrier の phase6 tranche
   - theorem/model-check/order-handoff の helper-local preview / artifact / bridge
   までであり、
   - final parser grammar
   - final public parser / checker / runtime API
   - parser-to-Program full lowering
   - final public verifier contract
   - concrete theorem/model-check production binding
   はまだない。
4. **現在地は「理論しか残っていない」段階ではない。**
   むしろ、
   - compare floor は close
   - actual adoption floor は close
   - helper-local actualization floor も大半 close
   - representative runnable / Lean execution floor も reached
   - current active line は parser-side second tranche の narrow bundle comparison と final public seam mixed gate maintenance
   という段階である。
5. **「これらに答えればもう最後まで完全自走できるか」には、near-end target ならほぼ yes、full final-public completion なら still conditional が正確である。**
   repo-local near-end success
   （runnable CLI / tests / emitted artifact / reproducible compare floor）
   まではかなり強く自走できる。
   ただし final public language / final verifier / packaging / broader app completion までを no-question で保証するのは、まだ正確ではない。

---

## 2. `faq_009.md` 以後に genuinely progressed したこと

`faq_009.md` の後で進んだのは、単なる言い換えではなく、次の actual progress である。

### 2.1 phase6 implementation line が parser-side / checker-runtime-side へ進んだ

`faq_009.md` 時点の強い進展は、
representative Lean sample set actual Lean execution と residual public-seam compression だった。

それ以後 current repo は、phase6 narrow actualization として少なくとも次を close した。

- phase6 actual parser / AST carrier first tranche
- phase6 actual checker/runtime first tranche
- phase6 compile-ready verification / formal hook threshold
- phase6 next-reopen sequencing
- phase6 parser second tranche attached-slot / predicate fragment first package
- phase6 reserve formal tool binding inventory
- phase6 parser-side follow-up package sequencing
- phase6 parser second-tranche shared single attachment frame first package
- fixed-subset source-sample corpus scope / file-layout threshold
- request clause suite publicization
- perform head structural carrier

つまり current repo は、Lean 側 bridge や helper-local preview だけでなく、
**phase6 の narrow code carrier を実際に `mir-ast` / `mir-runtime` 側へ ratchet している**。

### 2.2 first strong typing layer の current default がさらに明確になった

`faq_009.md` の時点でも strong typing line はかなり整理されていたが、
2026-04-20 には `specs/examples/557` により、

- full dependent core を first public core に要求しない
- finite decidable index fragment を principal target にする
- `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` を conceptual spine に置く
- IFC / taint / capture / lifetime / simple cost を first strong typing layer の first-class target にする

という current reading が、より明示的になった。

これは final typed calculus adoption ではない。
しかし、**Problem 1 が「何を first implementation target にしてよいか」については、以前よりかなり明瞭になった**ことを意味する。

### 2.3 current queue が theory search ではなく parser-side closeout 比較に narrowed した

現在の current self-driven queue は、

- `Package 91`
  - phase6 perform-head-request-clause-bundle-attachment comparison

である。

ここでやっているのは、

- perform head structural carrier
- request clause suite carrier

をどの narrow combined carrier で束ねるかの比較であり、
final grammar や final public parser API をいきなり決めることではない。

したがって current queue の性格は、
**foundation search の継続**ではなく、
**already chosen current first line の implementation-side narrowing** に近い。

---

## 3. いま既に終わっていること

### 3.1 floor ごとの整理

現状を floor ごとに分けると、次の読みが最も誤読が少ない。

| floor | 現状 | 読み方 |
|---|---|---|
| compare floor | close | `specs/examples/458...465` で current first line / retained alternatives / stop line / compare floor を整理済み |
| actual adoption floor | close | `specs/examples/466...469` で Problem 1 / Problem 2 / syntax-modality / near-end closeout を current package judgment に上げ済み |
| helper-local actualization / narrowing floor | largely close on current tranche | `specs/examples/470...563` により theorem/model-check/order-handoff/shared-space / phase6 parser-side line の narrow actualization を進め済み |
| representative execution floor | reached on current mapped corpus | authored sixteen、`p01...p14`、representative Lean sample set、CLI / runner / static gate まで machine-check された current floor がある |
| final public language / tool / app floor | not close | final parser grammar / public API / verifier contract / production tool binding / exhaustive catalog / packaging は未 close |

### 3.2 current runnable / machine-check evidence

少なくとも次は current state で runnable / machine-check の evidence を持っている。

- current authored sixteen
  - `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype set
  - `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08 / p09 / p10 / p11 / p12 / p13 / p14`
- runner / CLI / regression
  - authored corpus + corrected prototype path
- theorem-side
  - review-unit first line
  - Lean-stub pilot
  - artifact-conformance bridge
  - representative trace alignment
  - public-seam compression
  - representative Lean sample set actual Lean execution
- model-check side
  - row-local property route
  - checker-boundary contract route
  - checker-artifact route
  - public-seam compression
- order-handoff / shared-space side
  - authoritative-room vertical slice
  - minimal companion surface
  - stage-block secondary surface
  - witness/provider route
  - emitted-contract trace alignment
  - late-join negative static stop

この意味で、repo は docs-only skeleton ではない。
**current mapped corpus の corrected runnable floor は already reached** している。

### 3.3 phase6 implementation line で実際に code 側へ上がったもの

これは `faq_009.md` より一段進んだ、今回の重要点である。

実装側では少なくとも次が code anchor と test を持つ。

- `mir-ast::current_l2`
  - attached slot / predicate fragment first package
  - shared single attachment frame minimum
  - `Stage3RequestClauseSuite`
  - `parse_stage3_request_clause_suite_text()`
  - `Stage3PerformHead`
  - `Stage3PerformTargetRef`
  - `parse_stage3_perform_head_text()`
- `mir-runtime`
  - phase6 parser / checker / runtime / formal hook threshold manifest 群
  - current self-driven reopen sequencing manifest
  - operational CLI summary mirror

これは final parser grammar ではない。
しかし、
**phase6 implementation line が docs-only compare floor から narrow non-production code carrier へ進んでいる**
ことは明確である。

---

## 4. 二大問題ごとに、どこまで終わっているか

### 4.1 Problem 1 — 型システム / 定理証明 / モデル検査

#### いま終わっていること

- current first line は source-backed である。
  - checker-adjacent semantic carrier principal
  - finite decidable index fragment principal
  - `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C` conceptual spine
  - security label / taint / capture / lifetime / simple cost first-class target
  - theorem-first external integration target
  - notebook-first theorem line
  - row-local model-check carrier first
- representative source-side evidence がある。
  - `p06-typed-proof-owner-handoff`
  - `p10-typed-authorized-fingerprint-declassification`
  - `p11-typed-unauthorized-fingerprint-release`
  - `p12-typed-classified-fingerprint-publication-block`
- Lean-side first fragment がある。
  - `samples/lean/foundations/CurrentL2LabelModel.lean`
  - `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
  - `samples/lean/foundations/CurrentL2ProofSkeleton.lean`
- theorem/model-check bridge は helper-local actualization floor まである。
  - theorem review-unit transport
  - theorem result-object preview / route actual adoption
  - model-check checker-artifact route
  - public-seam compression
  - reopen-threshold helper mirror
- rough progress snapshot では、
  - 論理仕様 95%
  - ユーザ向け仕様 92%
  - 実装 / 運用 84%
  と読むのが current snapshot に沿う。

#### まだ終わっていないこと

- stronger typed surface actual adoption
- final typed source principal
- final public theorem result object
- consumer-shaped theorem payload public contract
- proof object public schema
- first settled property language
- concrete theorem prover brand / production binding
- concrete model-check tool brand / production binding
- final public checker artifact
- actual public checker migration
- final public verifier contract

#### したがってどう読むべきか

Problem 1 は、
**「もう未着手ではない」どころか、current first line と helper-local implementation floor はかなり厚い。**
ただし、
**final public theorem/model-check/type system adoption はまだ完了していない。**

---

### 4.2 Problem 2 — order / handoff / `memory_order` / authority-handoff

#### いま終わっていること

- current first line は source-backed である。
  - cut family decomposition
  - relation decomposition principal
  - `authority_serial_transition_family` first
  - thread/node parity wording default
  - authoritative room first actual default profile
  - low-level family retained-later reference
- representative source-side evidence がある。
  - `p07-dice-late-join-visible-history`
  - `p08-dice-stale-reconnect-refresh`
  - `p09-dice-delegated-rng-provider-placement`
  - `p13-dice-late-join-missing-publication-witness`
  - `p14-dice-late-join-handoff-before-publication`
- operational side では、少なくとも次が actualize 済みである。
  - authoritative-room vertical slice
  - minimal companion surface
  - stage-block secondary surface
  - serial-scope reserve surface
  - witness/provider route actual adoption
  - witness/provider schema route actual adoption
  - emitted-contract trace alignment
  - order-handoff/shared-space public-seam helper mirror
  - CLI `surface_preview`
  - negative static stop
- rough progress snapshot では、
  - 論理仕様 89%
  - ユーザ向け仕様 87%
  - 実装 / 運用 66%
  と読むのが current snapshot に沿う。

#### まだ終わっていないこと

- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- combined public witness/provider contract
- final emitted-handoff contract
- stronger fairness / replay profile
- exhaustive shared-space final catalog

#### したがってどう読むべきか

Problem 2 も、
**decomposition / default profile / runnable prototype / static stop / helper-local operational mirror までは十分進んでいる。**
ただし、
**final wording と final public shared-space contract はまだ未完である。**

---

## 5. もう「二大問題を完全に解決し、言語側の実装まで終わっている」と言えるか

結論から言うと、**まだそうは言えない。**

理由は単純で、現在 close しているのが

- current first line
- retained alternatives
- actual adoption package
- helper-local actualization floor
- representative execution floor
- parser-side narrow carrier floor

であって、

- final public language implementation
- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- production theorem/model-check binding
- final public witness/provider/artifact contract

ではないからである。

より短く言えば、現在の repo は

- **理論だけの段階ではない**
- **かなり実装されている**
- **しかし final public product shape までは到達していない**

という位置にある。

---

## 6. 全体像に対して今どこにいるか

macro phase で読むと、現在地は次のように整理できる。

- `Macro 4`
  - fixed authored/prototype floor active
- `Macro 5`
  - final-layer closeout packages active
- `Macro 6`
  - minimal working subset actual default + public-seam compression closed
- `Macro 7`
  - mixed with repo-local near-end success criteria
- `Macro 8`
  - first authoritative-room scenario selected、しかし broader app target は still mixed

これを全体 ladder に直すと、次のように読むのが自然である。

1. semantic kernel / parser-free / compile-ready minimal actualization
   - already substantial
2. fixed-subset runnable corpus
   - reached
3. theory-lab compare floor / actual adoption floor
   - reached
4. helper-local actualization / residual compression / representative execution
   - reached
5. phase6 parser-side / checker-runtime-side narrow code carrier actualization
   - active and progressing
6. final public seam / final parser grammar / final verifier contract / production binding
   - still later
7. packaging / installed binary / FFI / broader app target
   - still later and partly user-spec

したがって現在地は、
**near-end toward repo-local fixed-subset completion**
であって、
**final public language complete**
ではない。

---

## 7. その段階へ持っていくために、何がまだ必要か

ここは、**self-driven で詰められる mixed gate** と
**true user-spec gate** を分けて読むのが重要である。

### 7.1 remaining mixed gate

これは repo 側で比較・実装・検証を続けて narrow にできるが、
まだ final adoption へは上げていないものを指す。

- perform head と request clause suite の combined carrier minimum
- stronger typed surface を source principal に上げるかどうか
- final typed source principal の shape
- theorem result object / payload public-contract shape
- model-check property language / checker artifact / migration shape
- final source-surface handoff wording
- final witness/provider/artifact public shape
- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract

ここで大事なのは、
**「まだ mixed gate が残る」ことは「もう何も決められない」ことではない**
という点である。
current repo は、このかなりの部分を helper-local floor まで進めてある。

### 7.2 true user-spec gate

これは repo 側だけでは勝手に final adoption しない方がよいものを指す。

- shared-space exhaustive final catalog beyond minimal working subset
- installed binary / packaging / FFI / engine adapter / host integration target
- upper-layer application target beyond authoritative-room first scenario

### 7.3 implementation-side remaining work

current near-end target に限っても、implementation-side には次が残る。

- Package 91
  - perform head / request clause suite bundle attachment comparison
- parser-side second tranche の narrow follow-up
- parser carrier と checker/runtime side の reconnect hardening
- helper-local CLI / artifact / trace mirror の drift suppression
- docs / traceability / roadmap sync

したがって、まだ work はある。
ただし、その多くはすでに
**「何を比較し、何を保留し、何を実装するか」**
が narrow に切り分けられた work である。

---

## 8. これらに答えればもう最後まで自走できるのが理想だが、どうか

厳密には、**「最後まで」の意味で答えが変わる。**

### 8.1 repo-local near-end success までなら

かなり強く **yes** と言ってよい。

なぜなら現在の default はすでにかなり揃っているからである。

- theorem-first external integration target
- first strong typing layer = finite decidable index fragment + IFC / taint + capture / lifetime + simple cost
- first application target = authoritative shared-space turn-based room
- first room default profile = authority-ack / single room authority / authoritative serial transition / authority_rng / late join visible past / stale reconnect fail-then-refresh
- repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor = near-end success

この範囲では、repo はもうかなり長く self-driven に進める。

### 8.2 full final-public completion までなら

まだ **conditional** である。

理由は、repo 自身が意図的に次を deferred / mixed gate に残しているからである。

- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- concrete theorem/model-check production binding
- final public witness/provider/artifact contract
- exhaustive shared-space final catalog
- packaging / FFI / host integration target

これは information shortage だけの問題ではなく、
**repo が premature adoption を避けるために意図的に止めている gate**
でもある。

したがって、
「この FAQ に答えれば絶対に full final-public completion まで no-question で行ける」
とはまだ書けない。

### 8.3 では、この FAQ の practical meaning は何か

この FAQ の practical meaning は、

- どこまで終わっているか
- 何が genuinely progressed したか
- 何が mixed gate で
- 何が true user-spec gate で
- current self-driven queue がどこにあるか

を誤読なく整理し、
**repo-local near-end target へ向かう自走の障害をかなり減らす**
ことにある。

その意味では、かなり有効である。

---

## 9. 現時点での実装＆実行による比較検証は、どれくらい進んでいるか

簡潔に言えば、**もう「かなり進んでいる」段階であり、初期比較段階ではない。**

current practical reading は次である。

- current authored sixteen は runner / regression / static gate / artifact wiring に乗っている
- corrected prototype set `p01...p14` は explicit path で runnable
- Lean foundations は actual small proof fragment を持つ
- representative Lean sample set は actual Lean execution reached
- order-handoff / shared-space は helper-local operational preview と negative static stop を持つ
- parser-side phase6 line は narrow code carrier まで actualize された

今回の fresh validation としても、少なくとも次を再確認した。

- `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
- `cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact`
- `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
- `python3 scripts/validate_docs.py`

したがって、
**実装＆実行による比較検証はもう「理論の前段」ではなく、closeout のための implementation ratchet に入っている**
と読んでよい。

---

## 10. 最後に、いまの状態を一文で言うとどうか

いまの repo は、

**二大問題の current first line と helper-local actualization はかなり深く進み、fixed-subset runnable floor と representative Lean execution floor も reached しているが、final public language / verifier / shared-space contract まではまだ intentionally 止めてあり、現在地は repo-local near-end success に向けた phase6 parser-side closeout 実装中**

と読むのが最も正確である。


---

# Appendix B — Previous final-layer closeout handoff preserved

The prior handoff remains inherited context. If it conflicts with FAQ 10 or the new sections above, use FAQ 10 and the new sections above.

# Codex final-layer closeout handoff — Mir theory spine / verification / implementation plan

**Date:** 2026-04-19  
**Audience:** CodeX / GPT-5.4 xhigh + sub-agents  
**Purpose:** This is a single Markdown handoff intended to let CodeX continue the Mir theory layer and implementation closeout autonomously as far as possible.

This document supersedes the earlier `codex_theory_handoff_2026-04-18.md` for the next task.  
It preserves the previous handoff, incorporates the full `faq_008.md` reading, adds the user-authorized theory defaults, and adds concrete sample/spec/proof/validation targets.

---

## 0. Non-negotiable repository discipline

Before doing any work, preserve the following hierarchy.

1. **`specs/` is the normative source of truth.**  
   It defines semantics, boundaries, and decisions.
2. **`plan/` is repository memory.**  
   It summarizes current status, sequencing, risks, boundaries, and future work. It does not replace `specs/`.
3. **`docs/reports/` is chronological evidence.**  
   Every substantial Codex task must write a report.
4. **`progress.md`, `tasks.md`, and `faq_*.md` are current snapshots / explanations.**  
   They are useful, but they are not normative.
5. **Never factify OPEN / FUTURE / COMPARISON material.**  
   When in doubt, keep a candidate in a comparison document or report, not in the decision register.

Required reading order for this task:

```text
specs/00-document-map.md
specs/01-charter-and-decision-levels.md
specs/02-system-overview.md
specs/03-layer-model.md
specs/04-mir-core.md
specs/05-mirrorea-fabric.md
specs/07-typed-effects-wiring-platform.md
specs/09-invariants-and-constraints.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
specs/12-decision-register.md
plan/00-index.md
plan/01-status-at-a-glance.md
plan/03-decision-strengths-and-boundaries.md
plan/04-core-semantics-current-l2.md
plan/06-surface-notation-status.md
plan/07-parser-free-poc-stack.md
plan/08-representative-programs-and-fixtures.md
plan/09-helper-stack-and-responsibility-map.md
plan/10-roadmap-overall.md
plan/11-roadmap-near-term.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/14-glossary-and-boundary-rules.md
plan/15-current-l2-fixture-authoring-template.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
plan/91-maintenance-rules.md
faq_006.md
faq_007.md
faq_008.md
codex_theory_handoff_2026-04-18.md, if present
progress.md
tasks.md
Documentation.md
```

---

## 1. Current state after FAQ 08

### 1.1 One-line reading

The project is **not** still in abstract-only research. It has a runnable fixed subset, corrected prototypes, helper-local comparison floors, theorem/model-check pre-floors, and source-facing narrowing.  
However, it is **not** final public language completion. The remaining work is adoption / formalization / implementation closeout for the theory layer, plus later public-surface and system-level gates.

### 1.2 FAQ 08 distilled

`faq_008.md` establishes the following current reading:

- Problem 1 and Problem 2 have source-backed current first lines.
- Compare floor is close.
- Actual adoption package is close on current tranche.
- Helper-local actualization / narrowing floor is close on current tranche.
- Reserve strengthening is active:
  1. `auditable_authority_witness`
  2. `delegated_rng_service`
  3. model-check second-line concretization
- Final public language/tool/app completion is **not** close.
- Remaining blockers are not lack of theory candidates. They are adoption / public seam / proof-tool / final target decisions.

### 1.3 Evidence already present

At the `faq_008.md` stage, the repo has evidence for:

- current authored sixteen source samples:
  - `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype octet:
  - `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08`
- current comparison floors:
  - verifier preview alignment pre-floor
  - model-check projection pre-floor
  - theorem discharge pre-floor
- helper-local actualization / narrowing floors:
  - theorem-first experimental pilot actualization
  - theorem-prover experimental binding preflight
  - authoritative-room vertical-slice actualization
  - minimal companion / experimental order-handoff surface
  - stage-block secondary order-handoff surface

Representative fresh validation from `faq_008.md`:

```text
cargo test -p mir-runtime --test current_l2_source_sample_runner                 => 22 passed
cargo test -p mir-runtime --test current_l2_operational_cli                     => 12 passed
cargo test -p mir-runtime --test current_l2_verifier_preview_alignment          => 5 passed
cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor     => 5 passed
cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor          => 5 passed
cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization   => 5 passed
cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight    => 4 passed
cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring => 9 passed
cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization => 3 passed
cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface => 3 passed
cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface => 3 passed
python3 scripts/current_l2_source_sample_regression.py inventory                => authored sixteen present
python3 scripts/current_l2_source_sample_regression.py regression               => all regression commands passed
```

These are **not** final public language evidence. They are evidence for the current runnable / helper-local / prototype floor.

---

## 2. Source-backed boundaries that must remain intact

### 2.1 Project architecture

The project is a layered stack, not one runtime. Preserve this separation:

```text
Mir                         = semantic language core
Mirrorea                    = distributed control / routing / audit fabric
PrismCascade                = independent media-processing kernel
Typed-Effects Wiring        = operational effect-boundary inspection / rewiring layer
shared space / VRSNS / apps = upper-layer application / environment line
```

Do not collapse Prism into Mir runtime. Do not make Typed-Effects the language itself. Do not let Mirrorea absorb all application logic.

### 2.2 Mir-0 / Mir-1 / Mirrorea cut boundary

Mir-0 includes:

```text
event DAG
place
minimal effect request operation (perform is only a companion token)
effect / contract
minimal structured failure space
primitive fallback
local try / rollback
place-local atomic_cut
linear resource
```

Mir-0 does **not** include:

```text
barrier
durable_cut
full fallback algebra
coroutine semantics
emit semantics
distributed scheduler
route rebinding / overlay details
```

Important:

- `atomic_cut` is a **place-local finalizing cut**.
- `atomic_cut` only fixes the current `place` rollback frontier.
- `atomic_cut` is not global synchronization.
- `atomic_cut` is not distributed agreement.
- `atomic_cut` is not durable commit.
- `barrier` and `durable_cut` belong to later cut-family work.

### 2.3 Current L2 fallback / lease / chain

Keep these settled current L2 readings:

```text
fallback = guarded option chain
lease = option-local lifetime guard
same-lineage chain = left-to-right monotone degradation
no re-promotion to earlier option
write-after-expiry = try later write-capable option or Reject
rollback / atomic_cut do not reset degradation order
```

Static evidence floor:

```text
same-lineage static evidence = declared access target + edge-local documented lineage annotation
```

Do not hidden-accept underdeclared chains. Do not send malformed branch structure to dynamic `Reject`.

### 2.4 Current syntax boundary

The current surface is companion notation, not final grammar.

Settled companion direction:

```text
explicit edge-row family
A2 hanging continuation = polished first choice
A1 inline row = companion-equivalent shorthand
line-leading > ladder = comparison only
packed metadata row = avoid
```

Do not freeze final parser grammar in this task.

### 2.5 Verification boundary

Preserve the four-way split:

```text
core_static_checker
  local / structural / decidable floor

theorem_prover_boundary
  global semantic laws and proof obligations

protocol_verifier_boundary
  bounded / protocol / transition-system properties

runtime_policy_boundary
  operational policy, admission, retry, timeout, provider policy
```

Do not collapse theorem/model-check/runtime policy into first checker cut.

### 2.6 Shared-space current line

Preserve:

```text
source of truth = session-scoped membership registry
array/tree views = derived snapshots / UI / serialization
membership_epoch + member_incarnation split = first practical candidate
authority-ack = first activation candidate for authoritative room
single room authority = first authority placement
authoritative serial transition = first consistency mode
authority_rng = first RNG source
auditable_authority_witness = next strengthening candidate
delegated_rng_service = next practical candidate
distributed randomness provider = future comparison
```

---

## 3. User-authorized defaults for final-layer closeout

The user has now authorized the following defaults. Use them unless they conflict with stronger source-backed spec decisions. If conflict occurs, report it and do not silently override specs.

### 3.1 Main theory spine

Adopt as current principal direction:

```text
main spine = multimodal dependent core
λ◯□ = partial staging basis only
guarded λ / MDTT / MTT / Fitch-style multimodal = stronger foundation candidates
```

Do not flat-fuse all candidate theories. Use one spine plus attachment layers.

### 3.2 Strong typing

Type system should be strong, but layered.

Required layers:

```text
Layer 0: Core event semantics
Layer 1: Multimodal dependent spine
Layer 2: Linear / affine capability
Layer 3: Row-polymorphic effect
Layer 4: Decidable refinement / contract
Layer 5: Information-flow labels / policy
Layer 6: Residual theorem/model-check obligations
```

### 3.3 Multistage computation

Multistage computation is core-adjacent, not an external plugin.

- `λ◯□` supplies a useful micro-core for later/stable/staging.
- Full foundation needs multimodal dependent typing because the language has more axes than time/stage:
  - `stage`
  - `place`
  - `visibility`
  - `publication`
  - `witness`
  - `durability`
  - `authority`

### 3.4 Contract / refinement

Contracts remain first-class, but the first checker fragment should be decidable.

Use:

```text
require / ensure / admit
minimal predicate fragment
Liquid-style decidable refinement as reference family
```

Do not make arbitrary dependent predicates mandatory for the first checker.

### 3.5 Information-flow control / security labels

User-defined security labels are required as a serious first-class design target.

Goal example:

> User defines confidentiality levels and statically verifies that a secret key cannot flow outside an allowed scope.

First fragment should support:

```text
Labeled ℓ A or A @ ℓ
flows_to relation
join / meet
pc label
explicit declassification requiring authority/capability
linear secret-key capabilities
```

Covert channels, timing channels, distributed leakage, and richer policies go to theorem/model-check/runtime-policy boundaries.

### 3.6 Proof assistant strategy

Default:

```text
Lean-first
Rocq/Iris reserve for concurrency separation logic
Apalache/TLA+ style model checking for protocol/model-check second line
```

Use Lean for:

- reference core syntax/semantics
- typing skeleton
- theorem statements
- checker soundness fragment
- elaboration soundness plan

Use Rocq/Iris only if runtime/concurrency separation logic pressure becomes concrete.

Use Apalache/TLA+ style model checking for bounded protocol properties such as handoff, stale reconnect, late join, and no double owner.

### 3.7 Order / handoff / memory-order

Adopt:

```text
relation decomposition principal
authority-serial first
witness-aware second
low-level std::memory_order + std::kill_dependency = backend/reference family
```

Source-level surface should not expose exact low-level `memory_order` as principal syntax.

Relation family:

```text
program_order
dependency_order
publication_order
observation_order
witness_order
finalization_order
scoped_happens_before
```

Thread/node parity wording:

```text
thread と node は同じ causal language で書く。
違いは lowering / evidence / transport / failure / durability / policy に残す。
```

### 3.8 Syntax family

Default:

```text
principal = explicit edge-row / vertical continuation
secondary = stage / after / witness stage-block
reserve = authoritative-room serial sugar
```

Do not optimize for keyword polish yet. Optimize for semantic honesty, static checkability, and misreading resistance.

### 3.9 First completion scope

For this tranche, first completion **does not require**:

```text
installed binary
packaging
FFI
game engine adapter
final parser grammar
final public parser/checker/runtime API
final public verifier contract
broad application target
exhaustive shared-space catalog
```

First completion **does require**:

```text
multimodal dependent spine document
strong typing layer document
Lean-first formal skeleton / proof plan
authoritative-room first scenario runnable/prototype evidence
order/handoff helper-local source surface
p06/p07/p08 integration
current validation suite passing
specs/plan/progress/tasks/docs updated
report written
```

---

## 4. Principal theory design

### 4.1 Core judgement

Use this schematic judgement as the unifying spine:

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

Meaning:

```text
Ψ  = mode theory / world assumptions
Γ  = unrestricted and dependent context
Δ  = linear / ownership context
e  = expression / computation
A  = type
m  = current mode
ε  = effect row
O  = residual obligations
```

This judgement deliberately keeps axes separated.

- Dependent typing is in `Γ` and `A`.
- Linearity/ownership is in `Δ`.
- Modal/staged/place structure is in `m` and `Ψ`.
- Effects are in `ε`.
- Things not discharged by the local checker become `O`.

### 4.2 Minimal mode inventory

Initial mode family candidates:

```text
local(p)        -- computation at place p
later           -- staged / next computation
stable          -- persistent / □-like availability
published(s)    -- visible at scope s
observed(s)     -- observed by scope/actor s
witnessed(w)    -- backed by witness w
durable(k)      -- backed by persistence / durable evidence kind k
```

Do not assume all of these must become primitive. The formal skeleton can represent them as an inductive `Mode` with later refactoring.

### 4.3 Effect row inventory

Initial effect families:

```text
perform(op, target)
publish(value, scope)
observe(ref, scope)
handoff(resource, from, to)
emit(event)
try_rollback
atomic_cut
barrier_candidate
durable_cut_candidate
```

`barrier_candidate` and `durable_cut_candidate` must remain later-family unless source-backed adoption exists.

### 4.4 Obligation inventory

Initial obligation kinds:

```text
same_lineage_floor
missing_option_structure_floor
capability_strengthening_floor
try_atomic_cut_structural_floor
canonical_normalization_law
no_repromotion
rollback_cut_non_interference
hidden_rollback_absence
authority_handoff_requires_witness
late_join_visible_history
stale_reconnect_refresh
no_double_owner
ifc_no_illicit_explicit_flow
```

Map these to:

```text
checker
Lean theorem
model-check carrier
runtime policy
```

### 4.5 Why this is not flat fusion

Do **not** write the theory as “MTT + Koka + Liquid + Jif + memory_order all at once.”

Instead:

```text
MTT/MDTT-like spine       = core judgement and modes
linear capability         = Δ discipline
Koka-like effects         = ε discipline
Liquid-like refinements   = decidable fragment in O/checker
Jif/Flow-Caml-like IFC    = label constraints and pc-flow
C++ memory_order family   = backend/reference mapping, not source principal
Lean/Rocq/Apalache        = proof/model-check tooling layers
```

This preserves a single clean theory while avoiding an unprincipled fusion.

---

## 5. Formal definitions to write first

### 5.1 Core structures

In docs and Lean skeleton, define at least:

```lean
abbrev EventId := Nat
abbrev PlaceId := Nat
abbrev ActorId := Nat
abbrev ResourceId := Nat
abbrev WitnessId := Nat

inductive FailureKind where
  | reject
  | approximate
  | compensate

inductive CutKind where
  | atomic
  | barrierCandidate
  | durableCandidate

inductive Effect where
  | perform    : String -> String -> Effect
  | publish    : String -> String -> Effect
  | observe    : String -> String -> Effect
  | handoff    : ResourceId -> ActorId -> ActorId -> Effect
  | emit       : String -> Effect
  | atomicCut  : Effect

inductive Mode where
  | local      : PlaceId -> Mode
  | later      : Mode -> Mode
  | stable     : Mode -> Mode
  | published  : String -> Mode
  | observed   : String -> Mode
  | witnessed  : WitnessId -> Mode
  | durable    : String -> Mode
```

These names are placeholders; do not treat them as final public API.

### 5.2 Event DAG

Conceptual definition:

```text
EventDAG = {
  events : FiniteSet Event
  edge   : EventId -> EventId -> Prop
  acyclic : no cycle in edge
}
```

Required relations:

```text
program_order(e1,e2)
dependency_order(e1,e2)
publication_order(e1,e2)
observation_order(e1,e2)
witness_order(e1,e2)
finalization_order(e1,e2)
scoped_happens_before(scope,e1,e2)
```

Derived rule example:

```text
if publication_order(roll, publish_roll)
and witness_order(publish_roll, handoff)
and observe(B, handoff)
then roll ∈ causal_past(B, handoff)
```

### 5.3 Handoff contract

Minimal handoff contract:

```text
handoff(resource r, from A, to B)
requires:
  owner(r) == A
  published(prereq)
  witness(prereq)
ensures:
  owner(r) == B
  prereq ∈ causal_past(handoff)
  observation(handoff) implies observation(prereq) or witness(prereq)
```

For the dice example:

```text
roll_event = roll(draw via authority_rng)
publish_event = publish(draw)
handoff_event = handoff(dice_owner, A, B)

requirements:
  after(handoff_event, publish_event)
  witness(handoff_event, publish_event)
  authority(owner_slot(dice_owner))
```

### 5.4 IFC label model

Docs-level interface:

```text
LabelModel = {
  Label      : Type
  flows_to   : Label -> Label -> Prop
  join       : Label -> Label -> Label
  meet       : Label -> Label -> Label
  flows_dec  : Decidable (flows_to a b)
  laws       : preorder + semilattice laws
}
```

Typing ideas:

```text
Γ ; pc = ℓpc ⊢ x : Labeled ℓx A
flows_to (join ℓpc ℓx) ℓdest
---------------------------------
Γ ⊢ publish_to(dest, x) allowed
```

Declassification:

```text
Γ ; Δ ⊢ cap : Declassify ℓhigh ℓlow @ m
Γ ⊢ x : Labeled ℓhigh A
--------------------------------------
Γ ; Δ ⊢ declassify cap x : Labeled ℓlow A
```

Secret key rule sketch:

```text
SecretKey : Type
SecretKey @ High : linear
No Copy
No Publish Low unless explicit Declassify High Low capability exists
```

---

## 6. Proof obligations

### 6.1 Core metatheory obligations

Eventually prove:

```text
Type preservation
Progress-or-explicit-failure
Linearity preservation
No resource duplication under fallback / rollback / handoff
No rollback across atomic_cut
No re-promotion in same-lineage chain
Elaboration soundness from companion surface to core IR
Effect-row soundness
IFC explicit-flow noninterference for first fragment
Checker soundness for decidable fragment
```

### 6.2 What does “progress” mean here?

Because Mir has external effects and explicit failure, do not use a naive value-or-step theorem.

Use:

```text
If Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O,
then e is one of:
  - a value
  - can take a semantic step
  - is blocked on an explicit external effect request
  - has produced an explicit structured failure outcome
  - has been stopped by a static gate before runtime evaluation
```

### 6.3 Checker soundness

For first checker cut:

```text
checker_accept(program) => declarative_well_formed(program)
```

But completeness only for the decidable fragment:

```text
declarative_well_formed_in_fragment(program) => checker_accept(program)
```

Do not claim completeness for the full language.

### 6.4 Theorem/model-check extraction soundness

If a program produces obligation rows, prove or plan to prove:

```text
extracted_obligation(row, program) => row corresponds to a real semantic property of program
```

For model-check carrier:

```text
finite_projection_safe(program, carrier) => any carrier counterexample maps back to a real or conservative source-level issue
```

### 6.5 IFC soundness first target

First target should be explicit-flow safety, not full covert-channel freedom.

```text
If no authorized declassification exists from High to Low,
then a value labeled High cannot be directly published or assigned into Low-observable state.
```

Reserve:

```text
timing channels
termination channels
distributed observation leaks
fairness-dependent leaks
provider trust leaks
```

---

## 7. Syntax package

### 7.1 Principal syntax: edge-row / vertical continuation

Use as main experimental companion surface:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Static checks:

```text
roll draw exists before publish(draw)
publish(draw) exists before handoff
handoff has owner authority
handoff has witness(draw)
no stale owner handoff
no duplicate active owner
```

Expected event skeleton:

```json
{
  "static_verdict": "valid",
  "events": [
    {"kind":"roll", "id":"draw"},
    {"kind":"publish", "subject":"draw"},
    {"kind":"handoff", "resource":"dice_owner", "from":"A", "to":"B"}
  ],
  "relations": [
    ["program_order", "roll(draw)", "publish(draw)"],
    ["publication_order", "roll(draw)", "publish(draw)"],
    ["witness_order", "publish(draw)", "handoff(dice_owner,A,B)"],
    ["scoped_happens_before", "room", "roll(draw)", "handoff(dice_owner,A,B)"]
  ],
  "terminal_outcome": "success"
}
```

### 7.2 Secondary syntax: stage-block

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      after publish(draw)
      requires witness(draw)
}
```

Use when multiple stages need grouping.

Expected lowering:

```json
{
  "transition": "handoff_turn",
  "stages": ["roll", "publish", "handoff"],
  "stage_edges": [
    ["roll", "publish"],
    ["publish", "handoff"]
  ],
  "obligations": [
    "handoff_requires_witness",
    "handoff_after_publish",
    "single_owner_after_handoff"
  ]
}
```

### 7.3 Reserve sugar: serial scope

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

Interpretation:

```text
serial on dice_owner = authoritative serial transition scope over owner slot dice_owner
```

Do not interpret as raw shared-memory lock or `atomic_cut`.

### 7.4 Negative samples

Missing witness:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
```

Expected:

```json
{
  "static_verdict": "malformed",
  "checked_reason_codes": ["missing_handoff_witness"],
  "enters_evaluation": false
}
```

Stale reconnect accepted silently:

```text
member C incarnation 4 leaves
member C incarnation 5 joins
message from C incarnation 4 requests handoff
accept message
```

Expected:

```json
{
  "static_or_protocol_verdict": "reject_or_refresh_required",
  "reason": "stale_incarnation",
  "required_action": "refresh_membership_view"
}
```

Secret key leak:

```text
label High
label Low
secret sk : SecretKey @ High
publish sk to Low
```

Expected:

```json
{
  "static_verdict": "malformed",
  "checked_reason_codes": ["ifc_illicit_flow"],
  "enters_evaluation": false
}
```

Authorized declassification:

```text
label High
label Low
secret sk : SecretKey @ High
cap audit_release : Declassify High Low
pub <- declassify audit_release sk
publish pub to Low
```

Expected:

```json
{
  "static_verdict": "valid",
  "obligations": ["declassification_audit_required"],
  "terminal_outcome": "success"
}
```

---

## 8. Implementation packages

### Package A — Drift and document audit

Goal:

```text
Integrate FAQ08 and this handoff into specs/plan/progress/tasks without factifying FAQ wording.
```

Update / check:

```text
specs/00-document-map.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
specs/12-decision-register.md only if evidence supports new rows
plan/01-status-at-a-glance.md
plan/06-surface-notation-status.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
progress.md
tasks.md
Documentation.md
```

Report must say what was updated and what was intentionally not updated.

### Package B — Theory spine docs

Create / update:

```text
specs/examples/<next>-mir-theory-spine-multimodal-dependent-core.md
```

Must include:

- `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O`
- mode inventory
- effect row inventory
- obligation inventory
- relation to Mir-0 / Mir-1 / Mirrorea
- non-goals
- formalization plan
- stop lines

### Package C — Strong typing / IFC design docs

Create / update:

```text
specs/examples/<next>-mir-strong-typing-layered-verification-design.md
```

Must include:

- layer architecture
- `LabelModel`
- secret-key examples
- checker vs theorem split
- proof obligations
- negative corpus

### Package D — Lean formal skeleton

Create if absent:

```text
formal/lean/lakefile.lean
formal/lean/Mir/Core.lean
formal/lean/Mir/Modes.lean
formal/lean/Mir/Effects.lean
formal/lean/Mir/Typing.lean
formal/lean/Mir/Obligations.lean
formal/lean/README.md
```

Allowed:

```text
sorry placeholders
experimental namespace
non-production markers
```

Required theorem placeholders:

```text
preservation
progress_or_explicit_failure
linearity_preservation
no_rollback_across_atomic_cut
no_repromotion
checker_soundness_fragment
refinement_fragment_decidable
ifc_no_illicit_explicit_flow_fragment
elaboration_soundness_surface_to_core
```

If Lean toolchain unavailable, report it. Do not fake validation.

### Package E — Problem 1 implementation closeout

Goal:

```text
Move verifier-boundary / typed-theorem-model-check line to implementable first tranche.
```

Do:

- integrate `p06`
- update property-to-boundary matrix
- add typed/effect/contract/IFC markers to helper-local or prototype carrier if needed
- theorem-first Lean preflight route
- model-check second-line carrier inventory

Do not:

- freeze final public verifier contract
- make stronger typed surface final source principal

### Package F — Problem 2 implementation closeout

Goal:

```text
Implement order/handoff relation family enough for authoritative-room first scenario.
```

Do:

- implement / update helper-local experimental source surfaces:
  - edge-row principal
  - stage-block secondary
  - serial sugar reserve if helpful
- update `p07` and `p08` mapping
- add relation-family carrier if needed
- keep low-level memory-order exact syntax out of source principal

Required static checks:

```text
witness target exists
after references prior event/stage
handoff requires prior publication or witness
handoff requires owner authority
late join sees published history as past
stale reconnect is not silently accepted
```

### Package G — Shared-space reserve strengthening

Current queue:

```text
auditable_authority_witness
delegated_rng_service
model-check second-line concretization
```

For `auditable_authority_witness`:

```json
{
  "witness_kind": "authority_rng_draw",
  "action_ref": "roll(draw)",
  "draw_slot": "turn_42",
  "draw_result": 5,
  "authority_ref": "room_authority",
  "provider_ref": null,
  "receipt_ref": null
}
```

For `delegated_rng_service`:

```json
{
  "authority_ref": "room_authority",
  "provider_ref": "rng_service_1",
  "request_ref": "rng_req_42",
  "draw_result": 5,
  "provider_receipt": "rng_receipt_42",
  "authority_commit": "handoff_commit_42"
}
```

Provider must not mutate room state. Authority remains owner of request / lock / commit / publish.

For model-check second line, first properties:

```text
late join sees published history
stale reconnect fails then refreshes
no handoff without witnessed publish
no double owner in authoritative transition
```

---

## 9. Sample corpus to add or verify

### 9.1 Authoritative dice handoff — valid

Input:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Expected static:

```json
{
  "verdict": "valid",
  "enters_evaluation": true,
  "required_obligations": [
    "handoff_after_publish",
    "handoff_requires_witness",
    "single_owner_transition"
  ]
}
```

Expected runtime / prototype:

```json
{
  "terminal_outcome": "success",
  "event_kinds": ["roll", "publish", "handoff"],
  "owner_after": "B",
  "relations": {
    "program_order": [["roll(draw)", "publish(draw)"], ["publish(draw)", "handoff(dice_owner,A,B)"]],
    "witness_order": [["publish(draw)", "handoff(dice_owner,A,B)"]]
  }
}
```

### 9.2 Missing witness — static stop

Input:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
```

Expected:

```json
{
  "static_verdict": "malformed",
  "enters_evaluation": false,
  "checked_reason_codes": ["missing_handoff_witness"]
}
```

### 9.3 Handoff before publish — static stop

Input:

```text
roll draw via authority_rng
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
publish draw
```

Expected:

```json
{
  "static_verdict": "malformed",
  "enters_evaluation": false,
  "checked_reason_codes": ["handoff_before_publication"]
}
```

### 9.4 Late join visible history

Scenario:

```text
A rolls draw=5
publish draw
handoff A -> B
C joins after handoff
C observes current room state
```

Expected:

```json
{
  "terminal_outcome": "success",
  "late_joiner": "C",
  "visible_history_contains": ["roll(draw=5)", "publish(draw)", "handoff(A->B)"],
  "current_owner": "B"
}
```

### 9.5 Stale reconnect refresh

Scenario:

```text
C incarnation=4 disconnects
C incarnation=5 reconnects
old message from C incarnation=4 attempts action
```

Expected:

```json
{
  "terminal_outcome": "Reject",
  "reason": "stale_incarnation",
  "refresh_required": true,
  "no_silent_merge": true
}
```

### 9.6 Secret key IFC valid / invalid

Invalid:

```text
label High
label Low
secret sk : SecretKey @ High
publish sk to Low
```

Expected:

```json
{
  "static_verdict": "malformed",
  "checked_reason_codes": ["ifc_illicit_flow"]
}
```

Valid with declassification:

```text
label High
label Low
secret sk : SecretKey @ High
cap release_key_fingerprint : Declassify High Low
fp <- declassify release_key_fingerprint (fingerprint sk)
publish fp to Low
```

Expected:

```json
{
  "static_verdict": "valid",
  "obligations": ["declassification_audit_required"],
  "terminal_outcome": "success"
}
```

---

## 10. Validation commands

Run the existing representative set where available:

```bash
cargo test -p mir-runtime --test current_l2_source_sample_runner
cargo test -p mir-runtime --test current_l2_operational_cli
cargo test -p mir-runtime --test current_l2_verifier_preview_alignment
cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor
cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor
cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization
cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight
cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring
cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization
cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface
cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface
python3 scripts/current_l2_source_sample_regression.py inventory
python3 scripts/current_l2_source_sample_regression.py regression
```

If new tests are added, add them to this list and update `progress.md`.

If Lean skeleton is added:

```bash
cd formal/lean
lake build
```

If Apalache/TLA+ carrier is added:

```bash
apalache-mc check specs/model-check/authoritative_room.tla
```

or repo-local wrapper if created.

Do not fake validation. If toolchain is unavailable, report exact reason.

---

## 11. Required documentation updates

At the end of the task, check all of these.

```text
specs/00-document-map.md
specs/10-open-questions.md
specs/11-roadmap-and-workstreams.md
specs/12-decision-register.md
plan/01-status-at-a-glance.md
plan/06-surface-notation-status.md
plan/07-parser-free-poc-stack.md
plan/08-representative-programs-and-fixtures.md
plan/09-helper-stack-and-responsibility-map.md
plan/12-open-problems-and-risks.md
plan/13-heavy-future-workstreams.md
plan/16-shared-space-membership-and-example-boundary.md
plan/17-research-phases-and-autonomy-gates.md
plan/90-source-traceability.md
progress.md
tasks.md
Documentation.md
docs/research_abstract/* if relevant
```

If a file is not updated, the report must explicitly say why.

---

## 12. Stop lines

This tranche may claim:

```text
theory layer actual package close
helper-local implementation close
authoritative-room first scenario close
Lean skeleton / theorem preflight close
model-check carrier inventory close
```

This tranche may **not** claim:

```text
final public parser grammar complete
final public parser/checker/runtime API complete
final public verifier contract complete
production theorem/model-check binding complete
installed binary / packaging complete
host integration complete
full shared-space final catalog complete
broader application target complete
```

---

## 13. Report requirements

Write a report including:

1. updated files
2. new files
3. unchanged files and why
4. current theory spine
5. Problem 1 closeout state
6. Problem 2 closeout state
7. syntax family state
8. proof assistant / formalization state
9. model-check second-line state
10. shared-space reserve strengthening state
11. validation results
12. remaining mixed gates
13. remaining true user-spec gates
14. next self-driven packages
15. whether first completion target is now satisfied
16. exact report path

---

## 14. Success checklist for CodeX

A successful run should leave the repo in this state:

```text
[ ] FAQ08 integrated as current explanation, not normative decision
[ ] principal theory spine documented
[ ] strong typing layer design documented
[ ] IFC first fragment documented
[ ] Lean skeleton created or explicitly deferred with reason
[ ] Problem 1 actual package updated
[ ] Problem 2 actual package updated
[ ] order/handoff syntax samples checked
[ ] p06/p07/p08 integrated into docs/status
[ ] auditable authority witness started or queued
[ ] delegated RNG service started or queued
[ ] model-check second-line concretization started or queued
[ ] specs/plan/progress/tasks/Documentation consistent
[ ] report written
[ ] validation run or tool unavailability reported
```

---

## 15. External reference pack for this tranche

Verify bibliographic details before writing normative docs. Use only as supporting research context, not as direct adoption.

### Theorem proving / formalization

- Lean 4 reference manual: Lean is an interactive theorem prover based on dependent type theory with a small kernel that checks proof terms.  
  Source: <https://lean-lang.org/doc/reference/latest/>
- The Rocq Prover 9.0.0 release notes: Rocq 9.0 completes the renaming from The Coq Proof Assistant to The Rocq Prover.  
  Source: <https://rocq-prover.org/releases/9.0.0>
- Apalache official site: symbolic model checker for TLA+ that translates TLA+ to SMT solver logic.  
  Source: <https://apalache-mc.org/>

### Modal / staged / dependent foundations

- Davies and Pfenning — modal analysis of staged computation.
- Yuse and Igarashi — `λ◯□` / modal type system with persistent code.
- Guarded lambda calculus.
- Modal Dependent Type Theory.
- Multimodal Dependent Type Theory.

### Effects / refinements / security

- Koka / row-polymorphic effect types.
- Liquid Types / decidable refinement checking.
- Jif / Decentralized Label Model.
- Flow Caml.
- RHTT or dependent information-flow type systems.

### Distributed order / memory models

- Lamport 1978 — happened-before and logical clocks.
- Chandy–Lamport 1985 — distributed snapshots / consistent cuts.
- Boehm–Adve 2008 — C++ concurrency memory model.
- WG21 `P0750` / `P3475` — consume / kill_dependency instability as public surface.
- HSA 1.2 — memory scopes.
- Herlihy–Wing 1990 — linearizability.
- Jeffrey–Riely event structures / relaxed memory.

---

## 16. Final instruction to Codex

Do not keep researching indefinitely.  
The task is not to invent an entirely new theory family.  
The task is to **close the current theory layer** using these defaults, update the repo, implement the helper-local pieces, start the formal skeleton, run validation, and report remaining gates honestly.

When a choice is sufficiently narrowed and has prototype evidence, move it to actual package closeout.  
When a choice still depends on implementation evidence, narrow it to one next experiment.  
When a choice is truly user-spec-dependent, leave it explicitly as such.


---

# Appendix A — Full previous integrated handoff preserved

# Codex handoff — theory closeout / principal direction / FAQ08-complete integrated brief (2026-04-18)

## 0. この文書の位置づけ

この文書は、現在の repo に関する **CodeX 向けの統合 handoff** である。  
目的は次の 3 点。

1. ここまでの議論、`faq_006.md` / `faq_007.md` / `faq_008.md`、`specs/`、`plan/`、current samples/prototypes から読める **現在地** を一つにまとめる。
2. 二大問題（Problem 1: 型 / 定理証明 / モデル検査境界、Problem 2: order / handoff / `memory_order` 再構成）について、**これ以上 compare-floor を増殖させず、actual adoption package を閉じるための principal direction** を明示する。
3. そのうえで、CodeX が `specs/` / `plan/` / `progress.md` / `tasks.md` / reports を更新しながら、**self-driven theory line を near-end まで自走**できるようにする。

この文書は **規範判断の正本ではない**。  
常に次の優先順位を守ること。

1. `specs/` = 規範正本
2. `plan/` = repository memory
3. `docs/reports/` = change rationale / historical trace
4. `progress.md` / `tasks.md` / `faq_*.md` = current explanation / snapshot

この文書と `specs/` が衝突したら、**必ず `specs/` を優先**すること。  
ただし、この文書は current reading を更新するための **explicit direction / defaults / package design** を含む。  
source-backed に反映可能なものは、relevant `specs/` / `plan/` / reports に戻すこと。

---

## 1. 一番短い現在地の読み

### 1.1 結論

- repo はもう docs-only abstract research ではない。
- parser-free current L2、compile-ready minimal actualization、fixed-subset source sample、corrected prototype、compare floor、actual adoption package、helper-local actualization / narrowing floor まである。
- 二大問題の **current first line** は source-backed に選ばれ、actual adoption package と helper-local actualization floor まで machine-check 済み。
- しかし、**二大問題の final adoption** と **final public language / tool / app completion** はまだ終わっていない。
- 現在地は  
  **near-end theory closeout の後半、reserve strengthening / second-line / mixed gate の入口**  
  と読むのが最も誤読が少ない。
- 研究の主たる debt は discovery debt ではなく **selection/adoption debt** である。
- したがって、ここから先の主眼は「新しい理論候補を増やすこと」ではなく、**principal spine を固定し、compare-floor を stop line で止め、actual adoption package を閉じること**である。

### 1.2 FAQ 08 の核心を一文で言うと

**「二大問題の current first line は source-backed に選ばれ、actual adoption package と helper-local actualization floor まで machine-check 済みだが、final public language / tool / app completion はまだ mixed gate と user-spec gate に残っている。」**

---

## 2. current repo の規範境界（絶対に崩さないもの）

### 2.1 全体アーキテクチャ

- **Mir** が主眼の意味論コア。
- **Mirrorea** は distributed control / routing / audit / overlay / patch の fabric。
- **PrismCascade** は独立 kernel。Mir runtime に早期統合しない。
- **Typed-Effects Wiring Platform** は lower/adjacent operational layer。Mir そのものではない。
- shared space / VRSNS / Reversed Library は上位 layer / application track。

### 2.2 Mir-0 / Mir-1 / Mirrorea の切り分け

Mir-0 に残すもの:
- event DAG
- `place`
- minimal effect request operation (`perform` は説明用 token)
- effect / contract
- minimal structured failure space
- primitive fallback
- local `try` / rollback
- place-local `atomic_cut`
- linear resource

Mir-0 に入れないもの:
- `barrier`
- `durable_cut`
- full fallback normalization
- distributed scheduler
- rich coroutine / emit / overlay detail

Mirrorea に残すもの:
- logical naming
- route rebinding
- overlay registration
- patch activation
- audit / path proof
- distributed realization

### 2.3 `atomic_cut` の current meaning

`atomic_cut` は **place-local finalizing cut nucleus**。  
これは次を意味しない。

- global consistent cut
- process-wide synchronization point
- distributed agreement point
- durable commit

`atomic_cut` は current `place` の rollback frontier を確定するだけであり、以後の higher-level ordering / witness / fairness / scheduler / commit family を背負わせない。

### 2.4 fallback / `lease` の current meaning

- fallback = guarded option chain
- same-lineage chain = left-to-right monotone degradation
- no re-promotion
- `lease` = option-local lifetime guard
- `lease-expired` は option-local miss / non-admissible metadata であり request-level `Reject` そのものではない
- request-level `Reject` は well-formed chain が admissible success を返せず尽きたときの final outcome
- `try` / rollback / `atomic_cut` は degradation order を巻き戻さない

### 2.5 proof / checker boundary の current meaning

current principal split:
- `core_static_checker`
- `theorem_prover_boundary`
- `protocol_verifier_boundary`
- `runtime_policy_boundary`

local / structural / decidable fragment は checker へ。  
global law / fairness / protocol / richer property は theorem/model-check/runtime policy へ。

### 2.6 syntax の current meaning

- current L2 notation は **companion notation**
- final parser grammar ではない
- explicit edge-row family を current polished first choice とする
- compactness より **semantic honesty** を優先
- final grammar / final public parser API / final keyword set はまだ OPEN

---

## 3. FAQ 08 への完全統合回答

この節は `faq_008.md` の内容を **ここでの handoff 用に完全統合したもの** である。

### 3.1 何が終わっているか

#### floor 読み

現在の floor を誤読なく言い換えると次。

| floor | 現状 | 読み |
|---|---|---|
| compare floor | close | current first line / stop line / mixed gate を compare 可能な形で固定済み |
| actual adoption floor | close | 二大問題と syntax/modality の current recommendation は docs-first actual package に上がっている |
| helper-local actualization / narrowing floor | close on current tranche | theorem-first pilot、room default vertical slice、minimal companion、stage-block secondary、theorem-preflight を helper-local に actualize 済み |
| reserve strengthening floor | active | `auditable_authority_witness`、`delegated_rng_service`、model-check second line が current queue |
| final public language / tool / app floor | not close | parser / public API / final verifier contract / concrete external tool / packaging / host / broader app は未 close |

#### current runnable / machine-check evidence

少なくとも次は current state で runnable / machine-check の evidence を持つ。

- current authored sixteen  
  `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype octet  
  `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08`
- compare floor
  - verifier preview alignment pre-floor
  - model-check projection pre-floor
  - theorem discharge pre-floor
- helper-local actualization / narrowing floor
  - theorem-first experimental pilot actualization
  - theorem-prover experimental binding preflight
  - authoritative-room vertical-slice actualization
  - minimal companion / experimental order-handoff surface
  - stage-block secondary order-handoff surface

#### 2026-04-18 fresh validation

少なくとも current explanation では次が fresh pass と読まれている。

- `cargo test -p mir-runtime --test current_l2_source_sample_runner` → `22 passed`
- `cargo test -p mir-runtime --test current_l2_operational_cli` → `12 passed`
- `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization` → `5 passed`
- `cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight` → `4 passed`
- `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring` → `9 passed`
- `cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization` → `3 passed`
- `cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface` → `3 passed`
- `cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface` → `3 passed`
- `python3 scripts/current_l2_source_sample_regression.py inventory` → authored sixteen present
- `python3 scripts/current_l2_source_sample_regression.py regression` → all regression commands passed

要するに、「実装・実行による比較検証がまだ薄い」という読みはもう正しくない。

### 3.2 何がまだ終わっていないか

#### Problem 1 で未完了

source-backed に close 済み:
- verifier-boundary matrix current first line
- checker-adjacent semantic carrier principal
- source-visible structural marker family first
- notebook-first theorem line
- row-local model-check carrier first
- `p06` bridge-floor evidence
- verifier preview alignment pre-floor
- model-check projection pre-floor
- theorem discharge pre-floor
- theorem-first pilot actualization
- theorem-preflight

未完了:
- stronger typed surface actual adoption
- theorem discharge transport / public-contract actual format
- concrete theorem prover brand / binding
- settled property language
- concrete model-check tool seam
- final public verifier contract
- proof object public schema

つまり、「Problem 1 の current recommendation はある」が、「final public theorem/model-check language/tool contract はまだない」。

#### Problem 2 で未完了

source-backed に close 済み:
- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread/node parity current safer wording
- low-level `std::memory_order` / `std::kill_dependency` retained-later reference line
- `p07/p08` bridge-floor evidence
- authoritative room first actual default
- vertical-slice actualization
- minimal companion / stage-block floor

未完了:
- final source-surface handoff wording
- final emitted-artifact schema / handoff contract
- exhaustive shared-space final catalog
- stronger fairness / replay operational theorem or profile
- low-level family public stance の最終 cut

つまり、「Problem 2 の decomposition と default profile はある」が、「final public wording と final public handoff contract はまだない」。

#### 言語側実装として未完了

もし「言語側の実装まで終わっている」と言うなら、少なくとも以下が必要だが、まだ無い。

- final parser grammar
- final public parser API
- final public checker API
- final public runtime API
- final public verifier contract
- final source-surface order/handoff wording
- stronger typed surface final disposition
- concrete theorem/model-check production bindings
- packaging / installed binary / host integration completion

### 3.3 overall ladder で今どこか

current best reading:
1. candidate comparison / drift audit → close
2. current first line の actual adoption package 化 → close
3. helper-local actualization / narrowing → current tranche close
4. reserve strengthening / second-line concretization → active
5. final public seams の mixed-gate actual adoption → 未 close
6. final public language / toolchain integration → 未 close
7. broader host / packaging / application realization → user-spec dependent で未 close

macro phase 言い換え:
- execution lane: `Macro 4 active on fixed authored/prototype floor`
- theory-lab lane: `Macro 5 near-end actual-adoption closeout`
- reserve integration lane: `Macro 6 minimal working subset actual default / Macro 7 mixed`

### 3.4 何を答えればさらに長く自走できるか

`faq_008` が一番効くと整理している問いは次。

true user-spec / target side:
1. first completion に packaging / host integration を含めるか
2. authoritative-room first scenario を first completion の acceptance と見なしてよいか
3. broader application target をすぐ要求するか、後段に送ってよいか
4. theorem-first concrete binding を早めに actualize してよいか
5. model-check second line を theorem-first close と同 tranche に置くか
6. final public parser/checker/runtime API をこの closeout の必須条件にするか

現在の解釈:
- これらに答えればかなり先まで自走しやすくなる
- しかし remaining mixed gate は evidence-dependent なので、「これだけで最後まで完全 no-question」はまだ無理

---

## 4. ここまでの議論を踏まえた principal direction（この文書の主張）

この節は、ここまでの対話を踏まえて **assistant 側が責任を持って勧める主軸方針** である。  
以後、CodeX はこれを **current default direction** として使ってよい。  
ただし、`specs/` の settled judgment と正面衝突する場合は report で conflict を明示すること。

### 4.1 原則

- 候補理論を横並びで「全部そのまま融合」してはいけない。
- 一つの **judgmental spine** を principal にし、その上に effect / refinement / IFC / theorem/model-check boundary を attachment layer として載せる。
- 多段階計算は core 寄りである。plugin 的扱いにしない。
- information-flow / security / richer contract は重要だが、first checker cut に全部押し込まない。
- final grammar は theory より後。
- final public implementation は theory closeout と別 lane。
- mainline actualization と theory-lab closeout を分ける。

### 4.2 principal spine

**principal spine = multimodal dependent core**

理由:
- `λ◯□` は stage/later/always の partial basis として非常に有力
- しかし full language は
  - stage
  - place
  - visibility/publication
  - witness/audit
  - durability/cut
  - authority/handoff
  と多軸
- したがって full basis は `λ◯□` 単独では不足し、multimodal dependent type theory 系がより自然

#### current recommendation

- `λ◯□` = partial basis candidate
- guarded lambda-calculus = strong candidate for later/guarded/coinductive line
- Modal Dependent Type Theory = strong candidate for dependent modal core
- Multimodal Dependent Type Theory = strongest current candidate for full spine
- Fitch-style multimodal basis = keep as formulation family

### 4.3 layered theory stack

私が推奨する理論 stack は次。

1. **Multimodal dependent core**
   - stage
   - place
   - stable/later-like structure
   - mode-indexed typing judgment

2. **Linear / affine capability layer**
   - ownership
   - move / borrow / delegation constraints
   - capability monotonicity
   - no hidden duplication

3. **Row-polymorphic effect layer**
   - `perform`
   - publication / observation / handoff / emit / cut 等の effect family
   - effect inference / polymorphism

4. **Decidable refinement / contract layer**
   - `require` / `ensure`
   - option-local admissibility
   - checker-friendly decidable fragment
   - refinement obligations

5. **Information-flow / security-label layer**
   - user-defined label lattice / principal model
   - `Labeled ℓ A`
   - explicit declassification / endorsement
   - compile-time info-flow checks

6. **Theorem / model-check bridge**
   - residual proof obligations
   - protocol properties
   - fairness/replay/global law
   - machine-facing or notebook-facing artifacts

### 4.4 principal judgment sketch

Use this as the conceptual reference judgment:

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

Where:

- `Ψ` = mode theory / world assumptions
- `Γ` = unrestricted + dependent context
- `Δ` = linear / ownership context
- `m` = mode (at least stage/place; later may include visibility/witness-oriented refinements)
- `ε` = effect row
- `O` = residual obligations not discharged by the core checker

Interpretation:
- `Γ/Δ` split handles linearity and ownership.
- `m` handles stage/place/modal positioning.
- `ε` handles effect families.
- `O` is the hook to theorem/model-check/runtime-policy boundaries.

This exactly matches the current repo’s intuition that there is a **checker floor** and then **residual proof obligations**.

---

## 5. metatheory design: layer compatibility / compilation / composition

あなたが指摘した通り、局所的に異なる理論層を入れ、後で連携させるなら、**それらの互換性を示すメタ理論** が必要。  
ここは必ず整備すること。

### 5.1 基本構図

各 module / component / source region は、必要な theory layer を持ってよい。  
だが compilation / linking / external boundary では、以下を明示する。

- what modal worlds / modes the module assumes
- what effects it may emit
- what obligations it discharges itself
- what obligations it exports
- what security / label policy it assumes
- what handoff/publication/witness guarantees it requires/provides

### 5.2 必要な compatibility theorems

最低限、次の meta-theory packages が必要。

1. **elaboration soundness**
   - richer source forms / syntax sugar / typed markers / contracts / IFC annotations が core IR へ落ちても typing と semantics が保存される

2. **module interface well-formedness**
   - imported/exported effect rows, label constraints, ownership/capability assumptions, modal assumptions が interface で一致する

3. **cross-layer preservation**
   - capability layer / effect layer / refinement layer / IFC layer を stacked しても preservation が壊れない

4. **obligation export soundness**
   - module A で未解決として輸出した obligation を module boundary で明示できる
   - module B / theorem / model-check side がそれを正しく引き受ける

5. **boundary checking soundness**
   - external boundary checks が source-level assumptions を破らない
   - host / tool / artifact seams で hidden strengthening が起きない

### 5.3 実装上の基本方針

- first implementation では monolithic full calculus を作らない
- core calculus + elaborator + module manifest/interface + obligation artifacts で進める
- richer layers は syntax に直結するより पहले IR / metadata / manifest / artifacts に寄せる
- module-local principal theory と cross-module surface を混ぜない

---

## 6. Problem 1 — 型 / 定理証明 / モデル検査

### 6.1 current reading

現在 close 済み:
- checker-adjacent principal
- source-visible structural marker family first
- notebook-first theorem line
- row-local model-check carrier first
- `p06` bridge-floor evidence
- verifier preview / theorem discharge pre-floor / model-check projection pre-floor

まだ mixed gate:
- stronger typed surface actual adoption
- theorem discharge transport/public-contract actual format
- settled property language
- concrete theorem prover binding
- concrete model-check seam
- final public verifier contract
- proof object public schema

### 6.2 principal direction for Problem 1

#### principal recommendation

- **first checker cut** は local / structural / decidable fragment に限定する
- stronger typed surface は early source principal には上げない
- theorem-first external integration target を採る
- notebook-first theorem line + symbolic refs + preflight manifest を current principal bridge とする
- row-local model-check carrier を second-line concretization target にする
- theorem/model-check public contract は final adoption まで上げない

#### why

- current repo has bridge-floor evidence (`p06`, previews, theorem preflight)
- but no final public theorem/model-check contract
- premature stronger surface adoption would freeze the calculus too early
- theorem-first line gives the best near-end closeout ratio

### 6.3 static verification architecture

#### checker (must be low-cost / natural)

Keep in checker:
- same-lineage static floor
- missing option structure floor
- capability strengthening prohibition
- `try` / `atomic_cut` structural floor
- local contract shape
- minimal predicate fragment
- module interface consistency
- straightforward info-flow / label-flow fragment
- linear/capability floor

#### theorem side

Send to theorem:
- canonical normalization law (general form)
- no re-promotion global law
- rollback-cut non-interference
- stronger typing theorems
- module composition theorems
- IFC richer noninterference / declassification soundness
- modal coherence laws
- elaboration correctness in stronger formulations

#### model-check side

Send to model-check:
- bounded protocol properties
- replay / stale reconnect / late join scenarios
- fairness policy comparisons
- room-profile dynamics
- bounded handoff obligations
- specific order/handoff scenario checks

#### runtime policy

Leave in runtime policy:
- actual transport / retry / resend / timeout / backoff
- host/tool integration policy
- deployment-specific fairness / availability knobs

### 6.4 strong typing recommendation

You explicitly want strong typing. Current recommendation:

- yes to strong typing
- no to “put everything into one first-core huge dependent calculus”

Use:
- linear / capability typing in the core
- effect rows in the core+inference layer
- decidable refinement for first checker cut
- IFC labels / user-defined lattice in the first checker cut
- richer dependent proofs / theorem side for the rest

### 6.5 IFC / security-label plan

This should be a first-class package, not a vague future wish.

#### principal design

Introduce a user-definable label model:

```text
LabelModel = {
  Label       : Type
  flows_to    : Label -> Label -> Prop
  join        : Label -> Label -> Label
  meet        : Label -> Label -> Label
  flows_dec   : Decidable (flows_to a b)
  laws        : preorder / semilattice laws
}
```

Then allow types like:

```text
Labeled ℓ A
```

or a judgmental annotation style:

```text
Γ ; Δ ⊢ e : A @ m ! ε ▷ O [pc = ℓ]
```

#### first checker obligations

Must statically check:
- explicit data flow respects `flows_to`
- pc-flow respected
- move / share of labeled capabilities respects policy
- declassification / endorsement require explicit authority capability
- secret-key-like capabilities do not escape to broader labels without explicit authority

#### theorem side obligations

Keep for theorem / richer proof:
- covert channel considerations
- protocol-level information release through replay/fairness behavior
- stronger noninterference theorems
- distributed trust reasoning

### 6.6 what to prove for Problem 1

#### core metatheory
- preservation
- progress (with explicit external-effect/open-world refinement)
- linearity / no duplication
- label preservation
- effect soundness
- elaboration soundness

#### algorithmic checker results
- soundness of checker for fragment
- completeness for the decidable fragment only
- decidability / inference results for current fragment

#### bridge results
- sound obligation extraction
- sound projection to theorem/model-check carriers
- conservative/bounded abstraction soundness for model-check line

### 6.7 current stop line for Problem 1

Near-end closeout for Problem 1 is reached when:
- first checker fragment is explicitly frozen
- theorem-first concrete binding actualization package exists
- stronger typed surface is either:
  - explicitly retained as non-principal experimental surface, or
  - explicitly adopted with evidence
- model-check second line reaches first settled seam
- remaining work is final public contract / public API / production tool binding only

---

## 7. Problem 2 — order / handoff / `memory_order` 再構成

### 7.1 current reading

close 済み:
- cut family decomposition
- relation decomposition principal
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread/node parity safer wording
- low-level `std::memory_order` / `std::kill_dependency` retained-later reference line
- `p07/p08` evidence
- authoritative room first actual default
- vertical-slice actualization
- minimal companion / stage-block floor

still mixed:
- final source wording
- final emitted-artifact schema / handoff contract
- fairness / replay final operational profile
- exhaustive shared-space catalog
- low-level family final public stance

### 7.2 principal direction for Problem 2

#### principal recommendation

- principal semantics = **relation decomposition**
- principal operational family = **authority-serial transition**
- second strengthening family = **witness-aware commit/handoff**
- low-level `memory_order` family = **backend-aligned retained reference family**
- thread/node parity wording = **same causal language; different lowering/evidence/policy**

### 7.3 cut family

Keep separate:
- `atomic_cut`
- `barrier`
- `durable_cut`
- optional comparison candidate: `snapshot_cut` / `consistent_cut`

Interpretation:
- `atomic_cut` = local rollback-stop / local finalization nucleus
- `barrier` = ordering-only candidate
- `durable_cut` = commit/evidence-bearing family
- `snapshot_cut` = observation/snapshot-only candidate (comparison only unless source-backed later)

Do **not** collapse these.

### 7.4 relation family

The conceptual order family should explicitly include:

- `program_order`
- `dependency_order`
- `publication_order`
- `observation_order`
- `witness_order`
- `finalization_order`
- `scoped_happens_before`

This lets source language stay high-level while backend/reference lines preserve low-level orderings.

### 7.5 thread/node parity

Use the safer wording:

> thread と node は同じ causal language で書く。  
> 差は lowering / evidence / transport / failure / durability / policy に残す。

Do **not** write `thread == node`.

### 7.6 authority / handoff structure

The structure that should become principal is:

- owner slot
- transition stage family
- stage sequence
- stage-local obligation
- handoff epoch ref
- witness-aware handoff
- replay attachment
- payload ref

This is already aligned with the retained theorem-line bridge docs around `authority_transition_stage_family`, `stage_local_obligation_family`, witness-aware handoff, replay attachment, payload refs, etc.

### 7.7 fairness / replay first default

Use the first default profile:

- activation = `authority-ack`
- authority placement = `single room authority`
- consistency mode = `authoritative serial transition`
- RNG = `authority_rng`
- late join = visible history as past
- stale reconnect = fail then refresh
- replay = stale/incompatible replay invalidated, not silently merged
- no distributed fairness theorem required in first completion line

Reserve strengthening:
- `auditable_authority_witness`
- `delegated_rng_service`
- later maybe broader fairness families

### 7.8 what to prove for Problem 2

#### core semantic results
- cut separation soundness
- no hidden rollback across `atomic_cut`
- relation family coherence
- authority-serial transition soundness
- witness requirement preservation

#### checker fragment
- structural stage sequence well-formedness
- required witness presence
- owner-slot / authority-scope structural floor
- local order/handoff static obligations

#### theorem side
- handoff requires prior publication law
- witness-aware handoff correctness
- no invalid replay across epochs/incarnations
- stronger fairness/replay theorems (optional later)

#### model-check side
- late join / visible history
- stale reconnect fail-then-refresh
- race/replay bounded scenarios
- authority-handoff protocol bounded safety

### 7.9 stop line for Problem 2

Near-end closeout for Problem 2 is reached when:
- relation decomposition is fixed as principal
- source wording narrows to one principal family
- `p07/p08` plus authoritative-room vertical slice are fully integrated in docs/reports/tests
- first fairness/replay operational default is written into docs
- remaining work is final public wording / broader catalog / low-level public stance only

---

## 8. syntax / semantics / modality

### 8.1 core principle

**theory first, syntax second**

Do not let existing languages dictate the surface.  
Do not let parser polish freeze the semantics early.

### 8.2 syntax evaluation axes

Every syntax candidate must be evaluated on:

1. semantic honesty
2. checker legibility
3. modal adequacy
4. misreading resistance
5. lowering friendliness

### 8.3 principal syntax family

#### principal
**explicit edge-row / vertical continuation**

Example:

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

Why principal:
- edge-local info stays edge-local
- static checking is syntax-directed
- aligns with explicit edge-row tradition already chosen for fallback
- avoids packed rows

#### strong secondary
**stage / after / witness stage-block**

Example:

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      requires witness(draw)
}
```

Why secondary:
- readable
- good for explanation and stage structure
- closer to `λ◯□`-like intuition
- but slightly more block-heavy and may tempt over-structuring

#### reserve sugar
**authoritative-room serial sugar**

Example:

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

Why reserve:
- probably good for authoritative room ergonomics
- but too high-level to be principal core surface right now

### 8.4 modal foundation recommendation

- `λ◯□` = partial basis candidate for stage/later/always
- guarded λ-calculus = keep strong candidate for guarded recursion / coinductive traces
- MDTT = strong candidate for dependent modal core
- MTT = strongest current candidate for full multimodal spine
- final calculus adoption = **not in this tranche**

### 8.5 what to prove here

- elaboration from chosen companion/source form to core IR preserves typing and semantics
- syntax sugar desugaring is coherent
- modal annotations / source markers elaborate soundly
- companion forms do not introduce stronger semantics than the core

---

## 9. proof plan and formalization strategy

### 9.1 Lean-first recommendation

Use **Lean-first** for:
- core metatheory
- typing judgments
- elaboration correctness
- checker fragment correctness
- modal/dependent core experiments
- obligation extraction proofs

Use **Rocq/Coq + Iris fallback** only if/when:
- you need very strong concurrent separation logic for runtime internals
- you need an Iris-style existing ecosystem

Do not make Isabelle the principal route for this repo line.

### 9.2 staged proof roadmap

#### Stage A — paper spec / mechanization-ready core
- define core syntax / semantics / typing in prose + pseudocode
- define judgmental spine
- define module/interface and obligation export shapes

#### Stage B — Lean core
- preservation
- progress (open-world/external-effect aware)
- linearity / ownership soundness
- effect soundness
- elaboration soundness

#### Stage C — checker fragment
- algorithmic checker soundness
- decidability proofs where needed
- limited completeness for chosen fragment

#### Stage D — obligation/bridge layer
- sound extraction to theorem/model-check obligations
- row-local carrier/projection soundness
- bounded model-check abstraction soundness

#### Stage E — Rust/Lean alignment
- differential testing
- property-based testing
- artifact conformance
- trace alignment on representative corpus

### 9.3 what does NOT need proof before implementation

Do **not** block implementation on:
- full final calculus adoption
- full final public parser/runtime API proof
- complete global fairness theorems
- exhaustive shared-space catalog proofs

Instead:
- implement against the current principal spine
- prove the core
- use runner/prototypes/tests as evidence
- leave mixed/public/application gates explicit

---

## 10. completion target defaults (assistant recommendation)

These are the defaults I recommend using unless a strong source-backed conflict appears.

### 10.1 first completion definition

**first completion = authoritative-room first scenario**

Include:
- repo-local CLI + tests + artifacts + compare floor
- authoritative-room vertical slice
- theorem-first concrete binding actualization package
- first settled order/handoff source family
- current principal theorem/model-check bridge

Do **not** require:
- installed binary / packaging
- FFI / engine adapter
- broad host integration
- exhaustive shared-space catalog
- final public parser/checker/runtime API

### 10.2 first application target

- authoritative-room first scenario = yes
- append-friendly contrast room = yes, but as contrast not as equally heavy first target
- broader app target = later
- theorem-first integration target = yes
- model-check second line = after theorem-first close or as second strengthening package

### 10.3 problem-specific defaults

#### Problem 1 defaults
- stronger typed surface stays non-principal for now
- checker-adjacent + structural markers first
- theorem-first actualization yes
- notebook-first line yes
- final public verifier contract not required for first completion

#### Problem 2 defaults
- relation decomposition principal
- authority-serial first
- witness-aware second
- edge-row principal source family
- stage-block strong secondary
- serial sugar reserve
- low-level memory-order family retained-later reference only

---

## 11. what Codex should do next

This is the **actual execution program**.

### Package 0 — drift audit / queue reconstruction
Must do:
- reconcile `faq_006` / `faq_007` / `faq_008` with `specs/`, `plan/`, `tasks.md`, `progress.md`
- eliminate queue drift
- reflect current self-driven queue explicitly
- ensure `current self-driven queue != 0`

Expected outputs:
- updated `tasks.md`
- updated `progress.md`
- audit report
- `plan/01`, `plan/11`, `plan/17` sync if needed

### Package 1 — Problem 1 actual adoption closeout
Must do:
- freeze current first line wording for Problem 1
- update `specs/10`, `specs/11`, `specs/12` and relevant `specs/examples/*`
- integrate `p06`
- decide whether stronger typed surface remains experimental or gets a narrow adoption package
- move theorem-first line from preflight to actual experimental binding package
- keep final public verifier contract open, but make stop line explicit

Expected outputs:
- a current adoption note
- a theorem-first actualization package
- updated matrix of proof/checker/model-check responsibilities
- reports and docs alignment

### Package 2 — Problem 2 actual adoption closeout
Must do:
- freeze current first line wording for Problem 2
- integrate `p07`, `p08`, authoritative-room vertical slice
- write the principal source family as recommendation
- keep low-level memory-order family as retained reference mapping note
- define first fairness/replay operational default explicitly
- update shared-space docs and relevant examples

Expected outputs:
- cut family note
- relation family note
- authority/handoff note
- source wording recommendation
- updated shared-space defaults
- reports/tests/docs alignment

### Package 3 — syntax/modality convergence
Must do:
- write syntax-semantics principle explicitly
- record principal / secondary / reserve syntax families
- record modality recommendation:
  - `λ◯□` partial basis
  - guarded / MDTT / MTT / Fitch-style stronger families
- do not freeze final calculus
- do not start endless syntax comparisons

Expected outputs:
- one convergence doc
- updates to `plan/06`, `plan/13`, `specs/10`, maybe new `specs/examples/*`

### Package 4 — Lean/Rust proof plan package
Must do:
- write formal proof roadmap
- write what must be proven before/after implementation
- define Lean-first line and Rocq fallback line
- define artifact conformance and differential-testing strategy
- connect this to current runner/prototype evidence

Expected outputs:
- theory/proof plan doc
- updated roadmap and heavy workstream note
- explicit proof obligations list

### Package 5 — near-end closeout
Must do:
- summarize what is now close
- summarize remaining mixed gates
- summarize true user-spec gates
- define next reserve packages
- ensure no misleading “theory solved” wording appears anywhere

Expected outputs:
- final closeout report
- updated `plan/90-source-traceability.md`
- relevant snapshot updates

---

## 12. explicit “do not do” list

Codex must not:

- claim theory solved
- claim final public language implementation complete
- collapse `atomic_cut` into global cut / seq_cst / durable commit
- import low-level `memory_order` exact surface as source-principal wording
- freeze final parser grammar in this closeout tranche
- freeze final public parser/checker/runtime API in this closeout tranche
- freeze final public verifier contract in this closeout tranche
- finalize the exhaustive shared-space catalog
- treat `λ◯□` as sufficient full basis
- keep compare-floor alive when actual adoption package is already possible
- ignore queue drift
- skip plan/spec/report synchronization

---

## 13. current open but acceptable residuals

It is acceptable for the following to remain open after this tranche:

- final parser grammar
- final public parser/checker/runtime API
- final public verifier contract
- final public theorem/model-check production contract
- exhaustive shared-space final catalog
- broader fairness/distributed-fairness theorem
- packaging / installed binary / host integration target
- upper-layer application target beyond authoritative-room first scenario
- final calculus adoption among stronger modal foundations

The key is:
- these must be **explicitly named**
- they must not block near-end closeout of the theory line
- they must be separated into mixed gate / true user-spec gate / reserve integration

---

## 14. exact answers to the user positions already expressed

These are already-given preferences and should be treated as defaults unless conflict appears.

1. A single strong theoretical spine is preferred over a bag of unrelated theories.
2. Layering theories per module/region is acceptable, but compatibility metatheory must be built.
3. Multistage computation is core-ish and should be treated carefully in the principal theory.
4. Strong typing is desired.
5. Contract-programming-like aspects are desired.
6. Some dependent-typing-like power is desired.
7. User-defined security label models are desired.
8. Compile-time guarantees such as “a secret key is not shared outside policy scope” are desired.
9. Theory should be fixed before syntax.
10. Syntax should correspond cleanly to theory and semantics, not be dragged by existing languages.
11. Lean is preferred if possible.
12. Existing layers like security certification, visualization, host/adapters, etc. are known to exist as later layers.
13. Local theories per source area/module plus checked external boundaries is an acceptable architectural style.

Codex should **not re-open these as if they were unanswered**.  
If a conflict emerges, narrow it to one explicit issue and report it.

---

## 15. one-page summary for fast recall

If you only remember one page, remember this:

- principal spine = multimodal dependent core
- `λ◯□` = partial basis, not full basis
- linear capability + effect rows + decidable refinement + IFC labels = layered typing architecture
- checker / theorem / protocol / runtime policy split = principal verification architecture
- relation decomposition + authority-serial + witness-aware = principal order/handoff direction
- explicit edge-row / vertical continuation = principal syntax family
- stage-block = strong secondary syntax family
- serial sugar = reserve room-level sugar
- Lean-first = principal proof path
- Problem 1 and Problem 2 are **not solved**, but their current first lines are source-backed and machine-checked
- the main debt is adoption/selection, not discovery
- next move = close actual adoption packages, not invent more candidate families

---

## 16. Codex final response requirements

When finishing the next tranche, return at least:

1. updated files
2. new files
3. drift audit summary
4. current first line (Problem 1 / Problem 2 / syntax-modality)
5. retained alternatives
6. stop lines
7. remaining mixed gates
8. remaining true user-spec gates
9. queue reconstruction summary
10. tests / validation actually run
11. prototypes / samples used as evidence
12. next self-driven packages
13. reserve integration packages
14. report paths

And explicitly say:
- what is now actually close
- what is still mixed
- what is still target-dependent
- what you intentionally did **not** decide

---

## 17. appendices

### 17.1 candidate syntax snippets (non-final, but recommended comparison material)

#### A. explicit edge-row / vertical continuation

```text
roll draw via authority_rng
publish draw
handoff dice_owner A -> B
  after publish(draw)
  requires witness(draw)
```

#### B. stage-block secondary

```text
transition handoff_turn(owner = A) {
  stage roll:
    draw <- perform via authority_rng

  stage publish:
    publish draw

  stage handoff:
    change_owner dice_owner to B
      requires witness(draw)
}
```

#### C. serial sugar reserve

```text
serial on dice_owner {
  draw <- perform via authority_rng
  publish draw
  handoff dice_owner A -> B
    requires witness(draw)
}
```

### 17.2 conceptual type/interface sketches

#### judgment sketch

```text
Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ O
```

#### label model sketch

```text
LabelModel = {
  Label       : Type
  flows_to    : Label -> Label -> Prop
  join        : Label -> Label -> Label
  meet        : Label -> Label -> Label
  flows_dec   : Decidable (flows_to a b)
  laws        : preorder / semilattice laws
}
```

#### security-typed value sketch

```text
Labeled ℓ A
```

#### module boundary sketch

```text
module M :
  assumes  modes ΨM
  exports  types, effects, labels, obligations
  proves   discharged obligations OM_local
  requires imported obligations OM_ext
```

### 17.3 literature pack to keep in view

Modal / multimodal / staging:
- Davies–Pfenning
- Yuse–Igarashi (`λ◯□`)
- guarded lambda-calculus
- Modal Dependent Type Theory
- Multimodal Dependent Type Theory

Effects / refinement / IFC:
- Koka / row-polymorphic effects
- Liquid Types
- Jif / DLM
- Flow Caml
- RHTT-like dependent IFC references

Order / cuts / concurrency:
- Lamport 1978
- Chandy–Lamport 1985
- Boehm–Adve 2008
- HSA scoped memory model references
- WG21 `memory_order` / `consume` / `kill_dependency` line
- event-structure / relaxed-memory references
- linearizability references

---

## 18. final instruction

Use this document not as prose to archive, but as an **execution brief**.  
The right next move is not to reopen the whole design space.  
The right next move is to:

1. synchronize docs and queue,
2. lock the principal spine,
3. close actual adoption packages for the two big problems,
4. encode the proof plan,
5. leave only genuine mixed/user-spec gates visible.

That is the shortest route from the current state to a defensible near-end theory closeout.


---

# Appendix B — Full FAQ 08 preserved

# FAQ 08

## この文書について

この文書は、2026-04-18 時点の current reading に基づいて、

- 現状でどこまで何が終わっているか
- もう二大問題を完全に解決して、言語側の実装まで終わっていると言えるか
- 全体像に対して今どこにいるか
- そこへ持っていくために何がまだ必要か
- それらに答えれば repo がどこまで自走できるか

を、`faq_007.md` の次段として整理する FAQ である。

規範判断の正本は `specs/`、長期の repository memory は `plan/`、薄い snapshot は `progress.md` と `tasks.md` にある。
この FAQ 自体は **current explanation** であり、新しい規範判断を作る文書ではない。

今回の FAQ は、特に次を踏まえている。

- `specs/examples/473`
  - order / handoff source-facing narrowing
- `specs/examples/474`
  - theorem-prover experimental binding preflight
- 2026-04-18 再実行の representative validation
  - runner / CLI / compare floor / actualization floor / regression

---

## 1. 先に短く答えるとどうか

先に結論だけ言うと、次の読みが正確である。

1. **かなり進んでいる。**
   parser-free current L2、fixed authored sample、corrected prototype、compare floor、actual adoption package、helper-local actualization / narrowing floor までは揃っている。
2. **しかし、二大問題を完全に解決したとはまだ言えない。**
   source-backed に固まっているのは、主に
   - current first line
   - retained alternatives
   - stop line
   - mixed gate
   - true user-spec gate
   の切り分けである。
3. **言語側の実装まで終わっているともまだ言えない。**
   final parser grammar、final public parser / checker / runtime API、final public verifier contract、concrete theorem/model-check production binding はまだない。
4. **現在地は「抽象研究しかない段階」ではない。**
   むしろ、
   - compare floor は close 済み
   - actual adoption package は close 済み
   - helper-local actualization / narrowing floor もかなり close 済み
   - remaining work は reserve strengthening、model-check second line、mixed gate、true user-spec gate に narrowed
   という段階である。
5. **「これらに答えればもう最後まで完全自走できるか」には、厳密にはまだ yes と言えない。**
   true user-spec gate に答えればかなり先まで自走しやすくなるが、remaining mixed gate の一部は好みではなく implementation evidence に応じた actual adoption judgment だからである。

---

## 2. `faq_007.md` 以後に genuinely progressed したこと

`faq_007.md` の後で進んだのは、単なる言い換えではなく、次の actual progress である。

### 2.1 Problem 2 / syntax 側の narrowing

- `specs/examples/473`
  - order / handoff source-facing narrowing を 1 段進めた
- current source-facing recommendation は次へ狭まった
  - explicit edge-row / vertical continuation principal
  - `stage` / `after` / `witness` strong secondary candidate
  - authoritative-room `serial` sugar reserve
- `current_l2_order_handoff_stage_block_surface`
  - secondary candidate を helper-local 実行 floor で確認できるようにした

これは、Problem 2 の source wording mixed gate を完全に閉じたわけではない。
しかし、**low-level exact surface へ戻らずに、どの family を principal / secondary / reserve と読むかを 1 段具体化した**点で、genuine progress である。

### 2.2 Problem 1 側の theorem-first preflight

- `specs/examples/474`
  - theorem-prover experimental binding preflight を actualize した
- current theorem-first line は
  - notebook-first theorem line
  - brand-neutral preflight manifest
  - symbolic `evidence_refs`
  - `proof_notebook_review_unit` principal
  まで helper-local に上がった
- `current_l2_theorem_prover_binding_preflight`
  - typed / static / order-handoff representative corpus で preflight route を machine-check できるようにした

これは concrete theorem prover brand や final public theorem contract の確定ではない。
しかし、**theorem-first target を purely prose のままにせず、non-production preflight line まで押し上げた**点で、genuine progress である。

### 2.3 current queue の整理

`faq_007.md` の時点では広めに残っていた next line は、その後の closeout でさらに narrowed した。
現時点の current self-driven queue は、`progress.md` / `tasks.md` の読みでは次の 3 本である。

1. `auditable_authority_witness`
2. `delegated_rng_service`
3. model-check second-line concretization

したがって、**current queue が 0 ではない**ことも、引き続き明確である。

---

## 3. いま既に終わっていること

### 3.1 floor ごとの整理

現状を floor ごとに分けると、次の読みがもっとも誤読が少ない。

| floor | 現状 | 読み方 |
|---|---|---|
| compare floor | close | `specs/examples/458...465` と関連 test 群により、current first line / stop line / mixed gate を compare 可能な形で固定済み |
| actual adoption floor | close | `specs/examples/466...469` により、二大問題と syntax/modality の current recommendation を docs-first actual package に上げ済み |
| helper-local actualization / narrowing floor | close on current tranche | `specs/examples/470...474` と関連 test 群により、theorem-first pilot、room default vertical slice、minimal companion、stage-block secondary、theorem-preflight を helper-local actualization 済み |
| reserve strengthening floor | active | `auditable_authority_witness`、`delegated_rng_service`、model-check second line が current queue |
| final public language / tool / app floor | not close | parser / public API / final verifier contract / concrete external tool / packaging / host / broader app は未 close |

### 3.2 current runnable / machine-check evidence

少なくとも次は current state で runnable / machine-check の evidence を持っている。

- current authored sixteen
  - `e1 / e2 / e3 / e4 / e5 / e12 / e14 / e15 / e16 / e13 / e19 / e21 / e22 / e18 / e20 / e23`
- corrected prototype octet
  - `p01 / p02 / p03 / p04 / p05 / p06 / p07 / p08`
- compare floor
  - verifier preview alignment pre-floor
  - model-check projection pre-floor
  - theorem discharge pre-floor
- helper-local actualization / narrowing floor
  - theorem-first experimental pilot actualization
  - theorem-prover experimental binding preflight
  - authoritative-room vertical-slice actualization
  - minimal companion / experimental order-handoff surface
  - stage-block secondary order-handoff surface

この意味で、repo は docs-only skeleton ではない。
**corrected runnable version と helper-local compare/actualization floor がすでにある**。

### 3.3 2026-04-18 再実行で確認できたこと

今回 FAQ 08 を書くにあたって、representative validation を再実行した。
少なくとも次が fresh に確認できた。

| validation | fresh result |
|---|---:|
| `cargo test -p mir-runtime --test current_l2_source_sample_runner` | `22 passed` |
| `cargo test -p mir-runtime --test current_l2_operational_cli` | `12 passed` |
| `cargo test -p mir-runtime --test current_l2_verifier_preview_alignment` | `5 passed` |
| `cargo test -p mir-runtime --test current_l2_model_check_projection_prefloor` | `5 passed` |
| `cargo test -p mir-runtime --test current_l2_theorem_discharge_prefloor` | `5 passed` |
| `cargo test -p mir-runtime --test current_l2_theorem_first_pilot_actualization` | `5 passed` |
| `cargo test -p mir-runtime --test current_l2_theorem_prover_binding_preflight` | `4 passed` |
| `cargo test -p mir-runtime --test current_l2_source_sample_emitted_artifact_wiring` | `9 passed` |
| `cargo test -p mir-runtime --test current_l2_authoritative_room_vertical_slice_actualization` | `3 passed` |
| `cargo test -p mir-runtime --test current_l2_order_handoff_minimal_companion_surface` | `3 passed` |
| `cargo test -p mir-runtime --test current_l2_order_handoff_stage_block_surface` | `3 passed` |
| `python3 scripts/current_l2_source_sample_regression.py inventory` | authored sixteen present |
| `python3 scripts/current_l2_source_sample_regression.py regression` | all regression commands passed |

したがって、**「実装＆実行による比較検証がまだ薄い」という読みはもう正しくない。**
ただし、この事実はそのまま final public language completion を意味しない。

---

## 4. まだ終わっていないこと

### 4.1 二大問題はまだ final adoption ではない

現時点で終わっているのは、二大問題それぞれの **current first line の actual package 化** である。
まだ終わっていないのは **final adoption** である。

### 4.2 Problem 1 でまだ終わっていないもの

Problem 1 の current first line は source-backed であり、`p06`、preview alignment、theorem discharge pre-floor、model-check projection pre-floor、theorem-first pilot actualization、theorem-preflight まである。

それでも未完了なのは、次である。

- stronger typed surface actual adoption
- theorem discharge transport / public-contract actual format
- concrete theorem prover brand / binding
- settled property language
- concrete model-check tool seam
- final public verifier contract
- proof object public schema

つまり、**Problem 1 の current recommendation はあるが、final public theorem/model-check language/tool contract はまだない。**

### 4.3 Problem 2 でまだ終わっていないもの

Problem 2 の current first line は source-backed であり、cut family decomposition、relation decomposition principal、`authority_serial_transition_family` first、thread/node parity wording、authoritative room first actual default、low-level retained-later reference family、`p07/p08` evidence、vertical-slice actualization、minimal companion / stage-block floor まである。

それでも未完了なのは、次である。

- final source-surface handoff wording
- final emitted-artifact schema / handoff contract
- exhaustive shared-space final catalog
- stronger fairness / replay operational theorem or profile
- low-level family public stance の最終 cut

つまり、**Problem 2 の decomposition と default profile はあるが、final public wording と final public handoff contract はまだない。**

### 4.4 言語側の実装はまだ final public implementation ではない

「言語側の実装まで終わっている」と言うためには、少なくとも次が必要である。

| needed for that claim | current state |
|---|---|
| final parser grammar | still open |
| final public parser API | still open |
| final public checker API | still open |
| final public runtime API | still open |
| final public verifier contract | still open |
| final source-surface order/handoff wording | still open |
| stronger typed surface final disposition | still mixed gate |
| concrete theorem/model-check production bindings | still mixed gate |
| packaging / installed binary / host integration completion | still user-spec or later |

したがって、**今は final public language implementation complete とは書けない。**
それを書くのは material に misleading である。

---

## 5. 全体像に対して今どこにいるか

repo 全体を 1 本の progress bar で読むのは不正確である。
少なくとも 4 段の ladder で読む方が正しい。

### 5.1 overall completion ladder

1. candidate comparison / drift audit
   - close
2. current first line の actual adoption package 化
   - close
3. helper-local actualization / narrowing
   - current tranche は close
4. reserve strengthening / second-line concretization
   - current active
5. final public seams の mixed-gate actual adoption
   - 未 close
6. final public language / toolchain integration
   - 未 close
7. broader host / packaging / application realization
   - user-spec dependent で未 close

### 5.2 macro phase での現在地

`progress.md` の current reading を誤読なく言い換えると、次である。

- execution lane:
  `Macro 4 active on fixed authored/prototype floor`
- theory-lab lane:
  `Macro 5 near-end actual-adoption closeout`
- reserve integration lane:
  `Macro 6 minimal working subset actual default / Macro 7 mixed`

重要なのは、ここでの `Macro 5 near-end` は
**current docs-first / non-production theory line の near-end**
であって、
**repo 全体の public language/toolchain/app completion の near-end**
ではない、という点である。

### 5.3 いま最も正確な一文

一文で言うなら、現状は次である。

**「二大問題の current first line は source-backed に選ばれ、actual adoption package と helper-local actualization floor まで machine-check 済みだが、final public language / tool / app completion はまだ mixed gate と user-spec gate に残っている。」**

---

## 6. 「二大問題を完全に解決して、言語側の実装まで終える」ために残っているもの

残りは大きく 3 層に分かれる。

### 6.1 層A: self-driven reserve strengthening / second line

これは user 追加仕様なしでも、現在の default line から比較的進めやすい層である。

1. `auditable_authority_witness`
2. `delegated_rng_service`
3. model-check second-line concretization

これはまだ active queue である。
したがって、**theory line が空になったわけではない。**

### 6.2 層B: mixed gate

これは user の好みだけでは閉じきれず、実装 evidence を見ながら actual adoption を切る層である。

| topic | current line | why still mixed |
|---|---|---|
| stronger typed surface | checker-adjacent principal + structural markers first | source principal へ本当に上げるかは implementation pressure 次第 |
| theorem discharge transport / public-contract format | notebook-first + preflight + symbolic refs | public seam をどこで切るかは pilot evidence が必要 |
| concrete theorem prover brand | theorem-first, but brand-neutral preflight | actual brand は adapter cost / artifact shape / proof consumer pressure に依存 |
| settled property language / tool seam | row-local model-check carrier first | second-line concretization が必要 |
| final source-surface handoff wording | edge-row principal、stage-block secondary、serial sugar reserve | source wording は helper-local narrowing の先に actual adoption judgment が要る |
| final emitted-artifact schema | symbolic-ref family-first ratchet | public schema に上げる cut がまだない |
| final modal foundation / final source marker | partial basis + stronger family keep | order/handoff/syntax pressure を見て決める必要がある |
| final public verifier contract / final parser-checker-runtime API | not adopted in this line | public commitment の scope がまだ広すぎる |

### 6.3 層C: true user-spec gate

これは repo 内だけでは final に決められない層である。

| item | why it is still true user-spec |
|---|---|
| exhaustive final shared-space catalog beyond minimal subset | room/authority/replay/admission scope が target 依存だから |
| installed-binary / packaging / FFI / engine adapter / host integration target | repo-local CLI success を超えた acceptance criteria が target 依存だから |
| upper-layer application target beyond authoritative-room scenario | first scenario を超える acceptance definition 自体が target 依存だから |

注意すべき点は、
**theorem-first external target、repo-local near-end success criteria、authoritative-room first scenario、minimal working subset catalog、fairness/replay first default profile**
のような大きな defaults は、すでに current default として採用済みだということである。

つまり、残っている true user-spec gate は、以前よりかなり narrowed している。

---

## 7. いま必要な判断や情報は何か

### 7.1 「まだ user から欲しいもの」は以前より減っている

すでに current default へ上げたものが多いので、今ほしい情報は以前より狭い。
優先度順に言うと、次である。

1. **packaging / host integration を first completion に含めるか**
   - repo-local CLI + tests + emitted artifacts + reproducible compare floor でよいのか
   - それとも installed binary / FFI / engine adapter / host embedding まで first completion に含めるのか
2. **authoritative-room first scenario の次に何を first-class target にしたいか**
   - append-friendly contrast room の強化か
   - theorem consumer / model-check workflow か
   - それとも broader app/host か
3. **exhaustive shared-space catalog をどこまで求めるか**
   - minimal working subset のままで first completion とみなすか
   - それとも membership/admission/replay/policy family をもっと早く finalization したいか

これは true user-spec gate である。

### 7.2 「user が答えても mixed gate は残る」もの

次は user preference をもらっても、それだけでは閉じきれない。

1. stronger typed surface を principal に上げるか
2. theorem discharge transport / public-contract format をどう actualize するか
3. concrete theorem prover brand をどこで切るか
4. model-check property language / tool seam をどこで固定するか
5. final source-surface handoff wording をどう actualize するか
6. final modal foundation / final source marker をどこで切るか

これらは、**答えをもらえば再探索回数は減る**。
しかし、最後は experimental package と validation evidence を通して actual adoption judgment を切る必要がある。

### 7.3 もし「これだけ答えれば最大限進めやすい」というセットを挙げるなら

次の 5 点が最も効く。

1. first completion に packaging / host integration を含めるか
2. first completion の application acceptance を authoritative-room scenario で止めてよいか
3. theorem-first line で concrete prover brand selection を早めに許すか
4. model-check second line を same milestone に含めるか、theorem-first close 後に送るか
5. final public parser/checker/runtime API をこの tranche の必須条件にするか

このうち 1, 2, 5 は user-spec 寄り、3, 4 は mixed gate を進めやすくする preference である。

---

## 8. これらに答えれば、もう最後まで自走できるのか

### 8.1 短い答え

**厳密には、まだ no である。**

### 8.2 その理由

理由は単純で、残りの一部が「好みや target を聞けば終わる種類の問い」ではないからである。

#### 理由1: remaining mixed gate は evidence-dependent

たとえば、次は preference だけでは決まらない。

- theorem discharge transport / public-contract format
- theorem prover concrete brand
- model-check property language / tool seam
- final source-surface handoff wording
- final modal foundation / source marker

これらは、pilot / second-line / surface helper の実装と検証の結果を見ながら決める必要がある。
したがって、**一度 preferences をまとめてもらえば最後まで no-question とはまだ言えない。**

#### 理由2: final public completion の definition 自体がまだ広い

仮に current default を最大限固定しても、次は別の tranche である。

- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- packaging / installed binary / FFI / host integration
- broader application realization

これは current docs-first / helper-local line の外縁に近い。
したがって、**near-end theory closeout から final public completion までは、まだ 1 段ではない。**

### 8.3 それでも、かなり長く自走できるか

ここは **yes** と言ってよい。

もし上の user-spec / preference が与えられれば、repo は少なくとも次をかなり迷いなく進めやすくなる。

1. `auditable_authority_witness`
2. `delegated_rng_service`
3. model-check second-line concretization
4. theorem-first concrete binding actualization package
5. final source-surface wording の narrowed adoption package
6. public seam の ratchet package

つまり、**「最後まで完全保証」はまだ無理だが、かなり長い連続 run を user 追加入力なしで続けられる状態には近い」**。

### 8.4 どこまで行けたら「ほぼ最後まで自走」に近づくか

次が source-backed + machine-check になれば、その読みはかなり強くなる。

- concrete theorem binding が 1 つ actualized される
- model-check second line が first settled seam まで閉じる
- final source-surface handoff wording が principal 1 本に narrowed される
- parser/public API の minimum acceptance scope が決まる
- first completion が repo-local か host-facing か明示される

逆に言えば、**今はその一歩手前**である。

---

## 9. 一番誤読の少ない current explanation

一番誤読の少ない要約は、次である。

1. repo はもう docs-only abstract research ではない。
2. 二大問題の current first line は source-backed に選ばれ、actual adoption package に上がっている。
3. corrected prototype、compare floor、helper-local actualization / narrowing floor も machine-check されている。
4. しかし、二大問題の final adoption と final public language implementation はまだ終わっていない。
5. 現在地は、
   **near-end theory closeout の後半、reserve strengthening / second-line / mixed gate の入口**
   である。
6. true user-spec residual は以前よりかなり narrowed している。
7. それでも、remaining mixed gate は evidence-dependent なので、「一度答えたら最後まで完全 no-question」とまではまだ言えない。

---

## 10. 最後に、いま user から欲しいものを短くまとめると

最優先で欲しいのは、次である。

1. first completion に packaging / host integration を含めるか
2. authoritative-room first scenario を first completion の acceptance と見なしてよいか
3. broader application target をすぐ要求するか、後段に送ってよいか
4. theorem-first concrete binding を早めに actualize してよいか
5. model-check second line を theorem-first close と同 tranche に置くか
6. final public parser/checker/runtime API をこの closeout の必須条件にするか

このうち 1, 2, 3, 6 は user-spec 側、
4, 5 は mixed gate を進めやすくする preference 側である。

これらがあれば、repo はかなり先まで自走できる。
しかし current best reading としては、

**「near-end theory closeout 後の long run はかなり可能だが、final public language / tool / app completion までの完全無停止保証はまだできない」**

が正確である。


---

## Appendix C. FAQ 11 full text

# FAQ 11

## この文書について

この文書は、2026-04-20 時点の current reading に基づいて、

- 現状どこまで何が終わっているか
- もう二大問題を完全に解決して、言語側の実装まで終わっていると読めるか
- その段階へ持っていく全体像に対して今どこにいるか
- そこへ持っていくために、まだ何の判断や情報が要るか
- これらに答えれば repo がどこまで自走できるか

を、`faq_010.md` の次段として整理する FAQ である。

規範判断の正本は `specs/`、長期の repository memory は `plan/`、薄い snapshot は `progress.md` と `tasks.md` にある。
この FAQ 自体は **current explanation** であり、新しい規範判断を作る文書ではない。

今回の FAQ は、特に次を踏まえている。

- `specs/examples/609`
  - theorem-first external pilot summary index actualization
- `specs/examples/610`
  - auditable-authority-witness reserve package summary index actualization
- `specs/examples/611`
  - delegated-rng-service reserve package summary index actualization
- `progress.md`
  - 2026-04-20 21:32 JST 時点の lane-based snapshot
- `tasks.md`
  - 2026-04-20 21:32 JST 時点の current self-driven queue
- `plan/01`
  - lane / macro phase / capability snapshot
- `plan/11`
  - near-term package reading
- `plan/17`
  - autonomy gate の current reading

---

## 1. 先に短く答えるとどうか

短く言うと、次の読みが現在もっとも正確である。

1. **かなり進んでいる。**
   current repo は、docs-only skeleton ではない。
   parser-free current L2、compile-ready minimal actualization、authored sixteen、corrected prototype set `p01...p16`、theorem / model-check / order-handoff / shared-space の helper-local bridge、representative Lean sample set actual Lean execution、phase6 parser-side narrow carrier actualization を already 持つ。
2. **しかし、二大問題を completely solved と呼ぶのはまだ正確ではない。**
   close しているのは主として、
   - current first line
   - retained alternatives
   - stop line
   - helper-local actualization floor
   - repo-local runnable floor
   - mixed gate と true user-spec gate の切り分け
   である。
3. **言語側の実装まで終わっているともまだ言えない。**
   いまあるのは、
   - parser-free validation substrate
   - compile-ready minimal actualization
   - runner / CLI / regression
   - narrow non-production parser carrier
   - theorem/model-check/order-handoff の helper-local preview / artifact / bridge
   - representative Lean foundation + generated stub corpus
   までであり、
   - final parser grammar
   - final public parser / checker / runtime API
   - final public verifier contract
   - concrete theorem/model-check production binding
   - final source-surface handoff wording
   - final public witness/provider/artifact contract
   はまだない。
4. **全体像の中での現在地は、「foundation をまだ決め切れていない初期」ではない。**
   むしろ、
   - compare floor は close
   - actual adoption floor は close
   - helper-local actualization floor は大半 close
   - representative runnable / Lean execution floor は reached
   - current active self-driven line は `model-check-second-line` に narrow 化済み
   という段階である。
5. **「これらに答えればもう最後まで自走できるか」への答えは、repo-local near-end target ならほぼ yes、full final-public completion なら still conditional である。**
   今ある default と current reading だけで、repo-local runnable CLI / tests / emitted artifacts / reproducible compare floor まではかなり強く自走できる。
   ただし、final public language / final verifier / packaging / broader app completion までを no-question で保証するのは、まだ正確ではない。

---

## 2. `faq_010.md` 以後に genuinely progressed したこと

`faq_010.md` の後で進んだのは、単なる言い換えではなく、次の actual progress である。

### 2.1 reserve integration lane がさらに単独 package 化された

`faq_010.md` 時点では、reserve integration lane は既にだいぶ narrow になっていた。
その後 current repo は、さらに次を close した。

- Package 137
  - `auditable_authority_witness` reserve package summary index actualization
- Package 138
  - `delegated_rng_service` reserve package summary index actualization

これにより、

- `emit-reserve auditable-authority-witness`
- `emit-reserve delegated-rng-service`

で、それぞれの reserve package を **単独の repo-local summary index として再 materialize できる** 状態になった。

これは単なる FAQ wording refresh ではない。
reserve package 群が、より大きな bundle の中の節ではなく、**個別 package として再実行・再確認できる current floor** に上がったことを意味する。

### 2.2 current live queue がさらに narrowed した

Package 137 / 138 close 後の current reading では、

- reserve integration reopen
- later mixed gate reopen
- true user-spec hold line

の 3 束は維持しつつも、**active self-driven package として本当に残っているのは `model-check-second-line` だけ** という読みがかなり強くなった。

これは「何も残っていない」という意味ではない。
むしろ、

- first completion line
- reserve strengthening lane
- hold line

の切り分けが、以前より明確になったという意味である。

### 2.3 問題 2 の reserve route が first line と混ざりにくくなった

Problem 2 については、

- `p07 / p08`
  - first actual adoption line
- `p09`
  - delegated RNG reserve
- `p13 / p14`
  - negative static-stop pair

という current reading が前からあった。
今回さらに、

- `auditable_authority_witness`
- `delegated_rng_service`

が、それぞれ単独 summary index を持つようになったことで、
**first line と reserve strengthening line の境界が docs / helper / emitted artifact の三方で読みやすくなった**。

---

## 3. いま既に終わっていること

### 3.1 floor ごとの整理

現状を floor ごとに分けると、次の読みがもっとも誤読が少ない。

| floor | 現状 | 読み方 |
|---|---|---|
| compare floor | close | current first line / retained alternatives / stop line の比較自体は大きく終わっている |
| actual adoption floor | close | Problem 1 / Problem 2 / syntax-modality の current recommendation は actual package として docs に載っている |
| helper-local actualization floor | largely close | theorem/model-check/order-handoff/shared-space / parser-side tranche の narrow actualization は大半済んでいる |
| representative runnable floor | reached | authored sixteen、`p01...p16`、guided sample、smoke、CLI、regression が current mapped corpus 上で動く |
| representative Lean floor | reached | `samples/lean/foundations/` の small proof fragment と `samples/lean/current-l2/` の generated stub corpus が current role 分担で揃っている |
| final public language / tool / app floor | not close | final grammar、public API、verifier contract、production tool binding、packaging、exhaustive catalog は未 close |

### 3.2 current runnable / machine-check evidence

少なくとも次は current state で runnable / machine-check の evidence を持っている。

- `samples/current-l2/`
  - authored sixteen
- `samples/prototype/`
  - corrected prototype set `p01...p16`
- `scripts/current_l2_guided_samples.py`
  - `list`
  - `show problem1`
  - `run problem1`
  - `matrix problem1`
  - `matrix problem2`
  - `emit-theorem problem1`
  - `emit-scenario problem2`
  - `emit-reserve auditable-authority-witness`
  - `emit-reserve delegated-rng-service`
  - `smoke problem1`
  - `smoke problem2`
  - `smoke-all`
  - `closeout`
- `samples/lean/foundations/`
  - `CurrentL2LabelModel.lean`
  - `CurrentL2IfcSecretExamples.lean`
  - `CurrentL2FiniteIndexFirstLayer.lean`
  - `CurrentL2ProofSkeleton.lean`
- `samples/lean/current-l2/`
  - representative generated theorem stub corpus

この意味で、repo は docs-only skeleton ではない。
**current mapped corpus の corrected runnable floor は already reached** している。

### 3.3 実用的 sample family ごとの current state

user が気にしている「実用的なサンプル」ごとに言い換えると、現在の状態は次である。

| sample family | current state | まだないもの |
|---|---|---|
| 型検査 / IFC / taint | `p10 / p11 / p12` と Lean first fragment まで actualize 済み | final typed source principal、final public checker/verifier contract |
| capture / lifetime / simple cost | `p15 / p16` と finite-index first layer まで actualize 済み | full dependent layer、general theorem proving in compiler |
| theorem proving bridge | theorem-first emitted artifact loop、Lean foundation、generated stub corpus まで actualize 済み | final proof object public contract、production theorem prover binding |
| model-check bridge | row-local carrier と helper-local second-line floor はある | first settled property language、public checker artifact、production tool binding |
| async / order / handoff | `p07 / p08 / p09 / p13 / p14` と authoritative-room first scenario まで actualize 済み | final source wording、final emitted schema、final public witness/provider/artifact contract |
| communication / shared-space / auth | authoritative shared-space turn-based room first scenario は actualize 済み | exhaustive final catalog、broad host/app integration |
| full dependent type sample | intentionally not first-cut target | first public core に full dependent type を要求しない reading を維持している |

---

## 4. まだ終わっていないこと

現在の repo が **まだ終わっていない** のは、主として次である。

### 4.1 final public language / verifier / tool seam

- final parser grammar
- final public parser / checker / runtime API
- final public verifier contract
- final proof object public schema
- final public theorem result object
- consumer-shaped theorem payload public contract
- final public checker artifact
- actual public checker migration
- final emitted verifier handoff artifact

### 4.2 final wording / schema / catalog

- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- combined public contract
- exhaustive final shared-space catalog

### 4.3 concrete external binding / packaging

- concrete theorem prover production binding
- concrete model-check production binding
- installed binary / packaging success criteria
- FFI / engine adapter target
- broader host integration target

したがって、
**「二大問題を completely solved」でもなければ、「言語側の実装まで complete」でもない**。
ただし、これは「まだ何も決まっていない」という意味ではない。
むしろ、final public seam だけを意図的に残している current phase だと読むのが正確である。

---

## 5. Problem 1 の current status

Problem 1 は、型システム / 定理証明 / モデル検査の line である。

### 5.1 何が current first line として close しているか

current first line は、次である。

- checker-adjacent semantic carrier principal
- finite decidable index fragment principal
- conceptual spine:
  `Ψ ; Γ ; Δ ⊢ e : A @ m ! ε ▷ C`
- first strong typing layer の principal target:
  - security label / IFC
  - taint
  - capture set
  - lifetime preorder
  - simple cost bound
- theorem-first external integration target
- notebook-first theorem line
- row-local model-check carrier first
- stronger typed surface は early に source principal へ昇格しない

この line 自体は、現段階で source-backed と読んでよい。

### 5.2 何が実装・実証として終わっているか

Problem 1 側で current repo が already 持っているものは、次である。

- `p06`
  - bridge-floor evidence
- `p10 / p11 / p12`
  - IFC / declassification / label-flow line
- `p15 / p16`
  - capture / lifetime / simple cost line
- helper-local checker summary
- verifier preview alignment
- theorem-first emitted artifact loop
- model-check row-local projection pre-floor
- `samples/lean/foundations/`
  - label / IFC / finite-index / proof-skeleton first fragment
- `samples/lean/current-l2/`
  - representative generated theorem stub corpus

つまり、Problem 1 は
**理論だけが先にあり実装が全く無い段階ではない**。
first strong typing layer と theorem-first pilot の repo-local floor までは、かなり前に進んでいる。

### 5.3 何がまだ残っているか

Problem 1 で still later に残るのは、主として次である。

- stronger typed source principal の final adoption
- final public theorem result object
- consumer-shaped theorem payload public contract
- first settled property language
- concrete theorem prover brand
- concrete model-check tool brand
- final public checker artifact
- final public verifier contract

### 5.4 したがって、Problem 1 はどこまで終わったと言えるか

もっとも正確な言い方は次である。

- **Problem 1 の current first line は close 済み**
- **Problem 1 の first executable / helper-local / Lean-first floor も reached**
- **しかし final public theorem/model-check/verifier seam は未 close**

従って、Problem 1 を
「完全解決」
と呼ぶのはまだ早いが、
「repo-local near-end の first completion line はかなり close している」
とは言ってよい。

---

## 6. Problem 2 の current status

Problem 2 は、order / handoff / `memory_order` 再構成 line である。

### 6.1 何が current first line として close しているか

current first line は、次である。

- cut family decomposition
  - `atomic_cut`
  - `barrier`
  - `durable_cut`
  - `snapshot_cut` / `consistent_cut` は comparison candidate
- relation decomposition principal
  - program / dependency / publication / observation / witness / finalization / scoped happens-before
- `authority_serial_transition_family` first
- `witness_aware_commit_family` second
- thread / node parity wording:
  - same causal language
  - differences remain in lowering / evidence / transport / failure / durability / policy
- low-level `std::memory_order` / `std::kill_dependency` family は retained-later backend/reference family
- authoritative-room first default profile:
  - `authority-ack`
  - `single room authority`
  - `authoritative serial transition`
  - `authority_rng`
  - late join visible past
  - stale reconnect fail-then-refresh
  - replay invalidation

ここでも、`atomic_cut` を global consistent cut や durable commit と同一視しない current line は preserve されている。

### 6.2 何が実装・実証として終わっているか

Problem 2 側で current repo が already 持っているものは、次である。

- `p07 / p08`
  - authoritative-room first line の representative pair
- `p09`
  - delegated RNG reserve route
- `p13 / p14`
  - late-join visibility negative static-stop pair
- `surface_preview`
  - explicit edge-row principal
  - stage-block secondary
  - `serial on ...` reserve
- `emit-scenario problem2`
  - repo-local authoritative-room scenario bundle
- `emit-reserve auditable-authority-witness`
  - reserve strengthening summary index
- `emit-reserve delegated-rng-service`
  - reserve practical package summary index

つまり、Problem 2 も
**理論比較だけで止まっているわけではない**。
first line と reserve line を分けた runnable / emitted / summary floor が already ある。

### 6.3 何がまだ残っているか

Problem 2 で still later に残るのは、主として次である。

- final source-surface handoff wording
- final emitted-artifact schema
- final public witness schema
- final public provider receipt schema
- combined public contract
- stronger fairness / replay profile
- exhaustive shared-space catalog
- low-level exact source surface adoption

### 6.4 したがって、Problem 2 はどこまで終わったと言えるか

もっとも正確な言い方は次である。

- **Problem 2 の current first line は close 済み**
- **Problem 2 の authoritative-room first scenario floor も reached**
- **reserve strengthening lane も単独 package として切り分け済み**
- **しかし final wording / final public artifact schema / exhaustive catalog は未 close**

従って、Problem 2 も
「完全解決」
ではないが、
「repo-local near-end の first completion line はかなり close している」
とは言ってよい。

---

## 7. syntax / modality の current status

syntax / modality line の current recommendation は、現在かなり安定している。

- semantic honesty first
- explicit edge-row principal
- stage-block secondary
- `serial on ...` reserve
- `lambda_circle_box` は partial basis candidate
- guarded / MDTT / MTT / Fitch-style multimodal は stronger candidate family

ここで重要なのは、
**syntax / modality は implementation をもう止める種類の未解決ではない**
という点である。

まだ残っているのは、

- final modal foundation adoption
- final source marker
- final grammar / public surface freeze

であって、
current helper-local companion / experimental surface と actual runnable floor を進めるうえでの principal blocker ではない。

---

## 8. 実装はどこまで終わっているか

「言語実装」という言葉は広すぎるので、層ごとに分けて読む必要がある。

| 層 | current state | 読み方 |
|---|---|---|
| parser-free semantics / runtime / runner | reached | current L2 / authored corpus / prototype corpus は runnable |
| compile-ready minimal actualization | reached | Rust / Python helper / CLI / regression は current floor で動く |
| parser-side narrow carrier | partial but real | phase6 narrow actualization はあるが final grammar / full lowering ではない |
| theorem bridge | reached at helper-local first line | emitted artifact loop + Lean foundation / stub corpus まではある |
| model-check bridge | reached at helper-local first line | row-local carrier / second-line reopen まではある |
| order-handoff / shared-space scenario | reached at helper-local first line | authoritative-room first scenario と reserve package summary index がある |
| final public language implementation | not reached | final parser grammar / public checker/runtime API / final public verifier contract はない |

したがって、
**「もう実際に言語実装して全部終わった」ではない**
が、
**「もう runnable prototype / helper-local actualization / narrow parser-side tranche まで実装された」ではある**
というのが current reading である。

---

## 9. 全体像の中で今どこにいるか

macro phase と lane で読むと、現在地は次である。

- execution lane:
  `Macro 4 active on fixed authored/prototype floor`
- theory-lab lane:
  `Macro 5 repo-local once-through near-end packages active after FAQ10 handoff integration`
- reserve integration lane:
  `Macro 6 minimal working subset actual default / Macro 7 mixed`

rough に一言で言い換えると、

- **repo-local near-end target に対しては 95% 前後**
- **final public language / verifier / packaging / broad app completion に対しては、そこまで高い progress 読みはしない**

が、もっとも誤読が少ない。

ここでの 95% は、

- fixed-subset runnable floor
- actual adoption package
- helper-local actualization
- representative Lean / theorem / order-handoff floor
- repo-local closeout defaults

までを scope にした rough estimate である。
repo 全体の final public completion を意味しない。

---

## 10. そこに持っていくためにまだ必要な判断や情報

ここは、**self-driven で進められる残件** と **mixed gate** と **true user-spec gate** を分けて読むのが正確である。

### 10.1 いま self-driven で進める残件

current active self-driven package は、実質的には次だけである。

- `model-check-second-line`
  - row-local property carrier second-line を public checker finalization と切り離したまま narrow actualization する

この package は、current defaults の範囲で進められる。

### 10.2 まだ mixed gate として残るもの

current defaults を使えば前進はできるが、final public adoption まではまだ上げないものは次である。

- stronger typed surface promotion
- final public theorem result object
- consumer-shaped theorem payload public contract
- concrete theorem prover brand
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- final source-surface handoff wording
- final emitted-artifact schema
- final public witness/provider/artifact contract
- final modal foundation adoption
- final source marker
- final parser grammar / final public parser-checker-runtime API

### 10.3 true user-spec gate として残るもの

current near-end target の外に置いてよいものは次である。

- exhaustive final shared-space catalog
- installed binary / packaging
- FFI / engine adapter / host integration target
- upper-layer application target beyond first authoritative-room scenario

### 10.4 いま不足しているのは何か

正確に言うと、**global architecture information が足りない** わけではない。
今不足しているのは主として、

- final public seam を本当に freeze するかどうか
- どの concrete tool / schema / API を public に見せるか
- near-end target の先で、どこまで productionized するか

という later decision である。

つまり、
**「まだ foundation が足りない」ではなく、「final public seam を凍らせるための later decision をまだ intentionally 残している」**
と読むのが正確である。

---

## 11. これらに答えればもう最後まで自走できるのか

ここは、「最後」の意味で答えが変わる。

### 11.1 repo-local near-end target なら

かなり強く **yes** と言ってよい。

理由は次である。

- current first line は source-backed
- actual adoption package は close
- corrected runnable floor は reached
- representative Lean floor は reached
- user defaults により current default decision はかなり埋まった
- active self-driven queue は `model-check-second-line` まで narrow 化済み

従って、
**repo-local runnable CLI / tests / emitted artifacts / representative samples / helper-local theorem-order-handoff bridges をさらに押し切る**
という意味なら、かなり自走しやすい。

### 11.2 full final-public completion なら

まだ **no** に近い。

理由は次である。

- final public grammar
- final public parser/checker/runtime API
- final public verifier contract
- final public theorem/model-check/witness-provider schema
- production tool binding
- packaging / host target

が、still mixed gate または true user-spec gate に残っているからである。

したがって、
**「これらに答えればもう repo-local near-end まではかなり自走できる」**
は current reading と整合する。
一方で、
**「これらに答えれば final public completion まで no-question で保証できる」**
とまでは、まだ言わない方が正確である。

---

## 12. いま自然な次順

この FAQ の後に自然なのは、次である。

1. `model-check-second-line`
   - reserve integration reopen の remaining active package を close する
2. theorem/model-check/order-handoff public seam の later mixed gate を、current defaults の範囲で further narrowing する
3. final public seam に踏み込まないまま、representative runnable samples と emitted artifacts の読みやすさをさらに上げる

つまり、次にやるべきことは
**foundation をまた掘り返すことではなく、already chosen current first line の execution-side / bridge-side closeout を詰めること**
である。

---

## 13. まとめ

誤解を避けるために、最後に current reading を 4 行で圧縮しておく。

1. **二大問題は、current first line と first completion line まではかなり close している。**
2. **しかし、二大問題を completely solved、language implementation complete と呼ぶのはまだ早い。**
3. **repo-local near-end success に対しては、かなり強く自走できる段階に入っている。**
4. **残っている大物は、foundation search というより final public seam / packaging / broader integration 側である。**
