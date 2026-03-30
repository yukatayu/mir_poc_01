# 08 — システム横断関係

## 要約

このプロジェクトには現在、少なくとも三つの主要な概念システムがある。

1. Mir / Mirrorea
2. PrismCascade
3. Typed-Effects Wiring Platform

これらは関連していますが、未分化な 1 つの runtime に潰してはなりません。

## 有用な関心事の分割

### Mir / Mirrorea

- 分散計算の意味論核
- contract、ownership、lifetime、cut、overlay、安全な進化
- 論理 routing と audit

### PrismCascade

- 時系列メディア graph カーネル
- Meta / Core / Runtime 分離
- scheduling と memory planning
- live / offline distinction

### Typed-Effects Wiring Platform

- typed effect boundary representation
- rewritable route
- observability と state↔event graphing
- legacy integration

## 共有基盤

三つの project はいずれも次を重視する。

- 明示的 effect
- dependency と causality の可視性
- replaceability と compatibility
- 規範的 semantics と implementation choice を分離する必要

## 推奨される関係

- 実装では分離したまま保つ。
- 共有は本当に必要なものに限る:
  - identifier
  - 必要に応じた contract syntax または schema
  - trace linking strategy
  - 互換性がある範囲での effect vocabulary
