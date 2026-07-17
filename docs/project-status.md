# Project status

最終更新: 2026-07-17 09:51 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは人間が短時間で全体像を読むための **LAB派生ビュー** です。規範判断、
Gate / Phase 移行、OBL 状態、適合性、実装完了を作りません。

- 規範正本: `mirrorea_canon/`
- 詳細な計画・研究の記憶: `plan/`
- 実行証拠: `docs/reports/`
- runnable LAB の一覧: `samples_progress.md`

## 全体の進行チェックリスト

Gate: [ ] G0 軸と語彙 -> [ ] G1 普通の代入 -> [ ] G2 存在と fallback ->
[ ] G3 権限 -> [ ] G4 効果と観測 -> [ ] G5 cut と保存 -> [ ] G6 射影 ->
[ ] G7 hot-plug

Phase: [ ] T0 語彙と決定 -> [ ] T1 計算体系 -> [ ] T2 骨格証明 ->
[ ] I1 参照実装 -> [ ] I2 multi-locus -> [ ] I3 実 transport ->
[ ] I4 永続と patch -> [ ] I5 射影と View -> [ ] I6 分散永続と連合

チェックは引用可能な canon record が exit を成立させたときだけ埋めます。
基準は `mirrorea_canon/plan/00-gates.md` と
`mirrorea_canon/plan/01-phases.md` を読むものとし、ここでは再定義しません。

## 現在地

| 面 | 現在の読み | 根拠 |
| --- | --- | --- |
| Canon lifecycle | `T0/G0 rebaseline`。G0 exit と T1 entry は未成立。 | `mirrorea_canon/plan/01-phases.md` |
| T0-T2 research | owner 指示により、既存根拠へ接続した非休眠の LAB research work unit を `research-complete` / `decision-ready` まで自走できる。これは canon package close ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| Runnable LAB | alpha / Surface / operational sample は再現可能な限定証拠。ただし canon 実装状態・適合性・proof ではない。 | `samples_progress.md` |
| いまの研究 | OBL-001/020/021 の反例監査、`[E-WRITE]` store-key、`[E-OBS]` append kernel を `research-complete` とした。後者は incoming-only graph extension の条件付き結果であり、canon の append 定義ではない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
| 最新 preflight | literal-RHS foreign-locus write の source 正負 pair は再現できたが、authority/capability/witness carrier を OBL-001 の抽象 predicate へ結ぶ既存 lane はない。`T-RESEARCH-004` は未選定。 | `plan/156-t0-t2-research-autonomy-envelope.md` |

## 現在の停止線

- phase-governance/t0-g0 の `pass` は T0 profile evidence だけであり、G0
  exit、T1 entry、SCN conformance、OBL completion、proof、runtime/product
  readiness を作らない。 | `mirrorea_canon/plan/01-phases.md` |
- canon package の close に owner acceptance、ADR/canon/SCN の変更、
  theory/11 の status 更新、Gate / Phase acceptance、L0/L1 choice が必要なら、
  agent は `decision-ready` で止まる。 | `mirrorea_canon/plan/02-operating-model.md` |
- 新しい helper / evidence lane / schema / CI / Make target / main merge /
  conformance claim は T1 exit 前の研究範囲外。 | `plan/156-t0-t2-research-autonomy-envelope.md` |
- OBL-001 の concrete-evidence bridge を作るには owner の明示 promotion が
  必要であり、agent は JSON field、Lean interpretation、wrapper を自走で追加
  しない。 | `plan/156-t0-t2-research-autonomy-envelope.md` |

## オーナーの確認・判断待ち

| ID | 状態 | いま必要なこと |
| --- | --- | --- |
| G0-D1 / D2 / D4 | recorded | ADR-0013 に記録済み。再判断待ちではない。 |
| G0-D3 | DEFERRED (dormant) | owner が明示的に reopen するまで研究単位の選定対象にしない。 |
| T0-T2 research autonomy | recorded LAB operating authorization | `plan/156-t0-t2-research-autonomy-envelope.md` の選定規則と停止条件の範囲で agent が調査を継続する。 |
| OBL-001 concrete-evidence bridge | decision-ready | existing elaborator output と abstract OBL-001 predicate の bridge を将来作るか。current LAB recommendation は proof-facing need まで defer。 |

`G0-D3` の defer は canon lifecycle を変えず、LAB 上の選定ガードとしてだけ
扱います。 | `plan/155-t0-g0-governance-profile-proposal.md`

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 目的・体系の地図 | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `docs/diagrams/layer-stack.mmd` |
| Gate / Phase と実用化の順序 | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md`, `docs/diagrams/workflow.mmd` |
| エージェントと owner の境界 | `mirrorea_canon/plan/02-operating-model.md`, `plan/156-t0-t2-research-autonomy-envelope.md` |
| proof の唯一の状態台帳 | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable LAB の範囲 | `samples_progress.md` |
| 詳細な現況と次の作業 | `progress.md`, `tasks.md`, `plan/156-t0-t2-research-autonomy-envelope.md` |

## 更新規約

まず canon または bounded LAB evidence を更新し、次に `progress.md`、
`tasks.md`、`samples_progress.md` を必要な範囲で同期し、この派生ビューを最後に
更新します。根拠が未解決なら `STALE - source reconciliation required` と明記し、
推測で current state を置き換えません。詳細な履歴は `plan/` と
`docs/reports/` に残します。
