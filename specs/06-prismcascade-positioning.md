# 06 — PrismCascade の位置づけ

## この文書の目的

この文書は、PrismCascade が Mir と Mirrorea に対してどこに位置づくかを説明する。
これは PrismCascade の完全仕様ではなく、アーキテクチャ上の位置づけ要約である。

## 現在の結論

PrismCascade は、Mir の sub-runtime ではなく、**独立した kernel** として開発すべきである。

## 理由

### PrismCascade は最適化中心が異なる

その中核的な関心事には次が含まれる。

- effect-only graph の正規化
- 実行前 planning
- 明示的な memory ownership と再利用
- CPU / GPU / transfer scheduling
- offline / live の因果区別

これらの関心事は Mir の主要関心事と十分に異なるため、runtime を早期に統合すると、両方の設計を歪める可能性が高い。

### それでも PrismCascade は Mir と強い統合点を持つ

良い統合点には次が含まれる。

- Meta-layer effect provider（例えば TTS、model inference、asset lookup）
- remote execution または remote resource delegation
- tracing と audit のための共有 identifier
- Prism graph 周辺での collaborative editing と synchronization

## 強い推奨

- PrismCascade は独立に開発する。
- Mir / Mirrorea との統合点は狭く明示的に定義する。
- Prism runtime の内部を Mir の一般 runtime model に押し込まない。

## 開かれた統合上の問い

- Prism と Mir の間でどの effect contract を共有すべきか。
- 最小の共有 trace schema は何か。
- Prism にとって最も小さく妥当な remote execution unit は何か。
