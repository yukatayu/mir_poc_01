# Report 2270 - OBL-022 stream read-side boundary audit

## Objective

Determine whether the theory/09 two-layer-time source cut derives a complete
proof-facing interpretation of OBL-022: stream samples cannot influence
discrete state except through declared adapter effects.

## Scope and assumptions

Canon remains normative. The disposable Lean models are LAB evidence about an
under-specified read-side formalization interface, not a counterexample to the
canonical two-layer-time policy, a canonical stream/adapter carrier, a View or
Provider ABI, a proof of OBL-022, or an implementation decision.

## Start state / dirty state

The worktree was clean at `644d110a`. T-RESEARCH-017 recorded its Discord task
baseline before candidate reading and placed its Lean experiment only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/02, theory/04, theory/06, theory/09,
  architecture/02, architecture/03, ADR-0007, plan/00, and plan/01
- Canon SCN-06 and SCN-07
- LAB `plan/156`, `tasks.md`, `progress.md`, and `docs/project-status.md`
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Compared the remaining independent candidates locally. OBL-022 was selected
  because theory/09 and BND-007 directly constrain the two-layer and typed
  adapter boundary, while OBL-027 remains closely coupled to the already
  audited successful-load/cut source family and OBL-023 explicitly has a
  pending formal statement plus open clock model.
- Preserved the source separation: stream samples are not occurrences, do not
  enter `H`, and are not saved per-sample; anchors/frontiers and the declared
  adapter boundary remain discrete/typed concerns.
- Built a disposable transition kernel in which sample-only input changes only
  a sample slot. A direct proof establishes preservation of the selected
  discrete field; declared adapter and discrete occurrence actions are separate
  branches that may change it.
- Built a separate unstructured-label model where sample and no-adapter labels
  coexist with a discrete-change label. It demonstrates that labels alone do
  not supply a read-side frame relation.
- Did not retry Oracle for this package: the two immediately preceding
  selection tasks each exhausted the one allowed retry on the same pre-submit
  browser model-picker failure, so no advisory result was available.

## Files changed

- `docs/reports/2270-obl022-stream-read-side-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-017/StreamReadSideBoundary.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- `lean --trust=0 /tmp/mirrorea-t-research-017/StreamReadSideBoundary.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Frozen source-adequacy result: `0 direct / 0 delegated / 1 missing` coupled
  stream read-side formalization boundary. This is one complete proof-facing
  row; it does not deny the direct canonical policy anchors.
- Theory/09 directly says that samples are not occurrences, never enter `H`,
  are not saved per-sample, and cannot influence discrete state except through
  declared adapter effects. It also fixes anchor/frontier admissibility and
  drop-not-buffer behavior. BND-007 retains typed adapters while denying View
  and Provider semantic ownership; BND-008 keeps observation typed and
  occurrence-derived. These anchors are not a complete Lean relation.
- The source does not define a proof-facing sample carrier, a discrete-state
  projection, declared-adapter-effect evidence and application relation, the
  input/transition/trace relation, or the frame/equality used by "cannot
  influence." It also leaves the clock/latency model open. These choices are
  coupled: choosing only an effect name or a snapshot field would silently
  determine the missing transition semantics.
- The positive model has experiment-local snapshots and three action branches.
  Its sample-only preservation theorem has no axioms. The unstructured-label
  theorem also has no axioms. The latter is not a source-compatible stream
  execution; it demonstrates only that named labels without a frame relation
  cannot express the policy.
- Before broad validation, the root filesystem had 21 GB free (89% used) and
  the system reported about 9.1 GB available memory. The package adds no
  tracked build artifact and keeps the Lean model under `/tmp`.
- `make check` passed: source hierarchy found all 704 required paths, document
  validation accepted 1,424 numbered reports, and `cargo check` passed.
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `git diff --check` passed before staging. Scratch hash:
  `02952957dc9e107056017cbc6f213d5c31ead3b9bdc5d31de54f993e1a7e1396`.

## What changed in understanding

The two-layer policy is strong and concrete at the architectural level: stream
payload does not become discrete history or authority. Its proof-facing gap is
the precise bridge from a sample/adapter input to a discrete transition. This
can remain open without weakening the policy or making the stream a Mir core
primitive.

## Open questions

- What carriers and identities represent samples, anchors, and discrete state?
- What makes an adapter effect declared, and how is its application related to
  a stream input and a discrete transition?
- What frame/equality relation expresses that a sample-only action leaves the
  relevant discrete state unchanged?
- How do frontier admissibility and the open clock/latency model compose with
  a later OBL-022/023 statement without adding an untyped backchannel?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when an OBL-022 proof-facing package must choose
the stream/adapter/discrete-transition interface.

## Plan update status

Updated: plan/156 records candidate selection, direct two-layer policy anchors,
the conditional read-side kernel, unstructured-label boundary, stop threshold,
and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates the fixed two-layer/read-side policy
from the missing proof-facing stream/adapter/transition interface.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-017.

## tasks.md update status

Updated: T-RESEARCH-017 is closed as LAB source-adequacy evidence and the
next source selection excludes silently choosing its stream or adapter
interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/09, the relevant core/type/cut/fallback sources,
and BND-007/BND-008; it verified that the three experiment-local actions do
not claim to implement a canonical adapter or stream carrier. No local
sub-agent service was available.

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
