# Full System Completion Handoff — Mir / Mirrorea

この handoff は、新規コンテキストの Codex に、現在の Mir / Mirrorea を **「本来の理想像」へ戻し、最後まで実装を進めるための全体情報**を渡すための package である。

## この package の前提

現在 repo は次を既に持つ。

- Product Alpha-1 release-candidate workflow
- `mirrorea-alpha` CLI
- `package.mir.json` alpha package surface
- local/Docker controlled runtime
- same-session hot-plug
- observer-safe devtools/viewer
- R0/R2 save evidence
- native host launch bundle
- installed-binary adoption probe
- canonical operational product sample suite
- Mir-owned computation first-floor evidence
- PoseGraph no-split-frame helper evidence
- projection/backend and engine/FFI/WASM boundary inventories

ただし、本来の理想像はまだ未完成である。

## 本来の理想像

Mir / Mirrorea の最終的な正本は `package.mir.json` ではなく、Mir の文法に従った **Mir source files** である。

```text
Mir source files
  world-core.mir
  membership-chat.mir
  sugoroku-world.mir
  avatar-pose.mir
  portal-worldlink.mir
  shard-boundary.mir
  ...

    -> parser / typed IR / checker / model obligations
    -> projection / deployment planning
    -> server artifact / browser-client artifact / adapter artifact
    -> Mirrorea runtime / browser-like client / renderer backend / FFI boundary
```

サーバ・クライアント・ブラウザ・headless client・renderer backend は、Mir source 由来の artifact を実行する。外部 engine、WASM、native library、Unity / UE は semantic owner ではなく、typed boundary を持つ backend/provider として扱う。

## 誤解の修正

以前の product alpha の `AddOne` は `typed_host_io.add_one` adapter evidence だった。これは「外部 adapter に typed に接続できる」証拠ではあるが、「Mir が `x + 1` を定義し、型検査し、実行している」証拠ではない。

現行 repo はこれを修正し、`samples/product-alpha1/computational/add-one-pure-mir/` 等で Mir-owned computation first floor を持つ。しかし、まだ Rust 程度の実用言語ではない。

## この package の使い方

1. まず `prompt-docs-rebaseline.md` を Codex に与え、既存 `progress.md` / `tasks.md` を完全に置き換えて、現在地と最終 milestones を明確化させる。
2. 次に `prompt-implementation-complete.md` を Codex に与え、`agents/` 以下の sub-agent を適宜使わせながら、全 milestones を順に完了させる。
3. Codex は package-by-package で止まってはいけない。各大 milestone の終了時に `progress.md` を更新し、必要な report を書き、validation を実行し、commit/push する。

## 収録ファイル

- `01-axis-and-nonnegotiables.md`
- `02-current-state-and-gap.md`
- `03-final-system-milestones.md`
- `04-mir-language-design.md`
- `05-computational-core-theory.md`
- `06-effect-contract-capability.md`
- `07-verification-model-check-proof.md`
- `08-cut-save-load-continuation.md`
- `09-membership-clock-compaction.md`
- `10-fallback-guarded-reference.md`
- `11-posegraph-transform.md`
- `12-projection-deployment-backend.md`
- `13-engine-ffi-wasm-adapters.md`
- `14-operational-samples.md`
- `15-repository-structure.md`
- `16-validation-and-release.md`
- `17-subagent-and-autonomy.md`
- `18-risk-register.md`
- `19-codex-package-sequence.md`
- `20-progress-tasks-replacement-model.md`
