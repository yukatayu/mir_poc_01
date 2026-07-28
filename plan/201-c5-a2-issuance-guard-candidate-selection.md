# Plan 201 - C5-PRE conditional-A2 issuance-guard candidate selection

## Role and authority

This is LAB repository memory and a pre-registration preflight. The normative
source is `mirrorea_canon/`. It does not select an admission occurrence,
projection/facet scheme, request identity, grant/witness cardinality, Core
rule, history carrier, OBL, scenario, runtime, or public contract.

`C5-PRE` is not the C5 composite-admission design. It is the narrower question
of whether a current, **ordinary admission** source span literally exposes an
issuance phase that P012 says would stop the conditional-A2 direction. A later
`working/WRK-0032` record must pin the exact authority and LAB input cut before
it may retain any result.

## Inputs and local review

The 2026-07-28 temporary GPT-5.6 Sol Pro portfolio review is advisory only.
Its useful claim was checked against the current repository:

| Item | Local reading | Consequence |
| --- | --- | --- |
| ADR-0014 | literal transcription in existing `plan/` and `docs/reports/` lanes may be L3 when pre-registered, reversible, non-duplicative, and outside reserved surfaces | C5-PRE can be considered; its standing predicate remains for the WRK record |
| P012 A2 | one verdict may be composite only conditionally; a separately failing, observable, or schedulable issuance phase requires an A1 successor | supplies a guard, not current admission semantics |
| theory/01 | ordinary `admitreq`/`verdict`, Config `M/G/W`, and `[E-ADMIT]` say that a verdict updates M and issues grants/witnesses; generic step scheduling does not classify those actions as one or several occurrences | source query may record wording but cannot infer atomicity or occurrence identity |
| theory/04 | causal family names admission request/verdict/activation and grant/witness/membership dependencies | edges do not choose the history mapping of admission effects |
| theory/05 | AdmissionRequest/Verdict, lineage, and lifecycle wording name authority facts but no ordinary-admission issuance transition | source query must not infer facet cardinality or identity |
| spec/05 | request lifecycle and membership behavior are observable requirements; no ordinary-admission issuance scheduler is named | no scheduling conclusion from a missing marker |
| theory/08 | patch admission/HotPlugVerdict is a separate patch subsystem | exclude it from C5-PRE to avoid conflating patch lifecycle with ordinary admission |
| Plan 186 and WRK-0028 | record the broader admission/occurrence gap and P012's bounded direction | they do not retain a current-cut, ordinary-admission, span-by-span audit of P012's separate-issuance guard |

`rg` across current `working/`, `plan/`, and reports found no existing working
record for the P012 A2 separate-failure/observation/scheduling guard. This is a
non-duplication screen, not a global claim about historical repository text.

## Portfolio disposition

| Family | Selection result | Reason and next boundary |
| --- | --- | --- |
| C5-PRE | selected for L3 pre-registration | a source-local literal guard audit can be independently falsified without defining a composite occurrence |
| C5 proper | defer to ordinary Canon design | named facets, their references, rejection residue, and load/rollback lineage require an occurrence/history model |
| C4 SW1 | defer to ordinary Canon design | D9 requires a served-write identity, validation/mutation transition boundary, and request-to-serve binding |
| C3 V1/R1 | defer to ordinary Canon design | D3/D4 require a pending unit, receipt correlation, held linear context, failure/resume transition, and persistence rule |

C3/C4/C5 are not ordered semantic dependencies. In particular, C3 cannot use
M1 validation claims as a receipt correlation, and C4 cannot use an unspecified
admission facet as a request binding. Those shortcuts would collapse the C2
identity boundary or turn non-authoritative claims into authority.

## Proposed C5-PRE pre-registration

### Narrow question

At one pinned current Canon cut, do the following pre-enumerated **ordinary
admission** spans contain literal wording that names a membership/grant/witness
issuance phase distinct from verdict/`[E-ADMIT]` through a distinct rule,
transition, state, issuance-specific failure, queue/scheduling point, or
independent observation point?

1. P012 A2 disposition and its conditional stop sentence, as a direction only.
2. theory/01 Core `admitreq`/`verdict`, Config `M/G/W`, `[JOIN]`, `[E-ADMIT]`,
   and generic scheduling paragraph.
3. theory/04 causal generating family and consistent-cut consequences.
4. theory/05 AdmissionRequest/AdmissionVerdict, post-admission, and lifecycle
   passages.
5. spec/05 request lifecycle and membership passages.

The query may retain each span's literal wording and whether the registered
marker query matches that span. It must not aggregate non-matches into a global
absence claim.

### Alternative and expected falsifier

Alternative: an enumerated normative span explicitly names a distinct ordinary
admission issuance transition/state, its separate failure, its scheduling, or
its independent observation. The retained result then remains a source fact;
it does not select A1. It requires a future ordinary Canon/A1 successor
assessment before any design relies on A2.

Freeze or decline registration if the query needs to decide same/different
occurrence identity, infer atomicity from a shared rule label or zero-or-one
step wording, resolve singular/plural grant/witness cardinality, construct an
operational trace, classify a patch-admission phase as ordinary admission,
define an absence claim beyond the named spans, or touch any Core, grammar,
theory/11, OBL, scenario, Gate/Phase, helper/schema/CI, runtime, wire, or
public surface. Also stop if an identical current-cut span audit is discovered
or a pinned input changes.

### Permitted result and non-effects

The evidence may be an ordinary Markdown matrix with source span, authority
class, literal action wording, registered marker matches, and explicit
non-claim. It may say only that a particular named span has or lacks a queried
literal marker. It cannot say that A2 is atomic, compatible, current, or
refuted; that all issuance is one/several occurrences; that M/G/W are an
atomic transaction; that a rejection leaves no ordinary-admission residue; or
that any source fact may be ergonomically inferred.

## Execution order

1. Commit this selection and synchronize current LAB snapshots.
2. Create `WRK-0032` with the standing predicate, one frozen cut, source and
   LAB digests, alternative, falsifiers, non-effects, rollback trigger, and
   exact registered commands.
3. Commit and push the registration before running its outcome commands.
4. If every falsifier remains absent, retain only the source-local matrix in
   the existing `plan/` lane, then link it forward in a metadata-only commit.
5. Re-open C3/C4/C5 proper only through an ordinary Canon design proposal that
   selects the required reference/correlation/persistence boundary.

## Execution outcome

WRK-0032 was registered at `a6c2981b4b222ab90af68dfb1f58b5ab22800c80`, retained
its matrix at `7737b0348dadf6271beff466f648106ce66487a6`, and linked the exact
artifact digest forward at `339377e9fca7b867142a13bdef0ef6cce1bd9f25`. The
matrix retains P012's conditional guard direction and a non-match for each of
the four named ordinary-admission theory/spec spans. It is not an A1/A2
decision, atomicity proof, global absence claim, occurrence identity, or
implementation authorization. See
`plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md` and
`working/WRK-0032-c5pre-ordinary-admission-issuance-guard.md`.

## Non-claims

This selection does not advance official T0/T1/T2 or I1, promote an L3 result,
change the P012/P013 directions, adopt A1/A2/SW1/V1/R1/M1 semantics, define an
admission protocol, or authorize implementation.
