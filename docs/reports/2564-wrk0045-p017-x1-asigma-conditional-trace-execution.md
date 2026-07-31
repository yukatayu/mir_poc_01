# Report 2564 — WRK-0045 P017 X1 A-Sigma conditional trace execution

- Date: 2026-07-31
- Author / agent: Codex
- Scope: Execute exactly the pre-registered WRK-0045 Markdown-held Lean source,
  preserve a reproducible negative outcome, and make no replacement candidate.
- Decision levels touched: L3 evidence only; no L0/L1/L2 decision, theorem/OBL,
  Gate, Phase, implementation contract, or public claim changed.

## Objective

Test whether the registered A-Sigma `H_K-rs` ledger supports the one permitted
candidate-local conditional trace without hidden branch matching, a reserved
semantic surface, or a vacuous premise bundle. Freeze the record and return
`DEFER` if any registered falsifier is reproducible.

## Scope and assumptions

`mirrorea_canon/` is normative. WRK-0045 was already committed and pushed as
a source-free L3 record; its declared authority cut remains unchanged. The
only retained experiment artifact is the declared `plan/` Markdown block. All
finite values, positive interpretations, RED checks, and adverse models were
created under `/tmp`, were not committed, and are not a Mir occurrence carrier,
matching key, relation schema, or semantic model.

The result is negative evidence: it does not repair the source, select B-Pi,
or infer any missing Core, receipt, identity, transition, persistence,
authority, runtime, transport, or public behavior.

## Start state / dirty state

`main` and `origin/main` were equal and clean at
`7dece72402910ab375fb2f857de88dcf04ad005f`. WRK-0045 was `L3-open`,
`not-promoted`, source-free, and unexecuted. The Canon `INDEX.json` was found
stale for the already committed WRK-0045 registration and current `MAP.md`;
regeneration changes only those generated entries.

## Documents consulted

- Canon: `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, ADR-0014,
  P012, P013, P017, theory/01, 02, 04, 05, and 07, and WRK-0045.
- LAB: Plans 229, 241--244, the WRK-0044 source/evidence, `Documentation.md`,
  `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`,
  and the report template.
- Operations: the local Oracle manual and the repo-local Oracle notes.

## Actions taken

1. Confirmed the committed/pushed source-free registration and materialized
   exactly one no-import Lean block at the registered path.
2. Ran two disposable RED checks: `q prec s` does not establish `q prec r`,
   and a receipt fact does not establish request uniqueness. The first receipt
   draft had a harness type error and was discarded; the corrected RED check
   failed only for the intended missing equality proof.
3. Kept only generic type parameters and Prop-valued predicates. The retained
   block has 19 small conditional declarations; it has no constructor, finite
   role index, key, lookup, matching function, restore function, import, or
   data-returning definition.
4. Ran a fresh-extract `lean --trust=0` check, `#print axioms` on every
   declaration, syntax/surface scans, a finite positive harness, and 17
   disposable adverse harnesses.
5. Requested one temporary GPT-5.6 Sol Pro source review, then checked its
   decisive branch-sharing finding locally with an exact extracted-source
   countermodel.
6. Rebuilt the Canon generated index after confirming it only catches up the
   already-committed WRK-0045 and `MAP.md` metadata.

## Files changed

- `plan/wrk-0045-p017-x1-k0-hk-rs-asigma-conditional-trace.md`
- `mirrorea_canon/INDEX.json`
- `docs/reports/2564-wrk0045-p017-x1-asigma-conditional-trace-execution.md`

## Commands run

- Source extraction from the sole `lean` fenced block and `lean --trust=0`.
- `#print axioms` for all 19 retained declarations in a fresh extracted file.
- Static source scans for proof escapes, declaration/choice/classical escapes,
  finite/index/key/lookup escapes, and import count.
- Disposable RED, positive finite interpretation, 17 adverse interpretations,
  and the exact branch-sharing countermodel, all with `lean --trust=0`.
- `python3 scripts/check_source_hierarchy.py`,
  `python3 scripts/validate_docs.py`, and
  `python3 meta/build-index.py --check` from `mirrorea_canon/`.
- Git status/diff checks, disk/memory audit, and Oracle session inspection.

## Evidence / outputs / test results

The retained source has exactly one Lean block, 19 declarations, no imports,
and SHA-256
`690d67db0de7aca7182cf6dc6c74988480c0923fffc6fa687c132cd706dbba1d`.
Fresh extraction accepted `lean --trust=0`; every declaration reported no
axiom dependency. The source scans found none of the registered construct,
classical-choice, finite-role, identity/key, lookup, or restore-function
escapes.

The positive `Fin 3` harness instantiated `q < s < r`, exact local receipt
matching, an owner-failure branch, r-sensitive prefix closure, and a
correspondence fact without retaining any finite value in the source. It
compiled under `--trust=0`. The 17 adverse harnesses also compiled and cover
A-Sigma residence, pending, M1/authority, owner exclusivity, both direct legs,
order inclusion, typed result, matching, use, failure prerequisites, authority
mapping, K0, cuts, restore, and non-vacuity.

However, the exact extracted-source branch-sharing model is decisive. With one
opaque requester and one binding but two `Bool` branches, `pending` is true on
both branches; the source theorem still accepts its binding-uniqueness and
request-only non-sharing premises, while `false = true -> False` proves the
branches distinct. It therefore fails the pre-registered branch-to-binding
non-sharing requirement. This is a reproducible falsifier, so the outcome is
`DEFER` and the current source must be retained as negative evidence rather
than repaired in place.

The temporary Oracle review independently reached `FREEZE`. Its other findings
were advisory but consistent with the local result: receipt matching is too
local, A-Sigma tags are detached from substantive facts, several restore/K0/
non-vacuity declarations are premise wrappers, and a successor would need a
new pre-registration rather than an in-place patch. No external transcript is
treated as repository state.

The hierarchy check passed `794/794`; the rebuilt Canon index passed its
check. With this report as the latest numbered report, documentation validation
passed and the complete focused validator suite passed all 88 tests in
`4201.114s`.

## What changed in understanding

The selected A-Sigma shape can be written in Lean without reserved language or
runtime machinery, but its first materialization does not make the required
facts mutually constraining. In particular, request-only non-sharing is
insufficient for the registered pending account. Conditional source validity
therefore requires not only typed predicates but also independently audited
cross-fact consumers; opaque predicate names cannot supply that connection.

## Open questions

WRK-0045 is frozen pending its Canon evidence linkage. Any later inquiry must
be a forward successor with a new source-free registration and recheck whether
the needed atomization can be expressed without a branch key, receipt identity,
schema, transition, or restore function. All P017 X1 semantic and operational
questions remain open.

## Suggested next prompt

Link this negative evidence to WRK-0045, set its reliance status to frozen,
record `DEFER`, and refresh reader snapshots. Only then consider whether a
new, independently pre-registered L3 successor is justified.

## Plan update status

`plan/` 更新済み: the only planned source path now contains the exact failed
conditional experiment. Plan 244 is not rewritten; its `DEFER`/freeze rule is
being followed.

## Documentation.md update status

`Documentation.md` 更新不要: this source evidence is not yet linked into the
working record or reader-facing plan index; that happens in the separate
post-evidence metadata package.

## docs/project-status.md update status

更新不要: the source/evidence package is intentionally limited to the
registered path, direct report, and generated operational metadata. The next
metadata package will expose the frozen status to readers.

## progress.md update status

`progress.md` 更新不要: no snapshot is changed before the working-record
linkage. The immediately following freeze/link package must update it.

## tasks.md update status

`tasks.md` 更新不要: no new autonomous task is selected before the frozen
record is linked and the successor boundary is reviewed.

## samples_progress.md update status

`samples_progress.md` 更新不要: this L3 Lean evidence creates no runnable Mir
sample, runner, debug surface, or sample workflow.

## Reviewer findings and follow-up

The temporary Oracle review completed and advised `FREEZE`; its decisive
branch-sharing countermodel was reproduced locally against the exact extracted
source. The remaining review points are recorded as successor constraints, not
as an in-place repair specification. No callable sub-agent interface was
available.

## Skipped validations and reasons

No parser, runtime, transport, or sample command applies because this is not
an implementation or a public workflow. The frozen working-record edit and
reader snapshot checks are intentionally separate from this source/evidence
commit, as required by WRK-0045.

## Commit / push status

Pending at report write. The source/evidence commit will be pushed before the
append-only frozen-status linkage is made in `working/WRK-0045`.

## Sub-agent session close status

No callable sub-agent session was opened. Temporary Oracle session
`wrk0045-source-audit` completed and was distilled into this report; its
external transcript is not normative repository state.
