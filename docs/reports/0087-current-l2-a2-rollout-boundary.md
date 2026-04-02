# Report 0087 — current l2 a2 rollout boundary

- Date: 2026-04-02T16:00:19.062121Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤と 0079 / 0080 / 0081 / 0082 / 0083 / 0084 を前提に、explicit edge-row family の polished rendering（A2: hanging lineage continuation）を representative examples へどこまで広げるべきかを docs mirror の範囲で整理する。比較判断の基準 example は E3 variant / E6 / E7 / E8 に限定し、実際の rollout mirror は `specs/examples/00` にすでに存在する chain-heavy な representative code block にだけ適用する。
- Decision levels touched: current L2 companion notation の rollout boundary に関する L2 companion mirror だけを更新し、runtime semantics と machine-check surface は変更しない

## 1. Objective

current L2 semantics を guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` のまま維持しつつ、A2 を docs-only に留めるべきか、representative examples の first-choice rendering へ広げるべきかを判断する。

## 2. Scope and assumptions

- runtime semantics、fixture schema、interpreter、host harness、bundle / batch / selection / profile helper には触れない。
- 比較判断の基準 example は E3 variant / E6 / E7 / E8 に限る。
- 実際の rollout mirror は `specs/examples/00` にすでに存在する chain-heavy な representative code block だけに適用し、今回追加で新しい representative example は増やさない。
- task-start dirty state は tracked change なし、untracked の `diff_investigation_01.md`、`旧資料_参考_ChatGPT_01_69c5e3f6/`、`旧資料_参考_ChatGPT_02_Mirrorea_2025/` のみだった。
- `plan/` 維持ルールに従い、syntax candidate の current status が変わるため `plan/06-surface-notation-status.md` を同じ task で更新した。

## 3. Documents consulted

1. `AGENTS.md`
2. `README.md`
3. `Documentation.md`
4. `specs/00-document-map.md`
5. `specs/01-charter-and-decision-levels.md`
6. `specs/02-system-overview.md`
7. `specs/03-layer-model.md`
8. `specs/04-mir-core.md`
9. `specs/09-invariants-and-constraints.md`
10. `specs/10-open-questions.md`
11. `specs/11-roadmap-and-workstreams.md`
12. `specs/12-decision-register.md`
13. `specs/examples/00-representative-mir-programs.md`
14. `specs/examples/01-current-l2-surface-syntax-candidates.md`
15. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
16. `plan/06-surface-notation-status.md`
17. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
18. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
19. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
20. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
21. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
22. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
23. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
24. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
25. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
26. `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
27. `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`
28. `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md`
29. `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`
30. `crates/mir-ast/tests/fixtures/current-l2/`
31. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `code_mapper` を先に使い、A2 / A1 rendering の mirror 箇所、E3 / E6 / E7 / E8 の representative / fixture / test 対応、task-start dirty state を棚卸しした。
2. current L2 の fallback reading を再確認し、notation rollout の前提を次の 5 点に固定した。
   - guarded option chain
   - left-to-right monotone degradation
   - no re-promotion
   - write-after-expiry try-later-or-`Reject`
   - rollback / `atomic_cut` non-interference
3. rollout 境界を次の 3 案で比較した。
   - docs-only 維持
   - representative examples への limited rollout
   - fixture / tests まで含む broader rollout
4. 比較の結果、A2 を docs-only 候補に留めるのではなく、representative examples のうち fallback / preference chain 自体を主題にする code block へ limited rollout するのが最小だと判断した。
5. その判断に合わせて、`specs/examples/00-representative-mir-programs.md` の chain-heavy な representative code block を A2 first-choice rendering に揃えた。
   - E3
   - E3 variant
   - E4
   - E6
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` に、A2 は representative examples の chain-heavy code block に広げ、A1 は minimal snippet / shorthand に残すという boundary を追記した。
7. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` に、docs-only / limited / broader rollout の比較と暫定判断を追記した。
8. `specs/10-open-questions.md` と `plan/06-surface-notation-status.md` に mirror を足し、current L2 が A2 をどこまで広げるかの repo-wide wording を揃えた。
9. 今回は runtime semantics や machine-check surface を変えない方針なので、fixture JSON、host plan sidecar、interpreter、tests には触れなかった。
10. reviewer を 1 回だけ起動し、最初の wait は timeout、retry 1 回で completion を取得した。finding は 2 点で、`specs/examples/01-current-l2-surface-syntax-candidates.md` の representative example code block を A2 に揃えることと、この report の scope wording を「比較基準」と「実際の rollout mirror」で分けることだった。どちらも最小修正で反映した。

## 5. Files changed

- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `specs/10-open-questions.md`
- `plan/06-surface-notation-status.md`
- `docs/reports/0087-current-l2-a2-rollout-boundary.md`

## 6. Evidence / outputs / test results

### 6.1 Commands run

```bash
git status --short --branch
sed -n '1,240p' README.md
sed -n '1,260p' Documentation.md
sed -n '1,220p' specs/00-document-map.md
sed -n '1,220p' specs/01-charter-and-decision-levels.md
sed -n '1,220p' specs/02-system-overview.md
sed -n '1,220p' specs/03-layer-model.md
sed -n '1,240p' specs/04-mir-core.md
sed -n '1,220p' specs/09-invariants-and-constraints.md
sed -n '1,240p' specs/10-open-questions.md
sed -n '1,220p' specs/11-roadmap-and-workstreams.md
sed -n '1,240p' specs/12-decision-register.md
sed -n '1,260p' specs/examples/00-representative-mir-programs.md
sed -n '1,320p' specs/examples/01-current-l2-surface-syntax-candidates.md
sed -n '1,320p' specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
sed -n '1,220p' docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md
sed -n '1,260p' docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md
sed -n '1,240p' docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md
sed -n '1,220p' docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md
python3 scripts/new_report.py --slug current-l2-a2-rollout-boundary
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

### 6.2 Exact outputs

- task start `git status --short --branch`

```text
## main...origin/main
?? diff_investigation_01.md
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
?? "旧資料_参考_ChatGPT_02_Mirrorea_2025/"
```

- `python3 scripts/new_report.py --slug current-l2-a2-rollout-boundary`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0087-current-l2-a2-rollout-boundary.md
```

- final `cargo test -p mir-semantics`

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
running 2 tests
test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok
test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 33 tests
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

- final `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 87 numbered report(s).
```

- final `git diff --check`

```text
(no output)
```

### 6.3 Findings

- A2 を examples まで広げる価値はある。
  - outer / inner wrapper 誤読を prose だけでなく code block 側でも減らせる。
  - edge-local lineage と request-evaluation boundary の見せ方を representative examples 本体で揃えられる。
  - rollback / cut / `Reject` の読みを「edge row の列」として維持しやすい。
- ただし rollout は limited で十分である。
  - E3 / E6 を含む chain-heavy な representative code block には A2 を first-choice として広げる。
  - E7 / E8 は fixture / comparison prose の根拠として十分であり、今回 representative programs 本体へ追加しない。
  - fixture schema、interpreter、tests は notation rollout に巻き込まない。
- machine-check と human-facing explanation の境界は維持した。
  - tests は引き続き existing fixture / public behavior coverage を exact compare する。
  - A2 rollout は docs / prose mirror だけで扱い、`must_explain` も narrative explanation obligation のまま残す。
- reviewer completion は取得できた。
  - finding 1: `specs/examples/01` の representative example 当てはめ section がまだ A1 inline を practical norm として見せていたため、A2 に揃えた。
  - finding 2: report の scope が「比較基準」と「実際に更新した representative examples」を混同していたため、scope / assumptions を補正した。

## 7. What changed in understanding

- 0083 の時点では A2 first choice が syntax candidate 文書に主に留まっており、representative examples 本体を先に読む読者には A1 が依然として第一候補に見える余地があった。
- A2 の rollout は docs-only では弱いが、fixtures / tests まで広げると notation task の境界を越えやすい。representative examples の chain-heavy code block に限定するのが、current L2 の companion notation と machine-check surface の責務分担に最も素直である。
- E7 / E8 は rollout 比較の判断材料として重要だが、representative programs 本体へ直ちに追加しなくても current L2 の drift 防止には十分だと整理できた。

## 8. Open questions

- final parser syntax
- explicit edge-row family の rollout を将来どこまで広げるか
- line-leading `>` ladder を将来再比較する閾値
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、representative examples に残る fallback / preference chain の prose 説明について、A2 rendering を前提に outer/inner wrapper 誤読をさらに減らすための wording 整理が必要か、それとも現状で十分かを narrow scope で点検してください。`
