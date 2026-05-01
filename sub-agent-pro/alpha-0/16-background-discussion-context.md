# 16 — Background discussion context

This file preserves the conceptual background from the user discussion. It should inform product scope and naming, but it is not itself normative unless mirrored into `specs/` or `plan/`.

## 1. Synchronization language background

The user originally explored a language for coordinated systems where the whole-system meaning can be specified without prematurely fixing every node-to-node interface.

Important idea:

- A system-level operation such as `b.hp -= a.attack_power` may be evaluated at different Places: server, attacker, victim, or elsewhere.
- The high-level meaning can be stable while the required interfaces differ drastically.
- Therefore, source-level meaning should not be prematurely collapsed into a fixed server/client split.

Mirrorea consequence:

- `Place` is the execution locus where state/queue/capability/visibility/observation are interpreted.
- The system should support projection from whole-system source to place-specific programs/routes.
- Network/interface details should be derived from placement, route, contract, and effect decisions, not hard-coded as the language's starting point.

## 2. Multi-layer DAG background

The user also explored platform/world/avatar/item layering.

Important idea:

- Platform -> world -> avatar -> item-like containment/availability suggests a layered dependency graph.
- Later additions should preserve acyclicity.
- Cross-layer effects that move against dependency direction need time/order/witness/authority treatment.

Mirrorea consequence:

- Safe evolution should default to downstream addition.
- Hot-plug must preserve DAG discipline.
- Hidden backward edges are suspicious.
- `atomic_cut`, rollback, witness, membership, and publication/observation order are central.

## 3. Algebraic Effects / OS-like background

The user has an OS/container-like interpretation: programs/containers should declare effects; an outer layer can inspect, route, remap, and visualize those effects.

Important idea:

- This is a useful local interpretation, not the main project definition.
- The main axis remains: correct theory + correct hot-plug + virtual-space system.

Mirrorea consequence:

- Typed-Effect Wiring Platform remains adjacent/lower layer.
- Standard I/O is not Mir core primitive.
- External world connection goes through typed effect adapter.
- Debug/telemetry/visualization are information-bearing effects with label/authority/redaction/retention.

## 4. No-code / design visibility background

The user distinguishes visual/no-code surfaces from true design transparency.

Important idea:

- Merely hiding code in GUI does not democratize design.
- The structure of state, operation, effect, and contract must be visible and inspectable.

Mirrorea consequence:

- Viewer/devtools is not a cosmetic feature; it is part of the platform's semantic transparency.
- Future graphical editors should operate over typed commands/effects/contracts, not hidden imperative state.
- Alpha should implement devtools/inspection before attempting no-code world editing.

## 5. VR interface background

The user values interfaces that reflect the intrinsic shape of the represented object.

Mirrorea consequence:

- Event DAG should be shown as event DAG.
- Place graph should be shown as Place graph.
- Witness timeline should be shown as timeline.
- Capability/authority preorder/lattice should be shown structurally.
- Hot-plug activation cuts should be visible as boundaries.

## 6. Reversed Library background

The user's Reversed Library / 裏返した図書館 vision is an upper-layer knowledge-space experience:

- books, concepts, fields, communities, media, experiments, and discussions become spatial/world entities
- fields are not closed shelves but connected places
- people moving between concepts changes perceived proximity
- knowledge relations become paths, portals, overlays, or shared objects

Mirrorea consequence:

- Reversed Library is not the immediate runtime target.
- Mirrorea Spaces and Mirrorea Atlas should make it possible later.
- Do not implement Reversed Library during alpha except possibly a tiny seed demo clearly marked non-final.

## 7. VRChat / browser-like world background

The user wants at least VRChat-class capabilities eventually, but with a client closer to a browser and virtual worlds closer to WWW.

Mirrorea consequence:

- Mirrorea Spaces is not merely VRChat clone.
- World packages should be addressable, inspectable, contract-bearing, hot-pluggable, and portable.
- The client should mediate capabilities/effects/adapters and provide devtools-like inspection.
- Avatar compatibility should be adapter/runtime-package based, not core-special.

## 8. PrismCascade background

The user provided PrismCascade theory separately. It is a media-processing kernel candidate for video editing/generation/live streaming with:

- plugin input/output types
- main output tree + side output DAG
- Effect and Macro plugins
- AST operations such as cut/assign/substitute/undo/redo
- look-ahead/look-behind scheduling
- finite group delay and strong non-causal effect classification
- timestamp/rank scheduling
- cache/lifetime management for media values

Mirrorea consequence:

- PrismCascade is separate.
- Do not implement it in Mirrorea alpha.
- Future connection may be through typed external effect adapter, render provider, shared trace id, or world object displaying Prism output.

## 9. User's current priority

The current priority is to let Codex proceed safely by freezing the decisions that would be hard to change later:

- lifetime/fallback and guarded references
- contract variance/layer compatibility
- save/load/consistent cut/Z-cycle boundaries
- runtime package/avatar adapter non-core model
- Mirrorea Spaces alpha scope and stage/phase tracking

## 10. How to use this file

Use this file to keep intent aligned. Do not copy it wholesale into normative specs. Extract decisions into `specs/` only when they are precise and intended to be normative.
