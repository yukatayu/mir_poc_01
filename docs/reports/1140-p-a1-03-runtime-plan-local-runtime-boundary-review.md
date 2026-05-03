# Report 1140 — P-A1-03 Runtime-Plan / Local-Runtime Boundary Review

- Date: 2026-05-03 16:43:17 JST
- Author / agent: Codex
- Scope: theory/spec boundary review for `P-A1-03` after `P-A1-02`, focused on overclaim risk, kept-later boundaries, and the minimum honest carrier split between checker report, runtime plan, and local runtime report
- Decision levels touched: L1, L2

## Objective

Review `P-A1-03` from a maintainer/spec-editor perspective and identify concrete boundary findings before implementation widens the practical alpha-1 line from checker floor into runtime-plan/local-runtime execution.

## Scope and assumptions

- This task is review-only; no implementation, spec wording, or roadmap wording is changed here.
- The review uses the named alpha-1 handoffs plus the minimum required repo orientation sequence and the current practical checker/runtime code carriers.
- The goal is not to design the whole `P-A1-03` implementation, but to state what must not be collapsed semantically.

## Start state / dirty state

- Start state: worktree clean.
- Dirty state during task: none introduced before report creation.

## Documents consulted

- `sub-agent-pro/alpha-1/03-decisions.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`
- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `crates/mir-ast/src/practical_alpha1.rs`
- `crates/mir-ast/src/practical_alpha1_checker.rs`
- `scripts/practical_alpha1_check.py`
- `crates/mir-runtime/src/alpha_local_runtime.rs`
- `scripts/alpha_local_runtime_samples.py`
- `crates/mir-runtime/src/hotplug_runtime.rs`
- `crates/mirrorea-core/src/runtime.rs`
- `crates/mirrorea-core/src/fabric.rs`

## Actions taken

1. Read the user-named alpha-1 handoff documents in the requested order.
2. Read the required repo orientation docs and relevant practical alpha-1 spec/plan/report slice.
3. Inspected the practical front-door/checker carriers in `mir-ast` and the existing alpha-local runtime/report carriers in `mir-runtime` and `mirrorea-core`.
4. Compared those carriers against `specs/18` and `plan/44` to derive boundary findings for `P-A1-03`.

## Files changed

- `docs/reports/1140-p-a1-03-runtime-plan-local-runtime-boundary-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,260p' /home/yukatayu/dev/mir_poc_01/.agents/skills/discord-report/SKILL.md
sed -n '1,260p' /home/yukatayu/.codex/skills/superpowers/skills/receiving-code-review/SKILL.md
rg --files sub-agent-pro | rg 'alpha-1'
sed -n '1,220p' sub-agent-pro/alpha-1/03-decisions.md
sed -n '1,220p' sub-agent-pro/alpha-1/04-stage-roadmap.md
sed -n '1,220p' sub-agent-pro/alpha-1/06-toolchain-architecture.md
sed -n '1,220p' sub-agent-pro/alpha-1/09-validation-plan.md
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' plan/00-index.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/18-practical-alpha1-scope.md
sed -n '1,300p' plan/44-practical-alpha1-roadmap.md
sed -n '1,260p' docs/reports/1139-p-a1-02-typed-ir-checker-first-floor.md
rg -n "practical_alpha1|runtime_plan|accepted_obligations|rejected_rows|run_local_claimed|runtime_plan_emitted|event DAG|event_dag|LogicalPlaceRuntimeShell|MembershipRegistry|PlaceCatalog|MessageEnvelope|AuthEvidence" crates/mir-ast crates/mir-runtime scripts samples/practical-alpha1 -g '!target'
nl -ba crates/mir-ast/src/practical_alpha1.rs | sed -n '1,260p'
nl -ba crates/mir-ast/src/practical_alpha1_checker.rs | sed -n '1,940p'
nl -ba crates/mir-runtime/src/alpha_local_runtime.rs | sed -n '1,980p'
nl -ba crates/mir-runtime/src/hotplug_runtime.rs | sed -n '1,260p'
nl -ba crates/mirrorea-core/src/runtime.rs | sed -n '1,420p'
nl -ba crates/mirrorea-core/src/fabric.rs | sed -n '1,320p'
nl -ba scripts/practical_alpha1_check.py | sed -n '1,260p'
nl -ba scripts/alpha_local_runtime_samples.py | sed -n '1,340p'
nl -ba samples/practical-alpha1/README.md | sed -n '1,220p'
nl -ba scripts/README.md | sed -n '1,120p'
sed -n '1,260p' docs/reports/TEMPLATE.md
git status --short
date '+%Y-%m-%d %H:%M:%S %Z'
ls docs/reports | rg '^[0-9]+' | sort | tail -n 5
```

## Evidence / outputs / test results

- Review evidence is textual/code inspection only.
- No runtime or checker validations were run because this task did not change implementation and the request was for boundary findings, not a closeout claim.
- Key evidence anchors:
  - `specs/18-practical-alpha1-scope.md` requires `checker -> runtime plan -> local runtime` as separate path elements and states `P-A1-02` does not emit runtime plans.
  - `plan/44-practical-alpha1-roadmap.md` says `P-A1-03` is safe only if checked packages are consumed through a distinct runtime-plan boundary.
  - `crates/mir-ast/src/practical_alpha1_checker.rs` encodes explicit stop lines that the checker report is not a runtime plan and does not claim local/Docker execution.
  - `crates/mir-runtime/src/alpha_local_runtime.rs` is a sample-ID keyed Stage-B carrier with explicit non-public notes and retained-later refs.

## What changed in understanding

- The decisive issue for `P-A1-03` is not whether local execution can be made to work, but whether execution can start from a carrier that is narrower than the checker report and more general than the Stage-B sample-ID runner.
- Existing `mirrorea-core` carriers are sufficient substrate for a first local-runtime execution floor, but not sufficient reason to collapse plan semantics into the runtime report itself.

## Open questions

- Whether `P-A1-03` should introduce an explicit serializable runtime-plan struct in `mir-ast` or a runtime-facing module shared with `mir-runtime` remains open.
- Whether the first `P-A1-03` runtime-plan floor should reject packages lacking execution sections, or permit a narrow default mapping from `world`/`places`/`layers`, remains open.

## Suggested next prompt

Define the `P-A1-03` runtime-plan carrier explicitly, including which checker outputs are admissible inputs, which fields belong only to runtime execution reports, and which Stage-B sample-ID/runtime-report fields are prohibited from becoming practical runtime-plan fields.

## Plan update status

`plan/` 更新不要: this task is a review of the existing boundary, not a roadmap decision change.

## Documentation.md update status

`Documentation.md` 更新不要: no new repo state or public-facing status changed.

## progress.md update status

`progress.md` 更新不要: no package was implemented or closed.

## tasks.md update status

`tasks.md` 更新不要: the promoted queue remains `P-A1-03`.

## samples_progress.md update status

`samples_progress.md` 更新不要: no runnable sample status changed.

## Reviewer findings and follow-up

- Finding 1:
  `P-A1-03` cannot honestly consume `PracticalAlpha1CheckReport` directly as its runtime input carrier. `specs/18-practical-alpha1-scope.md` fixes `checker -> runtime plan -> local runtime` as separate stages, `plan/44-practical-alpha1-roadmap.md` repeats that `P-A1-03` must use a distinct runtime-plan boundary, and `crates/mir-ast/src/practical_alpha1_checker.rs` embeds stop lines stating the checker report is not a runtime plan. If implementation feeds `accepted_obligations` / `canonical_fallback_chains` / `package_admission_report` straight into execution, the repo will silently collapse proof/report surface into plan semantics and overclaim `P-A1-02` as already having plan emission.
- Finding 2:
  The existing `alpha_local_runtime` report is the wrong carrier to promote as the practical local-runtime report without an intervening plan layer. `crates/mir-runtime/src/alpha_local_runtime.rs` is explicitly sample-ID keyed (`LR-01`, `LR-02`), hard-codes envelope contents and event ids, and labels itself `alpha_local_runtime_stage_b_narrow` with notes that it is a non-public in-memory floor. Reusing that report shape as `P-A1-03` output would falsely imply that a practical package was executed, when the current code actually executes a fixed Sugoroku scenario independent of `samples/practical-alpha1/`.
- Finding 3:
  The minimum honest split is three carriers, not two: `checker report`, `runtime plan`, and `local runtime report`. The current checker report already contains normative judgment evidence and kept-later markers (`accepted_obligations`, `rejected_rows`, `runtime_plan_emitted: false`, `run_local_claimed: false`) in [crates/mir-ast/src/practical_alpha1_checker.rs]. The current local runtime report contains operational evidence (`runtime_snapshot`, `message_envelopes`, `dispatch_records`, `event_dag`) in [crates/mir-runtime/src/alpha_local_runtime.rs]. `P-A1-03` therefore needs a middle carrier that contains only execution-admissible structure derived from a checked package, not diagnostics and not post-execution traces. Without that split, either the checker report becomes execution state or the runtime report becomes the only “plan,” both of which violate the spec boundary.
- Finding 4:
  `P-A1-03` must keep several later boundaries explicit even if it reuses `mirrorea-core` substrates. `MessageEnvelope` and `LogicalPlaceRuntimeShell` in [crates/mirrorea-core/src/fabric.rs] and [crates/mirrorea-core/src/runtime.rs] provide transport/auth/membership/place lanes and validation, but they do not constitute package admission, hot-plug verdicting, transport completion, save/load completion, or devtools schema completion. The current Stage-B retained-later sets in [crates/mir-runtime/src/alpha_local_runtime.rs:908] and checker retained-later sets in [crates/mir-ast/src/practical_alpha1_checker.rs:17] already mark these as later. If `P-A1-03` exposes them as one merged “runtime success” surface, it will overclaim `P-A1-04` through `P-A1-07`.
- Finding 5:
  The first `P-A1-03` runtime-plan floor should not import alpha-0 helper-local identities as canonical plan state. `scripts/alpha_local_runtime_samples.py` and `crates/mir-runtime/src/alpha_local_runtime.rs` validate exact sample ids, fixed event relations, and exact reason refs for `LR-01/02`; `crates/mir-runtime/src/hotplug_runtime.rs` already warns against importing helper-local lifecycle ids into canonical runtime state. If `P-A1-03` bakes `LR-*` scenario names, `roll_request#1`, or report-local DAG node ids into the practical runtime-plan schema, it will freeze alpha-0 evidence artifacts into the practical carrier instead of deriving runtime state from package input.

## Skipped validations and reasons

- No checker/runtime command validations were run because this task produced no implementation change and made no success claim about `P-A1-03`.
- No `cargo fmt`, `cargo test`, or docs validators were run for the same reason; this report is the only file added.

## Commit / push status

Pending at report write.

## Sub-agent session close status

- No additional sub-agent sessions were opened for this review task.
