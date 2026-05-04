# 2035 — Selective Main Merge Of FAQ015

## Objective

ユーザ依頼に従い、current docs branch から main へ取り込むべき file を切り分け、必要最小限の file だけを main へ selective merge して push する。

## Scope and assumptions

- main へ取り込む対象は、これまでの user 指示と current branch diff から判断して `tmp_faq/faq_015.md` のみとする。
- `docs/reports/2028..2034` は task trace であり、main の selective merge 対象には含めない。
- 既存 untracked report `1177...` と `2027...` は今回も対象外とする。

## Start state / dirty state

- start branch: `docs/layered-repro-guide-001`
- start dirty state:
  - untracked: `docs/reports/1177-layered-repro-guide-001-readonly-repro-audit.md`
  - untracked: `docs/reports/2027-mir-bottom-layer-readonly-explanation-001.md`
- branch diff against `main` at task start:
  - `tmp_faq/faq_015.md`
  - `docs/reports/2028..2034`

## Documents consulted

- `tmp_faq/faq_015.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- user messages in this thread about FAQ accumulation and later selective manual/main integration intent

## Actions taken

- current branch diff against `main` を確認した。
- main に持っていく対象を `tmp_faq/faq_015.md` のみに限定した。
- FAQ に selective merge 運用メモを一行追加した。
- docs branch 側にこの report を作成した。
- docs branch の FAQ/report commit を作成した。
- docs branch commit を push した。
- main へ切り替え、`tmp_faq/faq_015.md` だけを current docs branch から checkout して commit / push した。
- 必要に応じて docs branch report の commit/push status を更新し、最後に main へ戻る。

## Files changed

- `tmp_faq/faq_015.md`
- `docs/reports/2035-selective-main-merge-of-faq015.md`

## Commands run

```bash
python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .
git branch --show-current
git status --short
git diff --name-status main...HEAD
git ls-tree --name-only main tmp_faq
git show main:tmp_faq/faq_014.md | sed -n '1,40p'
git rev-parse --short HEAD
git rev-parse --short main
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
git add tmp_faq/faq_015.md docs/reports/2035-selective-main-merge-of-faq015.md
git commit --no-gpg-sign -m "docs: prepare selective main merge for faq015"
git push origin docs/layered-repro-guide-001
git switch main
git pull --ff-only origin main
git checkout docs/layered-repro-guide-001 -- tmp_faq/faq_015.md
git add tmp_faq/faq_015.md
git commit --no-gpg-sign -m "docs: add faq 015 rolling explanation memo"
git push origin main
```

## Evidence / outputs / test results

- `git diff --name-status main...HEAD`
  - branch diff は `tmp_faq/faq_015.md` と `docs/reports/2028..2034`
- current reading:
  - main に取り込む value があるのは FAQ 本体
  - reports は docs branch 側の task trace として残すのが自然
- docs branch push:
  - `381ac2e962d29384818e8d6d60691138424f65d1`
  - message: `docs: prepare selective main merge for faq015`
- main selective merge push:
  - `dae4a20bfdec65a1dd6f7944f7b38448a7ec3ea3`
  - message: `docs: add faq 015 rolling explanation memo`
- `python3 scripts/check_source_hierarchy.py`
  - pass
- `python3 scripts/validate_docs.py`
  - pass
- `git diff --check`
  - pass

## What changed in understanding

- earlier user intent と current branch diff を合わせると、main へ持っていくべき file は FAQ 本体だけと読むのが自然である。
- reports を main へ持ち込むと、helper memo と task trace の selective merge という意図から外れやすい。

## Open questions

- なし。current user request に必要な selective merge target は十分に特定できた。

## Suggested next prompt

`main に入った faq_015 を前提に、next capability milestone を lower-bound ベースで切り直してください。`

## Plan update status

- `plan/` 更新不要。
- selective merge 作業のみで repository-memory judgement は変えていない。

## Documentation.md update status

- `Documentation.md` 更新不要。

## progress.md update status

- `progress.md` 更新不要。

## tasks.md update status

- `tasks.md` 更新不要。

## samples_progress.md update status

- `samples_progress.md` 更新不要。

## Reviewer findings and follow-up

- reviewer / sub-agent は未使用。

## Skipped validations and reasons

- runtime / transport / save-load rerun
  - file selection と git integration task であり、新しい behavior claim は無い。

## Commit / push status

- docs branch committed and pushed:
  - `381ac2e962d29384818e8d6d60691138424f65d1`
  - message: `docs: prepare selective main merge for faq015`
- main committed and pushed:
  - `dae4a20bfdec65a1dd6f7944f7b38448a7ec3ea3`
  - message: `docs: add faq 015 rolling explanation memo`
- current file may receive a small follow-up status-only commit if later push metadata needs explicit mirroring.

## Sub-agent session close status

- sub-agent session なし。
