# Report 2383 - WRK-0018 telemetry-effect direct outcome

- Date: 2026-07-23 02:52 JST
- Author / agent: Codex
- Scope: registered Lean-tail outcome capture before working-record manifestation
- Decision levels touched: none; this is bounded LAB evidence, not a Canon or OBL result

## Objective

Execute the pushed WRK-0018 commands far enough to determine whether its exact
concrete telemetry-dependency toy model compiles and establishes the registered
positive and adverse observations without crossing a reserved semantic boundary.

## Scope and assumptions

WRK-0018 at pushed registration `9828dc20` is the sole authority for this
experiment.  The added source is retained only as unmanifested LAB evidence
until the working record appends this report's full commit hash.  The result is
limited to its concrete `Nat`/`Bool` model and makes no claim about an actual
Mir effect, telemetry row, low-equivalence relation, or observer-safe export.

## Start state / dirty state

The registration package was pushed at
`9828dc20`.  The worktree was clean before the registered source edit.  The
root filesystem had 6.9 GiB available (97% used), no external workdir was
mounted, and Cargo metadata resolves the target directory to the repository
root.

## Documents consulted

Read Canon README/MAP, ADR-0014, theory/02, theory/07, theory/11,
`arch/02-boundary-contracts`, WRK-0018, plan 177, Reports 2381 and 2382, the
exact IFC Lean foundation and companion explanation, current status snapshots,
and the working-record lifecycle rules.

## Actions taken

1. Added the single registered `WRK0018TelemetryEffectModel` tail and matching
   explanation marker after confirming an initial deliberately incomplete tail
   failed on its missing model identifiers.
2. Kept the model concrete: `Nat` low positions, `Bool` high flags, one visible
   position row, a low-determined telemetry function, and a high-dependent
   telemetry function with a fixed adverse pair.
3. Compiled the completed foundation and ran the exact source-prefix/forbidden-
   vocabulary guard from WRK-0018.
4. Audited available storage and Cargo's target path before deciding not to run
   the registered sync script, which would create a heavy repo-root build.

## Files changed

- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.md`
- this report

## Commands run

- `lean --version`
- an intentionally incomplete `lean --trust=0` tail compile (red check)
- `lean --trust=0 samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- the exact WRK-0018 SHA-256 prefix/marker/forbidden-vocabulary guard
- `git diff --check`
- `df -h .`, `free -h`, and `cargo metadata --no-deps --format-version 1`
- `make docs` before the source edit, which passed Canon index, hierarchy, and
  documentation validation at the registered state

## Evidence / outputs / test results

Lean 4.29.1 compiled the completed foundation with `--trust=0` and no output.
The registered source-shape guard printed `WRK-0018 source-shape guard
passed`.  The positive theorem proves that two configurations with the same
`lowPosition` export the same concrete row through
`lowDeterminedTelemetry`.  The adverse theorem proves that `adverseLeft` and
`adverseRight` agree at `lowPosition = 7` but export different rows through
`highDependentTelemetry`.

The deliberate red check first failed only because its model names had not yet
been declared.  During green completion, Lean rejected `!=` as a Boolean
comparison and then rejected a whole-conjunction `decide` over an abstract
proposition; the final source uses `Not (x = y)` and proves the conjunction in
two concrete branches.  These were local proof-form corrections, not changes
to the pre-registered question or its result.

`git diff --check` passed.  The full current-L2 sync did not run: its Cargo
target is `/home/codex/dev/mir_poc_01/target`, and starting it on a filesystem
with only 6.9 GiB free would violate the repository's heavy-artifact policy.

## What changed in understanding

The source-level experiment confirms only the intended dependency distinction:
low agreement is sufficient for the low-determined toy export, while a toy
export that reads a modeled high flag has a fixed low-agreeing counterpair.  It
does not identify either function with Canon's declared telemetry effect or
resolve the missing provenance and low-equivalence relations noted in theory/07
and BND-008.

## Open questions

- Is there a future, separately registered reason to select a real
  low-equivalence/provenance relation rather than retain this toy boundary?
- Can a safe external build workdir be mounted before a later full Lean manifest
  synchronization is required?

## Suggested next prompt

Append this evidence commit to WRK-0018, retain its bounded result as
not-promoted, and perform one independent boundary review before selecting any
successor research question.

## Plan update status

`plan/` 更新不要: plan 177 is the immutable selection and pre-registration
input.  This direct outcome does not alter its question, alternatives, or
freeze line.

## Documentation.md update status

`Documentation.md` 更新不要: the reader map changes only when the working
record append-only manifests this retained evidence.

## docs/project-status.md update status

更新不要: this report records an unmanifested direct source outcome; the current
status changes with the subsequent working-record manifest.

## progress.md update status

`progress.md` 更新不要: no retained current-state change is published until the
working record contains this exact evidence commit.

## tasks.md update status

`tasks.md` 更新不要: package 48 remains at its registered evidence boundary
until the append-only record manifestation is committed.

## samples_progress.md update status

`samples_progress.md` 更新不要: the existing Lean foundation command remains
unchanged, no workflow readiness is added, and the full dashboard sync is
explicitly unexecuted for storage safety.

## Reviewer findings and follow-up

The selection package's two subagent audits and temporary Oracle ranking
constrained the question.  A fresh independent review is required after this
source evidence is committed; it must verify the bounded interpretation and
must not promote the toy theorem into a telemetry semantic claim.

## Skipped validations and reasons

`python3 scripts/current_l2_lean_sample_sync.py` is skipped because it builds
into the repo-root Cargo target on a nearly full root filesystem.  Runtime,
distributed, and broad Cargo suites do not exercise this helper-local Lean
tail.  The comprehensive Python documentation-test run from the selection
package had no final retrievable result after its long execution, so this report
does not treat it as passing.

## Commit / push status

Pending at report write.  This direct evidence will be committed with
`--no-gpg-sign` and pushed before its exact full commit hash is appended to
WRK-0018.

## Sub-agent session close status

The selection package's subagents are closed.  No subagent was opened while
the deterministic Lean compiler outcome was being established; a focused
reviewer will be opened after this evidence commit.
