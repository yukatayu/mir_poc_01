# Report 1156 — Review P-A1-05 Docs/Progress Consistency

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review wording changes required if `P-A1-05` closes `PA1-5` and the practical transport stage becomes complete
- Decision levels touched: none

## Objective

Review the current snapshot, sample-taxonomy, roadmap-memory, and normative practical-alpha-1 wording and identify what must move together, what wording becomes stale, and how to keep practical transport separate from current-scope Stage C evidence if `P-A1-05` closes.

## Scope and assumptions

- Review-only task; no file edits besides this report.
- Assumed close condition: `P-A1-05` closes the practical transport requirement in `specs/18` with product-like Docker/local TCP execution over the same practical package input, separated auth/membership/capability/witness lanes, and route-trace export.
- Focused on the user-requested files plus the mandatory front-door and normative prerequisites from `AGENTS.md`.
- This review does not assume that `P-A1-06` devtools, `P-A1-07` save/load, or `P-A1-08` product prototype also close.

## Start state / dirty state

- start state: `main` at current workspace HEAD
- worktree status: clean before this report

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `samples/README.md`
- `samples/practical-alpha1/README.md`
- `scripts/README.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`

## Actions taken

1. Read the required repository front-door and normative documents in the order mandated by `AGENTS.md`.
2. Read the current snapshot, task map, sample dashboard, sample taxonomy, and script taxonomy.
3. Compared the practical transport completion condition in `specs/18` and `plan/44` against the current snapshot wording.
4. Collected the specific snapshot, taxonomy, and roadmap sites that would become stale if `P-A1-05` closes.
5. Checked where the current repo already distinguishes practical transport from helper-local canaries and alpha-0 Stage C evidence, and where that distinction would still need wording hardening.

## Files changed

- `docs/reports/1156-review-p-a1-05-docs-progress-consistency.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md
sed -n '1,220p' .agents/skills/discord-report/SKILL.md
git status --short
rg --files specs plan | sort
date '+%Y-%m-%d %H:%M:%S %z %Z'
sed -n '1,220p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,240p' progress.md
sed -n '1,260p' tasks.md
sed -n '1,260p' samples_progress.md
sed -n '1,260p' samples/README.md
sed -n '1,260p' samples/practical-alpha1/README.md
sed -n '1,260p' scripts/README.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,240p' specs/09-invariants-and-constraints.md
sed -n '1,320p' specs/18-practical-alpha1-scope.md
sed -n '1,240p' plan/00-index.md
sed -n '1,320p' plan/44-practical-alpha1-roadmap.md
rg -n "P-A1-05|PA1-5|transport practical E2E|Docker/local TCP|Stage C|alpha-0.7|current-scope evidence closeout|practical transport|local TCP|route trace|network" README.md Documentation.md progress.md tasks.md samples_progress.md samples/README.md samples/practical-alpha1/README.md scripts/README.md specs/18-practical-alpha1-scope.md plan/44-practical-alpha1-roadmap.md
nl -ba README.md | sed -n '46,74p'
nl -ba Documentation.md | sed -n '56,95p'
nl -ba progress.md | sed -n '20,120p'
nl -ba tasks.md | sed -n '1,220p'
nl -ba samples_progress.md | sed -n '1,160p'
nl -ba samples_progress.md | sed -n '100,124p'
nl -ba samples/README.md | sed -n '1,120p'
nl -ba samples/practical-alpha1/README.md | sed -n '1,120p'
nl -ba scripts/README.md | sed -n '1,120p'
nl -ba specs/18-practical-alpha1-scope.md | sed -n '1,260p'
nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '1,260p'
```

## Evidence / outputs / test results

- `specs/18` defines `PA1-5` as a distinct transport stage after package/hot-plug, with Docker/local TCP E2E, separated transport/auth/membership/capability/witness lanes, and observer-safe route trace; it is explicitly not production WAN/federation (`specs/18-practical-alpha1-scope.md:195-200`, `specs/18-practical-alpha1-scope.md:238-247`).
- `plan/44` mirrors that staging and currently keeps `P-A1-05` as the promoted next package after `P-A1-04c` (`plan/44-practical-alpha1-roadmap.md:119-124`, `plan/44-practical-alpha1-roadmap.md:162-174`, `plan/44-practical-alpha1-roadmap.md:247-260`).
- The snapshot authorities are aligned today around the pre-close state: `PA1-5` is pending, `P-A1-04c` is the last closed package, and `P-A1-05` is next (`progress.md:25-38`, `tasks.md:76-84`, `tasks.md:129-133`, `samples_progress.md:29-40`, `samples_progress.md:57-59`, `samples_progress.md:117-119`).
- The root summaries and taxonomies still encode transport as later practical work: `README.md:52-56`, `Documentation.md:61-77`, `samples/README.md:17-20`, `samples/practical-alpha1/README.md:52-63`, `scripts/README.md:30-42`, `plan/44-practical-alpha1-roadmap.md:193-209`.
- The repo already has two distinct non-practical transport surfaces that must remain separate from `P-A1-05`: helper-local canaries `NET-02..05` and alpha-0 Stage C closeout over `NET-02/03/04/05/07/09` (`Documentation.md:67-90`, `tasks.md:46-51`, `tasks.md:141`, `samples/README.md:46-47`, `scripts/README.md:19-20`, `scripts/README.md:65-66`, `samples_progress.md:66`, `samples_progress.md:104`, `samples_progress.md:131`, `samples_progress.md:147`, `samples_progress.md:160`).
- No repository validation commands were rerun because this was a wording-consistency review, not an implementation or freshness-verification task.

## What changed in understanding

- The main `P-A1-05` consistency risk is not only advancing one stage percentage. It is accidentally collapsing three transport lanes into one: helper-local canaries, alpha-0 Stage C evidence, and practical package-driven transport.
- `PA1-5` closeout will require more synchronized wording than `PA1-4` did, because transport affects stage tables, sample taxonomy, script taxonomy, validation anchors, and the practical sample-root roadmap simultaneously.
- The practical line can truthfully claim transport completion without claiming devtools, save/load, product prototype, or public transport ABI, but only if the wording stays anchored on the practical package input and keeps alpha-0 Stage C as prerequisite evidence rather than the authority for the new closeout.

## Open questions

- What exact practical command/report surface will name the closed `P-A1-05` lane: a new `scripts/practical_alpha1_run_docker.py`, a widened `practical_alpha1_run_local.py`, or a different practical transport front door?
- Will `samples/practical-alpha1/docker/` become an actualized path at `P-A1-05`, or will Docker/local TCP evidence stay under `packages/` plus generated runtime artifacts only?
- Which practical sample IDs will anchor the transport floor, and will there be explicit negative practical transport fixtures separate from the existing `RUN-02` and `HP-A1-04B1/04B2` rows?

## Suggested next prompt

- `If P-A1-05 closes, update README.md, Documentation.md, progress.md, tasks.md, samples_progress.md, samples/README.md, samples/practical-alpha1/README.md, scripts/README.md, specs/18-practical-alpha1-scope.md, and plan/44-practical-alpha1-roadmap.md so PA1-5 is shown as complete, P-A1-06 becomes next, practical transport validation/commands are named explicitly, and helper-local canaries plus alpha-0 Stage C remain documented as separate prerequisite evidence rather than the practical transport authority.`

## Plan update status

`plan/` 更新不要: review-only taskであり repository memory は変更していない。

## Documentation.md update status

`Documentation.md` 更新不要: review-only taskであり snapshot wording は変更していない。

## progress.md update status

`progress.md` 更新不要: review-only taskであり current status snapshot は変更していない。

## tasks.md update status

`tasks.md` 更新不要: review-only taskであり task map は変更していない。

## samples_progress.md update status

`samples_progress.md` 更新不要: review-only taskであり sample dashboard snapshot は変更していない。

## Reviewer findings and follow-up

- Finding 1:
  The three snapshot authorities must move in lockstep if `PA1-5` closes. They currently agree that `PA1-5` is still pending and that `P-A1-04c` is the last closed package (`progress.md:25-38`, `tasks.md:76-84`, `tasks.md:129-133`, `samples_progress.md:29-40`). If only one or two of these are updated, the repo will simultaneously claim that practical transport is both complete and still the next package. Required follow-up: update stage percentage, phase wording, package status, current status, blocker wording, next package, readiness map, and any package tables together so `P-A1-05` becomes the last closed package and `P-A1-06` becomes next everywhere.

- Finding 2:
  The root summaries and practical taxonomies will go stale immediately unless they are widened selectively. `README.md:52-56` and `Documentation.md:61-77` still say Docker/local TCP transport is outside the current practical floor. `samples/README.md:17-20` and `samples/practical-alpha1/README.md:52-63` still describe the practical root as front-door + checker + local runtime + hot-plug only. `scripts/README.md:30-42` still says practical `run-docker` remains later work. If `P-A1-05` closes, these lines must be updated to name the practical transport surface explicitly, but they must still not claim devtools, save/load, product prototype, final public runtime/devtools ABI, or production WAN.

- Finding 3:
  `plan/44` has multiple “later” statements that become stale together if practical transport lands. The package map still keeps `PA1-5` only as the next package (`plan/44-practical-alpha1-roadmap.md:119-124`, `plan/44-practical-alpha1-roadmap.md:162-174`), the sample-root roadmap still says `docker/` is reserved for later (`plan/44-practical-alpha1-roadmap.md:193-209`), and the validator roadmap stops at local-runtime and hot-plug paths (`plan/44-practical-alpha1-roadmap.md:222-236`). Required follow-up: synchronize package-map state, sample-root reading, and validation-anchor memory in the same task, or roadmap-memory will contradict the snapshots.

- Finding 4:
  The practical transport closeout must stay explicitly separate from alpha-0 Stage C evidence. `Documentation.md:67-77` and `tasks.md:141` already define Stage C as a current-scope Docker/local-subprocess narrow cut over `samples/alpha/network-docker/`, while `samples_progress.md:147` and `scripts/README.md:65-66` keep `alpha_network_docker_e2e.py` as alpha-0 closeout authority. If `P-A1-05` is described merely as “Docker/local TCP now works” without naming the practical package-driven route and transport-specific output, readers will collapse `P-A1-05` into `P-A0-24`. Required follow-up: keep the wording anchored on “same practical package input,” a practical transport command/report, and the transport/auth/membership/capability/witness split from `specs/18-practical-alpha1-scope.md:195-200`.

- Finding 5:
  `samples_progress.md` needs more than a stage-row bump when `P-A1-05` closes. Its top-level toolchain summary currently points to attach-only validation (`samples_progress.md:57-59`), and its practical matrix currently ends at `SRC-*`, `CHK-*`, `RUN-*`, and `HP-*` (`samples_progress.md:115-119`). Required follow-up: add or widen the practical transport evidence row and its validation anchor, then update the toolchain summary so the dashboard shows concrete practical transport evidence rather than only a stage percentage. Without that, `PA1-5 = 100%` would be evidence-thin compared to the dashboard’s own 100% rule.

## Skipped validations and reasons

- No implementation or sample validation commands were rerun because this was a wording-consistency review, not a closeout or freshness-verification task.
- No snapshot, taxonomy, or normative docs were edited beyond this report, so cargo/python/runtime reruns would not materially change the review evidence.

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agents used.
