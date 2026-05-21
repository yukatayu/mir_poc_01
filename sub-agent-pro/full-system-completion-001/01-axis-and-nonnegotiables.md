# 01 — Project Axis and Non-Negotiables

## 一文の軸

```text
Mir source files に system-wide semantics を書き、
それを型検査・検証・投影・実行することで、
Place をまたいで実行・通信・hot-plug・save/load・可視化できる
仮想空間システムを作る。
```

## 本質

Mir は単なる manifest 言語ではない。Mir は、仮想空間上で実際に起きる計算、状態遷移、依存関係、契約、権限、観測、fallback、save/load を所有する。

Mirrorea は、Mir で定義された system を実ノード・サーバ・クライアント・ブラウザ的 runtime・adapter 上に配置し、動かし、可視化し、安全に進化させる fabric である。

## 絶対に守ること

### 1. Mir source が意味の正本

`package.mir.json` は alpha surface または compiled/manifest artifact としてはよいが、最終の意味正本ではない。

最終的には:

```text
.mir source
  -> AST
  -> typed IR
  -> checker / proof/model obligations
  -> projection / deployment plan
  -> runtime artifacts
```

### 2. 外部 backend は semantic owner ではない

Unity / UE / WASM / native library / FFI は、外部処理 provider または renderer backend である。

外部 backend が以下を隠してはいけない。

- world authority
- membership
- capability
- witness
- fallback
- save/load
- transform dependency
- synchronization policy
- hot-plug semantics
- observation/redaction policy

### 3. AddOne drift を再発させない

`typed_host_io.add_one` のような host-owned adapter は「外部 effect 接続」の証拠であり、「Mir-owned computation」の証拠ではない。

Mir-owned computation は、Mir source / typed IR / runtime / future compiler target に計算が現れる必要がある。

### 4. C-like baseline から Rust-like へ

最終的には Rust 程度の表現力を目指す。ただし最初から Rust 全部を実装しない。

段階:

1. C-like safe subset
   - primitive values
   - variables
   - records
   - arrays
   - control flow
   - functions/imports
2. Effectful Mir
   - perform
   - publish/observe
   - witness/handoff
   - fallback
   - cut/save/load
3. Rust-like widening
   - ownership/borrow-like discipline
   - traits/interfaces limited
   - generics limited
   - async/effect handlers
4. backend/projection
   - server/client artifacts
   - packet schema
   - FFI schema
   - native/LLVM optional backend

### 5. Debug / observability は optional ではない

各 milestone は以下を持つ。

- input/source
- check
- run/evaluate
- observe/debug
- negative case
- save/load relation if relevant
- report/export

### 6. Alpha と final を混ぜない

分類を必ず分ける。

```text
evidence-ready
first-floor-ready
workflow-ready
product-alpha-ready
final-public-ready
production-ready
```

## Current target completion

この handoff が目指す「完全」は、まず **Full System V1** である。

Full System V1 は、production WAN/federation を含む巨大サービス完成ではなく、以下を満たす状態である。

- Mir textual source files を書ける
- C-like safe computation が書ける
- effect / contract / capability / witness / fallback / save/load が書ける
- parser / checker / runtime が通る
- local/Docker product sample が動く
- server/client projection manifest が生成される
- renderer/FFI/WASM/native provider boundary が型付きに扱われる
- Transform/PoseGraph の no-split-frame runtime sample が動く
- devtools viewer で状態を見られる
- native host launch bundle または equivalent bundle が作れる
- release-check が全て通る

Full System V1 は、以下を non-goal として残してよい。

- global production WAN federation
- arbitrary native package execution
- full VRChat / VRM / Unity compatibility
- direct LLVM optimizer-grade backend if projection/interpreter backend is not yet ready
- hosted service / marketplace
- Reversed Library application
