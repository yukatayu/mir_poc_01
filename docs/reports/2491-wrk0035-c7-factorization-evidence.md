# Report 2491 - WRK-0035 C7 factorization evidence

**Identifier:** `LAB-REPORT-2491`
**Date:** 2026-07-28 14:54 JST
**Status:** evidence package validated; commit/push pending

## Objective

Run the post-registration outcome procedure for `working/WRK-0035` and retain
only its declared artifact-local constructive conditional lemma, collision
refutations, and fixed full-codomain countermodel.

## Scope and assumptions

This package is confined to the already permitted `plan/` and `docs/reports/`
LAB lanes. The local `E`, `S`, `O`, `erase`, and `observe` symbols remain
uninterpreted mathematical parameters. The result neither selects a Mir source
form nor authorizes omission, desugaring, reconstruction, an interface, or a
semantic contract.

## Start state / dirty state

Start point was clean `main` at `f46d3d058294b02e894b841c4899b3e23d8bdd13`,
equal to `origin/main`, with `WRK-0035` registered and pushed. The working tree
then contained only the new evidence artifact and its `plan/00-index.md` entry.

## Documents consulted

- `AGENTS.md`, `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and ADR-0014
- `working/WRK-0035-c7-parametric-factorization.md`, theory/03, and P012
- Plans 199, 204, and 205; report 2489; and the registration report 2490
- The advisory Oracle eligibility review already summarized in Plan 205 and WRK-0035

## Actions taken

1. Rechecked that every pinned Canon/LAB input exists and has the registered SHA-256 digest.
2. Materialized the single fenced Lean block into a disposable temporary file and ran the
   registered `lean --trust=0` command.
3. Ran `#print axioms` for the two positive/refutation theorem families and the
   full-codomain countermodel theorem.
4. Scanned the exact extracted source for the pre-registered classical, quotient, unsafe,
   placeholder, and axiom tokens.
5. Kept the proof pointwise: it states a unique realized observation for each point in
   `range erase`; it does not construct a reconstructor.

During proof authoring, the deliberately incomplete RED attempt did not close by `rfl`.
Bare Lean 4.29 in this unimported lane also exposes neither `exists!` notation nor
`ExistsUnique`; the artifact therefore spells the same pointwise proposition as an
ordinary existential plus a universal uniqueness implication. Initial green-source errors
from ASCII boolean `!=` and `not` were corrected to proposition-level `≠` and `¬` syntax.
These were local notation/proof fixes, not changes to the registered statement or scope.

## Files changed

- `plan/wrk-0035-c7-parametric-factorization.md`
- `plan/00-index.md`
- this report

## Commands run

- Registered-input presence check and SHA-256 capture
- Registered fenced-block extraction followed by `lean --trust=0`
- Extracted-source forbidden-token scan
- `git diff --check`

## Evidence / outputs / test results

All seven registered input digests match the WRK-0035 authority cut. The evidence artifact
digest is `8e27a94f876b9db33d6d30cc56b4569f83094b0cc4d17261bd680497327309a3` before
the evidence commit.

The exact extraction command passed. Lean reported that each of
`fiberConstant_iff_uniqueObservedOnImage`, `collision_not_fiberConstant`,
`collision_not_uniqueObservedOnImage`, and `noUniqueFullReconstructor` does not depend
on any axioms. The forbidden-token scan was clean, and `git diff --check` passed.

The retained positive result is the generic equivalence between fiber constancy and
pointwise unique realized observation on the image. The retained negative results show
that an explicit collision refutes both predicates, and that the pointwise result does
not imply a unique function over all of the codomain.

## What changed in understanding

The C7 design constraint now has a checked generic boundary: extensional uniqueness of a
realized observation can be expressed constructively without choice, quotients, or a
concrete source interpretation. This is still insufficient for ergonomics in Mir, because
inspectable grounds, a concrete elaborated artifact, and a source-level authorization rule
remain intentionally outside this L3 result.

## Open questions

- Which concrete elaborated artifact, if any, could carry both an omitted fact and its
  inspectable grounds without selecting a premature surface grammar.
- Whether a later source design needs an executable reconstruction function, which would
  require a separate boundary and cannot be inferred from this pointwise theorem.
- Whether any C7 candidate has a non-reserved consumer before the relevant source and
  elaboration decisions exist.

## Suggested next prompt

Append the evidence metadata to WRK-0035 without rewriting its pre-registration, then
synchronize the LAB status snapshots while retaining C7 as non-promoted research evidence.

## Plan update status

更新済み: `plan/00-index.md` now registers the evidence artifact. Plan 205's selection
is unchanged; outcome disposition belongs to the later synchronized snapshot.

## Documentation.md update status

更新不要: this evidence commit is restricted to WRK-0035's declared LAB lanes. The
reader-facing status is synchronized in a subsequent snapshot package.

## docs/project-status.md update status

更新不要: this evidence commit must not enlarge its declared evidence surface. A later
snapshot records that the L3 result is evidence only.

## progress.md update status

更新不要: no Gate, Phase, OBL, proof, sample workflow, or official readiness changes.
A later snapshot records the completed research package.

## tasks.md update status

更新不要: the next research frontier is not selected by this limited evidence commit.

## samples_progress.md update status

更新不要: no active sample root, validation command, debug surface, or runnable workflow changed.

## Reviewer findings and follow-up

The advisory Oracle eligibility review required a range-only, pointwise statement and warned
against full-codomain reconstruction and hidden classical machinery. The retained artifact
meets those constraints and supplies the declared full-codomain countermodel. No additional
independent review is required for this L3 evidence package. No callable sub-agent session
was available.

## Skipped validations and reasons

No concrete Mir source/elaboration instantiation, sample execution, parser/checker run, or
formal proof status promotion was attempted: each would exceed the pre-registered scope.
The project-wide documentation build and Canon result metadata update follow in separate
packages so the evidence commit retains the exact declared surface.

## Commit / push status

Pending evidence commit, push, fetch, and `HEAD == origin/main` verification.

## Sub-agent session close status

No callable sub-agent session was opened.
