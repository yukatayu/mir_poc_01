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

- **2026-07-24** `PROPOSAL-009` の owner disposition を記録。将来の
  OBL-001 proof-facing package は THM-001 の既存 every-write Core `c` 条件を
  直接表明できる。この記録は再triage を許すだけで、新規 WRK、Core
  representation / traversal、OBL status、proof、Gate / Phase を選ばない。
- **2026-07-21** PROPOSAL-007 により、ADR-0014 の既存 L3 retained-evidence
  condition を WRK の append-only `Evidence commits:` と reachable-DAG audit
  で精密化した。これは L2 activation、reserved boundary、L0/L1 theory を変更せず、
  既存 LAB lane の証拠帰属を機械検査可能にする運用改定である。
- **2026-07-21** ADR-0014 を PROPOSAL-006 により改訂。owner-maintained exact
  editable-target table を standing bounded autonomy に置換し、agent-maintained
  canon surface を `working/` に限定した。L3 pre-registration、existing-lane
  evidence、L2 の author/reviewer distinct signed frozen-material review、forward rollback を要する。L0/L1、contracts、
  SCN/Gate/Phase、`theory/11`、final proof、public claim は引き続き留保する。
- **2026-07-21 (superseded operating detail)** ADR-0014 により、existing LAB lane における可逆な L2/L3
  working-theory research を委任した。canon update は owner-maintained exact
  editable-target row、rebased frozen evidence/diff、independent review、reviewed
  rollback を要する。L0/L1、external contract、SCN/Gate/Phase、全 `theory/11`、
  implementation/public status は委任していない。初期 editable-target row は空。
- **2026-07-15** ADR-0013 により `phase-governance/t0-g0` version 1 と、
  pinned evidence cut に対する G0-D1 acceptance / G0-D4 waiver を採択した。
  one-off LAB-derived JSON は許可したが、G0-D3 は defer のため G0 exit / T1
  entry / SCN conformance / implementation state は変更していない。
- **2026-07-14** `PROPOSAL-001` の owner disposition を記録。abstract
  OBL-020 Lean statement shape は、full OBL-020 completion を伴わない
  G1-supporting proposal-preparation scope としてのみ受理された。OBL status、
  proof、artifact identity / wrapper、Gate / Phase は変更していない。
- **v0.1.0** (2026-07-02) 初回生成。ADR-0001〜0012 制定。MirCore v0 初稿、Surface 文法 v0、SCN-01〜10 凍結、Gate/Phase 計画制定。全証明は OBL 台帳(未 discharge)。既存 repo mir_poc_01 を LAB に格下げ(手続きは meta/source-hierarchy.md)。
