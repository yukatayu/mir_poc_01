# Project status

最終更新: 2026-07-22 20:35 JST

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
| 論理仕様 | L0/L1 の軸は canon に固定。ADR-0014 は standing boundary 内の existing LAB research と committed `WRK-####` L3 record を許す。WRK-0014 は same-carrier relation inclusion を前提にした三つの conditional transfer lemma を manifest した。これは実際の bridge を選ばず、各補題形での十分条件だけを示す。post-WRK-0014 remaining-ledger revalidation は diagnostics / authority / time / cut の既存 source-boundary 結果を新規 WRK に重複させないと確認した。Canon lifecycle は T0 のままで、後段の LAB evidence floor は T1/T2 entry や proof completion ではない。L2 は owner-authenticated trust anchor が未構成のため fail-closed であり、reproducible falsifier は L3/L2 を問わず record を即時 `frozen` にする安全弁である。 | `mirrorea_canon/adr/ADR-0014.md`; `mirrorea_canon/working/README.md`; `mirrorea_canon/working/WRK-0014-same-carrier-variance.md`; `mirrorea_canon/plan/02-operating-model.md`; `plan/post-wrk0014-remaining-ledger-revalidation.md`; `plan/149-current-phase-position-reading.md` |
| ユーザ向け仕様 | companion notation と runnable LAB examples はあるが、Surface v0 grammar closure と public contract は未完。 | `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`; `samples_progress.md` |
| 実装 / 運用 | alpha / Surface / operational roots は限定 LAB evidence。Full System V1 helper は accepted/rejected の内側 CLI 終了コード整合を確認し、committed provider/renderer generated evidence は実行時に書き換えず fresh report と比較する。Full System V1 checker は既存 host adapter の exact pair を宣言 signature / operation-specific capability / transition context に照合し、duplicate record field と非 scalar equality を runtime 前に拒否する。clean `4a52dd3e` と upstream tracking ref の一致上で 21/17/12 = 50 行を含む final release check 29 command を再現した。ただしこれは LAB の typed boundary guard であり、実行主体への trusted authorization、public adapter ABI、関数をまたぐ capability inheritance、Float64 runtime 実行は未実装である。release bundle は C-distributed conformance / 実 transport / multi-process execution を非主張とする。 | `samples_progress.md`; `scripts/README.md`; `tasks.md`; `mirrorea_canon/plan/01-phases.md` |
| Mir computational evidence | 15 行の sample matrix は 2 direct Product Alpha runtime acceptance、10 helper-only rows、3 direct package-check rejection から成る。`P-COMP-03` fixture は helper-only だが、Rust runtime test は構成した有効 package で五つの正例を直接実行し、五つの負例を直接 reject する。closed registry の負例は四つの静的型検査拒否と一つの評価時範囲外拒否に分かれる。WRK-0012 は固定一正例・一負例を観測したが番号付き結果 artifact の保持に許可外変更が必要となり `frozen` である。WRK-0013 は二 sidecar を immutable input として fresh run し、同じ正負分類を既存 unnumbered plan memo/index 経路へ保持した `not-promoted` L3 evidence である。これは一般 direct carrier や workflow の evidence ではない。`.mir` は Product Alpha `check` / `run-local` の入力ではない。 | `plan/53-mir-computational-core-roadmap.md`; `plan/166-mir-computational-baseline-directness-audit.md`; `plan/167-pcomp03-rejection-phase-cross-carrier-audit.md`; `plan/170-post-wrk0011-candidate-selection.md`; `plan/wrk-0013-retained-reproduction-selection.md`; `plan/wrk-0013-pcomp03-retained-reproduction.md`; `mirrorea_canon/working/WRK-0012-pcomp03-direct-carrier.md`; `mirrorea_canon/working/WRK-0013-pcomp03-retention-reproduction.md`; `samples_progress.md` |
| 研究 lifecycle | T-RESEARCH-001..033 は pre-delegation evidence。WRK-0001..0006 は既存 Lean lane で finite-index、OBL-021 outcome/projection、OBL-020 familywise/global の限定 L3 evidence を manifest し、いずれも `not-promoted` である。WRK-0007 は OBL-001 LAB draft の Result/write enumeration gap を、WRK-0008 は current-L2 formal-hook row が same-Place cut frontier を保持せず coarse runtime-cluster identity に留まることを、WRK-0009 は e5 foundation/static-route tuple 不一致を、WRK-0010 は static decision payload 非 attribution を、WRK-0011 は named source-route と direct-evaluator の final-store assertion location の差を記録した。WRK-0012 は固定一正例・一負例の direct-carrier command を観測後、必要な番号付き結果 artifact が許可外の validator/source-hierarchy 変更を要求したため `frozen` になった。WRK-0013 retained reproduction は二 sidecar を input として pin し、fresh post-registration output を existing unnumbered plan artifact へ保持して manifest 済みである。これは scoped provenance/retention evidence に限られ、Canon theorem/OBL の反証・証明、OBL status、Gate/Phase、general carrier/workflow の変更ではない。L2 selection は owner trust configuration まで fail-closed である。 | `mirrorea_canon/working/WRK-0001-finite-index-boundaries.md`; `mirrorea_canon/working/WRK-0002-obl021-projection-vacuity.md`; `mirrorea_canon/working/WRK-0003-obl021-projection-extensionality.md`; `mirrorea_canon/working/WRK-0004-obl021-outcome-totality.md`; `mirrorea_canon/working/WRK-0005-obl021-conditional-outcome-relation.md`; `mirrorea_canon/working/WRK-0006-obl020-familywise-global-boundary.md`; `mirrorea_canon/working/WRK-0007-obl001-result-write-coverage.md`; `mirrorea_canon/working/WRK-0008-obl027-formal-hook-attribution.md`; `mirrorea_canon/working/WRK-0009-current-l2-e5-skeleton-identity.md`; `mirrorea_canon/working/WRK-0010-static-formal-hook-decision-attribution.md`; `mirrorea_canon/working/WRK-0011-current-l2-final-store-directness.md`; `mirrorea_canon/working/WRK-0012-pcomp03-direct-carrier.md`; `mirrorea_canon/working/WRK-0013-pcomp03-retention-reproduction.md`; `plan/164-obl001-result-write-coverage-boundary.md`; `plan/165-post-wrk0007-candidate-selection.md`; `plan/168-wrk0009-e5-skeleton-identity-selection.md`; `plan/wrk-0009-e5-skeleton-identity.md`; `plan/wrk-0010-static-formal-hook-decision-attribution.md`; `plan/wrk-0011-current-l2-final-store-directness.md`; `plan/170-post-wrk0011-candidate-selection.md`; `plan/wrk-0013-retained-reproduction-selection.md`; `plan/wrk-0013-pcomp03-retained-reproduction.md`; `plan/wrk-0008-obl027-formal-hook-attribution.md` |
| WRK-0014 evidence | Canon carrier を具体化せず、同一 carrier の relation inclusion を前提にした三つの generic Lean lemma を manifest した。universal safety/coherence には intended-to-model inclusion、outcome existence には model-to-intended realization を置く補題形での十分条件であり、一般の必要条件や actual coverage/realizability、Core/result correspondence、fairness、BND-001 totality placement、OBL status は未選択である。`f01e5160` の source-history audit は、ignored local state を可逆に退避した状態で通過した。通常作業ツリーでは validator がその local state も dirty として拒否するため、これは clean-worktree release claim ではない。 | `mirrorea_canon/working/WRK-0014-same-carrier-variance.md`; `docs/reports/2361-wrk0014-same-carrier-variance-evidence.md`; `docs/reports/2363-wrk0014-authoritative-audit.md`; `plan/171-theory-core-correspondence-and-disposition-checkpoint.md` |
| システム配置 | Mir Surface/Core/Trace/Verify、Mirrorea fabric/projection、typed provider/View、domain application は分離して扱う。 | `docs/diagrams/layer-stack.mmd`; `mirrorea_canon/MAP.md` |

`plan/162-post-wrk0006-candidate-selection.md` の priority pause は、当時の既知候補に
対する LAB disposition のままである。WRK-0007 はその再開条件に該当した、Result/write
coverage の別系統の structural mismatch を L3 evidence として記録する。Foundation audit の
PROPOSAL-008 は BND-001 outcome-totality 専用の owner decision surface のままで、この
countermodel はそれに依存せず解決もしない。ADR-0014 の standing eligibility は有効である。
`plan/165-post-wrk0007-candidate-selection.md` は、この後の候補を再選別し、新しい
statement-shaped artifact を開かない理由と再開条件を記録する。これは eligibility を
狭める Canon rule ではない。

WRK-0008 はその後に選んだ既存 current-L2 lane の attribution question である。four
runtime examples の hook row は event-kind presence から生成され、same-Place /
frontier relation を artifact に運ばない。これは BND-003 carrier の選択、OBL-027 の
statement/proof/status、runtime semantics を決めない。詳細は
`plan/wrk-0008-obl027-formal-hook-attribution.md` に残す。

WRK-0009 は同じ helper を修理せず、existing static e5 route と foundation の
literal tuple identity だけを調べた L3 record である。両位置で literal mismatch
を retained evidence として記録したが、mapping や semantic meaning は主張しない。
詳細は `plan/wrk-0009-e5-skeleton-identity.md` に残す。

WRK-0010 は e4/e5/e12/e14 の static-gate decision payload が existing formal-hook
artifact に literal または exact artifact reference として残らないことを manifest
した L3 record である。これは attribution classification だけであり、diagnostic
meaning、defect、schema/helper repair、carrier、OBL、Gate/Phase は扱わない。

WRK-0011 は、e21/e22 の named source-route body が exact
`RunReport.final_place_store` を直接比較せず、separate fixture/direct-evaluator body
が exact `evaluator.state.place_store` を直接比較するという assertion-location
だけを manifest した。state correctness、source/fixture equivalence、coverage、
defect、schema/helper repair、carrier、OBL、Gate/Phase は扱わない。

post-WRK-0011 selection は、以前の record が許した root を恒久 whitelist として
扱わないよう訂正した。ADR-0014 では future WRK が自らの既存 documented LAB lane を
宣言でき、その lane で bounded non-production source/test change を行える。WRK-0012 は
P-COMP-03 の固定一正例・一負例に既存 `world` package の `MirCompute` carrier を sidecar
manifest として適用し、committed pre-registration 後に登録済み command を実行した。
ただし required numbered result artifact は除外済み validator/source-hierarchy change を
要求したため frozen である。これは全 row、一般の Mir execution、schema/helper/runtime/CLI
repair、Canon carrier、OBL、Gate/Phase を決めず、future successor は独立に登録する必要がある。
詳細は `plan/170-post-wrk0011-candidate-selection.md` と WRK-0012 に残す。

post-WRK-0013 triage は evidence-backed no-candidate で閉じた。Surface source-patch と
ELAB には事前固定できる literal span/fixture key がなく、byte-identical
`Computational.Scope.Positive.clamp_zero` source の二経路観測は既存 Full System V1 runner の
低情報量な reserve に留める。Product Alpha 側の `.mir` は引き続き explanatory であり、
source authority、Product Alpha textual input、helper/source equivalence、general compatibility、
runtime correctness、workflow、Canon/OBL/Gate/Phase は扱わない。
`plan/post-wrk0013-no-candidate-disposition.md`

その後の whole-portfolio review も、formal/Lean、current-L2、Surface patch、Full System
V1、operational の各 existing lane に、二つの live downstream branch を持つ新規 L3 target
がないことを確認した。以後は exact structural mismatch と documented downstream decision が
同時に現れるまで新しい L3 record を開かない。theory-core correspondence audit はその cut 後に、safety/coherence と outcome existence の conditional transfer lemma が異なる inclusion direction を使う、別の source-grounded structural mismatch を切り出した。WRK-0014 はその三補題を manifest したが、具体意味論や actual bridge は追加していない。これは将来の研究禁止ではない。
`plan/post-wrk0013-portfolio-review.md`

Theory core correspondence audit は、既存 Lean source を clean worktree で再実行し、
current LAB OBL-001 draft の Core `c` / opaque Result enumeration gap、current LAB
OBL-020 wrapper の global / familywise coverage boundary、current LAB OBL-021 coherence
draft の totality separation を再確認した。これは Canon OBL の反証・完了・変更ではない。
現時点の proof-facing package は OBL-001 を直接 `c` に結び、coverage を示さない
familywise result を global result として扱わず、PROPOSAL-008 について owner response と
required Canon process が完了するまで OBL-021 で outcome existence を仮定または非主張に
留める。`plan/171-theory-core-correspondence-and-disposition-checkpoint.md`

WRK-0014 は、same-carrier の parameterized relation に限定した三つの conditional Lean
lemma を manifest した。universal safety/coherence は intended-to-model inclusion を前提とする
補題形で、outcome existence は model-to-intended realization を前提とする補題形で transfer を
示す。これは各補題形の十分条件であり、actual Canon relation の coverage/realizability、
Core/result correspondence、fairness、BND-001 totality placement を選ばず、OBL を前進させない。
`mirrorea_canon/working/WRK-0014-same-carrier-variance.md`
post-WRK-0014 actual-bridge screen は no-candidate で閉じた。OBL-020 direct-global route は
既存の第二 same-carrier relation と literal mapping が現れた場合だけ reserve とし、OBL-001 は
direct-`c` または output/Core-write interface の owner choice、OBL-021 は PROPOSAL-008 の
owner disposition を待つ。`plan/post-wrk0014-actual-bridge-disposition.md`
remaining-ledger revalidation は、OBL-024/025 と authority/time/cut の残り source family が
既存 T-RESEARCH boundary の言い換えに留まることを確認した。この screen から新規 WRK は開かず、
actual bridge を再開する最小条件は OBL-001 の direct-`c` または exact output/Core-write
interface の owner/canon disposition である。ただしこれは ADR-0014 の別種の standing-eligible
L3 research を閉じない。`plan/post-wrk0014-remaining-ledger-revalidation.md`
P-SURF-05 second-admission stale-fence candidate は必須入力が許可済み LAB root 外と判明し、
`WRK-0015` を作らず停止した。fresh 実行・Canon 解釈はなく、事前探索 command は除外したままである。許可 root はこの候補のために拡張しない。`plan/wrk-0015-stale-grant-fence-registration-preflight.md`
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
- `PROPOSAL-003`、`PROPOSAL-004`、`PROPOSAL-008` は L1-reserved question なので delegated route
  では採択しません。`mirrorea_canon/meta/proposals/PROPOSAL-003-obl020-formalization-boundary-review.md`; `mirrorea_canon/meta/proposals/PROPOSAL-004-surface-v0-grammar-closure.md`; `mirrorea_canon/meta/proposals/PROPOSAL-008-elaboration-outcome-totality-boundary.md`
- 運用 storage は root filesystem のみで、2026-07-21 21:48 JST 時点の空きは約
  20 GiB です。承認済み cleanup で再生成可能な local build output と Mirrorea 一時成果物を
  除去しました。configured external workdir は依然未マウントなので、次の heavy build / generated
  artifact / toolchain work の前にも容量を再確認し、外部 workdir を優先します。

## オーナーの確認・判断待ち

| ID | 状態 | いま必要なこと |
| --- | --- | --- |
| G0-D3 | DEFERRED (dormant) | owner が明示的に reopen するまで選定しない。 |
| OBL-001 Core/result correspondence | owner/canon record pending | future proof-facing package が Result carrier を使うなら、THM-001 の `c` write とどう対応させるかを明示する。現在は direct-`c` reading を推奨し、実験用 Result を Core と同一視しない。 |
| PROPOSAL-003 | owner record pending | OBL-020 organization の A / B / C。 |
| PROPOSAL-004 | owner record pending | Surface v0 grammar closure の A / B / C。LAB recommendation は A。 |
| PROPOSAL-008 | owner record pending | BND-001 outcome totality の読みと、将来の obligation placement。 |

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
