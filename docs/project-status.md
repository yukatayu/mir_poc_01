# Project status

最終更新: 2026-08-04 16:50 JST

**Canon notice:** `mirrorea_canon/` is the normative source for direction,
theory, ADRs, conformance, and process. Everything outside `mirrorea_canon/`
is LAB; canon wins. This document is a LAB derived view.

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
| active frontier | **M7 checker/elaborator**。次は M8 deterministic runtime | `plan/247-mir-theory-v0-i1plus-current-roadmap.md` |
| authority | ADR-0015 の owner-approved bounded program。owner-reserved condition 以外は milestone evidence で自走する | `mirrorea_canon/adr/ADR-0015.md` |
| official lifecycle | `T1`。v3 `pass` digest acceptance により G0-D3/G0 exit/T1 entry を順に受理。v1/v2 artifact は historical evidence のまま | `mirrorea_canon/plan/01-phases.md`, `mirrorea_canon/adr/ADR-0017.md` |
| proof / scenarios | General OBL-001..028 は `intentionally-deferred`; finite M3 OBL-029..032 は `lean-proved`、033 は `model-checked-bounded`、034 は `runtime-monitored`; finite M4 OBL-035..039、M5 OBL-040..047、M6 OBL-048 は `lean-proved` の exact scope のみ。SCN-01..10 official status は不変、SCN-11/12 は milestone pressure scenario | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| M0--M6 close/non-effect | M6 は bounded declaration/action grammar、span-rich AST、M5-aligned Core-template/typed Diagnostic classification を、3 AST parser tests、11 classifier tests、trusted OBL-048 Lean evidenceで閉じた。M7 checker/elaborator、M8 runtime、final public grammar/diagnostic ABI/wire、general theorem、transport、official conformance、I1、production deployment は動かさない | `docs/reports/2581-mir-theory-v0-i1plus-milestone-0-bootstrap.md`, `docs/reports/2582-mir-theory-v0-i1plus-milestone-1-constitution.md`, `docs/reports/2583-mir-theory-v0-i1plus-milestone-2-t0-g0-semantic-assertions.md`, `docs/reports/2584-mir-theory-v0-i1plus-milestone-3-evaluation-materialization.md`, `docs/reports/2585-mir-theory-v0-i1plus-milestone-4-maintained-relation-projection.md`, `docs/reports/2586-mir-theory-v0-i1plus-milestone-5-shared-model-metatheory.md`, `docs/reports/2587-mir-theory-v0-i1plus-milestone-6-surface-v0.md` |

M0 は governance/agent role/sole roadmap/derived-status cut を fresh validation、independent
review、one report、commit/push/remote parity で閉じた。M1 は concise Constitution の Canon
placement と contradiction audit を同じ discipline で閉じた。M2 は revision-bound semantic
assertion producer、fresh `pass` artifact、negative control、acceptance record を閉じ、T1
entry を受理した。M3 は ADR-0018/theory/13 の有限 calculusを、Lean・64 target-set
enumeration・focused Rust traces・independent reviewで閉じた。M4 は ADR-0019/theory/14/SCN-12 の
有限 relation calculusを閉じた。M5 は ADR-0020/theory/15 の non-opaque finite shared modelを閉じた。
M6 は ADR-0021/spec/01--04 の ordinary Surface grammar、span-rich AST、total M5-aligned
classificationを、3 AST parser tests、11 classifier tests、OBL-048 `--trust=0` Lean、independent
final reviewで閉じた。M7 は M6 の syntax/span/Core-template meaningを変更せず、source-first
checker/elaboratorを作る唯一の active milestoneである。

## 現在の停止線

- M7 は M6 grammar、AST/source span、total classificationを固定入力として、唯一の
  `parse → check → elaborate` route と inspectable generated Core/obligation/source traceを作る。
  M8 runtime、final public grammar/diagnostic ABI/wire は開始・固定しない。
  `plan/247-mir-theory-v0-i1plus-current-roadmap.md`
- final public contract、production deployment、guarantee weakening、domain vocabularyのCore化
  は ADR-0015 owner-reserved condition である。`mirrorea_canon/adr/ADR-0015.md`

## オーナーの確認・判断待ち

現在の M7 を止める owner decision はない。以下だけが escalation 条件であり、通常の
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
