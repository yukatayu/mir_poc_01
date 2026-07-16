# ドキュメント要約

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

この文書は、リポジトリを読み始めるための短い案内です。現在の状態は
`docs/project-status.md`、詳細な研究計画は `plan/`、不変の作業証跡は
`docs/reports/` に置きます。

## 最初に読む順序

1. `mirrorea_canon/README.md`
2. `mirrorea_canon/MAP.md`
3. `mirrorea_canon/plan/00-gates.md` と `mirrorea_canon/plan/01-phases.md`
4. `docs/project-status.md`, `progress.md`, `tasks.md`, `samples_progress.md`
5. 対象領域の canon と、その根拠としての `plan/` / `specs/`

canon が方針・理論・ADR・適合性・プロセスの正本です。`plan/` と legacy
`specs/` は LAB の証拠・履歴・比較であり、規範判断を上書きしません。

## プロジェクトの目的

Mir の `.mir` を意味の正本とし、そこから配置・明示通信・検証・観測・安全な
evolution を導ける仮想空間基盤を作ることです。Mir、Mirrorea、PrismCascade、
Typed-Effect Wiring Platform は関連しますが、意図的に分離して扱います。

```text
.mir source
  -> Surface parser / elaboration
  -> Core Mir / typed IR / proof obligations
  -> checker / runtime
  -> projection / provider boundary / devtools evidence
```

実装上の層とその関係は `docs/diagrams/layer-stack.mmd`、理論から実装までの
進行と判断点は `docs/diagrams/workflow.mmd` を読むと確認できます。

## 現在の位置

- canon 上は `T0/G0 rebaseline`。G0 exit と T1 entry はまだありません。
- T0-T2 は、既存 canon に接続した非休眠の LAB research work unit を、
  `research-complete` または `decision-ready` まで自走する研究段階です。
  Gate exit、ADR effectivity、L0/L1 決定、`theory/11` の proof status は owner
  と canon process の責務です。
- Product Alpha、Full System V1、Surface sample は限定された runnable LAB
  evidence です。最終言語・最終ABI・実 transport・分散永続・public product を
  意味しません。

この境界の詳細と停止条件は `plan/156-t0-t2-research-autonomy-envelope.md`、
実行可能サンプルの正確な分類は `samples_progress.md` にあります。

## Roadmap の読み方

| 段階 | 主眼 | 現在の扱い |
| --- | --- | --- |
| T0 | 語彙・decision・G0 | current。exit は未承認。 |
| T1 | 計算体系・G1-G3 statement | 研究は可能、公式 entry / exit は未成立。 |
| T2 | OBL-020/021/002 proof skeleton・G5 statement | 研究は可能、proof status は `theory/11` が唯一の正本。 |
| I1 | 単一プロセス reference implementation | T1/T2 の後。全SCNの C-static/C-runtime が必要。 |
| I2 | process 内 multi-place | I1 後。 |
| I3 | 実 socket transport | 初めて LAN 上で実際に複数人が動かせる段階。 |
| I4-I6 | 永続/patch、View、分散永続/federation | 後段。 |

各段階の gate 条件は `mirrorea_canon/plan/00-gates.md` と
`mirrorea_canon/plan/01-phases.md` が正本です。

## 作業の管理

- `docs/project-status.md`: 人間向けの現在地、停止線、判断待ち。
- `progress.md`: 三軸の進捗スナップショットと最近の作業。
- `tasks.md`: 自走可能な work unit と、owner 判断または研究で扱う未決事項。
- `plan/`: 詳細な根拠、比較、方針、実行計画。
- `samples_progress.md`: runnable sample の dashboard。
- `docs/reports/`: task ごとの不変な証跡。

調査で semantic choice、SCN expectation の変更、canon-LAB 衝突、新しい実装レーン、
proof status 更新、Gate exit が必要になったら、agent は止まり、選択肢・影響・
反例・Lean/実行証拠・非主張を含む decision bundle を作ります。
