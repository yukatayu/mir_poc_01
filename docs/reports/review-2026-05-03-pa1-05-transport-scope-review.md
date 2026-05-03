# P-A1-05 Transport Scope Review

## 1. Title and identifier

- `review-2026-05-03-pa1-05-transport-scope-review`

## 2. Objective

- Review the narrowest safe scope for `P-A1-05` practical transport E2E after `P-A1-04c`.
- Focus on admissible row set, explicit non-claims, and contradictions between roadmap text and the practical alpha-1 sample matrix.

## 3. Scope and assumptions

- Scope was limited to the repository guidance and the documents named in the task: `README.md`, `Documentation.md`, `specs/00..03`, `specs/09`, `specs/18-practical-alpha1-scope.md`, `plan/00-index.md`, `plan/44-practical-alpha1-roadmap.md`, `progress.md`, `tasks.md`, and `sub-agent-pro/alpha-1/04-stage-roadmap.md`, `06-toolchain-architecture.md`, `08-sample-matrix.md`, `09-validation-plan.md`.
- Review mode only. No implementation or snapshot edits were allowed outside `docs/reports/`.
- The review treats `specs/18` plus current snapshot docs as the safest authority for current-stage boundary reading; `sub-agent-pro/` remains handoff material, not normative source.

## 4. Start state / dirty state

- Worktree was clean at start according to `git status --short`.
- This task only adds this new report file.

## 5. Documents consulted

- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/18-practical-alpha1-scope.md`
- `plan/00-index.md`
- `plan/44-practical-alpha1-roadmap.md`
- `progress.md`
- `tasks.md`
- `sub-agent-pro/alpha-1/04-stage-roadmap.md`
- `sub-agent-pro/alpha-1/06-toolchain-architecture.md`
- `sub-agent-pro/alpha-1/08-sample-matrix.md`
- `sub-agent-pro/alpha-1/09-validation-plan.md`

## 6. Actions taken

- Read the required repository guidance and baseline specs in the mandated order.
- Compared the current practical alpha-1 normative boundary against the handoff roadmap and sample matrix.
- Isolated the currently actualized substrate rows, the proposed `P-A1-05` transport rows, and the later-stage rows that must remain non-claims.
- Checked the current snapshot docs to see how `P-A1-05` is already narrowed after `P-A1-04c`.

## 7. Files changed

- `docs/reports/review-2026-05-03-pa1-05-transport-scope-review.md` (new)

## 8. Commands run

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
- `pwd && ls -1 README.md Documentation.md progress.md tasks.md plan/00-index.md plan/44-practical-alpha1-roadmap.md specs/00* specs/01* specs/02* specs/03* specs/09* specs/18-practical-alpha1-scope.md sub-agent-pro/alpha-1/04-stage-roadmap.md sub-agent-pro/alpha-1/06-toolchain-architecture.md sub-agent-pro/alpha-1/08-sample-matrix.md sub-agent-pro/alpha-1/09-validation-plan.md`
- `rg -n "Title and identifier|sub-agent session close status|reviewer findings and follow-up" docs/reports -g '*.md' | head -n 20`
- `sed -n '1,220p' /home/yukatayu/.codex/skills/superpowers/skills/using-superpowers/SKILL.md`
- `sed -n '1,260p' /home/yukatayu/dev/mir_poc_01/.agents/skills/discord-report/SKILL.md`
- `sed -n '1,220p' README.md`
- `sed -n '1,260p' Documentation.md`
- `sed -n '1,220p' plan/00-index.md`
- `nl -ba progress.md | sed -n '1,260p'`
- `nl -ba tasks.md | sed -n '1,260p'`
- `nl -ba specs/18-practical-alpha1-scope.md | sed -n '1,320p'`
- `nl -ba sub-agent-pro/alpha-1/04-stage-roadmap.md | sed -n '1,260p'`
- `nl -ba sub-agent-pro/alpha-1/06-toolchain-architecture.md | sed -n '1,260p'`
- `nl -ba sub-agent-pro/alpha-1/08-sample-matrix.md | sed -n '1,320p'`
- `nl -ba sub-agent-pro/alpha-1/09-validation-plan.md | sed -n '1,260p'`
- `nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '1,360p'`
- `sed -n '1,220p' specs/00-document-map.md`
- `sed -n '1,240p' specs/01-charter-and-decision-levels.md`
- `sed -n '1,240p' specs/02-system-overview.md`
- `sed -n '1,240p' specs/03-layer-model.md`
- `sed -n '1,240p' specs/09-invariants-and-constraints.md`
- `sed -n '1,260p' docs/reports/TEMPLATE.md`
- `sed -n '1,220p' docs/reports/review-2026-05-03-pa1-01-front-door-overclaim-review.md`
- `git status --short`
- `date '+%Y-%m-%d %H:%M:%S %Z'`
- `rg -n "missing capability|missing witness|stale membership|route trace|object package attach|detach|minimal contract|preview seam|reject/defer/fallback|run-docker|Docker/local TCP|TR-A1|HP-A1-06|HP-A1-07|RUN-03|RUN-04|RUN-05|PE2E" sub-agent-pro/alpha-1/04-stage-roadmap.md sub-agent-pro/alpha-1/06-toolchain-architecture.md sub-agent-pro/alpha-1/08-sample-matrix.md sub-agent-pro/alpha-1/09-validation-plan.md plan/44-practical-alpha1-roadmap.md specs/18-practical-alpha1-scope.md progress.md tasks.md`
- `nl -ba tasks.md | sed -n '124,136p'`
- `nl -ba plan/44-practical-alpha1-roadmap.md | sed -n '118,130p;162,176p;193,210p;247,259p'`
- `nl -ba sub-agent-pro/alpha-1/04-stage-roadmap.md | sed -n '58,82p'`
- `nl -ba sub-agent-pro/alpha-1/08-sample-matrix.md | sed -n '24,49p;76,86p'`
- `nl -ba specs/18-practical-alpha1-scope.md | sed -n '145,173p;195,200p;209,219p;288,297p'`

## 9. Evidence / outputs / test results

- `git status --short` returned no entries at review start.
- No code or sample validation was run. This task was a document-scope review only.
- The strongest current-scope narrowing signal for `P-A1-05` is in `tasks.md`, which defines the stage as: same practical package input, Docker/local TCP execution, and export of route trace plus separated lanes.
- `plan/44` simultaneously narrows the currently actualized substrate to `RUN-01/02` and `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, `HP-A1-07`, while the sample matrix lists broader runtime and hot-plug row labels without planned/current distinction.

## 10. What changed in understanding

- The narrowest safe `P-A1-05` reading is smaller than the combined handoff text suggests.
- Safe current prerequisites are the already-actualized practical front-door, checker first floor, local-runtime first floor over `RUN-01/02`, and hot-plug first floor over `HP-A1-01..05`, `HP-A1-04B1`, `HP-A1-04B2`, `HP-A1-06`, `HP-A1-07`.
- The least contradictory transport row set presently visible in the docs is `TR-A1-01..05`, but even that set needs clearer status because the roadmap text and snapshot wording do not agree on whether missing capability / missing witness negatives are part of `P-A1-05`.

## 11. Open questions

- Should `P-A1-05` include transport-specific missing-capability and missing-witness negatives as distinct `TR-*` rows, or should those remain package/hot-plug-only evidence and stay out of the transport package?
- Should `RUN-03/04/05` be marked planned-only or moved out of the runtime section until the roadmap actually widens beyond `RUN-01/02`?
- Should `HP-A1-06` and `HP-A1-07` be relabeled in the matrix to preserve the current non-claims (`preview seam`, `defer-only explicit boundary`)?

## 12. Suggested next prompt

- `Reconcile P-A1-05 scope docs so specs/18, plan/44, tasks.md, and sub-agent-pro/alpha-1/08-sample-matrix.md agree on the exact admissible transport row set and preserve the current HP-A1-06 / HP-A1-07 non-claims.`

## 13. `plan/` update status

- `plan/` update not performed. Review-only task; repository-memory edits were out of scope.

## 14. Documentation.md update status

- `Documentation.md` update not performed. Reviewed only.

## 15. progress.md update status

- `progress.md` update not performed. Reviewed only.

## 16. tasks.md update status

- `tasks.md` update not performed. Reviewed only.

## 17. samples_progress.md update status

- `samples_progress.md` update not performed. Not edited in this review-only task.

## 18. reviewer findings and follow-up

- Finding 1:
  `P-A1-05` does not have one stable admissible row set across the reviewed docs. The handoff roadmap and `plan/44` say the transport package includes stale-membership, missing-capability, and missing-witness negatives (`sub-agent-pro/alpha-1/04-stage-roadmap.md:75-79`, `plan/44-practical-alpha1-roadmap.md:119-123`), but the sample matrix defines only five transport rows and names just one negative, `TR-A1-03` stale membership (`sub-agent-pro/alpha-1/08-sample-matrix.md:42-49`). The current snapshot narrows `P-A1-05` further to Docker/local TCP plus route trace and separated lanes export, again without transport-specific capability/witness negatives (`tasks.md:130`). This lets one implementation satisfy one authority while violating another.
- Follow-up 1:
  Either add explicit transport rows for capability/witness negatives and mirror them into the matrix and snapshot, or remove those negatives from the `P-A1-05` package definition and keep them as already-closed package/hot-plug evidence.

- Finding 2:
  The sample matrix overstates the runtime substrate available to `P-A1-05`. `plan/44` says the current first cut uses only `RUN-01/02` and explicitly recommends keeping `P-A1-03` as the first local-runtime floor over `RUN-01/02` (`plan/44-practical-alpha1-roadmap.md:208-209`, `plan/44-practical-alpha1-roadmap.md:254-257`). In contrast, the matrix presents `RUN-03` missing capability reject, `RUN-04` missing witness reject, and `RUN-05` fallback degradation trace as plain runtime rows (`sub-agent-pro/alpha-1/08-sample-matrix.md:24-30`). That erases the current lane split: capability/witness negatives are still being carried without full runtime-enforcement claim, while fallback degradation is listed under devtools in the normative scope (`plan/44-practical-alpha1-roadmap.md:259`, `specs/18-practical-alpha1-scope.md:198`, `specs/18-practical-alpha1-scope.md:209-219`).
- Follow-up 2:
  Mark `RUN-03/04/05` as planned-only, move them to later-stage sections, or remove them until the roadmap explicitly widens the runtime floor beyond `RUN-01/02`.

- Finding 3:
  The current non-claims around object attach and detach are not preserved in the sample matrix labels. The normative/snapshot reading says `HP-A1-06` is only a narrow object-package attach preview seam, and `HP-A1-07` is only an explicit deferred detach boundary; full object attach and accepted detach execution remain unclaimed (`specs/18-practical-alpha1-scope.md:150-173`, `plan/44-practical-alpha1-roadmap.md:169-173`, `plan/44-practical-alpha1-roadmap.md:257-258`, `tasks.md:129`). The matrix instead labels them broadly as `object package attach` and `detach-minimal reject/defer/fallback` (`sub-agent-pro/alpha-1/08-sample-matrix.md:39-40`). If `P-A1-05` uses those labels as substrate facts, it will smuggle in stronger prerequisites than the current package line actually closed.
- Follow-up 3:
  Rename the matrix rows to reflect the existing stop line, for example `object package attach preview seam` and `detach-minimal explicit defer boundary`, before using them as dependencies for transport E2E planning.

## 19. skipped validations and reasons

- Did not run `validate_docs.py`, Rust tests, or sample helpers because the task was a narrow document review and the user explicitly restricted edits to a new report file.
- Did not run Docker-related commands because this review was about scope admissibility, not transport implementation evidence.

## 20. commit / push status

- No commit created.
- No push performed.

## 21. sub-agent session close status

- No sub-agent used.
