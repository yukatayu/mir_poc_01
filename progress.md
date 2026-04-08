# progress

最終更新: 2026-04-08 12:30 JST

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
- **detached validation loop** は、bundle / aggregate / static gate の emit・保存・compare・smoke を回せる入口まで来ている。
- **parser boundary** は、stage 1 private spike と stage 3 declaration-side / later malformed-source first tranche まで actualize 済みである。
- **shared-space / membership** は mainline ではないが、upper-layer docs-first boundary として「participant plain array を core に焼き込まず、session-scoped membership registry + derived snapshot view を第一候補にする」比較に加え、tree-like view を derived に留めること、activation visibility の compile-time over-approximation と runtime control-plane を分けること、authority / consistency / RNG provider を別軸で比較すること、room resource ごとの owner slot / delegated capability / handoff epoch を分けて読む current working model、authoritative room の activation rule 最小候補を `authority-ack` に置き、さらに **authoritative room に限って** authority placement の current first choice を `single room authority`、consistency mode の current first choice を `authoritative serial transition`、RNG / fairness source の current first choice を `authority_rng` に置き、authoritative game room の current minimal concrete bundle を `authority-ack` + `single room authority` + `authoritative serial transition` + `authority_rng` に整理し、RNG だけを `delegated_rng_service` に差し替える next practical bundle を分離した。さらに reconnect / late leave / in-flight action は room profile に全部入れず、`member_incarnation` と uncommitted action invalidation を minimal room-profile rule、timeout / retry / resend を external policy layer に残す line まで進んだ。causal metadata 側では plain vector deletion を避け、epoch / incarnation split を first practical candidate、control-plane separated carrier を next stronger candidate に置くところまで進んだ。fairness trust model 側では `opaque authority trust` を current minimal candidate、`auditable authority witness` を next narrow strengthening candidate に置き、provider placement と witness requirement を別軸で比較する line を追加した。append-heavy room では `append-friendly room` を first practical catalog、`delegated_rng_service` を next practical candidate にする line を維持している。
- 現在の主ボトルネックは semantics の大崩れではなく、
  - fixture authoring / elaboration の反復コスト
  - parser boundary の staged 実装
  - richer host interface を後段に残したまま、必要最小限の typed coverage / proof boundary をどこで切るか
  である。

## 研究フェーズ（大局）

| Phase | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---|---|---|---|---|
| Phase 0 | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / progress の整合維持 |
| Phase 1 | current L2 semantics stabilization | 終盤 | 中 | 自走可能 | mainline semantics drift は narrow regression 中心 |
| Phase 2 | parser-free PoC / detached validation loop | 終盤 | 中 | 自走可能 | loop 入口は成立、現在は運用摩擦低減 |
| Phase 3 | parser boundary / first checker cut | 前半〜中盤 | 中〜やや重い | 自走可能 | stage 1 / stage 3 first tranche は actualize 済み |
| Phase 4 | shared-space / membership / practical example boundary | 前半 | 重い | 一部自走可能 | docs-first boundary は進めてよいが final profile は要仕様確認 |
| Phase 5 | static analysis / type / theorem prover / model checker boundary | 入口整理 | とても重い | 後段依存 | small decidable core の inventory までは進めてよい |
| Phase 6 | actual parser / checker / runtime commitment | 未着手 | 重い | 後段依存 | Phase 2 / 3 / 5 の gate 後に入る |
| Phase 7 | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Typed-Effect / Prism / app contract は user 仕様待ち |

### 現在の主線

- **主線**: Phase 2 終盤 + Phase 3 前半〜中盤
- **side line**: Phase 4 前半
- **まだ勝手に finalization しない**: final parser grammar、production exporter API、shared-space final catalog、higher-layer application contract

## いま自走で進めてよい範囲

### 着手可能

- current L2 semantics の narrow regression 追加
- fallback / notation / representative example の drift 修正
- parser-free PoC helper / detached validation loop の改善
- fixture authoring / elaboration の補助改善
- parser boundary の staged spike comparison / private helper actualization
- first checker cut の narrow helper / compare family actualization

### 後段依存

- richer host interface の typed carrier 化
- static analysis / type system / theorem prover / model checker boundary の本格 actualization
- final parser grammar の固定

### 要仕様確認

- Mirrorea Fabric の具体仕様
- Typed-Effect Wiring Platform の具体仕様
- shared-space / session activation / membership protocol の具体仕様
- PrismCascade / 上位 shared space / 上位アプリケーション
- Reversed Library の具体要件

## 現在の mainline

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

### 3. parser boundary staged spike

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

## 残課題の優先順位

### Priority A — すぐ続けてよい

1. stage 3 later branch の次段比較
   - request head + clause attachment multiline shape を docs-only で比較するか
   - predicate fragment boundary の reopen 条件を先に切るか
2. detached validation loop を何本か追加 fixture で回し、authoring / compare の friction を実地で減らす
3. first checker cut / parser boundary の staged line を無理なく合流させる

### Priority B — A の後でよい

1. detached exporter actual narrow API / storage policy の final cut
2. richer host interface に行く前の typed coverage carrier の narrow cut
3. static analysis / type / theorem prover / model checker の boundary inventory を core / external で分ける

### Priority C — user specification が必要

1. Mirrorea / Typed-Effect / Prism / shared space の詳細仕様
2. 上位アプリケーションや Reversed Library の domain-specific contract

## validation loop 入口までの見積もり

- **detached validation loop の入口自体は、すでに入っている**と見てよい。
- ただし「継続的に追加し続けても手戻りが小さい」状態までは、まだ次の refinement が必要である。
  1. stage 3 parser boundary の later branch comparison をもう 1〜2 task
  2. fixture authoring / elaboration の friction を下げる実地反復を 1〜2 task
  3. detached exporter narrow API / storage candidate を実地 smoke で固める 1〜2 task

rough estimate:
- validation loop を「入口に到達した」と言い切るには **あと 2〜4 task**
- current L2 mainline を「数日単位でかなり自走できる」と言えるには **あと 5〜8 task**

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書 / decision level / invariants | 92% | 87% | 72% | 着手可能 | 基礎境界はかなり安定 |
| Mir current L2 core semantics | 86% | 80% | 78% | 着手可能 | current mainline には十分安定 |
| fallback / notation / representative examples | 86% | 82% | 68% | 着手可能 | examples drift はかなり抑えられている |
| parser-free PoC execution stack | 90% | 85% | 98% | 着手可能 | runtime / bundle / batch / selection / profile は揃っている |
| detached export / validation loop | 98% | 96% | 99% | 着手可能 | 入口は成立、現在は運用面の摩擦低減フェーズ |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 79% | 72% | 74% | 着手可能 | stage 1 と stage 3 first tranches は actualize 済み |
| first checker cut / helper-local compare family | 82% | 70% | 80% | 着手可能 | narrow helper family は安定、generic/public は後段 |
| richer host interface / typed coverage carrier | 45% | 32% | 25% | 後段依存 | current phase では太らせない |
| static analysis / type / theorem prover boundary | 36% | 26% | 12% | 後段依存 | hybrid staged approach を採る前提 |
| shared-space / dynamic membership boundary | 68% | 59% | 8% | 要仕様確認 | docs-first boundary と example、tree-view vs registry、activation visibility、authority / consistency / RNG provider の比較に加え、resource owner slot / delegated capability / handoff epoch の working model、authoritative room の `authority-ack` / `single room authority` / `authoritative serial transition` / `authority_rng` first choice、authoritative game room の minimal concrete bundle、`member_incarnation` と uncommitted action invalidation を room-profile 側に残す reconnect policy cut、plain vector deletion を避けて epoch / incarnation split を first practical candidate にする causal metadata cut、fairness trust model を `opaque authority trust` / `auditable authority witness` で分ける line、append-heavy room の `append-friendly room` first practical catalog と `delegated_rng_service` next candidate までは進められるが、relaxed room と final activation / auth / consistency / fairness catalog は user 仕様待ち |
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 16% | 11% | 5% | 要仕様確認 | higher-layer の具体仕様は依然 user からの追加仕様が必要 |

## 現時点での大きい未解決問題

1. fixture authoring / elaboration bottleneck
2. detached exporter の actual API / storage policy finalization
3. final parser syntax と companion notation の切り分け
4. richer host interface / typed coverage carrier
5. static analysis / type system / theorem prover / model checker boundary
6. shared-space / membership protocol / authority / consistency mode catalog / fairness source の finalization

## 次に進めるべき task

1. stage 3 later branch の
   - request head + clause attachment multiline shape
   - predicate fragment boundary reopen 条件
   のどちらを先に比較するかを narrow に決める
2. current detached loop を新しい fixture 追加手順と結び付けて、authoring friction をさらに 1 段下げる
3. parser boundary staging と first checker cut の接点を docs-only で再棚卸しし、無理な合流を避ける
4. shared-space / membership boundary は docs-first example まで進めてよいが、activation / authority / auth / consistency catalog の finalization は user 仕様確認で止める

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
- 2026-04-08 12:25 JST — shared-space の fairness trust model を `opaque authority trust` / `auditable authority witness` / `delegated provider attestation` / `distributed fairness protocol` で比較し、authoritative room では opaque trust を current minimal、auditable witness を next narrow strengthening candidate に置き、provider placement と witness requirement を別軸で扱う line を plan mirror に追記した。次は identity / auth layering を fairness / authority / membership boundary とどう分離するかを narrow に比較できる段階。
- 2026-04-08 12:30 JST — repo 全体の大局 phase / autonomy gate を `plan/17` に集約し、`progress.md` に current phase・重さ・自走可否の snapshot を追加した。次は shared-space の identity / auth layering を participant carrier / authority / fairness と混ぜずにどう切るかを narrow に比較する段階。
