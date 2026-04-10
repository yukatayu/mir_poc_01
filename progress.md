# progress

最終更新: 2026-04-10 11:11 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度

## 現在の要約

- **current L2 semantics** は、current mainline を進めるには十分安定している。
- **Phase 0 / 1 / 2** は maintenance tail である。parser-free PoC、detached validation loop、fixture 実務の入口は成立しており、主眼は drift suppression と residual maintenance に移っている。
- **Phase 3** は reserve path である。stage 1 / 2 / 3 の private staged spike と reconnect freeze threshold まで source-backed に揃ったが、current promoted line ではなく、later pressure が出たときだけ reopen する。
- **Phase 4** は `specs/examples/121...125` までで current package close である。authoritative room baseline、working subset row、minimal witness core、delegated-provider practical cut、control-plane threshold comparison までは整理済みで、final catalog はまだ固定しない。
- **Phase 5** は `specs/examples/126...181` までで theorem-line later package close である。small decidable core / proof / async-control boundary inventory と symbolic retained bridge は `archive_retention_layout_ref` まで current first choice を切れており、**next later reopen** は `actual retained archive payload body family comparison` に置いている。
- current main bottleneck は semantics の大崩れではなく、**shared-space final catalog**、**final parser / public checker boundary**、**actual type / proof / protocol verifier actualization timing** である。

## 研究フェーズ（大局）

| Phase | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---|---|---|---|---|
| Phase 0 | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / snapshot の整合維持 |
| Phase 1 | current L2 semantics stabilization | 終盤 | 中 | 自走可能 | mainline semantics drift は narrow regression 中心 |
| Phase 2 | parser-free PoC / detached validation loop | 終盤の maintenance tail | 中 | 自走可能 | 入口は成立、残りは bless/update など policy-dependent residual |
| Phase 3 | parser boundary / first checker cut | reserve path | 中〜やや重い | 後段依存 | private staged spike と reconnect freeze threshold までは整理済み |
| Phase 4 | shared-space / membership / practical example boundary | checkpoint close | 重い | 一部自走可能 | `121...125` までは current package close。final catalog は user spec / later pressure 依存 |
| Phase 5 | static analysis / type / theorem prover / async-control boundary | theorem-line later package close | とても重い | 自走可能 | `126...181` まで current package close。next reopen は actual retained archive payload body family |
| Phase 6 | actual parser / checker / runtime commitment | 未着手 | 重い | 後段依存 | final public boundary はまだ固定しない |
| Phase 7 | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Prism / 上位アプリは user specification が要る |

## 現在の主線

1. **Phase 5 later reopen**
   - actual retained archive payload body family comparison を next promoted line に置く。
2. **cross-phase checkpoint maintenance**
   - `Documentation.md`、`plan/`、`tasks.md`、`progress.md`、research abstract の drift を抑える。
3. **Phase 4 later reopen candidate**
   - authority handoff / provider binding / activation frontier の concrete pressure が出たときだけ reopen する。

## いま自走で進めてよい範囲

### 着手可能

- current L2 semantics の narrow regression
- parser-free PoC / detached validation loop の maintenance residual
- Phase 5 theorem-line later reopen の docs-first comparison
- shared-space の docs-first boundary comparison と practical example 整理
- checkpoint close 済み package の drift suppression

### 後段依存

- final parser grammar
- public checker API
- richer host interface の全面 actualization
- Phase 3 reserve path の promoted reopen
- actual type / theorem prover / protocol verifier integration

### 要仕様確認

- shared-space final activation family
- shared-space final authority / consistency / fairness catalog
- higher-layer application contract
- Mirrorea / Typed-Effect / Prism / 上位アプリの concrete goal

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書 / decision level / invariants | 93% | 89% | 73% | 着手可能 | 基礎境界はかなり安定 |
| Mir current L2 core semantics | 86% | 81% | 79% | 着手可能 | mainline には十分安定 |
| fallback / notation / representative examples | 86% | 83% | 69% | 着手可能 | final grammar は still later |
| parser-free PoC execution stack | 90% | 85% | 98% | 着手可能 | runtime / bundle / batch / selection / profile は揃っている |
| detached export / validation loop | 98% | 97% | 99% | 着手可能 | current self-driven friction reduction は checkpoint close |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 88% | 81% | 86% | 後段依存 | reserve path。later pressure 時だけ reopen |
| first checker cut / helper-local compare family | 89% | 79% | 88% | 後段依存 | reconnect subline は freeze threshold まで整理済み |
| shared-space / dynamic membership boundary | 83% | 76% | 12% | 一部自走可能 | `121...125` まで current package close |
| static analysis / type / theorem prover / async-control boundary | 98% | 92% | 18% | 着手可能 | `126...181` まで current package close。next は actual retained archive payload body family |
| richer host interface / typed coverage carrier | 45% | 32% | 25% | 後段依存 | current phase では太らせない |
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 16% | 11% | 5% | 要仕様確認 | higher-layer の具体仕様がまだ足りない |

## 現時点での大きい未解決問題

1. shared-space の final activation rule
2. authority placement の final shape
3. consistency mode catalog をどこまで language 側に持つか
4. fairness / RNG trust model
5. control-plane separated causal carrier の final actualization
6. final parser grammar / public checker boundary

## 次に進めるべき task

1. **Phase 5 later reopen** として `actual retained archive payload body family comparison` を扱う
2. その task close の中で mirror sweep を同時に行い、checkpoint drift を抑える
3. Phase 4 は current package close を維持し、stronger split は concrete pressure が出たときだけ reopen する

## 作業ログ（簡潔）

- 注記: この欄は **recent log** として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-09 13:20 JST — detached validation loop の second friction tranche として `compare-fixture-aggregates` を追加し、single-fixture aggregate 同士の compare を noisy な full-vs-single contrast から分離した。次の friction は reference update / longer compare triage 側に寄ることを確認した。
- 2026-04-10 08:44 JST — stage 3 request / predicate / attachment branch の current snapshot を整理し、Phase 3 を reserve path として読む phase gate を固定した。next は shared-space の identity / auth / admission / fairness line を narrow に比較する段階。
- 2026-04-10 09:19 JST — Phase 5 theorem-line later package を `archive_bless_update_policy_ref` まで current first choice に伸ばし、next later reopen を retained archive payload comparison に置いた。Phase 5 は current package close と読める状態になった。
- 2026-04-10 09:50 JST — `tasks.md` と `progress.md` を current snapshot として全面整理し、review finding に合わせて `plan/11` / `plan/17` も current promoted line へ追随させた。next は retained archive payload comparison を promoted line として進める段階。
- 2026-04-10 10:23 JST — Phase 5 theorem-line later reopen として `retained_archive_payload_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は archive retention layout comparison で、Phase 5 は `126...180` まで current package close と読める状態になった。
- 2026-04-10 11:11 JST — Phase 5 theorem-line later reopen として `archive_retention_layout_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained archive payload body family comparison で、Phase 5 は `126...181` まで current package close と読める状態になった。
