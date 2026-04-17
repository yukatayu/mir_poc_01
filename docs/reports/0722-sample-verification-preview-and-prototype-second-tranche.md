# Report 0722 — sample verification preview and prototype second tranche

- Date: 2026-04-17T03:20:00Z
- Author / agent: Codex (GPT-5)
- Scope: helper-local verification preview、prototype second tranche、snapshot / FAQ / plan sync
- Decision levels touched: L2 / L3

## 1. Objective

- sample / prototype 実行時に theorem / model-check bridge の current floor を helper-local preview として見える化する。
- final public verifier contract や concrete tool binding を決めずに、runtime reached / static reached / guarded not reached の 3 状態を sample-driven に比較できるようにする。
- order-handoff domain の corrected runnable prototype を second tranche まで増やす。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/10-open-questions.md`
- `specs/11-roadmap-and-workstreams.md`
- `plan/01-status-at-a-glance.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `samples/prototype/README.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`

## 3. Actions taken

1. operational CLI summary に helper-local `verification_preview` を追加した。
2. preview は final public artifact schema ではなく、current source report から読める次だけに留めた。
   - `formal_hook_status`
   - `subject_kind`
   - `subject_ref`
   - proof notebook review unit obligation list
   - model-check concrete carrier obligation list
   - `guard_reason`
3. preview rule は次の 3 状態へ縮約した。
   - runtime reached (`runtime_try_cut_cluster`)
   - static reached (`fixture_static_cluster`)
   - guarded not reached
4. order-handoff prototype second tranche として `p04` / `p05` を追加した。
   - `p04-dice-owner-duplicate-declaration`
   - `p05-dice-owner-guarded-chain`
5. CLI test を拡張し、runtime / static / guarded の 3 状態を ratchet した。
6. `specs/examples/453` を追加し、helper-local sample verification preview cut を docs-first に固定した。
7. `Documentation.md`、`progress.md`、`tasks.md`、`faq_005.md`、relevant `plan/`、`samples/prototype/README.md`、`specs/00-document-map.md` を current snapshot に同期した。
8. reviewer で helper-local preview と docs drift を確認し、no actionable findings を得た。

## 4. Files changed

- code
  - `crates/mir-runtime/src/current_l2_cli.rs`
  - `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- samples
  - `samples/prototype/README.md`
  - `samples/prototype/current-l2-order-handoff/p04-dice-owner-duplicate-declaration.txt`
  - `samples/prototype/current-l2-order-handoff/p04-dice-owner-duplicate-declaration.host-plan.json`
  - `samples/prototype/current-l2-order-handoff/p05-dice-owner-guarded-chain.txt`
  - `samples/prototype/current-l2-order-handoff/p05-dice-owner-guarded-chain.host-plan.json`
- docs / snapshot / memory
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `faq_005.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/08-representative-programs-and-fixtures.md`
  - `plan/09-helper-stack-and-responsibility-map.md`
  - `plan/10-roadmap-overall.md`
  - `plan/11-roadmap-near-term.md`
  - `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
  - `plan/90-source-traceability.md`
  - `specs/00-document-map.md`
  - `specs/examples/453-current-l2-sample-verification-preview-and-prototype-second-tranche.md`

## 5. Commands run and exact outputs

```bash
$ cargo test -p mir-runtime --test current_l2_operational_cli
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p04-dice-owner-duplicate-declaration.txt --format pretty
sample: p04-dice-owner-duplicate-declaration
...
verification_preview:
  formal_hook_status: reached
  subject_kind: fixture_static_cluster
  subject_ref: p04-dice-owner-duplicate-declaration
  proof_notebook_review_unit_obligations:
  - canonical_normalization_law
  - no_re_promotion

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p05-dice-owner-guarded-chain.txt --format pretty
sample: p05-dice-owner-guarded-chain
...
verification_preview:
  formal_hook_status: guarded_not_reached
  subject_kind: none
  subject_ref: none
  guard_reason: current helper preview only reaches runtime_try_cut_cluster when rollback or atomic-cut evidence exists for `p05-dice-owner-guarded-chain`
```

最終検証は §6 に追記する。

## 6. Evidence / findings

- prototype bucket で runtime reached / static reached / guarded not reached の 3 状態を sample-driven に比較できるようになった。
- `verification_preview` は obligation kind までを helper-local に見せる cut であり、public theorem / model-check contract には昇格していない。
- order-handoff domain で、user-origin rough sample の corrected line が実行と verification stop line の両方を見せられるようになった。
- reviewer 最終確認は `No actionable findings remain` だった。

最終確認で実施した command:

```bash
$ cargo test -p mir-runtime --test current_l2_source_lowering
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p mir-runtime --test current_l2_source_sample_runner
test result: ok. 18 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p mir-runtime --test current_l2_operational_cli
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p01-dice-publication-handoff.txt --format pretty
sample: p01-dice-publication-handoff
...
verification_preview:
  formal_hook_status: reached
  subject_kind: runtime_try_cut_cluster

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p04-dice-owner-duplicate-declaration.txt --format pretty
sample: p04-dice-owner-duplicate-declaration
...
verification_preview:
  formal_hook_status: reached
  subject_kind: fixture_static_cluster

$ cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p05-dice-owner-guarded-chain.txt --format pretty
sample: p05-dice-owner-guarded-chain
...
verification_preview:
  formal_hook_status: guarded_not_reached
  subject_kind: none

$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 721 numbered report(s).

$ git diff --check
# no output
```

## 7. Changes in understanding

- theory-lab の重い topic でも、final adoption を決めずに sample-visible preview として切ることで自走可能な比較 package を作れる。
- prototype second tranche は current authored inventory を壊さずに、mixed-gate topic の usability / semantic honesty comparison を前に進めるのに有効である。

## 8. Open questions

- `verification_preview` を helper-local preview 以上に強くする必要があるか。
- typed-core を sample-visible にする second tranche をどこまで prototype bucket へ切るか。
- shared-space fairness / replay line を sample bucket へ出すとき、first domain scenario を何にするか。
- `plan/12` と `specs/10` は今回は更新不要と判断した。

## 9. Suggested next prompt

`samples/prototype/` の third tranche として、typed-core / theorem discharge / model-check property-language の mixed-gate topicを sample-visible comparison にする corrected prototype 群を追加してください。
