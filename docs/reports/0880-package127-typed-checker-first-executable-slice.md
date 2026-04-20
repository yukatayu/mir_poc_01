# Report 0880 — package127 typed checker first executable slice

- Date: 2026-04-20T09:47:16.003575Z
- Author / agent: Codex
- Scope: Package 127 として `check-source-sample` focused checker command を追加し、Problem 1 bundle / snapshot docs を同期する
- Decision levels touched: L2

## 1. Objective

- first strong typing sample set `p10 / p11 / p12 / p15 / p16` に対する focused checker slice を repo-local command として actualize する。
- `run-source-sample` の巨大 summary と final public verifier contract の間に、checker-adjacent executable slice を置く。
- Problem 1 sample bundle と snapshot docs を stale wording なしで同期する。

## 2. Inputs consulted

- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/557-current-l2-first-strong-typing-layer-finite-index-spine-default.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `scripts/tests/test_problem_sample_bundles.py`

## 3. Actions taken

- `crates/mir-runtime/tests/current_l2_operational_cli.rs` に
  - `check-source-sample` pretty output
  - `check-source-sample` json output
  - non-typed sample reject
  の 3 本の RED を追加した。
- `crates/mir-runtime/src/current_l2_cli.rs` に
  `check-source-sample`
  command を追加し、typed sample manifest / static gate verdict / terminal outcome /
  typed checker hint / actual checker payload family / row detail / row body threshold を
  focused summary として返すようにした。
- `samples/problem-bundles/problem1-typed-theorem-model-check.md` に
  `check-source-sample`
  の quickstart 導線を追加した。
- `specs/examples/600-current-l2-typed-checker-first-executable-slice-actualization.md`
  を追加し、repo-local focused checker slice の current cut と stop line を文書化した。
- `Documentation.md`、`plan/01`、`plan/11`、`plan/17`、`plan/18`、`plan/90`、`progress.md`、`tasks.md`、`specs/00`、`specs/11`、`specs/12`
  を Package 127 closeout 読みに同期した。

## 4. Files changed

- `docs/reports/0880-package127-typed-checker-first-executable-slice.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `plan/17-research-phases-and-autonomy-gates.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `tasks.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `crates/mir-runtime/src/current_l2_cli.rs`
- `crates/mir-runtime/tests/current_l2_operational_cli.rs`
- `scripts/tests/test_problem_sample_bundles.py`
- `specs/00-document-map.md`
- `specs/11-roadmap-and-workstreams.md`
- `specs/12-decision-register.md`
- `specs/examples/600-current-l2-typed-checker-first-executable-slice-actualization.md`

## 5. Commands run and exact outputs

- `cargo test -p mir-runtime --test current_l2_operational_cli`
  - RED:
    - `unknown command \`check-source-sample\``
    - non-typed sample reject が usage error で止まる
  - GREEN:
    - `test result: ok. 33 passed; 0 failed`
- `python3 -m unittest scripts.tests.test_problem_sample_bundles`
  - RED:
    - `check-source-sample` not found in Problem 1 bundle doc
  - GREEN:
    - `Ran 3 tests`
    - `OK`
- `cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format pretty`
  - `cluster_kind: ifc_authority_release_cluster`
  - `typed_checker_hint_status: reached`
  - `actual_checker_payload_family_threshold_status: reached`

## 6. Evidence / findings

- typed sample set には既に checker-adjacent manifest / threshold family が揃っていたので、
  final public checker contract を作る前に focused executable slice を切る方が自然だった。
- `check-source-sample` により、
  `p10 / p11 / p12 / p15 / p16`
  の cluster / case / row-body を 1 画面で読み返せるようになり、
  helper preview と actual executable command の間の空白が小さくなった。
- non-typed sample を reject する設計にしたことで、
  current cut が first strong typing sample set 限定であることを command surface 自体に反映できた。

## 7. Changes in understanding

- Package 127 は new type calculus 比較を増やすより、existing typed preview family を focused executable slice に ratchet する方が効果的だった。
- current default の「checker-adjacent principal」は、docs-only wording ではなく repo-local command まで下ろせる段階にある。

## 8. Open questions

- `check-source-sample` の次段を actual checker payload family beyond first sample set へ広げるか。
- theorem-first emitted artifact line をどの command surface で tighten するか。
- authoritative-room first scenario bundle の smoke / artifact hardening をどの command surface で tighten するか。

## 9. Suggested next prompt

- `Package 127 は閉じたので、次は theorem-first emitted-artifact hardening か authoritative-room runnable scenario hardening を進めてください。`
