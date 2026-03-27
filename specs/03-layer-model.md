# 03 — Layer Model

## Layers

### L0 — Existing operating-system and network substrate
Examples: processes, file systems, sockets, kernel interfaces, device drivers, graphics APIs, service orchestrators.
This layer is assumed, not redesigned.

### L1 — Mir Core
Responsibilities:
- causality and directed-acyclic-graph execution model,
- effects and contracts,
- cuts / boundaries / rollback constraints,
- ownership / lifetimes / monotone degradation,
- safe evolution primitives.

### L2 — Mirrorea Fabric
Responsibilities:
- logical naming,
- routing and route rebinding,
- overlay insertion,
- patch application,
- distributed cut realization,
- audit and path proof,
- safe integration of non-Mir systems.

### L3 — Shared Space / Shared State
Responsibilities:
- session / space abstractions,
- shared object synchronization models,
- consistency mode selection,
- movement or linkage between spaces.

### L4 — Domain Engines / Frameworks
Responsibilities:
- domain-specific kernels such as PrismCascade,
- virtual-reality world engines,
- collaborative document engines,
- future GUI-programming substrate.

### L5 — Applications / Communities / Reversed Library
Responsibilities:
- actual user-facing spaces,
- concrete virtual social systems,
- web synchronization experiences,
- curriculum/progression systems,
- the Reversed Library as an application-level architecture.

## Cross-cutting concern: observability
Observability crosses all layers but should be conceptually anchored in:
- L1 for normative event semantics,
- L2 for routing/audit,
- L4 for domain-specific traces (for example media plans),
- L5 for user-visible diagnostics where appropriate.

## Cross-cutting concern: replaceability
Replaceability is strongest when boundaries are explicit:
- language semantics boundary at L1,
- routing/overlay boundary at L2,
- domain-kernel boundary at L4,
- user experience boundary at L5.
