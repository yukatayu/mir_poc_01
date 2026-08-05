---
id: spec/11-m10-i1plus-conformance
status: L1-fixed
maturity: draft
depends_on: [spec/06-conformance, spec/08-m7-checked-elaboration, spec/09-m8-deterministic-runtime, spec/10-m9-auth-verification, adr/ADR-0025]
summary: 凍結SCN-01..10をordinary sourceからdirect pathで実現するM10 I1+有限conformance profile。
open_items: []
---

# 11 — M10 I1+ finite conformance

## Profile input and source path

`M10ConformanceProfile` is a versioned typed input:

```text
profile_version + source_revision
+ release_family_id
+ { SCN id, source-unit kind/name/hash }[SCN-01..10]
+ { finite typed carrier id/hash, profile context, policy stamps, action schedule }[SCN-01..10]
+ carrier-correspondence table + expectation predicates
+ profile_hash
```

Each SCN correspondence names one primary ordinary `.mir` source plus finite
named negative source variants. A primary source may be shared across SCNs but
keeps its own bound source identity. SCN-09 additionally has finite named
candidate patch sources. Setup prose is typed profile input; it is neither
source text nor expected output. `SCN-11` and `SCN-12` remain pressure rows and
are not members of this 10/10 profile.

For each source unit, M10 runs exactly one direct path:

```text
ordinary .mir source
→ M6 parse + classify
  ├─ typed diagnostic terminal (M6 negative)
  └─ M7 check + elaborate
       ├─ typed diagnostic terminal (M7 negative)
       └─ retained CheckedSurfaceV0 identity
            → direct M8 DeferredToM9
            → source-bound M9 resolution
            → crate-private lossless M9→M8 authority inventory
            → deterministic M8 runtime
            → observer-safe typed projection
```

An M6/M7-negative source unit terminates at its required typed diagnostic and
does not fabricate later checked, runtime, trace, or projection artifacts. An
accepted executable unit continues through the displayed M8/M9/runtime path.
Each source unit retains its own identity continuity to that terminal; no
identity is shared or inferred across primary, negative, or candidate variants.

An SCN-09 candidate source is not by itself a checked patch artifact. It is
paired at candidate check/compatibility with its versioned, hash-bound typed
patch-intent carrier. Only the pair's source identity, intent identity,
candidate check result, and compatibility result form the artifact that may
later be submitted as a patch request. Neither patch intent nor its candidate
result is a schedule action or expected result.

The implementation must not select a result from expected JSON, a report,
fixture name, sidecar, or reconstructed evaluator. A waiver carrier is
prohibited and always empty.

## Finite release-carrier correspondence

`release_family_id` groups this finite source bundle but never replaces a
per-source-unit identity. The profile contains a hash-bound correspondence
declaration with one row for every frozen positive or negative expectation.
The verifier appends its outcome and emits the finite correspondence table with
these required columns:

| Column | Required binding |
|---|---|
| SCN id / expectation id | frozen scenario and individual expectation |
| phase | exactly one lower-case literal: `static` or `runtime`; never a C-level name |
| carrier kind | ordinary source, patch source, finite typed carrier, profile context, or schedule action |
| artifact identity | exact source-unit hash or typed-carrier hash; a negative source never names a successful Core identity |
| diagnostic location | exact source span for source diagnostics, or exact typed-carrier field reference for a validated non-Surface carrier |
| source-derived reference | exact `SourceRef` plus checked Core/effect/obligation reference when one exists |
| schedule-action reference | exact typed external action that triggered a runtime transition, if any |
| evidence predicate | general assertion over generated evidence, never a preassembled result |
| result | verifier output: `pass` or `fail`; `N/A` and waiver are prohibited |

The table must establish actual derivation, not mere construct presence. Every
static edge, dependency, effect, and obligation names its source reference,
span, stable checked reference, and semantic kind. Missing, mismatched, or
unrepresented correspondence fails the claimed C-level.

`C-static` and `C-runtime` are aggregate conformance results, not row-phase
values. `C-static` passes only when every row whose phase is `static` passes.
`C-runtime` is claimable only when `C-static` passes and every row whose phase
is `runtime` passes. A runtime row therefore never substitutes for a missing
static row.

## Frozen SCN-01..10 expectation IDs

This is the closed finite inventory for the M10 correspondence declaration.
Each listed ID has exactly one row with the shown phase; evidence references
may be shared, but rows and predicate results may not. The IDs are finite
release-profile vocabulary, not Surface syntax, public diagnostic/API names, or
an extension of the frozen scenario set.

| Frozen SCN | `static` expectation IDs | `runtime` expectation IDs |
|---|---|---|
| SCN-01 | `SCN01-S-P-REQ`, `SCN01-S-P-DEP`, `SCN01-S-P-PUB`, `SCN01-S-P-SPANS`, `SCN01-S-P-CAP`, `SCN01-S-N-VISROW` | `SCN01-R-P-STATE`, `SCN01-R-P-ORDER` |
| SCN-02 | `SCN02-S-P-REQ-RMW`, `SCN02-S-P-DEPS`, `SCN02-S-P-FAIL-SPAN`, `SCN02-S-P-LOCUS`, `SCN02-S-N-CAPROW`, `SCN02-S-N-REQUESTER-READ`, `SCN02-S-N-BLIND-WRITE`, `SCN02-S-N-NO-XOWNER-TXN` | `SCN02-R-P-ONE`, `SCN02-R-P-TWO`, `SCN02-R-N-NOCAP`, `SCN02-R-N-STALE` |
| SCN-03 | `SCN03-S-N-PREVERDICT` | `SCN03-R-P-ADMIT`, `SCN03-R-P-LINEAGE`, `SCN03-R-P-PAST`, `SCN03-R-N-PREVERDICT`, `SCN03-R-N-ROLE-SPOOF`, `SCN03-R-N-CAPREPLAY` |
| SCN-04 | — | `SCN04-R-P-STALE`, `SCN04-R-P-AUDIT`, `SCN04-R-P-BLOCK-COMPACT`, `SCN04-R-P-ALLOW-COMPACT`, `SCN04-R-P-REJOIN`, `SCN04-R-N-HIDDEN-REPAIR` |
| SCN-05 | `SCN05-S-N-MISSING-VISROW` | `SCN05-R-P-HANDOFF`, `SCN05-R-P-OBS`, `SCN05-R-N-SECRET`, `SCN05-R-N-WRONGCAP` |
| SCN-06 | `SCN06-S-P-REQFAIL`, `SCN06-S-N-ROW` | `SCN06-R-P-ABSENT`, `SCN06-R-P-PATCHED`, `SCN06-R-N-NOHANG` |
| SCN-07 | `SCN07-S-N-PRIVATEPOL`, `SCN07-S-N-WIDEN` | `SCN07-R-P-FIELDS`, `SCN07-R-P-ADMIN`, `SCN07-R-P-POLICY`, `SCN07-R-N-HORIGIN` |
| SCN-08 | `SCN08-S-P-CARRIER`, `SCN08-S-N-LINEAGE`, `SCN08-S-N-CAPFLOOR` | `SCN08-R-P-LIVE`, `SCN08-R-P-EXPIRE`, `SCN08-R-P-WRITE`, `SCN08-R-P-REACQUIRE`, `SCN08-R-P-ROLLBACK`, `SCN08-R-N-REPROMOTE` |
| SCN-09 | `SCN09-S-P-CHECKEDPAIR`, `SCN09-S-N-SELFGRANT`, `SCN09-S-N-MISSINGCAP` | `SCN09-R-P-PIPELINE`, `SCN09-R-P-INIT`, `SCN09-R-P-OBS`, `SCN09-R-N-DRIFT` |
| SCN-10 | — | `SCN10-R-P-S1`, `SCN10-R-P-S2`, `SCN10-R-P-LOADFRESH`, `SCN10-R-N-MERGE`, `SCN10-R-N-LEASEDOCTOR`, `SCN10-R-N-CUTDOCTOR`, `SCN10-R-P-TIMELINE`, `SCN10-R-P-REACQUIRE` |

The SCN-10 C-distributed Z-cycle extension has no M10 expectation ID and no
`N/A` row: it is outside this finite single-process profile. SCN-11/12 remain
pressure rows and likewise have no row in this inventory.

The selected carrier families are:

| Frozen scope | Required carrier family |
|---|---|
| SCN-01/02 | primary ordinary source, distinct ordinary negative source variants, and source-derived owner action/visibility/failure rows |
| SCN-03/04 | primary ordinary source plus sealed M9 envelope and typed admission/leave/lifecycle requests or context |
| SCN-05 | primary ordinary source plus the source-bound observation-policy carrier below, sealed M9 envelope, and typed admission/leave/lifecycle requests |
| SCN-06 | primary ordinary source and negative variants, with typed topology context and source-derived failure rows |
| SCN-07 | primary ordinary source plus the source-bound observation-policy carrier below, its typed policy-negative variant, and typed observation context |
| SCN-08 | the explicit finite typed three-option fallback carrier below, its typed negative variants, and typed clock/lease/reacquire actions |
| SCN-09 | separately parsed, classified, and checked ordinary base source and every candidate patch source, followed by a typed submission of the checked patch artifact |
| SCN-10 | source family and typed save/release/restore/merge requests, with the relevant SCN-08 fallback carrier identity retained |

This carrier rule is a finite M10 conformance boundary. It does not make a
schedule a source-language substitute, turn carrier role names into public
APIs, or silently change an expectation that lacks the required row.

## External schedules, provenance, and verification

An action schedule is limited to exogenous typed requests or deterministic
context: source-derived handler invocation with parameters; admission or leave
request; clock advance or lease expiry; topology/route context; observation
request; submission of an already checked patch artifact; save, release,
restore, or merge request; and deliberately corrupted external object for a
negative test. It may not mint a grant, verification verdict, epoch,
incarnation, capability, witness, store mutation, relation mutation, fallback
position, patch declaration, history row, projection row, or expected result.

Every runtime row has two separate provenance fields. `program_artifact`
identifies the exact checked effect/evaluation that caused a semantic
occurrence. `schedule_action` identifies the typed external request/context
that triggered it. An external control row has only `schedule_action`; it must
not fabricate a Core or source origin. Projection rows additionally identify
their subject history occurrence and preserve monotone redaction without raw
capability, witness, authorization, or verification payload.

Evidence generation and expectation verification are distinct logical stages:

```text
evidence generator: source bundle + finite typed carriers + typed context + action schedule
                 → actual artifacts, traces, states, cuts, projections
conformance verifier: actual evidence + correspondence table + predicates
                 → C-static / C-runtime pass or fail
```

The generator must not read the expectation predicates. Altering a predicate
without changing sources, context, or schedule may change verification only;
it must not change generated evidence.

## Non-Surface candidate and observation carriers

### SCN-09 patch-intent carrier

`PatchIntentCarrier` is a versioned, hash-bound input to candidate
check/compatibility, paired with exactly one candidate ordinary source. It
contains only the finite patch facts already required by theory/08: proposed
surfaces/state additions, required capabilities, effect/failure and
observation/redaction/retention rows, membership/frontier witness references,
and explicit authority/write intents. It cannot execute a patch, mutate
configuration, append a lifecycle row, or supply a verdict.

Candidate check derives a checked patch artifact from the pair. An authority
intent that grants the candidate itself is `E-PATCH-003` at the carrier's
`authority_intent` field reference. A write intent without its declared
capability is `E-PATCH-002` at the carrier's `write_intent` or
`required_capability` field reference. Both have the candidate source and
patch-intent identities, no activated composite identity, and the frozen
before/after no-mutation predicate. The correspondence records these as
candidate-check/compatibility diagnostics, not M6/M7 source diagnostics and
not schedule-supplied rejected verdicts.

For an accepted SCN-09 candidate, the same paired artifact records its checked
base/candidate provenance, compatibility, admission frontier, verdict, and
activation cut. A schedule may submit that already checked artifact only. This
does not add M6 patch/grant grammar or a public patch-intent API.

### SCN-05/07 observation-policy carrier

`ObservationPolicyCarrier` is a versioned, hash-bound typed validator input
that pairs an exact ordinary source identity and source field reference with a
frozen observation-policy constraint. It may restrict or verify source-bound
observation; it cannot widen a source declaration, make private data public,
or serve as hidden profile-only visibility metadata. Its validation happens
before an observation request is scheduled, and its result is carried in the
correspondence table.

For SCN-05, the carrier binds the source-declared private `secret_key` field,
the cross-locus observation request class, and the required private-field
denial. The request then yields `VisibilityDenied` with no publish/state
mutation. If its required failure declaration is absent, the typed carrier
validator derives the frozen `E-ROW-002` correspondence at its
`required_failure` field reference, retaining the related source field span as
provenance. This is not a claim that the current M6/M7 checker recognizes a
cross-locus private-field read.

For SCN-07, the carrier binds the ordinary source's
`inventory_note` observer-safe field reference to its `private_like` policy
constraint and redaction/retention order. Their conflict derives the frozen
`E-VIS-002` correspondence at the carrier's `field_policy` reference, with the
source declaration span retained. No observer publication is produced. The
positive gradient rows require the same carrier to preserve the frozen
redaction order and suppress raw authority/witness/verification payloads.
These are explicit finite release-carrier changes, not absent M6/M7 diagnostics
laundered through a runtime schedule or a public observation grammar.

## One M6/M7 direct-consumer seam

The M10 direct consumer uses the bounded M6/M7 declaration
`visible observer_safe fields (FieldName {, FieldName})` within a `StateDecl`.
It follows the field declarations and occurs at most once. Unlisted fields are
private. The listed names must be known state fields and unique; unknown or
duplicate names yield typed diagnostics at their declaration span, with no
checked Core. A write to a listed field alone emits a
source-bound observer-publish effect and its `VisibilityDenied` failure entry.
A private-field write emits neither; attempted observer publication of it
rejects without semantic mutation.

Within the same and only M10 M6/M7 seam reopen, `Role[self] at L_actor` is the
authority origin and nested `at L_owner` is the owner evaluation/request site.
`L_actor != L_owner` is accepted, does not mint authority, and must not select
a rejection path. The target state owner must equal `L_owner`; same-owner RHS
reads resolve at `L_owner`, not at `L_actor`. The generated owner failure row
still includes `RouteUnavailable`.

Metadata-only visibility is not an alternate implementation route because it
would make observation policy hidden profile state rather than source-bound
checked meaning. This seam is not a final grammar, diagnostic catalog, or
public API commitment.

## M9-to-M8 authority inventory

Direct M8 treatment of `AuthDeferred` and `VerifyDeferred` remains
`DeferredToM9`. M9 resolution may issue active membership, capability, and
witness records only through its source-bound authority route. The provisional
crate-private bridge losslessly translates exactly those issued records into
the typed M8 authority inventory. It does not expose/accept provider proof,
reparse source, rewrite an M7 residual, or attach a `ContractUpdate`
automatically.

The sealed M10 M9 resolution envelope identifies the exact
`CheckedSurfaceV0` identity, embedded M8 identity, and every residual-bearing
row by residual kind, canonical `SourceRef`, and named target. It either
provides the complete declared discharge/admission evidence for that exact
artifact or leaves it residual/deferred; no selected runtime can consume a
partial envelope. A missing, mismatched, or hand-built artifact fails before
authority inventory construction. M9 adds envelope evidence but does not
mutate, replace, or summarize away the retained M7 evidence.

## SCN-08 finite typed fallback carrier

SCN-08's full fallback expectation is intentionally outside the bounded M6
Surface. Its required carrier is therefore a versioned, hash-bound finite typed
profile carrier, not a schedule-only operation and not a public API. The
carrier records exactly three ordered options:

```text
live   : live_pose,    Read, avatar_session
anchor : room_anchor,  Read, room_epoch, same-lineage edge from live
frozen : default_pose, Read, static,     same-lineage edge from anchor
```

It also retains the frozen predicates: semantic expiry advances `live → anchor
→ frozen` monotonically on one lineage; same-lineage re-promotion is rejected;
a request after expiry with no later write-capable option is rejected; explicit
reacquire with fresh witness and epoch creates a new lineage; and `try`/
rollback does not rewind the selected option. A typed carrier validator checks
the lineage-edge and capability-floor negatives and records their exact carrier
field reference plus the corresponding frozen `E-DECL-001` or `E-LIN-003`
diagnostic predicate. The correspondence row must identify the retained
safety property; an alias cannot stand for an absent check.

This finite carrier is only the M10 release conformance carrier for SCN-08. It
does not reopen M6 grammar, establish a general chain checker, or make the
carrier encoding/API public.

## Required evidence predicates

A profile pass requires all of the following under the bound profile:

1. each source unit retains its declared identity chain to its required
   terminal: typed diagnostic for a negative unit, and source/checked
   identity/Core/source-map/trace/observer projection for an executable unit;
2. every correspondence-table row is exact and `pass`/`fail`; an absent row,
   false provenance link, `N/A`, or waiver fails the claimed C-level;
3. every negative source unit reports its required diagnostic at its required
   span and preserves semantic state; a rejected source has no successful Core
   identity;
4. each sealed M9 envelope resolves the exact residual-bearing checked
   artifact, or direct M8 stays `DeferredToM9` and execution is not admitted;
5. every semantic runtime/projection row has the required dual provenance;
   every no-mutation failure checks before/after store, membership, grant,
   relation, and configuration hashes, allowing only its named lifecycle rows;
6. every SCN-09 candidate binds both source and patch-intent identities before
   candidate check/compatibility; self-grant and capability-less-write intents
   derive `E-PATCH-003` and `E-PATCH-002` at their typed carrier field
   references, while an accepted patch identifies base and checked patch
   artifacts, compatibility result, admission frontier, verdict, and
   activation cut; a rejected patch has no activated composite identity;
7. SCN-08 evidence retains the complete three-option carrier semantics and
   the no-repromotion-after-rollback control; and
8. SCN-05/07 typed observation-policy correspondence retains its source
   reference, diagnostic field reference where negative, privacy/redaction
   condition, and no-publication/no-mutation consequence; and
9. replay under the same frozen source set, typed carriers, context, policy
   stamps, and action schedule is deterministic, while a fresh checkout
   reproduces every bound hash, predicate, and direct-path result.

The phase lifecycle is separate. An acceptance record must additionally name
the accepted profile hash, source revision, validation cut, independent review,
non-claims, and accepting authority. A profile pass alone is not a release,
phase exit, or public completion.

## Assurance and boundary

Proof status remains exclusively in theory/11. M10 adds no general proof and
does not promote finite Lean, bounded model, or runtime evidence beyond its
recorded scope. The release inventory recompiles the existing M3--M9 Lean
carriers and reports actual Lean kernel dependencies, including `propext` and
`Quot.sound` where present; it must not call them axiom-free.

This profile does not claim C-distributed, sockets, transport delivery, public
grammar/API/ABI/wire, deployment, product completion, I2+, or official behavior
outside the frozen SCN-01..10 direct path.
