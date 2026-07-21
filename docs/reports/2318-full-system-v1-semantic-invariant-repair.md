# Report 2318 - Full System V1 semantic invariant repair

- Date: 2026-07-21 23:48 JST
- Author / agent: Codex with a temporary Oracle design review and an independent read-only reviewer
- Scope: bounded Full System V1 source checker and interpreter boundary repair
- Decision levels touched: none; LAB evidence-maintenance only

## Objective

Repair three independently identified static-semantics gaps before they can
reach the bounded Full System V1 runtime: exact host-adapter capability
binding, duplicate record-field rejection, and the current scalar-only
equality restriction.

## Scope and assumptions

This package applies only to the parser-free, source-first Full System V1 LAB
surface. The private adapter pairs remain `read_int@host_input` and
`write_int@host_output`; no Mir core I/O primitive, public adapter ABI,
authentication system, or final grammar is introduced. The Canon remains the
normative source and is unchanged.

## Start state / dirty state

Started from pushed commit `c862ec39` with a clean worktree. Report 2317 had
already scheduled the three concrete checker defects after independent review.

## Documents consulted

- `AGENTS.md`
- `CANON.md`
- `mirrorea_canon/README.md`
- `mirrorea_canon/MAP.md`
- `mirrorea_canon/theory/02-types-effects-failures.md`
- `mirrorea_canon/spec/03-static-semantics.md`
- `mirrorea_canon/spec/07-diagnostics.md`
- `specs/35-full-system-v1-static-semantics-and-runtime.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`
- `tasks.md`, `progress.md`, `samples_progress.md`, and `docs/project-status.md`
- active Full System V1 source matrices, expected diagnostics, checker,
  interpreter, and focused tests
- the repository-local Oracle operating guidance before a temporary design
  review

## Actions taken

1. Reproduced all three reported defects with adversarial source programs and
   test assertions before changing implementation code.
2. Obtained a temporary Oracle design review for the adapter-policy boundary;
   it recommended exact operation/pair policy shared by lowering and runtime,
   with source signature, operation capability, and transition ambient context
   checked separately.
3. Added a private `adapter_policy` module for the two exact host adapter
   pairs, their input/output types, and required capabilities.
4. Made lowering reject source declarations whose host-adapter signature or
   declared capability disagrees with that policy, include the policy
   capability in the effective required row, and reject host-adapter calls in
   function contexts that have no transition ambient capability row.
5. Made record construction reject every second and later occurrence of a
   field at the duplicate field span, before lowering can silently overwrite
   it in the interpreter's map representation.
6. Restricted source equality and inequality to matching scalar types:
   `Int64`, `Float64`, `Bool`, `Text`, and recovery-only `Error`. Named
   records, fixed arrays, unit, and unsigned integers are rejected pending a
   deliberate semantic decision.
7. Added nine active expected-negative source rows and focused regression
   coverage, then synchronized the active sample dashboard, status snapshot,
   task map, LAB roadmap memory, and concise project entry.

## Files changed

- `crates/mir-semantics/src/full_system_v1/adapter_policy.rs`
- `crates/mir-semantics/src/full_system_v1/mod.rs`
- `crates/mir-semantics/src/full_system_v1/checker.rs`
- `crates/mir-semantics/src/full_system_v1/interpreter.rs`
- `crates/mir-semantics/tests/typed_ir_interpreter.rs`
- `samples/full-system-v1/computational/typed-ir-matrix.json`
- nine new expected-negative fixture roots under
  `samples/full-system-v1/computational/`: host capability/signature/context,
  duplicate record field, and record/fixed-array equality cases
- `scripts/full_system_v1_release_check.py`
- `scripts/tests/test_full_system_v1_release_check.py`
- `scripts/tests/test_full_system_v1_samples.py`
- `README.md`
- `samples/README.md`
- `samples/full-system-v1/computational/README.md`
- `plan/58-full-system-v1-roadmap.md`
- `plan/60-computational-runtime-roadmap.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/reports/2318-full-system-v1-semantic-invariant-repair.md`

## Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- focused source, Canon, matrix, and implementation inspection with `rg` and
  `sed`
- red-phase focused Rust and Python regression tests for all reported gaps
- `cargo test -p mir-semantics --test typed_ir_interpreter -- --nocapture`
- `python3 -m unittest scripts.tests.test_full_system_v1_samples`
- `python3 scripts/full_system_v1_samples.py checker-check-all --format json`
- `python3 scripts/full_system_v1_samples.py check-all --format json`
- `cargo test -p mir-runtime --test full_system_v1_session -- --nocapture`
- `cargo fmt --all -- --check`
- `python3 scripts/validate_docs.py`
- `python3 scripts/check_source_hierarchy.py`
- `make check`
- `git diff --check`
- final `full_system_v1_release_check.py --format json check-all --out ...`

## Evidence / outputs / test results

The red phase demonstrated that a `write_int@host_output` declaration carrying
only `HostRead` could execute, duplicate record fields were accepted and
silently overwrote earlier values, and record/fixed-array equality was
accepted. Additional adversarial rows showed that source signatures could be
wrong and host-adapter calls inside functions could execute without an ambient
transition capability row.

After the repair, the focused typed-IR interpreter suite passed 20 tests; the
Full System V1 sample suite passed 24 tests; the runtime session suite passed
17 tests. The checker matrix accepted all 21 rows (3 positive and 18 expected
negative), and the aggregate executable matrix accepted all 50 rows (21
checker + 17 runtime + 12 operational) with no validation failures. Formatting,
documentation validation, source-hierarchy validation, and `make check` also
passed before final report validation. The isolated final release workflow
accepted all 29 planned commands, including the 50-row aggregate check.

## What changed in understanding

`TypedPerformCall.required_capabilities` is now an effective lower-bound row
for the two private host adapter pairs, rather than merely a reflection of an
arbitrary source effect declaration. That prevents an accepted source program
from presenting `host_output` as `HostRead`-only.

This is not trusted authorization. It does not identify or authorize a
principal, assign runtime grants, define a public adapter ABI, or infer
capability inheritance through functions. The scalar equality restriction is
also an intentional current boundary, not a decision that composite equality
is impossible.

## Open questions

1. A trusted runtime admission, authentication, authorization, and grant model
   remains outside Full System V1's bounded source/runtime evidence.
2. Function effect summaries and capability-context inheritance need a
   separate interprocedural design before host operations can be permitted
   through functions without explicit transition context.
3. Composite equality, `Key` representation, and equality's final type-class
   boundary remain unresolved; records and fixed arrays stay unsupported.
   `Float64` equality is statically admitted by Canon but Float64 execution is
   outside the current interpreter floor, so it is not a runnable claim.
4. A defense for manually constructed malformed `TypedIR` is a possible
   implementation-integrity hardening, separate from the repaired source path.

## Suggested next prompt

Select the next bounded LAB maintenance candidate from the current task map,
then advance it with the same source-first adversarial evidence, theory-boundary
review, runnable validation, and status synchronization discipline.

## Plan update status

`plan/` 更新済み: `plan/58` and `plan/60` now record the shared private adapter
policy, the 3-positive/18-negative checker guard, and the explicit nonclaims.
No Canon roadmap or decision level changed.

## Documentation.md update status

`Documentation.md` 更新不要: no new reader-facing entry point, command family,
or public claim was added.

## docs/project-status.md update status

更新済み: the concise status view now distinguishes the repaired source-policy
guards from the absent trusted runtime authorization, public ABI, function
capability inheritance model, and Float64 runtime support.

## progress.md update status

更新済み: the current LAB snapshot records the repaired static semantic
boundary and the 21/17/12 = 50 active executable partition.

## tasks.md update status

更新済み: the completed invariant-repair package is closed and no unrelated
Full System V1 widening is selected as an immediate replacement.

## samples_progress.md update status

更新済み: the computational checker row now names the 3-positive/18-negative
guard and records the repaired host, duplicate-field, and equality boundaries.

## Reviewer findings and follow-up

The temporary Oracle review recommended the exact shared pair policy adopted
here, including independent validation of effect signature, operation-specific
capability, and transition context. It also recommended duplicate-span
diagnostics and a scalar equality whitelist; both are covered by focused
regressions.

The independent read-only reviewer found no blocking issue. Its two low-risk
findings were incorporated: capability fixtures now isolate declaration-policy
mismatch from ambient-row absence, and this report records that Float64
equality is static-only while the interpreter floor excludes Float64 execution.
No Canon change is proposed because the repair makes bounded LAB behavior obey
the existing static-semantics boundary rather than choosing new normative
semantics.

## Skipped validations and reasons

No Canon edit or formal proof was attempted: this package adds no normative
decision and does not claim a proof of trusted authorization. Float64 runtime
execution remains out of scope because it predates this narrow checker repair;
the report therefore makes no runnable Float64 equality claim. A runtime
defense against manually constructed malformed typed IR is deliberately
deferred because the repaired claim is the accepted source path; it is recorded
as a separate hardening question.

## Commit / push status

Pending at report write; the task closes only after final release validation,
review incorporation, commit, immediate push, and clean-worktree confirmation.

## Sub-agent session close status

The temporary Oracle design review completed without workspace edits. The
independent read-only final reviewer completed, made no workspace edits, and
will be closed after final validation. No sub-agent owns or edits
implementation files.
