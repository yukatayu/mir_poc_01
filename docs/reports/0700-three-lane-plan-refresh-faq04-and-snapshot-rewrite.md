# 0700 — three-lane plan refresh, FAQ 04, and snapshot rewrite

## Objective

`plan/` と `docs/` を、current repo を **execution / theory-lab / reserve integration** の 3 lane で読む形へ整理し直し、
その後で `progress.md` と `tasks.md` を薄い snapshot として全面更新し、
`faq_003.md` を継ぎ足さずに新規版 `faq_04.md` を追加する。

## Scope and assumptions

- 規範判断の正本は `specs/` に置いたままにする。
- 今回は mainline actualization を進める task ではなく、docs / plan / snapshot の再構成 task として扱う。
- `faq_003.md` は historical FAQ として残し、current explanation は `faq_04.md` に切り出す。
- final parser grammar、final public API、concrete theorem / model-check tool binding、shared-space final catalog は決めない。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `.docs/progress-task-axes.md`
- `faq_003.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/06-surface-notation-status.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase4-shared-space-membership-and-practical-room-boundary.md`
- `docs/research_abstract/phase5-small-decidable-core-and-proof-boundary.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/357...412`

## Actions taken

1. `Documentation.md` を concise current summary に置き換え、three-lane reading と `faq_04.md` の導線を追加した。
2. `plan/00`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/18` を、execution / theory-lab / reserve integration の 3 lane を前提に再構成した。
3. `plan/08` の stale current-line wording を current lane split に合わせて修正した。
4. `docs/research_abstract/phase4`、`phase5`、`phase6` の stale current-line wording を整理し、phase6 abstract は current decet / three-lane reading に合わせて書き直した。
5. `progress.md` と `tasks.md` を append ではなく snapshot rewrite として全面更新した。
6. `faq_04.md` を新規追加し、current reading を Q&A 形式で整理した。
7. `plan/90-source-traceability.md` に今回の addendum を追加する前提で traceability 根拠を整理した。
8. reviewer を 1 回だけ回し、reserve-lane wording、FAQ の順序 drift、`progress.md` の autonomy nuance、missing report citation を吸収した。
9. `specs/`、`plan/06`、`plan/16`、`.docs/`、`faq_003.md` は今回の scope では更新不要と判断し、そのまま残した。

## Evidence / outputs / test results

- pre-report validation:
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 698 numbered report(s).`
  - `git diff --check`
    - 無出力
- reviewer:
  - 4 件の medium/low finding を返した
  - reserve-lane wording、FAQ 順序、`progress.md` autonomy nuance、report citation を修正して吸収した
- final validation:
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
    - `Found 699 numbered report(s).`
  - `git diff --check`
    - 無出力
  - reviewer narrow re-review:
    - `No remaining actionable findings in the four previously flagged areas.`

## What changed in understanding

- current repo を `current promoted line` 1 本だけで読むと、theory-lab line が埋もれやすい。3 lane split の方が current reading を誤読しにくい。
- `progress.md` と `tasks.md` は、plan の detail を再列挙するよりも、line / order / blocker だけを残す方が repository memory として健全である。
- `faq_003.md` を current 説明として延命するより、historical FAQ と current FAQ を分けた方が整合性監査しやすい。

## Open questions

- typed first attachment candidate を semantic carrier / checker boundary / source-visible surfaceのどこに置くか
- semantic-core theorem pilot の first lemma order
- model-check first property family
- shared-space room-profile / confusion-replay note の current reserve cut
- host binding bridge-only note の wording と stop line

## Suggested next prompt

`tasks.md` の先頭どおり、theory-lab lane の package 1〜3（typed-core attachment inventory、semantic-core theorem pilot planning、model-check projection/property-family reserve inventory）を順に閉じ、必要なら package close ごとに mirror と FAQ の drift を再監査してください。
