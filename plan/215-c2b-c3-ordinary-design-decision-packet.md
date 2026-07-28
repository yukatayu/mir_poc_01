# Plan 215 - C2-B/C3 ordinary design decision packet

## Role and authority

This is a **LAB ordinary-design decision-preparation packet**. The normative source remains `mirrorea_canon/`. It does not select a Family A/B/C carrier, Core constructor, occurrence equality rule, pending state, relation, Config, SaveObject, source form, elaboration rule, runtime, wire/API contract, OBL, Gate, Phase, or public behavior.

The packet reduces the owner-facing decision surface while preserving the distinction between settled bounded directions and unselected semantics. Its three bundles are an organizational synthesis of existing questions, not a new Canon decomposition. A normal Canon proposal and owner decision are still required before a shared model may rely on any bundle.

## Authority cut and inputs

Review cut: `8b201d0ecc061d698d63f9fc02deb1d2d69fc81c`.

| Input | SHA-256 | Role |
| --- | --- | --- |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | autonomous-research and reserved-boundary rule |
| P012 / P013 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` / `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | recorded V1/R1/SW1/conditional-A2 and M1 directions |
| theory/01 / 04 / 05 | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` / `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` / `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` | request/step, cut/load, and authority constraints |
| theory/02 / 03 / 06 | `40c49504e86162fb065d0f5850c4039d88d08af30da7d12dc2e073c43a107257` / `2d703895da4f75bf57848275db6ae03e0abe7d56f62a11ef364af8fe22677641` / `3da20d43a0a87ec8417a4519700777adea141f499e2627f433927ce975a086c8` | type/effect/failure, elaboration, and conditional fallback constraints |
| spec/04 / spec/05 | `50c23acf01deedbe5bdb78baeba58053e28c940d8202b6d25bfd1f03546fd950` / `25749e3b171659fa59e3de6ff49126e15331ef52cf3ba5337ece4c46e72ca06c` | Core/edge stratum and runtime-observation boundary |
| P008 / Plan 197 | `777a6b2e043ae0313c402c836341bdedf9e12758f480c44fef8391715d34f3dc` / `95c5e4276c04495f97eaad21aa812192e87de2a81be5f48dba6dd4f71fb5bb2d` | separate elaboration-totality boundary and I1 non-readiness cross-check |
| Plans 208 / 209 / 210 | `84380a9d2f9929f4ffe5d48f4baf74083fb00cc34d30132e43c45f48f1ddef55` / `50dc299076df7844f3dd2fe641bbd65a57269d305743556bda07525c588faefa` / `4cba73fdbb245b16bf9fdd312609401518abaae6c96273923e1bf861e1548ffe` | common obligations, adverse rows, and A/B/C stop lines |
| Plan 214 | `b6a2a3023ad617ae9624797a223699e653170d4d69cc906a6bc59ef241e1e658` | post-WRK-0039 no-successor disposition |

A temporary GPT-5.6 Sol Pro review (SHA-256 `45d11b4325b04d3e4a81f480cfd4e3d29f9cf587a101320c5024ced1d0e1c42f`) proposed the initial packet shape. A second broad-boundary review (SHA-256 `19cae3668c56cfbd1fe54ea49e3a67c684b85974536d818ed228e76ed838003f`) found missing compatibility constraints, which this revision incorporates. Both are advisory LAB input only. The decision surface below follows the pinned repository sources and retains all unsupported assumptions as unresolved.

## Bounded directions and comparison boundary

P012 V1 permits a later restricted, one-shot, locus-bound use of a read result with held `Gamma`/`Delta`; it does not select a carrier or source form. P012 R1 permits separate typed owner reply and requester receipt; it does not select correlation, delivery, or transport behavior. P013 M1 permits request-associated, non-authoritative validation claims; it does not select request/occurrence identity or encoding.

The decision cut is only:

```text
request emission -> owner validation and typed reply/failure -> requester receipt
-> zero-occurrence restricted pure resume -> later dependent ordinary occurrence
```

The later occurrence's service semantics, result freshness, and atomicity are outside this cut. WRK-0039 remains finite key-supplied presentation evidence; it does not decide semantic correlation, equality, persistence, or inference. Plan 214 therefore prohibits treating a renamed finite theorem as a substitute for the ordinary decision below.

The common audit vocabulary is Plan 209's staged signature:

```text
CtxOf(q, ctx)                  PendingFor(q, p)
ValidatedOutcome(q, ctx, facts, outcome)
ReplyFor(q, p, r)              ReceiptFor(r, p, t)
FailureFor(q, p, f)            ResultOf(r, value, provenance)
Accepted(t, p)                 ResumeOnce(p, t, r, value, provenance)
DepOf(later, p, t, value)
```

These are comparison judgments, not selected Canon predicates, fields, or constructors. Completed-success shorthand cannot be the sole specification because it does not exist before reply/receipt and cannot state failure.

## Cross-boundary candidate constraints

Before any concrete candidate is compared, it must state a semantic-stratum
map: which facts are definitional, dynamically produced in Core/`G_e`/history/
configuration/channel state, or uniquely derived. A checker, evaluator,
transport, queue, proof-only relation, or helper-local side table cannot carry
semantic correlation invisibly. Field names, JSON, storage layout, and final
wire form may remain deferred only after this map exists.

The candidate domain is cross-locus result-returning access only. Owner-local
`read` remains the existing dependency-only path and must not acquire pending,
reply, receipt, or correlation lifecycle. Each new pending, reply, receipt,
acceptance, consumption, failure availability, and later action must be mapped
to an occurrence, a zero-occurrence transition, or a derived projection in a
way compatible with MirCore's zero-or-one-occurrence step discipline, owner
seriality, and causal DAG acyclicity. This does not select any of those maps.

Result, success availability, receipt, and dynamic failure must be typed with
respect to the requested operation. Every generated dynamic failure must be
contained in the declared request failure row; effects and rows may not be
silently widened. Ambiguous correlation, underdeclared rows, malformed fallback
metadata, or inability to establish the admitted administrative distinction are
static Diagnostics, not generic runtime `Reject`. Awaiting has no liveness
guarantee: at-most-once safety, observed-branch partition, and eventual
delivery are separate claims.

If fallback-mediated access is in scope, the candidate must preserve selected
option and lineage, monotone degradation, no rewind after load, and fresh
explicit reacquisition. Otherwise it must explicitly exclude fallback-mediated
access from this cut. If it defers SW1 or conditional A2, it must state that
D1--D3 are genuinely parametric over those event decompositions; otherwise the
relevant compatibility decision is part of the candidate.

## Minimum decision surface

The owner-facing proposal must supply one coherent candidate for all three bundles. It may use a compact object or a derived projection rather than one field per listed fact, but it must state every required projection and meaning.

### D1 - Definitional correlation basis

| Alternative | Minimum content | Stop line |
| --- | --- | --- |
| Family A: relation-first | direct relations have stated locus, functionality, branch meaning, and restore behavior | a relation only in prose, evaluator meta-state, or proof notation is not a solution |
| Family B: request-occurrence anchor | a selected request occurrence has direct non-circular projections to staged facts; equality/reconstruction scope is stated | ancestry, source span, payload, claims, queue position, locus, session, or transport endpoint alone is not correlation |
| Family C: nominal attempt/exchange | a fresh semantic identity has explicit equality, lifecycle, persistence, retirement, and non-reuse rules | no wire/session/queue/envelope identifier may be relabelled as semantic identity |

Families A/B/C are non-exhaustive LAB comparison views, not a Canon taxonomy or
admissibility order. A concrete or hybrid candidate is acceptable only if it
states what is definitional versus derived and closes every cross-boundary and
adverse-case constraint without hidden identity. No family is selected or
globally rejected by this packet.

### D2 - Branch model and explicit projections

M1 context is semantically request-local. It may be carried by, or projected
directly from, the selected request-associated object, but it may not be
recovered by ambient lookup, a correlation-only relation, transport, queue
position, source locus, session, or incidental equality. For each owner outcome
the candidate must account for the authoritative validation frontier: verdict,
principal, role, target, epoch, incarnation, capability lineage, required
witness, grant-policy version, visibility, and relevant history.

The candidate must distinguish request emission, pending creation, owner
outcome, owner success availability, requester receipt, receipt acceptance,
success consumption, explicit owner failure, requester-side failure
availability, and the corresponding pending disposition. Result value,
redacted history/audit projection, result provenance, authority-lineage
provenance, source/diagnostic provenance, source span, and failure information
remain separate. A compact representation is allowed only if every projection
is explicit and functional.

Reply remains distinct from receipt. Only matching accepted requester-side
success may enable V1 computation; failure cannot enable it. Failure is not
inferred from missing delivery, timeout, queue loss, absence of a reply, or
receipt possession. The candidate must state whether invalid/duplicate/stale
receipt rejection is audit-only or a declared dynamic failure; it may not
collapse owner failure, requester-observed failure, static Diagnostic, load
refusal, and non-delivery into one `failed` state.

### D3 - Restore, one-shot, linearity, and dependency scope

Decide equality or unique reconstruction only for a successfully loaded
`SaveObject` satisfying the full theory/04 admissibility conditions, not for an
arbitrary cut. At every frontier for which successful load is claimed, the
candidate must retain or uniquely reconstruct request-local M1 context and its
validation grounds; request/pending; owner success availability; requester
receipt and acceptance status; explicit failure and requester-side failure
availability; unconsumed result value separately from redacted history; all
three provenance kinds; held `Gamma`/`Delta`; terminal state; and causal facts
needed for the selected dependency projection.

Restoration respects prefix/channel-state requirements, atomic-cut rollback
boundaries, connected authority provenance, and no stale membership, witness,
lease, capability, or severed-lineage resurrection. Reacquisition is fresh and
explicit. It must not silently substitute validation against post-load current
state for the recorded service outcome.

Within one trace extending that restored configuration, success and failure for
one pending are mutually exclusive terminal branches. At most one matching
success receipt may be accepted and at most one success computation may consume
the pending. A consumed or failed pending cannot become resumable after load.
The candidate identifies the result-supplying fact and the later consuming
occurrence or projection, with explicit or uniquely derivable state dependency
that preserves DAG acyclicity. This does not require a particular direct-edge
granularity or prescribe whether a resume is an occurrence.

These are safety properties, not transport exactly-once, durable exactly-once,
global uniqueness across independent loads, freshness, liveness, or authority
for the later occurrence.

## Required coupling and permitted deferral

The following must be decided together in one candidate:

1. D1 correlation basis with D3 restore scope. A relation needs a semantic locus and restore account; a request anchor needs equality/reconstruction; a nominal identity needs persistence and non-reuse.
2. D2 lifecycle with D3 one-shot/linearity. Duplicate-resume exclusion, terminal failure, and `Delta` accounting require selected lifecycle and post-load status.
3. M1 context with selected outcome. The model identifies the claims checked and authoritative grounds used for that request outcome.
4. Result provenance, accepted receipt, and `DepOf`. Program order, owner seriality, common ancestry, or a shared name cannot prove result use.

Representation may remain deferred only after semantic residence is stated.
Generic `bind`/`let`/ANF, futures, and first-class continuations may defer, but
V1's delimited, single-evaluation, non-capturable, non-storable,
non-transmissible, non-reentrant, locus-bound, non-duplicating `Delta`
boundary may not. Retry, timeout, cancellation, fairness, delivery guarantees,
migration, durable exactly-once, successful-write acknowledgement, C1
snapshot/read-modify-write atomicity, later-operation service semantics, and
exact requester failure syntax may defer. Failure availability and pending/
context disposition may not.

SW1/conditional-A2 detail may defer only under the parametricity condition
above. Exact later revalidation timing may defer only when the candidate makes
no freshness or authority claim for a received result. Proof discharge,
Lean/OBL/Gate/Phase movement, implementation, and I1 readiness remain later.

## Candidate evaluation and adverse cases

Every proposed candidate, including an A/B/C-like or hybrid candidate, must
state a result for each case. A failed row rejects that instantiation under its
stated scope; it does not reject an entire comparison view without a broader
source-backed argument.

| Case | Required result |
| --- | --- |
| two requests share payload and M1 claims | distinct request/pending/correlation instances |
| two active principals share one requester locus | neither authority nor correlation collapses to locus or transport identity |
| duplicate or late success reply | no second accepted receipt, transition, or resume after consumed/failed status |
| wrong-locus receipt | cannot consume the requester pending |
| copied/replayed claims | authoritative membership, lineage, witness, admission, visibility, and history validation decides; claims confer no authority |
| leave/rejoin, revocation, wrong target, missing witness, or severed lineage | fail closed, no owner mutation, no matching success continuation |
| owner-service failure | no accepted success receipt, resume, or later dependency from the failed pending/result |
| successful load at every frontier the candidate claims theory/04 admissible | correlation, status, validation grounds, provenance, result availability, and linear disposition remain explicit or uniquely reconstructible; a consumed computation is not rerun |
| owner interleaves before later write | no result freshness, snapshot, fusion, or read-modify-write atomicity follows |
| owner success exists but requester receipt does not | distinction survives admissible load and requester computation cannot consume the result |
| accepted receipt is saved before consumption | load preserves exactly one enabled consumption, not zero or two |
| explicit owner failure and success facts compete for one pending | selected validity rules admit at most one terminal branch and state disposition of the invalid fact |
| same source site executes twice | distinct dynamic request/pending instances; span remains shared diagnostic provenance only |
| owner-local read | no remote lifecycle is introduced |
| fallback-selected target, if in scope | degradation does not rewind; lineage is retained and reacquisition is fresh |

For the restore row, an A-like candidate must retain/reconstruct direct
relations and status, a B-like candidate must retain/reconstruct anchor
equality plus staged projections, and a C-like candidate must retain/reconstruct
nominal identity plus lifecycle. These are comparison views, not exhaustive
requirements. Exact storage is a later decision.

## Future ergonomic projection boundary

The user-facing goal is to avoid forcing authors to spell merely administrative facts when a selected model can create them unambiguously. It is not a license to infer semantic identity, authority, provenance, or restoration from source resemblance. This boundary is downstream of D1--D3 and P008; it creates no current grammar or inference rule.

A later convenience package must separately state its admitted domain, well-scopedness predicate, result/Diagnostic abstraction, output equality, outcome-existence obligation, determinism obligation, and explicit-presentation correspondence. Outcome existence and determinism remain distinct: a no-outcome input is not vacuously deterministic. The static property must not be confused with runtime schedule determinism, eventual delivery, or equality of dynamic fresh identities.

For every admitted source input, elaboration must yield one equivalent checked artifact or one equivalent Diagnostic. A successful artifact must preserve all selected Core/`G_e`/`C`/`O` facts, spans on generated Core items and edges, effect and failure rows, authority obligations, pending/correlation/provenance/redaction projections, dependency, one-shot conditions, and admissible-load properties of the explicit model. It may not add a hidden edge, callback, future completion, runtime side table, or transport lookup. If reply/receipt needs a generated-edge kind beyond the current schema, the later Canon proposal must either derive it from the selected request semantics or amend that schema explicitly.

Any generated discriminator must be required by the selected model, never recovered from payload, claims, principal, locus, source span, session, queue position, transport metadata, or another incidental value. Source span remains diagnostic/provenance information, not identity; two dynamic executions of one site cannot collapse. Static ambiguity or inability to establish the admitted administrative relation remains a Diagnostic, not runtime `Reject`; dynamic receipt rejection is a declared failure only if the candidate selects it and contains it in the failure row.

The runtime condition is preservation of the selected permitted trace set and its observable success/failure, receipt/acceptance, provenance/redaction, linear-resource, cut, and dependency distinctions. The deterministic conformance schedule may test a selected scenario but cannot substitute for that relation. When unique elaboration cannot be established in the admitted finite fragment, the distinction remains explicit or the result is a Diagnostic. A residual obligation cannot become a hidden carrier of identity, authority, or pending state.

## Recommended normal-process next step

Prepare one ordinary Canon proposal whose candidate semantics answer D1--D3
together and are checked against the adverse table. Treat A/B/C only as
non-exhaustive comparison views; do not require a prior failure of another
family to propose a candidate. The proposal must state what is definitional,
dynamically produced, or derived; map each fact to current Core, generated
edges, obligations, history, runtime configuration, channel state, or
admissible load; and request an explicit Canon carrier decision if the current
strata cannot represent a required fact.

Before a shared proof model, ergonomic elaboration proposal, or I1-readiness
claim relies on the candidate, it must state its relation to exact theory/01
through theory/05, conditionally theory/06, spec/04, spec/05, P008, P012,
P013, OPEN-010/011, and the current source hierarchy at its final authority
cut. This packet does not make that proposal or choose its outcome.

## Explicit unresolved assumptions and non-effects

UNRESOLVED: the three-bundle grouping and A/B/C views are LAB synthesis; no
Canon source calls them an exhaustive taxonomy or canonical decomposition.
UNRESOLVED: "obvious" has no current formal criterion; model-relative outcome
existence, determinism, and trace preservation are proposed later tests.
UNRESOLVED: no post-load occurrence equality, semantic locus for an A-like
relation, exact reply/receipt/failure occurrence carrier, or demonstrated need
for a C-like identity is currently supplied.

This packet changes no Canon text, working record, Core, authority rule, Config/history/SaveObject model, theorem/OBL ledger, source syntax, inference rule, scenario, Gate/Phase, runtime, API, sample, or public claim. It creates no new ADR-0014 research record, theorem, helper, schema, CI target, or implementation lane.
