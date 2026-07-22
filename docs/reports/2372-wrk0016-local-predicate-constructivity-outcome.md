# Report 2372 - WRK-0016 local-predicate constructivity outcome

- Date: 2026-07-23 00:00 JST
- Author / agent: Codex
- Scope: registered Lean outcome capture before working-record manifestation
- Decision levels touched: none; this is a direct LAB command record, not a Canon or OBL result

## Objective

Execute the pushed WRK-0016 command plan far enough to determine whether the
registered named non-instance `Decidable (captureSubset lhs rhs)` target can be
retained without a new definition, generic carrier, global instance, or
classical machinery.

## Scope and assumptions

WRK-0016 at pushed registration `0d56c1d3` is the sole authority for this
experiment. The Lean source was temporarily edited only for red/green testing
and has been restored byte-for-byte to the pinned pre-experiment content before
this report is committed. This report captures the direct command output; the
next commit alone may manifest it by updating the working record to `frozen`.

## Start state / dirty state

The registration and its report correction were pushed at `ef2d8e55`. The
worktree was clean before the registered outcome commands. A new Discord
baseline was recorded after the registration package closed.

## Documents consulted

Read WRK-0016, plan 173, the exact current-L2 Lean foundation, its explanation,
the working-record history validator, and the existing report template.

## Actions taken

Ran the pre-existing-name red check, recorded Lean 4.29.1, and replayed the
unmodified foundation successfully. Added three minimal red examples that
referenced the registered future names, observed the expected unknown-name
errors, then attempted only the declared local terms. After the decisive
declaration-kind failure, restored the source instead of introducing a
forbidden definition. Replayed the restored source and the registered opaque
arbitrary-domain adverse probe.

## Files changed

- this report

## Commands run

- registered pre-existing-name red check for the marker and three names
- `lean --version`
- `lean --trust=0 samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`
  before the trial and after source restoration
- minimal red examples referring to each registered future name
- minimal attempted local theorem declarations for the three target decision
  values
- registered temporary opaque-domain adverse probe under `lean --trust=0`
- post-restoration pre-existing-name red check

## Evidence / outputs / test results

The initial red check passed, the installed compiler reported
`Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit
f72c35b3f637c8c6571d353742168ab66cc22c00, Release)`, and the unmodified
foundation compiled with `lean --trust=0`.

The three red examples failed solely because their candidate names were absent:

```text
error(lean.unknownIdentifier): Unknown identifier `outlives_decidable_control`
error(lean.unknownIdentifier): Unknown identifier `capture_subset_decidable_constructive`
error(lean.unknownIdentifier): Unknown identifier `remote_call_allowed_decidable_control`
```

The registered green attempt then failed decisively before any proof could be
accepted. Lean reports that each proposed `theorem` has a non-proposition type,
including:

```text
error: type of theorem
`CurrentL2FiniteIndexFirstLayer.capture_subset_decidable_constructive`
is not a proposition
  (lhs rhs : CaptureSet) → Decidable (captureSubset lhs rhs)
```

The two control declarations fail for the same reason. The attempted body also
emitted two name-shadowing application errors in unreachable fallback branches;
they are not relied on, because the declaration-kind error independently stops
the experiment before any body can define a named `Decidable` value. A named
value of this type requires a value-defining declaration such as `def` (or an
equivalent new declaration form), which WRK-0016 explicitly forbids.

The source was restored, its final name-absence check passed, and it compiled
again. The registered opaque-domain adverse probe returned the expected
nonzero Lean result containing `Decidable`; no generic decision was inferred.

## What changed in understanding

The two-constructor carrier is not the first blocker for the selected question.
Lean separates proof declarations (`theorem`, whose target must be `Prop`) from
named data values such as `Decidable p`. Therefore the requested named
non-instance form is structurally incompatible with the record's no-new-
definition condition. This is a method-boundary result, not evidence that the
predicate itself is constructively undecidable or that Mir needs a decision
primitive.

## Open questions

- A future, separately registered question could ask whether a local unnamed
  `example` or a restricted value declaration is useful, but neither is
  authorized by WRK-0016.
- Any general decision interface, typeclass policy, or relationship to MirCore
  remains outside this result and requires a new scope decision.

## Suggested next prompt

Manifest this exact report commit as WRK-0016 frozen evidence, verify the
working-record history, and then re-evaluate theory candidates without reviving
the excluded definition-based route.

## Plan update status

plan 更新不要: plan 173 is a pinned pre-registration input. This outcome does
not revise its selection or add a future route.

## Documentation.md update status

Documentation.md 更新不要: no reader-facing capability, runnable workflow,
or current plan link changed before working-record manifestation.

## docs/project-status.md update status

更新不要: this report captures an unmanifested direct command outcome; the
current status must change only with the subsequent frozen-record commit.

## progress.md update status

progress.md 更新不要: no current-state conclusion is published until the
working record carries the append-only evidence commit.

## tasks.md update status

tasks.md 更新不要: package 43 remains the registered state until its frozen
manifest is committed.

## samples_progress.md update status

samples_progress.md 更新不要: the source was restored; no runnable sample,
command, dashboard row, or workflow readiness changed.

## Reviewer findings and follow-up

No independent reviewer was required for this L3 outcome. The stopping rule is
directly the registered one: a new value definition is required for a named
`Decidable` target. The next package should review only whether the record
faithfully reports this command output and freezes the route; it must not
repair the proof or broaden the language surface.

## Skipped validations and reasons

No source-tail lexical audit, sample sync, runtime, distributed, or product
suite ran. The lexical audit assumes a retained source tail with three named
terms, but the decisive registered falsifier requires that tail to be removed.
Sample sync is unnecessary because the final source is restored. Broader
runtime and distributed checks do not exercise this helper-local Lean outcome.

## Commit / push status

Pending at report write. This direct outcome report will be committed and
pushed before its full commit hash is append-only manifested in WRK-0016.

## Sub-agent session close status

No new sub-agent was opened for the direct compiler outcome. The candidate
selection's completed planner, adversarial, and Oracle reviews remain advisory
input to the already-pushed registration.
