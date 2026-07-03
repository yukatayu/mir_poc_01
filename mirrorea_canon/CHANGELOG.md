---
id: root/changelog
status: L1-fixed
maturity: draft
depends_on: [meta/style-guide]
summary: canon 自体の版履歴と改定手続きの要約。
open_items: []
---

# Changelog

## 改定手続き(要約)

1. 提案(人間または AI)を `meta/proposals/PROPOSAL-###.md` として起票。
2. 人間(プロジェクトオーナー)が採否を決定。
3. 採択なら該当ファイルを改定し、L0/L1 に触れる場合は ADR を追記・改訂し、本ファイルに 1 行記録。
4. `python3 meta/build-index.py` で `INDEX.json` を再生成し、参照整合を検証。

## 履歴

- **v0.1.0** (2026-07-02) 初回生成。ADR-0001〜0012 制定。MirCore v0 初稿、Surface 文法 v0、SCN-01〜10 凍結、Gate/Phase 計画制定。全証明は OBL 台帳(未 discharge)。既存 repo mir_poc_01 を LAB に格下げ(手続きは meta/source-hierarchy.md)。
