# Project status

最終更新: 2026-07-21 20:25 JST

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

チェックは引用可能な canon record が exit を成立させたときだけ埋めます。基準は
`mirrorea_canon/plan/00-gates.md` と `mirrorea_canon/plan/01-phases.md` です。

## 現在地

| 面 | 現在の読み | 根拠 |
| --- | --- | --- |
| Canon lifecycle | `T0/G0 rebaseline`。G0 exit と T1 entry は未成立。 | `mirrorea_canon/plan/01-phases.md` |
| 論理仕様 | L0/L1 の軸は canon に固定。ADR-0014 は standing boundary 内の existing LAB research と committed `WRK-####` L3 record を許す。L2 は owner-authenticated trust anchor が未構成のため fail-closed で、falsifier は既存 L2 を即時 `frozen` にする将来の安全弁として定義される。 | `mirrorea_canon/adr/ADR-0014.md`; `mirrorea_canon/working/README.md`; `mirrorea_canon/plan/02-operating-model.md` |
| ユーザ向け仕様 | companion notation と runnable LAB examples はあるが、Surface v0 grammar closure と public contract は未完。 | `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`; `samples_progress.md` |
| 実装 / 運用 | alpha / Surface / operational roots は限定 LAB evidence。conformance、実 transport、分散永続、public product は未主張。 | `samples_progress.md`; `mirrorea_canon/plan/01-phases.md` |
| 研究 lifecycle | T-RESEARCH-001..033 は pre-delegation evidence。WRK-0001 は theory/02 の lifetime/capture 有限断片と `Nat` budget parameter を existing Lean fragment で L3 として再現し、証跡 manifest・clean-worktree validation・cross-cut review まで完了した。WRK-0002 は OBL-021 LAB statement draft の projection-vacuity countermodel を L3 evidence として manifest した。WRK-0003 は全 projection の total/unique witness と equality comparison を与えても Result identity が導けない countermodel を L3 evidence として manifest し、全 premise を束ねる theorem へ追加 evidence で訂正した。次は outcome の存在性を検査する。いずれも `not-promoted` で、L2 selection は将来の owner trust configuration まで fail-closed である。 | `mirrorea_canon/working/WRK-0001-finite-index-boundaries.md`; `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md`; `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`; `plan/wrk-0001-finite-index-reproduction.md`; `plan/wrk-0001-pilot-checkpoint.md`; `plan/wrk-0002-projection-vacuity-countermodel.md`; `plan/wrk-0003-projection-extensionality-countermodel.md`; `plan/156-t0-t2-research-autonomy-envelope.md`; `plan/158-standing-bounded-autonomy.md`; `plan/159-wrk-evidence-commit-integrity-recut.md` |
| システム配置 | Mir Surface/Core/Trace/Verify、Mirrorea fabric/projection、typed provider/View、domain application は分離して扱う。 | `docs/diagrams/layer-stack.mmd`; `mirrorea_canon/MAP.md` |

## 現在の停止線

- L0/L1、core primitive、external contract、SCN expectation、conformance、Gate /
  Phase、すべての `mirrorea_canon/theory/11-metatheory-ledger.md` change、public
  completion は owner/canon action が必要です。`mirrorea_canon/adr/ADR-0014.md`
- 新 evidence lane、helper、schema、CI、Make target、production implementation は
  T1 exit 前の moratorium に残ります。`mirrorea_canon/plan/02-operating-model.md`
- working annex の `WRK-####` 外の既存 canon text は read-only です。standing predicate を
  満たす candidate は L3 record と manifest 済み LAB evidence を自律的に進められ、L2 promotion には
  将来の rebased final cut review が必要です。現行は owner-authenticated trust anchor
  未構成のため fail-closed です。frozen L2 は successor で forward に扱います。
  `plan/158-standing-bounded-autonomy.md`
- `PROPOSAL-003` と `PROPOSAL-004` は L1-reserved question なので delegated route
  では採択しません。`mirrorea_canon/meta/proposals/PROPOSAL-003-obl020-formalization-boundary-review.md`; `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`
- 運用 storage は root filesystem のみで、2026-07-21 17:49 JST 時点の空きは約
  12 GiB です。承認済み cleanup で再生成可能な local build output と Mirrorea 一時成果物を
  除去しました。configured external workdir は依然未マウントなので、次の heavy build / generated
  artifact / toolchain work の前にも容量を再確認し、外部 workdir を優先します。

## オーナーの確認・判断待ち

| ID | 状態 | いま必要なこと |
| --- | --- | --- |
| G0-D3 | DEFERRED (dormant) | owner が明示的に reopen するまで選定しない。 |
| OBL-001 concrete-evidence bridge | owner record pending | defer、または artifact-free design comparison だけを許可するか。 |
| PROPOSAL-003 | owner record pending | OBL-020 organization の A / B / C。 |
| PROPOSAL-004 | owner record pending | Surface v0 grammar closure の A / B / C。LAB recommendation は A。 |

この表以外の non-reserved candidate は ADR-0014 route により LAB research で選別
できます。routine target approval は不要です。canon current state は
working annex の `WRK-####` に限定され、L2 selection は将来の trust anchor 構成まで
fail-closed です。
`tasks.md`; `plan/158-standing-bounded-autonomy.md`

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 目的・体系の地図 | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md`, `docs/diagrams/layer-stack.mmd` |
| Gate / Phase と実用化の順序 | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md`, `docs/diagrams/workflow.mmd` |
| delegated research の境界 | `mirrorea_canon/adr/ADR-0014.md`, `mirrorea_canon/working/README.md`, `mirrorea_canon/plan/02-operating-model.md`, `plan/158-standing-bounded-autonomy.md`, `plan/159-wrk-evidence-commit-integrity-recut.md` |
| storage / heavy work guard | `docs/reports/2295-approved-artifact-cleanup.md`; `AGENTS.md` |
| proof の唯一の状態台帳 | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| runnable LAB の範囲 | `samples_progress.md` |
| 現在の次作業と選択肢 | `progress.md`, `tasks.md`, `plan/158-standing-bounded-autonomy.md` |

## 更新規約

まず authority-bearing canon source または bounded LAB evidence を更新し、次に
`progress.md`、`tasks.md`、`samples_progress.md` を必要な範囲で同期し、この派生
ビューを最後に更新します。根拠が未解決なら `STALE - source reconciliation required`
と明記し、推測で current state を置き換えません。詳細な履歴は `plan/` と
`docs/reports/` に残します。
