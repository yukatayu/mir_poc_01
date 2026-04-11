# 0616 — phase6 parser second tranche attached slot and predicate fragment first package

- Date: 2026-04-11T17:06:14.927063Z
- Author / agent: Codex
- Scope: Phase 6 parser second tranche first package として、stage3 declaration-side admit attached slot と shared isolated predicate fragment を `mir_ast::current_l2` に actualize する。
- Decision levels touched: current L2 / L2 non-production parser carrier widening

## 1. Objective

- parser second tranche first package を source-backed actual code anchor に落とす。
- stage3 admit-slot / predicate route を crate surface reuse へ寄せる。
- multiline attachment / request clause suite / perform head / richer diagnostics を still later に残す。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/83-current-l2-stage3-admit-slot-branch-comparison.md`
- `specs/examples/85-current-l2-stage3-admit-slot-first-tranche-actualization.md`
- `specs/examples/92-current-l2-stage3-predicate-fragment-reopen-sequencing.md`
- `specs/examples/94-current-l2-stage3-predicate-fragment-first-tranche-actualization.md`
- `specs/examples/299-current-l2-phase5-proof-protocol-runtime-policy-handoff-closeout-ready-phase6-actual-parser-ast-carrier-first-tranche-comparison.md`
- `specs/examples/300-current-l2-phase6-actual-parser-ast-carrier-first-tranche-ready-minimal-phase6-actual-parser-ast-carrier-first-tranche-threshold.md`
- `specs/examples/305-current-l2-phase6-compile-ready-checkpoint-close-ready-phase6-next-reopen-sequencing-comparison.md`
- `specs/examples/306-current-l2-phase6-next-reopen-sequencing-ready-minimal-phase6-next-reopen-sequencing-threshold.md`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`

## 3. Actions taken

1. parser second tranche first package の最小 cut を再確認し、attached-slot / predicate route を `perform head` / request suite より先に actualize するのが current minimum だと整理した。
2. TDD の red として stage3 admit-slot / predicate-fragment / multiline-attachment / request-clause-suite tests を crate import へ寄せ、`cargo test -p mir-ast --test ...` を実行して `mir_ast::current_l2` に stage3 parser surface が足りない compile failure を確認した。
3. `crates/mir-ast/src/current_l2.rs` に stage3 declaration-side admit attached slot carrier と shared isolated predicate fragment parser を追加した。
4. `crates/mir-ast/src/lib.rs` の crate docs を更新し、stage3 attached-slot / predicate fragment が current non-production carrier に入ったこと、multiline attachment / request suite / perform head などは later であることを明示した。
5. stage3 admit-slot / predicate support helper を crate type reuse へ寄せ、multiline attachment / request clause suite tests も same predicate parser を reuse する line に揃えた。
6. `specs/examples/307...308`、`Documentation.md`、`progress.md`、`tasks.md`、`specs/00-document-map.md`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/17`、`plan/90`、`docs/research_abstract/phase6...` を current mainline へ合わせて更新した。

## 4. Files changed

- `crates/mir-ast/src/lib.rs`
- `crates/mir-ast/src/current_l2.rs`
- `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_predicate_fragment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_multiline_attachment_spike.rs`
- `crates/mir-ast/tests/current_l2_stage3_request_clause_suite_spike.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs`
- `crates/mir-ast/tests/support/current_l2_stage3_predicate_fragment_spike_support.rs`
- `specs/examples/307-current-l2-phase6-next-reopen-sequencing-ready-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-comparison.md`
- `specs/examples/308-current-l2-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-ready-minimal-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package-threshold.md`
- `specs/00-document-map.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/90-source-traceability.md`
- `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`
- `docs/reports/0616-phase6-parser-second-tranche-attached-slot-and-predicate-fragment-first-package.md`

## 5. Commands run and exact outputs

```bash
cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike --test current_l2_stage3_predicate_fragment_spike --test current_l2_stage3_multiline_attachment_spike --test current_l2_stage3_request_clause_suite_spike
cargo test -p mir-ast
python3 scripts/validate_docs.py
git diff --check
git status --short
```

## 6. Evidence / findings

- red verification:
  - `cargo test -p mir-ast --test current_l2_stage3_admit_slot_spike --test current_l2_stage3_predicate_fragment_spike --test current_l2_stage3_multiline_attachment_spike --test current_l2_stage3_request_clause_suite_spike`
  - `error[E0432]: unresolved import mir_ast::current_l2::parse_stage3_minimal_predicate_fragment_text`
- green verification:
  - same selected stage3 suite が通過
- multiline attachment / request clause suite 自体は still support-local structural helper だが、predicate parser reuse だけは crate surface に揃った。

## 7. Changes in understanding

- parser second tranche first package の最小 actualization は、attached-slot と isolated predicate fragment を crate surface に上げるだけで十分だった。
- multiline attachment / request clause suite は parser second tranche first package の code anchor には参加できるが、まだ crate public-ish surface に昇格させる必要はない。
- stage3 tests を crate parser reuse に寄せることで、later structural helper と actual code anchor の境界を狭く保てる。

## 8. Open questions

- shared single attachment frame を次 package で actualize するか
- request clause suite publicization をいつ library side へ上げるか
- theorem-first reserve line を notebook-pressure wording までどう mirror するか

## 9. Suggested next prompt

```text
Phase 6 reserve formal tool binding inventory を進め、theorem-first / model-check reserve line を docs と plan に整理してください。
```
