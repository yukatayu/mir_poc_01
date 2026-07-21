# Report 2327 - Mir computational baseline directness audit

- Date: 2026-07-22 03:35 JST
- Author / agent: Codex
- Scope: existing Product Alpha computational evidence classification and
  current-status synchronization
- Decision levels touched: none; Canon and runtime implementation are read-only

## Objective

Reproduce the executable Mir computational baseline and distinguish direct
Product Alpha Rust execution, package-check rejection, helper-local behavior,
and textual-source non-goals without making a grammar, runtime-completion, or
public-product claim.

## Scope and assumptions

`mirrorea_canon/` is normative. The current moratorium permits existing-lane
research and bounded implementation validation but reserves new helper,
schema, and production-implementation changes for owner/canon action. This is
a read-only execution and source-mapping audit; it adds LAB memory, snapshot
clarification, and the required registration entries for its numbered plan.

## Start state / dirty state

Started from clean `main` at pushed `38be0d0c`, after post-WRK-0007 candidate
selection. No user changes were present or reverted.

## Documents consulted

- Canon README/MAP, ADR-0014, and plan/02 operating model.
- `specs/28`, `plan/53`, and the computational sample README/matrix.
- `scripts/mir_computational_samples.py`, Product Alpha CLI/schema/runtime,
  and computational semantic-core source as mapped by a read-only sub-agent.
- `Documentation.md`, `docs/project-status.md`, `progress.md`, `tasks.md`,
  `samples_progress.md`, and the Oracle operating notes.

## Actions taken

1. Replayed the matrix, full computational helper check, representative direct
   acceptance/rejection commands, focused Python tests, and focused Rust tests.
2. Probed direct textual `.mir` input and a `P-COMP-03` package through the
   Product Alpha CLI to establish their current rejection boundaries.
3. Obtained a read-only source-path map and a temporary Oracle review.
4. Recorded the direct/helper split and the moratorium stop line in LAB memory
   and reader-facing snapshots.

## Files changed

- `plan/166-mir-computational-baseline-directness-audit.md`
- `plan/00-index.md`
- `plan/53-mir-computational-core-roadmap.md`
- `Documentation.md`
- `docs/project-status.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `scripts/check_source_hierarchy.py`
- `scripts/validate_docs.py`
- this report

## Commands run

- `python3 scripts/mir_computational_samples.py matrix --format json`
- `python3 scripts/mir_computational_samples.py check-all --format json`
- direct `comp-02`, positive `comp-04`, and negative undeclared-effect helper
  runs
- `python3 -m unittest scripts.tests.test_mir_computational_samples`
- `cargo test -p mir-semantics --test mir_computational_core -- --nocapture`
- `cargo test -p mir-ast --test product_alpha1_package_schema -- --nocapture`
- `cargo test -p mir-runtime --test product_alpha1_session -- --nocapture`
- focused positive/negative `P-COMP-03` Product Alpha runtime test commands
- direct CLI probes for representative `.mir` and `P-COMP-03` inputs
- resource check with `df -h .` and `free -h`
- temporary Oracle consultation with computational README/matrix/dashboard and
  prior computational reports attached
- `make check`
- `python3 -m unittest scripts.tests.test_validate_docs`

## Evidence / outputs / test results

The matrix reported 15 executable rows: 7 accepted, 5 expected runtime
rejections, and 3 expected check rejections; `check-all` had no failures.
`comp-02` and positive `comp-04` produced `Int(42)` with ordered
`host_input_received`, `mir_compute_step`, and `host_output_emitted` events.
The negative effect package rejected with `SchemaDecode` because its output
effect is undeclared.

The focused Python test suite passed 17 tests. Rust semantic, schema, and
runtime suites passed 4, 32, and 29 tests respectively. The source map shows
that checked-in `comp-03` fixtures use Python's module-ID dispatcher and do
not invoke the Rust runtime through their fixture path. The Rust runtime suite
also constructs valid Product Alpha packages, executes all five positive
closed-registry `comp-03` modules, and directly rejects all five negative
modules before evaluator execution; the semantic suite typechecks and evaluates
the registered module forms. Direct `.mir` input returned
`direct_mir_non_goal` with exit 2 from Product Alpha `check`; direct `comp-03`
`run-local` returned its expected package-shape error with exit 2.

After documentation synchronization, `make check` passed Canon index,
source-hierarchy (`716` required paths), documentation validation (`1481`
numbered reports), and `cargo check`. The full documentation regression suite
passed 83 tests in 282.909 seconds.

After reviewer correction, the focused direct runtime commands independently
passed the positive and negative `P-COMP-03` registry tests, one test each.

## What changed in understanding

The existing matrix is internally consistent but has two evidence mechanisms.
Its direct runtime evidence is narrower than its `executable` matrix count,
while a separate runtime suite directly covers the broader closed registry with
constructed valid packages. The correct claim is bounded package/runtime and
host-boundary sample evidence plus helper-fixture, semantic-test, and
closed-registry runtime-test evidence, not general direct textual Mir program
execution.

## Open questions

- Whether a direct computational confidence fixture can remain inside the
  existing non-production lane without reserved changes, or needs a new
  helper, schema, CI/Make surface, production runtime implementation, or public
  contract.
- How a future textual grammar and general package execution should connect;
  this remains distinct from the current direct `package.mir.json` path.

## Suggested next prompt

Continue autonomous existing-lane research on the fixture/runtime-test
correspondence, or escalate before any direct computational package requires a
new helper, schema, CI/Make surface, production runtime implementation, or public
contract.

## Plan update status

`plan/` 更新済み: added plan/166, indexed it, and clarified the direct/helper
split in plan/53.

## Documentation.md update status

`Documentation.md` 更新済み: records the bounded computational evidence and
points to plan/166.

## docs/project-status.md update status

更新済み: adds a separate current-status row for direct, helper-only, and
package-check computational evidence.

## progress.md update status

`progress.md` 更新済み: records the implementation classification and a dated
recent-log entry without changing phase or workflow status.

## tasks.md update status

`tasks.md` 更新済み: closes the audit and identifies the moratorium as the
boundary for any widening with reserved implementation effects.

## samples_progress.md update status

`samples_progress.md` 更新済み: distinguishes direct runtime, helper-only, and
direct check-rejection rows and adds the reproducibility result.

## Reviewer findings and follow-up

Read-only sub-agent `Confucius` traced the direct CLI/session/semantic path,
the direct check path, and the helper-only `comp-03` dispatcher. A temporary
Oracle review independently recommended the same narrow evidence classification
and cautioned against claiming final grammar, general effects, or public
product capability. Final reviewer `Beauvoir` found the separate direct
closed-registry runtime tests and corrected the moratorium and CLI scope:
checked-in helper fixtures are not the whole runtime evidence, and only an
expansion with a reserved effect needs owner/canon action. Its re-review after
those corrections found no remaining issue. The advisory output is not
committed.

## Skipped validations and reasons

No full release workflow, Docker flow, workspace-wide test sweep, Lean replay,
or heavy backend build was rerun: this package changes only LAB documentation
and report memory after focused computational reproduction. Documentation and
source-hierarchy validation were run; the omitted commands would not validate
any changed runtime behavior.

## Commit / push status

Pending at report write; the final documentation package will be committed with
`--no-gpg-sign` and pushed before continuing.

## Sub-agent session close status

`019f85f2-e22b-7bd2-8f12-b53ec72a2d60` (`Confucius`) completed its read-only
mapping and was closed. The temporary Oracle consultation completed; its
output remains external advisory input. `019f8601-5c72-7e73-a162-bc36daa3646e`
(`Beauvoir`) completed the final review and re-review and was closed.
