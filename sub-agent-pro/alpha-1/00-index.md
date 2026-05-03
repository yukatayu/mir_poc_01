# Alpha-1 Handoff Index — Practical 100% Rebaseline

この directory は、新規コンテキストの Codex が Mirrorea practical alpha-1 へ進むための handoff である。これは規範正本ではない。repo に取り込むときは、必要な内容を `specs/`, `plan/`, `progress.md`, `tasks.md`, `samples_progress.md`, sample README, report に mirror する。

## なぜこの handoff が必要か

現行 repo は `Stage A..F 100% current-scope closeout` と表示しているが、それは実用可能な alpha-1 完了ではない。現在の 100% は、helper-local / non-public / thin bridge / current-scope evidence closeout に対する 100% であり、ユーザが意図する「実用的な製品開発ができる 100%」とは異なる。

今後は次を徹底する。

- `current-scope evidence closeout` と `practical alpha-1 readiness` を別 metric にする。
- practical alpha-1 の 100% は、実際に source から検査・実行・package・transport・hot-plug・devtools・local save/load まで使える toolchain を意味する。
- final public product ではないが、実用 prototype product をこの上で作れる状態を alpha-1 とする。

## Files in this package

- `00-index.md`: この目次
- `01-current-state-gap.md`: 現状と progress 表示の乖離
- `02-practical-alpha1-definition.md`: practical alpha-1 の定義と 100% 条件
- `03-decisions.md`: ここで固定する意思決定
- `04-stage-roadmap.md`: 新 stage / phase roadmap
- `05-theory-freeze.md`: 型・契約・cut・hot-plug の理論要件
- `06-toolchain-architecture.md`: 実用 toolchain architecture
- `07-repository-structure.md`: repo 構成案
- `08-sample-matrix.md`: 必須 sample / E2E / negative tests
- `09-validation-plan.md`: validation / command floor
- `10-subagent-plan.md`: sub-agent 活用方針
- `11-docs-progress-protocol.md`: docs / dashboard / progress 修正規則
- `12-risk-register.md`: risk / overclaim 防止
- `13-autonomous-package-sequence.md`: 自走 package sequence
- `14-codex-operational-rules.md`: Codex 運用 rules

## Required first package

`P-A1-00 practical-alpha-rebaseline`:

1. `specs/18-practical-alpha1-scope.md` を追加。
2. `plan/44-practical-alpha1-roadmap.md` を追加。
3. `progress.md` / `tasks.md` / `samples_progress.md` の percentage semantics を実用 100% 前提へ修正。
4. 現行 Stage A..F closeout を evidence closeout として保持しつつ、practical readiness から分離。
5. report, validation, commit, push。

## Ground rule

迷った場合は、実装を進めるよりも overclaim を止めることを優先する。ただし、user 入力なしで narrow package を進められるなら止まらない。
