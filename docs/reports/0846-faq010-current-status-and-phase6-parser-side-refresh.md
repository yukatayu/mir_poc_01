# 0846 — FAQ 010 current-status and phase6 parser-side refresh

## Objective

2026-04-20 時点の current reading を再確認し、

- 現状どこまで終わっているか
- 二大問題を completely solved / language implementation complete と読めるか
- 全体像に対して今どこにいるか
- 何が still mixed gate / true user-spec gate か
- どこまでなら現実的に自走できるか

を `faq_010.md` に正確にまとめる。

併せて、FAQ を current explanation delta として document map / traceability から辿れるようにする。

## Scope and assumptions

- 今回の更新は current explanation refresh であり、新しい規範判断は作らない
- 規範判断の正本は `specs/` に残す
- current snapshot を変える task ではないため、`progress.md` と `tasks.md` は内容に drift がないか確認し、更新不要ならそのままにする
- `faq_009.md` 以後の genuine progress は、主として phase6 parser-side / checker-runtime-side actualization と first strong typing layer default 明確化として読む

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
- `faq_009.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/examples/552-current-l2-phase6-actual-parser-ast-carrier-first-tranche-threshold-helper-mirror.md`
- `specs/examples/553-current-l2-phase6-actual-checker-runtime-skeleton-first-tranche-threshold-helper-mirror.md`
- `specs/examples/554-current-l2-phase6-compile-ready-verification-and-formal-hook-threshold-helper-mirror.md`
- `specs/examples/555-current-l2-phase6-next-reopen-sequencing-threshold-helper-mirror.md`
- `specs/examples/556-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold-helper-mirror.md`
- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`
- `specs/examples/558-current-l2-phase6-reserve-formal-tool-binding-inventory-threshold-helper-mirror.md`
- `specs/examples/559-current-l2-phase6-parser-side-follow-up-package-sequencing-threshold-helper-mirror.md`
- `specs/examples/560-current-l2-phase6-parser-second-tranche-shared-single-attachment-frame-first-package-threshold-helper-mirror.md`
- `specs/examples/561-current-l2-fixed-subset-source-sample-corpus-scope-and-file-layout-threshold-helper-mirror.md`
- `specs/examples/562-current-l2-phase6-request-clause-suite-publicization-threshold-helper-mirror.md`
- `specs/examples/563-current-l2-phase6-perform-head-structural-carrier-threshold-helper-mirror.md`
- `docs/reports/0844-package89-request-clause-suite-publicization.md`
- `docs/reports/0845-package90-perform-head-structural-carrier.md`

## Actions taken

1. AGENTS 指示に従い、status 問い合わせ向けの必読順で `README.md`、`Documentation.md`、`progress.md`、`specs/00...03`、`specs/09` を再読した。
2. `faq_009.md` と current snapshot / roadmap / autonomy gate を照合し、2026-04-20 時点で genuinely progressed した点を抽出した。
3. phase6 implementation line の current status を、`specs/examples/552...563` と Package 89/90 report を根拠に再確認した。
4. `faq_010.md` を新規作成し、
   - done / not done
   - overall ladder
   - twin peaks current state
   - mixed gate / true user-spec gate
   - self-drive bound
   を current reading に沿って整理した。
5. `Documentation.md`、`specs/00-document-map.md`、`plan/90-source-traceability.md` を更新し、`faq_010.md` を current explanation delta として辿れるようにした。
6. `progress.md` と `tasks.md` は current snapshot と一致していることを確認し、今回は更新不要と判断した。

## Evidence / outputs / test results

- fresh status evidence
  - `cargo test -p mir-ast --test current_l2_request_clause_suite_manifest current_l2_request_clause_suite_manifest_keeps_fixed_two_slot_cut -- --exact`
  - `cargo test -p mir-ast --test current_l2_perform_head_manifest current_l2_perform_head_manifest_keeps_minimum_cut -- --exact`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_late_join_order_handoff_prototype -- --exact`
- docs validation
  - `python3 scripts/validate_docs.py`
  - `git diff --check`

## What changed in understanding

- `faq_009.md` 以後の genuine progress は、representative Lean execution の追加そのものより、
  phase6 parser-side / checker-runtime-side narrow actualization が code anchor 付きで積み上がった点にある。
- current repo は「理論だけが残っている」段階ではなく、current first line と helper-local actualization を implementation-side closeout に接続している。
- それでも final public language implementation complete とはまだ言えず、current near-end target と full final-public completion を明確に分けて読む必要がある。

## Open questions

- perform head と request clause suite をどの combined carrier で narrow に束ねるか
- final parser grammar / final public parser-checker-runtime API をどの reopen 条件で扱うか
- final public verifier contract をどの mixed gate まで deferred に保つか
- final public witness/provider/artifact contract と exhaustive shared-space catalog をどこで分け続けるか

## Suggested next prompt

Package 91 として、perform head structural carrier と request clause suite carrier の
bundle attachment comparison を current next parser-side package として進め、
combined carrier minimum を選びつつ、
diagnostics / final grammar / final public parser API を still later に保ったまま closeout を続けてください。
