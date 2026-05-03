# Report 1145 — Review P-A1-04b Snapshot Overclaim Risk

- Date: 2026-05-03
- Author / agent: Codex
- Scope: review wording changes required if `P-A1-04b` closes with freshness negatives and only a narrow object-attach seam
- Decision levels touched: none

## Objective

Review the current snapshot and front-door documents and identify the wording that must change if `P-A1-04b` closes, so readers do not misread that closeout as `PA1-4` completion or practical alpha-1 completion.

## Scope and assumptions

- Review-only task; no file edits besides this report.
- Assumed close condition: `P-A1-04b` closes attach-time `missing-witness` and `stale-membership` negatives plus a narrow object package attach seam, but does not close detach minimal contract.
- Focused on the user-requested files plus the mandatory repo reading order needed to compare snapshot wording against `specs/18`.

## Start state / dirty state

- start state: `main` at `5f62503a326016d0718c100608860bccb3d3cbec`
- worktree status: clean before this report

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `samples/practical-alpha1/README.md`

## Actions taken

1. Read the required repo front-door and normative documents in repository order.
2. Read the requested snapshot and practical-alpha-1 documents.
3. Compared current snapshot wording with the `specs/18` package/hot-plug completion condition, especially the remaining `detach minimal contract` requirement.
4. Collected the specific wording sites that would become stale or overclaiming after `P-A1-04b` closes.

## Files changed

- `docs/reports/review-2026-05-03-pa1-04b-snapshot-overclaim-review.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
wc -l README.md Documentation.md progress.md tasks.md samples_progress.md specs/00-document-map.md specs/01-charter-and-decision-levels.md specs/02-system-overview.md specs/03-layer-model.md specs/09-invariants-and-constraints.md specs/18-practical-alpha1-scope.md plan/00-index.md plan/44-practical-alpha1-roadmap.md samples/practical-alpha1/README.md
git status --short
date '+%Y-%m-%d %H:%M:%S %z'
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' progress.md
sed -n '1,240p' tasks.md
sed -n '1,240p' samples_progress.md
sed -n '1,120p' samples/practical-alpha1/README.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,220p' plan/00-index.md
sed -n '1,240p' specs/18-practical-alpha1-scope.md
sed -n '1,240p' plan/44-practical-alpha1-roadmap.md
sed -n '220,280p' plan/44-practical-alpha1-roadmap.md
rg -n "P-A1-04b|PA1-4|object attach|object-attach|freshness negatives|missing-witness|stale-membership" README.md Documentation.md progress.md tasks.md samples_progress.md samples/practical-alpha1/README.md plan/44-practical-alpha1-roadmap.md specs/18-practical-alpha1-scope.md
nl -ba README.md | sed -n '50,58p'
nl -ba Documentation.md | sed -n '58,78p'
nl -ba progress.md | sed -n '20,45p'
nl -ba progress.md | sed -n '112,122p'
nl -ba tasks.md | sed -n '72,90p'
nl -ba samples_progress.md | sed -n '26,42p'
nl -ba samples_progress.md | sed -n '88,94p'
nl -ba samples/practical-alpha1/README.md | sed -n '1,60p'
nl -ba specs/18-practical-alpha1-scope.md | sed -n '120,205p'
nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '108,190p'
```

## Evidence / outputs / test results

- `specs/18-practical-alpha1-scope.md` keeps `PA1-4` broader than `P-A1-04b`: package/hot-plug completion still requires debug/auth/rate-limit/object attach, typed rejects for incompatible patch / missing capability / missing witness / stale membership, and explicit detach minimal contract handling (`specs/18-practical-alpha1-scope.md:180-185`).
- `plan/44-practical-alpha1-roadmap.md` currently recuts `PA1-4` into only `P-A1-04a` and `P-A1-04b`, but its stage bullet list omits detach minimal contract (`plan/44-practical-alpha1-roadmap.md:108-117`).
- `samples/practical-alpha1/README.md` also omits detach minimal contract from the remaining-boundary list (`samples/practical-alpha1/README.md:44-54`).
- Current snapshot docs consistently describe `P-A1-04a` as the last closed package and `P-A1-04b` as the next in-stage package, at `PA1-4 45%` (`progress.md:23-38`, `tasks.md:73-84`, `samples_progress.md:27-40`, `samples_progress.md:88-92`).
- Current root summaries also still say practical alpha-1 has only a layer-only hot-plug first floor and does not include object attach or freshness negatives (`README.md:52-56`, `Documentation.md:61-75`).

## What changed in understanding

- The main overclaim risk after `P-A1-04b` is not stale mention of object attach/freshness negatives by itself. The larger risk is that removing those items without promoting `detach minimal contract` to the visible remaining `PA1-4` blocker would make the stage read complete.
- `plan/44` and `samples/practical-alpha1/README.md` already underexpose the detach requirement compared with `specs/18`. That mismatch becomes dangerous the moment `P-A1-04b` closes.

## Open questions

- What package identifier will be used for the post-`P-A1-04b` detach-minimal-contract tranche: `P-A1-04c` or another explicit in-stage name?
- Should `PA1-4` progress move to a named rough value such as `60-65%`, or should the repo keep the percentage unchanged until the next in-stage recut is written?

## Suggested next prompt

- `If P-A1-04b closes, update README.md, Documentation.md, progress.md, tasks.md, samples_progress.md, samples/practical-alpha1/README.md, and plan/44-practical-alpha1-roadmap.md so they say P-A1-04b actualized attach-time missing-witness/stale-membership negatives plus a narrow object-attach seam, but PA1-4 remains open on detach minimal contract and practical alpha-1 remains incomplete.`

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
  `specs/18` requires `detach minimal contract` as part of package/hot-plug completion, but `plan/44` and `samples/practical-alpha1/README.md` do not keep that residual visible. `specs/18-practical-alpha1-scope.md:180-185` says package/hot-plug completion still needs explicit detach handling, while `plan/44-practical-alpha1-roadmap.md:108-117` lists `PA1-4` only as package schema, admission checker, debug/auth/rate-limit/object attach, and negative cases, and `samples/practical-alpha1/README.md:44-54` lists remaining non-claims without detach. If `P-A1-04b` closes and the repo simply removes object/freshness items from the remaining lists, readers can reasonably infer `PA1-4` is done. Required update: add detach minimal contract explicitly to `plan/44` and `samples/practical-alpha1/README.md`, and keep it as the visible remaining in-stage blocker after `P-A1-04b`.

- Finding 2:
  The stage snapshot lines in `progress.md`, `tasks.md`, and `samples_progress.md` must not roll from “`P-A1-04b` next” to “`PA1-5` next” unless a new in-stage detach package is named. The current stage rows already encode that `PA1-4` is in progress (`progress.md:26-38`, `tasks.md:76-84`, `samples_progress.md:30-40`, `samples_progress.md:92`). Under the assumed closure, those lines need to change to something like “`P-A1-04b` closed freshness negatives and a narrow object-attach seam; detach minimal contract remains next inside `PA1-4`”. Required update: keep `PA1-4` below `100%`, keep `Package status` on the last closed package rather than the whole stage, and make the next autonomous package an in-stage detach tranche rather than `P-A1-05`.

- Finding 3:
  The root summary docs will become stale, but broadening them carelessly would overclaim. `README.md:52-56` and `Documentation.md:61-75` currently say the practical line has only a layer-only first hot-plug floor and explicitly excludes object attach and freshness negatives. After `P-A1-04b`, these passages must not be rewritten as “package/hot-plug practical API exists” or “practical hot-plug is complete”. Required update: rewrite them to say the current practical hot-plug floor now includes attach-time `missing-witness` / `stale-membership` rejects and a narrow object package attach seam, while detach minimal contract, transport, save/load, devtools, product prototype, and final public ABI remain unclaimed.

- Finding 4:
  The phrase `freshness/object seam` is acceptable as an in-flight shorthand, but it is too vague for a closed-package snapshot. `progress.md:28-38`, `tasks.md:78-84`, and `samples_progress.md:32-40` currently use that shorthand because `P-A1-04b` is still pending. After closure, leaving the wording at “freshness negatives” risks conflating attach-time negatives with already-existing runtime freshness rows such as `RUN-02` and alpha save/load stale-membership rows. Required update: closed-package wording should expand the shorthand to “attach-time `missing-witness` and `stale-membership` typed rejects plus narrow object package attach seam”.

- Finding 5:
  `samples_progress.md` needs two separate honesty guards when `P-A1-04b` closes. First, the stage/package map rows must keep `PA1-4` in progress rather than letting the new `HP-*` family look like stage completion (`samples_progress.md:88-92`). Second, if a new practical hot-plug sample row reaches `100%` for the narrow floor, its notes must still say that only the sample/floor is complete for current scope, not `PA1-4` or practical alpha-1. Required update: keep the hot-plug row notes scoped to “non-final alpha-local hot-plug floor”, add the remaining detach minimal contract non-claim, and only increase the overall practical-alpha-1 percentages modestly.

## Skipped validations and reasons

- No repo validation commands were run because this was a wording-consistency review, not an implementation or freshness-verification task.
- No doc edits beyond this report were made, so rerunning `validate_docs.py`, source hierarchy checks, or cargo/python tests would not add signal for the requested review.

## Commit / push status

- No commit created.
- No push performed.

## Sub-agent session close status

- No sub-agents used.
