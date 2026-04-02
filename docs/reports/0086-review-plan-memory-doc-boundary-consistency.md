# Report 0086 — review plan memory doc boundary consistency

- Date: 2026-04-02T15:11:06.586934Z
- Author / agent: Codex
- Scope: uncommitted `plan/` repository-memory docs, modified `AGENTS.md`, modified `Documentation.md`, modified `specs/00-document-map.md`, and `docs/reports/0085-plan-repository-memory-externalization.md` の semantic / documentation review
- Decision levels touched: none; review only

## 1. Objective

current uncommitted documentation changesについて、repo canon との整合、current L2 fallback reading の表現、parser-free PoC helper stack の責務境界、`AGENTS.md` 追加 rule の非衝突性、report 0085 の構造と evidence quality を確認する。

## 2. Inputs consulted

- `AGENTS.md`
- `README.md`
- `Documentation.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/04-mir-core.md`
- `specs/06-prismcascade-positioning.md`
- `specs/09-invariants-and-constraints.md`
- `specs/10-open-questions.md`
- `specs/12-decision-register.md`
- `specs/examples/09-current-l2-bundle-loader.md`
- `specs/examples/10-current-l2-batch-runner.md`
- `specs/examples/11-current-l2-selection-helper.md`
- `specs/examples/12-current-l2-selection-profiles.md`
- `specs/examples/13-current-l2-profile-catalog.md`
- `specs/examples/15-current-l2-fallback-reconciliation-and-compact-syntax.md`
- `plan/00-index.md`
- `plan/01-status-at-a-glance.md`
- `plan/03-decision-strengths-and-boundaries.md`
- `plan/04-core-semantics-current-l2.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/06-surface-notation-status.md`
- `plan/07-parser-free-poc-stack.md`
- `plan/08-representative-programs-and-fixtures.md`
- `plan/09-helper-stack-and-responsibility-map.md`
- `plan/10-roadmap-overall.md`
- `plan/12-open-problems-and-risks.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/91-maintenance-rules.md`
- `crates/mir-semantics/src/harness.rs`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`
- `docs/reports/0085-plan-repository-memory-externalization.md`

## 3. Actions taken

1. entry docs と基礎 specs を read order に沿って再確認した。
2. scoped diff と new `plan/` 文書の本文を読み、settled / OPEN / future の切り分けを確認した。
3. fallback / `lease` / monotone degradation の記述を `specs/examples/15`、decision register、fixture coverage と照合した。
4. parser-free helper stack の責務記述を `specs/examples/09..13`、`crates/mir-semantics/src/harness.rs`、`crates/mir-semantics/tests/current_l2_minimal_interpreter.rs` と照合した。
5. `cargo test -p mir-semantics` を実行し、helper stack と fixture coverage の現行 behavior を確認した。
6. findings を severity 順に整理した。

## 4. Files changed

- `docs/reports/0086-review-plan-memory-doc-boundary-consistency.md`
- `plan/ 更新不要`

## 5. Commands run and exact outputs

```bash
git status --short
```

```text
 M AGENTS.md
 M Documentation.md
 M specs/00-document-map.md
?? docs/reports/0085-plan-repository-memory-externalization.md
?? plan/
?? "旧資料_参考_ChatGPT_01_69c5e3f6/"
```

```bash
cargo test -p mir-semantics
```

```text
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 33 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

## 6. Evidence / findings

- `plan/05` の fallback reading は、guarded option chain、left-to-right monotone degradation、no re-promotion、write-after-expiry try-later-or-`Reject`、rollback / `atomic_cut` non-interference について canon と整合していた。
- helper stack の大枠 call chain と alias set は code / tests と整合していた。
- ただし `plan/91` の最低限読む文書リストは、`AGENTS.md` が要求する `README.md` と core specs (`specs/01..03`, `specs/09`) を落としており、read-order guidance が衝突していた。
- `plan/09` の helper responsibility table は `run_directory_named_profile` に alias list まで含めており、`ProfileCatalog::aliases()` / `resolve()` と thin wrapper の責務分離を曖昧にしていた。
- report 0085 は source inventory は広いが、validation evidence が弱く、helper/code alignment を確認した具体的 command evidence が不足していた。
- reviewer completion: not requested for this review task; local diff inspection and test evidence used.

## 7. Changes in understanding

- current uncommitted docs の中心内容は概ね canon と合っており、主な問題は semantics そのものより instruction boundary と helper boundary の表現精度にあった。
- current repo では named profile catalog boundary がかなり明確に実装されているため、docs 側の責務混線は小さくても放置しない方がよい。

## 8. Open questions

- none

## 9. Suggested next prompt

`plan/91` の read-order guidance、`plan/09` の named-profile helper boundary、report 0085 の evidence section を今回の review findings に沿って修正してください。
