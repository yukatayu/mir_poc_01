# Report 0725 — typed/theorem/model-check corrected prototype tranche

- Date: 2026-04-17T05:19:00Z
- Author / agent: Codex
- Scope: current L2 prototype sextet expansion, typed/theorem/model-check sample-visible corrected prototype tranche, snapshot synchronization
- Decision levels touched: L2 current design proposal, L3 helper-local prototype convenience

## 1. Objective

`admit` / `require` / `ensure` を含む corrected prototype を runnable に追加し、
current theorem/model-check bridge floor が runtime reached でどう見えるかを sample-visible にする。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/prototype/README.md`
- `faq_005.md`
- `docs/reports/0722-sample-verification-preview-and-prototype-second-tranche.md`
- `docs/reports/0723-sample-artifact-preview-third-tranche.md`
- `docs/reports/0724-underdeclared-source-actualization-and-artifact-preview-widening.md`

## 3. Actions taken

1. `samples/prototype/current-l2-typed-proof-model-check/` を追加した。
2. `p06-typed-proof-owner-handoff.txt` と adjacent host-plan sidecar を追加した。
3. runner / CLI test を先に追加し、missing file failure を確認した。
4. sample / host-plan を追加して、typed marker を含む runtime prototype として green にした。
5. `specs/examples/456` を追加し、current package judgment を docs-first に固定した。
6. `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`faq_005.md`、`samples/prototype/README.md` を prototype sextet と next queue に同期した。

## 4. Files changed

- Added:
  - `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt`
  - `samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.host-plan.json`
  - `specs/examples/456-current-l2-typed-theorem-model-check-sample-visible-corrected-prototype-tranche.md`
- Modified:
  - `crates/mir-runtime/tests/current_l2_source_sample_runner.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
  - `samples/prototype/README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `faq_005.md`
  - `specs/00-document-map.md`

## 5. Commands run and exact outputs

- RED:
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_typed_bridge_prototype_path -- --exact --nocapture`
    - `FAILED`
    - `No such file or directory`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_typed_bridge_prototype_preview -- --exact --nocapture`
    - `FAILED`
    - `source sample not found`
- GREEN:
  - `cargo test -p mir-runtime --test current_l2_source_sample_runner current_l2_source_sample_runner_accepts_typed_bridge_prototype_path -- --exact --nocapture`
    - `1 passed`
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_pretty_reports_typed_bridge_prototype_preview -- --exact --nocapture`
    - `1 passed`

## 6. Evidence / findings

- `p06` は runtime success まで到達し、`proof_debug_text_output` に 2 record を残す。
- `p06` は helper-local `verification_preview` で `runtime_try_cut_cluster` reached を示す。
- `p06` は helper-local `artifact_preview` で proof notebook review unit / model-check concrete carrier preview を表示する。
- typed marker family を prototype で見せても、public typed/theorem/model-check contract を増やさずに current bridge floor の usability を比較できる。

## 7. Changes in understanding

- p01/p04/p05 だけだと order/handoff の runtime/static/guarded 対比は見えるが、typed marker family が current bridge floor にどう映るかは弱かった。
- `p06` を別 bucket に切ることで、typed marker と theorem/model-check preview を sample-visible に保ちつつ、source-authored current corpus を増やさずに比較できる。

## 8. Open questions

- stronger typed surface promotion の threshold 自体は未決である。
- theorem discharge transport / public theorem contract は mixed gate に残る。
- model-check property language / concrete tool seam も mixed gate に残る。

## 9. Suggested next prompt

`samples/prototype/` の order/handoff corrected prototype third tranche を追加し、late join / stale message / publication-handoff difference のどれを current L2 corrected sample に落とせるか比較してください。
