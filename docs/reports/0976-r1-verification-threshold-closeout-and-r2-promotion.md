# Report 0976 — `R1` verification threshold closeout and `R2` promotion

- Date: 2026-04-28
- Author / agent: Codex
- Scope: `R1` `VerificationLayer` widening threshold inventory の closeout trail を completion まで閉じ、snapshot / queue / reader-facing current-line wording を `R2` `AttachPoint` compatibility / detach minimal contract へ同期する
- Decision levels touched: L2 / L3 documentation boundary only。final public verifier contract、runtime crate hot-plug engine、rollback protocol、durable migration engine、final public hot-plug ABI は固定しない

## 1. Objective

`R1` を close 済み package として正しく記録し、
`R2` を stale drift なしで promoted next line に送る。

この report は次を担う。

- `0975` first-cut report の後段として closeout trail を閉じる
- `progress.md` recent log requirement を満たす
- `samples_progress.md` `PH0` row と recent validation を同期する
- research / hands-on landing current-line drift を解消する

## 2. Inputs consulted

- `docs/reports/0975-r1-verification-layer-widening-threshold-inventory-first-cut.md`
- `progress.md`
- `tasks.md`
- `samples_progress.md`
- `README.md`
- `Documentation.md`
- `plan/01-status-at-a-glance.md`
- `plan/11-roadmap-near-term.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/mirrorea_future_axis_01.md`
- `docs/hands_on/current_phase_closeout_01.md`
- reviewer output:
  `Confucius`

## 3. Actions taken

1. `R1` closeout memory と `R2` promoted-next wordingを
   `progress.md`、`tasks.md`、`samples_progress.md`、`plan/01`、`plan/11`、
   `README.md`、`Documentation.md`、reader-facing summary / landing page に同期した。
2. `docs/research_abstract/README.md` の stale active reference を修正し、
   `R1` current promoted next line と読める drift を解消した。
3. `progress.md` recent log と `samples_progress.md` `PH0` row を
   `R1` closeout package に合わせて更新した。
4. `0975` の note を update し、
   closeout trail が `0976` に続くことを明示した。
5. reviewer finding を受けて、
   report trail / recent log / dashboard row / research landing の不足を埋めた。

## 4. Files changed

- updated:
  - `README.md`
  - `Documentation.md`
  - `progress.md`
  - `tasks.md`
  - `samples_progress.md`
  - `plan/01-status-at-a-glance.md`
  - `plan/11-roadmap-near-term.md`
  - `docs/research_abstract/README.md`
  - `docs/research_abstract/mirrorea_future_axis_01.md`
  - `docs/hands_on/current_phase_closeout_01.md`
  - `docs/reports/0975-r1-verification-layer-widening-threshold-inventory-first-cut.md`
  - `docs/reports/0976-r1-verification-threshold-closeout-and-r2-promotion.md`

## 5. Commands run and exact outputs

Resource floor before rerun:

```bash
df -h .
```

Relevant exact output:

```text
/dev/vda2        99G   63G   32G  67% /
```

```bash
free -h
```

Relevant exact output:

```text
Mem: 960Mi total, 766Mi used, 88Mi free, 193Mi available
Swap: 19Gi total, 2.0Gi used, 17Gi free
```

`R1` evidence anchors:

```bash
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
```

Relevant exact output:

```text
"name": "verification_handoff_witness"
"layer_signature_scope": "representative_slice"
"name": "verification_model_check"
"layer_signature_scope": "clean_near_end_canonical_inventory"
```

Final validation rerun:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
python3 scripts/sugoroku_world_samples.py run 03_roll_publish_handoff --debug layers --format json
python3 scripts/sugoroku_world_samples.py closeout --format json
cargo run -q -p mir-runtime --bin mir-clean-near-end -- closeout --format json
git diff --check
```

Relevant exact output:

```text
all required paths present
Found 974 numbered report(s).
"name": "verification_handoff_witness"
"layer_signature_scope": "representative_slice"
"name": "verification_model_check"
"layer_signature_scope": "clean_near_end_canonical_inventory"
git diff --check: no output
```

Post-fix doc-level rerun:

```bash
python3 scripts/check_source_hierarchy.py
python3 scripts/validate_docs.py
git diff --check
```

Relevant exact output:

```text
all required paths present
Found 974 numbered report(s).
git diff --check: no output
```

## 6. Evidence / findings

- `R1` closeout の substantive content は `0975` first-cut で十分揃っていたが、
  report trail / dashboard / recent-log closure が不足していた。
- `R2` promotion で新たに固定したのは next line の読みだけであり、
  runtime crate hot-plug engine や final public hot-plug ABI ではない。
- `R1` closeout 後も helper representative / runtime canonical の
  verification scope split を保ち、viewer / telemetry / theorem / runtime-policy lane を
  active emitted row に上げていない。
- reviewer initial findings は 4 件とも解消済みであり、
  narrow re-review は no findings で完了した。

## 7. Changes in understanding

- この repo では package close は docs 内容だけでなく、
  report trail、recent log、dashboard row まで揃って初めて close と言える。
- `R2` は `P14` closeout の再実装ではなく、
  helper-local lifecycle canary と final public hot-plug ABI のあいだにある
  minimal contract row を narrow に切り出す package と読むのが最も整合的である。

## 8. Open questions

- `R2` で minimal contract row をどこまで
  `hotplug_activation_boundary` / `hotplug_detach_boundary` から抽出するか
- `R2` current scope で `migration_contract` を row detail に留めるか、
  contract matrix へ上げるか
- `R2` close 後の next repo-side narrowing を theorem-side widening と hot-plug residual の
 どちらに寄せるか

## 9. Suggested next prompt

`R2` `AttachPoint` compatibility / detach minimal contract を進めてください。
`plan/21-hotplug-attachpoint-roadmap.md`、`docs/research_abstract/hotplug_attachpoint_plan_01.md`、
`python3 scripts/sugoroku_world_samples.py run 01_runtime_attach_game --debug hotplug --format json`、
`python3 scripts/sugoroku_world_samples.py run 09_detach_todo --debug hotplug --format json`
を anchor にして、minimal contract matrix、storage detach との non-equivalence、
kept-later migration / rollback gate、snapshot / report sync まで同じ package で閉じてください。
