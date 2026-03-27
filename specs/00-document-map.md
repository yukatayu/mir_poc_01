# 00 — Document Map

This file tells a reader or agent where each major concern lives.

## Normative order

1. `01-charter-and-decision-levels.md`
2. `02-system-overview.md`
3. `03-layer-model.md`
4. `09-invariants-and-constraints.md`
5. One or more subsystem documents:
   - `04-mir-core.md`
   - `05-mirrorea-fabric.md`
   - `06-prismcascade-positioning.md`
   - `07-typed-effects-wiring-platform.md`
   - `08-cross-system-relations.md`
6. `10-open-questions.md`
7. `11-roadmap-and-workstreams.md`
8. `12-decision-register.md`

## What each document does

- `01-charter-and-decision-levels.md`
  - Project intent, scope, and decision-level system.
- `02-system-overview.md`
  - High-level overview of the whole stack.
- `03-layer-model.md`
  - Precise layer responsibilities and boundaries.
- `04-mir-core.md`
  - Mir language core, four pillars, semantics-focused summary.
- `05-mirrorea-fabric.md`
  - Naming, routing, overlays, patching, audit, distributed control.
- `06-prismcascade-positioning.md`
  - Why PrismCascade is separate, and how it integrates.
- `07-typed-effects-wiring-platform.md`
  - Typed-effect routing / container-visibility platform and its role.
- `08-cross-system-relations.md`
  - Shared assumptions, boundaries, and interoperability points.
- `09-invariants-and-constraints.md`
  - The strongest rules that should not be silently broken.
- `10-open-questions.md`
  - Explicit unresolved issues.
- `11-roadmap-and-workstreams.md`
  - Proposed implementation and research sequence.

## Reports

- `docs/reports/` contains chronological work logs.
- A report is not normative, but it explains why a change was made.

- `12-decision-register.md`
  - Current major decisions and their strength levels.
