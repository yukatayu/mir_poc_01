# Plan 241: P017 X1 K0 H_K-rs Occurrence-Accounting Preflight

## Role and authority

This LAB ordinary-design preflight follows Plan 240. It tests only whether the three-node `q -> s -> r` account can be stated as a reversible candidate without silently selecting a Core primitive, occurrence kind, history schema, operational rule, binding interpretation, or other reserved surface. Canon remains normative.

The result is **`PREFLIGHT-ADMIT`**, with a strict meaning: an ordinary LAB screen may compare an explicitly hypothesized candidate history containing `q`, `s`, and `r`. It does not establish that current MirCore operational semantics generates `r`, that Canon supplies a successful requester receipt, or that P017 X1 has a complete model. No `working/WRK-*` record is opened by this plan.

The phrase **receive role** is intentional. Theory 04 supplies `send -> receive` as a member of the causal generating family; it does not supply a generic Core constructor, occurrence tag, or operational receive rule. Thus `r` below is a candidate occurrence bearing an `H_K` receive role, not an existing Canon occurrence kind.

## Source cut and alternatives

DIRECT constraints are theory/01, theory/02, theory/04, theory/05, P012 V1/R1, P013 M1, P017 X1, and ADR-0014. Plans 227 and 230--240 are LAB memory. A temporary Oracle review `p017-hk-rs-occurrence-preflight` challenged this plan against that cut. Its advice is restated and source-checked here; it is not an authority source.

The bounded scope remains one V1/R1 cross-locus read anchored by request occurrence `q` in one current or admissibly restored history, under K0 external rejection. The preflight compares exactly these alternatives:

| Alternative | Status at this cut |
| --- | --- |
| literal C receipt endpoint | unavailable: Canon fixes `q`, successful `s`, and `q prec s`, but no successful requester receipt occurrence or carrier |
| `H_K-rs` candidate trace | screenable: hypothesize a three-node role account and test its boundary/integration requirements |
| defer | required if any needed fact crosses a reserved boundary or cannot be made explicit |

`PREFLIGHT-ADMIT` selects none of these as Canon semantics. It permits only the second alternative to be prepared as a possible conditional LAB candidate.

## C-level constraints

The following claims are C and may constrain the screen:

| Item | C-level reading |
| --- | --- |
| history | `H = (E, prec)` is an acyclic occurrence DAG; `prec` is the transitive closure of Theory 04's fixed generating family |
| request | `[E-REQ]` appends request occurrence `q` |
| successful service | successful `[E-SERVE]` appends one served occurrence `s` and fixes `q prec s` |
| service failure | failed `[E-SERVE]` appends row-contained failure occurrence `f` and makes no owner-store mutation |
| step accounting | one primitive step appends zero or one occurrence; a reply projection cannot silently create a second same-step occurrence |
| causal vocabulary | `send -> receive`, grant/use, witness/use, authority-evidence/use, membership/dispatch, program order, and state dependency are existing Theory 04 generator families |
| receipt boundary | owner service/result, requester receipt, receipt acceptance, and restricted use remain distinct under P012 R1 and P017 X1 |
| authority | M1 claims and recorded provenance are non-authoritative; service validation uses live authoritative grounds |
| load | every relied-on post-load fact needs consistent-cut and causal-or-channel closure; independent loads provide no global occurrence equality |
| K0 | raw external rejection is candidate-local only; it is not C semantics |

Two negative readings are equally binding: `q prec s` does not identify a particular generator instance, and `[E-SERVE]`'s `read+reply` wording does not fix a requester receipt endpoint. OPEN-011 therefore remains open.

## Candidate hypothesis inventory

The candidate must not collapse its load-bearing assumptions into an informal arrow. The following are `H_K`, never C, if a later candidate uses this route:

| Hypothesis | Required content |
| --- | --- |
| `H_sr1` | existing nodes `(q, s)` directly instantiate `send -> receive`, with request-send and service-receive roles |
| `H_sproj` | `s` additionally bears a co-located reply-send projection; it creates no second occurrence and no internal service/reply order |
| `H_r` | the candidate history contains a distinct later occurrence `r` at the requester locus admitting a receive role; no occurrence kind is asserted |
| `H_sr2` | `(s, r)` directly instantiate the existing `send -> receive` generator using the reply-send and receive roles |
| `H_result-send` | the reply-send role is tied to the exact typed owner-result fact for this successful branch, not merely the word `reply` |
| `H_receipt` | a positive candidate-local relation associates `r`, the q-anchored pending branch, and the exact typed owner result from `s` |
| `H_match` | matching is functional enough that no `r` completes two branches and no branch accepts two semantic receipts; incidental equality is not a key |
| `H_K0` | a raw rejected delivery is outside the semantic exchange: no `r`, semantic failure, semantic receipt state, use-budget change, or restore frontier |

`H_r` is extensional: a candidate history contains `r`. The preflight does not add an `[E-RECEIVE]` rule, enabling condition, scheduler action, source form, or reachability assertion in order to create it.

## Erasable definitions and unresolved facts

Only after the corresponding hypotheses are explicit may the following be `D_K`:

| Definition | Allowed meaning |
| --- | --- |
| `request-send(q)`, `service-receive(s)`, `reply-send(s)`, `requester-receive(r)` | role names over fixed candidate hypotheses, not history fields or occurrence kinds |
| `q prec r` | transitive consequence of `H_sr1` and `H_sr2`, not a third primitive edge |
| distinctness of `q`, `s`, `r` | consequence of strict causal order and acyclicity, not a new exchange identifier |
| `semantic-receipt-occurrence(r)` | alias only after positive receipt association and typing hypotheses |
| matching from the path | erasable only if an explicit uniqueness proof makes it unique; otherwise it remains `H_match` |
| state labels | non-exhaustive views over positive candidate facts, not fields, tags, or a lifecycle enum |

The following remain `OPEN` at this preflight: an operational step that appends `r`; any concrete occurrence kind/carrier/payload; OPEN-011's reply/receipt carrier; all positive owner-basis selections from Plans 234--239; the exact A-Sigma/B-Pi relation presentation; owner-failure requester continuation; receipt acceptance, `Gamma`/`Delta`, consumption, and later dependency order; Config/SaveObject placement and restore relation; runtime reachability; and transport, delivery, retry, fairness, timeout, or public behavior. They may be assumed only in a later explicitly conditional integrated candidate; they may not be relied on while remaining implicit.

## Falsifiers and reserved-boundary stops

The screen must reject the `q -> s -> r` account when any of these occurs:

| Falsifier | Required outcome |
| --- | --- |
| `r` needs a `Receive`, `ReadReceipt`, or equivalent constructor/tag | Canon gap; stop |
| reply-send needs a second event in `[E-SERVE]` or an internal edge in `s` | falsify this three-node account |
| `q prec s` or `s prec r` is treated as proof of `send -> receive` without a direct candidate hypothesis | falsify the causal mapping |
| matching uses span, payload, queue position, adjacency, locus/principal alone, transport/session metadata, or current store equality | falsify matching |
| fixed positive facts still admit two q/result associations for one `r` | matching is not `D_K`; add explicit `H_K` or defer |
| a saved key, exchange/result/reply identifier, common witness, fiber identity, or cross-load equality is needed | Canon gap; stop |
| matching/typing adds result, provenance, q, pending, or acceptance fields to `s` or `r` | history-schema stop |
| raw rejected delivery creates `r`, a semantic rejection state, an occurrence, or a dynamic failure | leave K0; stop this candidate |
| rejected receipt creates a new failure row/member or folds into owner failure | failure-boundary stop |
| receipt typing needs a new `Receipt<tau>` type/Core constructor | Core/type stop |
| `s` is the requester receipt, or `r` is owner service/result | R1/X1 falsifier |
| relation membership, acceptance, coherence, or restore correspondence is used as a causal generator | falsify the claimed edge |
| zero-occurrence consumption is inserted into `H` only to create an edge | falsify |
| claims, provenance, anchoring, locus, transport, or the receive edge becomes authority | authority falsifier |
| load reconstructs matching/acceptance/provenance/use from adjacency, current authority, payload, or `r` alone | persistence falsifier |
| a restored state contains `r` without required predecessors and relation/channel closure | load-admissibility falsifier |

Stop for ordinary Canon work, rather than repairing a candidate, if it needs a new Core form/primitive; occurrence constructor/kind/action label/history field; `[E-*]` rule; zero-or-one discipline change; causal generator; concrete relation layout/identifier/persistence key; failure member/row; Config or SaveObject field; source/elaboration/G_e change; runtime/transport/API; unreviewed raw observation; theorem/OBL, scenario, Gate, Phase, or implementation claim; or an L0/L1 claim that Theory 04 already fixes this request/service/reply/receipt interpretation.

## Minimum coupled integration for a future candidate

A standalone acyclic path is not P017's minimum model. Before an L3 record can claim a conditional integrated candidate, it must use one presentation and close every relied-on R/B/T/U/C/L row with explicit `H_K` or proved `D_K`:

1. **R**: one q-scoped semantic relation, exactly-one non-shared requester pending binding, held `Gamma`/`Delta`, same-history association to `s` and `r`, and no public/transport/cross-load identity.
2. **B**: positive candidate bases for owner outstanding, exclusive typed success/failure, typed result, consulted validation provenance, and result provenance. `s` is tied to the successful branch/result; `f` is tied to the failure branch with no mutation and no `s/r` success chain. Plans 234--239 are advisory basis comparisons, not inherited bases.
3. **T**: one q-pending/s-result/r receipt association, existing type `tau`, functional matching, distinct receipt acceptance, and K0 rejected delivery with no semantic occurrence or state transition.
4. **U**: accepted receipt alone enables one administrative restricted use; consumption is zero-occurrence unless another existing occurrence is named; failure and raw rejection enable none; exact `Gamma`/`Delta` disposition is stated at owner success, acceptance, consumption, failure, and load.
5. **C**: direct `H_sr1` and `H_sr2`, transitive `q prec r`, explicit owner result linkage, every relied-on authority predecessor mapped to an existing generator, a named existing generator for each later dependent occurrence, and an acyclic complete projection graph.
6. **L**: live-fact lists for emitted/service-pending, owner failure, owner-success/receipt-pending, accepted/unconsumed, and consumed frontiers; a cut containing `r` includes `s` and `q`; a cut between `s` and `r` has abstract channel/in-flight closure; no revalidation, merge, duplicate, reset, or stale-authority resurrection occurs on restore.

No storage, history, or save fact becomes observable merely by appearing in this inventory. Raw result, receipt, validation, and provenance exposure remains behind a separate typed theory/07 projection.

## L3 eligibility and next autonomous package

ADR-0014 permits a later existing-LAB-lane L3 record only if it is an integrated, presentation-specific, falsifiable conditional trace model. A record that merely asserts a path `q -> s -> r` has no independent semantic consumer and is not eligible as a P017 minimum-model record.

Before a registration, the next package must select one candidate presentation without inventing a comparison schema, pin Canon and LAB inputs, identify an existing permitted LAB lane, preregister C-only/H_K-rs/defer alternatives and adverse cases, freeze every load-bearing `H_K` and erasable `D_K` fact, name a consumer for reply, receipt, acceptance, use, causality, and load, and disclaim operational reachability, runtime, Canon satisfaction, implementation, and public behavior. Failure of any condition yields `DEFER / Canon gap`, not an L3 registration.

No user decision is requested by this preflight. A subsequent candidate selection remains autonomous only while it stays inside this boundary.

## Non-effects

This plan changes no Canon text/status, `working/` record, Core, Config, SaveObject, relation schema, identity, transition, occurrence kind, causal family, failure row, authority rule, observation surface, source grammar, elaboration, runtime, adapter, wire/API, theorem/OBL, scenario, Gate, Phase, sample, implementation, or public behavior.
