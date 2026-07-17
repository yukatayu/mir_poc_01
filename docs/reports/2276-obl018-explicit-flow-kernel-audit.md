# Report 2276 - OBL-018 explicit-flow kernel audit

## Objective

Determine whether the observer-safe policy directly supports a minimal
explicit-flow noninterference kernel, without turning that finite fragment into
the complete THM-005 / OBL-017 statement or an OBL-018 proof.

## Scope and assumptions

Canon remains normative. The disposable Lean model represents only one
experiment-local input with low position, high health, raw witness, and raw
authorization fields. Its observer-safe redaction projects the low position.
It is not a MirCore configuration, label lattice, declassification relation,
authority check, occurrence/telemetry pipeline, ObservationEvent/export ABI,
or collection semantics.

## Start state / dirty state

The worktree was clean at `cf66faf5`. T-RESEARCH-023 recorded its Discord task
baseline before candidate reading and placed all Lean experiments only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/02, theory/07, architecture/02, SCN-07, ADR-0011,
  and plan/00--02
- LAB `plan/156`, `tasks.md`, `progress.md`, `docs/project-status.md`,
  `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`, and Report 2266
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Re-read theory/07's typed-observation pipeline, monotone redaction policy,
  observer-safe exclusions, THM-005 wording, and OPEN-020. Re-read SCN-07's
  position-only observer-safe surface and its negative visibility case.
- Kept T-RESEARCH-013's complete statement boundary intact. In particular, no
  two-point LAB lattice or declassification predicate was adopted as canonical
  evidence.
- Built a disposable finite Lean model whose observer-safe projection keeps
  only `lowPosition`. The positive theorem proves equal projected rows from
  equality of that low component.
- Added a separate high-state projection over two inputs with equal low
  position and different high/raw values. It changes under that variation, so
  an export record alone cannot imply the desired noninterference property.
- Replaced an initial `simp_all` proof after `#print axioms` showed that it
  depended on `propext`. The final proof case-splits the two inputs and their
  low-position equality directly, so both recorded theorems are axiom-free.
- Did not retry Oracle: the two immediately preceding selection tasks each
  exhausted the allowed retry on the same pre-submit browser model-picker
  failure. No environmental change makes another temporary consultation useful
  for this bounded source cut.

## Files changed

- `docs/reports/2276-obl018-explicit-flow-kernel-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The final disposable source remains outside the repository at
`/tmp/mirrorea-t-research-023/ExplicitFlowKernel.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- `lean --trust=0 /tmp/mirrorea-t-research-023/ExplicitFlowKernel.lean`
- forbidden-element scan and `sha256sum` over the final disposable Lean source
- `df -h .` and `free -h` before broad validation
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Source-adequacy result: **one direct conditional explicit-flow kernel** and
  **one remaining full-proof formalization boundary**. Theory/07 directly
  requires observer-safe exports to ignore high-label state and raw
  witness/auth payloads when low state agrees; the finite projection makes that
  policy mechanically explicit without selecting a general flow structure.
- The positive theorem says only that two experiment-local inputs with the
  same low position yield the same one-field observer-safe export. It does not
  identify the fields with a canonical configuration or export row.
- The high-state projection differs for two inputs with the same low position.
  This is not observer-safe and therefore not a canon counterexample. It is a
  negative control showing that the redaction-to-export relation, rather than
  a record name or a label alone, is required for the property.
- The full OBL-018 proof remains under-specified: the canon does not select a
  configuration relation, observer context, final label lattice or flow order,
  declassification treatment, authority/retention pipeline semantics,
  occurrence provenance relation, export ABI, or collection
  equality/renaming/order/multiplicity semantics.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.6 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `lean --trust=0` passed for the final scratch. `#print axioms` reports that
  both recorded theorems depend on no axioms. The scan for `sorry`, `admit`,
  `axiom`, `opaque`, `unsafe`, `partial`, and `implemented_by` had no matches.
  Scratch hash:
  `e684f8802ea41b2343abb95892b3228f7a6796344320877696021a7ee890c2be`.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,430 numbered reports, and `cargo check` finished successfully.
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `git diff --check` passed.

## What changed in understanding

The canon's observer-safe policy already contains a usable first explicit-flow
kernel: a redaction operation must semantically discard the high/raw
components, not merely carry an observer-safe label. This supports a small
mechanized fragment without resolving the independent question from
T-RESEARCH-013: what complete configuration-to-export relation and output
equivalence define THM-005 for Mirrorea.

## Open questions

- What canonical configuration relation expresses agreement on low-label
  state, and how is observer context represented?
- What finite lattice, flow order, and authority-governed declassification
  relation will OPEN-020 select?
- How do occurrence/telemetry provenance, authority checks, redaction,
  retention, and export compose into a proof-facing relation?
- What equality or quotient governs export identity, references, ordering,
  and multiplicity for the full observer-safe result?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare a canon
formalization decision only when a THM-005 / OBL-017 / OBL-018 package must
choose the complete configuration, flow, and export interface.

## Plan update status

Updated: plan/156 records the direct finite explicit-flow kernel, its negative
control, the full-proof stop threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now distinguishes the direct low-position projection
kernel from the still-unselected complete observer/export formalization.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-023.

## tasks.md update status

Updated: T-RESEARCH-023 is closed as conditional LAB mathematical evidence;
the next selection excludes silently choosing its complete flow/export
interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested for this package. The immediately
preceding two candidate-selection tasks each had two concrete pre-submit
browser model-picker failures, so retrying the same unavailable consultation
path without an environmental change would not add independent review. Local
review instead re-read theory/01--02, theory/07, BND-008, SCN-07, ADR-0011,
and T-RESEARCH-013's strict non-claims. The final scratch was checked with
`#print axioms`; both the positive kernel and the negative control have no
axioms. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed execution, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
dashboard is unchanged because no sample or runner was modified. No Oracle
temporary chat was attempted because the repeated pre-submit picker failure is
concrete and unchanged; this source cut has local mechanical evidence.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
