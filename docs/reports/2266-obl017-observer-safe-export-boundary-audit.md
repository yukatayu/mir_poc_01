# Report 2266 - THM-005 / OBL-017 observer-safe export boundary audit

## Objective

Determine whether the THM-005 / OBL-017 source cut derives a complete
proof-facing Lean interpretation of observer-safe noninterference.

## Scope and assumptions

Canon remains normative. The disposable two-configuration Lean model is LAB
evidence about an under-specified statement boundary, not a counterexample to
the canonical observation policy, an ObservationEvent implementation, or a
MirCore runtime configuration.

## Start state / dirty state

The worktree was clean at `6073d7f6`. THM-005 and OBL-017/018 remained open in
the canonical ledger. T-RESEARCH-013 began with the Discord task baseline
recorded and created only a disposable Lean artifact under `/tmp`.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/02, theory/07, SCN-07, and OPEN-020 references
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- LAB `plan/156`, `progress.md`, `tasks.md`, and `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Defined the audit row as the complete proof-facing THM-005 / OBL-017
  statement interpretation, excluding THM-005 and OBL-017 themselves as
  premises.
- Compared direct observation policy facts with the missing configuration,
  export, and equality interpretation.
- Built a disposable trusted finite model with one low projection and two
  designated high/raw projections.
- Replaced an initially free Boolean equality toggle with explicit
  constructor-identity and visible-position relations after exact-file Oracle
  review.
- Added a theorem that records low agreement and the designated high/raw
  variation, then recorded the bounded result in the LAB planning views.

## Files changed

- `docs/reports/2266-obl017-observer-safe-export-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-013/ObserverSafeExportCountermodel.lean`.

## Commands run

- focused canon/LAB source searches with `rg` and `sed`
- `lean --trust=0 /tmp/mirrorea-t-research-013/ObserverSafeExportCountermodel.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- temporary Oracle sessions `obl017-observatio-boundary` and
  `obl017-export-equality-review`
- final documentation/source-hierarchy and focused regression commands listed
  below

## Evidence / outputs / test results

- Frozen result: `0 direct / 0 delegated / 1 missing` coupled THM-005
  formalization boundary. This count applies only to the complete
  proof-facing statement interpretation; it does not say that direct policy
  facts are absent.
- Canon directly fixes observer-safe noninterference policy, observation-event
  vocabulary, typed observation, provenance, redaction/retention constraints,
  forbidden observer-safe contents, and selected SCN-07 expectations.
- The source cut does not select the coupled canonical configuration
  low-equivalence/high-variation relation, observer-safe export carrier and
  observer context, or equality/equivalence for event/reference identities,
  order, and multiplicity.
- The same finite model keeps selected LAB-local predicates observer-safe,
  occurrence-derived, session-local, free of an aggregate forbidden payload,
  and equal in visible position. Constructor identity falsifies the finite
  shape, while visible-position equality satisfies it. These side predicates
  are stipulated; they are not derived from an observation pipeline.
- Trusted Lean execution passed. The scan found no `sorry`, `admit`, declared
  axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`. `#print axioms`
  reports no axioms for `common_policy_facts` and Lean `propext` only for the
  other listed theorems. Scratch hash:
  `7e27c733803267fd55cef33212956e116c32d2b158c588836b6361ba6881d4a3`.
- Before the broad checks, the root filesystem had 21 GB free (89% used) and
  the system reported about 10 GB available memory. This package adds no
  tracked build artifact; the Lean experiment remains under `/tmp`.

## What changed in understanding

The observation policy is already direct canon, but it cannot become a
proof-facing THM-005 statement by merely choosing a toy label or a convenient
row comparison. Low-equivalence, export interpretation, and output equality
must be selected as one coupled formal boundary. The existing two-point IFC
example is useful adjacent LAB evidence only; it delegates none of this
boundary.

## Open questions

- Which canonical configuration relation expresses an observer's low view and
  allowed high/raw variation?
- What Lean carrier represents observer-safe rows or export collections,
  including observer context and provenance?
- How are event, proof, and reason identities; ordering; and multiplicity
  treated by observer-safe output equivalence?
- What relationship, if any, should a final label/declassification algebra have
  to the declared redaction chain?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare an owner/canon
formalization decision only when a proof-facing THM-005 / OBL-017 package needs
to select one of the listed interfaces.

## Plan update status

Updated: plan/156 records the audit row, direct policy cut, one coupled missing
boundary, corrected equality twin, adjacent-IFC separation, escalation
threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed observer-safe policy from the
missing proof-facing low-equivalence/export boundary.

## progress.md update status

Updated: current research, the remaining statement-boundary row, and the dated
recent log include T-RESEARCH-013.

## tasks.md update status

Updated: T-RESEARCH-013 is closed as LAB source-adequacy evidence and the next
selection excludes silently choosing its configuration or export interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

The temporary selection review chose the THM-005 / OBL-017 source cut as an
eligible independent audit. The completed temporary exact-file review approved
the classification after requiring: an equality relation connected to its
claimed projection; formal recording of high/raw variation; side predicates
described as stipulated; separation of the adjacent IFC file; and an explicit
definition of the counted audit row. All corrections were applied. The review
did not rerun Lean because its browser environment lacked Lean; local trusted
Lean was rerun after the corrections. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed, conformance, and product checks do not apply to this
documentation and disposable-Lean source audit. The runnable sample corpus was
not rerun because no sample, runner, or implementation source changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The completed temporary Oracle
selection and exact-file reviews were advisory and checked against the canon
source cut.
