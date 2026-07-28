# ドキュメント要約

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

この文書はリポジトリを読み始めるための短い案内です。規範判断は
`mirrorea_canon/`、現在地は `docs/project-status.md`、詳細計画は `plan/`、
task ごとの不変な証跡は `docs/reports/` にあります。

| 知りたいこと | 読む場所 |
| --- | --- |
| 前提知識なしで全体像を掴む | `docs/mirrorea-project-overview.html` |
| 規範正本の構造 | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| Gate / Phase | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| 現在地と owner 判断 | `docs/project-status.md` |
| T0 から T2 の依存 | `plan/196-t0-t2-implementation-entry-roadmap.md` |
| I1 実装開始の判断・readiness | `plan/197-i1-bootstrap-decision-and-readiness-audit.md` |
| 選択済み方向の合成・推論境界 | `plan/199-selected-semantic-composition-and-inference-boundary.md` |
| 合成研究の再anchor済み実行順 | `plan/200-reanchored-semantic-composition-research-plan.md` |
| C5-PRE admission 発行段階ガード監査の選別 | `plan/201-c5-a2-issuance-guard-candidate-selection.md` |
| current task と runnable evidence | `tasks.md`, `progress.md`, `samples_progress.md` |
| Oracle 運用 | `.docs/oracle-chatgpt-pro-operations.md` |

## 最初に読む順序

1. `mirrorea_canon/README.md`
2. `mirrorea_canon/MAP.md`
3. `mirrorea_canon/plan/00-gates.md` と `mirrorea_canon/plan/01-phases.md`
4. `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`
5. task-specific Canon と、根拠としての `plan/` / `specs/`

`plan/` と legacy `specs/` は LAB evidence / repository memory であり、Canon を
上書きしません。

## プロジェクトの目的

Mir の `.mir` を意味の正本とし、正しい理論に基づいて Place をまたぐ実行・通信・
検証・可視化と安全な hot-plug を行える仮想空間基盤を作ります。

```text
.mir source
  -> Surface parser / elaboration
  -> Core Mir / typed IR / proof obligations
  -> checker / runtime
  -> projection / provider boundary / View evidence
```

Mir、Mirrorea、PrismCascade、Typed-Effect Wiring Platform は関連しますが、
意図的に分離します。`World` と `Game` は Mir 上で user が定義する概念です。

## 現在の位置

- official lifecycle は `T0`。G0 exit と T1 entry はまだありません。
- OBL-001..028 は唯一の Canon ledger 上ですべて `open` です。
- T0 profile v2 は adopted 済みです。v1 artifact は nonconforming historical
  evidence として保持され、唯一の fresh v2 artifact は fixed-control drift により
  `fail` です。G0-D3、G0 exit、T1 entry、I1 authorization は未成立です。
- T1/T2 には canonical phase-exit JSON profile がありません。
- 現行 T2 criterion は OBL-020/021/002 proof skeleton と G5 statement 群です。
  これは全十 SCN を対象とする I1-entry readiness を自動的には保証しません。
- `spec/06` は C-static 10/10 を I1 entry、C-runtime 10/10 を I1 exit と書く一方、
  phase table は両方を I1 exit に置いています。bootstrap の定義と整合化が必要です。
- P004/P008/P012/P013/P015/P016 の方向は owner が記録しました。exact grammar、
  shared model、request/replay identity、scalar correspondence、profile は未確定で、
  Plan 199 の composition/falsifier 研究が先です。
- Surface、current-L2、Product Alpha、Full System V1、operational suite、Lean
  evidence は bounded LAB として実行できます。final grammar/API/ABI、official
  conformance、production runtime、WAN federation、distributed durable save/load
  は主張しません。

official T2 まで owner input なしで連続自走することはできません。ただし、
owner-reserved checkpoint の間は、candidate comparison、countermodel、shared formal
model、Lean statement/skeleton、bounded implementation validation、review、
validation、report、commit/push を package 単位で進められます。ただし owner
disposition 後も、個別 package が ADR-0014 の standing predicate、既存 lane、
pre-registration、falsifier、non-effects、rollback を満たす範囲だけが自走可能です。
Canon integration、ledger movement、production implementation、新 lane/helper は
owner action のままです。

owner 判断前でも、既存 Canon の literal transcription 又は conditional lemma だけで
閉じる候補は ADR-0014 の standing predicate を再審査できます。既存 lane、
非重複の利用先、falsifier は保守的な LAB 選別規律であり、standing predicate を
狭める追加の Canon 条件ではありません。predicate と reserved-boundary exclusion を
満たす真に新規な candidate は個別に検討し、official status とは区別します。

## Roadmap の読み方

| 段階 | 主眼 | 現在の扱い |
| --- | --- | --- |
| T0 | 語彙・decision・G0 | current。v2 evaluation は `fail`。4 control は統治文書 drift と監査済みだが、rebase/retry は未承認。将来の rebase decision と、その後の G0-D3 が必要 |
| T1 | 計算体系・G1-G3 statement | no official entry。記録済み方向の composition research と shared model 後に自走 |
| T2 | OBL-020/021/002 skeleton・G5 statement | later。profile と proof-skeleton evidence class が未定義 |
| I1 | 単一 process reference implementation | T2 と all-SCN/G0-G7 readiness の関係を先に固定 |
| I2 | process 内 multi-place | I1 後 |
| I3 | 実 socket transport | I2 後。最初の real LAN phase |
| I4-I6 | 永続/patch、View、分散永続/federation | 後段 |

I1 実装を始める文脈では、P016 が narrow T2 の直後に all-SCN / G0-G7
statement-level readiness、OBL-003/027 の evidence class、C-static の位置、範囲付き
production authorization を bind する別 I1-readiness record という方向を記録しました。
profile と Canon amendment は未作成です。

## 作業の管理

- `docs/project-status.md`: 人間向け現在地、停止線、判断待ち。
- `progress.md`: 三軸、macro phase、feature maturity、recent log。
- `tasks.md`: 自走 package、research discovery、owner decision。
- `plan/196-t0-t2-implementation-entry-roadmap.md`: T0--T2 の dependency DAG と
  package close 条件。
- `plan/197-i1-bootstrap-decision-and-readiness-audit.md`: I1 bootstrap、formal
  I1 entry、I1 exit の区別、owner 判断の順序・候補・推奨。
- `plan/199-selected-semantic-composition-and-inference-boundary.md`: selected
  directions を shared model に合成する前の countermodel と安全な推論境界。
- `plan/200-reanchored-semantic-composition-research-plan.md`: C0--C7 を
  bounded research package に分解する実行順。
- `plan/201-c5-a2-issuance-guard-candidate-selection.md`: C5 の本体設計を
  選ばず、通常 admission に独立した issuance phase が明示されるかだけを監査する
  次の候補と停止線。
- `samples_progress.md`: runnable sample dashboard。
- `docs/reports/`: task ごとの不変な証跡。

通常の新しい Oracle 相談は `ask-chatgpt-pro-temp`、継続は
`ask-chatgpt-pro-followup`、project-level continuity が本当に必要なときだけ
`ask-chatgpt-pro` を使います。Oracle は advisory であり、結果は repo source と照合し、
必要な内容だけを通常の source hierarchy へ mirror します。
