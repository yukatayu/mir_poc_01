# progress

最終更新: 2026-04-12 20:10 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度

## 現在の要約

- **current L2 semantics** は、Phase 6 前半の minimal compile-ready PoC に入るための semantic floor としては十分安定している。
- **Phase 1** は `specs/examples/291...292` により self-driven closeout fixed と読んでよい。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。
- **Phase 3** は `specs/examples/287...290` により self-driven freeze fixed と読んでよい。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。
- **Phase 6** は front-half compile-ready checkpoint 自体は fixed 済みである。`specs/examples/299...328` により parser / checker-runtime / formal-hook / source-sample first packages / theorem-first review-unit pilot が揃っている。
- `specs/examples/329...330` により、deferred authored-row widen sequencing も fixed 済みであり、current first choice は `e1 -> e21 -> e3` である。
- `specs/examples/331...332` により、proof-notebook bridge-sketch reopen ordering も fixed 済みであり、current first choice は **plain docs-only bridge sketch first / compare-ready bridge sketch second** である。authored-row actualization line を先に閉じ、theorem-side reopen はその後に置く。
- `specs/examples/333...334` により、first widened authored row `e1` actualization も fixed 済みである。`e1-place-atomic-cut` は helper-compatible single-line `ensure` source row として actualize 済みであり、current authored inventory / runner accepted set / regression bundle / README ladder で `runtime_try_cut_cluster` reached row に昇格した。
- `specs/examples/335...336` により、second widened authored row `e21` actualization も fixed 済みである。`e21-try-atomic-cut-frontier` は helper-compatible source row として actualize 済みであり、current authored inventory / runner accepted set / regression bundle / README ladder で `runtime_try_cut_cluster` reached row に昇格した。
- `specs/examples/337...338` により、third widened row `e3` theorem-side / formal-hook guard comparison も fixed 済みである。`e3` は current authored row にまだ入れず、theorem-side row-local review-unit cut と `runtime_try_cut_cluster` top を保ったまま、plain bridge sketch actualization の後段 reopen に残す。
- `specs/examples/339...340` により、plain proof-notebook bridge sketch actualization も fixed 済みである。old theorem-line `specs/examples/140` の docs-only bridge shape を current Phase 6 theorem-side first actualization として再利用し、compare-ready metadata / helper-emitter / concrete tool binding は still later に残す。
- current main bottleneck は semantics の大崩れではなく、**compare-ready bridge sketch second reopen を minimum に保ったまま current theorem-side line を 1 段進めること** である。

## compile-ready PoC の rough 読み

| 目標 | rough 進捗 | rough 残量 | 補足 |
|---|---:|---|---|
| parser-free current L2 PoC | 90%+ | maintenance closeout 数 package | `mir-semantics` 主線はすでに compile / test 可能 |
| Phase 6 前半 compile-ready minimal actual PoC | 96%+ | maintenance / reopen only | compile-ready checkpoint close は fixed と読める |
| fixed-subset syntax-backed sample verification milestone | 97%前後 | compare-ready bridge sketch second reopen、その後段の `e3` actualization reopen timing | source text sample を `static gate` / `interpreter` / `formal hook` へ段階接続し、repo-local policy と first theorem-side pilot、widen ordering、bridge-sketch reopen ordering、`e1` / `e21` actualization、`e3` guard comparison、plain bridge sketch actualization、maintenance close まで fixed 済み |

## 研究フェーズ（大局）

| Phase | rough % | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---:|---|---|---|---|---|
| Phase 0 | 94% | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / snapshot の整合維持 |
| Phase 1 | 96% | current L2 semantics stabilization | self-driven closeout fixed | 中 | 自走可能 | semantic core、invariant bridge、notation boundaryは fixed。final grammar / type / schema は later |
| Phase 2 | 98% | parser-free PoC / detached validation loop | self-driven closeout fixed | 中 | 自走可能 | compile/test/smoke gate、helper boundary、compare-only policyは fixed。bless / retention / exporter API は later |
| Phase 3 | 93% | parser boundary / first checker cut | self-driven freeze fixed | 中〜やや重い | 自走可能 | `287...290` で subset / reconnect freeze は fixed。later widen は reserve path |
| Phase 4 | 90% | shared-space / membership / practical example boundary | self-driven closeout fixed | 重い | 一部自走可能 | `121...125` current package と final catalog / later line の境界は fixed。final catalog は user spec 依存 |
| Phase 5 | 98% | static analysis / type / theorem prover / async-control boundary | self-driven closeout fixed | とても重い | 自走可能 | `297...298` で stop line と retained-later inventory は fixed。actual external contract は later |
| Phase 6 | 96% | actual parser / checker / runtime commitment と syntax-backed sample path | theorem-side compare-ready bridge sketch second reopen | 重い | 自走可能 | front-half compile-ready checkpoint、source path first packages、theorem-first review-unit pilot、authored-row widen sequencing、bridge-sketch ordering、`e1` / `e21` actualization、`e3` guard comparison、plain bridge sketch actualization、maintenance closeは fixed。残りは compare-ready bridge sketch second reopen と、その後段の `e3` actualization timing である |
| Phase 7 | 3% | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Prism / 上位アプリは user specification が要る |

## 現在の主線

1. **Phase 6 theorem-side compare-ready bridge sketch second reopen**
   - plain bridge sketch actualization の後段として compare-ready bridge sketch threshold を narrow reopen する。
2. **Phase 6 deferred `e3` actualization reopen timing**
   - theorem-side bridge line の後で `e3` widening をどこへ戻すかを narrow に固定する。
3. **Phase 6 actual `e3` authored-row reopen**
   - reopen timing fixed 後に `e3-option-admit-chain` を actual authored row として戻せる最小 package を比較する。

## いま自走で進めてよい範囲

### 着手可能

- theorem-side compare-ready bridge sketch second reopen
- deferred `e3` actualization reopen timing
- actual `e3` authored-row reopen

### 後段依存

- final parser grammar
- public parser / checker API の finalization
- richer host interface の全面 actualization
- actual type / theorem prover / protocol verifier integration
- `e3` theorem-side / formal-hook family widening

### 要仕様確認

- shared-space final activation family
- shared-space final authority / consistency / fairness catalog
- higher-layer application contract
- Mirrorea / Typed-Effect / Prism / 上位アプリの concrete goal

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書 / decision level / invariants | 93% | 89% | 73% | 着手可能 | 基礎境界はかなり安定 |
| Mir current L2 core semantics | 92% | 88% | 79% | 着手可能 | self-driven closeout は fixed。final grammar / type / schema は later |
| fallback / notation / representative examples | 91% | 88% | 69% | 着手可能 | explicit edge-row family と A2/A1 boundary は fixed。final lexical choice は later |
| parser-free PoC execution stack | 93% | 89% | 99% | 着手可能 | compile/test/smoke baseline と helper boundaryは fixed |
| detached export / validation loop | 99% | 98% | 99% | 着手可能 | compare-only / non-production helper と default candidate stop line は fixed |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 92% | 85% | 88% | 着手可能 | subset freeze と reconnect freeze は fixed。stage 3 reconnect は still later |
| first checker cut / helper-local compare family | 91% | 82% | 89% | 着手可能 | stage 1 + stage 2 bridge は fixed。`e19` / `E21` / `E22` は still later |
| source-backed sample corpus / verification ladder | 92% | 93% | 91% | 着手可能 | mapping / lowering / runner / ladder / policy / theorem-first pilot / widen sequencing / bridge-sketch ordering と `e1` / `e21` actualization、`e3` guard comparison、plain bridge sketch actualization は fixed。残りは compare-ready bridge sketch second reopen とその後段 reopen である |
| shared-space / dynamic membership boundary | 90% | 84% | 12% | 一部自走可能 | self-driven closeout は fixed。final catalog は user spec required |
| static analysis / type / theorem prover / async-control boundary | 99% | 98% | 51% | 着手可能 | `297...298` で closeout fixed。tool-neutral formal hook、review-unit pilot、bridge-sketch orderingは fixed。concrete tool bindingは still later |
| richer host interface / typed coverage carrier | 45% | 32% | 25% | 後段依存 | current phase では太らせない |
| Mirrorea / Typed-Effect / Prism / 上位アプリ | 16% | 11% | 5% | 要仕様確認 | higher-layer の具体仕様がまだ足りない |

## 現時点での大きい未解決問題

- これらは repo 全体では大きい未解決問題だが、current promoted line の直前 blocker ではない。

1. shared-space の final activation rule
2. authority placement の final shape
3. consistency mode catalog をどこまで language 側に持つか
4. fairness / RNG trust model
5. control-plane separated causal carrier の final actualization
6. final parser grammar / public checker boundary
7. actual parser/runtime sample runner の final public boundary
8. concrete formal tool binding / LLVM-family backend timing
9. `e3` widening を theorem-side bridge line の後でどこへ戻すか

## 次に進めるべき task

1. **Phase 6 theorem-side compare-ready bridge sketch second reopen** を immediate line として扱う
2. その後に deferred `e3` actualization reopen timing を扱う
3. さらに actual `e3` authored-row reopen を扱う

## 作業ログ（簡潔）

- 注記: この欄は **recent log** として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-12 20:10 JST — `docs/reports/0643` で post-task document consistency audit を閉じ、plain bridge sketch actualization が still-open に見える stale wording を `tasks.md` と relevant `plan/` から除去した。current line は compare-ready bridge sketch second reopen のまま維持した。
- 2026-04-12 20:04 JST — `docs/reports/0642` と `specs/examples/339...340` で plain proof-notebook bridge sketch actualization を閉じ、old theorem-line `specs/examples/140` の docs-only bridge shape を current theorem-side first actualization として再利用する cut を固定した。snapshot docs の current line は compare-ready bridge sketch second reopen に進めた。
- 2026-04-12 19:49 JST — `docs/reports/0641` と `specs/examples/337...338` で third widened row `e3` theorem-side / formal-hook guard comparison を閉じ、`e3` を deferred row のまま保つ current guard と next line の plain bridge sketch actualization を固定した。snapshot docs の current line は theorem-side plain bridge sketch actualization に進めた。
- 2026-04-12 19:33 JST — `docs/reports/0640` と `specs/examples/335...336` で second widened authored row `e21` actualization を閉じ、`e21-try-atomic-cut-frontier` を source row / runner accepted set / regression bundle / README ladder の current authored inventory に昇格した。snapshot docs の current line は `e3` theorem-side / formal-hook guard comparison に進めた。
- 2026-04-12 19:20 JST — `docs/reports/0639` と `specs/examples/333...334` で first widened authored row `e1` actualization を閉じ、`e1-place-atomic-cut` を source row / runner accepted set / regression bundle / README ladder の current authored inventory に昇格した。snapshot docs の current line は `e21` actualization に進めた。
- 2026-04-12 17:10 JST — `docs/reports/0638` で mirror sweep follow-up maintenance と document consistency audit を閉じ、snapshot docs の current line を `e1` actualization へ進めた。bridge-sketch ordering / widen sequencing / current next-step wording を `Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、abstract、sample README で同期した。
- 2026-04-12 17:06 JST — `specs/examples/331...332` を追加し、proof-notebook bridge-sketch reopen ordering を plain docs-only bridge sketch first / compare-ready bridge sketch second に固定した。authored-row actualization line を先に閉じる guard を明示し、current mainline は `mirror sweep follow-up maintenance` に切り替わった。
- 2026-04-12 16:49 JST — `specs/examples/329...330` を追加し、deferred authored-row widen sequencing を `e1 -> e21 -> e3` に固定した。`e1` / `e21` を current runtime formal-hook family の内側で先に扱い、`e3` は admit-family / theorem-side guard を伴う third slot に残した。
- 2026-04-12 15:08 JST — `docs/reports/0633` で post-task drift suppression / document consistency audit を記録し、`Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、Phase 5 / Phase 6 abstract の stale current-line wording を同期した。
