# 0715 — Macro 0〜5 closeout refresh and consistency audit

## Objective

`docs/reports/0714` で追加した `specs/examples/444...450` を受けて、
`Macro 0〜5` が current scoped reading で closeout fixed と読める状態を

- `progress.md`
- `tasks.md`
- `Documentation.md`
- `plan/`
- `faq_005.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`

へ mirror し、stale wording を除去する。

## Scope and assumptions

- 監査の主眼は snapshot / memory / FAQ / document-map の整合であり、mainline code は変更しない。
- `specs/11-roadmap-and-workstreams.md`、`specs/12-decision-register.md`、`plan/13-heavy-future-workstreams.md`、`.docs/` は今回更新不要と判断してよい。
- historical log wording は履歴として残し、current snapshot section だけを current reading に合わせる。

## Documents consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `faq_004.md`
- `faq_005.md`
- `samples/current-l2/README.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `docs/reports/0714-macro5-closeout-and-reserve-boundary-six-packages.md`

## Actions taken

1. `progress.md` を全面書き換えし、`Macro 5 boundary / pilot / framing closeout fixed` と `Macro 6/7 mixed-gate boundary fixed` を current line にした。
2. `tasks.md` の timestamp を更新し、current self-driven queue が空である current reading を維持した。
3. `Documentation.md`、`plan/00`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/16`、`plan/17`、`plan/18` を新 reading に揃えた。
4. `faq_004.md` と `faq_005.md` の stale current-line wording と rough progress 数値を更新した。
5. `specs/00-document-map.md` と `specs/10-open-questions.md` に `444...450` 起因の導線と OPEN の更新を追加した。
6. `plan/90-source-traceability.md` に今回の addendum を追加した。

## Files changed

- `progress.md`
- `tasks.md`
- `Documentation.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `faq_004.md`
- `faq_005.md`
- `samples/current-l2/README.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`

## Commands run and exact outputs

- `date '+%Y-%m-%d %H:%M JST'`
  - `2026-04-17 09:34 JST`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 714 numbered report(s).`
- `git diff --check`
  - 無出力

## Evidence / findings

- `tasks.md` だけを書き換えた状態では、`progress.md` と `plan/11` が古い self-driven queue を残していた。
- `specs/10-open-questions.md` も、modality trigger や later-gate framing が未 fixed であるかのように読める箇所があり、`444...449` を踏まえて更新が必要だった。
- `faq_005.md` の rough progress 数値と current phase も stale だったため、twin peaks の現況と一致するように修正した。

## Changes in understanding

- `Macro 5 closeout fixed` は「理論が solved」である意味ではなく、current self-driven boundary / pilot / framing package が尽きた、という意味に限る。
- そのため snapshot 側では、completion と mixed-gate reserve を同時に書く必要がある。

## Open questions

- mixed-gate topic の priority reorder
- mixed-gate topic をどこまで self-driven note として続けるか
- historical FAQ をどこまで current snapshot と同期し続けるか

## Suggested next prompt

`tasks.md` に残っている mixed-gate topic のうち、どれを先に reopen するかを `plan/11` と `progress.md` の twin peaks を見ながら再順序づけし、必要なら 1 package ずつ docs-first に切ってください。
