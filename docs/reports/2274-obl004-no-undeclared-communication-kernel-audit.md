# Report 2274 - OBL-004 no-undeclared-communication kernel audit

## Objective

Determine which composition fact for no-undeclared communication follows from
the theory/03 elaboration contract before THM-001 and a whole-program
elaboration/runtime relation are selected.

## Scope and assumptions

Canon remains normative. The disposable Lean model is LAB evidence about
composition of item-generated edge containment and an intentionally partial
checker counterexample. It is not a canonical Surface grammar, program
composition rule, elaboration relation, generated-edge carrier, declaration
context, runtime communication semantics, transport contract, proof of
THM-001, proof of OBL-004, or implementation decision.

## Start state / dirty state

The worktree was clean at `17816a56`. T-RESEARCH-021 recorded its Discord task
baseline before candidate reading and placed its Lean experiment only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/03, architecture/02, spec/03, ADR-0002, plan/00,
  and plan/01
- LAB `plan/73`, `plan/76`, `plan/156`, `tasks.md`, `progress.md`, and
  `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared remaining independent source cuts locally. OBL-004 was selected as
  a composition-only question: theory/03 directly says every cross-locus
  consequence appears in `G_e` and nothing else generates communication.
  LAB plan/73 and plan/76 explicitly keep the whole-program corollary outside
  the OBL-001 statement inventory; this audit preserves that boundary.
- Re-read the six BND-001 clauses, the unified judgment's `G_e` component,
  the request/publish/observe/witness examples, static cross-locus rules, and
  the fail-closed Core-to-runtime boundary.
- Built a disposable generic composition kernel. If each item-generated edge
  predicate is contained in a declared predicate, their binary sequence has no
  generated edge outside that predicate.
- Built a finite two-edge model: a checker that examines only a generated
  request accepts a program that also generates an undeclared publish edge.
- Did not retry Oracle for this package: the two immediately preceding
  selection tasks each exhausted the one allowed retry on the same pre-submit
  browser model-picker failure, so no advisory result was available.

## Files changed

- `docs/reports/2274-obl004-no-undeclared-communication-kernel-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-021/NoUndeclaredCommunicationKernel.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- `lean --trust=0 /tmp/mirrorea-t-research-021/NoUndeclaredCommunicationKernel.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Source-adequacy result: **one direct conditional composition kernel** and
  **one remaining full-corollary formalization boundary**. Theory/03 directly
  requires each Surface item's cross-locus consequences to be represented in
  `G_e` and forbids other communication generation. From itemwise containment,
  containment composes over the experiment-local binary sequence.
- The positive theorem quantifies over an arbitrary edge carrier, declared-edge
  predicate, and item-generated edge predicates. It assumes containment for
  every item and proves containment for the structural sequence; it selects no
  grammar, `G_e` representation, edge equality, declaration syntax, or
  runtime behavior.
- The negative finite model uses the source-named `request` and `publish`
  categories. Its intentionally partial checker validates only a request; it
  accepts a sequential program containing an undeclared publish. It is not a
  counterexample to theory/03; it shows that a checker/corollary must account
  for every communication-producing category or justify an equivalent closure.
- A full OBL-004 statement remains under-specified and later than the direct
  kernel: canon does not select program composition/elaboration relation,
  `G_e` carrier and equality, declaration-to-edge relation, composition over
  handlers/branches, the THM-001 proof dependency, or the runtime/transport
  relation that rules out communication outside elaborated edges.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.5 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,428 numbered reports, and `cargo check` finished successfully.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `lean --trust=0` reported that both scratch theorems depend on
  no axioms. The forbidden-element scan had no matches and `git diff --check`
  passed.
- Scratch hash:
  `2b30f922cf70141c5ae4af14d939c10f0dca5d08ee982ffc636daceb88a5dfe0`.

## What changed in understanding

The no-hidden-edge principle has a direct compositional core once an item is
already known to expose all of its generated edges. The remaining work is not
the set-containment algebra; it is the bridge from source-program elaboration
and its declared communication surface to that premise. Keeping that bridge
explicit prevents both silent communication and premature selection of a
runtime or transport design.

## Open questions

- What carrier and equality represent `G_e` and declared communication without
  freezing a final Core or JSON ABI?
- What elaboration/program composition relation establishes itemwise
  containment across handlers, conditionals, locus blocks, and auto publish?
- Which generated rows are communication versus dependency evidence, and how
  are request, publish, observe, and witness categories exhausted?
- What relation ensures runtime/transport communication is derived only from
  verdict-approved elaborated edges?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-004 package must choose the
program/elaboration/generated-edge interface.

## Plan update status

Updated: plan/156 records candidate selection, the direct containment kernel,
the partial-checker countermodel, the full-corollary stop threshold, and
non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the direct itemwise containment kernel
from the unselected program/elaboration/generated-edge interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-021.

## tasks.md update status

Updated: T-RESEARCH-021 is closed as direct conditional LAB evidence and the
next source selection excludes silently choosing its corollary interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/01, theory/03, BND-001/BND-004, spec/03, and the
plan/73/plan/76 anti-scope-creep notes. The final scratch was checked with
`#print axioms`; both the direct kernel and partial-checker countermodel have
no axioms. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. No Oracle session was started for
this package because the repeated pre-submit picker failure was already
concrete and unresolved; no advisory review is represented as completed.
