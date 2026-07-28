# Plan 210 - C2-B/C3 Family A/B instantiation audit

## Role and authority

This is a **LAB ordinary-design comparison audit** following Plans 208 and
209. `mirrorea_canon/` remains the normative source. It does not select Family
A or B as a Canon carrier, define an occurrence equality rule, add a relation
to Config/SaveObject/history, resolve OPEN-010/011, alter a source rule, or
authorize implementation, proof, Gate, Phase, or public behavior.

The purpose is narrower: distinguish (1) facts supplied by fixed Canon, (2)
the bounded directions recorded by P012 V1/R1 and P013 M1, and (3) the
carrier-neutral LAB audit judgments inherited from Plan 209. Those judgments
are not selected relation objects, fields, or carriers. "Not supplied" means
absence of a selected semantics at this cut; it is neither a negative result
nor a reason to insert a hidden carrier.

## Fixed input and comparison method

The audit uses the Plan 209 staged obligations:

```text
CtxOf(q, ctx)
PendingFor(q, p)
ValidatedOutcome(q, ctx, facts, outcome)
ReplyFor(q, p, r) / ReceiptFor(r, p, t) / FailureFor(q, p, f)
ResultOf(r, value, provenance) / Accepted(t, p)
ResumeOnce(p, t, r, value, provenance) / DepOf(later, p, t, value)
```

Where a relation is functional, this means each semantic reply, receipt, or
failure role has a unique applicable request/pending association in the stated
branch scope. Accepted receipt and resume, rather than transport delivery, are
one-shot. `ReplyFor` and `FailureFor` are mutually exclusive projections of
one applicable `ValidatedOutcome` branch.

The only current structural inputs are deliberately limited:

- theory/01 provides a request occurrence, owner queues, an owner-side
  validation sketch, explicit failed service with no store change, a
  request-to-serve edge, zero-or-one occurrence steps, `Gamma`/`Delta` in the
  judgment, and source spans in `G_e`;
- theory/04 supplies an occurrence DAG, a cut-backed SaveObject including
  queues/in-flight messages/provenance, and a well-formed restored
  configuration, but not occurrence equality or pending consumption semantics;
- theory/05 and P013 M1 supply request-local non-authoritative validation
  claims and the authoritative lineage/visibility facts that owner validation
  must compare;
- P012 V1/R1 supplies restricted one-shot result use and separate typed owner
  reply/requester receipt as a direction, while leaving the carrier open.
- architecture/04's L2 `MessageEnvelope` has an implementation-facing
  `envelope_id`, but its reply/receipt shape remains OPEN-030/011 and it is not
  semantic correlation, pending identity, or a Family C choice.

No entry below treats the current request grammar, a queue position, source
span, payload, transport endpoint, or causal ancestry as an implicit answer.

## Relation-by-relation comparison

| Staged obligation | Family A: relational administrative reference | Family B: request-occurrence anchor | Current conclusion |
| --- | --- | --- | --- |
| `PendingFor(q,p)` | can express a direct relation, but must choose the semantic locus and administrative state that make it meaningful | `q` can be the anchor, but occurrence existence alone neither creates nor identifies `p` | neither is instantiated; B has a structural anchor, A has a direct vocabulary |
| `CtxOf` / `ValidatedOutcome` | can state observational judgments between `q`, M1 context, authoritative facts, and service outcome without making claims authority | can define the same judgments from `q`, but `[E-SERVE]` wording does not choose their retained record or outcome projection | M1 supplies request-local non-authoritative claims and requires owner validation; the judgments need not be retained relation objects or outcome fields |
| `ReplyFor` / `ResultOf` | can name the owner result, requester-visible value, provenance, and redaction relation directly | the successful `[E-SERVE]` branch places a served occurrence after `q`, but does not identify one typed reply or its result | B cannot replace direct reply correlation with DAG ancestry |
| `ReceiptFor` / `Accepted` | can distinguish owner reply from requester receipt and state source-locus/order conditions | theory/04's generic send-to-receive order does not select a receipt occurrence or equality after load | OPEN-011 remains open in both; B has no implicit receipt mapping |
| `FailureFor` | can state the branch obligation between failed owner-service outcome, `q,p`, and its accounted failure path | an explicit owner-side failure occurrence exists, but no selected direct link makes it a requester pending transition | `FailureFor` does not resolve OPEN-010 or infer failure from absence of reply or queue state |
| `ResumeOnce` / held contexts | can state pending transition and exact success/failure `Gamma`/`Delta` disposition | `q` may be a provenance anchor, but cannot itself record zero-occurrence consumption or linear disposition | neither family is complete without a selected administrative state/relation |
| `DepOf` | can relate later occurrence to consumption of the receipt/result | `q` plus program/request-to-serve order does not prove a later action used this result | the later payload may be computed from, not equal to, the requester-visible result; C1 atomicity and later service remain outside this cut |
| restore/load | relation endpoints/status must be present in or uniquely reconstructed from restored configuration | preserved/reconstructed `q` equality plus staged judgments/projections and terminal status must be selected | theory/04 does not decide either preservation rule; B is not disproved |
| ergonomic projection | an elaborator may hide selected administrative relations only while retaining them and their source grounds | an elaborator may use a selected request occurrence as anchor only if it does not make source span or incidental data identity | neither family currently authorizes syntax or inference |

## What the Canon DAG does and does not provide

The DAG supplies ordering and prefix constraints for roles and edges
independently present in a chosen semantics. In particular, `[E-SERVE]`
constrains a successful served occurrence to follow its request, and an
already-defined send/receive edge is prefix-closed under a consistent cut.
Authority/membership/witness uses likewise retain named predecessor relations.
Neither clause identifies the R1 reply/receipt pair, establishes correlation,
or supplies post-load occurrence equality. These are necessary constraints for
both A and B.

They do **not** supply any of the following by themselves:

1. a functional `PendingFor`, `ReplyFor`, `ReceiptFor`, or `FailureFor`;
2. an association between a successful reply and its requester-visible result
   after redaction;
3. terminal status or one zero-occurrence consumption of a pending;
4. equality of an occurrence across a restored configuration; or
5. an M1 validation-outcome record that identifies the authoritative facts
   actually checked.

This avoids two symmetric errors. A cannot call an unspecified relation a
semantic object merely because it is convenient for a proof. B cannot call a
path through the DAG a correlation merely because an occurrence already
exists.

## Viability, stop lines, and non-selection

Family A is currently a **reference vocabulary**, not a viable selected
carrier: it becomes viable only if a design locates its staged judgments and
their status in a semantic configuration and gives load behavior. Family B is
currently a **conditional anchor candidate**, not a viable selected carrier:
it becomes viable only if a design states occurrence identity/equality under
the selected load scope and defines staged audit judgments or projections by
selected semantics stronger than ancestry, order, or incidental equality.

An explicit A or B instantiation fails when it cannot satisfy a Plan 209
obligation under its stated load scope. Such a failure is sufficient to reopen
comparison, including Family C, but does not by itself prove the entire family
impossible. Family-level rejection requires a source-backed argument covering
all admissible instantiations. Until then, Family C remains a reserve
comparison only.

## Minimum future design questions

The next ordinary Canon design package must decide, at no finer granularity
than needed for this cut:

1. whether staged judgments are primitive/relational as in A, or defined from
   a selected request-occurrence anchor with non-circular projections as in B;
   if both presentations appear, which is definitional and which is derived;
2. where pending, staged relations, result/provenance, validation outcome, and
   success/failure terminal status live in the semantic configuration;
3. the equality/reconstruction and one-shot scope used after load, within one
   admissible restored-prefix extension rather than as global exactly-once;
4. the occurrence/projection account for owner reply, requester receipt, and
   failure without resolving unrelated delivery/retry behavior;
5. awaiting / received-but-not-resumed / consumed / failed distinction, or a
   proved equivalent projection; one explicit branch-local disposition for
   each held `Delta`; and sufficient post-resume pure-computation/dependency
   state to avoid re-running a zero-occurrence resume after load;
6. exact `Gamma`/`Delta` disposition on success and failure; and
7. if a later source convenience omits administrative spelling, the elaborated
   artifact and diagnostic surface that preserve every selected semantic fact
   and satisfy Plan 209's projection conditions; no omission, syntax, or
   inference rule is authorized here.

These are a design-question set, not an order to add six fields or six Core
constructors. The smallest selected model may represent several through one
well-specified relation or projection, provided it keeps every required
projection explicit. Neither an A relation nor a B request-occurrence anchor
confers authority. Receipt and result dependency do not authorize or freshen
the later occurrence; it remains subject to its own owner-local or
grant/witness-lineage authority conditions.

## Reopen and non-effects

Reopen this audit on an owner/Canon selection for one of the questions above,
an A/B candidate that passes or fails a Plan 209 adverse row, or a
source-backed ergonomic proposal with a concrete elaborated artifact. Do not
open a new theorem, runtime helper, schema, queue, source syntax, or
implementation lane from this audit alone.
