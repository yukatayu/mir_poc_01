# Plan 244: P017 X1 K0 H_K-rs Single-Block Premise/Falsifier Design

## Role and authority

This LAB design gate follows the committed, pushed, source-free WRK-0045
pre-registration. `mirrorea_canon/` remains normative. Its result,
**`SOURCE-DESIGN-ADMIT`**, authorizes materializing only the one already
declared Markdown-held Lean source after this plan is committed and pushed.
It does not establish a candidate model, joint Canon satisfiability, a
semantic receipt transition, operational reachability of `r`, a theorem/OBL
result, or implementation readiness.

The source remains one `existing-lane-experiment` under the immutable
WRK-0045 question. It may state explicit candidate-local hypotheses and prove
conditional logical consequences. It must not define a Mir relation schema,
occurrence kind, Core/Config/SaveObject field, transition, failure row,
matching identity/key, source form, runtime, transport, observation surface,
or public interface. `DEFER` remains the sole in-scope failure result; B-Pi is
not a fallback.

## Input boundary and result ceiling

DIRECT constraints are ADR-0014; P012 V1/R1; P013 M1; P017 X1; theory/01,
02, 04, 05, and 07; and the committed WRK-0045 record. Plans 229, 241--243,
the linked WRK-0044 evidence, and the temporary Oracle review
`wrk0045-premise-design` are LAB evidence only. The review is advisory and was
checked against the listed sources before this plan was written.

The independent delta remains exactly the one registered by WRK-0045:

1. direct candidate `send -> receive` roles over `(q, s)`;
2. a co-located reply-send projection on `s`, without a second `s`-internal
   occurrence;
3. a distinct extensional requester-side candidate occurrence `r`;
4. direct candidate `send -> receive` roles over `(s, r)`;
5. exact typed result/reply/receipt association with functional matching; and
6. a middle channel closure plus an r-sensitive post-receipt cut/restore
   account.

The source is duplicate if it only restates WRK-0044's five static pairs,
proves acyclicity, or derives `q prec r` without consuming this whole delta.

## Design decision

`SOURCE-DESIGN-ADMIT` is conditional on the following source shape. It is not
an outcome and does not change the WRK-0045 pre-registration.

### Retained source: generic conditional layer

The sole retained Lean block must use only opaque carrier parameters and
Prop-valued predicates. It may quantify separate current/restored carriers,
occurrences, branches, bindings, values, uses, loci, cuts, M1 claims, and
grounds. It may state no function returning any of those values. The candidate
relation is an extensional q-scoped predicate, not a fiber object.

The generic layer may use only:

- a namespace, sections, `set_option autoImplicit false`, variables, theorem
  or example declarations, and elementary proof terms/tactics;
- candidate predicates for direct generator roles/order, pending, owner
  outcome, result/reply/receipt, acceptance/use, consulted provenance,
  authority predecessors, cuts/in-flight closure, and correspondence; and
- explicitly supplied C-level order laws and atomized `H_K` premises.

It must not declare a `structure`, `class`, `instance`, `inductive`,
data-returning `def`, `opaque`, `axiom`, constructor, enum, role index, state
tag, map/table/list history, lookup, key carrier, matching function, restore
function, source import, or global finite carrier. It must not use
`DecidableEq`, `BEq`, `Hashable`, `Classical`, `Choice`, `Quotient`,
`noncomputable`, `Subsingleton`, `Unique`, broad proof search, or a mechanism
that chooses a receipt/branch/result/binding from an existential.

Equality is permitted only as the conclusion of an explicit functionality law
or as strict distinctness derived from the candidate strict order. It may not
be used as a matching key, a cross-load identity, or a reconstruction rule.

### Disposable harnesses: logical independence only

The retained source must not contain a finite role index or a positive model.
The execution package may create disposable, untracked Lean harnesses to test
logical inhabitation and adverse counterinterpretations. Such harnesses are
metalogical tests, not candidate semantics: their finite atoms are never
retained, never exported, never used as a matching mechanism in the generic
block, and never reported as a Mir occurrence carrier or relation schema.

This refinement preserves two constraints simultaneously: WRK-0045's explicit
`H_nonvacuity` ledger remains visible in the generic conditional source, while
the evidence package must additionally demonstrate that the atomized premises
are jointly inhabitable without relying on an empty domain, contradiction, or
hidden identifier. No joint Canon satisfiability is claimed.

## Atomized premise ledger and required consumers

Each semicolon-separated item below must be a separately named Lean premise
or a separately audited source-shape condition. A conjunction-valued
`H_pending`, `H_match`, `H_restore`, or aggregate `Coherent` premise is
forbidden. A premise is load-bearing only if its named consumer and its
one-item adverse harness both exist.

| Family | Atomic premises that remain separate | Required derived consumer | Adverse harness when absent |
| --- | --- | --- | --- |
| A-Sigma residence | q/branch-indexed pending, outcome, receipt, acceptance, use; no fiber value | exact integrated witness over the same q/branch | move one fact to a different q/branch |
| Pending | existence; branch-to-binding uniqueness; binding non-sharing; requester locus; held Gamma; held Delta | unique named pending plus no shared binding and exact context witness | two bindings, one shared binding, wrong locus, missing Gamma, missing Delta |
| M1/provenance | q-associated claim; consulted ground; result-ground; live authority distinct | result traceability through the same ground without authority collapse | M1/provenance true while authority false; mismatched ground |
| Owner branches | outstanding; success; failure; exclusivity; row containment; no mutation | failure excludes success, receipt, acceptance, and use | simultaneous terminals, out-of-row failure, failure plus mutation |
| First causal leg | request-send at q; service-receive at s; direct mapping q/s | `q prec s` only via the named direct generator | order without the direct mapping |
| Second causal leg | reply-send on the same s; requester-receive at r; direct mapping s/r | `s prec r`, then `q prec r`, with r as the candidate endpoint | missing projection/r/mapping; order without mapping |
| Order laws | direct-generator inclusion; transitivity; irreflexivity | all strict pair distinctions and post-r prefix closure | missing inclusion/transitivity/irreflexivity respectively |
| Result/receipt | owner value; reply result; receipt relation; existing type fact; result provenance | one exact result threaded through service, reply, and receipt | mismatched result/type/ground |
| Matching | receipt-to-request; receipt-to-branch; receipt-to-result; branch-to-accepted-receipt functionality | explicit uniqueness only by the stated laws | two requests/branches/results/accepted receipts respectively |
| Acceptance/use | acceptance; enabling; consumed witness; consumption needs acceptance; use uniqueness; accepted-unconsumed witness | at-most-one use without forcing use from acceptance | absent enabling, consumed-without-acceptance, two uses, no accepted-unconsumed case |
| Failure prerequisites | receipt needs success; acceptance needs receipt; consumption needs acceptance | failed branch has no downstream success chain | one downstream fact after removal of each prerequisite |
| Authority order | separate capability, witness, auth-evidence, membership mappings; visibility condition | four separately named predecessors of service | remove one mapping or deny visibility |
| K0 | inhabited raw rejection; no semantic occurrence/failure/receipt/use/restore effect | raw rejection stays outside the semantic exchange | add exactly one prohibited semantic effect |
| Cuts | middle q/s membership; middle r absence; in-flight closure; post-r membership; prefix closure | middle closure and r-sensitive predecessor closure | no in-flight closure; r without q/s |
| Restore | five case correspondences; preserve pending/failure/result/receipt/acceptance/use/ground/channel facts | named preservation at every required frontier | loss, merge, duplication, reset, revalidation, stale-authority mutants |
| Non-vacuity | explicit independent emitted, failure, receipt-pending, accepted-unconsumed, consumed, raw-rejection, and restore witnesses | generic hypotheses have a disposable jointly inhabited interpretation | empty, contradictory, or hidden-index interpretation |

The q/s/r direct path itself is not a sufficient consumer. At least one
retained theorem must use `H_r`, `H_sr2`, `H_result-send`, `H_receipt`, a
matching law, and r-sensitive closure in one receipt-to-cut/restore result.

## Required conditional conclusions

The generic block must retain small conclusions rather than a single theorem
that repackages the premise list.

1. **Causal path:** named direct role mappings, their existing-family order
   inclusion, transitivity, and irreflexivity derive `q prec s`, `s prec r`,
   `q prec r`, and the three strict distinctions. It must not infer a direct
   mapping from `prec`.
2. **Pending/accounting:** separate uniqueness and non-sharing laws derive one
   named pending binding and prevent a shared binding. Neither pending nor
   locus may match a receipt.
3. **Typed receipt:** independent success, reply-result, receipt, type, and
   provenance facts plus matching laws derive an exact candidate-local result
   association. The result is not a new type/carrier or identifier.
4. **Failure exclusion:** terminal exclusivity and each downstream prerequisite
   derive the absence of success/receipt/accept/use from the named owner
   failure. This does not create a requester failure transition.
5. **Restricted use:** acceptance enables at most one named use, while an
   accepted-unconsumed witness prevents the source from treating acceptance as
   mandatory consumption. Consumption remains outside the occurrence carrier.
6. **Authority separation:** four independent predecessor facts are mapped to
   service order; M1, relation residence, q anchoring, locus, and provenance
   never imply authority.
7. **r-sensitive cut/restore:** a post-r prefix contains s and q; a middle cut
   has an explicitly assumed abstract in-flight closure; named current/restored
   correspondences preserve each relied-on fact without equality, a restore
   function, or global persistence claim.

Every theorem must list only the C/H premises it consumes. Do not use broad
`include`, automation that hides dependencies, or a supplied aggregate
coherence predicate.

## Execution protocol after source materialization

The later source/evidence commit may contain only the declared
`plan/wrk-0045-p017-x1-k0-hk-rs-asigma-conditional-trace.md` source and its
direct numbered report, plus allowed operational metadata. It must be committed
and pushed before append-only WRK evidence linkage.

The execution report must record all of the following:

1. exact source extraction, one fenced Lean block, `lean --trust=0`, and
   `#print axioms` for every retained declaration;
2. static scans for the preregistered escapes and the additional declaration,
   matching, classical-choice, causal, and restore bans above;
3. a premise-to-theorem map and a one-item adverse harness result for every
   table row; a removed binder causing a parsing or name error is not evidence
   of logical necessity;
4. one disposable positive interpretation of the atomized ledger and the
   listed adversarial interpretations, all kept outside the repository;
5. explicit proof that no harness value becomes a retained role index, key,
   schema, or semantic occurrence carrier; and
6. `git diff --check`, source digest, changed-path allowlist, normal/clean
   documentation checks, and remote-head verification.

## DEFER and freeze conditions

Do not materialize source, or freeze WRK-0045 after a reproducible outcome, if
any atom cannot have both a non-identity consumer and an adverse harness; if a
finite harness needs to be retained as a role/index/key; or if the source needs
a constructor, rule, occurrence kind, fixed tuple/record/family index, common
witness, matching/restore function, receipt carrier/type, failure member/row,
Config/SaveObject placement, observation surface, source form, runtime, or
transport behavior.

Freeze also if the source is an aggregate premise-to-conjunction tautology; if
`H_sproj` needs a second reply occurrence; if `H_r` needs a receive rule or
reachability claim; if `prec` is used to derive a direct generator; if matching
uses incidental facts or choice; if M1/provenance becomes authority; if raw
K0 rejection gains a semantic effect; if owner failure gains a success chain;
or if restore loses, merges, duplicates, resets, revalidates, or revives stale
facts. The remedy is `DEFER`/forward successor or Canon escalation, never
repair by adding a reserved surface.

## Non-effects

This plan creates no Lean source or execution result and changes no Canon text,
working record, relation schema, identity, occurrence kind, Core/Config/
SaveObject, transition, failure row, authority rule, observation policy, source
grammar, runtime, transport, theorem/OBL, scenario, Gate, Phase, sample, or
public behavior.
