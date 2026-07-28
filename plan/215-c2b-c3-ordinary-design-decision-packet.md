# Plan 215 - C2-B/C3 ordinary design decision packet

## Role and authority

This is a **LAB ordinary-design decision-preparation packet**. The normative source remains `mirrorea_canon/`. It does not select a Family A/B/C carrier, Core constructor, occurrence equality rule, pending state, relation, Config, SaveObject, source form, elaboration rule, runtime, wire/API contract, OBL, Gate, Phase, or public behavior.

The packet reduces the owner-facing decision surface while preserving the distinction between settled bounded directions and unselected semantics. Its three bundles are an organizational synthesis of existing questions, not a new Canon decomposition. A normal Canon proposal and owner decision are still required before a shared model may rely on any bundle.

## Authority cut and inputs

Review cut: `3c4ed56b6f2e63664a75a2e9187305ba5f895523`.

| Input | SHA-256 | Role |
| --- | --- | --- |
| ADR-0014 | `b6ec76541fede095803608024139badf671f9a48a977c62120abfdfb1f8d2323` | autonomous-research and reserved-boundary rule |
| P012 / P013 | `09ea4d6957c320b4d0647806714a1643101c2022b2893ac76ec7de3bf1db73d5` / `4e0ecf7475f20eec85c09d50201d2d2cc29848d480e8382935fe489b43877213` | recorded V1/R1/SW1/conditional-A2 and M1 directions |
| theory/01 / 04 / 05 | `35e2f52bebcf96332d8102e9110446930b7ff807d948737a5859909de34b0f12` / `70bde483330d3745a8694b15cd75f447b6610513ae66cb1ad4ec1faed274a264` / `e06dc5ef0539eb5b87bce71b34d3e8d2ab0638603642e0d9f89581f29d25e6c4` | request/step, cut/load, and authority constraints |
| Plans 208 / 209 / 210 | `84380a9d2f9929f4ffe5d48f4baf74083fb00cc34d30132e43c45f48f1ddef55` / `50dc299076df7844f3dd2fe641bbd65a57269d305743556bda07525c588faefa` / `4cba73fdbb245b16bf9fdd312609401518abaae6c96273923e1bf861e1548ffe` | common obligations, adverse rows, and A/B/C stop lines |
| Plan 214 | `b6a2a3023ad617ae9624797a223699e653170d4d69cc906a6bc59ef241e1e658` | post-WRK-0039 no-successor disposition |

A temporary GPT-5.6 Sol Pro review (SHA-256 `45d11b4325b04d3e4a81f480cfd4e3d29f9cf587a101320c5024ced1d0e1c42f`) independently reviewed this packet shape. It is advisory LAB input only. The decision surface below follows the pinned repository sources and retains all unsupported assumptions as unresolved.

## Fixed direction and comparison boundary

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

## Minimum decision surface

The owner-facing proposal must supply one coherent candidate for all three bundles. It may use a compact object or a derived projection rather than one field per listed fact, but it must state every required projection and meaning.

### D1 - Definitional correlation basis

| Alternative | Minimum content | Stop line |
| --- | --- | --- |
| Family A: relation-first | direct relations have stated locus, functionality, branch meaning, and restore behavior | a relation only in prose, evaluator meta-state, or proof notation is not a solution |
| Family B: request-occurrence anchor | a selected request occurrence has direct non-circular projections to staged facts; equality/reconstruction scope is stated | ancestry, source span, payload, claims, queue position, locus, session, or transport endpoint alone is not correlation |
| Family C: nominal attempt/exchange | a fresh semantic identity has explicit equality, lifecycle, persistence, retirement, and non-reuse rules | no wire/session/queue/envelope identifier may be relabelled as semantic identity |

Family A is the comparison reference. Family B is viable only if it closes each common obligation without hidden identity. Family C remains a reserve comparison until a concrete A or B candidate fails an adverse row. No family is selected or globally rejected by this packet.

### D2 - Branch model and explicit projections

Decide where the semantic configuration locates, or from what selected object it derives, M1 claims, authoritative facts actually checked, owner outcome, pending state, owner reply, requester receipt, owner-service failure, result value, provenance, redaction, source span, and failure information. It must represent `awaiting`, `received-but-not-resumed`, `consumed`, and `failed`, or prove an equivalent projection that distinguishes the same branch conditions.

The model must keep reply distinct from receipt. Only a matching accepted requester-side receipt may enable restricted resume. Failure is an explicit selected branch, not an inference from missing delivery, timeout, queue loss, absence of a reply, or receipt possession. M1 claims remain validation inputs, not authority.

### D3 - Restore, one-shot, linearity, and dependency scope

Decide the equality or unique-reconstruction scope after loading an admissible consistent cut, including how a restored request, pending, reply, receipt, failure, provenance, and terminal status are the relevant semantic facts. Within one admissible restored-prefix extension, the model must state at most one accepted success receipt and one resume for a pending, exact `Gamma` and `Delta` disposition for success and failure, post-resume state preventing a zero-occurrence resume from rerunning after load, and direct dependency evidence from the consumed receipt/result to the later occurrence.

This is not transport exactly-once, durable exactly-once, global uniqueness across independent loads, a freshness promise, or authority for the later occurrence.

## Required coupling and permitted deferral

The following must be decided together in one candidate:

1. D1 correlation basis with D3 restore scope. A relation needs a semantic locus and restore account; a request anchor needs equality/reconstruction; a nominal identity needs persistence and non-reuse.
2. D2 lifecycle with D3 one-shot/linearity. Duplicate-resume exclusion, terminal failure, and `Delta` accounting require selected lifecycle and post-load status.
3. M1 context with selected outcome. The model identifies the claims checked and authoritative grounds used for that request outcome.
4. Result provenance, accepted receipt, and `DepOf`. Program order, owner seriality, common ancestry, or a shared name cannot prove result use.

The following may remain deferred: source grammar; `bind`/`let`/ANF; futures and continuations; concrete Core/Config/history/SaveObject/IR/queue/wire fields; retry, timeout, cancellation, fairness, delivery guarantees, migration, durable exactly-once; successful-write acknowledgement; SW1 and conditional-A2 detail; C1 snapshot or read-modify-write atomicity; later operation service semantics; exact requester failure syntax; exact later revalidation timing when no freshness or authority claim is made; Lean/OBL/Gate/Phase/implementation work.

## Candidate evaluation and adverse cases

Every proposed A/B/C instantiation must state a result for each case. A failed row rejects that instantiation under its stated scope; it does not reject an entire family without a broader source-backed argument.

| Case | Required result |
| --- | --- |
| two requests share payload and M1 claims | distinct request/pending/correlation instances |
| two active principals share one requester locus | neither authority nor correlation collapses to locus or transport identity |
| duplicate or late success reply | no second accepted receipt, transition, or resume after consumed/failed status |
| wrong-locus receipt | cannot consume the requester pending |
| copied/replayed claims | authoritative membership, lineage, witness, admission, visibility, and history validation decides; claims confer no authority |
| leave/rejoin, revocation, wrong target, missing witness, or severed lineage | fail closed, no owner mutation, no matching success continuation |
| owner-service failure | no accepted success receipt, resume, or later dependency from the failed pending/result |
| save/load at every cut frontier | correlation, status, validation grounds, provenance, and linear disposition remain explicit or uniquely reconstructible; resume is not rerun |
| owner interleaves before later write | no result freshness, snapshot, fusion, or read-modify-write atomicity follows |

For the restore row, A must retain/reconstruct direct relations and status, B must retain/reconstruct anchor equality plus staged projections, and C must retain/reconstruct nominal identity plus lifecycle. Exact storage is a later decision.

## Future ergonomic projection boundary

The user-facing goal is to avoid forcing authors to spell merely administrative facts when the selected model can create them unambiguously. It is not a license to infer semantic identity, authority, provenance, or restoration from source resemblance.

After D1--D3 are selected, a source convenience is eligible only if a model-relative elaboration argument shows all of the following:

1. each dynamic evaluation of the delimited form yields exactly one request role and one pending role in the selected model;
2. the elaborated artifact retains, or uniquely reconstructs, the selected correlation basis, M1 context, validation outcome, requester/owner relation, reply/receipt/failure projections, provenance, failure row, held-context disposition, terminal status, and dependency grounds;
3. any generated discriminator is required by the selected model, never recovered from payload, claims, principal, locus, source span, session, queue position, transport metadata, or another incidental value;
4. source span remains diagnostic/provenance information, not identity; two dynamic executions of one syntactic site cannot collapse;
5. receipt stays a semantic role, pure resume stays zero-occurrence, and the later effect stays a separate ordinary occurrence;
6. every admissible save/load frontier preserves the selected reconstruction and one-shot conditions; and
7. failure exposes the failed relation and source span without duplicating or dropping `Delta`.

If uniqueness fails because of multiple outstanding requests, retry, migration, ambiguous restoration, or required revalidation, the administrative distinction must remain explicit until a later selected model proves an equally precise presentation. This is a proposed elaboration proof obligation, not a current source inference rule, grammar, or implementation task.

## Recommended normal-process next step

Prepare one ordinary Canon proposal whose candidate semantics answer D1--D3 together and are checked against the adverse table. Use Family A as common comparison vocabulary, test any Family B candidate against every obligation, and consider Family C only after a documented A/B candidate failure. The proposal must state what is definitional versus derived and must not add a runtime or source feature merely to encode the candidate.

Before a future implementation or shared proof model, that proposal must also name its relation to exact theory/01 request/step rules, theory/04 load admissibility, theory/05 authority lineage, OPEN-010/011, and the current source hierarchy at its final authority cut. This packet does not make that proposal or choose its outcome.

## Explicit unresolved assumptions and non-effects

UNRESOLVED: the three-bundle grouping is a LAB synthesis; no Canon source calls it the canonical decomposition. UNRESOLVED: "obvious" has no current formal criterion; unique trace-preserving elaboration under a selected model is a proposed test. UNRESOLVED: no post-load occurrence equality, semantic locus for Family A relations, or demonstrated need for Family C is currently supplied.

This packet changes no Canon text, working record, Core, authority rule, Config/history/SaveObject model, theorem/OBL ledger, source syntax, inference rule, scenario, Gate/Phase, runtime, API, sample, or public claim. It creates no new ADR-0014 research record, theorem, helper, schema, CI target, or implementation lane.
