# Phase 0: repository memory と decision boundary

## この phase の意味

Phase 0 は、この repo が **仕様・経緯・証跡をどこに置くか** を固定する層です。
agent が毎回ゼロから始めても current state を読み違えないように、文書構造そのものを correctness の一部として扱います。

## 2026-04-23 時点で固まっていること

- 規範判断の正本は `specs/`
- 長期の repository memory は `plan/`
- current snapshot は `Documentation.md` / `progress.md` / `tasks.md`
- 詳細な実行証跡は `docs/reports/`
- active sample line は `samples/clean-near-end/`
- old sample line は archive に分離済み

## この phase が current layer にどう効いているか

clean near-end alpha floor は、実装だけではなく文書の読み筋まで含めて再現可能である必要があります。
そのため Phase 0 では、

- active path
- archive path
- normative / memory / snapshot / evidence の分担

を崩さないことが実装の前提になります。

## まだ残ること

- public release 向けの external doc set をどう切るか
- final public API 文書の粒度をどこまで repo 内で持つか
- packaging / installed surface を docs にどう昇格させるか

## まず参照する文書

- `README.md`
- `Documentation.md`
- `progress.md`
- `tasks.md`
- `specs/00-document-map.md`
