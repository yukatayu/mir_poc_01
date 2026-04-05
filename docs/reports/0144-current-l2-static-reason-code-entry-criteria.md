# Report 0144 — current L2 static reason code entry criteria

- Date: 2026-04-05T10:58:00Z
- Author / agent: Codex
- Scope: current L2 parser-free PoC の `checked_reasons` から typed static reason code へ進める条件、stable cluster、parametric shape を docs-only で整理する
- Decision levels touched: L2

## 1. Objective

current L2 で additive optional `checked_reasons` を導入した次段として、

- immediate 全面 code 化を避けつつ
- どの static gate reason cluster なら typed code に進めるか
- code 化するならどんな shape が自然か

を narrow に比較する。

## 2. Scope and assumptions

- current `checked_reasons` 実装は維持する
- runtime semantics、failure family、detached static gate artifact loop は変更しない
- final reason taxonomy と final serialization はまだ固定しない

## 3. Documents consulted

1. `README.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. `specs/examples/30-current-l2-first-checker-cut-entry-criteria.md`
9. `specs/examples/32-current-l2-static-gate-artifact-loop.md`
10. `specs/examples/33-current-l2-checked-static-reasons-carrier.md`
11. `plan/07-parser-free-poc-stack.md`
12. `plan/11-roadmap-near-term.md`
13. `plan/12-open-problems-and-risks.md`
14. `plan/90-source-traceability.md`
15. `progress.md`
16. `crates/mir-semantics/src/lib.rs`

## 4. Actions taken

1. `static_gate_detailed()` の actual reason source を読み、current emitted pattern を列挙した。
2. `checked_reasons` bridge を前提に、次の 3 案を比較した。
   - immediate 全面 typed code 化
   - `checked_reasons` を維持しつつ stable cluster だけ inventory 化
   - string carrier を当面維持
3. stable cluster の条件を
   - semantics が安定している
   - user-facing prose と切り離せる
   - parameter slot が明確
   - detached artifact / checker / theorem prover で共通 skeleton を持てる
   の 4 条件に整理した。
4. `specs/examples/34-current-l2-static-reason-code-entry-criteria.md` を追加し、cluster inventory と parametric shape の current judgment を記述した。
5. docs / plan / progress mirror を更新した。

## 5. Files changed

- `specs/examples/34-current-l2-static-reason-code-entry-criteria.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `plan/11-roadmap-near-term.md`
- `plan/12-open-problems-and-risks.md`
- `plan/90-source-traceability.md`
- `progress.md`
- `docs/reports/0144-current-l2-static-reason-code-entry-criteria.md`

## 6. Evidence / outputs / test results

- `static_gate_detailed()` から current actual reason pattern として次を再確認した。
  - missing lineage assertion
  - lineage assertion edge mismatch
  - declared target missing / mismatch
  - capability strengthens
  - missing chain head / predecessor / successor option
  - declaration duplicate 由来 reason
- このうち、duplicate reason は declaration index 内部構造との結びつきが強いため、current L2 では code 化を急がない方が自然と判断した。
- code 化に進めるとしても flat string enum ではなく、`kind + parameter slots` の parametric shape が必要だと分かった。

## 7. What changed in understanding

- `checked_reasons` を入れた直後に全面 code 化するのは still too early である。
- ただし code 化に向く cluster 自体はかなり明確であり、次段では detached artifact mirror か theorem prover 向け relation のどちらかへ narrow に接続できる。

## 8. Open questions

- typed reason code を最初に detached static gate artifact 側へ mirror するか、fixture-side machine-check 側へ入れるか。
- duplicate reason cluster を別 namespace に切る必要があるか。
- parametric code shape の exact serialization を JSON object row にするか、別 internal carrier にするか。

## 9. Suggested next prompt

`current L2 parser-free PoC 基盤を前提に、typed static reason code を最初に detached static gate artifact 側へ mirror するか、fixture-side checked carrier 側へ導入するかを source-backed に比較し、helper boundary を壊さない最小 actualization を選んでください。`
