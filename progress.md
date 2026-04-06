# progress

最終更新: 2026-04-06 09:19 JST

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
- 現在の主ボトルネックは semantics の大崩れではなく、
  - fixture authoring / elaboration の反復コスト
  - parser boundary の staged 実装
  - richer host interface を後段に残したまま、必要最小限の typed coverage / proof boundary をどこで切るか
  である。

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
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 15% | 10% | 5% | 要仕様確認 | user からの追加仕様が必要 |

## 現時点での大きい未解決問題

1. fixture authoring / elaboration bottleneck
2. detached exporter の actual API / storage policy finalization
3. final parser syntax と companion notation の切り分け
4. richer host interface / typed coverage carrier
5. static analysis / type system / theorem prover / model checker boundary

## 次に進めるべき 3 task

1. stage 3 later branch の
   - request head + clause attachment multiline shape
   - predicate fragment boundary reopen 条件
   のどちらを先に比較するかを narrow に決める
2. current detached loop を新しい fixture 追加手順と結び付けて、authoring friction をさらに 1 段下げる
3. parser boundary staging と first checker cut の接点を docs-only で再棚卸しし、無理な合流を避ける

## 作業ログ（簡潔）

- 注記: **2026-04-06 09:19 JST 以降**の timestamp は `date` コマンドで取得した値を使う。以前の行は旧運用時の履歴である。
- 2026-04-06 08:44 JST — stage 3 admit-slot branch の carrier naming / compare surface を比較し、`decl_admit_slot` を第一候補にしつつ、fixture-side `admit` node へ direct lower せず、structural subset compare と slot retention smoke を分ける current judgment を固定した。次はこの cut を private helper の success-side first tranche として actualize する段階。
- 2026-04-06 08:52 JST — stage 3 admit-slot branch の success-side first tranche を actualize し、`e3` option / chain subset compare と `decl_admit_slot.surface_text` retention smoke を通した。次は malformed-source / request spillover のどちらを helper-local first tranche に入れるかを比較する段階。
- 2026-04-06 08:57 JST — stage 3 admit-slot branch の malformed-source pair を比較し、declaration-side `admit` payload 欠落と `PerformVia` spillover の 2 本だけを helper-local wording fragment smoke にする current judgment を固定した。次はこの pair を stage 3 private helper に actualize する段階。
- 2026-04-06 08:59 JST — stage 3 admit-slot branch の malformed-source first tranche を actualize し、`missing declaration-side admit slot payload` と `request head is outside stage 3 admit-slot first tranche` の 2 件を substring smoke として固定した。次は request-local clause spillover と fixture-side `admit` handoff のどちらを先に比べるかを決める段階。
- 2026-04-06 09:14 JST — stage 3 admit-slot branch の次段 sequencing を比較し、request-local clause spillover より先に fixture-side `OptionDecl.admit` handoff line を扱うのが自然だと整理した。次は handoff line 自体を actual compare に上げず、どこまで docs-only deferred に留めるかを詰める段階。
- 2026-04-06 09:18 JST — stage 3 admit-node handoff comparison を整理し、fixture-side `OptionDecl.admit` は already elaborated predicate node なので current phase では direct lowering も canonical surface compare も入れず、predicate fragment boundary が見えるまで docs-only deferred に留める判断を固定した。次は request-local `require` / `ensure` spillover を stage 3 later branch としてどこまで docs-only comparison に持つかを比べる段階。
- 2026-04-06 09:19 JST — stage 3 request-local clause spillover first tranche まで repo 状態を見直し、`progress.md` を snapshot 向けに再編した。次は stage 3 later branch の multiline attachment shape と predicate fragment reopen 条件のどちらを先に比較するかを narrow に決める段階。
