# Plan 239: P017 X1 K0 Consulted Validation-Provenance Basis and Nonconflation Screen

## Role and authority

This LAB ordinary-design card completes Plan 233's per-cell source-conformance
work. It compares only the consulted-validation-provenance cells for one K0
V1/R1 cross-locus read, separately in A-Sigma and B-Pi. It adopts no basis and
leaves every Plan 233 cell `OPEN`.

It selects no ground domain, cardinality, equality/identity, schema, record,
field, family index, validator, failure row, lifecycle, occurrence, causal
generator, persistence, receipt, source/runtime/API, theorem/OBL, Gate, Phase,
sample, implementation, or public behavior.

## Source cut and consumer

P013 M1 covers request-local non-authoritative claims and authority comparison.
P017 item 1 additionally requires immutable references to membership,
capability-lineage, witness, admission, visibility, and history grounds
**actually consulted** at owner validation. P017 item 3 is only a later outcome
compatibility consumer. Plan 236 separately screens result-producing grounds.

The immediate consumers are:

```text
Plan 233 / A-Sigma / consulted provenance
Plan 233 / B-Pi    / consulted provenance
```

No validation model, outcome attachment, causality, observation, or load rule
is supplied.

## Nonconflation and non-definability

M1 claims identify what was presented for checking; consulted validation
provenance identifies authoritative grounds actually used by owner validation;
result provenance identifies grounds producing a successful value. A ground may
later serve both provenance roles, but overlap/disjointness is unselected.

M1 claims/caprefs/witrefs, available current authority facts, validation
success/failure, source/payload/queue/transport data, q, Theory 04 ancestry,
result-producing grounds, and receipts cannot establish consultation. Theory 05
keeps every claim/provenance reference non-authoritative.

Hold q, M1 inputs, available authority grounds, validation outcome, and static
constraints fixed while varying the abstract consulted-ground incidence. The
incidence is therefore not derivable from these inputs. Separately vary
consulted grounds with result grounds fixed, then result grounds with consulted
grounds fixed. This prevents replacement by P013 M1 or Plan 236.

## A/B/C comparison

Candidate A is a positive native q-associated fact whose intrinsic role is
immutable references to authoritative grounds actually consulted:

```text
A-Sigma: Sigma_q positively contains consulted-validation-provenance(q)
B-Pi:    Pi positively contains, at q, consulted-validation-provenance(q)
```

A-Sigma must remain non-exhaustive; B-Pi needs no family/index, common key,
branch/result witness, pairing object, identity, latent fiber, or coherence as
residence. `Immutable` names P017's fact role only, not its installation or
load behavior. An adopted `H_K` would conditionally classify A as `primitive`.

Candidate B requires independently useful positive actual-consultation facts
plus a q-local, non-circular, erasable, choice-free linkage view. It must be
unique without a ground identity, pairing witness, schema, selected validator,
causal reachability, history lookup, or closed-world assumption. Current K0
does not provide this: M1 claims are inputs, authority facts are available
grounds, outcomes do not prove use, Plan 209 terms are audit vocabulary, and
Theory 04 predecessors do not encode consultation. B is only a comparison form.

Candidate C adds no basis and is operative.

| Alternative | Positive assumptions | Conditional status |
| --- | --- | --- |
| A | native consulted-validation-provenance linkage | `primitive` |
| B | independent actual-consultation facts plus exact view | `uniquely derived` |
| C | none | `OPEN` |

No A/B preference is adopted. Retain `OPEN` if A reconstructs consultation from
an excluded source or needs a representation surface; or if B merely renames A,
lacks independent premises, requires choice, or has two equal-input
interpretations with different linkage.

## Completion boundary and non-effects

This is ordinary LAB source-conformance/definability work, not L3. It selects
no model, semantics, theorem, or fixture. The Plan 233 per-cell inventory is
complete after this card but none of its eight bases is adopted. The next move
is a complete ordinary candidate `H_K` intake for the bounded P017 model, or a
stop record; no further autonomous per-cell cards are justified.

This changes no Canon text/status, `working/` record, authority/validation rule,
failure row, transition, occurrence/causal rule, save/load, grammar,
elaboration, runtime, adapter, wire/API, theorem/OBL, scenario, Gate, Phase,
sample, implementation, or public behavior.
