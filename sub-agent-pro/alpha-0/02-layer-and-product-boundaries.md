# 02 — Layer and product boundaries

This file prevents scope collapse. Each layer may interact with others, but its guarantees and responsibilities must remain separate.

## 1. Conceptual stack

```text
Mir
  semantic core language and theory
  effect / contract / ownership / lifetime / fallback / rollback / cut / proof obligations

Mirrorea
  fabric/runtime
  Place / route / membership / capability / witness / hot-plug / audit / visualization / transport separation

Mirrorea Lab
  developer/research environment
  sample execution / checker diagnostics / event DAG viewer / envelope trace / hot-plug inspector

Mirrorea Spaces
  shared virtual-space platform
  room / world / participant / avatar role / object / observer / portal / runtime package ecosystem

Mirrorea Atlas
  future multi-world graph and portal layer
  world graph / relation graph / route graph / spatialized navigation

Reversed Library
  flagship upper-layer knowledge-space application
  concepts, communities, documents, media, experiments, and people as connected spatial entities

Typed-Effect Wiring Platform
  adjacent operational layer
  typed effect boundary / adapter / routing / inspection / rewiring / legacy integration

PrismCascade
  separate media-processing kernel candidate
  media AST / plugin graph / look-ahead/look-behind scheduling / rendering / live/offline media processing
```

## 2. What Mir guarantees

Mir should guarantee or specify:

- semantic event DAG discipline
- explicit effects and contracts
- primitive fallback and monotone degradation
- ownership / linear resource discipline
- lifetime / region / lease reasoning
- structured failure outcomes such as `Reject`, `Approximate`, and `Compensate`
- local rollback discipline
- `atomic_cut` as a place-local rollback frontier
- type/effect/checker obligations
- proof obligations for no hidden backward edge, no hidden rollback, no resource duplication, no lifetime resurrection

Mir should not guarantee:

- concrete network transport
- concrete storage or distributed checkpoint algorithm
- avatar format compatibility
- production renderer
- browser UI
- PrismCascade media scheduling
- final public parser grammar until explicitly frozen

## 3. What Mirrorea guarantees

Mirrorea should guarantee or specify:

- `Place` as execution locus, not participant identity
- logical naming and routing
- MessageEnvelope lane separation: transport, auth, membership, capability, authorization, witness, dispatch outcome, notes
- MembershipRegistry / PlaceCatalog
- overlay insertion without silent API shadowing
- HotPlugRequest / HotPlugVerdict / activation cut discipline
- layer insertion compatibility checks
- audit and path proof direction
- typed visualization / telemetry boundaries
- transport/auth/membership/capability/witness non-collapse
- helper-local/network canary evolution toward real transport

Mirrorea should not guarantee:

- final avatar standard
- final public viewer API before alpha freeze
- durable distributed migration before save/load theory is frozen
- PrismCascade integration
- Reversed Library application semantics

## 4. What Mirrorea Spaces guarantees

Mirrorea Spaces is a product/platform layer. In the long term it should support VRChat-class social virtual-space capabilities while preserving Mirrorea's stronger typing and inspection model.

Long-term functional lower bound:

- worlds / rooms / instances
- participants and avatar roles
- portals / world links
- object interaction
- synchronized state
- voice/text/gesture/presence, eventually
- user-created worlds/objects/tools/avatar runtimes
- safety/trust/moderation
- permissions and private/public/invite scopes
- debug/admin/devtools surfaces

Alpha target:

- small world/room runtime
- placeholder avatar / custom Mir avatar runtime skeleton
- object/module hot-plug
- networked Place execution in Docker
- event DAG / witness / route / membership visualization
- negative tests for stale membership, missing capability, missing witness, incompatible hot-plug

Do not implement full VRChat compatibility in alpha. Provide the abstraction that makes compatibility adapters possible.

## 5. What Reversed Library is

Reversed Library / 裏返した図書館 is not the current implementation target. It is the upper-layer knowledge-space experience that becomes possible when Mirrorea Spaces and Atlas are mature.

It should eventually allow:

- concepts, fields, documents, media, experiments, and communities to become spatial objects/worlds
- fields and communities to be connected by portals/paths rather than static categories
- people moving through knowledge spaces to change perceived proximity
- knowledge relations, citations, dependencies, conflicts, and extensions to be inspectable
- collaborative learning, research, simulation, discussion, and creation to share one spatial fabric

It must not be confused with the Mirrorea runtime itself.

## 6. What PrismCascade is

PrismCascade is a separate media-processing kernel candidate. It is about plugin ASTs, main/side outputs, media types, look-ahead/look-behind, non-causal scheduling, cache/lifetime for media frames, Effect/Macro plugins, UI-independent rendering, and live/offline video processing.

For Mirrorea alpha:

- Do not implement PrismCascade.
- Do not import PrismCascade scheduling into Mir core.
- Leave future typed external effect / render provider / trace linkage as a possible adapter route only.

## 7. Browser-like interpretation

Mirrorea Spaces should eventually feel closer to a browser for virtual worlds than to a closed single-platform client.

Approximate mapping:

| WWW | Mirrorea Spaces |
|---|---|
| Browser | Mirrorea Client |
| Web page / web app | World / Space package |
| URL | Logical world name / route |
| Link | Portal / world reference |
| DOM | Place/object graph |
| JavaScript / WebAssembly | typed world behavior / runtime package |
| Web API | typed external effect adapter |
| Permission prompt | capability / authority request |
| DevTools | event DAG / route / witness debugger |
| Browser extension | client-side layer / policy / visualization |

The analogy is useful, but Mirrorea starts from shared stateful virtual space rather than documents. Therefore presence, shared state, witness, event ordering, hot-plug, rollback/cut, and visibility frontiers are more central than in the ordinary Web.
