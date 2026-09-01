---
id: meta/proposal-038
status: L1-fixed
maturity: reviewed
depends_on: [root/north-star, root/design-constitution, meta/proposal-037, adr/ADR-0034, arch/01-strata, plan/01-phases]
summary: ALIGN-1でsemantic strata・PL責任層・lifecycle phaseを分離し、PL-0--PL-6の三軸mapを採用する提案。
open_items: []
---

# PROPOSAL-038 — ALIGN-1 project/product layer constitution

## Owner disposition and direct capability

Under the owner direction recorded by PROPOSAL-037 / ADR-0034, accept a
canonical three-axis architecture map so a context-free implementer can place
a responsibility without turning product, host or lifecycle vocabulary into
Mir semantics.

```text
Direct consumer: ALIGN-2 Browser/Host/package/View/provider contracts
Blocker reduced: current Canon mixed semantic S0--S6 with legacy S0--S7 and
  had no separate project/product responsibility map
Acceptance use: constrain ALIGN-2 and I3-0 so host, package and transport
  mechanisms cannot become semantic owners or public product contracts
```

## Selected design

Keep `arch/01-strata` as the semantic axis, add
`arch/06-project-product-layers` as the PL responsibility axis, and keep
`plan/01-phases` as the sole lifecycle-state authority. The axes are
many-to-many and mutually non-authorizing.

Semantic strata are exactly `S0 Surface`, `S1 Core`, `S2 Trace`, `S3 Verify`,
`S4 Projection`, `S5 Domain`, and `S6 Host`. Formal theory may remain scoped
primarily to `S0--S5`; `S6` is the non-authoritative host/realization boundary,
not a new theorem claim. Legacy realization `S0--S7` and `S7 Application` are
not current semantic strata.

The separate responsibility axis is:

```text
PL-0 Host / physical substrate
PL-1 Mir language and semantic kernel
PL-2 Mirrorea distributed fabric
PL-3 Mir Browser / Host safe participant runtime
PL-4 Shared-Space / World-Web platform responsibility
PL-5 Domain Kits and applications
PL-6 Reversed Library / knowledge-world separate project
```

Each PL records admitted input, output, prohibited flow and actual maturity.
PL-4 records only stack position, lower requirements, upper promises,
non-primitives, deferred questions and future owner clarification. PL-6 is not
a Mirrorea completion condition. Satellites remain outside the numbered stack.

## Smallest alternative and rejection

The smallest alternative was to append PL rows to `arch/01`. It is rejected:
one table would invite `S6 == PL-0`, `S5 == PL-5` and `I2 == PL-2` readings,
precisely the cross-axis drift this milestone must remove. A separate document
keeps the existing semantic and phase authorities stable and permits later
conservative responsibility refinement.

## Falsifier and acceptance

The primary falsifier is any context-free classification that obtains two
semantic owners, infers phase acceptance from a PL row, treats current `S7` as
a semantic stratum, makes a package/session/renderer authoritative, promotes a
domain noun into Core, or makes PL-6 a lower completion gate.

Accept only when all current Canon pointers use S0--S6 consistently; no LAB
maturity scale reuses `S`; the three-axis map covers PL-0--6 and PL-4 restraint;
the reader/status views expose the same separation; no runtime/source/sample or
lifecycle delta occurs; and independent architecture review has no P0/P1.

## Non-effects

This proposal does not change the North Star, accepted I2 semantics, Theory T1,
broad PHASE-I1, official I3 lifecycle, OPEN-032 or either transport candidate.
It defines no trust tier, BND-007 clarification, FFI/resource contract, public
surface, package, Shared-Space mechanism, product, deployment or production
claim. ALIGN-2 owns those detailed responsibility contracts.
