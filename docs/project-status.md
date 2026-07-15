# Project status

最終更新: 2026-07-15 17:02 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

この文書は、人間が現在地、停止線、判断待ち、根拠を短時間で確認するための
LAB 派生ビューです。決定、Gate / Phase 移行、OBL 状態、適合性、実装完了を
作りません。規範判断が衝突する場合は `mirrorea_canon/` が常に優先し、LAB の
引用先は対応する事実または作業証拠としてのみ参照します。

## 全体の進行チェックリスト

Gate: [ ] G0 軸と語彙 -> [ ] G1 普通の代入 -> [ ] G2 存在と fallback ->
[ ] G3 権限 -> [ ] G4 効果と観測 -> [ ] G5 cut と保存 -> [ ] G6 射影 ->
[ ] G7 hot-plug

Phase: [ ] T0 語彙と決定 -> [ ] T1 計算体系 -> [ ] T2 骨格証明 ->
[ ] I1 参照実装 -> [ ] I2 multi-locus -> [ ] I3 実 transport ->
[ ] I4 永続と patch -> [ ] I5 射影と View -> [ ] I6 分散永続と連合

チェックは canon の exit が引用可能な記録で成立した時だけ埋めます。詳細は
`mirrorea_canon/plan/00-gates.md` と `mirrorea_canon/plan/01-phases.md` を読む
ものとし、この文書で exit criteria を再定義しません。

## 現在地

| 観点 | 現在の読み | 根拠 |
| --- | --- | --- |
| Canon lifecycle | `T0/G0 rebaseline`。G0 exit と T1 entry は未成立。 | `mirrorea_canon/plan/01-phases.md` |
| LAB management | 人間向けには 9 段階中 1 段階目、`late pre-exit`。これは canon の部分状態ではない。 | `plan/149-current-phase-position-reading.md` |
| 実行証拠 | LAB には runnable alpha / sample evidence があるが、canon 上の実装状態や適合性を意味しない。 | `samples_progress.md` |
| 現在の境界 | P111 は T0 governance profile の提案までを閉じた。G0 exit と T1 entry は依然未成立。 | `plan/155-t0-g0-governance-profile-proposal.md` |

## 現在の停止線

- `mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md` は
  profile の提案であり、adoption、JSON result、G0 exit を作らない。
- G0 exit、T1 entry、OBL completion、proof、`mir-conform` 結果、runtime /
  product readiness をこの文書、proposal、又は LAB evidence だけで主張しない。
- 次の自律 successor package を勝手に昇格しない。
- 再開候補は、G0-D1 の明示回答、profile proposal の owner/canon 採否、G0-D3、
  または引用済み根拠の具体的な drift に限る。

## オーナーの確認・判断待ち

| ID | 必要な確認・判断 | 決定者 | 詳細 |
| --- | --- | --- | --- |
| G0-D1 | 五つの ADR、GLOSSARY、LAB-demotion evidence を G0 factual criteria として exact に accept / defer する。 | owner / canon process | `plan/153-g0-closeout-evidence-and-exit-decision-packet.md` |
| G0-D2 | T0-specific governance JSON profile を提案するという mechanism choice は owner 入力として記録済み。exact profile の canon adoption は未決。 | owner / canon process | `mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md` |
| G0-D3 | G0 exit を approve / defer し、有効な canonical ADR / ledger record を指定する。 | owner / canon process | `plan/153-g0-closeout-evidence-and-exit-decision-packet.md` |
| G0-D4 | 追加 semantic / historical LAB-demotion audit はこの checkpoint では要求しないと owner が記録。 | owner | `mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md` |

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 権限階層 | `mirrorea_canon/README.md`, `mirrorea_canon/meta/source-hierarchy.md`, `mirrorea_canon/plan/02-operating-model.md` |
| Gate / Phase の全体 | `mirrorea_canon/MAP.md`, `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| 現在地の短い読み | `plan/149-current-phase-position-reading.md`, `progress.md`, `tasks.md` |
| 現在の停止線と判断票 | `plan/153-g0-closeout-evidence-and-exit-decision-packet.md` |
| P111 profile proposal | `plan/155-t0-g0-governance-profile-proposal.md`, `mirrorea_canon/meta/proposals/PROPOSAL-002-t0-g0-governance-profile.md` |
| P109 の監査証跡 | `docs/reports/2247-p109-g0-closeout-evidence-and-decision-packet.md` |
| runnable sample の状態 | `samples_progress.md` |
| 詳細な更新・停止・検証規約 | `plan/154-project-control-cockpit.md` |

## 更新規約

権限を持つ source または bounded LAB evidence を先に更新します。次に
`progress.md`、`tasks.md`、`samples_progress.md` を必要な範囲で同期し、この
文書を可変の status view の最後に更新します。validator 実行後に新しい
immutable report を確定します。canon lifecycle、promoted package、stop packet、
owner decision、runnable classification、根拠 path が変わる package では必ず
見直します。根拠を解決できない場合は `STALE - source reconciliation required`
と明記し、推測で current state を置き換えません。
