# Report 0142 — review current L2 static gate reasons machine-check

- Date: 2026-04-05T09:42:39.229880Z
- Author / agent: Codex
- Scope: report 0141 とその mirror diff に対する reviewer completion の記録
- Decision levels touched: L2

## 1. Objective

`expected_static.reasons` machine-check 化の試行が current fixture corpus と衝突する、という新判断について、

- docs / plan / progress が code と整合しているか
- machine-check behavior を過剰に主張していないか
- traceability が不足していないか

を確認する。

## 2. Inputs consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/07-current-l2-host-stub-harness.md`
9. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
10. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
11. `plan/07-parser-free-poc-stack.md`
12. `plan/11-roadmap-near-term.md`
13. `plan/12-open-problems-and-risks.md`
14. `plan/15-current-l2-fixture-authoring-template.md`
15. `plan/90-source-traceability.md`
16. `progress.md`
17. `docs/reports/0141-current-l2-static-gate-reasons-machine-check.md`
18. `crates/mir-semantics/src/harness.rs`
19. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. reviewer subagent を 1 回だけ起動し、completion まで待った。
2. reviewer が指摘した stale roadmap と traceability omission を読み、該当箇所を確認した。
3. `plan/11-roadmap-near-term.md` がまだ「`expected_static.reasons` を actual machine-check に上げるか比較する」と書いているのを、dedicated carrier 比較へ更新した。
4. `plan/90-source-traceability.md` が report 0142 を落としていたため、0141 / 0142 の両方を traceability に追加した。
5. reviewer が semantic mismatch なしと判断したことを確認し、その finding を report へ転記した。

## 4. Files changed

- `docs/reports/0142-review-current-l2-static-gate-reasons-machine-check.md`
- `plan/11-roadmap-near-term.md`
- `plan/90-source-traceability.md`

## 5. Commands run and exact outputs

```text
wait_agent reviewer
completed
```

## 6. Evidence / findings

- reviewer finding 1 は `plan/11-roadmap-near-term.md` の stale roadmap であり、現在の next step は direct promotion 比較ではなく dedicated carrier comparison だと判断した。
- reviewer finding 2 は `plan/90-source-traceability.md` の omission であり、reports 0141 / 0142 の両方を追えるようにする必要があった。
- reviewer は semantic mismatch や machine-check overclaim は見つけておらず、
  - `run_bundle()` は `expected_static.verdict` のみ比較
  - `static_gate_detailed()` は actual sorted reasons を返す
  - `expected_static.reasons` は valid / underdeclared fixture で user-facing wording を持つ
 という current code / fixture / docs の整合を確認している。

## 7. Changes in understanding

- この task の close に必要なのは、新しい machine-check 機能ではなく、carrier 境界を誤主張しないことだった。
- reviewer completion まで待つと、semantic overclaim の有無だけでなく roadmap freshness / traceability omission のような bookkeeping drift も拾える。

## 8. Open questions

- future checker API で `expected_static.reasons` 相当を導入する場合、checked string list / typed code / detached-only 維持のどれが最小か。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、expected_static.reasons の dual-use carrier を future checker API でどう分離するのが最小かを、checked_reasons / typed reason code / detached-only 維持の 3 案で source-backed に比較してください。`
