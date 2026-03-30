# 04 — Mir Core

## 状態

Mir Core は、このプロジェクトで最も基盤的な部分である。
明示的に別記されていない限り、そのアーキテクチャは decision level L0 / L1 として扱うべきである。

この文書は次を区別する。

- **Mir 全体**: 後続の進化や統合作業も拘束する、より広い意味論的基盤。
- **Mir-0**: 最初に安定化すべき最小意味論カーネル。

明示的に別記されていない限り、以下の「Mir-0 の最小意味論コア」節が、Workstream A における Mir-0 の現在の焦点定義である。
これは先行する規範文書を置き換えるものではなく、`specs/01` から `specs/03` と `specs/09` を併読して読む。

## 四本柱

### 1. 因果と時間

- 計算は event の directed acyclic graph として表現される。
- finalization boundary は意味論上重要である。
- より広い Mir の cut vocabulary では以前から `barrier`、`atomic_cut`、`durable_cut` が言及されていたが、Mir-0 が固定するのは `atomic_cut` だけである。`barrier` と `durable_cut` の扱いは後段へ送る。
- システムは「何が何より前に起きたか」と「何がいつ final になったか」を説明できなければならない。

### 2. Effects と Contracts

- 外部作用は、隠れた実装詳細ではなく first-class effect である。
- Contracts（`require`、`ensure`、`invariant`）は実行可能意味の一部である。
- failure は汎用的な exception の入れ物ではなく、構造を持ち contract-aware でなければならない。

### 3. Ownership と Lifetime

- linear resource、あるいは ownership に類する resource は重要である。
- lifetime と fallback 風の degradation は monotone に保たれなければならない。
- preference chain と guarded reference は現在も活発な設計論点である。

### 4. 安全な進化

- patch は意味を保ち、graph discipline を尊重すべきである。
- 既定規則は arbitrary rewiring ではなく downstream addition による進化である。
- overlay は互換性を保ち、既存 API を silent shadowing してはならない。

Mir-0 は、patch、overlay、route rebinding の完全な運用意味論 (operational semantics) を固定しない。
ここで固定するのは、後続の mechanism が従うべき意味論的制約だけである。

## Mir-0 の最小意味論コア

ここで固定する最小 kernel は、意図的に狭い。
Mirrorea 固有、Prism 固有の machinery を持ち込まずに、因果、明示的 effect、rollback constraint、ownership discipline を述べるのに必要な primitive だけを含む。

### 1. Event DAG と因果

- Mir-0 の計算は semantic event の directed acyclic graph で表現される。
- graph は因果説明を明示しなければならない。すなわち、どの event がどの event より前に起き、どの boundary が先行 prefix を final にしたかを説明できなければならない。
- 意味の上で隠れた backward edge は許されない。

### 2. `place`

- Mir-0 は `place` の最小概念を含む。
- `place` は、local state、rollback scope、ownership transfer が解釈される最小の意味論的 locus である。
- Mir-0 は、same-place reasoning と cross-place interaction を区別できるだけの構造を要求する。
- 正確な routing、naming、distributed placement policy は Mir-0 の外にあり、後段の Mir または Mirrorea の作業に属する。

### 3. `perform`、effect、contract

- Mir-0 では、明示された contract の下で effect を要求する最小操作だけを仮定する。
- 本文ではその操作を便宜上 `perform` と表記するが、これは Mir-0 の規範的な表面構文としてはまだ確定しない。
- `perform` を最終的な予約語として採用するかどうかは **未決定** である。
- effect は、隠れた実装詳細ではなく first-class semantic action である。
- contract は admission と outcome space の両方を制約する。現在の vocabulary には `require`、`ensure`、`invariant` があるが、正確な表面構文はここでは固定しない。
- effect success、rejection、approximation、compensation は event graph 上で可視であり続けなければならない。

### 4. 最小 failure space

- Mir-0 は、少なくとも次の named outcome を持つ最小 structured failure space を固定する。
  - `Reject`
  - `Approximate`
  - `Compensate`
- contract は、どの degraded outcome が許可されるかを明示しなければならない。
- これら最小 class を超えた lattice-theoretic な正確な表現は **UNRESOLVED** である。

### 5. Primitive fallback

- Mir-0 は、明示的 failure または monotone degradation の後に、次の contract-compatible option へ進む primitive として fallback を含む。
- fallback は、隠れた backtracking、隠れた API shadowing、linear resource の duplication を導入してはならない。
- nested fallback / preference chain の完全な normalization は Mir-0 の外にある。

### 6. local rollback を伴う `try`

- Mir-0 は、local rollback semantics を持つ `try` を含む。
- rollback は current `place` に局所であり、finalizing cut を越えていない state だけを巻き戻せる。
- Mir-0 における `try` の rollback frontier は current `place` 内で閉じており、途中に `atomic_cut` があればそこで停止する。
- rollback 不可能な effect は rollback region の内部で禁止するか、そこから隔離するか、明示的 compensation で扱わなければならない。
- rollback は隠れた control-flow trick ではなく、明示的な failure / effect semantics の一部である。

### 7. `atomic_cut`

- Mir-0 は、最小の place-local finalizing cut primitive として `atomic_cut` を含む。
- Mir-0 において `atomic_cut` が確定するのは current `place` の rollback frontier だけである。
- `atomic_cut` は、単一 process 全体の同期点、複数 `place` をまたぐ合意点、永続化完了点を意味しない。
- `place` の内部では、`atomic_cut` は rollback が通過できない明示的 decision boundary を作る。
- `atomic_cut` の後で failure が表面化しても、同じ `place` の pre-cut 部分は rollback されない。以後は compensation、fallback、または明示的 failure として扱う。
- Mir-0 は `atomic_cut` の rollback / finalization 上の役割を固定するが、それだけで persistence や distributed finalization の semantics を決めることはしない。
- 後続計算が pre-cut effect を考慮する必要があるなら、implicit rollback across the cut ではなく、明示的 continuation または compensation を通じて扱わなければならない。

### 8. Linear resource と monotone lifetime

- Mir-0 は、rollback、fallback、後続の進化 machinery による duplication を防ぐために必要な最小 linear-resource discipline を含む。
- ownership transfer は明示的でなければならない。
- lifetime degradation は monotone である。
- 後続の convenience layer は、これらの規則を弱めるのではなく保持しなければならない。

## Mir-0 の外に意図的に置く論点

- `durable_cut` は Mir-0 に含めない。Mir-1 側では、`atomic_cut` に abstract persistence requirement を伴う durable commit guarantee を追加する cut vocabulary 候補として扱う。
- `durable_cut` の最小意味は、successful として観測された pre-cut prefix が local rollback、process restart、route rebinding の後に未確定状態へ戻らないことである。Mir-1 はこの guarantee を意味づけるが、具体的な storage / replication / consensus mechanism は固定しない。
- `durable_cut` の failure は、durable guarantee を確立できず、その cut attempt を成功した cut として確定できなかったことを意味する。この段階では既存の failure lattice を再利用し、新しい failure class は導入しない。
- 既定では、durable guarantee を確立できなかった `durable_cut` は `Reject` として扱う。cut 後の外部化された obligations を明示的に巻き戻す必要がある場合だけ `Compensate` を使う。`Approximate` は contract が durability を弱めた代替を明示的に許す場合を除き、`durable_cut` failure の既定分類にはしない。
- local durable failure と distributed durable failure は、Mir-1 では別 failure class に分けない。どちらも同じ durable guarantee failure の実現上の差分であり、後者の distributed finalization は Mirrorea 側の実現責務に属する。
- distributed finalization は `durable_cut` の最小意味そのものではない。複数 `place` や実ノードをまたぐ `durable_cut` を成立させるときの operational realization 側に属する。
- Mirrorea は `durable_cut` を意味づける場所ではない。Mir-1 で定められた durable commit guarantee を、storage / replication / distributed cut realization / audit 上で実現する責務を持つ。
- `barrier` は Mir-0 の cut vocabulary に含めない。Mir-1 に残すとすれば、commit-like primitive ではなく ordering primitive 候補として扱う。
- **未決定**: `barrier` が Mir-1 で独立語彙として本当に必要か、それとも他の ordering / cut 構成に吸収されるか。
- **未決定**: `durable_cut` の persistence evidence をどの形式で観測・検証するか。
- **未決定**: contract が durability を弱めた代替結果を本当に許す場合、その degraded path を `Approximate` としてどこまで許容するか。
- **UNRESOLVED**: fallback の完全 normalization と、preference chain の完全代数。
- **UNRESOLVED**: `emit`、effect handler、structured event routing の正確な関係。
- **UNRESOLVED**: suspension restriction と cut / rollback / patching との相互作用を含む coroutine support。
- **UNRESOLVED**: overlay alias detail、route rebinding detail、およびその他の Mirrorea 依存の safe-evolution mechanism。

これらはより広い Mir の設計空間には属しているが、この段階で固定する最小 kernel には含めない。

## Mir が何ではないか

- それは deployment fabric と同一ではない。
- それは route / overlay の制御プレーン（control plane）と同一ではない。
- それは domain-specific media runtime ではない。
- それは virtual reality engine ではない。
- それは type system だけではなく、operational boundary と evolution rule も含む。
