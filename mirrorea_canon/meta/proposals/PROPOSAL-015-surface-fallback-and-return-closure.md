---
id: meta/proposal-015
status: L3-open
maturity: draft
depends_on: [spec/01-lexical-and-modules, spec/02-surface-grammar, theory/01-mircore-v0, theory/06-existence-fallback, scenarios/SCN-08, meta/proposal-004, adr/ADR-0014]
summary: SCN-08 の scalar terminal fallback と Surface v0 の未elaborated return を、明示的 closure として扱う owner disposition を記録する。
open_items: []
---

# PROPOSAL-015 - Surface fallback and return closure

> Decision-request artifact. The owner disposition is recorded below. It
> authorizes only bounded follow-up design packages; it does not yet change a
> grammar, Core rule, scenario, parser, checker, runtime, OBL, Gate, Phase, or
> public contract.

## Target and authority boundary

SCN-08 illustrates `room_anchor` as scalar state and names a terminal
`default_pose`, while current Surface grammar specifies only indexed state and
does not define a terminal/default declaration form. Separately, Surface
grammar and lexical text include `return`, but MirCore v0 has no corresponding
Surface form or elaboration rule. These are closure gaps, not evidence that a
new control-flow primitive, implicit fallback, arbitrary keyspace, or domain
`World`/`Game` concept belongs in Mir Core.

## Owner disposition

Recorded on 2026-07-28:

1. **Scalar terminal/default fallback is explicit.** A later package must give
   SCN-08's scalar anchor and terminal/default value a declared Surface/Core
   correspondence. It must not silently encode scalar state as an undeclared
   membership keyspace, infer a terminal value from a type, or synthesize an
   unbound `default_pose`.
2. **`return` is excluded from the v0 exact fragment.** A later Surface wording
   package must remove its statement production while retaining an explicit
   rejection policy for the reserved token (`E-PARSE-001` unless a later
   diagnostics package allocates a more specific existing-family code). It must
   not invent a control-flow rule merely to preserve the token.

## Required follow-up boundary

Before Canon integration, the scalar/default package must choose and test only
the minimum needed correspondence: scalar declaration/reference scope,
initialization/default evidence, chain target resolution, and the relation to
the existing fallback lineage law. It must show that SCN-08 preserves a
participant-indexed `live_pose` plus a longer-lived scalar anchor/terminal
without changing THM-002's monotone degradation or introducing a hidden
reacquire.

The package must state whether a scalar state is represented by a distinct Core
declaration or a conservative elaboration into an already declared finite
domain. That representation is **UNRESOLVED** here. It may not be selected by
example repair or implementation convenience. Any change to the frozen
SCN-08 expectation requires the ordinary ADR route.

## Non-effects

This disposition does not:

- close OPEN-005, choose chain syntax beyond its existing lineage floor, or
  change the fallback, lease, cut, or reacquire semantics;
- add `Unit`, a default-value builtin, implicit initialization, a general
  keyspace declaration, a `keyspace` keyword, `World`, `Game`, or a domain
  primitive;
- choose a final parser/AST, Core state representation, diagnostic catalog,
  implementation, SCN conformance result, proof, OBL status, Gate, Phase, or
  public API; or
- alter PROPOSAL-004's Participant-only indexed-state decision.
