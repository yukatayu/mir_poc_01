# Report 0798 — theorem actual Lean execution narrow probe after global toolchain install

- Date: 2026-04-19T03:58:00Z
- Author / agent: Codex
- Scope: Package 53。official `elan` stable を global install し、actual Lean execution を representative static sample `e5` に narrow に reopen する
- Decision levels touched: L2

## 1. Objective

toolchain unavailable reserve を実際に潰し、current theorem line を representative static sample での actual Lean execution まで進める。単なる install ではなく、generated Lean-stub source が Lean で通るかを root-cause first で確認する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/513-current-l2-theorem-actual-lean-execution-availability-probe.md`
- `specs/examples/514-current-l2-theorem-public-seam-compression-after-local-lean-unavailable-probe.md`
- `specs/examples/516-current-l2-theorem-actual-lean-execution-toolchain-probe-and-reopen-manifest.md`
- official Lean install docs / elan docs
- `scripts/current_l2_theorem_toolchain_probe.py`
- `scripts/current_l2_theorem_lean_stub_pipeline.py`
- `crates/mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs`

## 3. Actions taken

1. official Lean docs に従って `elan` stable を global install した。
2. observed toolchain version を取り直し、`stable` が `Lean 4.29.1` に解決することを確認した。
3. toolchain probe helper を再実行し、`ready` に遷移したことと sample-aware reopen manifest が維持されていることを確認した。
4. existing Lean-stub pipeline を `e5-underdeclared-lineage` で実行し、generated `.lean` source を actual Lean で叩いた。
5. failure を root-cause first で調べ、`theorem ... : Prop := by` が sort `Prop` を theorem type に置いていたことを特定した。
6. failing actual-probe test を追加してから、stub source placeholder を `True` に修正した。
7. actual Lean execution probe を再実行し、representative static sample `e5` が Lean で通ることを確認した。

## 4. Files changed

- `crates/mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs`
- `crates/mir-semantics/tests/current_l2_lean_theorem_stub_actual_probe.rs`
- `scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`
- `specs/examples/518-current-l2-theorem-actual-lean-execution-narrow-probe-after-global-toolchain-install.md`

## 5. Commands run and exact outputs

- `curl https://elan.lean-lang.org/elan-init.sh -sSf | sh -s -- -y --default-toolchain stable`
  - default toolchain resolved to `leanprover/lean4:v4.29.1`
- `source "$HOME/.elan/env" && lean --version && lake --version && elan --version`
  - `Lean 4.29.1`
  - `Lake 5.0.0-src+f72c35b`
  - `elan 4.2.1`
- `source "$HOME/.elan/env" && python3 scripts/current_l2_theorem_toolchain_probe.py e5-underdeclared-lineage`
  - `toolchain_status = "ready"`
  - `reopen_condition_met = true`
- `source "$HOME/.elan/env" && python3 scripts/current_l2_theorem_lean_stub_pipeline.py e5-underdeclared-lineage --artifact-root target/current-l2-theorem-lean-exec --run-label actual-lean-probe`
  - `review_unit_count = 2`
  - `lean_stub_count = 2`
  - `matched_pairs = 2`
- pre-fix manual `lean` probe
  - error: `type of theorem ... is not a proposition`
- `source "$HOME/.elan/env" && cargo test -p mir-semantics --test current_l2_lean_theorem_stub_actual_probe`
  - `test result: ok. 1 passed; 0 failed`
- `cargo test -p mir-semantics --test current_l2_lean_theorem_stub_support`
  - `test result: ok. 4 passed; 0 failed`
- `python3 -m unittest scripts/tests/test_current_l2_theorem_lean_stub_pipeline.py`
  - `Ran 5 tests`
  - `OK`

## 6. Evidence / findings

- toolchain unavailable reserve は、global install 後に actual execution reserve へ移行できた。
- first actual Lean failure は environment ではなく generated stub source shape にあり、root cause は theorem type に `Prop` sort を直接置いていたことだった。
- `True` placeholder へ修正後、representative static sample `e5` generated stubs は actual Lean execution を通った。

## 7. Changes in understanding

- theorem line の first actual Lean execution package は、public theorem contract より前に generated source validity を machine-check する narrow package として切るのが自然である。
- actual Lean execution は now/never の binary gate ではなく、representative sample narrow probe → corpus widening の順で ratchet できる。

## 8. Open questions

- `p06 / p07 / p08` まで actual Lean execution を widening するか。
- repo-local Lean project / `lean-toolchain` pin を今の段階で追加するか。
- proof object public schema / theorem public contract reopen を actual Lean widening の前後どちらに置くか。

## 9. Suggested next prompt

`specs/examples/518` を anchor に `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、`specs/10`、`specs/11`、`specs/12`、`plan/90` を actual Lean execution reached reading へ更新し、その後 `p06 / p07 / p08` への widening 可否を narrow package として検討してください。
