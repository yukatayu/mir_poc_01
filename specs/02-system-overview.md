# 02 — System Overview

## The whole stack in one page

The project currently consists of four conceptual systems organized into a layered stack:

1. **Mir** — the semantic language core.
2. **Mirrorea** — the distributed control / routing / audit fabric.
3. **PrismCascade** — a separate media-processing kernel for offline/live audiovisual pipelines.
4. **Typed-Effect Wiring Platform** — a routable, inspectable, contract-aware effect boundary for software and containers.

These are not independent in purpose, but they are intentionally **not the same software**.
They should share only the minimum necessary interfaces, identifiers, and contracts.

## Why the stack is large

The project is not merely "a new service".
It aims to change:

- the **language of execution and evolution** (Mir),
- the **fabric for distributed composition and safe insertion** (Mirrorea),
- the **kernel for one important performance-sensitive domain** (PrismCascade), and
- the **way operational boundaries are observed and rewired** (Typed-Effect Wiring Platform),
- and then use these to support higher-level spaces such as a virtual-reality social system and the Reversed Library.

## Core distinction from existing systems

Most current systems split these responsibilities between:

- programming language semantics,
- service mesh / deployment tooling,
- application code,
- observability stack,
- ad-hoc operational procedures.

This project aims to make those layers **theoretically aligned** rather than merely operationally compatible.

## What should remain separable

To preserve replaceability and conceptual clarity:

- Mir should not become a media-specific runtime.
- PrismCascade should not become a general distributed-language runtime.
- The Typed-Effect Wiring Platform should not become the language itself.
- Mirrorea should not absorb all application logic.

## High-level relationships

- Mir defines *what* computation means and how evolution is constrained.
- Mirrorea defines *where and through what route* that computation is executed and evolved.
- PrismCascade defines *how a specific high-performance media domain* is represented, planned, and executed.
- The Typed-Effect Wiring Platform defines *how effect boundaries can be made inspectable and rewritable*, especially for integration and operations.
