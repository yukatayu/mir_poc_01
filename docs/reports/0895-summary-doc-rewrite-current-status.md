# Report 0895 — summary doc rewrite current status

- Date: 2026-04-20T15:48:58.645912Z
- Author / agent: Codex
- Scope: current status の再確認と、summary docs の全面書き直し
- Decision levels touched: なし。規範判断の変更なし

## 1. Objective

2026-04-21 時点の実実行 evidence に基づいて、

- Problem 1
  - typed / IFC
  - theorem-first emitted artifact loop
  - model-check second-line reserve
  - Lean foundation
- Problem 2
  - order / handoff / authoritative-room representative pair
  - reserve strengthening lane
  - negative static-stop pair

の current floor と stop line を整理し、
top-level の summary docs を冗長さと stale wording を落とした日本語へ全面更新する。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/18-type-proof-modelcheck-and-ordering-research-program.md`
- `samples/problem-bundles/problem1-typed-theorem-model-check.md`
- `samples/problem-bundles/problem2-order-handoff-shared-space.md`
- `samples/lean/README.md`
- `samples/lean/foundations/CurrentL2IfcSecretExamples.lean`
- `samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean`

## 3. Actions taken

1. AGENTS 指示に従い、README / Documentation / progress / specs / plan を読み、summary docs の冗長箇所と current drift を確認した。
2. Problem 1 の representative commands を実行し、
   - positive
   - rejection pair
   - theorem-first emitted artifact loop
   - model-check reserve summary
   を再確認した。
3. Problem 2 の representative commands を実行し、
   - representative success pair
   - reserve route
   - negative static-stop pair
   - reserve package summaries
   を再確認した。
4. Lean toolchain availability を確認し、foundation files と generated current-L2 corpus を再検証した。
5. Lean stdin による self-contained negative snippet を流し、
   authority なし declassification が型検査で失敗することを確認した。
6. `README.md`、`Documentation.md`、`progress.md`、`tasks.md`、`plan/00-index.md`、`plan/01-status-at-a-glance.md` を全面書き直しし、
   repo-local near-end と final public stop line を明示的に分離した。

## 4. Files changed

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `docs/reports/0895-summary-doc-rewrite-current-status.md`

## 5. Commands run and exact outputs

主要 command のみを抜粋する。

1. representative bundle smoke

```bash
python3 scripts/current_l2_guided_samples.py smoke-all --format json
```

抜粋:

```json
[
  {
    "problem_id": "problem1",
    "status": "passed",
    "successful_steps": 5
  },
  {
    "problem_id": "problem2",
    "status": "passed",
    "successful_steps": 7
  }
]
```

2. typed positive / negative

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p10-typed-authorized-fingerprint-declassification.txt --format pretty
```

抜粋:

```text
terminal_outcome: success
typed_checker_hint_status: reached
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p11-typed-unauthorized-fingerprint-release.txt --format pretty
```

抜粋:

```text
terminal_outcome: Reject
case_label: authority_miss_negative
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p12-typed-classified-fingerprint-publication-block.txt --format pretty
```

抜粋:

```text
terminal_outcome: Reject
case_label: classified_to_public_negative
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p15-typed-capture-escape-rejected.txt --format pretty
```

抜粋:

```text
terminal_outcome: Reject
case_label: capture_escape_negative
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- check-source-sample samples/prototype/current-l2-typed-proof-model-check/p16-typed-remote-call-budget-exceeded.txt --format pretty
```

抜粋:

```text
terminal_outcome: Reject
case_label: remote_call_budget_negative
```

3. theorem-first emitted artifact loop

```bash
python3 scripts/current_l2_guided_samples.py emit-theorem problem1
```

抜粋:

```text
output dir: target/current-l2-guided/problem1-theorem-pilot
p06-typed-proof-owner-handoff ... pilot_status: reached
p07-dice-late-join-visible-history ... pilot_status: reached
p08-dice-stale-reconnect-refresh ... pilot_status: reached
stop line:
- final public theorem contract
- concrete theorem prover brand
- final public verifier contract
```

4. model-check second-line reserve

```bash
python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line
```

抜粋:

```text
p10-typed-authorized-fingerprint-declassification ... terminal_outcome: success
p11-typed-unauthorized-fingerprint-release ... terminal_outcome: reject
p12-typed-classified-fingerprint-publication-block ... terminal_outcome: reject
p15-typed-capture-escape-rejected ... terminal_outcome: reject
p16-typed-remote-call-budget-exceeded ... terminal_outcome: reject
stop line:
- first settled property language
- concrete model-check tool brand
- final public checker artifact
- actual public checker migration
- production checker/runtime-policy contract
- final public verifier contract
```

5. order / handoff representative success

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt --format pretty
```

抜粋:

```text
terminal_outcome: success
surface_preview:
  minimal_companion:
    status: reached
  serial_scope_reserve:
    status: reached
authoritative_room_first_scenario_actual_adoption:
  status: reached
```

6. order / handoff negative static-stop

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p13-dice-late-join-missing-publication-witness.txt --format pretty
```

抜粋:

```text
static_gate: underdeclared
static_reasons:
- missing publication witness before handoff for late-join visibility at root / room / dice_authority
entered_evaluation: false
```

```bash
cargo run -q -p mir-runtime --example mir_current_l2 -- run-source-sample samples/prototype/current-l2-order-handoff/p14-dice-late-join-handoff-before-publication.txt --format pretty
```

抜粋:

```text
static_gate: malformed
static_reasons:
- handoff appears before publish for late-join visibility at root / room / dice_authority
entered_evaluation: false
```

7. authoritative-room scenario / reserve packages

```bash
python3 scripts/current_l2_guided_samples.py emit-scenario problem2
python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness
python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service
```

抜粋:

```text
p07 ... first-line representative ... terminal_outcome: success
p08 ... first-line representative ... terminal_outcome: success
p09 ... reserve practical route ... terminal_outcome: success
p13 ... negative static-stop ... static_gate: underdeclared
p14 ... negative static-stop ... static_gate: malformed
```

8. Lean toolchain and foundations

```bash
source "$HOME/.elan/env" && lean --version && lake --version && elan --version
```

出力:

```text
Lean (version 4.29.1, x86_64-unknown-linux-gnu, commit f72c35b3f637c8c6571d353742168ab66cc22c00, Release)
Lake version 5.0.0-src+f72c35b (Lean version 4.29.1)
elan 4.2.1 (3d5138e15 2026-03-18)
```

```bash
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2IfcSecretExamples.lean
source "$HOME/.elan/env" && lean samples/lean/foundations/CurrentL2FiniteIndexFirstLayer.lean
```

両方とも exit code 0、stdout / stderr なし。

```bash
source "$HOME/.elan/env" && python3 scripts/current_l2_lean_sample_sync.py
```

抜粋:

```text
"verification": { "success": true, "returncode": 0 }
```

generated current-L2 側は `warning: declaration uses 'sorry'` を含むが、return code は 0。

9. Lean self-contained negative

```bash
source "$HOME/.elan/env" && printf '%s\n' \
  'inductive SecurityLabel where' \
  '  | low' \
  '  | high' \
  '' \
  'open SecurityLabel' \
  '' \
  'def flowsTo : SecurityLabel -> SecurityLabel -> Prop' \
  '  | low, _ => True' \
  '  | high, high => True' \
  '  | high, low => False' \
  '' \
  'def CanDeclassify (hasAuthority : Bool) (fromLabel toLabel : SecurityLabel) : Prop :=' \
  '  hasAuthority = true ∨ flowsTo fromLabel toLabel' \
  '' \
  'example : CanDeclassify false high low := by' \
  '  simp [CanDeclassify, flowsTo]' | lean --stdin
```

出力:

```text
<stdin>:15:42: error: unsolved goals
⊢ False
```

10. resources

```bash
df -h .
free -h
```

抜粋:

```text
/dev/vda2 99G 83G 12G 88% /
Mem: 960Mi total, 65Mi free, 19Gi swap
```

## 6. Evidence / findings

1. Problem 1 は **typed / theorem / model-check representative bundle としては実際に通る**。
   ただし theorem / model-check は final public contract ではなく、
   helper-local bridge / reserve summary / generated stub acceptance の段階である。

2. typed rejection pair は current repo で明確に確認できる。
   `p11`、`p12`、`p15`、`p16` はすべて `terminal_outcome: Reject` を返した。

3. Problem 2 は **order / handoff / authoritative-room representative bundle としては実際に通る**。
   `p07` / `p08` は success、`p13` / `p14` は static gate で止まる。

4. low-level `memory_order` exact surface は current practical line ではない。
   repo が current public line として実際に持っているのは、
   order / handoff / authoritative-room の high-level relation family である。

5. Lean は 2 層で読む必要がある。
   `foundations/` は actual small proof fragment、
   `current-l2/` は `sorry` を含む accepted stub corpus であり、保証の強さは同じではない。

6. summary docs の旧版は、package 名や helper-local line を大量に列挙しすぎており、
   現状把握の入口としては過密だった。
   そこで、verified floor / stop line / entry command の 3 点に圧縮した。

## 7. Changes in understanding

- 「二大問題は一通り終わっている」という読みは、**repo-local near-end** に限れば概ね妥当。
- ただし、その意味は
  - representative sample bundle が runnable
  - reject / static-stop pair が確認できる
  - Lean foundation と generated stub acceptance がある
  までである。
- 一方で、次を「もう done」と読むのは不正確だと再確認した。
  - final public theorem/model-check contract
  - concrete prover / checker tool binding
  - low-level `memory_order` exact reinterpretation
  - final verifier contract
  - final public witness/provider contract

## 8. Open questions

- top-level summary docs に、actual command output の抜粋まで常設するか
- low-level `memory_order` line を将来 separate summary doc として切るか
- Problem 1 / Problem 2 bundle docs も今回の top-level rewrite と同じ粒度までさらに短くするか

## 9. Suggested next prompt

`samples/problem-bundles/problem1-typed-theorem-model-check.md` と `samples/problem-bundles/problem2-order-handoff-shared-space.md` に、今回実行した representative output の抜粋を 1 つずつ追記し、README からそこへ最短導線を張ってください。final public stop line を崩さずに、人間が最初に読む quickstart として磨きたいです。
