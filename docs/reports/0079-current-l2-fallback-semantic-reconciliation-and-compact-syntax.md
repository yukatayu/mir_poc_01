# Report 0079 — current L2 fallback semantic reconciliation and compact syntax

- Date: 2026-04-02T12:09:21.744260Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC 基盤を前提にした fallback / `lease` semantic reconciliation と compact syntax candidate comparison、ならびに最小限の examples mirror 更新
- Decision levels touched: 既存の Mir-0 / Mir-1 境界は維持しつつ、current L2 examples companion 側の比較整理だけを補う

## 1. Objective

current L2 docs / fixtures / tests が採っている fallback 読みと、「寿命の長い外側 option へ fallback して使い続けたい」という直感が、どこで一致し、どこでズレるかを整理する。
そのうえで、fallback / preference chain の core notation として 2〜3 個の compact syntax candidate を比較し、current L2 companion notation として暫定的に最も筋が良い候補があるかを判断する。

## 2. Scope and assumptions

- 今回は current L2 semantics を変更せず、semantic reconciliation と compact syntax candidate comparison だけを扱った。
- parser grammar、production runtime、machine-readable catalog asset / manifest には進まない前提を維持した。
- 開始時点の既存 dirty state は tracked file では確認されず、`main...origin/main [ahead 9]` だけが残っていた。今回の差分は、examples mirror 4 ファイルと新規 report 1 ファイルである。
- fallback / `lease` の machine-check は、既存の E3 variant / E6 / E7 / E8 fixture と `mir-semantics` integration tests が current L2 reading をすでに支えている、という前提で確認した。

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
15. `specs/examples/02-current-l2-ast-fixture-schema.md`
16. `specs/examples/03-current-l2-evaluation-state-schema.md`
17. `specs/examples/04-current-l2-step-semantics.md`
18. `specs/examples/05-current-l2-oracle-api.md`
19. `specs/examples/06-current-l2-interpreter-skeleton.md`
20. `docs/reports/0018-fallback-preference-chain-and-lease-semantics.md`
21. `docs/reports/0019-fallback-preference-chain-canonical-normalization.md`
22. `docs/reports/0020-fallback-preference-chain-incompatible-branch-rejection-phase.md`
23. `docs/reports/0021-fallback-preference-chain-static-evidence-floor.md`
24. `docs/reports/0022-fallback-preference-chain-underdeclared-case-handling.md`
25. `docs/reports/0023-fallback-preference-chain-lineage-annotation-surface-form.md`
26. `docs/reports/0037-option-local-admit-runtime-admissibility-current-l2.md`
27. `docs/reports/0039-admit-vs-lease-trace-audit-current-l2.md`
28. `docs/reports/0043-non-admissible-reason-audit-metadata-shape-current-l2.md`
29. `docs/reports/0045-capability-mismatch-non-admissible-taxonomy-current-l2.md`
30. `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
31. `crates/mir-ast/tests/fixtures/current-l2/`
32. `crates/mir-semantics/src/harness.rs`
33. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 4. Actions taken

1. `code_mapper` を先に使い、fallback / `lease` の current L2 reading が `specs/04`、`specs/10`、`specs/12`、`specs/examples/00..02`、fixture E3/E6/E7/E8、integration tests にどう散っているかを read-only で棚卸しした。
2. current L2 では fallback が outer-longer-lifetime container ではなく guarded option chain として読まれていること、canonical chain は left-to-right flattening により outer / inner を残さないこと、rollback / `atomic_cut` でも degradation order は巻き戻らないことを再確認した。
3. その reading と user-intuition の一致点 / drift 点を比較し、方針としては semantics を変えず、prose で drift を明示する案を採るのが最小だと判断した。
4. compact syntax candidate を 3 案に絞って比較した。
   - Candidate A: 現行の explicit edge-row form
   - Candidate B: line-leading `>` による ladder form
   - Candidate C: `then` による prose-like ladder
5. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` を新設し、semantic reconciliation、2 案比較、compact syntax candidate comparison、暫定判断をまとめた。
6. `specs/examples/01-current-l2-surface-syntax-candidates.md` の chain declaration 節に、current L2 が outer lifetime extension を意味しないことと、compact syntax comparison への導線を追記した。
7. `Documentation.md` と `specs/00-document-map.md` に新しい examples 文書への最小導線を追加した。
8. 今回は新しい regression fixture や runtime semantics の変更は行わなかった。E3 variant / E6 / E7 / E8 の既存 machine-check で current L2 reading を支えるには十分だと判断した。
9. spec / examples 側の変更を先に 1 本コミットし、仕様本文コミット hash を固定した。
10. reviewer の review report 0080 に記録された「write-after-expiry を success guarantee に見せる prose の強さ」と「E3 variant を根拠列から落としていた点」を最小修正した。
11. review follow-up として `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` の write-after-expiry 要約を current L2 step semantics に合わせて補正し、その補正も別コミットに切り出した。

## 5. Files changed

- `Documentation.md`
- `specs/00-document-map.md`
- `specs/examples/01-current-l2-surface-syntax-candidates.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
- `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`

## 6. Evidence / outputs / test results

```bash
cargo test -p mir-semantics
python3 scripts/validate_docs.py
git diff --check
git add Documentation.md specs/00-document-map.md specs/examples/01-current-l2-surface-syntax-candidates.md specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
git commit --no-gpg-sign -m "fallback の意味整理と compact syntax 比較を追加する"
```

主要な実行結果は次のとおり。

- `cargo test -p mir-semantics`
  - unit test 2 件 pass
  - integration test 33 件 pass
  - doc test 0 件 pass
- `python3 scripts/validate_docs.py`
  - `Documentation scaffold looks complete.`
  - `Found 80 numbered report(s).`
- `git diff --check`
  - 出力なし
- `git commit --no-gpg-sign -m "fallback の意味整理と compact syntax 比較を追加する"`
  - `dcfe5924aa61b94591fe7c33dcd3da935ae6eb43`
- `git commit --no-gpg-sign -m "fallback の write-after-expiry 要約を補正する"`
  - `1d06a3ed2deaa5e44907d3bcd17b6905f8f41e7c`
- reviewer
  - `reviewer` subagent を 1 回だけ起動し、`wait_agent(timeout_ms=120000)` を 2 回行ったが completion は返らなかった。
  - 指示どおり retry は 1 回で止め、追加 retry は行わなかった。
  - ただし review artifact として `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md` が生成され、そこで指摘された 2 点を今回の最終化前に反映した。

仕様本文コミット hash:

- `dcfe5924aa61b94591fe7c33dcd3da935ae6eb43` `fallback の意味整理と compact syntax 比較を追加する`
- `1d06a3ed2deaa5e44907d3bcd17b6905f8f41e7c` `fallback の write-after-expiry 要約を補正する`

report 自身の commit hash は self-reference の都合で、この本文ではまだ固定していない。

### 6.1 current L2 fallback 読み

- current L2 は fallback を guarded option chain として読んでおり、outer-longer-lifetime container としては読んでいない。
- canonical chain は left-to-right flattening を採り、outer / inner nesting そのものは観測可能意味に残さない。
- earlier option への再昇格は禁止される。
- `lease-expired` 後の write は、later write-capable option があればその候補を試行でき、どの候補でも成立しなければ `Reject` である。
- `try` / rollback / `atomic_cut` は local state を巻き戻しても degradation order を巻き戻さない。

### 6.2 user-intuition との一致点と drift

- 一致する部分:
  - 後段にまだ有効な same-lineage option があれば request を継続できる。
  - write-after-expiry でも later write-capable option があればその候補を試行でき、success-side condition が満たされれば request を継続できる。
- drift している部分:
  - outer option を寿命の長い wrapper とみなして later に戻れるという読み。
  - rollback / cut で degradation order まで戻るという読み。
- drift の中心:
  - semantics と notation の両方にまたがるが、current L2 では semantics 側はすでに guarded option chain で揃っている。
  - 実務上の誤読源は notation が与える outer / inner 直感のほうが大きい。

### 6.3 2 案比較

- 案1: current L2 semantics を維持し、drift を prose / examples / regression で明示する
  - 最小性、machine-checkability、rollback / cut / `Reject` boundary の一貫性で有利。
- 案2: outer-longer-lifetime fallback を再導入する
  - 直感には合うが、canonical law、evaluation state、step semantics、fixture 読みへ広く波及する。

今回の結論は **案1を維持する** である。

### 6.4 compact syntax candidate の比較結果

- Candidate A: 現行の explicit edge-row form
  - 最も semantically honest で、current L2 では暫定 companion notation として維持するのが妥当。
- Candidate B: line-leading `>` ladder
  - 最も compact だが、operator 風に見えやすく current L2 ではまだ早い。
- Candidate C: `then` ladder
  - 非 C 的だが compact さで劣り、control-flow prose に寄りやすい。

### 6.5 review evidence

- reviewer completion は取得できなかったため、最終確認は local evidence に依拠した。
- local evidence として次を確認した。
  - `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` は guarded option chain / left-to-right monotone degradation / no re-promotion / write-after-expiry try-later-or-`Reject` を既存 current L2 reading の範囲で言い換えているだけで、新しい runtime rule を追加していない。
  - `specs/examples/01-current-l2-surface-syntax-candidates.md` の追記は chain declaration の companion notation に対する prose mirror だけであり、parser grammar や runtime semantics を固定していない。
  - 既存 fixture / tests は変更していないため、E3 variant / E6 / E7 / E8 を含む current machine-check surface はそのまま維持されている。

## 7. What changed in understanding

- current L2 の drift 中心は、「fallback semantics が曖昧」なのではなく、「nested に見える notation が outer-longer-lifetime 直感を誘発しやすい」点だった。
- E3 variant / E6 / E7 / E8 の machine-check があるため、semantic reconciliation のために新しい runtime 変更を入れる必要はなかった。
- compact notation の比較では、より短い案は作れるが、current L2 では explicit edge-row form のほうが semantics と rollback / cut / `Reject` 境界を誤読させにくい。

## 8. Open questions

- final parser syntax
- compact syntax candidate を parser grammar へ昇格させるかどうか
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、fallback / preference chain の compact syntax candidate のうち line-leading \`>\` ladder を本当に companion notation 候補へ上げる価値があるか、それとも current explicit edge-row form を維持するべきかを、examples の書き換えコストと誤読リスクの観点から比較してください。`
