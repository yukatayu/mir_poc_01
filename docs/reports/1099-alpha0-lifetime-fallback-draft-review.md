# Report 1099 — Alpha-0 Lifetime/Fallback Draft Review

- Date: 2026-05-02 06:20:41 JST
- Author / agent: Codex (GPT-5)
- Scope: review-only audit of `specs/13-type-system-lifetime-fallback.md`, `plan/39-type-system-freeze-roadmap.md`, `samples/alpha/lifetime-fallback/`, and touched snapshot docs only where they mention fallback semantics
- Decision levels touched: review only; no normative text changed

## Objective

Review the Alpha-0 type-system soundness draft for semantic contradictions, lifetime/fallback unsoundness, accidental target-lifetime extension wording, missing no-resurrection constraints, remote observed reference problems, and claims that exceed current implementation.

## Scope and assumptions

- Followed repository reading order before narrowing to the requested review surface.
- Treated this task as a maintainer/spec-editor review, not an implementation task.
- Did not modify the reviewed draft files; only added this report to satisfy repository reporting policy.
- Interpreted “touched snapshot docs” narrowly as modified snapshot/front-door documents that mention fallback semantics or fallback progress.

## Start state / dirty state

- Worktree was already dirty at task start.
- Relevant pre-existing modified/untracked paths included `README.md`, `Documentation.md`, `plan/00-index.md`, `progress.md`, `samples_progress.md`, `specs/00-document-map.md`, `specs/11-roadmap-and-workstreams.md`, `specs/12-decision-register.md`, `tasks.md`, `specs/13-type-system-lifetime-fallback.md`, `plan/39-type-system-freeze-roadmap.md`, `samples/alpha/`, and other unrelated paths shown by `git status --short`.
- This task added only `docs/reports/1099-alpha0-lifetime-fallback-draft-review.md`.

## Documents consulted

- `README.md`
- `Documentation.md`
- `plan/00-index.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/13-type-system-lifetime-fallback.md`
- `plan/39-type-system-freeze-roadmap.md`
- `samples/alpha/lifetime-fallback/README.md`
- `samples/alpha/lifetime-fallback/*.expected.json`
- touched snapshot docs searched for fallback semantics/progress:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `samples/README.md`
  - `scripts/README.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/19-repository-map-and-taxonomy.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`

## Actions taken

- Read the mandatory repository entry documents in required order.
- Inspected the target spec, roadmap, and alpha sample-family scaffold.
- Searched touched snapshot docs for fallback/lifetime/anchor/remote-reference wording.
- Cross-checked `plan/39` implementation claims against current parser/checker/runtime support in `crates/` and `scripts/`.
- Recorded review findings and supporting evidence here.

## Files changed

- `docs/reports/1099-alpha0-lifetime-fallback-draft-review.md` (new)

## Commands run

- `sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md`
- `sed -n '1,220p' /home/yukatayu/dev/mir_poc_01/.agents/skills/discord-report/SKILL.md`
- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `git status --short`
- `wc -l README.md Documentation.md specs/00* specs/01* specs/02* specs/03* specs/09* plan/00-index.md specs/13-type-system-lifetime-fallback.md plan/39-type-system-freeze-roadmap.md samples/alpha/lifetime-fallback/*`
- `sed -n '1,260p' README.md`
- `sed -n '1,320p' Documentation.md`
- `sed -n '1,220p' plan/00-index.md`
- `sed -n '1,260p' specs/00-document-map.md`
- `sed -n '1,200p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,200p' specs/02-system-overview.md`
- `sed -n '1,200p' specs/03-layer-model.md`
- `sed -n '1,200p' specs/09-invariants-and-constraints.md`
- `rg -n "fallback|lifetime|resurrect|remote observed|observed ref|anchor" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md scripts/README.md plan/01-status-at-a-glance.md plan/11-roadmap-near-term.md plan/19-repository-map-and-taxonomy.md specs/00-document-map.md specs/11-roadmap-and-workstreams.md specs/12-decision-register.md samples/not_implemented/README.md`
- `sed -n '1,340p' specs/13-type-system-lifetime-fallback.md`
- `sed -n '1,220p' plan/39-type-system-freeze-roadmap.md`
- `find samples/alpha/lifetime-fallback -maxdepth 1 -type f | sort | while read f; do echo "=== $f ==="; sed -n '1,120p' "$f"; done`
- `nl -ba specs/13-type-system-lifetime-fallback.md | sed -n '1,340p'`
- `nl -ba plan/39-type-system-freeze-roadmap.md | sed -n '1,220p'`
- `nl -ba samples/alpha/lifetime-fallback/README.md | sed -n '1,220p'`
- `rg -n "fallback|lineage|target mismatch|capability|lease|inherit_chain|snapshot_selected|StaleMembership|atomic_cut|resurrect|re-promotion|same-lineage" crates scripts samples/current-l2 samples/clean-near-end`
- `rg -n "lease" scripts/current_l2_* scripts/tests/test_current_l2_* crates/mir-ast/tests -g '!*.pyc'`
- `ls scripts/current_l2_*checker.py`
- `find crates/mir-ast/tests/fixtures/current-l2 -maxdepth 2 -type f | sort`
- `rg -n "lease-expired|expired|admissible|host-plan|write-after-expiry|fallback-after-expiry|no-resurrection|re-promot|stale membership|membership epoch|witness refs|remote" crates/mir-runtime crates/mir-semantics scripts`
- `sed -n '1,220p' scripts/current_l2_missing_option_checker.py`
- `sed -n '1,220p' scripts/current_l2_try_rollback_structural_checker.py`
- `ls docs/reports | sed -n '1,20p'; echo '---'; ls docs/reports | tail -n 20`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
- `python3 scripts/check_source_hierarchy.py`

## Evidence / outputs / test results

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `python3 scripts/check_source_hierarchy.py`
  - `required: 50`
  - `present: 50`
  - `missing: 0`
  - `all required paths present`
- Review findings identified:
  - canonicalization currently drops path identity information that the rest of the spec depends on
  - remote fallback semantics do not constrain label/epoch/frontier monotonicity strongly enough
  - `LIF-08` sample verdict is ambiguous against the spec’s current static/dynamic split
  - `plan/39` current-state wording is stale against the same patch and slightly overstates “existing ... lease checks”

## What changed in understanding

- The draft is already disciplined about “fallback is not target lifetime extension” and “no resurrection across rollback / cut / load”.
- The strongest remaining risks are not the headline rule, but the information that survives canonicalization and the missing remote monotonicity constraints.
- The current implementation does have parser/runtime support for lineage/capability/lease-shaped current-L2 fixtures, but the roadmap wording needs to distinguish parser/runtime metadata support from dedicated checker-floor coverage more carefully.

## Open questions

- Should canonicalization preserve `target` and `role` explicitly in the canonical chain artifact, or is there an intended external invariant that reconstructs them losslessly?
- For remote observed references, should stale membership/witness/frontier on any option poison the entire chain unless there is explicit reacquire evidence?
- Is `LIF-08` intended to pin a static rejection, a dynamic `Reject`, or a bifurcated pair of rows that separate statically known read-only fallback from runtime-selected read-only fallback?

## Suggested next prompt

`Address the findings in docs/reports/1099-alpha0-lifetime-fallback-draft-review.md, update specs/13 and plan/39 accordingly, and make the LIF-08 / remote-fallback sample expectations unambiguous.`

## Plan update status

`plan/` 更新不要 / 更新済み: `plan/39` was reviewed only; no update was made in this task.

## Documentation.md update status

`Documentation.md` 更新不要 / 更新済み: review-only task; no snapshot wording change made.

## progress.md update status

`progress.md` 更新不要 / 更新済み: review-only task; no progress snapshot change made.

## tasks.md update status

`tasks.md` 更新不要 / 更新済み: review-only task; no task-map change made.

## samples_progress.md update status

`samples_progress.md` 更新不要 / 更新済み: review-only task; no dashboard change made.

## Reviewer findings and follow-up

- High: `specs/13` canonicalization omits `target`/`role`, which can erase the access-path identity the same spec later needs to prevent cross-target or cross-role chain merging.
- High: `specs/13` does not make remote fallback monotonic in label/frontier/epoch terms, so a chain can still describe an information-widening or stale-observation successor while claiming monotone degradation.
- Medium: `samples/alpha/lifetime-fallback/README.md` and `LIF-08` sidecars leave the verdict as `Reject or static error`, which no longer pins the static/dynamic boundary that `specs/13` is trying to freeze.
- Medium: `plan/39` says the repo “still lacks Alpha-0 sample family / docs taxonomy / scope wording” even though this same change set adds them, and its “existing ... lease checks” phrasing is broader than the actual dedicated checker-floor inventory.

## Skipped validations and reasons

- Did not run full repo validation or cargo/pytest suites because the task was a review-only semantic audit with no implementation changes to the reviewed draft.
- Did not run alpha sample execution because the reviewed family is explicitly scaffold-only and not wired to active runners.
- Did not commit or push because this task added only a review report and did not apply requested draft fixes.

## Commit / push status

Not committed or pushed. Worktree was pre-dirty and this task was scoped to review findings plus the required report only.

## Sub-agent session close status

No sub-agent session used.
