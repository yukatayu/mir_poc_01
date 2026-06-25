# Surface Mir Brace Completion Handoff — 00 Index

この handoff は、Mir / Mirrorea の本来意図を取り戻すための **Surface Mir alpha** 実装パッケージである。

最大の修正点は次である。

```text
Canonical place-scope syntax は `S { ... }` とする。
`S[ ... ]` は採用しない。Sugar としても採用しない。
配列・Map・indexed state の参照は通常どおり `expr[index]`。
```

この決定により、初期ブログ案の `S[ ... ]` に近い直感は保持しつつ、`[]` を配列・indexed state 用に温存し、source syntax の混乱を避ける。

## この handoff の目的

Codex が新規コンテキストでも迷わず、以下を完走できるようにする。

1. Surface Mir の source syntax を `S { ... }` で再定義する。
2. `package.mir.json` ではなく `.mir` source files を意味の正本にする。
3. Surface Mir を Core Mir / Runtime IR へ elaboration する。
4. Indexed state、auto communication、auto publish/observe、role admission、source patch hot-plug を実装する。
5. 既存 Product Alpha / Operational Suite を壊さず、source-first の workflow を追加する。
6. 進捗・タスク・サンプル・検証を一目で分かる形へ整理する。

## ファイル一覧

```text
00-index.md
01-final-decisions.md
02-why-previous-plan-drifted.md
03-surface-syntax-brace.md
04-grammar-conflict-analysis.md
05-surface-to-core-elaboration.md
06-indexed-state-semantics.md
07-auto-communication.md
08-role-admission-capability.md
09-source-patch-hotplug.md
10-computational-core-roadmap.md
11-posegraph-transform-roadmap.md
12-projection-backend-engine.md
13-sample-matrix.md
14-repository-changes.md
15-validation-commands.md
16-progress-tasks-replacement.md
17-subagent-review-plan.md
18-risk-register.md
19-package-sequence.md
20-final-acceptance.md
sample-blueprints/01-world-core.md
sample-blueprints/02-membership-chat.md
sample-blueprints/03-sugoroku.md
sample-blueprints/04-role-admission.md
sample-blueprints/05-source-patch.md
```

## 絶対に守る stop lines

- `S[ ... ]` を実装しない。
- `S[ ... ]` を sugar としても採用しない。
- `package.mir.json` を final source authority と書かない。
- `transition ... at ...` を user-facing primary surface として固定しない。
- `perform` / `publish` / `observe` を全てユーザが手書きする前提にしない。
- 通信・publish・observe の自動生成を hidden にしない。生成された Core IR / devtools には必ず出す。
- role claim を authority と見なさない。
- Indexed state の key を owner / authority と見なさない。
- Unity / UE / WASM / native / FFI を semantic owner にしない。
- final public grammar / ABI / SDK / production WAN / distributed durable save-load / LLVM/native codegen 完成を claim しない。
