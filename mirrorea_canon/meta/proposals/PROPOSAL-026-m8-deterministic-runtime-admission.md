---
id: meta/proposal-026
status: L1-fixed
maturity: draft
depends_on: [meta/proposal-018, adr/ADR-0015, adr/ADR-0020, adr/ADR-0022]
summary: M7 checked artifactだけを入力にする、有限 M8 runtime admission と deterministic lowering の提案。
open_items: []
---

# PROPOSAL-026 — M8 deterministic runtime admission

## Owner disposition

Under the owner-approved ADR-0015 M8 scope, select one finite deterministic
runtime handoff.  Its only source-program input is the immutable M7 checked
artifact.  An M8 admission input binds all of the following to the same
checked-program identity:

```text
StaticEnvironment + checked evaluation/Core shape + effect/obligation shape
+ stable source-to-Core map + residual source references + runtime evidence
```

The runtime must not reparse source, reclassify M6 input, reconstruct source
maps by evaluation name, or use a helper-local evaluator side table.  The
checked identity is structural: a module/name digest alone is insufficient.

M7 `execution_is_admissible` remains its static predicate.  M8 constructs a
separate finite `runtime-admitted` instance only after identity-bound evidence
settles the selected base residuals.  It must not relabel the M7 artifact as
residual-free or change M7's predicate.

The exact M8 base rows are:

| M7 residual | M8 evidence required | M8 outcome when absent/mismatched |
|---|---|---|
| `Visibility` | source-ref-bound relation release/redaction | `Rejected`, no semantic mutation |
| `RelationLifetime` | source-ref-bound live lease/binding frontier | `Rejected`, no semantic mutation |
| `FallbackValidity` | source-ref-bound primary/fallback epoch relation | `Rejected`, no semantic mutation |
| `ValueVisibilityRedaction` | source-ref-bound value label/redaction | `Rejected`, no semantic mutation |
| `AuthDeferred` | no M8 discharge | `DeferredToM9`, no authority/success/mutation |
| `VerifyDeferred` | no M8 discharge | `DeferredToM9`, no proof verdict/success/mutation |

Deterministic lowering consumes the retained checked evaluation/Core/effect/
obligation/source-map rows in their stable order.  It produces owner request /
local-read / owner-write actions, owner-held relation publication plus
consumer-local projection, or designated request / receipt-use / value
publication as applicable.  These are finite M8 actions, not transport
delivery or a public Core exchange format.

## Selected alternative and falsifier

The smaller alternative is to call M7's `require_execution_admission()` for
every checked artifact.  It is rejected because M7 deliberately leaves
relation and designated base residuals unresolved for M8, so that route cannot
run the selected M8 relation/designated slice.

The other alternative is to treat M7 residual success as implicit runtime
success.  It is rejected because it would turn `AuthDeferred` or
`VerifyDeferred` into hidden authority or proof success.

The selected route is falsified if an admitted runtime input has a changed
static environment, evaluation/Core/effect/obligation shape, source map,
program identity, or residual source reference; if a relation/designated
value is created without its listed evidence; or if auth/verify yields an M8
success before M9.

## Scope and non-effects

This authorizes ADR-0023, theory/17, spec/09, and the exact finite
OBL-050--056 evidence package.  It does not identify the fresh Lean types
with Rust/M5/M7 types, prove a general runtime theorem, implement transport or
distributed delivery, discharge M9, claim official SCN conformance, or freeze
a public API/ABI/wire.
