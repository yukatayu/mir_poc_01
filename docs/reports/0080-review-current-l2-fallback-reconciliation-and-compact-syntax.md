# Report 0080 — review current L2 fallback reconciliation and compact syntax

- Date: 2026-04-02T13:40:00Z
- Author / agent: Codex
- Scope: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`、`specs/examples/01-current-l2-surface-syntax-candidates.md`、`specs/00-document-map.md`、`Documentation.md`、`docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md` の maintainer-style review
- Decision levels touched: current L2 companion notation と parser-free PoC 説明の整合確認のみ。規範意味は未変更

## 1. Objective

current L2 parser-free PoC 基盤に対する fallback / `lease` semantic reconciliation と compact syntax candidate comparison の変更が、既存 current L2 semantics と整合しているかを確認する。

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
10. `specs/examples/00-representative-mir-programs.md`
11. `specs/examples/01-current-l2-surface-syntax-candidates.md`
12. `specs/examples/04-current-l2-step-semantics.md`
13. `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
14. `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md`
15. `crates/mir-ast/tests/fixtures/current-l2/`
16. `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## 3. Actions taken

1. AGENTS.md の順序に従って入口文書と current L2 の基礎意味を再読した。
2. `specs/04-mir-core.md` の fallback / preference chain / `lease` 節と、`specs/examples/04-current-l2-step-semantics.md` の `PerformVia` 規則を基準に、対象 4 文書の prose を照合した。
3. `git diff dcfe592^ dcfe592` で今回の tracked diff を確認した。
4. parser-free fixture と integration test から、E3 variant / E6 / E7 / E8 がどこまで machine-check されているかを確認した。
5. maintainer 観点で semantic overstatement、既成事実化、docs/examples mirror、report の不足を切り分けた。

## 4. Files changed

- `docs/reports/0080-review-current-l2-fallback-reconciliation-and-compact-syntax.md`

## 5. Commands run and exact outputs

```bash
git status --short
```

```text
?? docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md
```

```bash
git show --stat --summary dcfe592
```

```text
 Documentation.md                                   |   3 +-
 specs/00-document-map.md                           |   5 +
 .../01-current-l2-surface-syntax-candidates.md     |   2 +
 ...2-fallback-reconciliation-and-compact-syntax.md | 198 +++++++++++++++++++++
 4 files changed, 207 insertions(+), 1 deletion(-)
 create mode 100644 specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md
```

```bash
rg -n "e8|monotone_degradation|monotone-degradation" crates/mir-semantics/tests/current_l2_minimal_interpreter.rs
```

```text
crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:164:        "e8-monotone-degradation-reject.json",
crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:190:        "e8-monotone-degradation-reject.json",
crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:229:            "e8-monotone-degradation-reject.json",
crates/mir-semantics/tests/current_l2_minimal_interpreter.rs:589:            "e8-monotone-degradation-reject",
```

## 6. Evidence / findings

1. Medium: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` は write-after-expiry を「後段 write-capable option があれば success」と要約しており、current L2 の実際の規則より強く読める。`specs/04-mir-core.md` は「後段へ進めるなら進む、無ければ `Reject`」までしか固定しておらず、`specs/examples/04-current-l2-step-semantics.md` でも admitted な後段 option の operation / `ensure` が失敗すれば最終的に `Reject` になりうる。該当箇所: `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:19`, `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:33`, `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:39`, `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:93`, `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md:185`.
2. Low: `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md` も同じ overstatement を繰り返しており、加えて machine-check の根拠を E6 / E7 / E8 に寄せすぎている。guarded option chain と non-admissible skip の説明には `e3-option-admit-chain.json` も実際に test coverage に含まれているため、review trace としては証拠列がやや不完全である。該当箇所: `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:18`, `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:58`, `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:68`, `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:114`, `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:121`, `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md:150`.

## 7. Changes in understanding

- current L2 semantics 自体は guarded option chain / monotone degradation / no re-promotion / rollback-does-not-restore-order で一貫していた。
- 問題は主に新設 prose の圧縮時に、「後段へ進める可能性」が「成功保証」に近く見える表現へ強まっている点だった。
- compact syntax comparison 自体は final parser syntax を既成事実化しておらず、Candidate A 暫定維持の mirror も `Documentation.md` と `specs/00-document-map.md` を含め概ね整合している。

## 8. Open questions

- write-after-expiry の短い説明を、`success` ではなく「後段 admissible candidate へ進める」に揃えるか。
- review report 0079 でも E3 variant を machine-check 根拠へ明示的に戻すか。

## 9. Suggested next prompt

`specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md` と `docs/reports/0079-current-l2-fallback-semantic-reconciliation-and-compact-syntax.md` の write-after-expiry 要約を、current L2 step semantics と矛盾しない表現に絞って修正してください。E3 variant を machine-check 根拠に含めるかどうかも併せて整理してください。
