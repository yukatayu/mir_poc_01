# Report 0813 — Package 60 theorem/model-check residual mixed-gate compression

## Objective

Package 60 として、theorem/model-check residual mixed gate を `run-source-sample` helper summary に actualize し、docs 側で source-backed だった reopen-threshold reading を operational evidence と同期する。

## Scope and assumptions

- `specs/` を規範正本とし、今回の作業は final public theorem/model-check contract adoption ではなく helper-local operational mirror に留める。
- theorem side は `e5 / p06 / p07 / p08` reached、`p09` guard の current cut を維持する。
- model-check side は `e5 / p06 / p07 / p09` reached、`p08` guard の current cut を維持する。

## Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/506-current-l2-theorem-final-public-contract-reopen-threshold.md`
- `specs/examples/507-current-l2-model-check-final-public-contract-reopen-threshold.md`
- `specs/examples/530-current-l2-theorem-and-model-check-helper-preview-widening.md`
- `specs/examples/531-current-l2-near-end-closeout-sync-after-package58.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `crates/mir-runtime/tests/support/current_l2_source_sample_emitted_artifact_support.rs`

## Actions taken

1. `current_l2_operational_cli` に theorem/model-check reopen-threshold helper mirror summary を追加した。
2. `p06 / p07 / p08 / p09` を使う CLI regression を追加し、reached/guard 非対称を test-first で固定した。
3. `specs/examples/532` を追加し、Package 60 の actual package reading を document 化した。
4. snapshot / roadmap / traceability を Package 60 close reading に同期した。

## Evidence / outputs / test results

- red:
  - `cargo test -p mir-runtime --test current_l2_operational_cli operational_cli_json_pins_typed_bridge_prototype_preview -- --exact`
  - 追加した reopen-threshold field が `Null` で失敗することを確認した。
- green:
  - `cargo test -p mir-runtime --test current_l2_operational_cli`
  - 16 test passed
- operational evidence:
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p08-dice-stale-reconnect-refresh.txt --format json`
  - `cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p09-dice-delegated-rng-provider-placement.txt --format json`
- docs validation:
  - `python3 scripts/validate_docs.py`
- diff hygiene:
  - `git diff --check`

## What changed in understanding

- theorem/model-check residual mixed gate は、docs-only reopen-threshold floor ではなく `run-source-sample` helper summary に mirror できる段階まで来ている。
- `p08` theorem reached / model-check guarded と `p09` theorem guarded / model-check reached の非対称を CLI 上で固定したことで、residual mixed gate を final public contract adoption と混同しにくくなった。
- current live queue は Package 60 close 後、Problem 2 側の residual compression にさらに narrow に寄せられる。

## Open questions

- Problem 2 residual mixed gate:
  final source wording / emitted-artifact schema / witness-provider public-contract residual をどの reopen order で narrow に残すか。
- strong typing / IFC beyond first checker fragment:
  sample-local helper preview を actual checker payload family にどこまで ratchet するか。
- final public theorem/model-check contract:
  current reopen-threshold helper mirror の先で、どの reopen order を user-spec / mixed gate に残すか。

## Suggested next prompt

Package 61 として、order-handoff/shared-space residual mixed gate を `run-source-sample` helper summary と docs に actualize し、final source wording / emitted-artifact schema / witness-provider public contract residual の reopen order を sharpen してください。
