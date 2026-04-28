# Report 0974 — `U1` post-`P18` true user-spec hold option matrix closeout

- Date: 2026-04-28
- Author / agent: Codex
- Scope: `U1` closeout、front-door / plan / snapshot sync、next promoted line の `R1` への切り替え
- Decision levels touched: L2 / L3 documentation boundary only。L0 / L1 semantics の新規固定は行わない

## 1. Objective

`P18` repo-side first-cut closeout の後に残る true user-spec hold line を
option inventory と provisional recommendation に整理し、
actual product decision や final public freeze を進めずに
repo docs / plan / snapshot を current line へ揃える。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `docs/hands_on/current_phase_closeout_01.md`
- `docs/hands_on/public_api_parser_gate_01.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/research_abstract/public_api_parser_gate_plan_01.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/19-repository-map-and-taxonomy.md`
- `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
- `specs/10-open-questions.md`
- `docs/reports/0973-p18-public-api-parser-gate-first-cut-closeout.md`
- explorer read-only findings:
  `Cicero` on `VerificationLayer` widening-threshold next package selection
  `Ohm` on `AttachPoint` / detach-contract alternative next package selection

## 3. Actions taken

1. `U1` option matrix を repository memory と reader-facing summary / landing page に整理した。
2. `installed binary / packaging adoption target`、`host integration target`、`first shipped public surface scope`、`final shared-space operational catalog breadth` を current user-spec hold axes として docs-first に固定した。
3. `public_api_parser_gate_*` reader docs に first shipped public surface scope を戻し、`P18` と `U1` の境界を揃えた。
4. `progress.md`、`tasks.md`、`samples_progress.md`、`plan/01`、`plan/11`、front-door docs を `U1 close -> R1 promoted next line` へ同期した。
5. next self-driven package は `R1` `VerificationLayer` widening threshold inventory を採用した。
   - 採用理由:
     `U1` の `two-step split` provisional recommendation と整合し、
     user choice を増やさずに core-side public-surface narrowing criteria を詰められるため。
   - alternative noted:
     `AttachPoint compatibility / detach lifecycle` minimal contract は候補として妥当だが、
     integration-surface / final hot-plug ABI 方向へ滑りやすいため next-after-`R1` 候補に留めた。

## 4. Files changed

- new:
  - `plan/28-post-p18-true-user-spec-hold-option-matrix.md`
  - `docs/research_abstract/post_p18_true_user_spec_hold_option_matrix_01.md`
  - `docs/hands_on/post_p18_true_user_spec_hold_01.md`
  - `docs/reports/0974-u1-post-p18-true-user-spec-hold-option-matrix-closeout.md`
- updated:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `docs/hands_on/README.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/hands_on/public_api_parser_gate_01.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/research_abstract/public_api_parser_gate_plan_01.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/19-repository-map-and-taxonomy.md`
  - `plan/90-source-traceability.md`
  - `specs/10-open-questions.md`
- untouched but dirty in worktree:
  - `crates/mir-ast/examples/current_l2_inspect_request_head_clause_bundle.rs`
  - `crates/mir-ast/src/current_l2.rs`

## 5. Commands run and exact outputs

```bash
python3 scripts/check_source_hierarchy.py
```

Exact output:

```text
source hierarchy check
  repo_root: /home/yukatayu/dev/mir_poc_01
  required: 23
  present: 23
  missing: 0
  all required paths present
```

```bash
python3 scripts/validate_docs.py
```

Exact output:

```text
Documentation scaffold looks complete.
Found 972 numbered report(s).
```

```bash
git diff --check
```

Exact output:

```text
(no output)
```

## 6. Evidence / findings

- `U1` では actual product target を選ばず、
  option inventory と provisional recommendation だけを current line に置いた。
- `library-first`、`native-process-first`、`two-step split`、`minimal subset keep` は
  current recommendation であり、final adoption ではない。
- `P18` reader docs 側に first shipped public surface scope を戻し、
  post-`P18` line が packaging / host / shipped-surface / catalog breadth の 4 軸で読めるようにした。
- next promoted line は `R1` `VerificationLayer` widening threshold inventory とし、
  helper `verification_handoff_witness` / runtime `verification_model_check` を current emitted floor に保ったまま、
  widening threshold を docs-first に詰める line へ送った。

## 7. Changes in understanding

- `U1` close後に直ちに user input が必要というより、
  repo-side で先に narrow できる research package がまだ残っている。
- その中でも `VerificationLayer` widening threshold は、
  `U1` の first shipped public surface `two-step split` provisional recommendation と
  最も素直に整合する。
- `AttachPoint` / detach minimal contract は価値が高いが、
  hot-plug final ABI / rollback / migration engine 側へ drift しやすいため
  `R1` の後段候補として扱う方が安全である。

## 8. Open questions

- actual installed binary / packaging adoption target
- actual host integration target adoption
- actual first shipped public surface adoption
- actual final shared-space operational catalog adoption
- `VerificationLayer` family をどの widening threshold で emitted-layer candidate に上げるか
- `AttachPoint` compatibility / detach lifecycle minimal contract をどの package で promoted するか

## 9. Suggested next prompt

`R1` `VerificationLayer` widening threshold inventory を実施し、helper `verification_handoff_witness` / runtime `verification_model_check` を current emitted floor に保ったまま、machine-check / theorem bridge / runtime policy / visualization-telemetry の widening threshold matrix、stop line、reader-facing summary、report を追加してください。hidden verifier builtin、final public verifier contract、final public layer law schema、concrete external theorem / model-check binding は固定しないでください。
