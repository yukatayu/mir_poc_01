# Plan 233: P017 X1 K0 B Fact-Status Screen

## Role and authority

This LAB ordinary-design card follows Plan 232. It classifies four owner-side
fact roles separately in retained A-Sigma and B-Pi presentations. It is not a
branch model, relation schema, lifecycle, transition system, validation design,
persistence design, proof, or implementation plan.

All eight rows start as `OPEN`. P017 requires the fact roles and safety
obligations, but it does not decide whether a fact is primitive or uniquely
derived. This card does not add auxiliary facts merely to avoid an `OPEN`
result. It selects no relation arity/carrier/family role, identity, record,
field, result carrier, failure member/row, validation/mutation rule,
occurrence/causal generator, Config/SaveObject placement, restore function,
source/runtime/API, theorem/OBL, conformance, Gate, Phase, sample,
implementation, or public behavior.

## Common obligations and status vocabulary

The scope is one V1/R1 cross-locus read anchored by in-history `q` under K0.
`q` is not source spelling, a public token, transport/session key, or global
cross-load identity. Incidental equality cannot merge effective facts.
Exactly-one requester pending is an R obligation, not evidence of owner
outstanding and not a branch key or package witness.

For both cards, retain these obligations without creating shared schema:

- owner service may remain outstanding indefinitely;
- at most one terminal owner success or typed owner-service failure is
  permitted, and they are distinct;
- owner failure has no owner mutation, while success is terminal only for owner
  service, not the whole exchange;
- success/failure are abstractly typed, and failure is row-contained without
  selecting a member or row;
- consulted membership, capability-lineage, witness, admission, visibility,
  and history grounds are immutable provenance references, not authority;
- raw rejected delivery changes no B fact, failure, occurrence, receipt status,
  use budget, or restore frontier; and
- relation membership/coherence/correspondence supplies no transition,
  causality, validation, mutation enforcement, or save/load proof.

| Status | Required basis |
| --- | --- |
| primitive | an explicit positive semantic relation membership in the card's native presentation |
| uniquely derived | a non-circular erasable definition from already-declared positive primitive memberships and common constraints, without a new subject, choice, witness, identity, package, or closed-world assumption |
| OPEN | neither basis exists, or closing the row needs an excluded surface |

Static ambiguity and underdeclared failure remain Diagnostic, not
owner-service failure. Storage and provenance references imply neither
observation nor authority.

## A-Sigma fact-status ledger

A-Sigma uses one primitive whole q-scoped relation restriction. Its fact views
remain non-exhaustive, not fields, constructors, slots, or a lifecycle enum.

| Fact role | Initial | Allowed primitive basis | Allowed derived basis | Open/falsifier |
| --- | --- | --- | --- | --- |
| owner outstanding | OPEN | positive outstanding membership | positive already-primitive pending-service fact with exact erasable view | absence of terminal facts, requester pending, or `q` itself is used |
| typed terminal success | OPEN | positive typed terminal-success membership | positive typed owner-result fact plus explicit local terminality premise | untyped value, absence of failure, or unselected transition supplies terminality |
| typed terminal failure | OPEN | positive typed terminal-failure membership with abstract row containment | positive typed failure fact plus explicit local terminality premise | failure member/row is selected, Diagnostic is folded in, or membership proves no-mutation |
| consulted provenance | OPEN | positive linkage to immutable actually-consulted grounds | positive consultedness facts determine linkage without choice | M1/current M/G/W/co-presence/source/transport/validation supplies consultedness |

Whole-slice correspondence may list primitive inputs requiring later closure; it
is neither equality, a restore function, nor proof that every fact survives.
Tags, fields, tuples, fixed relation roles, or complete view families leave
A-Sigma.

## B-Pi fact-status ledger

B-Pi has primitive q-anchored relation plurality with static
compatibility/non-sharing coherence and joint abstract correspondence. It does
not enumerate carrying families, add a role index, or package facts with a
branch/result witness.

| Fact role | Initial | Allowed primitive basis | Allowed derived basis | Open/falsifier |
| --- | --- | --- | --- | --- |
| owner outstanding | OPEN | positive native outstanding membership | positive q-anchored memberships and stated coherence determine it without packaging | absence of terminal facts, requester pending, or dedicated family index is needed |
| typed terminal success | OPEN | positive native typed terminal-success membership | positive typed memberships determine it without common result/branch witness or tuple | shared key/carrier, result identity, factor role, transition, or reachability is needed |
| typed terminal failure | OPEN | positive native typed terminal-failure membership with abstract row containment | positive typed memberships and static incompatibility determine it without row selection/packaging | coherence becomes residence, a row is selected, or membership proves no-mutation |
| consulted provenance | OPEN | positive native linkage membership to actually-consulted grounds | q anchoring and coherence determine linkage from positive facts without witness/pairing/identity/index | several matches require choice, or linkage needs object/tuple/record/latent fiber or validation |

Coherence may constrain terminal incompatibility, incidental non-sharing, and
compatibility of facts used together. It cannot establish a runtime package,
exhaustive lifecycle, dynamic reachability, validation success, authority,
causality, transition atomicity, or save/load correctness. Joint correspondence
is later closure obligation; independent component correspondence cannot
construct a restored account.

## Adversarial screen and stop conditions

| Case | Required result |
| --- | --- |
| equal span/payload/claims/queue/transport for two requests | B facts remain separately q-anchored |
| same source emits twice | source span is not branch identity |
| no terminal fact | outstanding stays OPEN without a positive native basis |
| success and failure both appear | no dynamic contradiction is inferred; a closed incompatibility row needs native basis |
| untyped success or uncontained failure | row stays OPEN; no carrier/member/row is invented |
| copied M1 claim or mere provenance reference | no authority, validation success, or actual consultation follows |
| repeated raw rejection | no B state, failure, occurrence, or frontier changes under K0 |
| owner failure membership | no-mutation stays an obligation, not membership evidence |
| changed authority after load | flag later no-revalidation/closure obligation only |

Downgrade a non-OPEN row to OPEN when its basis is missing. Stop the affected
card when it needs fixed field/family inventory, record/tuple/common witness,
identity/equality, closed-world absence, result carrier, failure member/row,
validation/mutation algorithm, transition/reachability, occurrence/causal
generator, receipt/acceptance/use/Gamma/Delta/one-shot behavior,
Config/SaveObject placement, restore function, runtime/proof/public claim, or
when K0 rejection becomes semantic state.

## Later closure inventory and disposition

Later L work must preserve only the primitive inputs or derivation premises of
owner-outstanding at emitted/service-pending, owner failure/provenance at
terminal owner-service failure, and owner success/provenance at owner
success/receipt-pending and later reliant frontiers. This names no storage or
correspondence property; K0 rejection adds none.

The card closes with eight `OPEN` rows, their legal closure bases, decisive
falsifiers, and later closure inputs. It neither eliminates A-Sigma/B-Pi nor
selects fact classification. A later candidate can close a row only through an
explicit H_K primitive basis or D_K derivation, then rerun this screen; T/U/C
work cannot assume it otherwise.

## Non-effects

This plan changes no Canon text, `working/` record, relation model, schema,
identity, transition, lifecycle, failure row, authority rule, observation
surface, save/load representation, source grammar, runtime, adapter, wire/API,
theorem/OBL, scenario, Gate, Phase, sample, implementation, or public behavior.
