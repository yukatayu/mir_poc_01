# Report 2269 - OBL-028 revocation monotonicity boundary audit

## Objective

Determine whether the theory/05 authority lifecycle source cut derives a
complete proof-facing interpretation of OBL-028: revocation remains monotone
unless a new epoch or evidence is issued.

## Scope and assumptions

Canon remains normative. The disposable Lean models are LAB evidence about an
under-specified revocation formalization interface, not a counterexample to
the canonical revocation policy, MirCore `Config`, admission/capability/
witness carriers, load/rollback behavior, or a proof of OBL-028.

## Start state / dirty state

The worktree was clean at `0d186312`. T-RESEARCH-016 recorded its Discord task
baseline before candidate reading and placed its Lean experiment only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/04, theory/05, theory/08, architecture/04, plan/00,
  plan/01, SCN-03, and SCN-04
- LAB `plan/156`, `tasks.md`, `progress.md`, and `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared three independent candidates: OBL-003 Line-1 decidability,
  OBL-019 rejection no-mutation, and OBL-028 revocation monotonicity.
- Selected OBL-028 after local source reading. OBL-003 has no derivation-ready
  finite judgment/grammar interface, while OBL-019 would substantially repeat
  the existing E-PATCH transition/frame gap already isolated in
  T-RESEARCH-006.
- Separated theory/05's fixed lifecycle and revocation policy from the missing
  proof-facing representations of revocation, reissue, state identity, and
  transition/trace relation.
- Built a disposable action kernel whose explicit `reissueEpoch` and
  `reissueEvidence` actions are the only actions that reactivate a revoked
  snapshot. A direct case proof shows revocation remains without either action.
- Built a separate unstructured-label model in which labels named revocation,
  no-reissue, and transition do not semantically constrain each other.
- Attempted a temporary Oracle candidate-selection consult twice. Both runs
  failed before prompt submission in the browser model picker, so no external
  answer was used in the selection.

## Files changed

- `docs/reports/2269-obl028-revocation-monotonicity-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-016/RevocationBoundary.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- temporary Oracle sessions `next-theory-source-cut-016` and
  `next-theory-source-cut-016-2`
- `lean --trust=0 /tmp/mirrorea-t-research-016/RevocationBoundary.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Frozen source-adequacy result: `0 direct / 0 delegated / 1 missing` coupled
  revocation-monotonicity formalization boundary. This is one complete
  proof-facing row; it does not deny the direct canonical policy anchors.
- Theory/05 directly fixes lifecycle epochs/incarnations, retirement and
  tombstoning, non-revival by old grants/witnesses, and monotone revocation
  unless new epoch/evidence is issued. Theory/01 names `M`, `G`, and `W` and
  gives selected admission/serve rule sketches; theory/04 constrains authority
  evidence causally and successful load; SCN-03/04 give concrete stale-capref
  and old-grant rejection expectations. These anchors are not a complete Lean
  relation.
- The source does not define a proof-facing revocation-state carrier, a
  `Revoked` predicate, epoch/evidence issuance events and their identity,
  relation between a prior and later grant/witness/capref, a trace/transition
  relation, or the exception's scope. Those choices are coupled: choosing only
  one would silently determine the others.
- The positive model has experiment-local `AuthoritySnapshot` and four
  actions. Its theorem needs explicit non-equality hypotheses for both
  reissue actions and has no axioms. The unstructured-label theorem also has
  no axioms. It is not a source-compatible reactivation: it demonstrates only
  that labels without the missing semantic link cannot express the theorem.
- Before broad validation, the root filesystem had 21 GB free (89% used) and
  the system reported about 8.9 GB available memory. The package adds no
  tracked build artifact and keeps the Lean model under `/tmp`.
- `make check` passed: source hierarchy found all 704 required paths, document
  validation accepted 1,423 numbered reports, and `cargo check` passed.
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `git diff --check` passed before staging. Scratch hash:
  `d706fc0420812dd9b50ebe84649b11e3b8a698af3c6cb868c8f5920023651422`.

## What changed in understanding

The authority chapter already settles the intended safety direction. The
missing work is not whether reactivation is allowed: it is the joint interface
that distinguishes reissue from an ordinary later state, identifies the same
authority across state changes, and connects that distinction to revocation.
The policy can therefore remain fixed while the proof-facing structure stays
open.

## Open questions

- What canonical carrier and predicate represent revocation and reactivation?
- What occurrence or state relation proves that an epoch/evidence is new?
- Which identity relation connects grants, witnesses, caprefs, memberships,
  and their revocation state across a trace?
- How does OBL-028 compose with stale-capref rejection and load/rollback
  non-resurrection without selecting a runtime or persistence ABI?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-028 proof-facing package must choose
the revocation/reissue carrier and transition interface.

## Plan update status

Updated: plan/156 records candidate selection, direct policy anchors, the
conditional action kernel, unstructured-label boundary, stop threshold, and
non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed revocation policy from the
missing proof-facing revocation/reissue/trace interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-016.

## tasks.md update status

Updated: T-RESEARCH-016 is closed as LAB source-adequacy evidence and the
next source selection excludes silently choosing its revocation or reissue
interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was received. Temporary consultations
`next-theory-source-cut-016` and `next-theory-source-cut-016-2` both failed
before the prompt was submitted because the browser model picker could not
select the requested model despite listing a similarly named option. Per the
retry rule, the second concrete failure ended retries. Local review therefore
compared candidate overlap and source anchors directly, then checked that the
scratch's reissue actions and labels remain explicitly experiment-local. No
local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The two temporary Oracle sessions
did not submit a prompt and produced no advisory result; their concrete
pre-submit failure is retained above rather than being represented as review.
