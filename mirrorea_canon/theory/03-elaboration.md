---
id: theory/03-elaboration
status: L1-fixed
maturity: draft
depends_on: [theory/01-mircore-v0, theory/02-types-effects-failures, adr/ADR-0002]
summary: Surface→Core elaboration の契約と THM-001(本プロジェクト第一目標定理)。
open_items: [OPEN-014]
---

# 03 — Elaboration and THM-001

## The elaboration contract (BND-001)

Elaboration is the function-like reading of the unified judgment: given a
well-scoped Surface item under (Σ, Ψ, Γ, Δ, L), it either produces
(c, A, μ, ε, φ, C, O, G_e) or a Diagnostic (theory/10). Contractual clauses:

1. **No hidden edges.** Every cross-locus consequence of the Surface item
   appears in G_e (request / publish / observe / witness rows). Nothing else
   may generate communication.
2. **Span preservation.** Every element of c and G_e carries the source span
   of the Surface syntax that caused it.
3. **Row containment.** φ(generated) ⊆ φ(declared `fails`), else E-ROW-001.
4. **Authority obligations.** Every generated request carries the capability /
   witness obligations that authorize it (discharged in C or listed in O).
5. **Determinism.** Elaboration is a function of its inputs (OBL-021).
6. **No authority creation.** Elaboration never mints grants; `O { ... }` from
   L ≠ O yields requests, not ambient authority.

## Worked shape (the canonical attack, SCN-02)

```text
BrowserClient[self] {                         (authority origin = self)
  when attack(target: Participant)
    fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
    S { player[target].hp = player[target].hp - player[self].atk }
  }
}
⇝  eval(r,
           body = player[target].hp - player[self].atk,
           EP = ⟨state, owner(S), on-request, caller(self), {store}, -, -⟩)
   with request(self → S, ρ̄ = {cap_write_player}, ω̄ = ∅,
                φ = {StaleMembership, MissingCapability, MissingWitness,
                     RouteUnavailable})
   G_e = { request row, S-local dependency rows(target.hp, self.atk),
           evaluation-plan row, spans }
```

This is the owner-store form of M3 `eval` (theory/13). Both operands are S-owned, so service at S reads
them and writes hp in one bounded owner transition. The requester never
receives either private value, and neither dependency is an actor-side
observe/read-request. A real other-owner operand instead needs an explicit
result/receipt path or a v0 Diagnostic (ADR-0016).

## THM-001 — Assignment elaboration soundness

```text
If  Σ;Ψ;Γ;Δ;L ⊢ s ⇝ c ▷ C;O;G_e  succeeds for a Surface assignment s,
then every write in c is either
  (a) owner-local at L, or
  (b) an explicit owner-directed request whose
      – authority obligation is in C ∪ O,
      – generated failure set is contained in the declared fails row,
      – dependency edges, including their evaluation locus, are recorded in G_e, and
      – source span maps back to s.
```

Status: OBL-001 (Lean statement), OBL-002 (proof). This is the project's first
theory gate exit (GATE-1). Corollary target: composing THM-001 over a program
gives "no undeclared communication" (OBL-004).

## Auto publish / observe

`visible vis [fields f̄]` on a state declaration causes owner-local writes to
visible fields to elaborate an additional publish row at declared visibility.
Private fields never auto-publish (E-VIS-002). Authority-bearing witnesses are
never silently created; devtools/audit witnesses may be, and are labelled so.

For a real other-owner operand, the only M3 success path is an explicit
`remote-result` receipt in Core/trace; otherwise `E-EVAL-CROSS-OWNER` is
returned. Cache windows and freshness optimization remain later projection
work and must preserve the dependency record (BND-006).
