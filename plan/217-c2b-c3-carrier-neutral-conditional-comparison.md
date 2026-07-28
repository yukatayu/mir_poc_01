# Plan 217: C2-B/C3 Carrier-Neutral Conditional Comparison

## Role and non-effects

This LAB method compares possible C2-B/C3 semantics without selecting a carrier.
It creates no Core sort, pending object, identity, lifecycle, equality, grammar,
API, runtime state, save format, proof claim, or implementation authorization.
A candidate card can only be `CONDITIONALLY-SATISFIES`, `COUNTERMODEL`, `OPEN`,
`CARRIER-GAP`, or `OUT-OF-SCOPE`.

## Authority cut and inputs

The working cut is `373fd66925172a9ebed5e5699446759c180e7dee`. Canon is the
authority; Plans 215--217 are LAB. The independent review that challenged the
comparison shape has SHA-256
`d496ba61d986013e25177e065cc1444365884de50421a20156adc2ad6967d502`.

| Source | Exact use |
| --- | --- |
| `theory/01-mircore-v0` | zero-or-one occurrence, owner seriality, causal DAG |
| `theory/02-types-effects-failures` | type/effect/failure containment and static Diagnostic |
| `theory/03-elaboration`, `spec/04-core-ir.md` | explicit semantic strata and no hidden carrier |
| `theory/04-ordering-and-cuts` | complete SaveObject and admissible load |
| `theory/05-authority` | claims are not authority and lineage validation |
| `theory/06-existence-fallback` | conditional fallback and fresh reacquisition |
| P008, P012, P013, `OPEN-010`, `OPEN-011` | future elaboration, restricted use, M1, and open carrier questions |

Every positive row in a future card cites an exact Canon clause. A Plan digest
or Oracle statement is not a substitute for its source ledger.

## Carrier-neutral construction

For each candidate `K`, record `C` (exact pinned Canon), `H_K` (explicit
candidate-local hypothesis delta), and `D_K` (definitions over `C + H_K`
only). `D_K` is a definitional extension: erasing it changes no sort, stored
field, fresh value, equality, transition premise, SaveObject component, or
permitted trace. Each `C + H_K` model has a unique `D_K` expansion. `H_K` is
tagged `CANON-NATIVE`, `LAB-HYPOTHESIS`, `OPEN`, or `CARRIER-GAP`; it never
becomes adopted merely by appearing in a card.

The common layer has no Interaction, Attempt, Pending, request key, reply
token, receipt token, global same-request equality, or lifecycle enum. It uses
only metalinguistic role labels with candidate-native formulas of arbitrary
arity: `emit`, `owner-outcome`, `reply`, `receipt`, `consume`, `later-use`, and
`load-frontier`. Candidate-native linkage is a relation or hyperedge, not a
common key. Functionality, uniqueness, and equality are obligations only where
a candidate claims them; this method never assumes them.

## Required candidate-card content

For every claimed row, state primitive `H_K` facts and their semantic strata;
role/link formulas; operation type/effect/failure row; success and failure
observations; occurrence accounting; later dependency basis; claimed load
frontiers and restore correspondence; applicable validation grounds; fallback
scope; and one-shot assumptions. Missing material yields `OPEN`.

The card may leave exact reply/receipt/failure/resume carriers, resume
classification, post-load identity versus reconstruction, direct-edge
granularity, receipt multiplicity, fallback event decomposition, revalidation,
storage, transport/liveness, source grammar, and future elaboration theorem
parameterized. A row dependent on an open choice remains `OPEN`.

## Conditional obligation matrix

| Row | Conditional requirement | Decisive falsifier |
| --- | --- | --- |
| M1 locality | owner outcome observes request-associated M1 and separate applicable authority grounds; incidental metadata is never the sole link | same incidental facts with distinct contexts cross-use an outcome |
| rows and branches | success has requested type; dynamic failure is declared; static malformedness is not an undeclared dynamic branch | row widening, undeclared failure, or ill-typed consumption |
| reply/receipt/use | reply, receipt, acceptance policy, and consumption are distinct; owner success alone does not enable use | owner-success/no-receipt loads as consumable or failure enables use |
| step and DAG | each transition has zero/one occurrence accounting; observations add none; later use has non-incidental dependency | helper completion has no accounting or a causal cycle occurs |
| load and one-shot | only complete admissible loads; relevant observations restore and enable at most one consumption in one extending trace | three frontiers collapse or use reruns |
| authority | M1 stays non-authoritative; validation grounds and lineage remain distinguishable; load revives no stale authority | copied claims, rejoin, revocation, or post-load substitution grants authority |
| fallback | explicitly excluded, or monotone lineage and fresh reacquisition are observed | load rewinds an option or reuses severed lineage |
| ergonomics | negative compatibility boundary only; later elaboration may not invent identity, authority, dependency, or lifecycle state | generated IDs, hidden callbacks, side tables, or transport correlation are required |
| coherence | overlapping candidate-native links agree, including after load | M1/outcome and receipt/consume select different emissions |

## Stop conditions and next use

Record `CARRIER-GAP` when a dynamic fact needs a semantic residence not admitted
by Canon. A proof relation, evaluator variable, queue property, helper table,
matrix identifier, or test witness cannot fill this gap. Record `OPEN` when an
exact source clause, occurrence/carrier choice, complete load frontier, or
linkage is absent. Record `OUT-OF-SCOPE` for fallback only if no row relies on
it. Do not introduce hypothetical carrier axioms until ADR-0014 permits that
research shape.

Plan 215's `q,p,r,t` signature is historical shorthand only, not this method's
shared signature. The matrix requires candidate-native observations, explicit
receipt policy, at-most-one consumption, and no semantic work by comparison
definitions; it does not require one pending object, factorization key,
functional projection, or at-most-one receipt fact.

Prepare a first candidate card only when `H_K` needs no reserved semantic
extension. Exercise owner-success/no-receipt, accepted-before-consumption, and
consumed-result frontiers. A card needing a common key, new pending carrier,
hidden persistence, or future runtime/source machinery is a gap report.
