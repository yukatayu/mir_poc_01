# Mir / Mirrorea / PrismCascade / Typed-Effect Wiring Platform — 初期ワークスペース

このリポジトリは、大規模な研究開発プログラムのための**仕様先行（specification-first）の初期ワークスペース**である。
保持された文脈がない agent でも、少数の文書を順番に読めばプロジェクト構造を復元できるよう、意図的に構成されている。

## このワークスペースの目的

このリポジトリは、密接に関係しつつも意図的に分離されている四つのシステムの作業開始点である。

1. **Mir** — 因果、contract、effect、ownership、lifetime、安全な進化を扱う言語・意味論コア。
2. **Mirrorea** — 論理名、compatibility-preserving overlay insertion、patching、routing、audit、動的再構成のための fabric / runtime 層。
3. **PrismCascade** — 動画・音声編集と live pipeline のための独立した media-processing kernel。
4. **Typed-Effect Wiring Platform** — container/service と legacy system integration のための、routable で inspectable な effect 層。

## 現在の段階

このリポジトリは**実装完了済み**でも**MVP コードベース**でもない。
現在は、次の作業を進めるための構造化された出発点である。

- 仕様の精密化
- 不変条件の証明または反証
- 小規模な proof of concept の構築
- 各タスク後の詳細レポート作成

## 人間と agent の必読順

1. `AGENTS.md`
2. `Documentation.md`
3. `specs/00-document-map.md`
4. `specs/01-charter-and-decision-levels.md`
5. `specs/02-system-overview.md`
6. `specs/03-layer-model.md`
7. `specs/09-invariants-and-constraints.md`
8. その後、必要な subsystem spec に入る
   - `specs/04-mir-core.md`
   - `specs/05-mirrorea-fabric.md`
   - `specs/06-prismcascade-positioning.md`
   - `specs/07-typed-effects-wiring-platform.md`
   - `specs/08-cross-system-relations.md`
9. 未解決事項は `specs/10-open-questions.md`
10. 実装や研究の順序は `specs/11-roadmap-and-workstreams.md`
11. 既存判断は `specs/12-decision-register.md`

文書マップ、用語方針、相互参照の基準は `specs/00-document-map.md` を参照すること。
current status / progress / remaining steps を見たいときは `progress.md`、repo 全体の研究 phase / autonomy gate を見たいときは `plan/17-research-phases-and-autonomy-gates.md` を読むこと。

## 作業スタイル

すべての non-trivial task は、`docs/reports/` 配下に**新しい markdown report** を生成すること。
report は `python scripts/new_report.py --slug <short-name>` でテンプレートから作成する。

## リポジトリ構成

- `specs/` — 正本となる仕様書と設計文書
- `docs/reports/` — 時系列の作業レポート
- `docs/diagrams/` — Mermaid ソース図
- `agents/` — プロジェクトローカルの agent 向けガイダンス / 設定ファイル
- `scripts/` — レポート作成と検証の補助スクリプト
- `crates/` — 意図的に最小構成に留めた Rust workspace skeleton

## 実装言語選択の現状

現時点の推奨は次のとおりである。

- core implementation と runtime kernel には **Rust**
- 必要になった場合の可視化には **Web-based visualization**（例: TypeScript / HTML / SVG / WebGL）
- 将来の game engine integration には **C ABI / engine adapters**

これは**推奨される実装方針**であり、アーキテクチャ上の法ではない。
実装ガイダンスは `specs/11-roadmap-and-workstreams.md` を参照すること。

## 現在の環境メモ

初期 scaffold 自体は Python のみが使える環境で起こされたが、current repo では Python と `cargo` の両方を使った local validation が運用に入っている。
したがって、current state の build / test / helper verification は report に記録された実行 evidence を正本として読むこと。
