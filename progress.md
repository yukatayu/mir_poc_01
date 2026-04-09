# progress

最終更新: 2026-04-09 16:07 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期参照用の repository memory は `plan/` である。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度

## 現在の要約

- **current L2 semantics** は、current mainline を進めるにはかなり安定している。
- **parser-free PoC** は、fixture / interpreter / host harness / bundle / batch / selection / profile / catalog まで揃っている。
- **detached validation loop** は、bundle / aggregate / static gate の emit・保存・compare・smoke を回せる入口が成立しており、Phase0/1/2 closeout の first-pass smoke と top-level mirror sweep も通した。
- detached validation loop の current helper baseline には、fixture stem shorthand、`smoke-fixture` / `compare-fixtures` の default run label derivation、missing fixture の fail-fast、`compare-fixture-aggregates` による single-fixture aggregate direct compare convenience、bundle / aggregate / static gate diff helper の shallow reference-only triage も入っている。したがって current self-driven friction reduction は checkpoint close とみなし、残りの `reference update / bless` は final path policy / retention policy と接続する later candidate に残してよい。
- **parser boundary / first checker reconnect** は、stage 1 / stage 2 / stage 3 の private staged spike と reconnect freeze threshold まで source-backed に揃ったうえで、current checkpoint では **Phase 3 self-driven portion を一旦尽きた reserve path とみなす threshold** まで固定できた。つまり、Phase 3 は「未完了の active line」ではなく「later pressure が出たときだけ reopen する line」と読むのが current snapshot である。
- **shared-space / membership** は mainline ではないが、upper-layer docs-first boundary として「participant plain array を core に焼き込まず、session-scoped membership registry + derived snapshot view を第一候補にする」比較に加え、tree-like view を derived に留めること、activation visibility の compile-time over-approximation と runtime control-plane を分けること、authority / consistency / RNG provider を別軸で比較すること、room resource ごとの owner slot / delegated capability / handoff epoch を分けて読む current working modelを維持している。authoritative room では current baseline judgment を `specs/examples/121-shared-space-authoritative-room-baseline.md` に集約し、**authoritative room に限って** activation rule の current first choice を `authority-ack`、authority placement の current first choice を `single room authority`、consistency mode の current first choice を `authoritative serial transition`、RNG / fairness source の current first choice を `authority_rng` に固定した。さらに、その次段として `specs/examples/122-shared-space-catalog-working-subset-comparison.md` で authoritative room と append-friendly room をまたぐ small cross-room working subset row を切り、authoritative room baseline、authoritative room witnessed draw、append-friendly room baseline、append-friendly room with optional RNG capability を current subset に置いた。reconnect / late leave / in-flight action は room profile に全部入れず、`member_incarnation` と uncommitted action invalidation を minimal room-profile rule、timeout / retry / resend を external policy layer に残す line まで進んだ。causal metadata 側では plain vector deletion を避け、epoch / incarnation split を first practical candidate、control-plane separated carrier を next stronger candidate に置くところまで進んだ。fairness trust model 側では `opaque authority trust` を current minimal candidate、`auditable authority witness` を next narrow strengthening candidate に置き、provider placement と witness requirement を別軸で比較する line を追加した。identity / auth layering 側では membership registry には identity core だけを残し、auth stack / admission policy は別 carrier に置く line を current first practical candidate にした。admission policy / compile-time visibility 側では role / capability / visibility requirement の over-approximation だけを compile-time に残し、actual admission / activation / reconciliation は runtime control-plane に残す line を current first practical candidate にした。したがって current self-driven mainline は authoritative room baseline 自体ではなく、その先の consistency / fairness / causal metadata catalog comparison のうち、まず `auditable_authority_witness` の最小 witness shape を比べる line に移ってよい。
- **async control / memory-model boundary** は、`atomic_cut` を place-local finalizing cut の最小核に留めたまま、higher-level ordering / fairness / consistency を event-tree / authority-serial transition / witness-aware commit family として Phase 4 / 5 の docs-first inventory へ送る line を current first practical candidate に置いた。C++ 的 low-level memory-order family は current immediate candidate にしていない。
- 現在の主ボトルネックは semantics の大崩れではなく、
  - fixture authoring / elaboration の反復コスト
  - parser boundary の staged 実装
  - async-control / proof boundary を `atomic_cut` の局所性と分離したまま inventory 化すること
  - richer host interface を後段に残したまま、必要最小限の typed coverage / proof boundary をどこで切るか
  である。

## 研究フェーズ（大局）

| Phase | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---|---|---|---|---|
| Phase 0 | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / progress の整合維持 |
| Phase 1 | current L2 semantics stabilization | 終盤 | 中 | 自走可能 | mainline semantics drift は narrow regression 中心 |
| Phase 2 | parser-free PoC / detached validation loop | 終盤の maintenance tail | 中 | 自走可能 | loop 入口と closeout baseline は成立、current self-driven friction reduction は checkpoint close |
| Phase 3 | parser boundary / first checker cut | current tranche closeout 完了、self-driven portion は reserve path | 中〜やや重い | 後段依存 | private staged spike / reconnect subline は freeze 済み。later pressure が出たときだけ reopen |
| Phase 4 | shared-space / membership / practical example boundary | 前半の authoritative baseline closeout 済み、catalog working subset first cut まで到達 | 重い | 着手可能 | baseline は `specs/examples/121...`、working subset は `specs/examples/122...`。current mainline は witness / provider / causal stop line の refinement |
| Phase 5 | static analysis / type / theorem prover / model checker boundary | 入口整理 | とても重い | 着手可能 | small decidable core / async-control inventory までは進めてよい |
| Phase 6 | actual parser / checker / runtime commitment | 未着手 | 重い | 後段依存 | Phase 2 / 3 / 5 の gate 後に入る |
| Phase 7 | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Typed-Effect / Prism / app contract は user 仕様待ち |

### 現在の主線

- **主線**: Phase 2 maintenance tail + Phase 4 side line + Phase 5 inventory line
- **maintenance tail**: Phase 0 / 1 / 2
- **side line**: Phase 4 前半
- **inventory line**: Phase 5 入口
- **まだ勝手に finalization しない**: final parser grammar、production exporter API、shared-space final catalog、higher-layer application contract

### immediate execution order

1. consistency / fairness / causal metadata catalog の current first cut を前提に、`auditable_authority_witness` の最小 witness shape を比較する
2. static analysis / type / theorem prover / async-control boundary の inventory を進める
3. checkpoint ごとに drift suppression と mirror sweep を入れる
4. detached validation loop は maintenance mode に戻し、policy-dependent な `reference update / bless` だけを later candidate に残す
5. authoritative room baseline は checkpoint close として維持する
6. Phase 3 は later pressure が出たときだけ reopen する

## いま自走で進めてよい範囲

### 着手可能

- current L2 semantics の narrow regression 追加
- fallback / notation / representative example の drift 修正
- parser-free PoC helper / detached validation loop の改善
- fixture authoring / elaboration の補助改善
- shared-space / authoritative room の docs-first boundary comparison
- static analysis / type / theorem prover / async-control boundary の inventory

### 後段依存

- parser boundary / staged spike の further reopening
- first checker cut / reconnect family の further reopening
- richer host interface の typed carrier 化
- static analysis / type system / theorem prover / model checker boundary の本格 actualization
- final parser grammar の固定

### 要仕様確認

- Mirrorea Fabric の具体仕様
- Typed-Effect Wiring Platform の具体仕様
- shared-space / session activation / membership protocol の具体仕様
- PrismCascade / 上位 shared space / 上位アプリケーション
- Reversed Library の具体要件

## 現在の主線と checkpoint までの evidence

### 1. current L2 core semantics

- fallback は guarded option chain / left-to-right monotone degradation / no re-promotion で整理済み。
- `TryFallback` / `AtomicCut` は runtime representative と dedicated structural helper first tranche の両方が揃っている。
- request / admissibility cluster は parser boundary inventory に入っているが、final parser grammar や public parser API にはまだ上げていない。

### 2. parser-free PoC / detached validation loop

- bundle-first emitter、aggregate emitter、static gate emitter がある。
- bundle / aggregate / static gate compare helper がある。
- tiny loop wrapper と fixture scaffold helper がある。
- detached artifact の split は
  - `payload_core`
  - `bundle_context`
  - `detached_noncore`
  - explanation
  で整理済み。
- `host_plan_coverage_failure` は current では aggregate-only を維持し、future typed carrier は bundle failure artifact 側の `failure.failure_kind` を最小核にする judgment で揃っている。

### 3. Phase 3 reserve-path までに揃った parser / checker evidence

- stage 1:
  - `e4` / `e7` の lowered fixture-subset compare と guard-slot retention smoke を actualize 済み
  - malformed-source first tranche も actualize 済み
- stage 3:
  - declaration-side `admit` attached slot の success-side first tranche を actualize 済み
  - malformed-source first tranche
    - `missing declaration-side admit slot payload`
    - `request head is outside stage 3 admit-slot first tranche`
    を actualize 済み
  - next-step sequencing と fixture-side `OptionDecl.admit` handoff は docs-only で整理済み
  - later branch の request-local clause spillover first tranche
    - `request-local require clause is outside stage 3 admit-slot first tranche`
    - `request-local ensure clause is outside stage 3 admit-slot first tranche`
    を actualize 済み
  - shared isolated predicate fragment helper の first tranche は actualize 済みであり、`e2` / `e3` / `e10` / `e11` anchor の predicate subset compare まで通っている
  - multiline attachment shape の docs-only comparison も済みであり、shared floor は clause suite 全体ではなく `<clause-head>:` + 直下 1 段深い predicate block という single attachment frame に留める judgment を固定した
  - shared single attachment frame の first tranche も actualize 済みであり、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline block extraction bridge を helper-local / test-only actual evidence として通した
  - request-local `require:` / `ensure:` の suite structural floor comparison も済みであり、current floor は `perform` owner の fixed two-slot suite に留める judgment を固定した
  - fixed two-slot suite floor の first actualization judgment も比較済みであり、current first choice は clause presence summary だけで止めず、`require_fragment_text` / `ensure_fragment_text` を持つ helper-local suite bridge を切ることである
  - fixed two-slot suite bridge の helper-local / test-only first tranche も actualize 済みであり、single-line / multiline mixed clause payload を同じ slot carrier に載せる line まで通した
  - `missing multiline ensure block` hidden path も helper-local / test-only actual evidence として actualize 済みである
  - current reopen judgment としては、remaining suite malformed wording family を suite helper 側でまだ追うより、fixture-side full request contract subset compare を narrow に reopen する方が自然である
  - current first-cut judgment としては、その compare を ad-hoc per-slot wiring に留めず、`Stage3RequestContractSubset` 相当の dedicated helper-local carrier へ切るのが自然である
  - `Stage3RequestContractSubset` helper-local / test-only first tranche も actualize 済みであり、same source-side suite carrier を `PerformOn` / `PerformVia` fixture の contract subset compare に通せる line まで source-backed に固定した
  - current guard judgment としては、その後の row-list widening は still early であり、current compare family は 0-or-1 guard に留める方が自然である
  - current sequencing judgment としては、この family は current tranche で一旦 freeze し、parser boundary staging と first checker cut 接点の re-sweep まで行ったうえで、current checkpoint では self-driven reopening をさらに積まず reserve path に戻す方が自然である
  - current side-selection judgment としては、その戻り先は parser boundary staging 側より first checker cut connection 側を先に取る方が自然であり、その reconnect line も freeze threshold まで整理済みである

## 残課題の優先順位

### Priority A — すぐ続けてよい

1. consistency / fairness / causal metadata catalog を room profile の working subset として比較する
2. static analysis / type / theorem prover / async-control boundary の small decidable core inventory を narrow に進める
3. detached validation loop は maintenance mode として drift suppression と smoke を継続し、`reference update / bless` は later candidate に留める
4. authoritative room baseline は checkpoint close として drift suppression を維持する

### Priority B — A の後でよい

1. Phase 3 reserve path を reopen する条件整理
2. detached exporter actual narrow API / storage policy の final cut
3. richer host interface に行く前の typed coverage carrier の narrow cut

### Priority C — user specification が必要

1. Mirrorea / Typed-Effect / Prism / shared space の詳細仕様
2. 上位アプリケーションや Reversed Library の domain-specific contract

## validation loop 入口までの見積もり

- **detached validation loop の入口自体は、すでに入っている**と見てよい。
- ただし「継続的に追加し続けても手戻りが小さい」状態までは、まだ次の refinement が必要である。
  1. fixture authoring / elaboration の friction を下げる実地反復を 1〜2 task
  2. detached exporter narrow API / storage candidate を実地 smoke で固める 1〜2 task
  3. shared-space / membership side line と static analysis inventory の stop line をもう少し明確にする 1〜2 task

rough estimate:
- validation loop を「継続運用しても手戻りが小さい」と言えるには **あと 2〜4 task**
- current L2 mainline を「数日単位でかなり自走できる」と言えるには **あと 4〜7 task**

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書 / decision level / invariants | 92% | 87% | 72% | 着手可能 | 基礎境界はかなり安定 |
| Mir current L2 core semantics | 86% | 80% | 78% | 着手可能 | current mainline には十分安定 |
| fallback / notation / representative examples | 86% | 82% | 68% | 着手可能 | examples drift はかなり抑えられている |
| parser-free PoC execution stack | 90% | 85% | 98% | 着手可能 | runtime / bundle / batch / selection / profile は揃っている |
| detached export / validation loop | 98% | 97% | 99% | 着手可能 | 入口は成立、current self-driven friction reduction は checkpoint close。残りは policy-dependent な bless/update 候補 |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 88% | 81% | 86% | 後段依存 | stage 1 / stage 2 / stage 3 の private staged spike は一区切りの freeze threshold まで整理済み。current checkpoint では reserve path |
| first checker cut / helper-local compare family | 89% | 79% | 88% | 後段依存 | reconnect subline は stage1/2 first tranches + freeze threshold まで揃い、current checkpoint では reserve path |
| richer host interface / typed coverage carrier | 45% | 32% | 25% | 後段依存 | current phase では太らせない |
| static analysis / type / theorem prover / async-control boundary | 36% | 26% | 12% | 着手可能 | hybrid staged approach を採る前提。`atomic_cut` の局所 cut と higher-level async control の切り分けを inventory 化する段階は自走可能 |
| shared-space / dynamic membership boundary | 79% | 72% | 9% | 着手可能 | docs-first boundary と example、tree-view vs registry、activation visibility、authority / consistency / RNG provider の比較に加え、resource owner slot / delegated capability / handoff epoch の working model、authoritative room baseline を `specs/examples/121...` に集約した current first choice、`specs/examples/122...` で切った small cross-room working subset row、`member_incarnation` と uncommitted action invalidation を room-profile 側に残す reconnect policy cut、plain vector deletion を避けて epoch / incarnation split を first practical candidate にする causal metadata cut、fairness trust model を `opaque authority trust` / `auditable authority witness` で分ける line、identity core と auth stack / admission policy を分ける line、admission policy / compile-time visibility を over-approximation + runtime control-plane で切る lineまでは進んだ。current self-driven mainline は final catalog ではなく witness / provider / causal stop line の refinement に移る |
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 16% | 11% | 5% | 要仕様確認 | higher-layer の具体仕様は依然 user からの追加仕様が必要 |

## 現時点での大きい未解決問題

1. fixture authoring / elaboration bottleneck
2. detached exporter の actual API / storage policy finalization
3. final parser syntax と companion notation の切り分け
4. richer host interface / typed coverage carrier
5. static analysis / type system / theorem prover / model checker / async-control boundary
6. shared-space / membership protocol / authority / consistency mode catalog / fairness source の finalization

## 次に進めるべき task

1. `auditable_authority_witness` の最小 witness shape を比べ、working subset row と deferred finalization の境界を保つ
2. static analysis / type / theorem prover / async-control boundary の small decidable core inventory を narrow に進める
3. detached validation loop は maintenance residual だけを見て、path policy / bless flow は later candidate に残す
4. authoritative room baseline は checkpoint close として drift を抑える
5. Phase 3 を reopen する場合は、current freeze line を壊さない later-pressure-driven subline だけを narrow に選ぶ

## 作業ログ（簡潔）

- 注記: **2026-04-06 09:19 JST 以降**の timestamp は `date` コマンドで取得した値を使う。以前の行は旧運用時の履歴である。
- 2026-04-06 08:44 JST — stage 3 admit-slot branch の carrier naming / compare surface を比較し、`decl_admit_slot` を第一候補にしつつ、fixture-side `admit` node へ direct lower せず、structural subset compare と slot retention smoke を分ける current judgment を固定した。次はこの cut を private helper の success-side first tranche として actualize する段階。
- 2026-04-06 08:52 JST — stage 3 admit-slot branch の success-side first tranche を actualize し、`e3` option / chain subset compare と `decl_admit_slot.surface_text` retention smoke を通した。次は malformed-source / request spillover のどちらを helper-local first tranche に入れるかを比較する段階。
- 2026-04-06 08:57 JST — stage 3 admit-slot branch の malformed-source pair を比較し、declaration-side `admit` payload 欠落と `PerformVia` spillover の 2 本だけを helper-local wording fragment smoke にする current judgment を固定した。次はこの pair を stage 3 private helper に actualize する段階。
- 2026-04-06 08:59 JST — stage 3 admit-slot branch の malformed-source first tranche を actualize し、`missing declaration-side admit slot payload` と `request head is outside stage 3 admit-slot first tranche` の 2 件を substring smoke として固定した。次は request-local clause spillover と fixture-side `admit` handoff のどちらを先に比べるかを決める段階。
- 2026-04-06 09:14 JST — stage 3 admit-slot branch の次段 sequencing を比較し、request-local clause spillover より先に fixture-side `OptionDecl.admit` handoff line を扱うのが自然だと整理した。次は handoff line 自体を actual compare に上げず、どこまで docs-only deferred に留めるかを詰める段階。
- 2026-04-06 09:18 JST — stage 3 admit-node handoff comparison を整理し、fixture-side `OptionDecl.admit` は already elaborated predicate node なので current phase では direct lowering も canonical surface compare も入れず、predicate fragment boundary が見えるまで docs-only deferred に留める判断を固定した。次は request-local `require` / `ensure` spillover を stage 3 later branch としてどこまで docs-only comparison に持つかを比べる段階。
- 2026-04-06 09:19 JST — stage 3 request-local clause spillover first tranche まで repo 状態を見直し、`progress.md` を snapshot 向けに再編した。次は stage 3 later branch の multiline attachment shape と predicate fragment reopen 条件のどちらを先に比較するかを narrow に決める段階。
- 2026-04-08 09:24 JST — shared-space / participant churn の boundary を repo source と blog 起点から再整理し、participant plain array を core に焼き込まず session-scoped membership registry + derived snapshot view を第一候補にする docs-first comparison と practical example を追加した。次はこの boundary を維持したまま current L2 mainline を続け、shared-space の final activation / consistency / auth は user 仕様確認で止める段階。
- 2026-04-08 09:34 JST — shared-space / membership boundary task を docs / plan / progress mirror まで閉じ、review でも substantive finding が出ないことを確認した。upper-layer は self-driven な boundary 整理まで進め、activation / authority / auth / consistency catalog の finalization では仕様確認待ちで止める状態。
- 2026-04-08 10:27 JST — shared-space 側の open questions を再整理し、participant の tree-like view は derived に留めて source of truth は registry に置くこと、activation visibility は compile-time over-approximation までで actual dissemination は runtime control-plane に残すこと、authority / consistency / RNG provider を別軸で比較する current working line を plan mirror に追記し、review 指摘に合わせて activation policy wording を boundary-safe に補正した。次は authority placement と consistency catalog の narrow docs-first comparison を進められる段階。
- 2026-04-08 10:54 JST — shared-space 側の authority / resource ownership / delegated capability / RNG provider placement の current working model を追加し、authoritative room では resource owner slot を 1 つに保ちつつ delegation を co-ownership にしない line を plan mirror に固定した。次は authority placement / consistency mode / RNG trust model の narrow comparison を続けられる段階。
- 2026-04-08 11:02 JST — shared-space authority / ownership comparisonの review 指摘を反映し、authoritative room 例で `owner slot` と `RNG provider placement` を再分離したうえで、membership registry が current source-of-truth model であることを report wording に明示した。次は activation rule と authority placement の narrow comparison を続けられる段階。
- 2026-04-08 11:05 JST — shared-space activation rule を `authority-ack` / `full-coverage-like` / `quorum-like` で比較し、authoritative room の最小 operational candidate を `authority-ack` に置きつつ、compile-time には visibility role の over-approximation だけを残す current working line を plan mirror に追記した。次は authoritative room の authority placement と consistency mode catalog をさらに narrow に比較できる段階。
- 2026-04-08 11:16 JST — shared-space authority placement を `single room authority` / `replicated authority` / `relaxed projection authority` で比較し、authoritative room の current first choice を `single room authority`、次候補を `replicated authority` に置く line を plan mirror に追記した。次は consistency mode catalog と RNG trust model の narrow comparison を続けられる段階。
- 2026-04-08 11:23 JST — shared-space authority placement comparison の review 指摘を反映し、progress snapshot を authoritative room scoped の current-phase candidate だと分かる wording に補正し、report / traceability hygiene を閉じた。次は consistency mode catalog と RNG trust model の narrow comparison を続けられる段階。
- 2026-04-08 11:32 JST — shared-space consistency mode catalog を `authoritative serial transition` / `append-friendly room` / `relaxed merge-friendly room` で比較し、authoritative room の current first choice と append-heavy room の first practical catalog を plan mirror に追記した。次は RNG trust model と fairness source placement の narrow comparison を続けられる段階。
- 2026-04-08 11:40 JST — shared-space RNG / fairness source placement を `authority_rng` / `delegated_rng_service` / `distributed_randomness_provider` で比較し、authoritative room の current first choice を `authority_rng`、append-heavy room の next practical candidate を `delegated_rng_service` とする line を plan mirror に追記した。次は activation / authority / consistency / fairness の 4 軸を authoritative game room の concrete profile にどう束ねるかを比較できる段階。
- 2026-04-08 11:54 JST — shared-space authoritative game room の concrete profile を `authority-ack` / `single room authority` / `authoritative serial transition` / `authority_rng` の minimal bundle と、RNG だけを `delegated_rng_service` に差し替える next practical bundle で整理し、append-friendly room との contrast も plan mirror に追記した。次は fairness / reconnect / auth をどこまで room profile に入れずに外部 policy に残すかをさらに narrow に比較できる段階。
- 2026-04-08 12:04 JST — shared-space の reconnect / late leave / in-flight action を room profile に全部入れず、`member_incarnation` と uncommitted action invalidation だけを minimal room-profile rule、timeout / retry / resend を external policy layer に残す cut を plan mirror に追記した。次は membership epoch / incarnation と causal metadata の接続を、plain vector deletion と対比しながら narrow に比較できる段階。
- 2026-04-08 12:13 JST — shared-space の causal metadata を plain vector deletion に寄せず、epoch / incarnation split を first practical candidate、control-plane separated carrier を next stronger candidate に置く line を plan mirror に追記した。次は identity / auth layering と fairness trust model のどちらを先に比較するかを narrow に決める段階。
- 2026-04-09 12:52 JST — 次に自走で進める順番を見直し、detached validation loop friction reduction → authoritative room baseline → consistency / fairness / causal metadata catalog comparison → static analysis / proof / async-control inventory の順を `tasks.md` / `progress.md` / `plan/11` / `plan/17` に揃えた。Phase 4 / 5 の docs-first package は current checkpoint では `着手可能` と読む。
- 2026-04-08 12:25 JST — shared-space の fairness trust model を `opaque authority trust` / `auditable authority witness` / `delegated provider attestation` / `distributed fairness protocol` で比較し、authoritative room では opaque trust を current minimal、auditable witness を next narrow strengthening candidate に置き、provider placement と witness requirement を別軸で扱う line を plan mirror に追記した。次は identity / auth layering を fairness / authority / membership boundary とどう分離するかを narrow に比較できる段階。
- 2026-04-08 12:30 JST — repo 全体の大局 phase / autonomy gate を `plan/17` に集約し、`progress.md` に current phase・重さ・自走可否の snapshot を追加した。次は shared-space の identity / auth layering を participant carrier / authority / fairness と混ぜずにどう切るかを narrow に比較する段階。
- 2026-04-08 12:36 JST — shared-space の identity / auth layering を `membership carrier に全部入れる` / `identity core と auth stack を分ける` / `opaque actor handle だけを room core に残す` で比較し、current first practical candidate を `identity core は membership registry、auth stack / admission policy は別 carrier` に置いた。次は admission policy と compile-time over-approximation の接点を、room capability / visibility requirement の line とどう切るかを narrow に比較する段階。
- 2026-04-08 12:45 JST — shared-space の admission policy / compile-time visibility を `runtime-only` / `declared over-approx + runtime admission` / `closed-world exact set` で比較し、current first practical candidate を `role / capability / visibility requirement は compile-time over-approx、actual admission / activation は runtime control-plane` に置いた。次は fairness witness と identity core / principal continuity の接点を、audit artifact line とどう切るかを narrow に比較する段階。
- 2026-04-08 12:59 JST — Phase0/1/2 closeout の consistency sweep として detached validation loop の helper surface を再確認し、`smoke-fixture` で E3/E6、`smoke-static-gate` で E4/E5 を回して bundle-first / aggregate / static-gate diff の current docs line と矛盾しないことを確認した。次はこの sweep evidence を report に固定し、必要な mirror drift だけを補正する段階。
- 2026-04-08 13:06 JST — Phase0/1/2 closeout smoke の report / document map / progress mirror を reviewer finding に合わせて補正し、detached validation loop の compare boundary と helper entry surface に concrete drift が無い状態で closeout できるところまで揃えた。次は README / Documentation / specs/examples / plan mirror 全体へ consistency sweep を広げる段階。
- 2026-04-08 13:10 JST — README / Documentation / `plan/00` / `plan/11` / `plan/17` / `progress.md` の top-level consistency sweep を入れ、detached validation loop の short summary を static gate artifact loop まで含む形へ揃え、Phase0/1/2 は maintenance tail・Phase3 は主線という current focus を mirror へ反映した。次は Phase3 later branch の次段比較へ戻る段階。
- 2026-04-08 13:38 JST — Phase 3 later branch の sequencing を比較し、request head + clause attachment multiline shape より先に predicate fragment boundary の reopen 条件を切る judgment を `specs/examples/92` と mirror へ固定し、late reviewer finding だった detached loop help wording / report placeholder も同 task で閉じた。次は minimal predicate fragment boundary の first docs-only cut を比較する段階。
- 2026-04-08 13:47 JST — Phase 3 later branch の predicate fragment boundary を shared isolated helper として reopen する judgment を `specs/examples/93` に固定し、`e2` / `e3` / `e10` / `e11` anchor の predicate subset compare を通す first tranche を private helper と test で actualize した。次は malformed-source pair と request head + clause attachment multiline shape のどちらを先に切るかを比較する段階。
- 2026-04-08 14:01 JST — Phase 3 later branch の next sequencing を比較し、predicate fragment helper の malformed-source pair より先に declaration-side `admit:` と request-local `require:` / `ensure:` の multiline attachment shape を shared structural floor として比べる judgment を `specs/examples/95` と mirror へ固定した。次は multiline attachment shape 自体の exact cut を docs-only で整理する段階。
- 2026-04-08 14:08 JST — Phase 3 later branch の multiline attachment shape を比較し、shared floor は clause suite 全体ではなく `<clause-head>:` + 直下 1 段深い predicate block の single attachment frame に留める judgment を `specs/examples/96` と mirror へ固定した。次はこの shared single attachment frame を helper-local / test-only actual evidence にどこまで actualize するかを比較する段階。
- 2026-04-08 14:23 JST — Phase 3 later branch の shared single attachment frame first tranche を actualize し、declaration-side `admit:` と request-local `require:` / `ensure:` の multiline block extraction bridge を helper-local / test-only actual evidence として通した。次は request-local clause suite completion と multiline attachment malformed-source pair の sequencing を比較する段階。
- 2026-04-09 13:20 JST — detached validation loop の second friction tranche として `compare-fixture-aggregates` を追加し、single-fixture aggregate 同士の compare を noisy な full-vs-single contrast から分離した。`scripts.tests.test_current_l2_detached_loop` と shorthand `e3` / `e6` aggregate compare を通し、next friction が reference update / longer compare triage 側に寄ることを確認した。
- 2026-04-08 14:30 JST — stage 3 multiline attachment first tranche の review tightening を反映し、clause header search を immediate child attachment line に限定し、blank line を fail-closed にして 8-test targeted suite と `cargo test -p mir-ast` を通した。次は request-local clause suite completion と multiline attachment malformed-source pair の sequencing を比較する段階。
- 2026-04-08 14:39 JST — Phase 3 later branch の next sequencing を比較し、multiline attachment malformed-source pair extension より先に request-local `require:` / `ensure:` の sibling clause suite structural floor を docs-only で比較する judgment を `specs/examples/98` と mirror へ固定した。次は sibling clause suite structural floor 自体の exact cut を整理する段階。
- 2026-04-08 14:47 JST — Phase 3 later branch の request-local clause suite structural floor を `perform` owner の fixed two-slot suite に留める judgment を `specs/examples/99` と mirror へ固定し、local diff inspection fallback で docs-only close を行った。次は fixed two-slot suite floor を helper-local / test-only structural compare にどこまで actualize するかを narrow に比較する段階。
- 2026-04-08 14:51 JST — Phase 3 later branch の fixed two-slot suite floor first actualization を比較し、clause presence summary だけで止めず `require_fragment_text` / `ensure_fragment_text` を持つ helper-local suite bridge を first choice に置いた。次はこの suite bridge first tranche を test-only helper と focused smoke で actualize する段階。
- 2026-04-08 14:53 JST — spec 100 と mirror / report chain を揃え、Phase 3 mainline の current next narrow step を fixed two-slot suite bridge first tranche actualization へ更新した。次は helper-local / test-only actual evidence と focused smoke を通す段階。
- 2026-04-08 15:03 JST — Phase 3 later branch の fixed two-slot suite bridge first tranche を helper-local / test-only helper と focused smoke で actualize し、single-line / multiline mixed clause payload compare と最小 structural fail-closed を `cargo test -p mir-ast` まで含めて通した。次は malformed/source family extension と fixture-side full request contract compare の sequencing を比較する段階。
- 2026-04-08 15:07 JST — Phase 3 later branch の next sequencing を比較し、fixture-side full request contract compare より先に request-local suite bridge family の helper-local malformed/source extension を docs-only で比較する judgment を mirror へ固定した。次は malformed/source pair の first cut を narrow に決める段階。
- 2026-04-08 16:11 JST — Phase 3 later branch の suite malformed/source first-pair comparison を整理し、`duplicate ensure` + unsupported direct child line を first actualization pair に置く judgment を mirror へ固定した。次はこの pair を helper-local / test-only actual evidence と focused smoke に上げる段階。
- 2026-04-08 16:11 JST — Phase 3 later branch の suite malformed/source first pair を focused smoke へ actualize し、`duplicate ensure` と unsupported direct child line の hidden fail-closed path を `cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike` と `cargo test -p mir-ast` で surfaced した。次は `missing multiline ensure block` family と fixture-side full request contract compare の sequencing を比較する段階。
- 2026-04-08 16:30 JST — Phase 3 later branch の next sequencing を比較し、first pair actualization の後も full request compare より先に `missing multiline ensure block` family を helper-local / test-only actual evidence に上げる judgment を mirror へ固定した。次はその hidden fail-closed path を focused smoke として surfaced する段階。
- 2026-04-08 16:43 JST — Phase 3 later branch の sequencing comparison 105 系列を canonical file 名と traceability addendum まで揃え、top-level docs から次段を `missing multiline ensure block` first-tranche actualization として辿れる状態にした。次はその focused smoke を追加して current hidden fail-closed path を surfaced する段階。
- 2026-04-08 16:50 JST — Phase 3 later branch の `missing multiline ensure block` hidden path を focused smoke へ actualize し、`cargo test -p mir-ast --test current_l2_stage3_request_clause_suite_spike` と `cargo test -p mir-ast` で 9-test suite まで通した。次は remaining suite malformed wording family と fixture-side full request contract compare の reopen 条件を比べる段階。
- 2026-04-08 17:17 JST — Phase 3 later branch の suite reopen 条件を比較し、remaining suite malformed wording family を still 追うより、request head parse を入れずに fixed two-slot suite bridge を fixture-side full request contract subset compare へ narrow に reopen する judgment を mirror へ固定した。次はその first actualization cut を比較する段階。
- 2026-04-08 17:30 JST — Phase 3 later branch の request contract subset compare cut を比較し、ad-hoc per-slot compare ではなく `Stage3RequestContractSubset` 相当の fixture-side expected carrier を helper-local / test-only first tranche として actualize する judgment を mirror へ固定した。次はその carrier を source-backed に actualize する段階。
- 2026-04-08 17:45 JST — Phase 3 later branch の `Stage3RequestContractSubset` first tranche を actualize し、same source-side suite carrier を `PerformOn` / `PerformVia` fixture の contract subset compare に通した。次は request head metadata を still later に残したまま contract-only compare surface の widening guard を比べる段階。
- 2026-04-08 17:55 JST — Phase 3 later branch の contract subset widening guard を比較し、current fixture corpus に multi-row clause array anchor が無いことから row-list widening を見送り、0-or-1 guard 維持を mirror へ固定した。次は source-side suite bridge widening の entry criteria を比べるか、この family を一旦 freeze して別 subline に戻るかを決める段階。
- 2026-04-08 18:05 JST — Phase 3 later branch の freeze sequencing を比較し、request contract subset family は current tranche で一旦 freeze し、次は parser boundary staging と first checker cut 接点の re-sweep に戻る judgment を mirror へ固定した。次はその 2 本のうちどちらから主線を再開するかを narrow に比較する段階。
- 2026-04-08 18:15 JST — Phase 3 の再開側選択を比較し、parser boundary staging 側の widening 再開より first checker cut connection 側から existing parser evidence の reconnect family を比べる方が自然だと mirror へ固定した。次はどの parser-boundary evidence family を first checker cut inventory へ最初に reconnect するかを narrow に比較する段階。
- 2026-04-08 18:29 JST — Phase 3 の first checker reconnect family を stage 1 chain / declaration structural floor に固定し、`Stage1ReconnectClusters` helper-local summary と `e13` / `e16` representative fixture compare を first tranche として actualize した。次は stage 1 reconnect family を `e18` / `e19` / `e20` まで widening するか、stage 2 `try` / rollback reconnect へ進むかを narrow に比較する段階。
- 2026-04-08 18:41 JST — Phase 3 の next reconnect step を stage 2 `try` / rollback structural floor に固定し、parser-side private summary を `checked_try_rollback_structural_*` に合わせて `e23` / `e24` malformed pair と valid one-shot `atomic_cut` in try body smoke まで actualize した。次は stage 2 reconnect family を `E21` / `E22` runtime contrast まで widening するか、stage 1 reconnect family の remaining widening (`e18` / `e19` / `e20`) に戻るかを narrow に比較する段階。
- 2026-04-08 18:57 JST — Phase 3 の次段は stage 2 runtime-contrast widening より先に stage 1 summary-preserving widening (`e18` / `e20`) を取る judgment に固定し、current `Stage1ReconnectClusters` contract を広げずに focused fixture compare と summary compare まで actualize した。次は `e19-malformed-target-mismatch` の summary contract と stage 2 `E21` / `E22` contrast threshold のどちらを先に比べるかを narrow に決める段階。
- 2026-04-08 19:13 JST — Phase 3 reconnect subline の threshold を比較し、`e19` は typed static reason family、`E21` / `E22` は runtime / proof boundary に残して current reconnect line 自体を freeze する judgment を mirror へ固定した。次は Phase 3 closeout sweep と current state summary を行う段階。
- 2026-04-08 19:28 JST — Phase 3 current tranche の closeout sweep を行い、top-level docs / plan / progress の phase reading を checkpoint 状態へ揃えた。現在は Phase 3 current tranche closeout 完了として一旦止めてよい段階。
- 2026-04-08 19:37 JST — Phase 3 closeout checkpoint の review 指摘を反映し、`plan/11` を historical appendix 明示へ補正、`plan/90` に closeout provenance を追記した。checkpoint wording の semantic overclaim は無く、commit / push ready の状態。
- 2026-04-08 20:04 JST — Phase 3 self-driven reopen threshold を docs-first で比較し、current checkpoint では Phase 3 を reserve path に戻す判断を `specs/examples/120` に固定しつつ、Phase 0〜3 の本質的成果を `docs/research_abstract/` に集約した。次は Phase 2 maintenance tail / Phase 4 side line / Phase 5 inventory line を主線として進める段階。
- 2026-04-09 12:29 JST — `atomic_cut` を core の最小 cut に留めつつ、higher-level async control は event-tree / authority-serial / witness-aware commit family として Phase 4 / 5 の inventory line で比較する方針を `tasks.md` と `plan/12` / `plan/13` / `plan/16` に追加した。次は shared-space 実例と small decidable core inventory の両側からこの boundary を絞る段階。
- 2026-04-09 12:53 JST — 次に自走で進める research package の順番を `detached validation loop → authoritative room baseline → consistency / fairness / causal metadata catalog → static analysis / proof / async-control inventory` に整理し、`tasks.md` / `progress.md` / `plan/11` / `plan/17` の immediate order を揃えた。次はこの順番どおり detached validation loop friction reduction から再開できる段階。
- 2026-04-09 13:07 JST — detached validation loop friction reduction の first tranche として、fixture stem shorthand・default run label derivation・missing fixture fail-fast を thin helper に actualize し、`scripts/tests/test_current_l2_detached_loop.py` と shorthand smoke で current baseline を通した。次は aggregate-noise / reference update / longer compare triage のどれを先に薄く改善するかを比較する段階。
- 2026-04-09 13:22 JST — detached validation loop friction reduction の second tranche として、`compare-fixture-aggregates` を追加し、single-fixture aggregate 同士の direct compare を noisy な full-vs-single contrast から分離した。`scripts/tests/test_current_l2_detached_loop.py`、shorthand aggregate compare、docs/plan/tasks mirror を通し、次の friction は reference update / bless と longer compare triage に寄った。
- 2026-04-09 14:53 JST — detached validation loop friction reduction の third tranche として bundle / aggregate / static gate diff helper の reference-only section を shallow per-field summary に揃え、longer compare triage を短くした。related unit test、E3/E6 compare smoke、docs/tasks/progress mirror を通し、current self-driven portion は checkpoint close、残りは policy-dependent な reference update / bless 候補に絞られた。
- 2026-04-09 15:35 JST — authoritative room baseline の docs-first 精密化を `specs/examples/121-shared-space-authoritative-room-baseline.md` に集約し、Phase 4 前半 package を checkpoint close に移して、current mainline を consistency / fairness / causal metadata catalog comparison へ更新した。review 指摘も反映したうえで `validate_docs` と `git diff --check` を通した。
- 2026-04-09 16:07 JST — shared-space catalog comparison の first cut として `specs/examples/122-shared-space-catalog-working-subset-comparison.md` を追加し、authoritative room / append-friendly room をまたぐ small working subset row と stop line を固定した。mirror も current mainline を `auditable_authority_witness` の最小 shape comparison へ揃え、`validate_docs` と `git diff --check` を通した。
