# Report 1120 — P-A0-18 Runtime-Mirror Boundary Review

- Date: 2026-05-02 18:54:29 JST
- Author / agent: Codex (GPT-5)
- Scope: semantics-boundary review for a possible helper-local `runtime_mirror` carrier limited to `VAR-08/11/13`, compared against widening `alpha-acceptance-floor` or claiming a parser/runtime bridge
- Decision levels touched: `L1` / `L2` review only; no normative edits

## Objective

Review whether a new helper-local `runtime_mirror` carrier for `VAR-08/11/13`, mirrored from `LI-04/01/03`, is semantically narrower than either:

- widening the existing `alpha-acceptance-floor`, or
- creating a parser/runtime bridge claim

Also confirm whether `LIF-11/13/15` and `VAR-14` must remain out.

## Scope and assumptions

- Read the repository-required sequence first, then the user-named Alpha-0 files.
- Focused on semantics-boundary / overclaim risk only.
- Did not change `specs/`, `plan/`, snapshots, or runtime code.
- Working assumption for review:
  a `runtime_mirror` carrier would be a new helper-local carrier, distinct from both the existing synthetic acceptance floor and any parser/runtime bridge.

## Start state / dirty state

- `git status --short --branch` at start: `## main...origin/main`
- No pre-existing tracked or untracked worktree changes were reported before this task.
- During closeout, unrelated untracked files appeared in the worktree
  (`docs/reports/1120-review-p-a0-18-runtime-mirror-wording-boundary.md`,
  `docs/reports/1121-review-p-a0-18-runtime-mirror-helper-boundary.md`,
  `scripts/alpha_contract_variance_runtime_mirror.py`,
  `scripts/current_l2_family_runtime_mirror_support.py`,
  `scripts/tests/test_alpha_contract_variance_runtime_mirror.py`,
  `scripts/tests/test_current_l2_family_runtime_mirror_support.py`).
  They were not reviewed or edited in this task.

## Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `specs/13-type-system-lifetime-fallback.md`
- `specs/14-contract-subtyping-layer-compatibility.md`
- `plan/39-type-system-freeze-roadmap.md`
- `plan/40-layer-compatibility-freeze-roadmap.md`
- `plan/43-alpha-e2e-roadmap.md`
- `samples/alpha/lifetime-fallback/README.md`
- `samples/alpha/contract-variance/README.md`
- `samples/alpha/layer-insertion/README.md`
- `samples/alpha/lifetime-fallback/lif-11-bird_sparkle_anchor_inheritance.expected.json`
- `samples/alpha/lifetime-fallback/lif-13-snapshot_selected_anchor.expected.json`
- `samples/alpha/lifetime-fallback/lif-15-remote_read_only_covariant.expected.json`
- `samples/alpha/contract-variance/var-08-ratelimit_declared_failure_valid.expected.json`
- `samples/alpha/contract-variance/var-11-redaction_layer_valid.expected.json`
- `samples/alpha/contract-variance/var-13-auth_layer_contract_update_valid.expected.json`
- `samples/alpha/contract-variance/var-14-adapter_transform_preserves_contract.expected.json`
- `samples/alpha/layer-insertion/li-01-debug_layer_attach_authorized.expected.json`
- `samples/alpha/layer-insertion/li-03-auth_layer_contract_update_path.expected.json`
- `samples/alpha/layer-insertion/li-04-ratelimit_declared_failure.expected.json`
- `scripts/alpha_contract_variance_acceptance.py`
- `scripts/current_l2_family_acceptance_support.py`
- `crates/mir-runtime/tests/alpha_layer_insertion_runtime.rs`
- `crates/mir-runtime/src/alpha_layer_insertion_runtime.rs`

## Actions taken

1. Read the mandatory repository context chain and the named Alpha-0 spec/plan/sample files.
2. Compared the current helper-local acceptance boundary against the runtime-backed `LI-01/03/04` mirrors.
3. Checked whether the existing contract-variance/lifetime docs already forbid widening the current acceptance floor.
4. Reviewed the current acceptance helper shape to confirm that its admitted kinds are only `transparent_observe_only_layer`, `output_covariance_checked`, and `readonly_covariance_checked`.
5. Reviewed the Stage-D runtime-side evidence carried by `LI-01/03/04` to isolate what each row actually proves.

## Files changed

- `docs/reports/1120-p-a0-18-runtime-mirror-boundary-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short --branch
date '+%Y-%m-%d %H:%M:%S %z'
rg --files -g 'README.md' -g 'Documentation.md' -g 'progress.md' -g 'tasks.md' -g 'samples_progress.md' -g 'specs/*.md' -g 'plan/*.md' | sort
ls specs
find samples/alpha/contract-variance -maxdepth 2 -type f | sort
find samples/alpha/layer-insertion -maxdepth 1 -type f | sort
find samples/alpha/lifetime-fallback -maxdepth 1 -type f | sort
rg -n "P-A0-18|runtime_mirror|acceptance_scope|alpha-acceptance-floor|VAR-08|VAR-11|VAR-13|VAR-14|LIF-11|LIF-13|LIF-15|LI-04|LI-01|LI-03" -S plan specs samples scripts crates docs progress.md tasks.md
sed -n '1,260p' <multiple repo files>
nl -ba <multiple repo files> | sed -n '<ranges>p'
```

## Evidence / outputs / test results

- `specs/14` explicitly confines the current helper-local positive acceptance floor to `VAR-01/04/06` and states that `VAR-08/11/13/14` are not raised into that floor because they carry runtime/layer/adapter meaning.
- `plan/40` repeats that the current safe reopen is either:
  proof that another row fits the existing acceptance schema without new runtime/layer/adapter semantics,
  or a layer/runtime-backed bridge that does not overclaim.
- `samples/alpha/contract-variance/README.md` already records that `VAR-08/11/13/14` are outside the acceptance floor, while `VAR-08/11/12/13` have runtime-backed mirrors under `samples/alpha/layer-insertion/`.
- `samples/alpha/layer-insertion/README.md` states the Stage-D cut is a non-public Rust attach-time floor and does not claim parser integration or final public ABI.
- `LI-01` runtime evidence carries accepted attach plus redacted post-attach trace rows.
- `LI-03` runtime evidence carries accepted path = explicit contract update with an activation-cut reference.
- `LI-04` runtime evidence carries accepted attach plus preview rejection with `RateLimited` / `rate_limit_budget_exhausted`.
- `LIF-11/13/15` and `VAR-14` sidecars remain planned skeletons with skeleton-only validation and no runtime-backed mirror carrier in the reviewed file set.

## What changed in understanding

- The semantically safe comparison point is not “can `VAR-08/11/13` be accepted somehow,” but “can they be admitted without collapsing the existing carrier boundaries.”
- The current helper-local acceptance floor is intentionally kind-limited and synthetic. Reusing its name/scope for runtime-backed rows would be a category error.
- A new helper-local `runtime_mirror` carrier can be narrower than both alternatives only if it is row-local and mirror-local:
  `VAR-08 <- LI-04`, `VAR-11 <- LI-01`, `VAR-13 <- LI-03`,
  with no widening to `VAR-14` and no inference for LIF rows.

## Open questions

- If a `runtime_mirror` carrier is introduced, what exact scope string and row schema distinguish it from `alpha-acceptance-floor` and from a parser/runtime bridge?
- Should `VAR-11` mirror only “redacted trace after attach” or also include explicit observation-label / retention facts in the mirror artifact, since the Stage-D sidecar itself records only part of that surface?
- Should `VAR-08/11/13` remain under `samples/alpha/contract-variance/` as mirror-only rows even after a new carrier is added, or should the runtime-floor authority move entirely to `layer-insertion/`?

## Suggested next prompt

Specify the exact proposed `runtime_mirror` artifact schema and scope string for `VAR-08/11/13`, and review whether each field is strictly mirror-derived from `LI-04/01/03` without implying generic variance acceptance or parser/runtime bridge completion.

## Plan update status

`plan/` 更新不要: reviewed against existing `plan/39`, `plan/40`, and `plan/43`; no wording drift requiring repository-memory edits was found in the reviewed boundary.

## Documentation.md update status

`Documentation.md` 更新不要: current snapshot wording already keeps helper-local acceptance, runtime floors, and non-claims separate.

## progress.md update status

`progress.md` 更新不要: the current snapshot already records that no safe `P-A0-18` is promoted and that remaining positive rows need either a new carrier or stronger evidence.

## tasks.md update status

`tasks.md` 更新不要: the current task map already frames post-`P-A0-17` work as a next-carrier decision rather than a promoted package.

## samples_progress.md update status

`samples_progress.md` 更新不要: this task reviewed semantics boundaries only and did not change sample status, validation commands, or blockers.

## Reviewer findings and follow-up

- Local maintainer-style review finding:
  any new `runtime_mirror` carrier is acceptable only if it remains distinct from `alpha-acceptance-floor`, is limited to the already mirrored `VAR-08/11/13`, and does not upgrade runtime-floor evidence into generic variance acceptance.
- Follow-up:
  if implementation proceeds, review the proposed mirror schema before promoting any `P-A0-18` package wording.

## Skipped validations and reasons

- Did not run Cargo or Python test suites in this task.
  Reason: the request was a semantics-boundary review of already committed specs/plans/sidecars, not a change to executable artifacts.
- Did not run parser/runtime samples.
  Reason: the review question was about whether a new carrier would overclaim semantics, and the needed evidence was already present in the committed docs and sidecars.

## Commit / push status

Not performed in this task. This task produced a review report only and left unrelated concurrent untracked files untouched.

## Sub-agent session close status

- No sub-agent used.
