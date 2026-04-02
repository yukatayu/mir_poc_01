# Report 0083 — current l2 explicit edge row notation polishing

## 1. Title and identifier

- Report ID: 0083
- Date: 2026-04-02T13:55:24.147939Z
- Author / agent: Codex

## 2. Objective

current L2 semantics を guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` のまま維持しつつ、explicit edge-row form の内部で 2〜3 個の微修正版 notation candidate を比較し、縦方向に読みやすく、outer / inner wrapper 誤読を起こしにくい polished companion notation 候補があるかを整理する。

## 3. Scope and assumptions

- 対象は current L2 companion notation の prose / examples mirror に限る。
- runtime semantics、fixture schema、interpreter、host harness、bundle / batch / selection / profile helper は変更しない。
- 比較対象 example は E3 variant / E6 / E7 / E8 に限る。
- 既存 dirty state は作業開始時点では無く、`git status --short --branch` は `## main...origin/main [ahead 3]` のみだった。
- reviewer は最後に 1 回だけ使い、長めに待つ。返らない場合だけ retry を 1 回行う。

## 4. Documents consulted

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
16. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
17. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
18. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
19. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
20. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
21. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
22. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
23. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
24. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
25. `docs/reports/0081-current-l2-ladder-vs-edge-row-notation.md`
26. `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`
27. `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`
28. `crates/mir-ast/tests/fixtures/current-l2/`
29. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 5. Actions taken

1. `code_mapper` を先に使い、current L2 fallback semantics の canonical anchor、explicit edge-row form の mirror 箇所、最小変更で済むファイル、既存 dirty state を棚卸しした。
2. `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` を基準に、notation polishing の前提を再確認した。
   - guarded option chain
   - left-to-right monotone degradation
   - no re-promotion
   - write-after-expiry 後の later write-capable option 試行または `Reject`
   - rollback / `atomic_cut` による degradation order 非干渉
3. 既存の family-level 比較を維持したまま、explicit edge-row family の内部候補を 3 つに絞った。
   - A1: current inline row
   - A2: hanging lineage continuation
   - A3: packed metadata row
4. 比較対象 example を E3 variant / E6 / E7 / E8 に限定し、次の基準で評価した。
   - 視覚的な簡潔さ
   - インデントとの相性
   - 縦方向の流れの読みやすさ
   - outer / inner wrapper 直感を不必要に誘発しないか
   - edge-local lineage / request-evaluation boundary を保てるか
   - rollback / cut / `Reject` 境界を誤読させないか
5. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` に、explicit edge-row family 内の polishing 比較、暫定判断、current L2 に残す事項を追記した。
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` の chain declaration 節を更新し、current L2 では hanging continuation を polished first choice としつつ、inline 省略形も companion-equivalent shorthand として残す方針を明記した。
7. reviewer の指摘を受けて、`specs/examples/01-current-l2-surface-syntax-candidates.md` の continuation 規則まで A2 を降ろし、`specs/examples/00-representative-mir-programs.md` と `specs/10-open-questions.md` の mirror も最小更新した。
8. 今回は semantics を変更していないため、fixture、host plan sidecar、interpreter、host harness、tests のコードは変更しなかった。

## 6. Files changed

- `specs/10-open-questions.md`
- `specs/examples/00-representative-mir-programs.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md`
- `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`

## 7. Evidence / outputs / test results

### 7.1 Commands run

```bash
git status --short --branch
sed -n '1,220p' AGENTS.md
sed -n '1,220p' README.md
sed -n '1,220p' Documentation.md
sed -n '1,240p' specs/00-document-map.md
sed -n '1,240p' specs/01-charter-and-decision-levels.md
sed -n '1,240p' specs/02-system-overview.md
sed -n '1,240p' specs/03-layer-model.md
sed -n '1,260p' specs/09-invariants-and-constraints.md
sed -n '1,260p' specs/10-open-questions.md
sed -n '1,260p' specs/11-roadmap-and-workstreams.md
sed -n '1,320p' specs/12-decision-register.md
sed -n '1,260p' specs/examples/00-representative-mir-programs.md
sed -n '1,320p' specs/examples/01-current-l2-surface-syntax-candidates.md
sed -n '1,320p' specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
sed -n '1,260p' docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md
sed -n '1,260p' docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md
sed -n '1,240p' docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md
rg -n 'fallback|preference chain|lease|monotone degradation|re-promotion|guarded option chain' specs/04-mir-core.md specs/10-open-questions.md specs/12-decision-register.md
python3 scripts/new_report.py --slug current-l2-explicit-edge-row-notation-polishing
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
```

### 7.2 Exact outputs

- `git status --short --branch` at start

```text
## main...origin/main [ahead 3]
```

- `python3 scripts/new_report.py --slug current-l2-explicit-edge-row-notation-polishing`

```text
/home/yukatayu/dev/mir_poc_01/docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md
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
Found 84 numbered report(s).
```

- final `git diff --check`

```text
(no output)
```

### 7.3 Findings

- current L2 の semantic 前提は維持されている。
  - fallback は guarded option chain
  - canonical chain は left-to-right monotone degradation
  - earlier option への再昇格は禁止
  - `lease-expired` 後の write は later write-capable option を試行し、どの候補でも成立しなければ `Reject`
  - rollback / `atomic_cut` は degradation order を巻き戻さない
- notation polishing の比較結果
  - A1: current inline row
    - mirror 変更は最小だが、E7 / E8 では横幅が伸びやすい。
  - A2: hanging lineage continuation
    - `fallback <successor>` を row head に絞れるため、E3 variant / E6 / E7 / E8 で縦方向の流れが最も読みやすい。
    - edge-local lineage は continuation metadata として保たれ、outer / inner wrapper 誤読や operator 読みを増やさない。
  - A3: packed metadata row
    - option-local `target` / `capability` / `lease` を edge row に重ねるため、role separation を壊しやすく、AST encoding 感が強い。
- 暫定判断
  - family としては explicit edge-row form を維持する。
  - polished rendering の第一候補は A2 とする。
  - A1 は companion-equivalent shorthand として残す。
  - A3 は current L2 companion notation に採らない。
- machine-check と prose の境界
  - machine-check 側の根拠列は引き続き E3 variant / E6 / E7 / E8 にある。
  - 今回の変更は companion notation の prose mirror だけであり、fixture schema や interpreter behavior には触れていない。
  - `must_explain` は従来どおり narrative explanation obligation に残し、machine-check に上げていない。

### 7.4 Review outcome と commit hashes

- reviewer は 1 回だけ起動し、1 回目の `wait_agent` は timeout、2 回目で completion を返した。
- reviewer の具体的な指摘は `docs/reports/0084-current-l2-explicit-edge-row-notation-polishing.md` ではなく、`docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md` に記録されている。
- 指摘は 3 点だった。
  - A2 hanging continuation の continuation / dedent rule を `specs/examples/01-current-l2-surface-syntax-candidates.md` の一般節まで降ろすこと
  - `specs/examples/00-representative-mir-programs.md` と `specs/10-open-questions.md` の優先順位 mirror を揃えること
  - この report を AGENTS.md の report structure に合わせ、outputs を明記すること
- これらは今回の最終差分で反映済みである。

仕様本文コミット hash:

- `1a97f08` `explicit edge-row notation の polishing を整理する`

report 側の commit hash は self-reference の都合で、この本文ではまだ固定しない。

## 8. What changed in understanding

- 0079 / 0081 で A family を維持する判断をした後でも、edge-row family の内部ではまだ「縦方向の読みやすさ」を改善する余地があった。
- current L2 で本当に守るべきなのは explicit edge-row family 自体より、edge-local lineage と request-evaluation boundary の可視性であり、A2 はそこを崩さずに横幅だけを減らせる。
- compact 化のために `target` / `capability` / `lease` を edge row へ再掲すると、notation 上の self-containment は増えるが、current L2 が大事にしている option-local surface と edge-local surface の分離を損ないやすい。
- reviewer の指摘により、A2 を first choice にするなら continuation / dedent 規則まで一般節へ降ろす必要があること、さらに入口 mirror でも A1/A2 の優先順位を揃える必要があることが明確になった。

## 9. Open questions

- final parser syntax
- explicit edge-row family の polished rendering を examples 全体へどこまで広げるか
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

## 10. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、explicit edge-row family の polished rendering（hanging lineage continuation）を representative examples 全体へどこまで広げるべきか、E3 variant / E6 / E7 / E8 から先に揃える案と docs-only に留める案を比較整理してください。`
