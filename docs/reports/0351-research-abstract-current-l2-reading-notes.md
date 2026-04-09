# Report 0351 — research abstract current L2 reading notes

- Date: 2026-04-09 11:22 JST
- Author / agent: Codex
- Scope: `docs/research_abstract/` に current L2 の短い用語補足を追加し、Phase 1〜3 要約から参照できるようにする
- Decision levels touched: none (companion explanation only)

## 1. Objective

`docs/research_abstract/` の current L2 要約を読んだときに起きやすい基礎的な疑問、特に

- `fallback` が 2 種類あること
- `writer` / `write` などの識別子が built-in かどうか
- `require` / `ensure` / `admit`
- `atomic_cut` の位置づけ
- `perform` / `option`
- `lineage` と `@`

の説明不足を補い、phase 要約を本質だけ読み返すときの混乱を減らす。

## 2. Inputs consulted

- `README.md`
- `Documentation.md`
- `progress.md`
- `specs/00-document-map.md`
- `specs/01-charter-and-decision-levels.md`
- `specs/02-system-overview.md`
- `specs/03-layer-model.md`
- `specs/09-invariants-and-constraints.md`
- `specs/examples/04-current-l2-step-semantics.md`
- `specs/examples/05-current-l2-oracle-api.md`
- `plan/05-fallback-lease-and-chain-semantics.md`
- `plan/14-glossary-and-boundary-rules.md`
- `plan/16-shared-space-membership-and-example-boundary.md`
- `docs/research_abstract/README.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`

## 3. Actions taken

1. `docs/research_abstract/current-l2-reading-notes.md` を新設し、current L2 を読むときに引っかかりやすい語の短い補足を追加した。
2. `docs/research_abstract/README.md` に補助メモへの導線を追加した。
3. Phase 1〜3 の research abstract 冒頭に、この補足メモへの参照を追加した。
4. `require` / `ensure` / `admit`、`TryFallback` / `AtomicCut`、`fallback` chain の説明が規範側の current wording と矛盾しないことを `specs/examples/04`、`specs/examples/05`、`plan/05`、`plan/14` で照合した。

## 4. Files changed

- `docs/research_abstract/README.md`
- `docs/research_abstract/current-l2-reading-notes.md`
- `docs/research_abstract/phase1-current-l2-semantics-stabilization.md`
- `docs/research_abstract/phase2-parser-free-poc-and-detached-validation-loop.md`
- `docs/research_abstract/phase3-parser-boundary-and-first-checker-cut.md`
- `docs/reports/0351-research-abstract-current-l2-reading-notes.md`

## 5. Commands run and exact outputs

```text
$ date '+%Y-%m-%d %H:%M JST'
2026-04-09 11:22 JST

$ python3 scripts/new_report.py --slug research-abstract-current-l2-reading-notes
/home/yukatayu/dev/mir_poc_01/docs/reports/0351-research-abstract-current-l2-reading-notes.md
```

検証コマンドは task close 前に別途実行する。

## 6. Evidence / findings

- `research_abstract` は phase ごとの本質を短く読むには有用だが、current L2 の companion notation を知らない読者には次の誤解が起きやすい。
  - chain fallback と `try` fallback を同一視する
  - example の `writer` / `write` を built-in と誤解する
  - `admit` を request-level precondition と読む
  - `atomic_cut` を global cut や transitive synchronization と誤読する
- これらは規範変更ではなく、既存判断の説明不足である。
- よって phase 要約を膨らませるより、短い reading note を別置きし、各 phase から参照する方が condensed summary の役割を崩さない。

## 7. Changes in understanding

- `research_abstract` では、phase 要約本体にすべての導入説明を書き込むより、cross-phase な読み方メモを 1 枚持つ方が保守しやすい。
- 特に current L2 では、companion notation と runtime semantics の語が近接しているため、「似ているが同じではない」概念の short clarification が有効である。

## 8. Open questions

- `research_abstract/current-l2-reading-notes.md` にどこまで detached loop / shared-space 側の語を追加するか。
- 将来の final parser grammar 決定後に、companion notation 起源の説明をどこまで整理し直すか。
- `plan/ 更新不要`
  - 今回は長期 repository memory や current working judgment を変更していない。
- `progress.md 更新不要`
  - current status / phase / rough progress は変わっていない。
- `tasks.md 更新不要`
  - current task map や blocker snapshot は変わっていない。

## 9. Suggested next prompt

`docs/research_abstract/current-l2-reading-notes.md` を前提に、shared-space / membership line についても同じ粒度の reading note が必要かを narrow に比較してください。
