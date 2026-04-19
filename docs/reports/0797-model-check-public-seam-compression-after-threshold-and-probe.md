# Report 0797 — model-check public seam compression after threshold and probe

- Date: 2026-04-19T03:37:30Z
- Author / agent: Codex
- Scope: Package 52。model-check property/tool seam probe と final public-contract reopen threshold を束ね、remaining model-check public seams を helper-local residual matrix に圧縮する
- Decision levels touched: L2

## 1. Objective

`specs/examples/480`、`501`、`507` を前提に、model-check mixed gate を comparison list のまま残さず、current representative corpus 上で machine-check 可能な public-seam compression manifest に actualize する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/480-current-l2-model-check-property-language-and-tool-seam-probe.md`
- `specs/examples/501-current-l2-model-check-checker-artifact-route-actual-adoption.md`
- `specs/examples/507-current-l2-model-check-final-public-contract-reopen-threshold.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- existing model-check runtime tests around `480` / `501` / `507`

## 3. Actions taken

1. model-check residual が property/tool side と checker-artifact/public-contract side にどう散っているかを再読した。
2. `crates/mir-runtime/tests/current_l2_model_check_public_seam_compression.rs` を先に追加し、`e5 / p06 / p07 / p08 / p09` reached、`p05` guard-only の expected residual matrix を赤で固定した。
3. support helper に `CurrentL2SourceSampleModelCheckPublicSeamCompression` と builder を追加し、threshold carry-over と property/tool probe carry-over を 1 本に束ねた。
4. focused runtime test を緑にし、reached representative corpus と guard-only contrast が current reading どおりに揃うことを確認した。
5. `specs/examples/517` を追加し、current recommendation / retained alternatives / stop line を source-backed に整理した。

## 4. Files changed

- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`
- `crates/mir-runtime/tests/current_l2_model_check_public_seam_compression.rs`
- `specs/examples/517-current-l2-model-check-public-seam-compression-after-threshold-and-probe.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_model_check_public_seam_compression`
  - `test result: ok. 6 passed; 0 failed`

## 6. Evidence / findings

- reached representative corpus は `e5 / p06 / p07 / p08 / p09`、guard-only contrast は `p05` に保てた。
- helper-local compression manifest では、
  - property language and tool brand first
  - public checker artifact and migration second
  - verifier handoff and runtime-policy contract third
  - final public verifier contract fourth
  を subject-local residual ref として固定できた。
- model-check line は theorem / order-handoff / witness-provider と同様に、actual adoption floor の次を residual matrix で読む段階に入っている。

## 7. Changes in understanding

- model-check mixed gate は compare-floor を増やすより、threshold + probe を圧縮して stop line を明確化する方が current repo の進め方に合う。
- `p09` は provider-placement practical line に留まらず、model-check public seam compression の reached representative corpus にも自然に入る。

## 8. Open questions

- first settled property language と concrete tool brand を truly paired reopen にするか。
- final public checker artifact と actual public checker migration を同一 reopen package にするか。
- verifier handoff / runtime-policy contract を final public verifier contract 直前の paired gate に固定するか。

## 9. Suggested next prompt

`specs/examples/516` と `517` を anchor に、`Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`specs/10`、`specs/11`、`specs/12`、`plan/90` を同期し、current self-driven queue を environment-aware theorem reserve と residual mixed gate maintenance に再圧縮してください。
