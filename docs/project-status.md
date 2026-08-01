# Project status

最終更新: 2026-08-01 12:43 JST

**Canon notice:** `mirrorea_canon/` is the normative source for project
direction, theory, ADRs, conformance, and process. Everything outside
`mirrorea_canon/` is LAB: evidence, history, implementation, and operational
notes. If LAB text conflicts with canon, canon wins.

## この文書の役割

これは、人間が短時間で現在地、公式の一本道、次の owner 判断を確認するための
**LAB 派生ビュー**です。Gate / Phase、OBL status、理論、適合性、実装完了を新たに
決めません。詳細な時系列の候補・実験は `plan/` と `docs/reports/` に残し、索引や
過去の「次」を current queue と読み替えません。

- 規範正本: `mirrorea_canon/`
- T0--T2 の依存: `plan/196-t0-t2-implementation-entry-roadmap.md`
- I1 bootstrap/readiness: `plan/197-i1-bootstrap-decision-and-readiness-audit.md`
- 目的起点の semantic integration / I1 entry: `plan/246-goal-first-semantic-integration-and-i1-entry.md`
- 現在の task map: `tasks.md`
- runnable LAB: `samples_progress.md`

## 全体の進行チェックリスト

Gate: [ ] G0 軸と語彙 -> [ ] G1 普通の代入 -> [ ] G2 存在と fallback ->
[ ] G3 権限 -> [ ] G4 効果と観測 -> [ ] G5 cut と保存 -> [ ] G6 射影 ->
[ ] G7 hot-plug

Phase: [ ] T0 語彙と決定 -> [ ] T1 計算体系 -> [ ] T2 骨格証明 ->
[ ] I1 参照実装 -> [ ] I2 multi-locus -> [ ] I3 実 transport ->
[ ] I4 永続と patch -> [ ] I5 射影と View -> [ ] I6 分散永続と連合

チェックは引用可能な Canon exit record が成立したときだけ埋めます。

## 現在地

| 観点 | 現在の状態 | 保証できること |
| --- | --- | --- |
| Canon lifecycle | official `T0`。G0 exit / T1 entry record はない | 正本の語彙・境界を読める |
| proof ledger | OBL-001..028 は全件 `open` | LAB evidence と official proof status を区別できる |
| T0 evaluation | v2 profile は adopted、唯一の fresh artifact は fixed-control drift に対する valid `fail` | v1 と malformed `fail` を区別し、G0-D3 を進めない証拠を再現できる |
| lifecycle / I1 | P016 は narrow T2、separate I1-readiness/bootstrap、C-static formal entry の方向を記録 | selected statement-level semantics を bind する profile、ledger mapping、phase/conformance wording、実装認可は未作成 |
| goal-first semantic integration | S2-A の LAB comparison は完了。`C1-A-r`（target owner 内 RMW）と `C1-B`（determined `v′`）を分離し、P017 X1 を candidate-specific presentation で拡張する `C2-A-r` を defer と比較した。SCN-02 には two dependency / read-authority reconciliation が残る | Canon Core/Config/SaveObject/failure/SCN/profile は未変更。S2-B shared model/prototype は、SCN-02 reconciliation と C1/C2 の ordinary owner/Canon selection 後にのみ開始する |
| semantic kernel | P004/008/012/013/015 の方向と有限 LAB evidence はある。P017 X1 は owner-accepted のまま、WRK-0045 predicate-only A-Sigma L3 line だけが `frozen / DEFER`。WRK-0046 は実行・link 済みの `L3-open` / `not-promoted` finite conditional evidence で、A0 は一つの supplied finite lineage 上の two-consume を排除し、A1 は restore preservation を外した two-consume control を構成した | この evidence は P017 model、K0、carrier、transition、receipt/identity、actual restore/persistence、proof/OBL、実装を選ばない |
| runnable LAB | Surface、current-L2、Product Alpha、Full System V1、operational suite、Lean evidence を限定範囲で実行できる | 個別の parser/checker/runtime/transport evidence を再現できる |
| public/product | 未到達 | final grammar/API/ABI、official conformance、production runtime、WAN federation は主張しない |

official lifecycle の最短経路と、現在自走する semantic integration は別 lane です。
前者の active node は valid `pass` route を開くか defer を続けるかという owner/Canon
判断です。後者の S2-A comparison は完了しており、S2-B に進むには別途、SCN-02 の
two dependency と read/visibility authority をどう Core/SCN に表すか、および C1/C2
amendment surface を owner/Canon が選択する必要があります。

```text
official T0
  -> fixed-control disposition (owner/Canon)
  -> authorized valid pass route, exact evaluation, G0-D3 acceptance
  -> G0 exit / T1 entry
  -> official T1/T2/I1 acceptance

parallel now:
  S1 authority-aware semantic-cut packet (review-corrected)
  -> S2-A bounded comparison / ordinary amendment packet (complete)
  -> SCN-02 reconciliation + owner/Canon selection -> S2-B shared model
  -> S3 candidate-local statements -> S4 narrow T2+G5 preparation
  -> S5 I1 readiness/bootstrap packet
  -> authorized I1 implementation
```

Owner instruction fixes a separate stop condition for this autonomous mainline:
when selected semantics, the shared model, all-SCN implementation scope, the
I1-readiness record, and implementation authorization make I1 startable, issue
an I1-entry closeout and stop before the first implementation package. This is
not an I1 authorization claim today.

P017 X1 は owner-accepted relation-state direction のままです。WRK-0045 の
`frozen / DEFER` は predicate-only A-Sigma line に限られます。WRK-0046 は
実行・link 済みで、sole 434-line source は Lean 4.29.1 `--trust=0` を通過し、
retained 53 declarations は axiom dependency を持ちません。A0 は登録済み
preservation premises の下で two-consume を排除し、A1 は restore preservation
を外した omission/reset control を構成しました。

WRK-0046 の registration/evidence package は完了していますが、record 自体は
`L3-open` / `not-promoted` です。Gate input でも critical-path dependency でもなく、
successor、inventory、lifecycle package は選択されていません。
根拠: `mirrorea_canon/plan/01-phases.md`,
`mirrorea_canon/theory/11-metatheory-ledger.md`,
`plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`。

### T2 の意味

現行 Canon T2 criterion は OBL-020 / 021 / 002 の proof skeleton と G5 statement 群です。
これは全十 SCN の I1 実装 readiness を自動では保証しません。P016 は narrow T2 の後に
separate I1-readiness/bootstrap record を置く方向を既に記録しました。integrated route 又は
phase-contract amendment は、必要になった場合だけ別の Canon reopen として扱います。
`mirrorea_canon/spec/06-conformance.md` と phase table の C-static / C-runtime 表記も、その
record で明示的に整合化する必要があります。

## 現在の停止線

### official lifecycle のみを止めるもの

- fixed-control drift の pin 維持、normal Canon rebase proposal、又は defer は owner/Canon
  action である。O0 は silent rebase / retry を許可していない。
- valid `pass` evidence がないため、G0-D3 / T0 exit は不可能である。current v2 `fail` は
  受理対象にならない。
- G0/T1/T2/I1 acceptance、Canon amendment、production implementation authorization は
  owner/Canon boundary に残る。

### semantic integration で今検証するもの

- S2-A は `plan/246-goal-first-semantic-integration-and-i1-entry.md` と Report 2577 に
  comparison を固定した。SCN-02 の two dependency rows と `[READ-CROSS]` の
  read/visibility authority/failure-row をどう整合化するかは、LAB が補完できない
  baseline reconciliation である。C1-A-r/C1-B/defer と C2-A-r/defer を ordinary proposal
  packet として選択した後にのみ、shared model, Lean, prototype, runtime を作る。
- C1-A-r は target owner 内の read/calculate/write atomicity を条件付きで提案するが、
  write capability から private read authority を導かない。C2-A-r は P017 X1 に対する
  candidate-specific extension として cross-locus receipt/result/use の residence と
  occurrence/consumption presentation を提案する。hidden cross-owner
  transaction、hidden request identity、evaluator-only exchange state、load による consumed
  reset、又は semantic receipt を static `G_e` row と読むことは immediate falsifier である。
- D4 では scalar/terminal を explicit declaration として扱い、hidden singleton/default
  を許さない。必要な Canon change は実証後の通常 proposal に集約する。
- P017 X1 の branch/request association、receipt matching、failure row、causal integration、
  actual restore relationは未選択である。WRK-0045 を修復せず、B-Pi へ切替えず、K1 を修復しない。
  WRK-0046 の bounded result は actual semantic surface、global one-shot、又は full P017 closureを
  供給しない。それらを必要とする future work は owner/Canon stop のままである。
- I1 は T2 close と同義ではない。all-SCN scope、G4/G6/G7、OBL-003/027、C-static、
  carrier/BND baseline、scoped moratorium lift を readiness record が bind するまで実装認可はない。

上の停止線に触れる L0/L1、Core/external contract、SCN/Gate/Phase、
`mirrorea_canon/theory/11-metatheory-ledger.md`、新規
moratorium-protected lane は owner/Canon task へ戻します。根拠:
`mirrorea_canon/adr/ADR-0013.md`, `mirrorea_canon/adr/ADR-0014.md`,
`plan/180-t1-t2-statement-identity-dependency-closure-audit.md`,
`plan/197-i1-bootstrap-decision-and-readiness-audit.md`。

## オーナーの確認・判断待ち

| 順序 | 判断 | 主な候補 | 現在の見解 |
| --- | --- | --- | --- |
| 1 | fixed-control drift の disposition | pin 維持/defer; normal Canon rebase proposal | scoped audit は統治文書 drift に限定。silent rebase はしない |
| 2 | G0-D3 | future valid `pass` digest を accept; defer 継続 | `fail` は受理不能。ここまで T1 entry は開かない |
| 3 | C1/C2 semantic composition | SCN-02 の two dependencies/read authority を整合化し、C1-A-r/C1-B/defer と C2-A-r/defer を ordinary Canon work で選ぶ | S2-A comparison は complete。finite evidence を正の意味論に読み替えず、selected amendment 前に model/prototype を作らない |
| parallel reserve | D0/D3/D4 candidate と ADR-0014 eligible literal/conditional work | selected semantics を前提にしない独立 consumer/falsifier を持つものだけを自走する | Core/Config/SCN delta を含む shared formal model/prototype は ordinary Canon selection 後にする |
| 4 | lifecycle/profile contract | selected statement-level semantics と narrow T2 evidence の後に、P016 の separate readiness を actual Canon profile/wording に bind | P016 は記録済みの本線。integrated/phase amendment は reopen-only |
| 5 | T1/T2/I1 acceptance | selected shared model、statement/profile、skeleton/readiness を evidence cut として受理 | 前段が揃うまで判断しない |

owner 判断前の autonomous work は ADR-0014 の standing predicate 内の pinned-source
audit、countermodel、conditional lemma、review、acceptance packet 準備に限ります。新規候補は
独立 consumer/falsifier と rollback trigger を要し、Gate/Phase/OBL/semantics を動かしません。
`plan/196-t0-t2-implementation-entry-roadmap.md` と
`plan/199-selected-semantic-composition-and-inference-boundary.md` がこの境界を詳述します。

## 根拠と詳細

| 知りたいこと | 正本または証拠 |
| --- | --- |
| 目的、source hierarchy、概念地図 | `mirrorea_canon/README.md`, `mirrorea_canon/MAP.md` |
| Gate / Phase と current lifecycle | `mirrorea_canon/plan/00-gates.md`, `mirrorea_canon/plan/01-phases.md` |
| G0 defer と agent research boundary | `mirrorea_canon/adr/ADR-0013.md`, `mirrorea_canon/adr/ADR-0014.md` |
| proof status | `mirrorea_canon/theory/11-metatheory-ledger.md` |
| T0--T2 critical path | `plan/196-t0-t2-implementation-entry-roadmap.md` |
| I1 readiness boundary | `plan/197-i1-bootstrap-decision-and-readiness-audit.md` |
| statement identity / shared-model gaps | `plan/180-t1-t2-statement-identity-dependency-closure-audit.md`, `plan/199-selected-semantic-composition-and-inference-boundary.md` |
| WRK-0045 predicate-only A-Sigma L3-line closure / P017 ordinary-design boundary | `plan/227-p017-x1-decision-vector-and-choice-neutral-consistency.md`, `plan/245-post-wrk0045-no-successor-ordinary-x1-handoff.md`, `docs/reports/2568-post-wrk0045-autonomous-frontier-reconciliation.md` |
| WRK-0046 finite conditional evidence | `mirrorea_canon/working/WRK-0046-p017-x1-k0-qf-ul-lift.md`, `plan/wrk-0046-p017-x1-k0-qf-ul-lift.md`, `docs/reports/2572-wrk0046-p017-x1-k0-qf-ul-lift-execution.md`, `docs/reports/2573-wrk0046-positive-conditional-evidence-metadata-link.md` |
| runnable evidence | `samples_progress.md` |

## 更新規約

authority-bearing Canon 又は bounded LAB evidence を先に更新し、その後この派生ビュー、
`progress.md`、`tasks.md`、`samples_progress.md` を必要な範囲で同期します。大局的な
current status、critical path、roadmap、phase recut、lifecycle inventory の更新は、Canon-first
read-only planner review を編集前と package close 前に受けます。planner は advisory であり、
source delta がない場合は新しい plan を作らず snapshot maintenance に留めます。
