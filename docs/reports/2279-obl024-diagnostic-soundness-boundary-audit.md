# Report 2279 - OBL-024 diagnostic soundness boundary audit

## Objective

Determine whether canon theory/10 and the diagnostics format derive a
proof-facing explanation-soundness statement, without promoting the existing
LAB diagnostic projection or abstract Lean draft.

## Scope and assumptions

Canon remains normative. The disposable Lean model represents only whether a
diagnostic is emitted, reports a rule/premise/bindings shape, and is connected
to an actual rule/premise/replay relation. It is not a MirCore diagnostic,
judgment, emission relation, replay engine, JSON ABI, or proof of OBL-024.

## Start state / dirty state

The worktree was clean at `27bcf12e`. T-RESEARCH-026 recorded its Discord task
baseline before candidate reading and placed all Lean experiments only under
`/tmp`. No tracked source had changed before this audit record.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, theory/03, theory/10,
  theory/11, spec/07, and architecture/02
- LAB `plan/81`, `plan/109`, `plan/110`, `plan/156`, `tasks.md`,
  `progress.md`, `docs/project-status.md`, and the OBL-024 Lean statement draft
- `.docs/oracle-chatgpt-pro-operations.md` and the Oracle operating guide

## Actions taken

- Re-read canon's Diagnostic carrier, blame direction, OBL-024 target wording,
  required fields, and the elaboration rejection arm.
- Re-read the LAB OBL-024 statement draft and executable E-ROW projection to
  distinguish report-local association/replay vocabulary from a future
  proof-level relation.
- Built a disposable Boolean twin. Both models contain an emitted Diagnostic
  with reported rule, premise, and bindings; only the aligned model has actual
  rule/premise and replay-at-reported-premise links. The divergent model keeps
  the carrier shape while failing the candidate soundness relation.
- Did not retry Oracle: the concrete pre-submit browser model-picker failure
  remains unchanged. The repository operating note defaults new questions to
  temporary chats; this bounded source cut had local mechanical evidence.

## Files changed

- `docs/reports/2279-obl024-diagnostic-soundness-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The final disposable source remains outside the repository at
`/tmp/mirrorea-t-research-026/DiagnosticSoundnessBoundary.lean`.

## Commands run

- focused canon/LAB source searches with `rg` and `sed`
- `lean --trust=0 /tmp/mirrorea-t-research-026/DiagnosticSoundnessBoundary.lean`
- `#print axioms` through a disposable imported-module check
- forbidden-element scan and `sha256sum` over the final disposable Lean source
- `df -h .` and `free -h` before broad validation
- `cargo test -p mir-semantics --test surface_to_core_elaboration -- --nocapture`
- `python3 -m unittest scripts.tests.test_surface_mir_samples`
- `python3 scripts/surface_mir_samples.py --format json check-all`
- `make check`
- `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync`
- final documentation/source-hierarchy and Git diff checks listed below

## Evidence / outputs / test results

- Source-adequacy result: `0 direct / 0 delegated / 1 missing` **coupled
  diagnostic-emission association / replay formalization boundary**. Theory/10
  and spec/07 give the target direction and field vocabulary but do not define
  the relation that makes an emitted report actual and replayable.
- The finite model proves a carrier-shaped report can exist without actual
  rule/premise/replay links. This is not a counterexample to canon's settled
  blame direction; it excludes the target property itself as a premise and
  demonstrates why the carrier fields do not derive it.
- Existing E-ROW `diagnostic_soundness_projection` and the LAB Lean draft are
  useful evidence for one instantiation and vocabulary shape. They remain
  report-local / compile-check-only, not delegated proof evidence or a final
  ABI.
- A full OBL-024 statement remains under-specified: canon does not select a
  diagnostic emission/association relation, judgment/rejection carrier,
  canonical rule/premise IDs, binding reconstruction, replay granularity,
  trace-local exact-at relation, span blame relation, or diagnostic
  equality/ordering semantics.
- Before broad validation, the repository filesystem had 21 GiB available
  (89% used). Memory had 9.4 GiB available and 14 GiB free swap; this package
  added no repository-local heavy artifact.
- `lean --trust=0` and `#print axioms` passed for the final scratch. The scan
  for `sorry`, `admit`, `axiom`, `opaque`, `unsafe`, `partial`, and
  `implemented_by` had no matches. Scratch hash:
  `c71c528eff2ba5fb4fae6f6076b84bee0a3e236a94fb20cde94cb6c177fa049e`.
- `cargo test -p mir-semantics --test surface_to_core_elaboration --
  --nocapture` passed all 36 tests, including the current OBL-024 E-ROW
  projection rows. `python3 -m unittest scripts.tests.test_surface_mir_samples`
  passed all 48 tests. Surface `check-all` passed all 53 listed samples with
  no validation errors; it reports `workflow_ready: false`, consistent with
  the non-final LAB classification.
- `make check` passed: source hierarchy `704 / 704`, documentation validation
  passed with 1,433 numbered reports, and `cargo check` finished successfully.
  `python3 -m unittest scripts.tests.test_current_l2_lean_sample_sync` passed
  all 21 tests. `git diff --check` passed.

## What changed in understanding

The carrier vocabulary is not the proof relation. The decisive missing link is
not one additional JSON field: it is a relation tying the emitted diagnostic to
the same rejected judgment and to a replay result that fails at the reported
premise. This preserves the value of the current E-ROW projection while
preventing its report-local keys and anchors from becoming implicit language
semantics.

## Open questions

- What judgment/rejection carrier and emission/association relation are
  canonical for diagnostics?
- What rule-instance, premise, and binding representation supports replay?
- Is replay whole-judgment, rule-local, or a two-level relation, and what does
  "exactly there" mean without claiming global root-cause uniqueness?
- How do multi-span declaration/use-site blame and multiple diagnostics affect
  equality and ordering?

## Suggested next prompt

Audit OBL-025 Line-1 repair completeness as the remaining independent
theory/10 source cut, retaining its LAB draft as non-normative evidence only.

## Plan update status

Updated: plan/156 records the OBL-024 source cut, divergent carrier model,
formalization stop threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now separates canon diagnostic direction from the
unselected emission-association/replay relation and LAB projection evidence.

## progress.md update status

Updated: current research and the dated recent log include T-RESEARCH-026.

## tasks.md update status

Updated: T-RESEARCH-026 is closed as LAB source-adequacy evidence; OBL-025 is
now the next independent source cut.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

No advisory Oracle conclusion was requested. The repeated browser model-picker
failure is concrete and unchanged, and this bounded audit relies on local
canon/LAB sources. The final scratch was checked with `#print axioms`; both
recorded theorems have no axioms. No local sub-agent service was available.

## Skipped validations and reasons

Distributed execution, conformance, and product checks do not apply to this
documentation and disposable-Lean source audit. Existing E-ROW validation is
run as LAB evidence only; it cannot validate a final replay engine or prove
OBL-024. The runnable sample dashboard is unchanged because no sample or
runner was modified.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available; no session was opened or requires
closure.
