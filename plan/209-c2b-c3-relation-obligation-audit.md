# Plan 209 - C2-B/C3 relation-obligation audit

## Role and authority

This is a **LAB ordinary-design comparison audit** following Plan 208. The
normative source remains `mirrorea_canon/`. This document neither chooses a
Core constructor, occurrence kind, identity/equality basis, Config or
SaveObject field, source grammar, elaboration rule, runtime, queue/wire
format, API, OBL, Gate, or Phase. It refines only the questions that an
eventual ordinary Canon proposal must answer without hiding a required
relation.

The audit is deliberately not a new ADR-0014 `working/` result: its outcome is
decision-preparation, not a theorem, countermodel, literal transcription, or
experiment in an existing evidence lane. A temporary Oracle challenge review
was used as advisory input; its response digest is recorded in this task's
report. The corrections below were checked against the cited Canon sources,
not adopted as external authority.

## Scope and comparison cut

The cut is exactly Plan 208's `C2B/C3-alpha` trace:

```text
request emission -> owner validation and typed reply/failure -> requester receipt ->
zero-occurrence restricted pure resume -> later dependent ordinary occurrence
```

`request`, `reply`, `receipt`, `pending`, and `dependent occurrence` below are
role names, not proposed Canon constructors. The cut contains one requester
locus, one owner locus, M1 validation context, one success branch, and one
failure branch. It excludes retry, timeout, cancellation, migration, durable
exactly-once, successful-write acknowledgement, source `bind`/`let` spelling,
and the later operation's service semantics.

The Canon anchors are unchanged: theory/01 supplies a request occurrence,
owner-side validation, explicit failure with no owner store change, and
zero-or-one-occurrence steps; theory/03 requires generated cross-locus edges,
source spans, declared failure containment, and no authority creation;
theory/04 requires a consistent save/load prefix; theory/05 makes claims
non-authoritative and rejects stale or severed lineage. OPEN-010 and OPEN-011
remain unresolved and are not silently closed here.

## Common obligations, made testable

Plan 208's four-ended `Corr` is retained only as a compact description of a
completed success branch. It cannot be the sole audit relation: after request
there is no reply/receipt, after reply there is no receipt, and on failure
there is no successful reply/receipt. The active comparison therefore uses
these staged, carrier-neutral audit obligations:

```text
CtxOf(request-role, validation-context)
PendingFor(request-role, pending)
ValidatedOutcome(request-role, validation-context, authoritative-facts, outcome)
ReplyFor(request-role, pending, reply-role)
ReceiptFor(reply-role, pending, receipt-role)
FailureFor(request-role, pending, failure-role)
ResultOf(reply-role, requester-visible-result, result-provenance)
Accepted(receipt-role, pending)
ResumeOnce(pending, receipt-role, reply-role, requester-visible-result, result-provenance)
DepOf(later-occurrence, pending, receipt-role, requester-visible-result)
```

These names are audit predicates/judgments, not proposed Canon primitives. A
candidate must make their applicable endpoints functional and must preserve
their direct meaning through restoration. In particular, for request `q`,
pending `p`, successful reply `r`, receipt `t`, and failure `f`:

```text
ReplyFor(q, p, r)       => owner(r) = owner(q) and q precedes r
ReceiptFor(r, p, t)     => locus(t) = source(q) and r precedes t
FailureFor(q, p, f)     => q precedes f and no matching successful ReplyFor branch
```

`ValidatedOutcome` records that owner validation compares request-local M1
claims with the authoritative facts used for this service outcome. Its facts
cover, as applicable, originating verdict, principal, admitted role, target,
membership epoch/incarnation, admission witness, grant-policy version,
referenced capabilities, required witnesses, visibility, and relevant
history/provenance. It does not select a data record, field layout, or wire
format. A DAG path, equal payload, source locus, source span, queue position,
session, or shared ancestry is not a substitute for these relations.

| Obligation | Minimum observable consequence | Rejection condition |
| --- | --- | --- |
| request/M1 context | `CtxOf`, `PendingFor`, and `ValidatedOutcome` associate this request with its checked M1 context and authoritative owner facts | copied claims alone create authority, or two same-claim requests collapse |
| typed reply/provenance | a successful owner outcome has a typed requester-visible result whose provenance and redaction policy are auditable | result value is used as identity, raw result leaks into history, or provenance/validation grounds are silently discarded |
| correlation | `PendingFor`, `ReplyFor`, `ReceiptFor`, and `FailureFor` are functional at their applicable endpoints within the audit's branch scope | payload, locus, transport, source span, or ancestry aliases two live requests |
| accepted receipt | exactly the matching requester-side receipt may change a pending from awaiting to received | an owner reply directly resumes requester computation, or a wrong-locus receipt is accepted |
| one restricted pure resume | an accepted matching receipt yields at most one zero-occurrence resume of the same requester-visible result/provenance; `Gamma` is restored and `Delta` has one continuation disposition | duplicate/late success resumes again, the value differs from the reply, or the later effect is counted as the resume |
| failure exclusion | a matching owner-service failure is terminal for this pending, has one failure-path `Delta` disposition, makes no matching accepted success receipt/resume, and makes no dependency derived from this failed pending/result | failure silently drops or duplicates held context, or later success derives from this failed pending |
| dependent action | the later ordinary occurrence records direct dependency on this consumed receipt/result | owner seriality, program order, or a shared pending name alone is used to claim the dependency |

"At most one" is the quantification domain of this audit: one admissible
restored-prefix extension. It is not a transport delivery promise, a global
claim across two independent loads of one SaveObject, or durable exactly-once;
the eventual across-load consumption scope remains unresolved.

## Family audit

| Family | How it can expose the common obligations | What must be stated before it is viable | Stop line |
| --- | --- | --- | --- |
| A: relational administrative reference | typed relations directly expose pending, validation/outcome, reply, receipt, failure, result/provenance, and held-context obligations | semantic locus of every relation, direct staged projections, and restore behavior | if a relation exists only in prose or evaluator meta-state, it is not a carrier-neutral solution |
| B: request-occurrence anchor | one request occurrence anchors M1 context plus direct staged relations to pending, outcome, reply/receipt or failure, and provenance | occurrence equality preserved or uniquely reconstructed under the selected load scope; direct relation, not merely a causal ancestor | if equal-payload requests or terminal states cannot be distinguished after restore, B cannot instantiate the obligations alone |
| C: nominal attempt/exchange | a fresh semantic identity maps injectively to request, pending, reply, and receipt status | freshness, equality, retirement, persistence, branch scope, and non-reuse rules | do not choose C merely to reuse a wire/session/queue identifier; it is a non-exhaustive comparison view, not a fallback priority |

The current comparison reading, corrected by Plans 215/216, is that A, B, and
C are non-exhaustive LAB views. A remains useful relation vocabulary and B
must satisfy every table row, but neither A priority nor an A/B failure is a
precondition for comparing C or a hybrid. No view is selected as a Canon
representation, and B is not approved merely because request-to-service order
already exists.

## Save/load frontier audit

Every row is a requirement on a future chosen representation, not a request to
add these fields to current `SaveObject`.

| Frontier | Must be present in, or uniquely reconstructible in, the restored configuration produced from an admissible SaveObject and consistent cut | Forbidden reconstruction shortcut |
| --- | --- | --- |
| after request, before owner result | request/pending relation, M1 claims, referenced authority-provenance links, source span, declared failure row, requester/owner relation, awaiting status, and held `Gamma`/`Delta` | source locus, equal claim/payload, queue position, or transport session |
| after successful reply, before receipt | above plus typed requester-visible result, result provenance, validation grounds, reply role, and direct `ReplyFor` relation to the still-awaiting pending | treating reply arrival at the owner as requester receipt |
| after receipt, before pure resume | receipt role, its request/reply relations, received state, result/provenance, and held contexts | recomputing a receipt from a matching payload or an arbitrary history path |
| after pure resume, before dependent occurrence | post-consumption administrative state sufficient to exclude another resume, computed value/dependency ground without re-running resume, and linear-context disposition | a later program step or owner seriality as proof that consumption occurred |
| failure branch | direct `FailureFor` relation, failure reason/validation outcome, terminal state, and accounted held context | absence of a reply or volatile queue loss as proof of failure |
| after dependent occurrence | the consumed pending/result dependency remains distinguishable from later unrelated requests | final store equality or a common owner locus |

The standard intentionally permits a compact representation, but no candidate
may claim reconstruction without naming the relation, the restored
configuration state on which it relies, and its uniqueness argument. No
particular `SaveObject` field follows from this audit.

## Adversarial discrimination matrix

| Input | Required conclusion | What it tests |
| --- | --- | --- |
| two requests have identical payload and M1 claims | they remain distinct pending/correlation instances | identity is not value equality |
| two active principals share a requester locus | authority and correlation do not collapse to locus or transport | theory/05 non-authority boundary |
| a duplicated or late success reply arrives | no second accepted success receipt, pending transition, or resume after the post-consumption/failed state | one-shot state is observable and retained |
| a receipt is delivered at a wrong locus | it cannot consume the requester pending | semantic requester/locus relation, not endpoint identity |
| copied or replayed M1 claims arrive | authoritative membership/lineage/witness/history validation decides; claims do not grant | P013 M1 and theory/05 |
| leave/rejoin, revocation, or lineage mismatch occurs | the owner-service outcome is fail-closed with no owner mutation and no matching success continuation | stale authority and requester failure boundary |
| an owner-service failure outcome is established before matching success receipt | no matching accepted success receipt, pure resume, or occurrence whose dependency is derived from this failed pending/result | terminal failure exclusion |
| save/load occurs at any frontier above | correlation, status, provenance, and linearity still discriminate the branch | theory/04 load admissibility |
| owner interleaves before the later write | no read-modify-write atomicity follows | separation from C1 / SCN-02 |

The finite WRK-0033/0034 evidence supports only the local need for matching,
single-use, and failure exclusion under an opaque fixed model. It does not
discharge any row of this matrix for Mir.

## Ergonomic projection boundary

The intended user experience may omit **administrative spelling**, but never
the semantic fact itself. A later source-level convenience candidate is
eligible for comparison only when all of the following are shown for its
elaborated artifact:

1. each dynamic evaluation of one syntactically delimited cross-locus result
   use yields exactly one request role and one pending role, while its source
   span remains provenance/diagnostic information rather than correlation
   identity;
2. the compiler-generated representation retains direct correlation,
   result-provenance, requester/owner relation, failure row, and held-context
   accounting;
3. any compiler-generated administrative discriminator, relation, or anchor
   required by the selected Canon model is opaque and not inferred from
   payload, claims, principal, locus, source span, session, queue position, or
   other incidental values; a fresh nominal identity is not required unless
   separately selected;
4. the restored configuration still has the required unique reconstruction;
5. a diagnostic can identify the source span and failed relation when the
   convenience cannot be elaborated; and
6. the construct has no retry, multiple outstanding use, migration, hidden
   authority, or implicit later write.

Thus a compiler may eventually *generate and hide* the administrative relation
or anchor selected by the model for a simple single-use form. That is different
from treating correlation as absent or inferring semantic identity from
incidental values. Any non-unique case, any explicit retry/multi-pending case,
and any case whose result freshness or authority must be revalidated remains
explicit until a selected model proves an equally precise presentation.

This is a design constraint for future elaboration work, not an approved
surface syntax, inference rule, or implementation task.

## Conclusion, unresolved choices, and non-effects

This audit establishes no new semantic fact. It narrows the comparison test:
Family B is viable only if it supplies request-occurrence identity/equality
preserved or uniquely reconstructed under its selected load scope **and**
direct staged relations; otherwise the ordinary design must compare Family C
rather than silently adding a hidden relation. Family A is a useful reference
only if its relations have an explicit semantic locus and restore behavior. The
user-facing convenience direction is compatible with this result only as a
trace-preserving elaboration projection after a carrier is selected.

Still UNRESOLVED for owner/Canon selection are: the identity/equality basis
and load scope; the locus and lifecycle of pending/correlation/status;
requester-side failure and revalidation timing; held `Delta` disposition; and
the exact retained elaborated artifact for M1 context, validation outcome,
provenance, source span, and dependency grounds. No Core, Config, history,
SaveObject, source grammar, runtime, transport, proof/OBL, Gate/Phase, sample,
or public status changes.

Reopen this comparison on an owner/Canon carrier choice, a concrete candidate
or hybrid that can meet every projection condition, a matrix-row defect, or a
source-based ergonomic proposal with complete elaboration evidence. Do not
create a new theorem or runtime helper solely from this audit.
