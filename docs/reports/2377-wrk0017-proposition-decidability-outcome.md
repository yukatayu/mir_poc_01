# Report 2377 - WRK-0017 proposition-decidability outcome

- Date: 2026-07-23 01:11 JST
- Author / agent: Codex
- Scope: registered Lean outcome capture before working-record manifestation
- Decision levels touched: none; this is direct LAB evidence, not a Canon or OBL result

## Objective

Execute the pushed WRK-0017 plan far enough to determine whether its exact
proposition theorem and opaque-domain adverse control jointly establish the
registered explicit closed-carrier constructivity boundary.

## Scope and assumptions

WRK-0017 at pushed registration `292ef96b` is the sole authority for this
experiment. The local theorem source was temporary and has been restored
byte-for-byte before this report is committed. This report captures direct
command output; the subsequent manifest may freeze the record but must not
repair its generic-control rule.

## Start state / dirty state

The registration package was pushed at `292ef96b`. The worktree was clean
before outcome execution, and a Discord baseline was recorded after that
package closed.

## Documents consulted

Read WRK-0017, plan 174, the exact Lean foundation, Reports 2374 through 2376,
the working-record history validator, and the report template.

## Actions taken

Ran the registered name-absence check, compiler/version baseline, and source
compile. Used a disposable red test to confirm the future theorem name was
absent. Added the registered proposition theorem, confirmed both the foundation
and disposable green test compile, ran the registered lexical audit, and then
ran the opaque-domain adverse probe. That probe unexpectedly compiled. A
diagnostic `#print axioms` comparison showed the generic proof used classical
axioms while the local theorem did not. Restored the source and recompiled it.

## Files changed

- this report

## Commands run

- registered pre-existing-name check, `lean --version`, and baseline
  `lean --trust=0` foundation compile
- disposable red and green theorem-name tests
- registered local proposition-theorem compile and lexical tail audit
- registered opaque generic-domain adverse probe
- disposable generic/local `#print axioms` diagnostics
- post-restoration foundation compile and name-absence check

## Evidence / outputs / test results

The baseline compiled under Lean 4.29.1. The disposable red test failed solely
on the absent theorem name. The temporary local theorem
`capture_subset_excluded_middle_constructive` compiled, its green reference
test compiled, and the registered tail audit found none of its forbidden
tokens.

The registered opaque-domain probe did **not** fail as its command required;
the Python assertion therefore raised `AssertionError`. The diagnostic generic
theorem compiled and reported:

```text
'generic_excluded_middle' depends on axioms: [propext, Classical.choice, Quot.sound]
```

By contrast, the temporary exact local theorem reported:

```text
'CurrentL2FiniteIndexFirstLayer.capture_subset_excluded_middle_constructive'
does not depend on any axioms
```

Thus `by_cases` in the generic temporary theorem silently obtained a classical
decision. The generic control's success without an explicit finite interface is
an exact WRK-0017 falsifier, even though the local theorem itself happened to
be constructive. The source was restored and again compiled; the registered
marker/name-absence check passed.

## What changed in understanding

The local theorem body and the generic adverse control require separate
constructivity evidence. A lexical ban on `Classical` is insufficient because
Lean tactics can introduce classical axioms without that token appearing in the
source. The only result of this route is that its original generic-control
criterion is unsound for the intended distinction. It does not establish a
local theorem as retained evidence, a generic theorem, undecidability, or any
MirCore/checker/OBL claim.

## Open questions

- Can a future successor use axiom-profile checks to distinguish a local
  theorem with no classical axioms from a generic theorem that depends on them?
- Would such a successor be distinct and decision-relevant rather than a
  repair of this frozen route?
- Does a future proof-level consumer need the local constructive body or a
  failure witness?

## Suggested next prompt

Manifest this direct outcome as WRK-0017 frozen evidence, then independently
review whether an axiom-profile-controlled successor is justified before any
new theorem body is retained.

## Plan update status

plan 更新不要: plan 174 remains the immutable selection/pre-registration
input. The outcome falsifies its registered route and does not prescribe a
successor.

## Documentation.md update status

Documentation.md 更新不要: the current reader map must change only when the
working record has append-only manifested this outcome.

## docs/project-status.md update status

更新不要: this report records an unmanifested direct outcome; current status
changes only with the subsequent frozen-record commit.

## progress.md update status

progress.md 更新不要: no published state changes until the record carries the
append-only evidence commit and frozen reliance status.

## tasks.md update status

tasks.md 更新不要: package 45 remains registration-only until the frozen
manifest is committed.

## samples_progress.md update status

samples_progress.md 更新不要: the temporary theorem was removed, so no runnable
sample, command, dashboard row, or workflow readiness changed.

## Reviewer findings and follow-up

No new independent reviewer was required for this direct L3 outcome. The
registered adverse-control failure is decisive. A subsequent review must judge
only the frozen interpretation and whether a distinct axiom-profile successor
has enough decision value; it must not retain or repair the removed theorem.

## Skipped validations and reasons

No source-tail evidence retention, sample sync, runtime suite, distributed
suite, or Oracle request ran after the control falsifier. The theorem source
was restored immediately as required. Broader runtime and distributed checks do
not exercise this helper-local Lean result.

## Commit / push status

Pending at report write. This direct outcome report will be committed and
pushed before its full commit hash is append-only manifested in WRK-0017.

## Sub-agent session close status

No new sub-agent was opened for the direct compiler outcome. The predecessor
selection package's planner and reviewer are closed; their results do not alter
the already-pushed WRK-0017 registration.
