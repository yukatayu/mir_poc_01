# 08 — Cross-System Relations

## Summary

This project currently contains at least three major conceptual systems:

1. Mir / Mirrorea,
2. PrismCascade,
3. the Typed-Effects Wiring Platform.

They are related but should not be collapsed into one undifferentiated runtime.

## A useful division of concerns

### Mir / Mirrorea
- semantic core of distributed computation,
- contracts, ownership, lifetimes, cuts, overlays, safe evolution,
- logical routing and audit.

### PrismCascade
- time-series media graph kernel,
- Meta/Core/Runtime separation,
- scheduling and memory planning,
- live/offline distinction.

### Typed-Effects Wiring Platform
- typed effect boundary representation,
- rewritable routes,
- observability and state↔event graphing,
- legacy integration.

## Shared foundations
All three projects care about:
- explicit effects,
- visibility of dependency and causality,
- replaceability and compatibility,
- the need to separate normative semantics from implementation choice.

## Recommended relationship

- Keep them separate in implementation.
- Share only what must be shared:
  - identifiers,
  - contract syntax or schemas where useful,
  - trace linking strategy,
  - effect vocabulary where compatible.
