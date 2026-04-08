# Report 0325 — current L2 stage 3 suite reopen conditions

- Date: 2026-04-08T08:07:44.984409Z
- Author / agent: Codex
- Scope: Phase 3 later branch における fixed two-slot suite bridge family の remaining hidden wording と fixture-side full request contract compare reopen 条件の docs-only comparison
- Decision levels touched: L2

## 1. Objective

`specs/examples/106-current-l2-stage3-missing-ensure-first-tranche-actualization.md` の直後に、
remaining suite malformed wording family をまだ追うべきか、それとも fixture-side full request contract compare を narrow に reopen してよいかを source-backed に比較する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/90-current-l2-stage3-request-local-clause-spillover-comparison.md`
- `specs/examples/91-current-l2-stage3-request-local-clause-spillover-first-tranche-actualization.md`
- `specs/examples/97-current-l2-stage3-multiline-attachment-first-tranche-actualization.md`
- `specs/examples/99-current-l2-stage3-request-clause-suite-structural-floor.md`
- `specs/examples/100-current-l2-stage3-request-clause-suite-first-tranche-comparison.md`
- `specs/examples/101-current-l2-stage3-request-clause-suite-first-tranche-actualization.md`
- `specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md`
- `specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md`
- `specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md`
- `specs/examples/105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md`
- `specs/examples/106-current-l2-stage3-missing-ensure-first-tranche-actualization.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## 3. Actions taken

1. fixed two-slot suite bridge helper の current hidden fail-closed path を列挙した。
2. それぞれについて、
   - suite family 固有の前進量があるか
   - shared attachment helper や clause-token family と evidence が重複するか
   を比較した。
3. fixture-side full request contract compare を reopen する条件を、
   - request head parse 非導入
   - two-slot carrier 維持
   - fixture-side contract subset compare 限定
   として切った。
4. comparison judgment を `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md` に追加し、mirror を更新した。

## 4. Files changed

- `specs/examples/107-current-l2-stage3-suite-reopen-conditions.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0325-current-l2-stage3-suite-reopen-conditions.md`

## 5. Commands run and exact outputs

```bash
date '+%Y-%m-%d %H:%M:%S %Z'
df -h .
free -h
git status --short --branch
sed -n '1,180p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,220p' progress.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,220p' specs/examples/102-current-l2-stage3-suite-malformed-vs-request-compare-sequencing.md
sed -n '1,220p' specs/examples/103-current-l2-stage3-suite-malformed-first-pair-comparison.md
sed -n '1,220p' specs/examples/104-current-l2-stage3-suite-malformed-first-pair-actualization.md
sed -n '1,220p' specs/examples/105-current-l2-stage3-missing-ensure-vs-request-compare-sequencing.md
sed -n '1,220p' specs/examples/106-current-l2-stage3-missing-ensure-first-tranche-actualization.md
sed -n '1,260p' crates/mir-ast/tests/support/current_l2_stage3_request_clause_suite_spike_support.rs
sed -n '1,260p' crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs
sed -n '1,260p' crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs
sed -n '1,260p' crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs
```

主要出力:

- resource check:
  - `Filesystem /dev/vda2 99G total / 89G used / 5.9G avail / 94%`
  - `Mem 960Mi total / 711Mi used / 72Mi free / 248Mi available`
- `git status --short --branch`
  - comparison 開始時は `## main...origin/main` で clean

## 6. Evidence / findings

- suite helper が still 持つ hidden wording のうち、`missing multiline predicate block after require:` は shared single attachment frame helper で already source-backed であり、request-local block-inside blank line wording も suite owner / ordering / termination より multiline block-internal policy に寄るため、suite bridge family 固有の前進量が弱い。
- bare clause payload 欠落 (`missing predicate fragment after \`require\` / \`ensure\``) は clause-token / fragment family に近く、suite owner / ordering / termination floor 固有の前進量が弱い。
- `unexpected nested continuation outside request-local clause block` を次に追うと generic continuation / diagnostics widening に寄りやすい。
- 一方、source-side helper output は already `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` まで actualize 済みであり、fixture-side contract subset compare を narrow に reopen する条件は整っている。

## 7. Changes in understanding

- fixed two-slot suite bridge family の current phase における固有 hidden path は、meaningful な範囲では already surfaced 済みだと整理した。
- next narrow step は suite malformed wording family の追加 actualizationではなく、fixture-side full request contract subset compare の first cut comparison へ移してよいと判断した。
- ただし reopen は request head parse 非導入・two-slot carrier 維持・fixture-side contract subset 限定という条件つきである。

## 8. Open questions

- fixture-side full request contract compare の first cut を
  - ad-hoc per-slot compare のままにするか
  - dedicated helper-local contract subset carrier に切るか
  のうちどこに置くか。
- suite helper 内に残る generic continuation wording を、later diagnostics family でどう吸うか。
- orphan になっている alias file `specs/examples/105-current-l2-stage3-missing-ensure-block-vs-request-compare-sequencing.md` を別 task で整理するか。

## 9. Suggested next prompt

`specs/examples/107-current-l2-stage3-suite-reopen-conditions.md` を前提に、fixed two-slot suite bridge を fixture-side full request contract subset compare へどこまで actualize してよいかを narrow に比較し、可能なら helper-local / test-only first tranche まで actualize してください。`
