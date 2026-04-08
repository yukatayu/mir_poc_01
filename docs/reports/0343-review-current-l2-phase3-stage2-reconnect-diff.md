# Report 0343 — review current L2 Phase 3 stage2 reconnect diff

- Date: 2026-04-08
- Author / agent: Codex
- Scope: `specs/examples/115` / `116`、`stage2 try/rollback reconnect first tranche actualization` の code anchor、および docs / plan / progress / report mirror の diff review
- Decision levels touched:
  - L2: current L2 parser boundary / first checker reconnect sequencing review

## 1. Objective

current L2 Phase 3 reconnect continuation のうち、stage2 try/rollback reconnect first tranche actualization とその mirror について、次の 4 観点で diff review する。

1. spec 115 / 116 の source-backed sequencing judgment が `specs/examples/30` / `45` / `51` / `68` / `73` / `113` / `114` と整合するか
2. private helper actualization が helper-local / test-only boundary を壊していないか
3. plan / progress / report が過剰主張していないか
4. test or docs drift がないか

## 2. Scope and assumptions

- review 対象は commit `c3ad2d5` 相当の stage2 reconnect tranche と、その mirror に限定する。
- 現在の worktree には後続 task の未 commit 差分があるため、本 review ではその差分を評価対象に含めない。
- current review は local diff inspection に基づく maintainer / spec editor review であり、新しい規範判断は追加しない。

## 3. Documents consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
- `specs/examples/45-current-l2-first-checker-cut-regression-baseline.md`
- `specs/examples/51-current-l2-try-rollback-structural-floor-and-restore-scope.md`
- `specs/examples/68-current-l2-try-rollback-ast-helper-first-tranche-actualization.md`
- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `specs/examples/113-current-l2-first-checker-reconnect-family-selection.md`
- `specs/examples/114-current-l2-stage1-first-checker-reconnect-first-tranche-actualization.md`
- `specs/examples/115-current-l2-stage1-widening-vs-stage2-try-rollback-reconnect.md`
- `specs/examples/116-current-l2-stage2-try-rollback-reconnect-first-tranche-actualization.md`
- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs`
- `crates/mir-ast/tests/current_l2_stage2_try_rollback_spike.rs`
- `docs/reports/0339-current-l2-stage2-try-rollback-reconnect.md`
- `docs/reports/0340-review-current-l2-stage2-try-rollback-reconnect.md`

## 4. Actions taken

1. 基礎 specs と current status mirror を再読した。
2. commit `c3ad2d5` の変更ファイル一覧と diff 本文を確認した。
3. spec 115 / 116 の sequencing judgment を `30` / `45` / `51` / `68` / `73` / `113` / `114` と照合した。
4. `current_l2_stage2_try_rollback_spike_support.rs` と `current_l2_stage2_try_rollback_spike.rs` が helper-local / test-only boundary を越えていないか確認した。
5. `Documentation.md` / `specs/00-document-map.md` / `plan/07` / `plan/09` / `plan/11` / `plan/12` / `plan/90` / `progress.md` / report chain の主張範囲を点検した。

## 5. Files changed

- Added: `docs/reports/0343-review-current-l2-phase3-stage2-reconnect-diff.md`

## 6. Evidence / outputs / test results

review 時に確認した主コマンド:

```text
$ git show --stat --name-only --format=fuller c3ad2d5
$ git show c3ad2d5 -- <review-target-files>
$ python3 scripts/validate_docs.py
Documentation scaffold looks complete.
Found 344 numbered report(s).

$ git diff --check
[no output]
```

## 7. Findings

**no findings**

### review conclusion

- spec 115 の「stage1 widening より先に stage2 try/rollback reconnect を取る」判断は、`specs/examples/30` の first checker cut cluster、`45` の regression baseline、`51` / `68` の try/rollback structural floor boundary、`73` の checker-led staged spike、`113` / `114` の reconnect family sequencingと矛盾していない。
- spec 116 の actualization cut は、`TryFallback` / `AtomicCut` の structural floorだけを helper-local / test-only に reconnect する line に留まっており、`place_anchor == current_place` gate、restore scope、runtime contrast を still later に残すという `51` / `68` の boundary を壊していない。
- `crates/mir-ast/tests/support/current_l2_stage2_try_rollback_spike_support.rs` は `mir-ast/tests/support/` 配下の private support helper であり、public API や shared helper へ昇格していない。`current_l2_stage2_try_rollback_spike.rs` も focused test だけに留まっていて、helper-local / test-only boundary を越えていない。
- `plan/` / `progress.md` / `0339` / `0340` は、public checker API 化、detached artifact handoff、runtime/proof boundary narrowing を既成事実化しておらず、current tranche を helper-local / test-only actualization として記述している。過剰主張は見当たらない。
- docs / tests の drift も見当たらない。`Documentation.md` / `specs/00-document-map.md` の導線は spec 115 / 116 の追加と整合し、report 0339 / 0340 の内容も diff 本体と一致している。

## 8. What changed in understanding

- stage2 reconnect tranche は source-backed sequencing、helper boundary、mirror 主張の 3 点で current repo の ratchet に沿っていると再確認した。
- current remaining questionは、review 対象の欠陥ではなく、次段の widening sequencing をどちらから取るかに移っている。

## 9. Open questions

- stage2 reconnect family を `E21` / `E22` contrast まで widening する threshold
- stage1 reconnect family の remaining widening (`e18` / `e19` / `e20`) に戻る threshold
- `e19-malformed-target-mismatch` を reconnect summary contract にどう乗せるか

## 10. Suggested next prompt

```text
Phase 3 の next narrow comparison として、
1. stage2 reconnect family を E21 / E22 runtime contrast まで widening する threshold
2. stage1 reconnect family の remaining widening とくに e19 reconnect summary contract
を source-backed に比較してください。
```

## 11. plan / progress

- plan/ 更新不要
- progress.md 更新不要
