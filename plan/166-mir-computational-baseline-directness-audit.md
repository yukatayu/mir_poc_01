# plan/166 - Mir computational baseline directness audit

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
language direction, grammar, effects, contracts, Gates, Phases, conformance,
and implementation authority. This audit changes none of them.

## Question

What part of the current Product Alpha computational sample matrix is executed
by the Product Alpha Rust runtime, what part is helper-local classification,
and what does that distinction permit us to claim about a minimum Mir
computation and typechecking floor?

## Reproduced evidence

On 2026-07-22, `scripts/mir_computational_samples.py matrix --format json` and
`check-all --format json` reported all 15 rows as matching their declared
outcomes: 7 accepted, 5 expected runtime rejections, and 3 expected package
check rejections. Focused commands also reproduced the two direct acceptances
as `Int(42)` and the undeclared-effect package rejection as `SchemaDecode`.

The focused Rust suites passed:

- `mir-semantics` computational core: 4 tests;
- Product Alpha package schema: 32 tests;
- Product Alpha session runtime: 29 tests.

## Execution classification

| Matrix portion | Actual execution path | Evidence classification |
| --- | --- | --- |
| `comp-02-pure-add-one` | helper -> `mirrorea-cli run-local` -> Product Alpha session -> Rust semantic evaluator | direct bounded package-runtime acceptance |
| positive `comp-04` | helper -> `mirrorea-cli run-local` -> Product Alpha session -> Rust semantic evaluator | direct bounded host-boundary package-runtime acceptance |
| negative `comp-04` | helper -> `mirrorea-cli check` -> Product Alpha package validation | direct package-check rejection |
| checked-in `comp-03` fixtures | Python `module_id` dispatcher in `mir_computational_samples.py` | helper-only fixture acceptance/rejection classification |
| closed `comp-03` registry | constructed valid packages in Product Alpha runtime tests | direct package-runtime acceptance/rejection test evidence |
| closed `comp-03` registry | `mir-semantics` unit tests | direct semantic typecheck/evaluate test evidence |

The direct CLI probe rejects a representative `.mir` file with
`direct_mir_non_goal` (exit 2). A direct `run-local` probe of a `comp-03`
package fails its current package shape at schema checking: `request_payload`
is not a Product Alpha runtime-input field. These are deliberate current
boundaries, not evidence that textual grammar or a general package model has
been finalized.

## Consequence and stop line

The accurate present claim is: the repository has reproducible, bounded
package-level sample-fixture evidence for two selected Mir computation paths
and explicit host-boundary declaration checks. It separately has direct runtime
test evidence for a closed broader first-floor registry, plus helper fixture
classification. It does not have a final textual Mir grammar, a general effect
system, a general source-to-package executor, a public ABI, or a completed
distributed runtime.

The current moratorium in ADR-0014 and canon plan/02 permits an existing-lane,
non-production fixture experiment only when it introduces no helper, schema,
CI/Make surface, evidence lane, public interface, or production runtime
implementation. An expansion with any of those reserved effects requires
owner/canon action.
This audit does not select Oracle's optional computational-boundary-confidence
implementation suggestion.

## References

- `specs/28-mir-computational-core.md`
- `plan/53-mir-computational-core-roadmap.md`
- `samples/product-alpha1/computational/README.md`
- `samples_progress.md`
- `docs/reports/2327-mir-computational-baseline-directness-audit.md`
