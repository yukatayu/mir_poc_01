# ドキュメント要約

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

この文書はリポジトリを読み始めるための短い案内です。規範判断は
`mirrorea_canon/`、現在地は `docs/project-status.md`、詳細計画は `plan/`、
task ごとの不変な証跡は `docs/reports/` にあります。

下の `plan/` 索引は時系列の LAB repository memory です。索引にある候補や過去の
「次」は current queue を意味しません。現在の一本道は
`docs/project-status.md`、`progress.md`、`tasks.md` で確認します。

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
| C5-PRE admission 発行段階ガード監査の証跡 | `plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md` |
| V1/R1 administrative presentation 比較の選別 | `plan/202-v1-r1-presentation-refinement-candidate-selection.md` |
| V1/R1 administrative presentation 比較の証跡 | `plan/wrk-0033-v1r1-presentation-refinement.md` |
| V1/R1 finite-sequence presentation 比較の選別 | `plan/203-v1-r1-finite-sequence-candidate-selection.md` |
| V1/R1 finite-sequence presentation 比較の証跡 | `plan/wrk-0034-v1-r1-finite-sequence-refinement.md` |
| WRK-0034 後の意味論合成 frontier provisional disposition | `plan/204-wrk0034-semantic-composition-no-candidate-disposition.md` |
| C7 parametric factorization の選別 | `plan/205-c7-parametric-factorization-candidate-selection.md` |
| C7 parametric factorization の L3 証跡 | `plan/wrk-0035-c7-parametric-factorization.md` |
| C7 cumulative-erasure countermodel の選別 | `plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md` |
| C7 cumulative-erasure countermodel の L3 証跡 | `plan/wrk-0036-c7-cumulative-erasure-countermodel.md` |
| WRK-0036 後の自律研究 frontier | `plan/207-post-wrk0036-autonomous-frontier-disposition.md` |
| C2-B/C3 value-flow design preparation | `plan/208-c2b-c3-value-flow-design-preparation.md` |
| C2-B/C3 relation-obligation audit | `plan/209-c2b-c3-relation-obligation-audit.md` |
| C2-B/C3 Family A/B instantiation audit | `plan/210-c2b-c3-family-a-b-instantiation-audit.md` |
| C2-B/C3 B-primary opaque-anchor candidate selection | `plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md` |
| C2-B/C3 bundled/relational presentation comparison selection | `plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md` |
| C2-B/C3 fiberwise relational comparison selection | `plan/213-c2b-c3-fiberwise-relational-comparison-selection.md` |
| WRK-0039 後の C2-B/C3 autonomous frontier | `plan/214-post-wrk0039-autonomous-frontier-disposition.md` |
| C2-B/C3 ordinary design decision packet | `plan/215-c2b-c3-ordinary-design-decision-packet.md` |
| C2-B/C3 cross-boundary compatibility audit | `plan/216-c2b-c3-cross-boundary-compatibility-audit.md` |
| C2-B/C3 carrier-neutral conditional comparison | `plan/217-c2b-c3-carrier-neutral-conditional-comparison.md` |
| C2-B/C3 first candidate-card source preflight | `plan/218-c2b-c3-first-card-source-preflight.md` |
| C2-B/C3 minimal semantic-residence options | `plan/219-c2b-c3-minimal-semantic-residence-options.md` |
| C2-B/C3 relation-state proof-obligation audit | `plan/220-c2b-c3-relation-state-proof-obligation-audit.md` |
| C2-B/C3 Canon proposal preparation | `plan/221-c2b-c3-canon-proposal-preparation.md` |
| P017 X1 finite L3 detector (not promoted) | `mirrorea_canon/working/WRK-0040-p017-x1-coupled-anti-collapse-countermodel.md`, `plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md` |
| P017 X1 terminal L3 detector (not promoted) | `plan/222-p017-x1-owner-terminal-exclusivity-candidate-selection.md`, `mirrorea_canon/working/WRK-0041-p017-x1-owner-terminal-exclusivity-countermodel.md`, `plan/wrk-0041-p017-x1-owner-terminal-exclusivity-countermodel.md` |
| P017 X1 owner-negative/mutation L3 detector (not promoted) | `plan/223-p017-x1-owner-negative-mutation-candidate-selection.md`, `mirrorea_canon/working/WRK-0042-p017-x1-owner-negative-mutation-countermodel.md`, `plan/wrk-0042-p017-x1-owner-negative-mutation-countermodel.md` |
| P017 X1 M1 adverse/mutation L3 detector (not promoted) | `plan/224-p017-x1-m1-adverse-mutation-candidate-selection.md`, `mirrorea_canon/working/WRK-0043-p017-x1-m1-adverse-mutation-countermodel.md`, `plan/wrk-0043-p017-x1-m1-adverse-mutation-countermodel.md`, `plan/225-post-wrk0043-fixture-frontier-disposition.md` |
| post-WRK-0043 cross-lane candidate preflight | `plan/226-post-wrk0043-cross-lane-p0a-preflight.md` |
| P017 X1 decision vector and choice-neutral candidate review | `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md` |
| P017 X1 minimum coherence candidate selection | `plan/228-p017-x1-minimum-coherence-candidate-selection.md` |
| P017 X1 linked static coherence evidence (not promoted) | `mirrorea_canon/working/WRK-0044-p017-x1-minimum-relation-envelope-coherence.md`, `plan/wrk-0044-p017-x1-minimum-relation-envelope-coherence.md` |
| post-WRK-0044 L3 frontier and ordinary-design handoff | `plan/229-post-wrk0044-no-successor-ordinary-design-boundary.md` |
| P017 X1 first ordinary-design card preflight | `plan/230-p017-x1-first-ordinary-design-card-preflight.md` |
| P017 X1 K0 R/L factorization preflight | `plan/231-k0-rl-factorization-preflight.md` |
| P017 X1 K0 R/L definitional collapse screen | `plan/232-p017-x1-k0-rl-definitional-collapse-screen.md` |
| P017 X1 K0 B fact-status screen | `plan/233-p017-x1-k0-b-fact-status-screen.md` |
| P017 X1 K0 typed terminal-success positive-basis card | `plan/234-p017-x1-k0-terminal-success-positive-basis-card.md` |
| P017 X1 typed owner-result role conformance audit | `plan/235-p017-x1-typed-owner-result-role-conformance-audit.md` |
| P017 X1 K0 owner-result provenance basis / definability screen | `plan/236-p017-x1-k0-owner-result-provenance-basis-and-definability-screen.md` |
| P017 X1 K0 owner-outstanding positive-basis / pending-nonconflation card | `plan/237-p017-x1-k0-owner-outstanding-positive-basis-and-pending-nonconflation-card.md` |
| P017 X1 K0 typed terminal owner-service-failure positive-basis card | `plan/238-p017-x1-k0-terminal-failure-positive-basis-and-failure-nonconflation-card.md` |
| P017 X1 K0 consulted validation-provenance basis / nonconflation screen | `plan/239-p017-x1-k0-consulted-validation-provenance-basis-and-nonconflation-screen.md` |
| P017 X1 K0 minimum-model H_K intake / receipt-endpoint reopen | `plan/240-p017-x1-k0-minimum-model-hk-intake-and-fail-closed-gate.md` |
| P017 X1 K0 H_K-rs occurrence-accounting preflight | `plan/241-p017-x1-k0-hk-rs-occurrence-accounting-preflight.md` |
| P017 X1 K0 H_K-rs integrated conditional candidate selection | `plan/242-p017-x1-k0-hk-rs-integrated-conditional-candidate-selection.md` |
| P017 X1 K0 H_K-rs L3 standing-eligibility recheck | `plan/243-p017-x1-k0-hk-rs-l3-standing-eligibility-recheck.md` |
| P017 X1 K0 H_K-rs single-block premise/falsifier design | `plan/244-p017-x1-k0-hk-rs-source-premise-falsifier-design.md` |
| P017 X1 K0 H_K-rs A-Sigma conditional trace (frozen / DEFER) | `mirrorea_canon/working/WRK-0045-p017-x1-k0-hk-rs-asigma-conditional-trace.md`, `docs/reports/2564-wrk0045-p017-x1-asigma-conditional-trace-execution.md` |
| post-WRK-0045 no-successor / ordinary X1 handoff | `plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md` |
| P017 X1 K0 q-fiber U/L finite conditional evidence (not promoted) | `mirrorea_canon/working/WRK-0046-p017-x1-k0-qf-ul-lift.md`, `plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`, `docs/reports/2572-wrk0046-p017-x1-k0-qf-ul-lift-execution.md`, `docs/reports/2573-wrk0046-positive-conditional-evidence-metadata-link.md` |
| post-WRK-0045 autonomous-frontier reconciliation | `docs/reports/2568-post-wrk0045-autonomous-frontier-reconciliation.md` |
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
  shared model、request/replay identity、scalar correspondence、profile は未確定です。
  Plan 199 は後続の composition/falsifier 境界を記録する LAB plan であり、現在の
  自律 package を開きません。
- P017 X1 は V1/R1 cross-locus-read に限る owner-accepted relation-state direction のままです。
  `frozen / DEFER` と Plan 245 の当初の `NO-SUCCESSOR` は WRK-0045 の predicate-only A-Sigma L3
  line にだけ及びます。WRK-0046 は `L3-open` / `not-promoted` の bounded evidence として実行・link 済みです。
  registered finite-line premises の下で ordinary-edge preservation と一つの supplied restore-edge
  preservation は二つの counted consume を排除し、A1 omission/reset control は restore preservation を
  外した exact two-consume trace を構成しました。これは K0、`Spent` の primitive/derived classification、
  carrier、receipt、identity、actual persistence/restore、Core、OBL、runtime、Gate/Phase、public behavior
  を選ばず、inventory/lifecycle の拡張も作りません。
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
- 大局的な current status / critical path / roadmap 更新は、Canon-first の read-only
  `planner` review を編集前と package close 前に受けます。review は新しい規範判断を
  作らず、既存 snapshot の blocker、権限境界、evidence、stop line を照合します。
- `plan/199-selected-semantic-composition-and-inference-boundary.md`: selected
  directions を shared model に合成する前の countermodel と安全な推論境界。
- `plan/200-reanchored-semantic-composition-research-plan.md`: C0--C7 を
  bounded research package に分解する実行順。
- `plan/201-c5-a2-issuance-guard-candidate-selection.md`: C5 の本体設計を
  選ばず、通常 admission に独立した issuance phase が明示されるかだけを監査する
  次の候補と停止線。
- `plan/wrk-0032-c5pre-ordinary-admission-issuance-guard.md`: P012 の条件文と
  four named ordinary-admission span の source-local reading。A2 atomicity や
  admission occurrence identity は導かない。
- `plan/202-v1-r1-presentation-refinement-candidate-selection.md`: C3 本体を
  設計せず、V1/R1 の administrative binding と one-slot presentation を有限 LAB
  comparison として検査できるかの選別。
- `plan/wrk-0033-v1r1-presentation-refinement.md`: WRK-0033 の登録後に
  `--trust=0` で確認した有限 comparison。matching/single-use/failure exclusion
  の下で二表現を比較し、各条件を外す adversarial distinction を保持する。C3 の
  semantic carrier や source inference を選ばない。
- `plan/203-v1-r1-finite-sequence-candidate-selection.md`: WRK-0033 と同じ
  finite model を一切変えず、arbitrary finite reply list へ閉包できるかを選別した。
- `plan/wrk-0034-v1-r1-finite-sequence-refinement.md`: WRK-0034 の登録後に
  `--trust=0` で確認した有限列 comparison。fixed translation の one-step
  preservation と `List.foldl` 後の local observation equality を保持するが、full
  trace equivalence、C3 proper、source inference は選ばない。
- `plan/204-wrk0034-semantic-composition-no-candidate-disposition.md`: WRK-0034 後の
  fixed finite-presentation line は scoped `no-candidate` とする。C7 の separate L3 result は
  retained 済みだが、carrier 選択、grounds、source rule には進まず、プロジェクト全体を閉じない。
- `plan/205-c7-parametric-factorization-candidate-selection.md`: C7 の local `erase`/`observe`
  parameter に対する pointwise unique-observation criterion を選別し、後続の登録・実行結果を記録する。
  Mir source rule や concrete inference を選ばず、choice/quotient を outcome stop line にする。
- `plan/wrk-0035-c7-parametric-factorization.md`: `range erase` 上の pointwise unique observation と
  fiber constancy の generic L3 conditional lemma、collision refutation、full-codomain countermodel を
  `--trust=0` で確認した LAB evidence。concrete Mir source、grounds、reconstruction は選ばない。
- `plan/206-c7-cumulative-erasure-countermodel-candidate-selection.md` と
  `plan/wrk-0036-c7-cumulative-erasure-countermodel.md`: individually checked erasures を common
  coarsening へ同時適用してはならない固定有限 countermodel を `--trust=0` で確認した L3 evidence。
  future C7 matrix の negative guard に限り、actual source transformation や omission rule は選ばない。
- `plan/207-post-wrk0036-autonomous-frontier-disposition.md`: current cut の C0-D--C7 を
  再審査し、新しい L3 candidate は作らないと記録する。次は C2-B/C3 の ordinary design
  preparation であり、identity、carrier、source rule は選ばない。
- `plan/208-c2b-c3-value-flow-design-preparation.md`: P012 V1/R1 と P013 M1 の最初の
  value-flow comparison cut。relation-first reference、request-occurrence anchoring、nominal
  attempt alternative を比較するが、carrier、runtime、source syntax は採択しない。
- `plan/209-c2b-c3-relation-obligation-audit.md`: Plan 208 の completed-success shorthand を
  prefix-local staged relation へ補正し、M1 validation outcome、linearity、save/load 復元、
  carrier-neutral な ergonomic projection の検査条件を整理する。Carrier や source rule は採択しない。
- `plan/210-c2b-c3-family-a-b-instantiation-audit.md`: staged obligation ごとに
  relation-first Family A と request-occurrence Family B を比較する。DAG は必要な順序制約のみを
  供給し、A/B とも semantic carrier ではなく、identity、pending、receipt、restore、held context
  の ordinary Canon design selection を待つ。
- `plan/211-c2b-c3-b-primary-opaque-anchor-candidate-selection.md`: B を Canon として選ばず、
  二つの不透明な request occurrence と明示的な model-local projection を使う有限 L3 experiment
  だけを pre-register 候補にする。payload、DAG、span、locus、transport、queue position は identity
  又は correlation の根拠にしない。
- `plan/212-c2b-c3-bundled-relational-presentation-comparison-selection.md`: WRK-0037 と同じ
  有限表に限り、bundled explicit view と independently stated relational view の観測・遷移保存を
  比較する。いずれも Canon carrier、identity、source inference を選ばない。
- `plan/213-c2b-c3-fiberwise-relational-comparison-selection.md`: bare view は key を保持しない
  ことを確認し、未実行の WRK-0038 を書き換えず、全十の supplied key ごとの fiberwise relation
  comparison だけを successor 候補にする。key recovery や source inference は選ばない。
- `plan/214-post-wrk0039-autonomous-frontier-disposition.md`: WRK-0039 後の同一 authority cut では
  新しい L3 candidate を作らない。有限比較の重複を避け、pending、reply/receipt/failure、restore/load、
  source reconstruction は ordinary Canon design boundary に戻す。これは carrier を選ばない。
- `plan/215-c2b-c3-ordinary-design-decision-packet.md`: ordinary design で連動して決める
  correlation basis、branch/lifecycle、restore/one-shot/linearity を整理する。自明な source
  convenience は、後段の選択済みモデルへの一意な elaboration proof としてのみ扱う。
- `plan/216-c2b-c3-cross-boundary-compatibility-audit.md`: Plan 215 を theory/01--06、
  typed branch、admissible load、M1 locality、trace-set boundary と照合する。carrier、
  Canon rule、implementation を選ばず、後段の省略記法を完全で検査可能な elaboration に限る。
- `plan/217-c2b-c3-carrier-neutral-conditional-comparison.md`: candidate-native の
  observation と erasable definition だけで比較する。shared pending/key/lifecycle を
  導入せず、`CARRIER-GAP`、`OPEN`、countermodel を candidate selection と区別する。
- `plan/218-c2b-c3-first-card-source-preflight.md`: existing `request` cut を
  source-ledger 化し、reply/receipt/consumption の selected semantic residence が
  無いため最初の candidate card は gap report に留まると確認する。
- `plan/219-c2b-c3-minimal-semantic-residence-options.md`: ordinary Canon design
  の選択肢を explicit relation state / history-only / nominal identity に絞る。
  relation state を LAB 推奨とするが、Canon carrier は選ばず、将来の source omission は
  選択済み意味論から一意に検査できる administration に限る。
- `plan/220-c2b-c3-relation-state-proof-obligation-audit.md`: Plan 219 の推奨を
  proof と取り違えず、pending、typed branch、causality、linearity、redaction、load、
  elaboration の obligation と falsifier を owner-facing decision 前に整理する。
- `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`:
  P017 X1 の通常設計で連動して比較する六つの意味論的選択軸と、選択中立の
  反例ケース・candidate-native card を整理する。具体 carrier、遷移、文法、runtime、
  proof は選ばない。
- `plan/228-p017-x1-minimum-coherence-candidate-selection.md`:
  P017 X1 の current cut で残る唯一の独立 candidate として、external rejection /
  no-observation の最小 coherence presentation を ADR-0014 L3 登録後にのみ既存
  Lean lane で検査する候補として選別した。fixture expansion、schema、transition、
  source、runtime、proof、Canon amendment は選ばない。
- `mirrorea_canon/working/WRK-0044-p017-x1-minimum-relation-envelope-coherence.md`:
  five-pair の非網羅的 witness と明示前提による static conditional account を
  Lean `--trust=0` で実行・link 済みとする。これは relation schema、lifecycle、
  transition、identity、causal order、SaveObject、validation、runtime、proof を
  選ばず、P017 X1 satisfaction や実装 readiness を主張しない。
- `plan/229-post-wrk0044-no-successor-ordinary-design-boundary.md`:
  WRK-0044 後の finite satisfiability、dynamic one-shot countermodel、causal-only
  audit を current cut では新規 L3 にしない理由を分け、Plan 227 の ordinary X1
  design card へ引き渡す。relation schema や R/B/T/U/C/L の答えは選ばない。
- `plan/230-p017-x1-first-ordinary-design-card-preflight.md`:
  Plan 227 の card を K0 external rejection と K1 typed rejection の最小差で
  preflight 化する。K0 は open、K1 は failure row 未選択の Canon gap であり、
  いずれも relation schema、transition、runtime、Canon proposal は選ばない。
- `plan/231-k0-rl-factorization-preflight.md`:
  K0 の relation residence と abstract restore correspondence だけを、一体の
  q-scoped fiber と coherence を持つ factorized relation families として比較する。
  両案が追加仮定なしに相互定義できるなら中立 skeleton に畳み、schema、ID、lifecycle、
  restore function、動的性質を選ばない。
- `plan/232-p017-x1-k0-rl-definitional-collapse-screen.md`:
  K0 R/L の共通制約は中立 skeleton にまとめるが、primitive unity と primitive
  plurality は tag/record/factorization/coherence を追加せずには相互定義できないと
  screen する。両案は未選択の OPEN delta として残し、次は各案に分けた B review とする。
- `plan/233-p017-x1-k0-b-fact-status-screen.md`:
  A-Sigma/B-Pi ごとに owner outstanding、typed terminal success/failure、consulted
  provenance の status を `OPEN` から始め、primitive/derived に閉じるための正の
  basis と停止線だけを記録する。branch model や failure row は選ばない。
- `plan/234-p017-x1-k0-terminal-success-positive-basis-card.md`:
  Plan 233 の typed terminal success 一行だけについて、direct native membership
  (A)、typed result と local terminality の derivation (B)、`OPEN` (C) を
  A-Sigma/B-Pi 別に比較する。A/A は advisory recommendation に留め、basis を
  adopt せず全 eight rows の repository-wide `OPEN` は維持する。
- `plan/235-p017-x1-typed-owner-result-role-conformance-audit.md`:
  P012/P017 を照合し、A の同一 membership は explicit candidate-local declaration
  がある場合だけ typed owner-result role も持てると整理する。payload typing
  だけでは足りず、requester receipt、provenance、causality、save/load は別の
  未決事項である。A は未採用で、Plan 233 の全 eight rows は `OPEN` のままである。
- `plan/236-p017-x1-k0-owner-result-provenance-basis-and-definability-screen.md`:
  result provenance を validation provenance や型付き値で代用しない。LAB-local
  `RP-min` の下で direct positive incidence (A) は conditional、静的導出 (B) は
  二解釈 countermodel、現況 (C) は `OPEN` とする。A は未採用で、reply/receipt、
  causal、save/load、schema はこの card では選ばない。
- `plan/237-p017-x1-k0-owner-outstanding-positive-basis-and-pending-nonconflation-card.md`:
  owner outstanding は requester `PendingFor`、端末事実の不在、M1/authority、queue、
  transport から導かない。A-Sigma/B-Pi とも direct positive membership (A) が最小の
  条件付き basis だが未採用であり、owner-service-pending 由来の導出 (B) は独立した
  positive source を待つ。したがって二 cell を含む Plan 233 の全 row は `OPEN` のまま。
- `plan/238-p017-x1-k0-terminal-failure-positive-basis-and-failure-nonconflation-card.md`:
  typed terminal owner-service failure は theory/02 の抽象 row containment の範囲で
  A/B/C を比較する。A/A は未採用の advisory で、failure row、validation、receipt、
  no-mutation、persistence は選ばず、Plan 233 の全 row を `OPEN` に保つ。
- `plan/239-p017-x1-k0-consulted-validation-provenance-basis-and-nonconflation-screen.md`:
  P013 の入力 claim、P017 item 1 の実際に consultation した authority ground、Plan 236
  の result-producing ground を分ける。direct linkage A は条件付き、導出 B は現在の
  positive premise 不足で不成立、C は `OPEN`。全 eight row は未採用のまま維持する。
- `plan/240-p017-x1-k0-minimum-model-hk-intake-and-fail-closed-gate.md`:
  Canon だけでは successful requester semantic receipt の endpoint は未選択だが、
  `s` の reply-send projection と、receive role を担う distinct candidate occurrence `r` を
  `H_K` として screen する余地は残る。owner-side service を receipt に潰さず、predicate や
  relation membership を causal edge の代用にしない。
- `plan/241-p017-x1-k0-hk-rs-occurrence-accounting-preflight.md`:
  この screen は `PREFLIGHT-ADMIT`。Theory 04 が既存の generic occurrence kind や receive
  rule を供給するとは読まない。`q -> s -> r` は明示した `H_K` trace としてのみ比較でき、
  `r` を operational に生成する rule、history field、new constructor、hidden matching/persistence
  key が必要なら Canon gap として止める。次は一つの presentation を選び、R/B/T/U/C/L を
  全て明示した conditional trace candidate が ADR-0014 の L3 preregistration 条件を満たすかを
  選別する。実装・Canon amendment・public behavior は未選択。
- `mirrorea_canon/meta/proposals/PROPOSAL-017-c2b-c3-relation-state-envelope.md`:
  V1/R1/M1 の C2-B/C3 について、owner が `X1 relation-state envelope` を記録した。
  cross-locus read に限る最小 L3 研究を開くが、carrier schema、transition、source grammar、
  runtime、OBL は選ばない。将来の省略記法も別 proposal として扱う。
- `plan/221-c2b-c3-canon-proposal-preparation.md`: P017 の X1 disposition と、次の
  ADR-0014 package の境界を整理する。`mirrorea_canon/working/WRK-0040-p017-x1-coupled-anti-collapse-countermodel.md` の最初の
  preregistration は、`plan/wrk-0040-p017-x1-coupled-anti-collapse-countermodel.md`
  で有限 fixture の `SEP` / `PHASE` / `ONE` / `AUTH` / `OBS` detector として実行済みで
  ある。これは carrier schema、transition、source grammar、runtime、OBL、または
  positive relation model を選ばない non-promoted L3 evidence である。Plan 222 は
  その後の distinct `X-BRANCH` candidate として WRK-0041 を登録した。これは
  owner-terminal overlap を四 fixture で検出し Lean `--trust=0` を通った
  non-promoted L3 evidence である。outcome type、failure row、branch state、
  transition、runtime は選ばない。Plan 223 / WRK-0042 は、P017 の別の
  no-owner-mutation clause を fixture-only overlap detector として実行し、Lean
  `--trust=0` と no-axiom checks を通った non-promoted L3 evidence である。failure
  semantics、mutation rule、attribution、carrier、transition、runtime は選ばない。
  Plan 224 / WRK-0043 は、P013/P017 の source-named M1 adverse condition を
  supplied finite tag とし、owner-mutation との overlap だけを検査した。四 fixture
  form は Lean `--trust=0` と four no-axiom checks を通った non-promoted L3 evidence
  である。validation、rejection、failure、mutation rule、carrier、transition、runtime
  は選ばない。Plan 225 はこの current cut の fixture-only line を scoped
  `no-candidate` として閉じ、per-tag/permutation の追加を禁じる。
- `samples_progress.md`: runnable sample dashboard。
- `docs/reports/`: task ごとの不変な証跡。

通常の新しい Oracle 相談は `ask-chatgpt-pro-temp`、継続は
`ask-chatgpt-pro-followup`、project-level continuity が本当に必要なときだけ
`ask-chatgpt-pro` を使います。Oracle は advisory であり、結果は repo source と照合し、
必要な内容だけを通常の source hierarchy へ mirror します。
