# Report 0084 — review current l2 explicit edge row notation polishing

- Date: 2026-04-02T14:04:53.715659Z
- Author / agent: Codex
- Scope: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md` の maintainer-style review
- Decision levels touched: review only。current L2 semantics と implementation boundary は未変更

## 1. Objective

current L2 parser-free PoC 基盤に対する explicit edge-row family 内の compact notation polishing が、既存 semantics、docs mirror、option-local / edge-local responsibility split、AGENTS.md の report policy を壊していないかを確認する。

## 2. Scope and assumptions

- 対象は docs-only 変更に限定する。
- 基準 semantics は `specs/04-mir-core.md`、`specs/10-open-questions.md`、`specs/12-decision-register.md` の current L2 reading とする。
- parser-free fixture / interpreter は implementation boundary の確認材料として参照するが、今回の review では挙動変更を加えない。
- A1 / A2 / A3 の比較は、変更文書自身が明示する「explicit edge-row family の内部だけを polishing する」という前提の中で評価する。

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
11. `specs/12-decision-register.md`
12. `specs/examples/00-representative-mir-programs.md`
13. `specs/examples/01-current-l2-surface-syntax-candidates.md`
14. `specs/examples/04-current-l2-step-semantics.md`
15. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
16. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
17. `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`
18. `docs/reports/0082-review-current-l2-fallback-compact-syntax-comparison.md`
19. `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md`
20. `docs/reports/TEMPLATE.md`
21. `crates/mir-ast/tests/fixtures/current-l2/`
22. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. AGENTS.md の順序に従って入口文書と基礎仕様を再読した。
2. `git diff` で対象 3 ファイルの差分を確認し、変更が semantics 変更ではなく notation polishing と report 追加に留まっていることを切り分けた。
3. `specs/04`、`specs/10`、`specs/12`、`specs/examples/04` を基準に、guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` / rollback-cut non-interference を照合した。
4. `specs/examples/01` と `specs/examples/15` の新記述を、`specs/examples/00` と `specs/10` の mirror 記述、および block/continuation の読み規則と突き合わせた。
5. `docs/reports/0083` を AGENTS.md と `docs/reports/TEMPLATE.md` の report structure と比較し、必須項目と evidence 記録の充足度を確認した。

## 5. Files changed

- `docs/reports/0084-review-current-l2-explicit-edge-row-notation-polishing.md`

## 6. Commands run and exact outputs

```bash
git status --short -- specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md specs/examples/01-current-l2-surface-syntax-candidates.md docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md
```

```text
 M specs/examples/01-current-l2-surface-syntax-candidates.md
 M specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
?? docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md
```

```bash
git diff -- specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md specs/examples/01-current-l2-surface-syntax-candidates.md docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md
```

```text
主要差分:
- `specs/examples/15...` に explicit edge-row family 内部の A1 / A2 / A3 比較と A2 優先判断を追加
- `specs/examples/01...` の chain declaration を hanging continuation を polished first choice、inline を shorthand とする説明へ更新
- `docs/reports/0083...` を新規追加
```

```bash
rg -n "fallback successor @ lineage|polished rendering|companion-equivalent|fallback mirror @ lineage|chain ref = head" specs/examples/00-representative-mir-programs.md specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md specs/10-open-questions.md
```

```text
specs/examples/00-representative-mir-programs.md:23:- `chain ref = head` と、それに続く `fallback successor @ lineage(predecessor -> successor)`
specs/examples/00-representative-mir-programs.md:165:        fallback mirror @ lineage(primary -> mirror)
specs/examples/00-representative-mir-programs.md:166:        fallback readonly @ lineage(mirror -> readonly)
specs/examples/00-representative-mir-programs.md:218:        fallback delegated_writer @ lineage(owner_writer -> delegated_writer)
specs/examples/01-current-l2-surface-syntax-candidates.md:167:- edge-local な `documented lineage annotation` は、各 `fallback` row にぶら下がる indented continuation line として `@ lineage(predecessor -> successor)` を置いてよい。current L2 では、この hanging continuation を polished rendering の第一候補とする。
specs/examples/01-current-l2-surface-syntax-candidates.md:168:- ただし row が短く収まり、視認性を落とさない場合に限って、`fallback mirror @ lineage(primary -> mirror)` のような inline 省略形を companion-equivalent な短い書き方として残してよい。
specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:304:- その family の中では、lineage annotation を hanging continuation にした A2 を polished rendering の第一候補に置く。
specs/10-open-questions.md:71:- option chain 側の候補書式として `option name on target capability cap lease guard` と `chain ref = head` / `fallback successor @ lineage(...)` を current L2 で使ってよいが、これも final parser syntax ではない。
```

## 7. Evidence / findings

1. Medium: 新しい A2 書式の attachment rule が `specs/examples/01-current-l2-surface-syntax-candidates.md` の一般的な block/continuation 規則まで降りていない。chain declaration 節では `@ lineage(...)` を `fallback` row にぶら下がる indented continuation line と定義している一方、statement separator / block nesting 節は chain continuation を「indented された `fallback successor ...` 行」としか説明していない。A2 を polished first choice に上げるなら、さらに深い `@ lineage(...)` 行がどの row に属し、どの dedent で終わるかも同じ節で明示した方がよい。該当箇所: `specs/examples/01-current-l2-surface-syntax-candidates.md:167`, `specs/examples/01-current-l2-surface-syntax-candidates.md:214`, `specs/examples/01-current-l2-surface-syntax-candidates.md:217`.
2. Low: A2 を polished first choice、A1 を shorthand とする判断は `specs/examples/01` と `specs/examples/15` では明示されたが、主要 mirror の一部はまだ A1 だけを current candidate として見せている。A1 自体は引き続き許容されるので意味論矛盾ではないが、入口文書だけ読んだ読者には A1 が依然として第一候補に見える。該当箇所: `specs/examples/00-representative-mir-programs.md:23`, `specs/examples/00-representative-mir-programs.md:164`, `specs/examples/01-current-l2-surface-syntax-candidates.md:167`, `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:304`, `specs/10-open-questions.md:71`.
3. Medium: `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md` は AGENTS.md の report policy を厳密には満たしていない。header に scope はあるが、要求されている `Scope and assumptions` が独立 section として置かれておらず、さらに `Commands run and exact outputs` にはコマンド列しかなく output が記録されていない。AGENTS.md は report structure を correctness の一部として扱っているため、これは process-only ではなく repository policy 上の defect である。該当箇所: `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md:8`, `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md:12`, `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md:74`.
4. No findings: current L2 semantics 自体は今回の差分で変わっていない。`guarded option chain`、`left-to-right monotone degradation`、`no re-promotion`、`write-after-expiry` の try-later-or-`Reject`、rollback-cut non-interference は `specs/04-mir-core.md:118-143`、`specs/10-open-questions.md:8-19`、`specs/12-decision-register.md:30-31`、`specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:202-209` と整合していた。
5. No findings: A1 / A2 / A3 比較は、`target` / `capability` / `lease` を option-local surface に残すという current L2 前提を明示したうえで行われており、その前提の中では fair だった。A2 優先判断も option-local surface と edge-local surface の責務分担を壊していない。

## 8. What changed in understanding

- 今回の drift は semantics ではなく、surface reading の明文化レベルと mirror の優先順位表示に集中していた。
- A2 自体は既存理論や implementation boundary を壊していないが、first choice に上げたなら indentation discipline まで追記しないと companion notation の安定性が落ちる。
- report policy は metadata header で概ね満たした気になりやすいが、この repository では section order と evidence の明記まで含めて correctness とみなす必要がある。

## 9. Open questions

- A2 の attachment / dedent rule を `specs/examples/01` の block nesting 節に追加するか、それとも chain declaration 節だけで十分とみなすか。
- `specs/examples/00` と `specs/10` も A2 first choice を mirror するか、それとも A1 shorthand の実例を残したまま `specs/examples/01` 参照だけで済ませるか。
- report 0083 を minimal fix で整える場合、独立した `Scope and assumptions` section と command outputs のどこまでを追記するか。

## 10. Suggested next prompt

`specs/examples/01-current-l2-surface-syntax-candidates.md` の chain continuation reading を A2 まで含めて明文化し、必要なら `specs/examples/00-representative-mir-programs.md` と `specs/10-open-questions.md` の mirror も最小更新してください。あわせて `docs/reports/0083-current-l2-explicit-edge-row-notation-polishing.md` を AGENTS.md の report policy に合わせて整えてください。`
