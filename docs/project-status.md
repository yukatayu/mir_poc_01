# Project status

最終更新: 2026-08-03 19:44 JST

**Canon notice:** `mirrorea_canon/` is the normative source for direction,
theory, ADRs, conformance, and process. This document is a LAB derived view.

## この文書の役割

これは人間向けの短い **派生ビュー** である。規範判断は
`mirrorea_canon/`、唯一の current execution roadmap は
`plan/247-mir-theory-v0-i1plus-current-roadmap.md`、詳細な履歴は他の `plan/` と
`docs/reports/` にある。この文書は Gate/Phase、OBL、SCN、適合性、実装完了を決めない。

## 全体の進行チェックリスト

```text
M0 bootstrap → M1 Constitution → M2 T0/G0 semantic assertions
→ M3 evaluation/materialization → M4 maintained relation/projection
→ M5 shared model → M6 Surface → M7 checker/elaborator
→ M8 deterministic runtime → M9 auth/verification → M10 closeout
```

各矢印は current milestone の rule/non-effect、positive/negative evidence、formal
classification、independent review、validation、commit/push/parity を閉じた後にだけ進む。

## 現在地

| 観点 | 状態 | 根拠 |
| --- | --- | --- |
| active frontier | **M1 Constitution**。次は M2 semantic-assertion T0/G0 closeout | `plan/247-mir-theory-v0-i1plus-current-roadmap.md` |
| authority | ADR-0015 の owner-approved bounded program。owner-reserved condition 以外は milestone evidence で自走する | `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | `T0`。v2 artifact は valid `fail`、G0-D3/G0 exit/T1 entry は未成立 | `mirrorea_canon/plan/01-phases.md` |
| proof / scenarios | OBL-001..028 は `open`、SCN-01..10 の official status は不変 | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| M0 close/non-effect | bootstrap payload `be5928a1` は review/validation/push 済み。I1+ runtime、SCN conformance、public API/ABI/wire、production deployment は開始・主張しない | `docs/reports/2581-mir-theory-v0-i1plus-milestone-0-bootstrap.md` |

M0 は governance/agent role/sole roadmap/derived-status cut を fresh validation、independent
review、one report、commit/push/remote parity で閉じた。M1 の direct blocker は concise
Constitution の Canon placement と contradiction audit であり、direct consumer は M2 の
semantic-assertion profile である。

## 現在の停止線

- M0 は M2 の semantic-assertion profile、G0 exit、T1 entryを先取りしない。
  `mirrorea_canon/adr/ADR-0013.md`
- M3 は evaluation site/trigger/authority/materialization と same-owner RMW を閉じるまで、
  cross-owner snapshot/transactionを仮定しない。`mirrorea_canon/plan/02-operating-model.md`
- M4 は bird/shoulder relation を C-local evaluationへ遅延し、split frame、stale anchor、
  semantic fallback re-promotion、derived information leakを拒否する。
  `mirrorea_canon/plan/02-operating-model.md`
- final public contract、production deployment、guarantee weakening、domain vocabularyのCore化
  は ADR-0015 owner-reserved condition である。`mirrorea_canon/adr/ADR-0015.md`

## オーナーの確認・判断待ち

現在の M1 を止める owner decision はない。以下だけが escalation 条件であり、通常の
grammar、internal carrier、proof decomposition、test、bounded model、roadmap wording は
milestone 内で evidence-gated に決める。

| 条件 | 影響 | 現在の扱い |
| --- | --- | --- |
| North Star / safety/privacy weakening | project guarantee | stop and issue a concise decision bundle |
| World等の Core primitive化 / v0 non-goalの必須化 | scope | stop and escalate |
| final public API/ABI/wire、deployment/publication | irreversible external contract | stop and escalate |
| user data/secret risk | safety | stop and escalate |

根拠: `mirrorea_canon/adr/ADR-0015.md`。

## 根拠と詳細

| 知りたいこと | 正本またはLAB evidence |
| --- | --- |
| Constitution/program authority | `mirrorea_canon/meta/proposals/PROPOSAL-018-mir-v0-i1plus-autonomous-execution.md`, `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle / Gate | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| operating rules / source hierarchy | `mirrorea_canon/plan/02-operating-model.md`, `mirrorea_canon/meta/source-hierarchy.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| current execution sequence | `plan/247-mir-theory-v0-i1plus-current-roadmap.md` |
| runnable historical evidence | `samples_progress.md` |

## 更新規約

M0--M10 close では Canon consequence を先に、then Plan 247、this derived view,
`progress.md`、`tasks.md` を同期する。一 milestone 原則一 reportであり、report/history
は current queue ではない。sample path/command/classification/blocker が変わらない限り
`samples_progress.md` は更新しない。

各 milestone は author と異なる independent reviewer の review、focused validation、
`git diff --check`、commit/push/remote parityを必要とする。未実行 validation を pass と
記録しない。
