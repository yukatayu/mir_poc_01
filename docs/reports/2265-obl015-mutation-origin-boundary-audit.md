# Report 2265 - THM-004 / OBL-015 mutation-origin boundary audit

## Objective

Determine whether the THM-004 source cut derives a complete proof-facing bridge
from every owner-state mutation to an owner-local declared transition or a
specific validating capability use.

## Scope and assumptions

Canon remains normative. The disposable three-event Lean model is a LAB
countermodel of an under-specified formal schema, not a counterexample to the
canonical authority policy or an executable MirCore trace.

## Start state / dirty state

The worktree was clean at `312028c6`. THM-004 and OBL-015/016 remained open in
the canonical ledger.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/04, theory/05, ADR-0005, SCN-03, and SCN-04
- LAB plan/73, plan/156, progress, tasks, project status, and Lean alpha stubs
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Compared the THM-004 statement to the authority and operational source cut.
- Confirmed existing authority-related Lean files are alpha `True` stubs, not
  authority-proof evidence.
- Constructed an experiment-local delegated three-event countermodel and its
  one-bit mutation-association repair twin.
- Obtained temporary Oracle selection and exact-file reviews; applied all
  source-reading corrections.

## Files changed

- `docs/reports/2265-obl015-mutation-origin-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

## Commands run

- focused canon/LAB source searches with `rg` and `sed`
- `df -h .` and `free -h` before broad validation
- `lean --trust=0 /tmp/mirrorea-t-research-012/AuthorityMutationBridgeCountermodel.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- temporary Oracle sessions `next-theory-audit-selection` and
  `obl015-mutation-bridge-review`
- attempted `ask-chatgpt-pro-followup obl015-mutation-bridge-review` after
  applying review corrections

## Evidence / outputs / test results

- Frozen result: `0 direct / 0 delegated / 1 missing` coupled
  mutation-origin/authorization formalization boundary.
- Canon directly establishes that delegated/capability-mediated authority is
  grant-lineage based, with validation coordinates for verdict, principal,
  role, target, epoch, incarnation, required witness, and policy version. It
  also separately allows owner-local mutation under declared transitions.
- The finite model adds the favorable experiment-local order
  `grant < use < mutation`; canon directly fixes only `grant -> use` as a
  causal generator. The grant and use match every modeled coordinate, while
  `MutationUses` is false and the non-owner-local mutation has no authorizing
  semantic association.
- The twin changes only `MutationUses(mutation, use)` and proves the delegated
  proposition for that same finite model. It does not establish a canonical
  interface or the owner-local branch.
- Trusted Lean execution passed. The source scan found no `sorry`, `admit`,
  declared axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`.
  `#print axioms` reports only Lean `propext` for the listed theorems. Scratch
  hash: `e51436512246cc1f0ff32e6ce0b479902089a19f6af4832c8e54a329666121ed`.
- Before broad validation, the root filesystem had 21 GB free and the system
  reported about 10 GB available memory; this package creates no tracked build
  artifact and keeps the Lean model under `/tmp`.

## What changed in understanding

The canonical authority policy is not missing or weakened. What remains absent
from a derivable Lean statement is the coupled formal machinery linking a
mutation to its authorizing use/request/capref or to the owner-local declared
transition alternative, together with the trace/step construction supporting
that link.

## Open questions

- What canonical occurrence/trace relation connects service, use, and mutation?
- How is the owner-local declared-transition branch represented and checked?
- Which validation and lineage record relations belong in a future statement?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare an owner/canon
formalization decision only when a proof-facing OBL-015 statement is required.

## Plan update status

Updated: plan/156 records the direct policy, coupled boundary, favorable-order
model condition, twin evidence, escalation threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed authority policy from the
missing proof-facing mutation-origin formalization.

## progress.md update status

Updated: current research and the dated log include T-RESEARCH-012.

## tasks.md update status

Updated: T-RESEARCH-012 is closed as LAB source-adequacy evidence and the
unselected successor excludes silently choosing the THM-004 interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

The temporary selection review ranked THM-004 first among the proposed bounded
audits. The exact-file review approved the result after requiring three
corrections: mark `use < mutation` as experiment-local; keep the owner-local
alternative distinct from delegated grant-lineage authority; and describe the
twin as sufficient only inside its finite delegated model. All were applied and
checked against the cited canon sources. A continuation re-review could not
start because the browser wrapper had not retained a ChatGPT conversation URL;
the completed exact-file review was not duplicated. Local trusted Lean and
documentation validation were rerun after the corrections. No local sub-agent
service was available.

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
