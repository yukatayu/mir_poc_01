# 00 — 文書マップ

この文書は、主要な関心事がそれぞれどこに書かれているかを読者または agent に示す。

## 規範的な読書順

1. `01-charter-and-decision-levels.md`
2. `02-system-overview.md`
3. `03-layer-model.md`
4. `09-invariants-and-constraints.md`
5. 1 つ以上の subsystem 文書
   - `04-mir-core.md`
   - `05-mirrorea-fabric.md`
   - `06-prismcascade-positioning.md`
   - `07-typed-effects-wiring-platform.md`
   - `08-cross-system-relations.md`
6. `10-open-questions.md`
7. `11-roadmap-and-workstreams.md`
8. `12-decision-register.md`

## 各文書の役割

- `01-charter-and-decision-levels.md`
  - プロジェクトの意図、スコープ、decision level system。
- `02-system-overview.md`
  - スタック全体の高水準な概要。
- `03-layer-model.md`
  - 各 layer の正確な責務と境界。
- `04-mir-core.md`
  - Mir の言語コア、四本柱、意味論中心の要約。
- `05-mirrorea-fabric.md`
  - naming、routing、overlay、patching、audit、distributed control。
- `06-prismcascade-positioning.md`
  - なぜ PrismCascade が分離されるのか、その統合面は何か。
- `07-typed-effects-wiring-platform.md`
  - Typed-effect routing / container visibility platform とその役割。
- `08-cross-system-relations.md`
  - 共有前提、境界、相互運用点。
- `09-invariants-and-constraints.md`
  - 黙って破ってはならない最重要ルール。
- `10-open-questions.md`
  - 明示的な未解決事項。
- `11-roadmap-and-workstreams.md`
  - 提案されている実装 / 研究の順序。
- `12-decision-register.md`
  - 現在の主要判断と、その強度レベル。

## レポート

- `docs/reports/` には時系列の作業ログが入る。
- report 自体は規範文書ではないが、なぜ変更が行われたかを説明する。

## 用語と参照の編集方針

- 日本語正本として統一する語:
  - `safe evolution` は「安全な進化」と書く。
  - `shared space / shared state` は「共有空間 / 共有状態」と書く。
  - `control plane` は各文書の初出で「制御プレーン（control plane）」と併記してよい。
- formal token / 原語保持を優先する語:
  - `downstream addition`
    - 文脈により `downstream-only patching` や `leaf-style patching` といった局所表現を使う場合があるが、既定の進化方向を指す語幹として扱う。
  - `compatibility-preserving overlay`
    - 日本語 prose では「互換性を保つ overlay」と説明してよいが、formal token としてはこの形を優先する。
  - `wrap`
    - 日本語 prose では「包む」と書いてよいが、legacy integration や boundary operation を指すときは `wrap` を残してよい。
  - `fabric`
  - `kernel`
  - `runtime`
- 入口文書は、完全な文書マップと用語方針については `00-document-map.md` を、確定済み判断については `12-decision-register.md` を参照先として明示する。
