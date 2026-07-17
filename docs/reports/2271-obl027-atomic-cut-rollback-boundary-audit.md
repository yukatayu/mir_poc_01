# Report 2271 - OBL-027 atomic-cut rollback boundary audit

## Objective

Determine whether the theory/04 atomic-cut source cut derives a complete
proof-facing interpretation of OBL-027: local rollback cannot cross an
`atomic_cut`.

## Scope and assumptions

Canon remains normative. The disposable Lean models are LAB evidence about an
under-specified atomic-cut/rollback formalization interface, not a
counterexample to the canonical cut policy, a canonical occurrence history or
causal order, a rollback or persistence ABI, a proof of OBL-027, or an
implementation decision.

## Start state / dirty state

The worktree was clean at `8a4f30b3`. T-RESEARCH-018 recorded its Discord task
baseline before candidate reading and placed its Lean experiment only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/04, theory/05, theory/06, theory/08, ADR-0004,
  ADR-0007, plan/00, and plan/01
- LAB `plan/156`, `tasks.md`, `progress.md`, and `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared remaining independent source cuts locally. OBL-027 was selected
  because T-RESEARCH-014 explicitly left it separately open, while the
  diagnostic obligations already have detailed LAB statement-shape inventories
  and OBL-019 substantially overlaps the E-PATCH transition/frame gap in
  T-RESEARCH-006.
- Re-read theory/04's local rollback-frontier property, theory/01's `[CUT]` /
  `[E-CUT]` sketches, and the related non-rewind policy in theory/06 and
  ADR-0004. Preserved the fixed statement that `atomic_cut` is neither a
  distributed commit nor a memory fence.
- Built a disposable local-frontier kernel. If a chosen rollback target is at
  or after a chosen cut frontier, the resulting experimental state retains the
  cut frontier.
- Built a separate unstructured-label model in which cut, local-rollback, and
  causal-containment labels do not order a rollback target.
- Did not retry Oracle for this package: the two immediately preceding
  selection tasks each exhausted the one allowed retry on the same pre-submit
  browser model-picker failure, so no advisory result was available.

## Files changed

- `docs/reports/2271-obl027-atomic-cut-rollback-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-018/AtomicCutBoundary.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- `lean --trust=0 /tmp/mirrorea-t-research-018/AtomicCutBoundary.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Frozen source-adequacy result: `0 direct / 0 delegated / 1 missing` coupled
  atomic-cut rollback formalization boundary. This is one complete
  proof-facing row; it does not deny the direct canonical policy anchors.
- Theory/04 directly fixes a local rollback frontier and forbids removing
  occurrences causally before the cut within that locus. Theory/01 names
  `cut(ell)`, the `[CUT]` / `[E-CUT]` readings, and the history component;
  theory/06 and ADR-0004 directly prohibit rollback from rewinding fallback
  degradation. ADR-0007 directly denies a memory-fence interpretation. These
  anchors are not a complete Lean relation.
- The source does not define a proof-facing occurrence identity, causal order,
  locus-membership predicate, cut-to-frontier projection, rollback operation,
  removal/prefix result, or relation between a rollback target and the prior
  local history. The condition "within ell" and the phrase "causally before"
  must be connected in one statement rather than independently guessed.
- The positive model has only experiment-local natural-number frontiers. Its
  theorem assumes the target is at or after the chosen cut and has no axioms.
  The unstructured-label theorem also has no axioms. The latter is not a
  source-compatible rollback; it demonstrates only that labels without an
  order and result relation cannot express the policy.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.0 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,425 numbered reports, and `cargo check` finished successfully.
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `lean --trust=0` reported that both scratch theorems depend on
  no axioms. The forbidden-element scan had no matches and `git diff --check`
  passed.
- Scratch hash:
  `f0f914289a303827753d7b633827d0156c69f0fb691afd89fc920b69ca4f68ce`.

## What changed in understanding

The atomic-cut direction is fixed, including its local scope and its
non-fence meaning. The missing work is the coupled mathematical interface that
states what a local history contains, what rollback removes, and how a cut
frontier bounds that removal. The policy can remain fixed without selecting a
storage, distributed-commit, or persistence design.

## Open questions

- What carrier identifies occurrences, their locus, and their causal order?
- What operation and result relation represent local rollback?
- How is a cut occurrence projected to the rollback frontier for that locus?
- How does a later OBL-027 statement compose with consistent cuts, load
  admissibility, and fallback non-rewind without selecting an ABI?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-027 proof-facing package must choose
the occurrence/causality/rollback interface.

## Plan update status

Updated: plan/156 records candidate selection, direct cut policy anchors, the
conditional frontier kernel, unstructured-label boundary, stop threshold, and
non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed atomic-cut policy from the
missing proof-facing occurrence/causality/rollback interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-018.

## tasks.md update status

Updated: T-RESEARCH-018 is closed as LAB source-adequacy evidence and the
next source selection excludes silently choosing its rollback interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/01, theory/04, theory/06, ADR-0004, ADR-0007,
and the T-RESEARCH-014 non-claims; it verified that the scratch's numeric
frontier is explicitly non-canonical. No local sub-agent service was available.

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
