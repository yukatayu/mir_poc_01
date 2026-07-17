# Report 2264 - THM-002 / OBL-007 trace-formalization boundary audit

## Objective

Determine whether the canonical THM-002 source cut directly derives a complete
Lean statement for monotone fallback selection and explicit fresh reacquisition.

## Scope and assumptions

Canon remains normative. The two finite Lean models are disposable LAB
countermodels of under-specified formal schemas, not counterexamples to the
canonical policy, MirCore definitions, or SCN-08.

## Start state / dirty state

The worktree was clean at `deb904d0`. THM-002 and OBL-007/008 remained open in
the canonical ledger.

## Documents consulted

- `mirrorea_canon/README.md` and `mirrorea_canon/MAP.md`
- Canon theory/01, theory/04, theory/06, theory/11, ADR-0004, and SCN-08
- LAB plan/156, progress, tasks, project status, and Report 2258
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Audited the direct THM-002 statement against the operational source cut.
- Constructed a trace-schema countermodel with a later `2 -> 1` selection.
- Constructed a fresh-lineage countermodel with no reacquire occurrence.
- Obtained one advisory temporary Oracle review and applied its boundary check.

## Files changed

- `docs/reports/2264-obl007-thm002-formalization-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- focused canon and LAB source searches with `rg` and `sed`
- `lean --trust=0 /tmp/mirrorea-t-research-011/TraceMonotonicityCountermodel.lean`
- `lean --trust=0 /tmp/mirrorea-t-research-011/ReacquireOriginCountermodel.lean`
- forbidden-element scan over `/tmp/mirrorea-t-research-011/`
- `sha256sum` over both disposable Lean sources
- temporary Oracle session `obl007-trace-boundary-review`

## Evidence / outputs / test results

- Frozen result: `0 direct / 0 delegated / 1 missing` complete THM-002
  formalization boundary.
- The canon directly fixes the policy: non-decreasing selected index on one
  lineage; earlier re-selection only through explicit reacquire beginning a
  new lineage with fresh witness and epoch.
- The first local schema admits ordered selections `2 -> 1` because it has no
  transition or lineage force.
- The second has `a0 < a1 < a2`, satisfies same-lineage monotonicity for
  `a0/a1`, and makes `a2` a fresh witness/epoch lineage at index `0`, yet has
  no reacquire. It proves the missing origin/reacquire bridge is independent.
- Both trusted Lean commands passed. The source scan found no `sorry`,
  `admit`, declared axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`.
  `#print axioms` reports only Lean `propext` for the second model's listed
  theorems.
- Scratch source hashes: `TraceMonotonicityCountermodel.lean` =
  `8c0cd427c46a0a788a2490018de60cd77c0fe20d10756e5fd0f3bc24a53abf5b`;
  `ReacquireOriginCountermodel.lean` =
  `f64245f7fa19dbe58e93b3bfb6d1409f4e7f92421d0e62190036900d65b56465`.

## What changed in understanding

THM-002 is a settled normative requirement, not an absent semantic intention.
The missing item is the coupled machinery required to express its full meaning
without circularity: trace/admissibility, order, chain identity, selection,
lineage continuity and origin, explicit reacquire, witness/epoch bindings and
freshness, transition binding, other-step framing, and history interpretation.

## Open questions

- Which of those formal objects belongs in a future canonical trace calculus?
- What is the non-circular definition of a well-formed trace and "only via"?
- How should `L.position`, selected options, and history maximum be related?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a narrowly
scoped owner/canon formalization decision only when a proof-facing OBL-007
statement is required.

## Plan update status

Updated: plan/156 records the direct policy, coupled statement boundary, two
falsifiers, decision threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view distinguishes fixed THM-002 policy from its missing
proof-facing trace formalization.

## progress.md update status

Updated: current research and the dated log include T-RESEARCH-011.

## tasks.md update status

Updated: T-RESEARCH-011 is closed as LAB source-adequacy evidence and the
unselected successor now excludes silently choosing the THM-002 interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

The temporary Oracle review agreed that the result must not say the canon lacks
a semantic requirement. It required the source policy to remain explicit and
identified the lineage-origin/reacquire bridge as an independent missing link.
Its advice is advisory and was checked against the cited canon sources. No
local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed, conformance, and product checks do not apply to this
documentation and disposable-Lean source audit. The runnable sample corpus was
not rerun because no sample, runner, or implementation source changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The completed temporary Oracle review
was advisory and checked against the canon source cut.
