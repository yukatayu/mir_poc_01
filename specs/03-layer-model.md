# 03 — レイヤーモデル

## レイヤー

### L0 — 既存の operating-system / network substrate

例: process、file system、socket、kernel interface、device driver、graphics API、service orchestrator。
この層は前提とするのであって、ここで再設計しない。

### L1 — Mir Core

責務:

- 因果と directed-acyclic-graph execution model
- effects と contracts
- cut / boundary / rollback constraint
- ownership / lifetime / monotone degradation
- 安全な進化の primitive

### L2 — Mirrorea Fabric

責務:

- logical naming
- routing と route rebinding
- overlay insertion
- patch application
- distributed cut の実現
- audit と path proof
- non-Mir system の safe integration

### L3 — 共有空間 / 共有状態

責務:

- session / space abstraction
- shared object の同期モデル
- consistency mode の選択
- space 間の movement または linkage

### L4 — ドメインエンジン / フレームワーク

責務:

- PrismCascade のような domain-specific kernel
- virtual reality world engine
- collaborative document engine
- 将来の GUI プログラミング基盤

### L5 — アプリケーション / コミュニティ / Reversed Library

責務:

- 実際の user-facing space
- 具体的な virtual social system
- web synchronization experience
- curriculum / progression system
- application-level architecture としての Reversed Library

## 横断的関心事: 可観測性（observability）

可観測性（observability）は全 layer を横断するが、概念上の主たる足場は次であるべきである。

- 規範的 event semantics を扱う L1
- routing / audit を扱う L2
- domain-specific trace（例: media plan）を扱う L4
- 必要に応じて user-visible diagnostics を扱う L5

## 横断的関心事: 置換可能性（replaceability）

置換可能性（replaceability）は、境界が明示的なときに最も強くなる。

- L1 の language semantics boundary
- L2 の routing / overlay boundary
- L4 の domain-kernel boundary
- L5 の user experience boundary
