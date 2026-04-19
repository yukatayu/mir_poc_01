# Report 0758 — theorem discharge public contract threshold default

- Date: 2026-04-18T12:59:44.439357Z
- Author / agent: Codex (GPT-5)
- Scope: `M1` theorem discharge / public-theorem-contract threshold package の close、helper-local threshold actualization、`specs/` / `plan/` / snapshot 同期
- Decision levels touched: L2

## 1. Objective

`specs/examples/479` と `474` の間にある theorem mixed gate を compare-floor のままにせず、

- review-unit-first
- discharge-entry-adjacent
- notebook-consumer-first
- brand-neutral theorem request

を current default threshold として helper-local actualization し、
`M1` package を close する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/463-current-l2-verifier-preview-alignment-prefloor-and-public-contract-mixed-gate-note.md`
- `specs/examples/465-current-l2-theorem-discharge-prefloor-and-public-contract-mixed-gate-note.md`
- `specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md`
- `specs/examples/470-current-l2-theorem-first-experimental-pilot-actualization.md`
- `specs/examples/474-current-l2-theorem-prover-experimental-binding-preflight.md`
- `specs/examples/479-current-l2-theorem-discharge-actual-format-probe.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/10-roadmap-overall.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/13-heavy-future-workstreams.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `crates/mir-runtime/tests/current_l2_theorem_discharge_actual_format_probe.rs`
- `crates/mir-runtime/tests/current_l2_theorem_prover_binding_preflight.rs`

## 3. Actions taken

1. theorem discharge actual-format probe と theorem-prover preflight の gap を監査し、helper-local threshold manifest が必要と判断した。
2. `CurrentL2SourceSampleTheoremDischargePublicContractThreshold` と builder を support helper に追加した。
3. `current_l2_theorem_discharge_public_contract_threshold.rs` を新設し、`e5 / p06 / p07 / p08` reached、`p05` guard-only を machine-check した。
4. `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md` を追加し、current default / retained alternatives / stop line を source-backed に固定した。
5. `specs/10`、`specs/11`、`specs/12` を更新し、`M1` close と `D-074` judgment を反映した。
6. `Documentation.md`、`progress.md`、`tasks.md`、`plan/00`、`plan/01`、`plan/10`、`plan/11`、`plan/12`、`plan/13`、`plan/17`、`plan/18`、`plan/90`、`docs/research_abstract/*` を同期し、current queue を `M2/M3` に narrowed した。
7. generated `scripts/__pycache__/` を削除した。

## 4. Files changed

- Added:
  - `crates/mir-runtime/tests/current_l2_theorem_discharge_public_contract_threshold.rs`
  - `specs/examples/481-current-l2-theorem-discharge-public-contract-threshold-default.md`
- Updated:
  - `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `specs/00-document-map.md`
  - `specs/10-open-questions.md`
  - `specs/11-roadmap-and-workstreams.md`
  - `specs/12-decision-register.md`
  - `plan/00-index.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/12-open-problems-and-risks.md`
  - `plan/13-heavy-future-workstreams.md`
  - `plan/17-research-phases-and-autonomy-gates.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/phase6-compile-ready-minimal-actualization.md`

## 5. Commands run and exact outputs

- `python3 .agents/skills/discord-report/scripts/discord_notify.py begin --cwd .`
  - `Task baseline recorded.`
- `df -h .`
  - `/dev/vda2 99G 78G 17G 83% /`
- `free -h`
  - `Mem: 960Mi total, 598Mi used, 80Mi free, 362Mi available`
- `python3 scripts/new_report.py --slug theorem-discharge-public-contract-threshold-default`
  - `docs/reports/0758-theorem-discharge-public-contract-threshold-default.md`
- `cargo test -p mir-runtime --test current_l2_theorem_discharge_public_contract_threshold`
  - `5 passed; 0 failed`
- `cargo test -p mir-runtime`
  - all suites green, `139 passed; 0 failed`
- `python3 scripts/current_l2_source_sample_regression.py regression`
  - `all regression commands passed`
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 757 numbered report(s).`
- `git diff --check`
  - no output

## 6. Evidence / findings

- new helper-local threshold manifest can carry:
  - review-unit-first principal
  - discharge-entry-adjacent reserve
  - transport preview only
  - public-contract preview only
  - notebook-consumer-first boundary
  - brand-neutral theorem request
- representative corpus is stable at this threshold:
  - reached: `e5`, `p06`, `p07`, `p08`
  - guard-only: `p05`
- this closes `M1` as a threshold package without promoting:
  - actual discharge transport
  - public theorem contract
  - theorem result public object
  - concrete theorem prover brand
  - proof object public schema

## 7. Changes in understanding

- `M1` was not waiting for a final public theorem contract decision; it was waiting for a source-backed threshold default that could stop comparison debt.
- the natural default is not a theorem-result object or concrete tool brand.
- the natural default is a review-unit-first threshold with preview-only transport / contract and a brand-neutral request boundary.
- once that threshold is explicit and executable, the immediate self-driven queue narrows to `M2` and `M3`.

## 8. Open questions

- actual discharge transport を actual adoption に送るなら、preview refs からどの payload cut を first contract に採るか。
- public theorem contract を actual adoption に送るなら、review-unit-first / discharge-entry-adjacent を consumer-shaped object にいつ移すか。
- theorem result public object と proof object public schema を separate gate にするか、public theorem contract gate に従属させるか。
- concrete theorem prover brand を theorem transport adoption より先に reopen する必要があるか。

## 9. Suggested next prompt

`M2` model-check property-language / tool-brand threshold package を進め、row-local carrier / property-tool-seam probe から first settled property language と concrete tool brand の current default / retained alternatives / stop line を source-backed に詰めたうえで、snapshot と plan/spec を再同期してください。
