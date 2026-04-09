# plan/11 — 近接ロードマップ

## 目的

この文書は、今から数 task 先までの near-term roadmap を示す。
ここに書く step 数や task 数は厳密な約束ではなく、**rough estimate** である。

current immediate execution order は `plan/17-research-phases-and-autonomy-gates.md` と `progress.md` の phase section を優先する。baseline closeout と top-level consistency sweep の first pass は fixed しており、reconnect subline も freeze threshold まで整理でき、さらに current checkpoint では **Phase 3 self-driven portion を reserve path に戻す threshold** まで固定した。その後、Phase 4 の current package も `specs/examples/125...` までで checkpoint close に入り、Phase 5 入口の small decidable core / proof / async-control inventory も `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` で first package close、`specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` で second package、`specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` で third package の threshold cut、`specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md` で first concrete consumer pressure の current first practical candidate、`specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md` で theorem-side docs-only projection bundle、`specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md` で typed symbolic `evidence_refs` family、`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md` で public checker migration を deferred に保つ threshold、`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md` で minimum contract row core を `obligation_kind + evidence_refs` に留める current first choice、`specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md` で `proof_notebook` を first practical consumer class に置く current first choice、`specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md` で notebook first bridge の attachment を `goal_text` に留める current first choice、`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md` で named notebook bridge artifact は current phase では導入せず docs-only derived view に留める threshold、`specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md` で next practical reopen order を concrete notebook workflow pressure first / `proof_assistant_adapter` pressure second に置く current order、`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` で concrete notebook workflow pressure の current first threshold を human review checklist / walkthrough pressure に置き、`specs/examples/139-current-l2-theorem-line-notebook-review-unit-named-bundle-threshold.md` で review unit を docs-only named bundle に寄せ、`specs/examples/140-current-l2-theorem-line-review-unit-to-bridge-sketch-comparison.md` で bridge-level の docs-only sketch に寄せ、`specs/examples/141-current-l2-theorem-line-bridge-sketch-compare-metadata-threshold.md` で compare basis refs までを、`specs/examples/142-current-l2-theorem-line-compare-ready-bridge-bless-decision-threshold.md` で bless decision state までを、`specs/examples/143-current-l2-theorem-line-bless-ready-bridge-review-session-threshold.md` で review-note refs までを、`specs/examples/144-current-l2-theorem-line-review-linked-bless-bridge-retained-notebook-threshold.md` で retained-notebook ref までを、`specs/examples/145-current-l2-theorem-line-retained-bridge-review-session-link-threshold.md` で review-session ref までを、`specs/examples/146-current-l2-theorem-line-session-linked-retained-bridge-review-observation-threshold.md` で `reviewed_by_ref + reviewed_at_ref` までを、`specs/examples/147-current-l2-theorem-line-observed-session-lifecycle-threshold.md` で `review_session_state` までを、`specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md` で `retention_state` までを、`specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md` で `retained_path_policy_ref` までを、`specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md` で `emitted_artifact_ref` までを、`specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md` で `handoff_emitter_ref` までを、`specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md` で `consumer_adapter_ref` までを、`specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md` で `exchange_rule_ref` までを、`specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md` で `adapter_validation_ref` までを、`specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md` で `consumer_invocation_surface_ref` までを、`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md` で `exchange_rule_body_ref` までを、`specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md` で `runtime_coupling_ref` までを、`specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md` で `transport_protocol_ref` までを、`specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md` で `failure_body_ref` までを、`specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md` で `actual_invocation_protocol_ref` までを、`specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md` で `consumer_host_binding_ref` までを、`specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md` で `failure_wording_ref` までを、`specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md` で `actual_runtime_handoff_ref` までを、`specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md` で `emitted_invocation_receipt_ref` までを、`specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md` で `runtime_transcript_ref` までを current first choice に置くところまで進んだ。したがって、**現在は Phase 0 / 1 / 2 を maintenance tail として維持しつつ、Phase 4 / 5 の current package は checkpoint maintenance を主とし、Phase 5 later reopen は actual notebook runtime handoff materialization comparison が必要になったときだけ narrow に再開する** のが自然である。

## いまから数 task の主眼

近い数 task の目的は、Phase 0 / 1 / 2 の closeout baseline を壊さずに current L2 の主線を次へ進めることである。

- parser-free PoC を継続的に回せる
- parser-free PoC の実行結果を process 内比較だけに閉じ込めない
- trace / audit と host coverage の operational boundary を semantics から独立に狭く切る
- notation の比較結果が docs / tests / fixtures と整合している
- helper stack の mirror drift が抑えられている
- parser / checker public boundary を早く既成事実化せずに進められる

## 次に自走で進める順番

### 1. consistency / fairness / causal metadata catalog を working subset として比較する

- authoritative room baseline の current first choice は `specs/examples/121-shared-space-authoritative-room-baseline.md` までで checkpoint close とみなしてよい
- current first tranche として、authoritative room / append-friendly room をまたぐ small working subset row は `specs/examples/122-shared-space-catalog-working-subset-comparison.md` までで切ってよい
- room mode catalog を final fixed catalog と見なさず、working subset と deferred finalization に分ける
- `auditable_authority_witness` の最小 witness shape は `specs/examples/123-shared-space-auditable-authority-witness-minimal-shape.md` までで切ってよい
- `delegated_rng_service` を authoritative room 側でも provider-placement candidate としてどこまで practical に読めるかの comparison は `specs/examples/124-shared-space-authoritative-room-delegated-rng-provider-placement.md` までで current first choice を切った
- control-plane separated causal carrier を authoritative room side line に reopen する threshold comparison も `specs/examples/125-shared-space-control-plane-carrier-threshold.md` までで current first choice を切った
- current checkpoint では、この package は checkpoint close とみなし、stronger control-plane split の actualization は later pressure が出たときだけ reopen 候補に戻す
- append-friendly room と authoritative room の contrast を崩さずに catalog の stop line を増やす
- rough weight: 重
- rough 所要: 3〜6 task / 4〜10日

### 2. static analysis / type / theorem prover / async-control boundary の inventory を進める

- local / structural / decidable 寄りの floor をどこまで core に入れるかを narrow に比べる
- parser boundary / first checker cut / detached validation loop と衝突しない small decidable core inventory を先に作る
- theorem prover / model checker 側へ残す global property を current docs に合わせて明確化する
- `atomic_cut` を最小核に留め、higher-level async-control family を docs-first に比較する
- current first package は `specs/examples/126-current-l2-small-decidable-core-and-proof-boundary-inventory.md` に集約済みであり、current checkpoint では
  - `core_static_checker`
  - `theorem_prover_boundary`
  - `protocol_verifier_boundary`
  - `runtime_policy_boundary`
  の 4-way split を current first choice とみなしてよい
- current docs-only later package は `specs/examples/127-current-l2-proof-obligation-matrix-and-external-handoff-artifact.md` と `specs/examples/128-current-l2-handoff-artifact-threshold-comparison.md` までで切ってよい
- current later reopen package は `specs/examples/129-current-l2-first-external-consumer-pressure-comparison.md`、`specs/examples/130-current-l2-theorem-line-narrow-actualization-comparison.md`、`specs/examples/131-current-l2-theorem-line-evidence-ref-family-comparison.md`、`specs/examples/132-current-l2-theorem-line-public-checker-migration-threshold.md`、`specs/examples/133-current-l2-theorem-line-minimum-contract-row-comparison.md`、`specs/examples/134-current-l2-theorem-line-consumer-class-comparison.md`、`specs/examples/135-current-l2-theorem-line-notebook-attachment-family-comparison.md`、`specs/examples/136-current-l2-theorem-line-notebook-bridge-artifact-threshold.md`、`specs/examples/137-current-l2-theorem-line-next-consumer-pressure-comparison.md`、`specs/examples/138-current-l2-theorem-line-concrete-notebook-workflow-pressure-comparison.md` までで切ってよい
- current later reopen package は `specs/examples/148-current-l2-theorem-line-lifecycle-ready-retention-state-threshold.md`、`specs/examples/149-current-l2-theorem-line-retention-ready-path-policy-threshold.md`、`specs/examples/150-current-l2-theorem-line-path-ready-emitted-artifact-threshold.md`、`specs/examples/151-current-l2-theorem-line-emitted-ready-handoff-emitter-threshold.md`、`specs/examples/152-current-l2-theorem-line-emitter-linked-consumer-adapter-threshold.md`、`specs/examples/153-current-l2-theorem-line-adapter-linked-exchange-rule-threshold.md`、`specs/examples/154-current-l2-theorem-line-exchange-ready-adapter-validation-threshold.md`、`specs/examples/155-current-l2-theorem-line-validation-ready-invocation-surface-threshold.md`、`specs/examples/156-current-l2-theorem-line-invocation-ready-exchange-body-threshold.md`、`specs/examples/157-current-l2-theorem-line-exchange-body-ready-runtime-coupling-threshold.md`、`specs/examples/158-current-l2-theorem-line-runtime-coupling-ready-transport-protocol-threshold.md`、`specs/examples/159-current-l2-theorem-line-transport-ready-failure-body-threshold.md`、`specs/examples/160-current-l2-theorem-line-failure-ready-actual-invocation-protocol-threshold.md`、`specs/examples/161-current-l2-theorem-line-invocation-ready-host-binding-threshold.md`、`specs/examples/162-current-l2-theorem-line-binding-ready-failure-wording-threshold.md`、`specs/examples/163-current-l2-theorem-line-wording-ready-runtime-handoff-threshold.md`、`specs/examples/164-current-l2-theorem-line-runtime-handoff-ready-invocation-receipt-threshold.md`、`specs/examples/165-current-l2-theorem-line-invocation-receipt-ready-runtime-transcript-threshold.md` までで切ってよく、lifecycle-ready retained bridge には `retention_state` を、retention-ready retained bridge には `retained_path_policy_ref` を、path-ready retained bridge には `emitted_artifact_ref` を、emitted-ready retained bridge には `handoff_emitter_ref` を、adapter-linked retained bridge には `consumer_adapter_ref` を、exchange-ready retained bridge には `exchange_rule_ref` を、validation-ready retained bridge には `adapter_validation_ref` を、invocation-ready retained bridge には `consumer_invocation_surface_ref` を、exchange-body-ready retained bridge には `exchange_rule_body_ref` を、runtime-coupling-ready retained bridge には `runtime_coupling_ref` を、transport-ready retained bridge には `transport_protocol_ref` を、failure-ready retained bridge には `failure_body_ref` を、failure-ready retained bridge の次段には `actual_invocation_protocol_ref` を、binding-ready retained bridge には `consumer_host_binding_ref` を、wording-ready retained bridge には `failure_wording_ref` を、handoff-ready retained bridge には `actual_runtime_handoff_ref` を、receipt-ready retained bridge には `emitted_invocation_receipt_ref` を、transcript-ready retained bridge には `runtime_transcript_ref` を足す current first choice までを固定した。next later candidate は、actual notebook runtime handoff materialization comparison であり、`proof_assistant_adapter` pressure は still second practical candidate に残す
- rough weight: 重
- rough 所要: 3〜6 task / 3〜8日

### 3. detached validation loop は maintenance mode に戻す

- current self-driven friction reduction は
  - fixture stem shorthand
  - missing fixture fail-fast
  - default run label derivation
  - `compare-fixture-aggregates`
  - bundle / aggregate / static gate の shallow reference-only triage
  までで checkpoint close とみなしてよい
- 残りの `reference update / bless` は、final path policy / retention policy と接続するため current mainline からは外し、later candidate に残す
- rough weight: 低
- rough 所要: 0〜1 task

### 4. authoritative room baseline は checkpoint close として維持する

- `specs/examples/121-shared-space-authoritative-room-baseline.md` を current baseline judgment として drift 監視する
- final activation / authority / auth / consistency / fairness catalog へは入らず、baseline と next practical candidate の stop line を維持する
- rough weight: 低
- rough 所要: 0〜1 task / drift 時のみ

### 5. parser boundary / first checker cut は reserve path として維持する

- current checkpoint では Phase 3 self-driven portion は一旦尽きたとみなし、active candidate にしない
- reopen は次の pressure が出たときだけ行う
  - actual parser / public checker pressure
  - Phase 5 inventory からの強い接続 need
  - runtime / proof mirror need の増加
- したがって near-term では comparison 対象ではあるが、mainline candidate ではない

## rough step estimate

以下は rough estimate であり、仕様 drift や review 指摘に応じて前後する。

| 目標 | rough step estimate | 注記 |
|---|---|---|
| consistency / fairness / causal metadata catalog を working subset として比較する | current package close | `specs/examples/121...` から `specs/examples/125...` までで checkpoint close。stronger control-plane split は later pressure が出たときだけ reopen |
| small decidable core / proof / async-control inventory を一段進める | current package close | `specs/examples/126...` で 4-way split、`specs/examples/127...` で proof-obligation matrix + mixed handoff sketch、`specs/examples/128...` で mixed row default / boundary-specific split / actual emitter の threshold、`specs/examples/129...` で first concrete consumer を theorem line に置き、`specs/examples/130...` で theorem-side projection bundle、`specs/examples/131...` で typed symbolic `evidence_refs` family、`specs/examples/132...` で public checker migration defer threshold、`specs/examples/133...` で minimum contract row core、`specs/examples/134...` で `proof_notebook` first consumer、`specs/examples/135...` で `goal_text` attachment、`specs/examples/136...` で notebook bridge artifact defer threshold、`specs/examples/137...` で next reopen order を notebook workflow first / `proof_assistant_adapter` second に固定し、`specs/examples/138...` で workflow pressure の first threshold を review checklist / walkthrough に置き、`specs/examples/139...` で review unit を docs-only named bundle に寄せ、`specs/examples/140...` で docs-only bridge sketch、`specs/examples/141...` で compare basis refs、`specs/examples/142...` で bless decision state、`specs/examples/143...` で review-note refs、`specs/examples/144...` で retained-notebook ref、`specs/examples/145...` で review-session ref、`specs/examples/146...` で `reviewed_by_ref + reviewed_at_ref`、`specs/examples/147...` で `review_session_state`、`specs/examples/148...` で `retention_state`、`specs/examples/149...` で `retained_path_policy_ref`、`specs/examples/150...` で `emitted_artifact_ref`、`specs/examples/151...` で `handoff_emitter_ref`、`specs/examples/152...` で `consumer_adapter_ref`、`specs/examples/153...` で `exchange_rule_ref`、`specs/examples/154...` で `adapter_validation_ref`、`specs/examples/155...` で `consumer_invocation_surface_ref`、`specs/examples/156...` で `exchange_rule_body_ref`、`specs/examples/157...` で `runtime_coupling_ref` までを current first choice に置いた。next は transport / failure threshold を比べる |
| detached validation loop を maintenance mode で維持する | 0〜1 task | current self-driven portion は close。`reference update / bless` は later candidate |
| authoritative room baseline の drift を checkpoint close として維持する | 0〜1 task | current baseline は `specs/examples/121...` に集約済み |
| Phase 3 reserve path を reopen する条件整理 | 0〜2 task | later pressure が出たときだけ着手 |

## いまの blocker

### 1. final parser grammar 未固定

- companion notation はかなり整理されたが、parser syntax はまだ未決である
- これを早く決めすぎると semantics より syntax が先行してしまう

### 2. fallback intuition drift

- guarded option chain 読みと outer-longer-lifetime 直感の tension は、まだ prose 補助が要る

### 3. heavier workstream の入口未整理

- 型システム、静的解析、決定可能性、theorem prover 連携は、まだ entry criteria の段階である

### 4. detached trace / audit が process 内比較に閉じていること

- batch / profile まで積んでも、結果を repo 外 artifact として残しにくい
- case 数が増えると「その場で読んで終わる」運用から抜けにくい
- docs-only minimal schema はできたが、thin export boundary と保存パス規約はまだ未決である
- current non-production candidate として `target/current-l2-detached/` は置けたが、final path policy と actual aggregate exporter API はまだ未決である
- current detached validation loop には aggregate emitter sketch と `emit-aggregate` wrapper を足せるが、これは production aggregate API finalization ではない
- ただし actual narrow cut としては、aggregate emitter 内 private transform を shared support module へ切り出し、`BatchRunSummary -> detached aggregate artifact` の repo 内 callable boundary までは進めてよい
- bundle emitter 側も同様に、private transform を shared support module へ切り出し、`FixtureBundle + BundleRunReport -> detached bundle artifact` の repo 内 callable boundary までは進めてよい

### 5. review infrastructure の変動

- reviewer completion が遅い / 返らない場合があり、task の closing evidence を自前で揃える必要がある

### 6. Phase 3 reserve path の premature reopen

- current checkpoint では staged parser spike / reconnect family は freeze threshold まで source-backed に揃っている
- ここで self-driven reopening を急ぐと、wording refinement に留まるか、public checker / runtime proof boundary を先取りしやすい

## いまの bottleneck

- `fixture authoring / elaboration` の独立 bottleneck は引き続き残っている
- そのうえで、**detached trace / audit serialization と richer host interface の 2 項目を比べるなら**、前者を先に切る方が前進量は大きい
- richer host interface と coverage analysis の入口整理は、その後段候補である
- notation 自体は current L2 の narrow task を回すには十分安定しており、直近では operational boundary の方が重い

## 近い将来の sequencing

推奨順は次である。
ただしこれは `fixture authoring / elaboration` の独立 bottleneck を取り消すものではなく、**trace/audit と host interface の比較に限った近い sequencing** である。

1. semantics drift regression を増やす
2. detached trace / audit serialization の最小境界を切る
3. detached exporter の first entry を bundle 層から narrow に始める
4. non-production の bundle-first emitter と core-only diff helper を足す
5. bundle artifact 保存先 / path policy と aggregate export の最小 cut を整える
6. aggregate summary export の smoke を数回回し、coarse summary / typed count / list anchor の cut を確認する
7. aggregate compare helper を narrow に足し、`summary_core` compare と run-label convenience を固める
8. fixture authoring / elaboration template を detached validation loop 前提へ寄せる
9. richer host interface / coverage analysis の入口を narrow に切る
10. parser / checker public boundary の inventory を Phase 5 側で整える
11. later pressure が出たときだけ Phase 3 reserve path を reopen する
12. その後で parser / richer runtime の判断に進む

## 今の working assumption

- current L2 semantics は大きく動かさない
- parser-free PoC は継続利用する
- hard-coded named profile catalog は維持する
- machine-readable catalog asset / manifest はまだ future option に留める

## Historical appendix — pre-checkpoint narrow-scope catalog

以下は、Phase 3 current tranche closeout 前に積み上げていた narrow-scope catalog を履歴 / provenance として残す section である。**current roadmap としては読まないこと**。再開候補の current reading は、この文書の上部にある `候補 1..4`、`plan/17-research-phases-and-autonomy-gates.md`、`progress.md` を優先する。

- detached trace / audit の docs-only schema から thin exporter 候補の carrier mapping を切り出す
- bundle / batch summary が detached artifact として最低限どこまで出せば比較可能かを棚卸しし、bundle-first exporter entry を docs に固定する
- bundle-first artifact の payload core / bundle_context / detached non-core / aggregate-only を docs-only で切り分ける
- detached exporter chain の docs-only judgment を 1 箇所へ集約し、non-production の tiny emitter / diff helper / fixture template を PoC loop 補助として足す
- detached validation loop の storage/path policy、tiny wrapper、aggregate export の actual narrow cut を docs-only から thin operational aid へ進める
- aggregate export の actual narrow cut として、example private transform を shared support module へ落とし、public API を増やさずに repo 内 callable boundary へ寄せる
- bundle export の actual narrow cut として、example private transform を shared support module へ落とし、public API を増やさずに repo 内 callable boundary へ寄せる
- first checker cut の local / structural floor を detached validation loop に接続するなら、runtime artifact と混ぜずに static gate artifact helper を別立てで足す
- detached validation loop の common path として、1 fixture の bundle export / optional reference compare / single-fixture aggregate smoke を 1 command で回す helper を足す
- static-only / malformed / underdeclared fixture の common path として、static gate artifact の emit / compare を 1 command で回す helper を足す
- first checker cut の local / structural floor について、`expected_static.reasons` の direct promotion は current fixture corpus と衝突することが確認できたので、次は additive optional `checked_reasons` を最小 dedicated carrier として導入し、typed reason code / detached-only 維持を後段比較に送る
- `checked_reasons` を bridge に置いたので、その次は typed reason code に進めてよい stable cluster と parametric shape を inventory 化する
- detached static gate artifact 側には helper-local / reference-only な `reason_codes` mirror を actualize してよいが、next narrow step はそれを first-class typed source に昇格させるか、fixture-side typed carrier を別に切るかの比較である
- その前段として、`e4` / `e5` から始めた static-only malformed / underdeclared fixture への `checked_reasons` narrow adoption を、`e12` / `e13` の stable cluster まで広げてよい
- duplicate declaration cluster は next fixture-authoring comparison point として actual corpus に入れてよいが、current helper cut では `checked_reasons` と detached `reason_codes` の stable cluster へはまだ昇格させない
- next actual corpus tranche として、missing chain head / predecessor / successor option cluster は stable malformed wording と detached `reason_codes` mirror を持つので、`e16` / `e17` / `e18` として actualize してよい
- same-lineage stable cluster inventory は current docs の範囲では `e19` によって一巡したので、その次の narrow step は first-class typed carrier actualization を急ぐか、fixture authoring 実地反復をもう少し積むかの比較になる
- current valid fixture 群では actual static gate `reasons` が空な例が多いため、`checked_reasons = []` を広く足す task は優先しない
- その next narrow step として、`checked_reasons` を auto-fill しない display-only authoring assist を current helper stack に置き、actual static gate wording の narrow adoption を review 可能にしてよい
- さらにその次段として、helper-local / reference-only `reason_codes` mirror を assist source にする display-only helper を current helper stack に置き、future typed carrier 候補 row を fixture schema に昇格させずに review 可能にしてよい
- その次の実地反復として、static-only fixture corpus を横断する readiness scan を current helper stack に置き、`checked_reasons` adoption と `reason_codes` suggestion availability を tranche 単位で観察してから future typed carrier actualization の着手順を決めてよい
- readiness scan の次段として、first actualization family は lineage edge pair family、second tranche は declared target edge pair family として比較結果を固定した
- current code / fixture corpus では、その carrier cut を current stable cluster inventory の 8 kind まで広げ、これを覆う 9 fixture で `checked_reason_codes` adoption と `reason_codes` suggestion availability が 9 / 9 で揃うところまで actualize 済みである
- さらに current readiness scan では、stable cluster 8 kind を覆う 9 fixture が `checked_reasons` / `checked_reason_codes` / actual suggestion の 3 者で揃っており、coexistence follow-up は `none` である
- さらに checker cluster roll-up では、same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` まで current static-only corpus が覆っていることを source-backed に確認できる
- さらに same-lineage floor については、fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` を narrow compare する helper-local first checker spike を追加し、`e4` / `e12` の smoke が通っている
- さらに missing-option structure floor については、same-lineage spike の次段として helper-local second checker spike を追加し、`e16` / `e17` の smoke が通っている
- さらに capability strengthening floor については、`e13` / `e20` を actual corpus に揃えたうえで helper-local third checker spike を追加し、capability family だけを narrow compare する smoke まで回してよい
- duplicate declaration cluster は current cut のまま `checked_reasons` / `checked_reason_codes` / detached `reason_codes` stable inventory へは上げず、actual wording を `checker_core.reasons` と focused smoke で見る
- current cut では、same-lineage / missing-option / capability の 3 spike に shared support helper を導入しつつ family facade script を残すところまでは actualize 済みである
- current docs-only judgment では、checker-side shared family compare entry はまだ切らず、family facade 維持で止める
- 次の narrow step は、public checker cut comparison に入る前に、generic entry を later public checker API comparison と同時に扱うかどうかを見極めることである
- `TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche は actualize 済みなので、次に自走で進めるなら
  - second malformed static tranche の comparison close
  - first-tranche wording / row family の drift 点検
  - saved artifact compare need が shared carrier threshold を本当に満たすかの再比較
  の順で narrow に進めるのが自然である
- current comparison としては、second malformed static tranche の問い自体は先に閉じるが、actual tranche 追加はまだ行わず、next は helper-local wording / finding family stability comparison に進むのが自然である
- current comparison としては、first-tranche wording / finding family stability では exact wording / row family を fixed working set として維持し、generic 化や alias 導入は shared carrier threshold の再比較と later generic/public comparison まで deferred にするのが自然である
- current comparison としては、saved artifact compare need の観点で shared carrier threshold を再比較しても、current helper-local checker が artifact path を直接読めるため threshold はまだ未充足とみなし、next は generic structural checker family / public checker API comparison の前提条件整理へ進むのが自然である
- current comparison としては、generic structural checker family / public checker API comparison に進む concrete pressure もまだ不足しているため、current try/rollback checker line はここで一旦 pause とみなし、next self-drivable mainline は parser boundary / first parser cut inventory 側へ移すのが自然である
- aggregate emitter sketch を current wrapper に接続し、directory summary を artifact として保存する smoke を増やす
- fixture authoring bottleneck のうち boilerplate 部分だけを `target/` 下の non-production scaffold helper へ切り出し、hand-written fixture を正本に保ったまま authoring cost を下げる
- parser-free host harness と richer host interface / coverage analysis の boundary inventory を作る
- current baseline として、current companion notation から first parser cut に入れてよい semantic cluster は narrow inventory 化済みである
- current baseline として、parser 導入前の syntax decision inventory は plan と spec に切り出し済みである
- current baseline として、first parser cut inventory は first checker cut の local / structural judgment と theorem prover / model checker 側へ残す property の entry criteria に docs-only で接続済みである
- current baseline として、actual parser spike の比較を monolithic cut ではなく checker-led staged spike として扱い、
  1. chain / declaration structural floor
  2. `try` / rollback structural floor
  3. request / admissibility cluster
  の順で source-backed priority を揃えるのが自然である
- current next narrow step としては、stage 1 parser spike の accepted parse cluster を
  - option declaration core
  - explicit edge-row family
  - edge-local lineage metadata
  - declaration-side guard slot
  に留めつつ、declaration-side guard slot は predicate fragment parse へ進めず opaque attached slot として扱う cut を維持するのが自然である
- current baseline として、stage 1 handoff は parser-side opaque slot carrier と current parser-free AST fixture schema を同一視せず、thin lowering bridge を介して `OptionDecl.lease` へ narrow に接続するのが自然である
- current baseline として、その parser-side opaque slot carrier の naming は `decl_guard_slot` を第一候補にし、thin lowering bridge は slot-only helper ではなく option-level structural transfer として読むのが自然である
- current baseline として、stage 1 actual parser spike の smoke family は `e4-malformed-lineage` と `e7-write-fallback-after-expiry` の two-fixture pair を最小 working set とし、`e3-option-admit-chain` は later-stage contrast reference に残すのが自然である
- current baseline として、actual stage 1 parser spike は `crates/mir-ast/tests/support/` 配置の private helper として始め、compare surface は lowered fixture-subset compare に留めるのが自然である
- current baseline として、actual implementation へ入る直前 cut では input surface は test inline string、`decl_guard_slot` internal carrier は dedicated wrapper + owned `surface_text`、private helper family は `current_l2_stage1_parser_spike_support` を第一候補にするのが自然である
- current actualization として、その first tranche は `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` で `e4` / `e7` の lowered fixture-subset compare と guard-slot retention smoke を通すところまで進めてよい
- current actualization として、その malformed-source first tranche は
  - `missing edge-local lineage metadata`
  - `option-local admit is outside stage 1 accepted cluster`
  の helper-local wording fragment 2 件まで actualize してよい
- current docs-only next step としては、stage order を崩さず、request / admissibility cluster を stage 3 として進めるときの最初の sub-cutとして declaration-side `admit` attached slot を比較し、`PerformVia` / request-local clause は still later stage に残すのが自然である
- current docs-only refinement として、stage 3 first tranche の parser-side carrier 名は `decl_admit_slot` を第一候補にし、fixture-side `OptionDecl.admit` へは direct lower せず、structural subset compare と slot retention smoke を分けるのが最小である
- current actualization として、その success-side first tranche は `crates/mir-ast/tests/support/current_l2_stage3_admit_slot_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage3_admit_slot_spike.rs` で `e3` 由来 option / chain subset compare と `decl_admit_slot.surface_text` retention smoke を通すところまで進めてよい
- current actualization として、その malformed-source first tranche は
  - `missing declaration-side admit slot payload`
  - `request head is outside stage 3 admit-slot first tranche`
  の helper-local wording fragment 2 件まで actualize してよい
- その次段で比較するべきなのは、
  - request-local `require` / `ensure` spillover を stage 3 admit-slot branch の later malformed pair にどこまで持たせるか
  - fixture-side `OptionDecl.admit` node handoff を current phase で docs-only deferred に留めるか
  - current private helper を public parser API へ昇格させる前提条件をどこまで narrow に切るか
  であり、predicate fragment floor や request cluster を still later stage に残す
- current sequencing judgment としては、stage 3 admit-slot branch の次段は request-local clause spillover より先に fixture-side `OptionDecl.admit` handoff comparison を扱うのが自然である
- ただし current handoff comparison の結論は actual compare への昇格ではなく docs-only deferred であり、predicate fragment boundary の first cut が見えるまで reopen しない
- current actualization として、その later branch の first tranche は
  - `request-local require clause is outside stage 3 admit-slot first tranche`
  - `request-local ensure clause is outside stage 3 admit-slot first tranche`
  の helper-local wording fragment 2 件まで進めてよい
- その次段では、predicate fragment boundary を declaration-side only compare や opaque slot retention に寄せず、declaration-side `admit` と request-local `require` / `ensure` の shared floor を持つ isolated helper として reopen するのが最小である
- current actualization として、その first tranche は `e2` / `e3` / `e10` / `e11` anchor の predicate subset compare に留め、request head + clause attachment multiline shape と fixture-side full request node compare は still later stage に残す
- その次段までの整理として、
  - helper-local malformed/source family の first pair は `duplicate ensure` + unsupported direct child line を first choice に置く
  - その first pair は helper-local / test-only focused smoke に上げてよい
  - その後は `missing multiline ensure block` family を fixture-side full request contract compare より先に扱うのが自然である
  - その `missing multiline ensure block` hidden path も focused smoke として surfaced 済みである
- current reopen judgment として、その後は remaining suite malformed wording family を suite helper 側で still 追うより、fixed two-slot suite bridge を fixture-side full request contract subset compare へどこまで narrow に actualize してよいかを比較するのが自然である
  - source-side helper output は `Stage3RequestClauseSuite { require_fragment_text, ensure_fragment_text }` に留める
  - request head parse と full request node compare は still later stage に残す
  - existing isolated predicate fragment helper を再利用し、private helper を public parser API へ昇格させる前提条件は別 comparison に残す
- current actualization として、その次段の `Stage3RequestContractSubset` helper-local / test-only first tranche は actualize 済みであり、same source-side suite carrier を `PerformOn` / `PerformVia` fixture の contract subset compare に通せる line まで source-backed に固定した
- current guard judgment として、その後の next step でも row-list widening は採らず、still 0-or-1 guard に留めるのが自然である
- current sequencing judgment として、その次は source-side suite bridge widening 条件をさらに積むより、この family を一旦 freeze し、parser boundary staging と first checker cut 接点の re-sweep へ戻るのが自然である
- current side-selection judgment としては、その戻り先は parser boundary staging 側より first checker cut connection 側を先に取るのが自然である
  - next narrow step は、existing parser-boundary evidence family のうち、どれを first checker cut inventory へ最初に reconnect するかの comparison である
