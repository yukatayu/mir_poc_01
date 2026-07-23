# ドキュメント要約

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

この文書は、リポジトリを読み始めるための短い案内です。現在の状態は
`docs/project-status.md`、詳細な研究計画は `plan/`、不変の作業証跡は
`docs/reports/` に置きます。

| 知りたいこと | 読む場所 |
| --- | --- |
| 正本の層と依存 | `mirrorea_canon/MAP.md` と `docs/diagrams/layer-stack.mmd` |
| 理論から実装への道筋 | `mirrorea_canon/plan/00-gates.md`、`mirrorea_canon/plan/01-phases.md`、`docs/diagrams/workflow.mmd` |
| 現在地と owner 判断 | `docs/project-status.md` |
| 三軸の成熟度と検証面 | `progress.md` と `samples_progress.md` |
| 次の研究と候補履歴 | `tasks.md`、`plan/158-standing-bounded-autonomy.md`、`plan/160-obl021-statement-shape-checkpoint.md`、`plan/161-post-checkpoint-candidate-triage-and-runnable-baseline.md`、`plan/163-foundation-integrity-and-elaboration-outcome-audit.md`、`plan/164-obl001-result-write-coverage-boundary.md`、`plan/165-post-wrk0007-candidate-selection.md`、`plan/168-wrk0009-e5-skeleton-identity-selection.md`、`plan/wrk-0009-e5-skeleton-identity.md`、`plan/169-wrk0010-static-decision-attribution-selection.md`、`plan/wrk-0010-static-formal-hook-attribution.md`、`plan/wrk-0011-current-l2-final-store-directness.md`、`plan/170-post-wrk0011-candidate-selection.md`、`plan/171-theory-core-correspondence-and-disposition-checkpoint.md`、`plan/172-standing-autonomy-lane-correspondence-checkpoint.md`、`plan/173-local-predicate-constructive-decidability-selection.md`、`plan/174-local-predicate-proposition-decidability-selection.md`、`plan/175-post-wrk0017-axiom-profile-disposition.md`、`plan/post-wrk0013-no-candidate-disposition.md`、`plan/post-wrk0013-portfolio-review.md`、`plan/post-wrk0014-actual-bridge-disposition.md`、`plan/post-wrk0014-remaining-ledger-revalidation.md`、`plan/wrk-0015-stale-grant-fence-selection.md`、`plan/wrk-0015-stale-grant-fence-registration-preflight.md`、`plan/wrk-0008-obl027-formal-hook-attribution.md`、`mirrorea_canon/working/WRK-0014-same-carrier-variance.md`、WRK provenance refinement `plan/159-wrk-evidence-commit-integrity-recut.md`、historical `plan/156-t0-t2-research-autonomy-envelope.md` / `plan/157-delegated-theory-research-governance.md` |

現在の候補選別: `plan/176-current-standing-candidate-disposition.md`、
`plan/177-thm005-telemetry-effect-boundary-selection.md`、
`plan/178-post-wrk0018-candidate-rescreen.md`、
`plan/179-independent-source-locus-audit.md`、
`plan/180-t1-t2-statement-identity-dependency-closure-audit.md`、
`plan/181-preservation-proof-prerequisite-literature-audit.md`、
`plan/182-canon-core-minimality-and-proof-interface-audit.md`、
`plan/183-transparent-cost-bound-substitutability-decision.md`

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
- T0-T2 は、既存 LAB lane で理論候補を自走して比較・反証・bounded implementation
  validation できる研究段階です。ADR-0014 の standing predicate を満たすときだけ、
  `working/WRK-####` に L3 record を作れ、rebased final cut の independent review を
  経て L2 working state を更新できます。Gate exit、
  ADR effectivity、L0/L1、external contract、SCN/Phase、`theory/11`、final proof /
  OBL discharge は owner と canon process の責務です。
- Product Alpha、Full System V1、Surface sample は限定された runnable LAB
  evidence です。最終言語・最終ABI・実 transport・分散永続・public product を
  意味しません。
- Mir computational sample の 15 行は、直接 Rust 実行の二つの Product Alpha
  package 行、Python helper の十行、package schema の三つの拒否行に分かれます。
  ただし Rust runtime test は別途、有効な構成 package で五つの `P-COMP-03` 正例を
  直接実行し、五つの負例を直接 reject します。この closed registry では四件が
  型検査で止まり、一件だけが評価中の範囲外拒否です。fixture の
  `runtime_rejection` はこの位相を表す名前ではなく、Product Alpha の current
  `MirCompute` carrier も両者を公開上は区別しません。`.mir` は Product Alpha
  `check` / `run-local` の実行入力ではありません。したがってこれは限定された計算・
  境界検査の証拠であり、完成した言語実装ではありません。詳細は
  `plan/166-mir-computational-baseline-directness-audit.md`、
  `plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`、
  `samples_progress.md` を参照してください。
- Foundation audit は、Core に不要な domain/I/O primitive を見つけず、BND-001 の
  outcome-totality 読みだけを owner-reserved な PROPOSAL-008 として分離しました。
  これは OBL/Gate/Phase の変更ではありません。
- PROPOSAL-009 は、THM-001 の既存 Core `c` write 条件を将来の OBL-001 package が
  直接表明するか、明示 correspondence を持つ output view で表明するかを owner に問います。
  実験用 Result を Core と同一視せず、Core/runtime/OBL の意味は変更しません。
- PROPOSAL-010 は、overview の `child locus` / `admission path` という未定義の主語を、
  既存の principal による join/admission 説明へ置換するか、削除するか、将来 extension に
  分離するかを owner に問います。Locus 階層、membership provenance、Core/OBL は追加しません。
- PROPOSAL-011 は、Contract に含まれる `cost_bound` が transparent overlay の列挙条件から
  漏れている点を、non-weakening、全変更の明示更新、advisory 化、現状維持の owner 選択として
  分離します。最終 cost algebra、runtime、OBL、patch carrier は選びません。
- WRK-0007 は、OBL-001 LAB draft の `GeneratedWrite` が実験用 Result 内の
  write を尽くさない有限 countermodel を L3 evidence として記録しました。これは
  Canon THM-001 の反証でも Core IR の選択でもなく、将来の proof-facing
  Core/result bridge を選ばずに残した statement-shape gap です。
- Post-WRK-0007 selection は OBL-001 の別 predicate model、OBL-025 の scope /
  metadata-tuple model、OBL-024 の projection modelを比較し、新しい L3 record を
  選びませんでした。既知の scope guard、既存の coupling boundary、または未選択の
  carrier law を言い換えるだけの候補を避けています。
- WRK-0008 は current-L2 formal hook の attribution を既存レーンで監査しました。
  cut だけの `e1`、rollback だけの `e2`、両方を含む `e21`、nested Place の
  `e22` はすべて同じ obligation-shaped row を出します。これは formal-hook row
  単体が same-Place frontier の証拠ではないことを示す LAB 結果です。current-L2
  interpreter の別の Place-sensitive rollback path を否定せず、OBL-027、carrier、
  helper/schema、runtime、Gate/Phase は変更しません。
- WRK-0009 は、current-L2 static e5 route と Lean proof skeleton の identity
  tuple を literal に照合し、不一致を retained LAB evidence として manifest しました。
  foundation の `e5-underdeclared-lineage` / `rollback_cut_non_interference` と
  emitted route の `e5_underdeclared_lineage` / `canonical_normalization_law` は
  literal に一致しません。mapping、意図的 synthetic role、defect、意味論、theorem/OBL、
  carrier は判断せず、修理もしません。選定理由は plan/168、再現結果と停止条件は
  `plan/wrk-0009-e5-skeleton-identity.md` にあります。
- WRK-0010 は、e4/e5/e12/e14 static-gate decision payload が existing formal
  hook に literal または exact artifact reference として残るかを監査し、残らない
  ことを scoped LAB evidence として manifest しました。これは diagnostic meaning、
  defect、schema repair、carrier、theorem/OBL を決めません。詳細は
  `plan/wrk-0010-static-formal-hook-decision-attribution.md` にあります。
- WRK-0011 は、e21/e22 の四つの named source-route test body に exact
  `RunReport.final_place_store` comparison がなく、別の二つの fixture/direct-evaluator
  test body には exact `evaluator.state.place_store` comparison があることを literal に
  記録しました。状態意味、正しさ、source/fixture equivalence、defect、coverage、修理、
  theorem/OBL は判断しません。詳細は
  `plan/wrk-0011-current-l2-final-store-directness.md` にあります。

- post-WRK-0011 の候補再選別から `WRK-0012` は、P-COMP-03 の固定一正例・一負例を
  既存 Product Alpha `MirCompute` carrier で直接実行するために事前登録されました。
  二つの non-production sidecar は登録済み command の観測を満たしましたが、必須の
  番号付き結果 artifact を保持するには許可外の validator/source-hierarchy change が
  必要となったため record は `frozen` です。sidecar 二つだけが artifact であり、
  R-2347 は履歴 metadata に留まります。全 P-COMP-03、`.mir` の直接実行、public
  interface、Canon status は主張しません。forward `WRK-0013` は、二 sidecar を input
  のみに pin した retained reproduction として fresh execution を行い、既存 unnumbered
  plan memo/index 経路へ独立に保持しました。W12/R-2347 の観測は再利用していません。
  これは `not-promoted` の provenance evidence であり、全 P-COMP-03、general carrier、
  workflow、runtime claim にはなりません。選定・結果は
  `plan/wrk-0013-retained-reproduction-selection.md` と
  `plan/wrk-0013-pcomp03-retained-reproduction.md` にあります。post-WRK-0013 の
  source screen は、現時点では十分な情報量を持つ次の standing target を選べないと記録した。
  byte-identical source の二経路観測は reserve に留まり、Product Alpha の textual input 化、
  helper/source equivalence、general source compatibility、workflow への昇格を意味しない。
  `WRK-0019` は別 input の一つの non-production direct-world sidecar を既存 package path
  へ通し、`check` の受理と `run-local` の固定 `MirCompute` / `OutOfBounds` を観測した。
  これは LAB の一件の evidence であり、general carrier、runtime、public error contract、
  Canon、Gate/Phase、workflow を主張しない。詳細と再開条件は
  `plan/wrk-0019-pcomp03-bounds-direct-carrier.md`、
  `plan/post-wrk0013-no-candidate-disposition.md`、
  `plan/post-wrk0013-portfolio-review.md` にあります。

この境界と current run の詳細は `plan/158-standing-bounded-autonomy.md`、過去の
research evidence は `plan/156-t0-t2-research-autonomy-envelope.md` と
`plan/157-delegated-theory-research-governance.md`、WRK の Git evidence
provenance は `plan/159-wrk-evidence-commit-integrity-recut.md`、実行可能
サンプルの正確な分類は `samples_progress.md` にあります。WRK-0008 の scoped
evidence は `plan/wrk-0008-obl027-formal-hook-attribution.md` にあります。

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

調査で L0/L1 / core / external contract / SCN / Gate / Phase / final proof に触れる
choice、canon-LAB 衝突、新しい実装レーン、または authority ambiguity が生じたら、
agent は止まり、選択肢・影響・反例・Lean/実行証拠・非主張を含む escalation bundle
を作ります。その他の scoped candidate は evidence を LAB working state として継続できます。
canon working state への更新は `working/WRK-####` に限られ、L3 の事前登録は ADR-0014 の
standing predicate と commit のみで開始できます。L2 promotion だけが rebased frozen
material の independent review を必要とし、現行は owner-authenticated trust anchor 未構成の
ため fail-closed です。frozen record の follow-up は in-place repair ではなく successor
または escalation にします。`plan/170` は WRK-0012 の選定履歴です。source screen は
closed で、WRK-0013 retained reproduction の fresh outcome は unnumbered memo とともに
manifest 済みです。二つの sidecar は input に限られ、得られた result も scoped
  provenance evidence に留まります。post-WRK-0013 target triage は evidence-backed
  no-candidate で閉じたが、その後の theory-core correspondence audit は同一 carrier の
  relation inclusion に polarity mismatch を見つけ、WRK-0014 が三つの generic Lean lemma として
  manifest した。これは future bridge に使える conditional transfer の方向だけを示し、実際の Canon correspondence、
  Core representation、outcome totality、workflow や runtime claim は増やさない。post-WRK-0014
  actual-bridge screen は、既存第二 relation と literal mapping がないため no-candidate で閉じた。
  続く remaining-ledger revalidation も、診断・authority・time・cut の既存 source-boundary
  結果を重複する新規 WRK にしないと確認した。この actual-bridge screen は ADR-0014 の別種の
  standing-eligible L3 research を閉じない。Canon lifecycle は依然 T0 であり、後段の LAB
  evidence floor と混同しない。`plan/post-wrk0014-remaining-ledger-revalidation.md`
  その後、P-SURF-05 の second-admission stale-fence branch を source-local 運用候補として
  選定したが、登録 preflight で入力が現行 validator の許可 root 外と判明したため、WRK を作らず停止した。
  登録済み fresh 実行・Canon 解釈はなく、事前の探索実行は引き続き除外する。
  `plan/wrk-0015-stale-grant-fence-selection.md`;
  `plan/wrk-0015-stale-grant-fence-registration-preflight.md`
  後続の許可済み root 再選別も、この source cut では候補を選ばなかった。これは恒久的閉鎖ではなく、
  fresh 実行や Canon 解釈を伴わない LAB の限定的な選別結果である。
  `plan/post-wrk0015-permitted-root-no-candidate-disposition.md`
  続く lane-correspondence checkpoint は、validator の exact-root guardrail が ADR-0014 の
  existing documented LAB lane の閉じた正本かを **UNRESOLVED** とした。現行 fail-close は維持し、
  有限な `plan/158` ratchet だけを閉じる。`plan/172-standing-autonomy-lane-correspondence-checkpoint.md`
  その後の基礎 obligation 再監査は、既存 OBL-003/005/015 の証跡を重複させず、現行
  `captureSubset` helper の named `Decidable` value route は、Lean declaration form のため
  frozen になった。後継の proposition-valued excluded-middle route も、局所定理自体は一時的に
  公理なしで通った一方、登録済み opaque generic-domain control が暗黙の古典公理を使って通ったため
  frozen である。どちらも OBL-003 を前進させず、試験ソースは復元済みである。
  続く axiom-profile screen は、同じ診断を再実行する具体的 consumer がないため新 WRK を作らず
  no-candidate で閉じた。`plan/173-local-predicate-constructive-decidability-selection.md`; `plan/174-local-predicate-proposition-decidability-selection.md`; `plan/175-post-wrk0017-axiom-profile-disposition.md`
  広域 screen は standing-eligible な候補を選ばなかったが、その後の focused audit は
  THM-005 の declared telemetry-effect dependency だけを L3 record として事前登録したが、
  marked tail の最初のコンパイル失敗が登録済み falsifier となり frozen である。後続の green tail は
  保持せず、source は復元済みである。`plan/176-current-standing-candidate-disposition.md`;
  `plan/177-thm005-telemetry-effect-boundary-selection.md`; `mirrorea_canon/working/WRK-0018-thm005-telemetry-effect-boundary.md`
