# 01 — Charter and Decision Levels

## Charter

This project aims to design a coherent stack in which:

1. A semantic core language (**Mir**) can describe causality, effects, contracts, ownership, lifetimes, and safe evolution.
2. A distributed fabric (**Mirrorea**) can execute and evolve systems under those semantics.
3. A media kernel (**PrismCascade**) can remain a high-performance, plan-first, UI-independent engine while still integrating with the larger system.
4. A typed-effect wiring platform can make containerized or otherwise opaque software more inspectable, contract-aware, and rewritable at the routing layer.
5. Higher-level systems (virtual reality social spaces, synchronized web views, collaborative editors, knowledge spaces, the Reversed Library) can be built on top without collapsing the lower layers into application-specific assumptions.

## Non-goals

The project does **not** currently aim to:

- replace existing physical or operating-system network stacks,
- define final product user interfaces,
- commit to a single consensus algorithm,
- commit to a single game engine,
- prematurely merge all subsystems into one runtime.

## Decision levels

### L0 — Foundational
A change here alters the meaning of the whole stack.
Examples:
- the four pillars of Mir,
- directed-acyclic-graph discipline,
- explicit contracts/effects,
- ownership/lifetime monotonicity,
- safe-evolution principles.

### L1 — Strong direction
A high-confidence architectural direction with room for syntactic or operational refinement.
Examples:
- keeping Mir, Mirrorea, PrismCascade, and the Typed-Effect Wiring Platform separate but interoperable,
- downstream-only patching as a default evolution rule,
- compatibility-preserving overlays.

### L2 — Design proposal
Useful working design that may still be revised.
Examples:
- exact syntax of overlays,
- exact shape of preference chains,
- exact implementation split of the effect-wiring layer.

### L3 — Exploratory
Known important but not settled.
Examples:
- final knowledge classification strategy for the Reversed Library,
- final graphical-programming substrate,
- full visual debugger model.

## Rule for unresolved issues
If a matter is unresolved, documents must say so explicitly.
Unresolved does not mean unimportant; it means the project has chosen not to pretend that a decision exists yet.
