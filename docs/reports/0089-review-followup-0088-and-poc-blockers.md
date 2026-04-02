# Report 0089 — review followup 0088 and poc blockers

- Date: 2026-04-02T16:36:40.159581Z
- Author / agent: Codex
- Scope: 途中で残っていた `0087` のコミットを先に確定し、その後 `0088` の wording task がすでに commit 済みであることを確認したうえで、reviewer completion と local verification を追加で揃える。current L2 semantics や machine-check surface は変更せず、report に reviewer follow-up と「PoC を大量に回す前の主ボトルネック」を整理して残す。
- Decision levels touched: 既存 L2 判断の wording / validation 状態の確認だけであり、新しい meaning judgment は追加しない

## 1. Objective

前タスクの未完了分を repo 上で閉じ、`0088-current-l2-representative-prose-drift-check.md` の内容が current repo と整合しているかを reviewer completion まで確認する。
そのうえで、current L2 parser-free PoC 基盤を「狭く正確に回す」段階から「大量に回して比較し続ける」段階へ進むために、何が次のボトルネックになるかを整理する。

## 2. Inputs consulted

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
16. `plan/01-status-at-a-glance.md`
17. `plan/05-fallback-lease-and-chain-semantics.md`
18. `plan/06-surface-notation-status.md`
19. `plan/08-representative-programs-and-fixtures.md`
20. `docs/reports/0087-current-l2-a2-rollout-boundary.md`
21. `docs/reports/0088-current-l2-representative-prose-drift-check.md`
22. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. task 開始時点で open のままだった subagent を閉じ、`0087` の未完了差分を local validation 付きで確認した。
2. `0087` の差分は reviewer finding を反映済みだったため、`5b34698` `A2 rendering の rollout 境界を整理する` として commit した。
3. 続けて今回 task の対象を確認したところ、representative examples prose wording 整理はすでに `82bdff3` `representative examples prose の drift を整流する` として commit 済みであることを確認した。
4. `specs/examples/00`、`specs/examples/15`、`plan/05`、`plan/06`、`plan/08`、`docs/reports/0088` を読み直し、A2 hanging continuation と write-after-expiry の wording が current L2 reading に沿っているか spot-check した。
5. reviewer を 1 回だけ起動し、completion まで待った。reviewer からは、今回 turn で追加修正すべき finding はなく、「repo にはすでに `82bdff3` が入っており task 本体は反映済み」との確認が返った。
6. `python3 scripts/validate_docs.py`、`git diff --check`、`cargo test -p mir-semantics` を取り直し、repo 状態を再確認した。
7. `plan/` mirror については、この turn では新しい判断を追加していないため **`plan/ 更新不要`** とした。

## 4. Files changed

- `docs/reports/0089-review-followup-0088-and-poc-blockers.md`

## 5. Commands run and exact outputs

```bash
git status --short --branch
git diff -- specs/examples/00-representative-mir-programs.md \
  specs/examples/01-current-l2-surface-syntax-candidates.md \
  specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md \
  specs/10-open-questions.md \
  plan/06-surface-notation-status.md \
  docs/reports/0087-current-l2-a2-rollout-boundary.md
python3 scripts/validate_docs.py
git diff --check
git -c commit.gpgsign=false commit -m "A2 rendering の rollout 境界を整理する"
git log --oneline -2
cargo test -p mir-semantics
```

### Exact outputs

- `git -c commit.gpgsign=false commit -m "A2 rendering の rollout 境界を整理する"`

```text
[main 5b34698] A2 rendering の rollout 境界を整理する
 6 files changed, 284 insertions(+), 10 deletions(-)
 create mode 100644 docs/reports/0087-current-l2-a2-rollout-boundary.md
```

- final `git log --oneline -2`

```text
82bdff3 representative examples prose の drift を整流する
5b34698 A2 rendering の rollout 境界を整理する
```

- final `python3 scripts/validate_docs.py`

```text
Documentation scaffold looks complete.
Found 88 numbered report(s).
```

- final `git diff --check`

```text
(no output)
```

- final `cargo test -p mir-semantics`

```text
Finished `test` profile [unoptimized + debuginfo] target(s) in 0.07s
running 2 tests
test harness::tests::named_profile_catalog_aliases_are_derived_from_internal_specs ... ok
test harness::tests::named_profile_catalog_resolve_is_derived_from_internal_specs ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 33 tests
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

running 0 tests
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 6. Evidence / findings

### 6.1 `0087` / `0088` の状態

- `0087` は reviewer 指摘を反映した docs mirror のまま `5b34698` で確定した。
- `0088` は task 本体がすでに `82bdff3` で commit 済みであり、この turn では追加の prose 修正は不要だった。
- reviewer completion でも、current repo は次の 3 点で揃っていると確認できた。
  - A2 の hanging continuation は outer/inner wrapper の追加入れ子ではなく、直前 edge row に属する metadata continuation
  - write-after-expiry は later write-capable option の存在だけで success ではなく、その option 自身の admissibility と request 充足が必要
  - rollback / `atomic_cut` は degradation order を巻き戻さない

### 6.2 PoC を「回しまくれる」段階へ進める前の主ボトルネック

今回の確認で見えたのは、current L2 の core reading や notation は、いま必要な安定度にはかなり達しているということである。次に重いのは syntax 候補ではなく、次の operational / verification 層である。

1. `fixture authoring / elaboration` の安定化  
   - final parser grammar を決めなくても fixture を増やし続けられる入力経路が必要である。
   - いまは AST fixture schema で十分回せているが、量が増えると authoring cost と drift 管理が効いてくる。

2. `detached trace / audit serialization`  
   - 現在の machine-check は強いが、実験結果を大量に比較・保存・再解析するには trace / audit を repo 外へ持ち出しやすい形が要る。
   - ここが弱いと PoC は「1 回ずつ読む」運用から抜けにくい。

3. `richer host interface` と coverage analysis  
   - host-plan sidecar は current L2 の最小 loop としては十分だが、bundle 数や scenario 数が増えると preflight / uncovered call detection / coverage explanation が主ボトルネックになる。

4. `multi-request scheduler`  
   - いまの loop は 1 request 単位ではかなり強い。
   - しかし PoC を本格的に回す段階では、複数 request / 並列 request / interleaving をどう持つかが未着手である。

5. `static analysis / theorem prover` との境界  
   - 何を current L2 machine-check に残し、何を external verifier 側へ送るかを明示しないと、PoC の広がり方がぶれる。
   - 型システム、決定可能性、定理証明可能性に触れる前提としても、この boundary が先に必要である。

6. `Approximate` / `Compensate` の再導入時期  
   - 今は active scope から外して正解である。
   - ただし richer runtime loop を考える段階では、failure surface をどこで再び広げるかを決める必要がある。

要するに、**次の主ボトルネックは notation ではなく、serialization / host integration / scheduler / analysis boundary** である。

## 7. Changes in understanding

- A2 rollout と representative prose drift 整理は、current repo ではすでに 2 commit に分けて収束していた。
- それにより、current L2 の fallback / `lease` 読みは docs / plan / examples でかなり安定しており、直近で再び semantics をいじる必要は見えていない。
- PoC をさらに前へ押し出すには、companion notation を磨く task よりも、PoC results をどう量産・保存・比較・外部連携するかの task を優先した方が効果が大きい。

## 8. Open questions

- detached trace / audit serialization の最小 boundary
- richer host interface と coverage analysis の最小 boundary
- multi-request scheduler の導入時期
- fixture authoring / elaboration を parser-free のままどこまで回せるか
- static analysis / theorem prover との boundary
- `Approximate` / `Compensate` を current loop に戻す閾値
- final parser syntax
- line-leading `>` ladder を再比較する閾値
- machine-readable catalog asset / manifest
- selector grammar / alias grammar の長期固定
- path canonicalization policy

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、detached trace / audit serialization と richer host interface / coverage analysis の最小 boundary を narrow scope で整理し、いまの parser-free fixture loop を “大量に回して比較できる PoC” に進めるために何を先に固定すべきかを比較してください。`
