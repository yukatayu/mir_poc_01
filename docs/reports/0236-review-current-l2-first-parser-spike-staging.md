# Report 0236 — review current L2 first parser spike staging

## 1. Title and identifier

- Report 0236
- review current L2 first parser spike staging

## 2. Objective

Report 0235 と関連差分について review を行い、

- first parser cut inventory と actual parser spike staging を混線させていないか
- lexical choice finalization を既成事実化していないか
- checker-led staged spike の順序が actual evidence と矛盾していないか
- plan / progress / traceability mirror drift がないか

を確認する。

## 3. Scope and assumptions

- 対象は `specs/examples/73-current-l2-first-parser-spike-staging.md` と関連 mirror 更新に限る。
- current L2 core semantics、parser-free PoC、failure family は変更しない。
- 今回は docs-only review に留める。

## 4. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/29-current-l2-first-parser-subset-inventory.md`
9. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
10. `specs/examples/73-current-l2-first-parser-spike-staging.md`
11. `plan/06-surface-notation-status.md`
12. `plan/11-roadmap-near-term.md`
13. `plan/12-open-problems-and-risks.md`
14. `plan/90-source-traceability.md`
15. `progress.md`
16. `docs/reports/0235-current-l2-first-parser-spike-staging.md`
17. `crates/mir-semantics/src/lib.rs`
18. `crates/mir-semantics/src/harness.rs`
19. `scripts/current_l2_same_lineage_checker.py`
20. `scripts/current_l2_missing_option_checker.py`
21. `scripts/current_l2_capability_checker.py`
22. `scripts/current_l2_try_rollback_structural_checker.py`
23. `scripts/current_l2_detached_loop.py`

## 5. Actions taken

1. reviewer subagent を 1 回だけ dispatch し、completion まで待った。
2. 指摘を technical requirement として読み替えた。
3. 指摘 1 に対して、stage 1 から predicate fragment を外し、guard slot だけを structural carrier として残すよう修正した。
4. 指摘 2 に対して、この report 自体を追加し、`plan/90-source-traceability.md` の future reference が dangling しないようにした。
5. 指摘 3 に対して、`progress.md` の作業ログ順を時系列に並べ直した。
6. 指摘 4 に対して、`plan/11-roadmap-near-term.md` で first parser cut inventory / checker-cut entry criteria を「これからの task」ではなく「既に source-backed に切れている baseline」として書き換えた。
7. 修正後に docs validation を再実行した。

## 6. Evidence / outputs / test results

### reviewer findings

- stage 1 に predicate fragment を混ぜていた問題
- `plan/90-source-traceability.md` の dangling report reference
- `progress.md` の作業ログ順 drift
- `plan/11-roadmap-near-term.md` の baseline / next-step 混線

### validation

```text
python3 scripts/validate_docs.py
```

```text
git diff --check
```

## 7. What changed in understanding

- checker-led staged spike の根拠を structural floor に限定するには、stage 1 の guard は opaque slot までに留め、predicate fragment 自体は later stage に残す方が正確である。
- roadmap mirror では「既に source-backed に閉じた前提」と「これからの next step」を分けて書かないと、parser inventory と staging の分離が崩れる。

## 8. Open questions

- stage 1 で guard slot を受けるとき、opaque node と minimal parsed node のどちらを later parser spike 前提に採るか。
- stage 2 と stage 3 の間で、request-side predicate fragment をどこまで actual parser spike に含めるか。

## 9. Files changed

- `specs/examples/73-current-l2-first-parser-spike-staging.md`
- `plan/11-roadmap-near-term.md`
- `progress.md`
- `docs/reports/0236-review-current-l2-first-parser-spike-staging.md`

## 10. Commands run

- `python3 scripts/validate_docs.py`
- `git diff --check`

## 11. plan/ updates

- `plan/11-roadmap-near-term.md` 更新
- `plan/90-source-traceability.md` はこの report 参照を解決済み

## 12. Suggested next prompt

`current L2 parser-free PoC 基盤と specs/examples/73-current-l2-first-parser-spike-staging.md を前提に、checker-led staged spike の stage 1 を actual parser spike として切るなら、declaration-side guard slot を opaque に留めるか minimal parsed node にするかを narrow に比較してください。`
