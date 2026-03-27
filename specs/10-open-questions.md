# 10 — Open Questions

This file intentionally records unresolved matters.

## Mir semantics

1. Final formalization of preference chains / fallback normalization.
2. Exact syntax and static rules for overlays and route rebinding.
3. Exact relationship between `emit`, effect handlers, and structured event routing.
4. Final coroutine model:
   - one-shot vs multi-shot,
   - suspension restrictions,
   - patch interaction,
   - lifetime crossing rules.
5. Whether and how to support constrained multi-shot using existing research on linear continuations.

## Mirrorea / routing

6. Final route proof representation.
7. Interaction between route changes and suspended tasks/coroutines.
8. How much of scaling belongs to Mirrorea vs external orchestration.

## PrismCascade

9. Precise integration surface with Mir / Mirrorea.
10. Audio block semantics.
11. `fps=0` or equivalent call-set semantics.
12. Exact color/HDR/format negotiation model.
13. Degree of remote execution granularity.

## Typed-Effects Wiring Platform

14. Whether it should be a subsystem of Mirrorea or a sibling project.
15. Exact contract language for effect declarations.
16. Treatment of opaque or partially wrapped legacy effects.
17. Scale limits and abstraction methods for state↔event graph visualization.

## Reversed Library / application layer

18. Relationship between virtual-reality social mode and Reversed Library mode:
    - one continuous mode or explicit mode switch?
19. Knowledge classification strategy:
    - humanities / sciences / practical knowledge,
    - relation to existing library classification systems.
20. Progression / capability unlock design.
21. How to incorporate play-theory evaluation perspectives (for example Caillois-like axes) without distorting the architecture.
22. Synchronized web browsing model.
23. Mir-based GUI-programming substrate.
24. Mapping of earlier prototype diagrams to final Mir syntax and semantics.
