---
id: scenarios/readme
status: L0-frozen
maturity: draft
depends_on: [spec/06-conformance, theory/03-elaboration]
summary: 10 本の凍結正準シナリオと M3 pressure scenario。理論ドリフト防止装置であり適合性の定義。変更には ADR が要る。
open_items: []
---

# scenarios/ — 正準シナリオと pressure scenario

SCN-01..10 は「本来上に載せたかったもの」の凍結標本である。**理論・仕様のどの改訂も、10 本すべてを説明できなければ却下される**(変更それ自体に ADR が必要)。SCN-11 以降は owner-approved milestone が追加する pressure scenario であり、凍結十本を遡及変更しない。各ファイルは: 目的 / Surface 又は Core-level source / 期待される elaboration・実行 / 否定変種と期待診断 / 参照 ID を持つ。語彙(World, Player 等)は S5 のドメイン語彙である(ADR-0001)。

| ID | 題材 | 主に縛る理論 |
|---|---|---|
| SCN-01 | 双六の roll(owner 宛 write + publish) | 01, 03 |
| SCN-02 | attack(owner-evaluated RMW + failure row) | 03 (THM-001) |
| SCN-03 | 後から join(admission・epoch・過去としての履歴) | 05 |
| SCN-04 | owner の退出(incarnation 引退・stale key) | 05, 04 |
| SCN-05 | portal(世界間リンクと可視性) | 03, 07 |
| SCN-06 | two-shard(硬い境界と RouteUnavailable) | 03, 04 |
| SCN-07 | gradient 観測(可視性水準・private 遮断) | 07 |
| SCN-08 | avatar fallback(lease 失効・単調劣化・再取得) | 06 (THM-002) |
| SCN-09 | patch 受理と拒否(self-grant 無変異) | 08 (THM-006) |
| SCN-10 | save/load の stale 拒否 | 04 (THM-003) |
| SCN-11 | designated evaluator / versioned value | 13 |
| SCN-12 | maintained relation / late projection | 14, 06, 07, 09 |
