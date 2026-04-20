# 0847 — once-through completion handoff integration

## Objective

`sub-agent-pro/codex_once_completion_handoff_after_faq010_2026-04-20.md`
を current explanation delta として取り込み、
`Package 91 only` に過度に narrow 化していた queue reading を
repo-local once-through near-end completion の staged package chain へ戻し、
`plan/`、`docs/`、`progress.md`、`tasks.md`、traceability を整合させる。

## Scope and assumptions

- `specs/` を規範正本として扱い、FAQ / handoff は current explanation source としてのみ使う。
- final parser grammar、final public parser/checker/runtime API、final public verifier contract、production theorem/model-check binding、exhaustive shared-space catalog、packaging / FFI / engine adapter はこの task で凍らせない。
- current queue reconstruction は Package 91...98 の staged self-driven sequence として表現するが、これは final public completion claim ではない。
- `plan/12`、`plan/13`、`plan/16` は今回の drift 解消に必須の stale current-line を持たないため、今回は update しない前提で確認した。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `specs/examples/562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
- `specs/examples/563-current-l2-phase6-perform-head-structural-carrier-threshold-helper-mirror.md`
- `sub-agent-pro/codex_final_layer_closeout_handoff_2026-04-19.md`
- `sub-agent-pro/codex_once_completion_handoff_after_faq010_2026-04-20.md`
- `faq_010.md`

## Actions taken

1. FAQ 10 後 handoff と current snapshot/roadmap の drift を確認し、stale current-line を inventory 化した。
2. `specs/examples/564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md` を追加し、Package 91 の compare floor を source-backed に整理した。
3. `Documentation.md` と `specs/00-document-map.md` に 2026-04-20 handoff への導線を追加し、`451...564` / `556...564` の current anchor 読みへ更新した。
4. `tasks.md` と `progress.md` を `Package 91...98` の once-through sequence reading に更新した。
5. `plan/01`、`plan/11`、`plan/17`、`plan/18` を同 reading へ更新し、`plan/90` に traceability entry を追加した。
6. `specs/10`、`specs/11`、`specs/12` の stale wording を必要最小限で同期した。
7. `perform-head-final-public-parser-API comparison` という stale label が残っていた箇所を、current structural-carrier reading に直した。
8. `plan/12`、`plan/13`、`plan/16` は確認のうえ、今回 update 不要と判断した。

## Evidence / outputs / test results

- 追加ファイル:
  - `specs/examples/564-current-l2-phase6-perform-head-request-clause-bundle-attachment-comparison.md`
- 更新ファイル:
  - `Documentation.md`
  - `specs/00-document-map.md`
  - `tasks.md`
  - `progress.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
  - `plan/90-source-traceability.md`
- 実行コマンドと結果:
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
    - pass
  - `cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact`
    - pass
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
    - pass
  - `python3 scripts/validate_docs.py`
    - `Documentation scaffold looks complete.`
  - `git diff --check`
    - no output

## What changed in understanding

- 2026-04-20 handoff は新しい規範判断そのものではなく、
  already source-backed な current first lines を
  `Package 91 only` の局所 reading から
  `Package 91...98` の repo-local once-through sequence に戻すための explanation source と読むのが自然だった。
- drift の主因は theory gap ではなく queue presentation gap であり、
  `Package 92...98` を snapshot/roadmap に visible に戻すだけで、
  self-driven scope と stop line がかなり明確になった。
- `specs/examples/564` を compare-floor anchor として追加することで、
  parser-side closeout と strong typing / Lean / order-handoff / room-profile の次 package 群へ自然に接続できるようになった。

## Open questions

- Package 91 compare floor を actual helper-local carrier に上げる際、`Stage3RequestHeadClauseBundle` の manifest 名や code anchor の exact naming をどこで固定するか。
- Package 92 で capture escape / cost bound sample をどの path に置くか。
- Package 94 以降で theorem/model-check helper preview をどこまで code carrier に寄せるか。

## Suggested next prompt

Package 91 の compare floor を actual helper-local carrier / test に落とし、`Stage3RequestHeadClauseBundle` の minimum cut をコードと sample explanation に actualize してください。続けて Package 92 の finite-index strong typing first layer を `p06 / p10 / p11 / p12` と capture/cost sample へ広げてください。
