# plan/170 - post-WRK-0011 candidate selection

## Role and authority

This is LAB repository memory. `mirrorea_canon/` remains authoritative for
theory, OBL status, Gates, Phases, contracts, conformance, and semantic
selection. This selection does not create `WRK-0012`, change a Canon statement,
proof, implementation, or public claim, or authorize an evidence command.

## Exact cut and correction

At clean `main` `0969a52cdfa139e3f7b10beece4f0a40feffec87`, the standing
boundary in ADR-0014 and the working annex; the prior Lean selection records;
WRK-0007 through WRK-0011; the current-L2 and computational audit memory; and
the current task/progress snapshots were reread. A local planner review, a
Lean-lane/source explorer review, and a temporary Oracle advisory review were
used to challenge the selection.

The first draft of this disposition incorrectly treated roots permitted by
earlier WRK records as a permanent global whitelist and consequently rejected
fixture changes categorically. ADR-0014 and the working annex instead require
each *new* record to declare its own existing documented LAB lane(s). They
allow bounded non-production source or test changes in that declared lane;
they still prohibit a new helper family, schema, CI/Make target, evidence lane,
public interface, or production implementation. This corrected selection uses
that standing rule. No candidate command was run before pre-registration.

## Candidate comparison

| Candidate family | Disposition | Reason |
| --- | --- | --- |
| P-COMP-03 direct-carrier fixture cut | **selected for next L3 pre-registration** | Two fixed helper-only rows are proposed to be re-encoded as bounded non-production sidecar packages in their existing Product Alpha row directories, then checked through the already-existing Product Alpha schema and `mirrorea-cli check` / `run-local` route. Whether that works is the experiment's falsifiable question, not a premise. This is a different question from the prior audit's helper/direct classification. |
| Surface patch or ELAB source-span literal parity | reserve | An existing literal audit is possible, but it primarily refines source-attribution evidence already represented by current-L2 studies. It has lower direct execution value than the selected carrier cut. |
| Full System / Surface cross-lane operational literal parity | conditional reserve | It needs an exact already-existing shared key and crosswalk. None was identified in this screen. A later record may declare its own documented lane if that concrete key is found. |
| clean-near-end forwarding parity | reserve | The present evidence has no identified behavioral divergence or direct carrier question, so a new record would have low discriminating value. |
| further current-L2 / Lean assertion variants | reserve or duplicate | These either broaden WRK-0007--WRK-0011 by coverage/field only or require a carrier, mapping, or interpretation selection. |

The selected cut has priority because it tests an existing executable carrier
against one fixed accepted and one fixed rejected computational row without
claiming that all computational fixtures, `.mir` source files, or the language
as a whole are directly executable.

## Selected next-record shape

The next package should pre-register a working record tentatively named
`WRK-0012-pcomp03-direct-carrier.md`; this plan is **not** that registration.

| Field | Proposed bounded content |
| --- | --- |
| Question | Can one fixed accepted P-COMP-03 row and one fixed rejected P-COMP-03 row be executed through only the existing Product Alpha package schema's `runtime_input.mir_compute` and existing `mirrorea-cli check` / `run-local` path? |
| Status quo | The checked-in P-COMP-03 rows are `computational_helper_row` fixtures dispatched by Python. The same closed registry is directly exercised only by Rust tests that construct valid packages; the checked-in P-COMP-03 manifests do not themselves satisfy the direct package shape. |
| Alternative | Bounded non-production sidecar manifests in the two existing row directories can instantiate the existing `world`-package `MirCompute` carrier without changing schema, helper, runtime, or CLI code. |
| Positive row | `samples/product-alpha1/computational/control-flow/positive/` (`comp-03-control-flow-positive`). |
| Adverse row | `samples/product-alpha1/computational/variables-scope/negative/` (`comp-03-variables-scope-negative`). |
| Expected falsifier | The existing schema cannot express the fixed module/function invocation; a validated sidecar does not execute the named computation; the positive/adverse result does not agree with the closed registry's accepted/rejected classification; or execution needs a helper/schema/runtime/CLI/public-carrier change. |
| Existing machinery | `python3 scripts/mir_computational_samples.py matrix --format json`, its `check-all` command, and existing `cargo run -q -p mirrorea-cli -- check` / `run-local` commands. Source scripts and Rust crates are inspection/execution machinery only unless a later record explicitly retains a permitted LAB change there. |

The future registration must declare permitted LAB locations before any outcome
command. Its planned registration scope is `plan` plus the existing
`samples/product-alpha1/computational` lane, so the matrix README and the two
fixed rows can be pinned as LAB inputs; its **retained source changes** must be
only non-production sidecar fixture material in the two listed row directories.
The matrix script and Rust crates may be pinned unmodified execution machinery,
as in WRK-0011; they are neither retained LAB inputs nor artifacts. It must not
silently generalize the retained fixture change to the other eight P-COMP-03
rows.

## Stop line and non-claims

Stop and report rather than adapt the experiment if it needs a helper/schema/
runner/CLI/runtime/public-interface change, a new schema or fixture family, a
new evidence lane, a production implementation, a final grammar/effect
decision, or a Phase/Gate/OBL/conformance claim. Passing two fixed sidecars
would be L3 evidence about this direct-carrier cut only. It would not establish
general direct Mir execution, fixture parity, a public Product Alpha interface,
or a Canon-level runtime result.

## Future re-triage

This selection is a current priority order, not a restriction on the standing
predicate. Re-triage when a concrete distinct existing-lane question has an
explicit status quo, alternative, falsifier, permitted locations, and stop
line. The priority signals above help choose work, but they do not make other
eligible future candidates ineligible. Every such candidate still requires a
committed pre-registration before its evidence commands run.
