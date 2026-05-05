# 2034 — P-A1-24 workflow readiness metric recut

## Objective

ユーザ指示に従い、repo の進め方と snapshot 表現を「進捗率」ではなく **外部開発者が再現できる operational workflow** 基準へ recut する。`100%` は operational workflow または product/public layer が外部から実際に使える状態だけに使い、helper / sidecar / report / expected JSON / first-floor runner は completion ではなく evidence として分類する。

## Scope and assumptions

- Docs / repository memory only.
- runtime behavior、sample behavior、expected JSON は変更しない。
- `α-0.5` は local session workflow、`α-0.8` は same-session hot-plug workflow、`α-0.9` は session-bound devtools workflow が再現できるときだけ workflow-ready と読む。
- practical alpha-1 first-floor runners and exact bundles remain evidence. `PA1W-01..08` は bounded practical workflow-ready だが product/public-ready alpha-1 ではない。

## Start state / dirty state

- start state:
  previous pushed HEAD `4afdb02` from `P-A1-23`.
- dirty state at start:
  clean.
- resource check:
  not repeated for this docs-only wording recut; no heavy build or generated artifact work was planned.

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/2033-p-a1-23-practical-alpha1-integrated-workflow-carrier.md`

## Actions taken

1. Replaced `progress.md` rough percentage tables with workflow-readiness / evidence classification tables.
2. Replaced `samples_progress.md` progress legend and `100` evidence rows with `workflow-ready`, `evidence-closed`, `first-floor evidence`, and `planned` classification.
3. Updated `README.md`, `Documentation.md`, and `tasks.md` so helper / sidecar / report / expected JSON / first-floor runner are explicitly evidence, not completion.
4. Updated `specs/18` and `specs/24` so `100%` is reserved for externally usable operational or product/public layers.
5. Updated `plan/44` to record `P-A1-24` as the metric recut and move the next product/public boundary reopen point to `P-A1-25`.
6. Verified no stale percentage-table patterns remained in the touched snapshot docs.

## Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `specs/18-practical-alpha1-scope.md`
- `specs/24-operational-alpha05-alpha08-readiness.md`
- `plan/44-practical-alpha1-roadmap.md`
- `docs/reports/2034-p-a1-24-workflow-readiness-metric-recut.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git status --short
rg -n 'Rough %|Progress|100%|\| 100 \||existing 100|first-floor closeout|current-scope evidence closeout|rough %|進捗率' README.md Documentation.md progress.md tasks.md samples_progress.md specs/18-practical-alpha1-scope.md specs/24-operational-alpha05-alpha08-readiness.md plan/44-practical-alpha1-roadmap.md
date '+%Y-%m-%d %H:%M %Z'
git diff --stat
rg -n 'Rough %|\| 100 \||existing 100|first-floor closeout|current-scope evidence closeout|rough %' README.md Documentation.md progress.md tasks.md samples_progress.md specs/18-practical-alpha1-scope.md specs/24-operational-alpha05-alpha08-readiness.md plan/44-practical-alpha1-roadmap.md
python3 -m unittest scripts.tests.test_validate_docs
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
cargo fmt --check
git diff --check
```

## Evidence / outputs / test results

- `date '+%Y-%m-%d %H:%M %Z'`
  returned `2026-05-05 11:59 JST`.
- `rg -n 'Rough %|\| 100 \||existing 100|first-floor closeout|current-scope evidence closeout|rough %' ...`
  returned exit `1`, meaning no matches remained in the targeted docs.
- `python3 -m unittest scripts.tests.test_validate_docs`
  passed; `11` tests passed.
- `python3 scripts/check_source_hierarchy.py`
  passed; `84/84` required paths present.
- `python3 scripts/validate_docs.py`
  passed; documentation scaffold complete and `1186` numbered reports found.
- `cargo fmt --check`
  passed.
- `git diff --check`
  passed.

## What changed in understanding

- Progress percentage is no longer a primary repo status metric.
- `100%` is too strong for evidence / first-floor / expected JSON rows, even when validation passes.
- The repo should advance by reproducible workflow slices: local session workflow for α-0.5, same-session hot-plug workflow for α-0.8, and session-bound devtools workflow for α-0.9.

## Open questions

- Whether future dashboard rows should eliminate `100%` entirely except for final product/public layer, or keep it for bounded operational workflow lines only.
- Whether `P-A1-25` should produce a user-facing `U1` decision checklist before any more implementation.

## Suggested next prompt

`P-A1-25` として、bounded practical workflow と product/public-ready alpha-1 の差を、進捗率ではなく external developer workflow / public surface / packaging / host target / support boundary の観点から recut してください。

## Plan update status

- updated:
  `plan/44-practical-alpha1-roadmap.md`
- reason:
  workflow-readiness metric recut and next reopen point changed.

## Documentation.md update status

- updated:
  snapshot reading now says progress percentage is not primary, and helper / sidecar / report / expected JSON / first-floor runner are evidence.

## progress.md update status

- updated:
  replaced progress percentage tables with workflow status and evidence classification.

## tasks.md update status

- updated:
  current recommendation now points to `P-A1-25` product/public boundary recut after this metric recut.

## samples_progress.md update status

- updated:
  replaced progress legend and `100` evidence rows with workflow status / evidence classification.

## Reviewer findings and follow-up

- sub-agent reviewers were not spawned because the user did not explicitly ask for sub-agents and active tool policy only permits spawning when explicitly requested.
- local focused review:
  checked touched docs for stale percentage patterns and first-floor completion wording.
- follow-up:
  future reports should describe helper/sidecar/expected-json results as evidence, not completion.

## Skipped validations and reasons

- Rust behavior tests:
  skipped; docs-only policy wording recut. No Rust runtime behavior changed.
- sample behavior tests:
  skipped; no sample runner or expected JSON changed.

## Commit / push status

- package commit:
  pending
- push:
  pending

## Sub-agent session close status

- no sub-agents were opened for this package.
