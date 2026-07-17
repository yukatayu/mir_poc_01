# Report 2267 - THM-003 / OBL-009 successful-load restoration boundary audit

## Objective

Determine whether the THM-003 / OBL-009 source cut derives a complete
proof-facing interpretation from a successful load to the restored
configuration and restored history prefix.

## Scope and assumptions

Canon remains normative. The disposable one-save/two-result Lean model is LAB
evidence about an under-specified restoration interface, not a counterexample
to canonical THM-003, a legal MirCore load, or an implementation of
SaveObject, Config, or persistence.

## Start state / dirty state

The worktree was clean at `48cfaace`. THM-003 and OBL-009 through OBL-014 / OBL-027
remained open in the canonical ledger. T-RESEARCH-014 began with the Discord
task baseline recorded and placed its Lean experiment only under `/tmp`.

## Documents consulted

- `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, and theory/11
- Canon theory/01, theory/04, theory/08, theory/09, theory/10, and theory/11
- LAB `plan/156`, `tasks.md`, `progress.md`, and `docs/project-status.md`
- Existing LAB Lean statement roots and historical OBL-024/025 evidence
- `.docs/oracle-chatgpt-pro-operations.md`

## Actions taken

- Compared four candidate theory source cuts with a temporary Oracle selection
  review and selected the earlier-ledger OBL-009 restoration bridge.
- Separated theory/04's SaveObject schema anchor and eight necessary
  successful-load conditions from the THM-003 target sentence.
- Modeled saved-cut and restored-prefix consistency through different
  projections into the same finite `Consistent` predicate.
- Built good/bad load-result twins that both respect the eight selected
  successful-load conditions, then differ only in the experiment-local result
  relation.
- Applied the exact-review corrections: result-sensitive policy arity,
  no-resurrection versus no-live separation, all selected no-live predicates,
  and a StoreKeyWF-only well-formedness failure.
- Reproduced and fixed a Lean parse error caused by the reserved identifier
  `prefix`; the final source has no warnings or placeholders.

## Files changed

- `docs/reports/2267-obl009-successful-load-restoration-boundary-audit.md`
- `plan/156-t0-t2-research-autonomy-envelope.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`

The disposable source remains outside the repository at
`/tmp/mirrorea-t-research-014/LoadRestoreBridgeCountermodel.lean`.

## Commands run

- focused canon/LAB source searches with `rg`, `sed`, and `find`
- `lean --trust=0 /tmp/mirrorea-t-research-014/LoadRestoreBridgeCountermodel.lean`
- forbidden-element scan and `sha256sum` over the disposable Lean source
- temporary Oracle sessions `next-theory-source-cut-014`,
  `obl009-load-restore-bridge-review`, and
  `obl009-corrected-load-restore-review`
- final documentation/source-hierarchy and focused regression commands listed
  below

## Evidence / outputs / test results

- Frozen source-adequacy result: `0 direct / 0 delegated / 1 missing` coupled
  successful-load restoration formalization boundary. This is one complete
  proof-facing row; it does not deny direct source policy anchors.
- Separate source-anchor inventory: one grouped direct SaveObject schema anchor,
  eight direct necessary successful-load condition anchors, and one
  theory/01-delegated Config/WellFormed vocabulary family. The chapter-local
  vocabulary delegation is not delegated proof evidence.
- The same finite model proves both `LoadResultGood` and `LoadResultBad`
  respect all eight selected successful-load condition tags. Both also have
  consistent empty saved/restored frontiers and all five selected no-live tags.
  `LoadResultGood` satisfies the selected THM-003 shape. `LoadResultBad`
  returns a configuration with only the modeled StoreKeyWF category false, so
  the selected shape is false.
- The model deliberately keeps the five no-live tags independent from the
  corresponding no-resurrection tags. It therefore does not identify or derive
  the two vocabularies.
- Trusted Lean execution passed. The source scan found no `sorry`, `admit`,
  declared axiom, `opaque`, `unsafe`, `partial`, or `implemented_by`. `#print
  axioms` reports Lean `propext` and `Quot.sound` only for the consistency-based
  theorems; the modeled well-formedness falsifier and final bad-result theorem
  have no axioms. Scratch hash:
  `0c4973aa465d18a2e0a5cabeca86b33affcf29be7690b54b6710c8b2d3c275e8`.
- Before broad validation, the root filesystem had 21 GB free (89% used) and
  the system reported about 9.5 GB available memory. The package adds no tracked
  build artifact and keeps the Lean model under `/tmp`.

## What changed in understanding

The source's necessary conditions constrain a successful load but do not by
themselves select what the successful restored result is. The proof-facing gap
is not merely a field list or a single liveness predicate: successful-load
recognition, restored configuration, restored prefix, and policy-to-result
interpretation must be chosen coherently. This can be recorded without choosing
any of them.

## Open questions

- What canonical relation or result discipline represents successful load?
- How are a saved cut/frontier and restored history prefix projected and related?
- What connects no-resurrection conditions to restored-state no-live predicates?
- Which full Config and liveness/provenance interpretations belong in a future
  proof-facing OBL-009 statement?

## Suggested next prompt

Select another independent existing-lane source cut, or prepare an owner/canon
formalization decision only when a proof-facing THM-003 / OBL-009 package needs
to select a restoration interface.

## Plan update status

Updated: plan/156 records the direct anchors separately from the source-
adequacy result, the corrected twin, the coupled missing boundary, stop
threshold, and non-claims.

## Documentation.md update status

`Documentation.md` update unnecessary: reader entry points did not change.

## docs/project-status.md update status

更新済み: the human view now distinguishes fixed load policy anchors from the
missing successful-load restoration interpretation.

## progress.md update status

Updated: current research, the remaining statement-boundary row, and the dated
recent log include T-RESEARCH-014.

## tasks.md update status

Updated: T-RESEARCH-014 is closed as LAB source-adequacy evidence and the next
selection excludes silently choosing its load/restoration interface.

## samples_progress.md update status

`samples_progress.md` update unnecessary: no runnable sample changed.

## Reviewer findings and follow-up

The temporary selection review ranked OBL-009 first and rejected the
hot-plug, stream, and diagnostic candidates for this unit. The first exact-file
review found five material defects: unary instead of result-sensitive load
conditions, schema counted as a policy premise, unshared consistency vocabulary,
omitted result-side no-live clauses, and an inaccurate source-adequacy count.
All were corrected. The completed corrected exact-file review confirmed the
`0 direct / 0 delegated / 1 missing` row, the separate `1 + 8 + 1` source-
anchor inventory, the StoreKeyWF-only twin characterization, and the
research-complete stop line. Oracle could not rerun Lean; local trusted Lean was
rerun after the corrections. No local sub-agent service was available.

## Skipped validations and reasons

Runtime, distributed persistence, conformance, and product checks do not apply
to this documentation and disposable-Lean source audit. The runnable sample
corpus was not rerun because no sample, runner, or implementation source
changed.

## Commit / push status

The final package commit uses `--no-gpg-sign` and is pushed immediately after
this status record is included.

## Sub-agent session close status

No local sub-agent service was available. The completed temporary Oracle
selection and exact-file reviews were advisory and checked against the canon
source cut.
