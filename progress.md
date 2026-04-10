# progress

最終更新: 2026-04-10 09:19 JST

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
- **shared-space / membership** は mainline ではないが、upper-layer docs-first boundary として「participant plain array を core に焼き込まず、session-scoped membership registry + derived snapshot view を第一候補にする」比較に加え、tree-like view を derived に留めること、activation visibility の compile-time over-approximation と runtime control-plane を分けること、authority / consistency / RNG provider を別軸で比較すること、room resource ごとの owner slot / delegated capability / handoff epoch を分けて読む current working modelを維持している。authoritative room では current baseline judgment を `specs/examples/121-shared-space-authoritative-room-baseline.md` に集約し、**authoritative room に限って** activation rule の current first choice を `authority-ack`、authority placement の current first choice を `single room authority`、consistency mode の current first choice を `authoritative serial transition`、RNG / fairness source の current first choice を `authority_rng` に固定した。さらに、その次段として `specs/examples/122-shared-space-catalog-working-subset-comparison.md` で authoritative room と append-friendly room をまたぐ small cross-room working subset row を切り、authoritative room baseline、authoritative room witnessed draw、append-friendly room baseline、append-friendly room with optional RNG capability を current subset に置いた。`auditable_authority_witness` の minimal witness core は `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` に切り出し、room profile には `fairness_claim` だけを残し、実体は `witness_kind + action_ref + draw_slot + draw_result` を持つ audit / receipt side の typed witness bundle に置く current first choice を固定した。さらに `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` で、authoritative room 側でも `delegated_rng_service` を provider-placement candidate として practical に読める current cut を整理し、authority は request / lock / commit / publish の owner のまま、provider は draw result だけを返し、provider receipt / draw ref は witness core ではなく audit / receipt side の optional attachment に留める judgment を固定した。`specs/examples/125-shared-space-control-plane-carrier-threshold.md` では、その次段として control-plane separated causal carrier を current default に reopen する threshold を比較し、authority handoff / provider binding / activation frontier を room rule 側へ上げない限り `membership_epoch + member_incarnation` を維持し、reopen するなら first cut は full control-plane log ではなく `control_epoch` 相当の lightweight split に留める current first choice を固定した。reconnect / late leave / in-flight action は room profile に全部入れず、`member_incarnation` と uncommitted action invalidation を minimal room-profile rule、timeout / retry / resend を external policy layer に残す lineまで進んだ。identity / auth layering 側では membership registry には identity core だけを残し、auth stack / admission policy は別 carrier に置く lineを current first practical candidate にした。admission policy / compile-time visibility 側では role / capability / visibility requirement の over-approximation だけを compile-time に残し、actual admission / activation / reconciliation は runtime control-plane に残す lineを current first practical candidate にした。したがって shared-space の current package は checkpoint close であり、stronger control-plane split 自体の actualization は later pressure が出たときだけ reopen 候補に残してよい。
- **async control / memory-model boundary** は、`atomic_cut` を place-local finalizing cut の最小核に留めたまま、higher-level ordering / fairness / consistency を event-tree / authority-serial transition / witness-aware commit family として Phase 4 / 5 の docs-first inventory へ送る line を current first practical candidate に置いた。C++ 的 low-level memory-order family は current immediate candidate にしていない。
- **Phase 5 small decidable core / proof / async-control boundary** は、`specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` から `specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md` までで theorem-line later package の current first choice を段階的に伸ばし、現時点では `archive_bless_update_policy_ref` までを symbolic retained bridge に寄せられるところまで整理した。したがって current package は theorem-line later package close とみなし、next later reopen は **retained archive payload comparison** の comparison に寄せてよい。
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
| Phase 4 | shared-space / membership / practical example boundary | authoritative baseline・working subset・minimal witness core・delegated-provider practical cut・control-plane threshold comparisonまで current package close | 重い | 着手可能 | baseline は `specs/examples/121...`、working subset は `122...`、minimal witness core は `123...`、delegated-provider practical cut は `124...`、threshold comparison は `125...`。stronger split は later reopen 候補 |
| Phase 5 | static analysis / type / theorem prover / model checker boundary | theorem-line later package close | とても重い | 着手可能 | `specs/examples/126...` で 4-way split、`specs/examples/127...` で proof-obligation matrix と mixed handoff sketch、`specs/examples/128...` で mixed row default / split threshold / emitter defer、`specs/examples/129...` で theorem first / protocol second / runtime later の pressure order、`specs/examples/130...` で theorem-side projection bundle、`specs/examples/131...` で typed symbolic `evidence_refs` family、`specs/examples/132...` で public checker migration defer threshold、`specs/examples/133...` で minimum contract row core、`specs/examples/134...` で `proof_notebook` first consumer、`specs/examples/135...` で `goal_text` attachment、`specs/examples/136...` で notebook bridge artifact を docs-only derived view に留める threshold、`specs/examples/137...` で next reopen order を notebook workflow first / `proof_assistant_adapter` second に固定、`specs/examples/138...` で workflow pressure の first threshold を review checklist / walkthrough に固定、`specs/examples/139...` で review unit を docs-only named bundle に寄せ、`specs/examples/140...` で bridge-level の docs-only sketch、`specs/examples/141...` で compare basis refs、`specs/examples/142...` で bless decision state、`specs/examples/143...` で review-note refs、`specs/examples/144...` で retained-notebook ref、`specs/examples/145...` で review-session ref、`specs/examples/146...` で `reviewed_by_ref + reviewed_at_ref`、`specs/examples/147...` で `review_session_state`、`specs/examples/148...` で `retention_state`、`specs/examples/149...` で `retained_path_policy_ref`、`specs/examples/150...` で `emitted_artifact_ref`、`specs/examples/151...` で `handoff_emitter_ref`、`specs/examples/152...` で `consumer_adapter_ref`、`specs/examples/153...` で `exchange_rule_ref`、`specs/examples/154...` で `adapter_validation_ref`、`specs/examples/155...` で `consumer_invocation_surface_ref`、`specs/examples/156...` で `exchange_rule_body_ref`、`specs/examples/157...` で `runtime_coupling_ref`、`specs/examples/158...` で `transport_protocol_ref`、`specs/examples/159...` で `failure_body_ref`、`specs/examples/160...` で `actual_invocation_protocol_ref`、`specs/examples/161...` で `consumer_host_binding_ref`、`specs/examples/162...` で `failure_wording_ref`、`specs/examples/163...` で `actual_runtime_handoff_ref`、`specs/examples/164...` で `emitted_invocation_receipt_ref`、`specs/examples/165...` で `runtime_transcript_ref`、`specs/examples/166...` で `materialized_runtime_handoff_ref`、`specs/examples/167...` で `concrete_payload_ref`、`specs/examples/168...` で `concrete_transcript_body_ref`、`specs/examples/169...` で `serialized_channel_body_ref`、`specs/examples/170...` で `emitted_attachment_body_ref`、`specs/examples/171...` で `emitted_attachment_blob_ref`、`specs/examples/172...` で `retained_file_body_ref`、`specs/examples/173...` で `archive_materialization_ref`、`specs/examples/174...` で `archive_body_ref`、`specs/examples/175...` で `archive_bundle_ref`、`specs/examples/176...` で `archive_bundle_manifest_ref`、`specs/examples/177...` で `archive_bundle_member_family_ref`、`specs/examples/178...` で `archive_member_body_compare_ref`、`specs/examples/179...` で `archive_bless_update_policy_ref` までを current first choice に固定。next は retained archive payload comparison |
| Phase 6 | actual parser / checker / runtime commitment | 未着手 | 重い | 後段依存 | Phase 2 / 3 / 5 の gate 後に入る |
| Phase 7 | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Typed-Effect / Prism / app contract は user 仕様待ち |

### 現在の主線

- **主線**: Phase 0 / 1 / 2 maintenance tail + cross-phase checkpoint maintenance
- **maintenance tail**: Phase 0 / 1 / 2
- **side line**: Phase 4 checkpoint maintenance / later reopen candidate
- **inventory line**: Phase 5 theorem-line later package close / later reopen candidate
- **まだ勝手に finalization しない**: final parser grammar、production exporter API、shared-space final catalog、higher-layer application contract

### immediate execution order

1. checkpoint ごとに drift suppression と mirror sweep を入れる
2. Phase 5 current package は checkpoint close として維持し、retained archive payload comparison が必要になったときだけ later pressure で reopen する
3. detached validation loop は maintenance mode に戻し、policy-dependent な `reference update / bless` だけを later candidate に残す
4. Phase 4 current package は checkpoint close として維持し、control-plane stronger split は later pressure が出たときだけ reopen する
5. Phase 3 は later pressure が出たときだけ reopen する

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

1. checkpoint close 済み package の drift suppression と mirror sweep を維持する
2. static analysis / type / theorem prover / async-control boundary は handoff actualization threshold だけを later reopen 候補として残す
3. detached validation loop は maintenance mode として drift suppression と smoke を継続し、`reference update / bless` は later candidate に留める
4. authoritative room baseline と working subset は checkpoint close として drift suppression を維持する

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
| static analysis / type / theorem prover / async-control boundary | 97% | 89% | 17% | 着手可能 | `specs/examples/126...` で 4-way split、`specs/examples/127...` で proof-obligation matrix と mixed handoff sketch、`specs/examples/128...` で mixed row default と boundary-specific split / actual emitter の reopen threshold、`specs/examples/129...` で theorem first / protocol second / runtime later の pressure order、`specs/examples/130...` で theorem-side projection bundle、`specs/examples/131...` で typed symbolic `evidence_refs` family、`specs/examples/132...` で public checker migration defer threshold、`specs/examples/133...` で minimum contract row core、`specs/examples/134...` で `proof_notebook` first consumer、`specs/examples/135...` で `goal_text` attachment、`specs/examples/136...` で notebook bridge artifact を docs-only derived view に留める threshold、`specs/examples/137...` で next reopen order を notebook workflow first / `proof_assistant_adapter` second に固定、`specs/examples/138...` で workflow pressure の first threshold を review checklist / walkthrough に固定、`specs/examples/139...` で review unit を docs-only named bundle に寄せ、`specs/examples/140...` で bridge-level の docs-only sketch、`specs/examples/141...` で compare basis refs、`specs/examples/142...` で bless decision state、`specs/examples/143...` で review-note refs、`specs/examples/144...` で retained-notebook ref、`specs/examples/145...` で review-session ref、`specs/examples/146...` で `reviewed_by_ref + reviewed_at_ref`、`specs/examples/147...` で `review_session_state`、`specs/examples/148...` で `retention_state`、`specs/examples/149...` で `retained_path_policy_ref`、`specs/examples/150...` で `emitted_artifact_ref`、`specs/examples/151...` で `handoff_emitter_ref`、`specs/examples/152...` で `consumer_adapter_ref`、`specs/examples/153...` で `exchange_rule_ref`、`specs/examples/154...` で `adapter_validation_ref`、`specs/examples/155...` で `consumer_invocation_surface_ref`、`specs/examples/156...` で `exchange_rule_body_ref`、`specs/examples/157...` で `runtime_coupling_ref`、`specs/examples/158...` で `transport_protocol_ref`、`specs/examples/159...` で `failure_body_ref`、`specs/examples/160...` で `actual_invocation_protocol_ref`、`specs/examples/161...` で `consumer_host_binding_ref`、`specs/examples/162...` で `failure_wording_ref`、`specs/examples/163...` で `actual_runtime_handoff_ref`、`specs/examples/164...` で `emitted_invocation_receipt_ref`、`specs/examples/165...` で `runtime_transcript_ref`、`specs/examples/166...` で `materialized_runtime_handoff_ref`、`specs/examples/167...` で `concrete_payload_ref`、`specs/examples/168...` で `concrete_transcript_body_ref`、`specs/examples/169...` で `serialized_channel_body_ref`、`specs/examples/170...` で `emitted_attachment_body_ref`、`specs/examples/171...` で `emitted_attachment_blob_ref`、`specs/examples/172...` で `retained_file_body_ref`、`specs/examples/173...` で `archive_materialization_ref`、`specs/examples/174...` で `archive_body_ref`、`specs/examples/175...` で `archive_bundle_ref`、`specs/examples/176...` で `archive_bundle_manifest_ref`、`specs/examples/177...` で `archive_bundle_member_family_ref`、`specs/examples/178...` で `archive_member_body_compare_ref`、`specs/examples/179...` で `archive_bless_update_policy_ref` までを current first choice に固定。next は retained archive payload comparison |
| shared-space / dynamic membership boundary | 82% | 75% | 11% | 着手可能 | docs-first boundary と example、tree-view vs registry、activation visibility、authority / consistency / RNG provider の比較に加え、resource owner slot / delegated capability / handoff epoch の working model、authoritative room baseline を `specs/examples/121...` に集約した current first choice、`specs/examples/122...` の small cross-room working subset row、`specs/examples/123...` の minimal witness core、`specs/examples/124...` の delegated-provider practical cut、`specs/examples/125...` の control-plane threshold comparison まで current package として切った。current promoted line は final catalog ではなく later reopen candidate と Phase 5 inventory に移る |
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 16% | 11% | 5% | 要仕様確認 | higher-layer の具体仕様は依然 user からの追加仕様が必要 |

## 現時点での大きい未解決問題

1. fixture authoring / elaboration bottleneck
2. detached exporter の actual API / storage policy finalization
3. final parser syntax と companion notation の切り分け
4. richer host interface / typed coverage carrier
5. static analysis / type system / theorem prover / model checker / async-control boundary
6. shared-space / membership protocol / authority / consistency mode catalog / fairness source の finalization

## 次に進めるべき task

1. checkpoint close 済み package の drift suppression と mirror sweep を先に回す
2. Phase 5 を reopen する場合は、actual archive member body compare をどこまで theorem-line bridge に寄せるかを先に narrow に扱う
3. detached validation loop は maintenance residual だけを見て、path policy / bless flow は later candidate に残す
4. Phase 4 current package は checkpoint close として drift を抑え、control-plane stronger split は later pressure が出たときだけ reopen する
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
- 2026-04-10 04:08 JST — Phase 5 theorem-line later package として `specs/examples/166...` を closeout し、`materialized_runtime_handoff_ref` を current first choice に固定したうえで、reviewer の mirror/traceability drift 指摘も反映した。next later reopen は concrete payload / transcript body comparison。
- 2026-04-10 04:26 JST — Phase 5 theorem-line later package として `specs/examples/167...` を closeout し、`concrete_payload_ref` を current first choice に固定したうえで、review report / traceability / phase snapshot の stale wording を reviewer finding に合わせて補正した。next later reopen は concrete transcript body comparison。
- 2026-04-10 04:33 JST — Phase 5 theorem-line later package として `specs/examples/168...` を追加し、`concrete_transcript_body_ref` を current first choice に固定した。next later reopen は actual serialized channel body comparison。
- 2026-04-10 04:40 JST — `168` package の review closeout を反映し、Phase 5 mirror / traceability / report hygiene を actual serialized channel body comparison 向けの current snapshot に揃えた。次は serialized body reopen の narrow comparison に進める状態。
- 2026-04-10 04:49 JST — Phase 5 theorem-line later package として `specs/examples/169...` を追加し、`serialized_channel_body_ref` を current first choice に固定した。next later reopen は actual emitted attachment blob / file body comparison。
- 2026-04-10 05:17 JST — `169` package の review closeout を反映し、Phase 5 mirror / report hygiene を actual emitted attachment blob / file body comparison 向けの current snapshot に揃えた。semantic split 自体は clean で、next later reopen は emitted attachment blob / file body comparison に固定した。
- 2026-04-10 05:27 JST — Phase 5 theorem-line later package として `specs/examples/170...` を追加し、`emitted_attachment_body_ref` を current first choice に固定した。next later reopen は actual emitted attachment blob / file materialization comparison。
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
- 2026-04-10 08:57 JST — Phase 5 theorem-line later package を `178` まで伸ばし、`archive_member_body_compare_ref` を symbolic retained bridge の current first choice に追加した。reviewer finding だった stale mirror drift 4 件も補正し、next later reopen は actual archive bless / update policy comparison に切り替わった。
- 2026-04-10 09:16 JST — Phase 5 theorem-line later package を `179` まで伸ばし、`archive_bless_update_policy_ref` を symbolic retained bridge の current first choice に追加した。next later reopen は retained archive payload comparison に切り替わり、Phase 5 snapshot / tasks / roadmap mirror もそれに合わせて更新した。
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
- 2026-04-09 16:11 JST — shared-space catalog comparison の first cut として `specs/examples/122-shared-space-catalog-working-subset-comparison.md` を追加し、authoritative room / append-friendly room をまたぐ small working subset row と stop line を固定した。mirror も current mainline を `auditable_authority_witness` の最小 shape comparison へ揃え、review / `validate_docs` / `git diff --check` を通した。
- 2026-04-09 16:19 JST — `auditable_authority_witness` の minimal witness core を `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` に切り出し、room profile の claim と audit / receipt side の typed witness bundle を分離した。current first choice は `witness_kind + action_ref + draw_slot + draw_result` で、次の主線は authoritative room 側の `delegated_rng_service` practical cut comparison に移った。
- 2026-04-09 17:05 JST — authoritative room 側の `delegated_rng_service` practical cut を `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` に固定し、authority commit ownership を維持したまま provider receipt を optional audit attachment に留める judgmentを mirror へ反映した。review 指摘も反映し、次の主線は control-plane separated causal carrier threshold comparison に移った。
- 2026-04-09 17:50 JST — control-plane separated causal carrier threshold を `specs/examples/125-shared-space-control-plane-carrier-threshold.md` に固定し、`membership_epoch + member_incarnation` を current default に維持する条件と `control_epoch` 相当の first reopen cut 条件を切り分けた。Phase 4 current package は checkpoint close とし、次の promoted line は Phase 5 inventory に移した。
- 2026-04-09 18:33 JST — Phase 5 の small decidable core / proof / async-control boundary を `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` に集約し、`core_static_checker` / `theorem_prover_boundary` / `protocol_verifier_boundary` / `runtime_policy_boundary` の 4-way split を current first choice に固定した。Phase 5 current package は first inventory package close とし、次は drift suppression と later reopen candidate 管理に移れる状態になった。
- 2026-04-09 19:35 JST — Phase 5 later package として `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` を追加し、proof-obligation matrix と mixed handoff row sketch を current first choice に固定した。review finding 2 件も反映し、Phase 5 は second inventory package close として checkpoint maintenance / later reopen 候補へ移れる状態になった。
- 2026-04-09 20:55 JST — Phase 5 later reopen threshold として `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` を追加し、mixed row bundle を current docs-only default に維持しつつ boundary-specific split / actual emitter は concrete consumer pressure が出たときだけ reopen する判断を mirror へ反映した。Phase 5 は third inventory package close と読み替え、reviewer completion で no findings を確認したうえで checkpoint maintenance に戻した。
- 2026-04-09 21:04 JST — Phase 5 の next later reopen 候補として `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` を追加し、first concrete external consumer pressure の current first practical candidate を `theorem_prover_boundary` に置いた。Phase 5 は fourth inventory package close と読み替え、next later reopen を theorem line narrow actualization に絞った。review completion は回収できなかったため `0386` に local evidence fallback を記録した。
- 2026-04-09 21:15 JST — Phase 5 theorem-line later package として `specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` と `specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` を追加し、mixed row default を維持したまま theorem-side projection bundle と typed symbolic `evidence_refs` family を current first choice に固定した。Phase 5 は theorem-line later package close と読み替え、next later reopen を public checker migration threshold comparison に更新した。
- 2026-04-09 21:20 JST — Phase 5 theorem-line later package の次段として `specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` を追加し、public checker migration は concrete theorem consumer pressure が出るまで deferred に保つ current threshold を固定した。Phase 5 は theorem-line later package close を維持したまま、next later reopen を minimum contract row comparison に更新した。
- 2026-04-09 21:38 JST — Phase 5 theorem-line later package の次段として `specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` を追加し、minimum contract row core を `obligation_kind + evidence_refs` に留め、`goal_text` / `proof_hint` / `consumer_hint` は later attachment に残す current cut を固定した。review finding 1 件も反映し、Phase 5 は theorem consumer class / attachment family comparison を later reopen 候補にしたまま checkpoint close を維持している。
- 2026-04-09 21:49 JST — Phase 5 theorem-line later package の次段として `specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` と `specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` を追加し、`proof_notebook` を first practical consumer class、`goal_text` を notebook first bridge の current lightweight attachment に固定した。review finding 2 件も反映し、Phase 5 は notebook bridge artifact / stable contract threshold comparison を next later reopen 候補にしたまま checkpoint close を維持している。
- 2026-04-09 22:04 JST — Phase 5 theorem-line later package の次段として `specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` を追加し、current phase では notebook bridge artifact を named family にせず docs-only derived view に留める threshold を固定した。Phase 5 は theorem-line later package close を維持したまま、next later reopen を concrete notebook workflow pressure または `proof_assistant_adapter` consumer pressure comparison に更新した。
- 2026-04-09 22:25 JST — Phase 5 theorem-line later package の次段として `specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` を追加し、next practical reopen は concrete notebook workflow pressure comparison、`proof_assistant_adapter` consumer pressure は second practical candidateに留める current order を固定した。Phase 5 は theorem-line later package close を維持したまま、next later reopen を notebook workflow pressure の最小条件 comparison に更新した。
- 2026-04-09 22:52 JST — Phase 5 theorem-line later package の次段として `specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` と `specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` を整理し、workflow pressure の current first threshold を review checklist / walkthrough に固定したうえで、review unit を docs-only named bundle に寄せる current first choiceまで進めた。138 package の stale provenance / report closeout も補正し、次は named review unit を stronger notebook bridge sketch へどこまで拡張するかを narrow に比べる段階。
- 2026-04-09 23:02 JST — Phase 5 theorem-line later package の次段として `specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` を追加し、docs-only named review unit の上に `bridge_subject_ref + review_units + bridge_goal_text` を持つ notebook bridge sketch まで寄せる current first choiceを固定した。next later reopen は bridge sketch に compare / bless-like review flow metadata をどこまで足すかの comparison へ更新した。
- 2026-04-09 23:22 JST — Phase 5 theorem-line later package の次段として `specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` と `specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` を整理し、bridge sketch に `comparison_basis_refs`、その次段に `bless_decision_state` までを current first choice として固定した。next later reopen は bless-ready bridge sketch に review-session metadata をどこまで足すかの comparison へ更新した。
- 2026-04-09 23:24 JST — Phase 5 theorem-line later package の次段として `specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` を整理し、bless-ready bridge sketch の上に `review_note_refs` までを current first choice として固定した。next later reopen は review-linked bless bridge に retained notebook path / review session lifecycle をどこまで足すかの comparison へ更新した。
- 2026-04-09 23:32 JST — Phase 5 theorem-line bridge package の review closeout を取り、141/142/143 の ratchet を `comparison_basis_refs` → `bless_decision_state` → `review_note_refs` の順で mirror / provenance まで揃えた。next later reopen は review-linked bless bridge に retained notebook path / review session lifecycle をどこまで足すかの comparison に更新した。
- 2026-04-09 23:58 JST — Phase 5 theorem-line later package の次段として `specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` と `specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` を整理し、`retained_notebook_ref` と `review_session_ref` までを current first choice に固定した。reviewer の mirror drift 3 件も反映し、next later reopen は session-linked retained bridge に actor / timestamp / lifecycle state をどこまで足すかの comparison に更新した。
- 2026-04-10 00:05 JST — Phase 5 theorem-line later package の次段として `specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` と `specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` を整理し、`reviewed_by_ref + reviewed_at_ref` と `review_session_state` までを current first choice に固定した。next later reopen は retention state / actual retained path policy / emitted artifact threshold の comparison に更新した。
- 2026-04-10 00:16 JST — Phase 5 theorem-line later package の次段として `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` と `specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` を整理し、`retention_state` と `retained_path_policy_ref` までを current first choice に固定した。next later reopen は actual emitted notebook artifact threshold の comparison に更新した。
- 2026-04-10 00:33 JST — Phase 5 theorem-line later package の次段として `specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` を整理し、path-ready retained bridge に `emitted_artifact_ref` までを current first choice に固定した。next later reopen は actual handoff emitter threshold の comparison に更新した。
- 2026-04-10 00:47 JST — Phase 5 theorem-line later package の次段として `specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` を整理し、emitted-ready retained bridge に `handoff_emitter_ref` までを current first choice に固定した。next later reopen は consumer adapter threshold の comparison に更新した。
- 2026-04-10 01:00 JST — Phase 5 theorem-line later package の次段として `specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` を整理し、emitter-linked retained bridge に `consumer_adapter_ref` までを current first choice に固定した。next later reopen は notebook exchange rule threshold の comparison に更新した。
- 2026-04-10 01:15 JST — Phase 5 theorem-line later package の次段として `specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` を整理し、adapter-linked retained bridge に `exchange_rule_ref` までを current first choice に固定した。next later reopen は adapter-validation threshold の comparison に更新した。
- 2026-04-10 01:27 JST — Phase 5 theorem-line later package の次段として `specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` を整理し、exchange-ready retained bridge に `adapter_validation_ref` までを current first choice に固定した。next later reopen は invocation-surface threshold の comparison に更新した。
- 2026-04-10 01:52 JST — Phase 5 theorem-line later package の次段として `specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` を整理し、validation-ready retained bridge に `consumer_invocation_surface_ref` までを current first choice に固定した。next later reopen は exchange-body threshold の comparison に更新した。
- 2026-04-10 01:56 JST — Phase 5 theorem-line later package の次段として `specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` を整理し、invocation-ready retained bridge に `exchange_rule_body_ref` までを current first choice に固定した。next later reopen は runtime-coupling threshold の comparison に更新した。
- 2026-04-10 02:14 JST — Phase 5 theorem-line later package の次段として `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` を整理し、exchange-body-ready retained bridge に `runtime_coupling_ref` までを current first choice に固定した。next later reopen は transport / failure threshold の comparison に更新した。
- 2026-04-10 02:29 JST — `specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` の reviewer 指摘を反映し、practical examples を 156 からの additive change に補正したうえで runtime-coupling threshold package を close した。
- 2026-04-10 02:29 JST — Phase 5 theorem-line later package の次段として `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` を整理し、runtime-coupling-ready retained bridge に `transport_protocol_ref` までを current first choice に固定した。next later reopen は failure-body threshold の comparison に更新した。
- 2026-04-10 02:43 JST — `specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` の review 反映を終え、transport-protocol threshold package を close した。
- 2026-04-10 02:58 JST — Phase 5 theorem-line later package の次段として `specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` を整理し、transport-ready retained bridge に `failure_body_ref` までを current first choice に固定した。next later reopen は actual-invocation-protocol threshold の comparison に更新した。
- 2026-04-10 03:17 JST — Phase 5 theorem-line later tranche として `specs/examples/160...` / `161...` / `162...` を追加し、retained bridge に `actual_invocation_protocol_ref`、`consumer_host_binding_ref`、`failure_wording_ref` までを段階的に current first choice として固定した。next later reopen は actual notebook runtime handoff actualization comparison に更新した。
- 2026-04-10 03:20 JST — Phase 5 theorem-line later tranche として `specs/examples/163...` / `164...` / `165...` を追加し、retained bridge に `actual_runtime_handoff_ref`、`emitted_invocation_receipt_ref`、`runtime_transcript_ref` までを段階的に current first choice として固定した。next later reopen は actual notebook runtime handoff materialization comparison に更新した。
- 2026-04-10 03:48 JST — Phase 5 theorem-line later tranche として `specs/examples/166...` を追加し、retained bridge に `materialized_runtime_handoff_ref` までを current first choice として固定した。next later reopen は concrete payload / transcript body comparison に更新した。
- 2026-04-10 05:53 JST — Phase 5 theorem-line later package の次段として `specs/examples/171-current-l2-theorem-line-attachment-body-ready-emitted-attachment-blob-threshold.md` を整理し、retained bridge に `emitted_attachment_blob_ref` までを current first choice として固定した。next later reopen は actual retained file body / archive materialization comparison に更新した。
- 2026-04-10 06:31 JST — Phase 5 theorem-line later package の次段として `specs/examples/172-current-l2-theorem-line-attachment-blob-ready-retained-file-body-threshold.md` を整理し、retained bridge に `retained_file_body_ref` までを current first choice として固定した。next later reopen は actual archive materialization comparison に更新した。
- 2026-04-10 06:44 JST — Phase 5 theorem-line retained-file-body package の review closeout を反映し、`plan/11` と Phase 5 table の stale wording を `172...` / actual archive materialization comparison に揃えた。package 自体の semantic cut はそのまま維持した。
- 2026-04-10 06:48 JST — Phase 5 theorem-line later package の次段として `specs/examples/173-current-l2-theorem-line-retained-file-body-ready-archive-materialization-threshold.md` を整理し、retained bridge に `archive_materialization_ref` までを current first choice として固定した。next later reopen は actual archive body / bundle comparison に更新した。
- 2026-04-10 07:12 JST — Phase 5 theorem-line archive-materialization package の review closeout を反映し、`plan/11` と `plan/17` の chain summary / next-step wording を `173...` close と actual archive body / bundle comparison に揃えた。
- 2026-04-10 07:12 JST — Phase 5 theorem-line later package の次段として `specs/examples/174-current-l2-theorem-line-archive-materialization-ready-archive-body-bundle-threshold.md` を整理し、retained bridge に `archive_body_ref` までを current first choice として固定した。next later reopen は actual archive bundle comparison に更新した。
- 2026-04-10 07:40 JST — Phase 5 theorem-line later package の次段として `specs/examples/175-current-l2-theorem-line-archive-body-ready-archive-bundle-threshold.md` を整理し、retained bridge に `archive_bundle_ref` までを current first choice として固定した。next later reopen は actual archive bundle manifest / member family comparison に更新した。
- 2026-04-10 07:54 JST — Phase 5 theorem-line later package の次段として `specs/examples/176-current-l2-theorem-line-archive-bundle-ready-archive-manifest-threshold.md` を整理し、retained bridge に `archive_bundle_manifest_ref` までを current first choice として固定した。next later reopen は actual archive bundle member family comparison に更新した。
- 2026-04-10 08:36 JST — Phase 5 theorem-line later package の次段として `specs/examples/177-current-l2-theorem-line-archive-manifest-ready-archive-member-family-threshold.md` を整理し、retained bridge に `archive_bundle_member_family_ref` までを current first choice として固定した。review finding だった `plan/11` の stale reopen wording と report evidence gap も補正し、next later reopen は actual archive member body compare comparison に更新した。
- 2026-04-10 09:19 JST — Phase 5 theorem-line later package の次段として `specs/examples/179-current-l2-theorem-line-archive-member-body-compare-ready-archive-bless-update-policy-threshold.md` を整理し、retained bridge に `archive_bless_update_policy_ref` までを current first choice として固定した。reviewer は substantive finding なしで、next later reopen は retained archive payload comparison に更新した。
