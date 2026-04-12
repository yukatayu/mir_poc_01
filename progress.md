# progress

最終更新: 2026-04-12 16:49 JST

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
- `specs/examples/329...330` により、deferred authored-row widen sequencing も fixed 済みであり、current first choice は `e1 -> e21 -> e3` である。`e1` / `e21` は current runtime formal-hook family の内側で先に扱い、`e3` は admit-family / theorem-side guard を伴う third slot に残す。
- current main bottleneck は semantics の大崩れではなく、**proof-notebook bridge-sketch reopen ordering をどこで止め、follow-up maintenance の後に first widened row `e1` actualization へどう入るか** である。

## compile-ready PoC の rough 読み

| 目標 | rough 進捗 | rough 残量 | 補足 |
|---|---:|---|---|
| parser-free current L2 PoC | 90%+ | maintenance closeout 数 package | `mir-semantics` 主線はすでに compile / test 可能 |
| Phase 6 前半 compile-ready minimal actual PoC | 96%+ | maintenance / reopen only | compile-ready checkpoint close は fixed と読める |
| fixed-subset syntax-backed sample verification milestone | 91%前後 | bridge-sketch ordering、follow-up maintenance、`e1 -> e21 -> e3` actualization line | source text sample を `static gate` / `interpreter` / `formal hook` へ段階接続し、repo-local policy と first theorem-side pilot まで fixed 済み。残りは widened authored row actualization と theorem-side bridge line |

## 研究フェーズ（大局）

| Phase | rough % | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---:|---|---|---|---|---|
| Phase 0 | 94% | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / snapshot の整合維持 |
| Phase 1 | 96% | current L2 semantics stabilization | self-driven closeout fixed | 中 | 自走可能 | semantic core、invariant bridge、notation boundaryは fixed。final grammar / type / schema は later |
| Phase 2 | 98% | parser-free PoC / detached validation loop | self-driven closeout fixed | 中 | 自走可能 | compile/test/smoke gate、helper boundary、compare-only policyは fixed。bless / retention / exporter API は later |
| Phase 3 | 93% | parser boundary / first checker cut | self-driven freeze fixed | 中〜やや重い | 自走可能 | `287...290` で subset / reconnect freeze は fixed。later widen は reserve path |
| Phase 4 | 90% | shared-space / membership / practical example boundary | self-driven closeout fixed | 重い | 一部自走可能 | `121...125` current package と final catalog / later line の境界は fixed。final catalog は user spec 依存 |
| Phase 5 | 98% | static analysis / type / theorem prover / async-control boundary | self-driven closeout fixed | とても重い | 自走可能 | `297...298` で stop line と retained-later inventory は fixed。actual external contract は later |
| Phase 6 | 90% | actual parser / checker / runtime commitment と syntax-backed sample path | proof-notebook bridge-sketch reopen ordering | 重い | 自走可能 | front-half compile-ready checkpoint、source path first packages、theorem-first review-unit pilot、authored-row widen sequencingは fixed。残りは bridge-sketch ordering、follow-up maintenance、`e1 -> e21 -> e3` actualization line |
| Phase 7 | 3% | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Prism / 上位アプリは user specification が要る |

## 現在の主線

1. **Phase 6 proof-notebook bridge-sketch reopen ordering**
   - review-unit current cut の後で theorem-side bridge sketch をいつ reopen するかを比較する。
2. **Phase 0 / 6 drift suppression**
   - snapshot 文書の current line / next line / retained-later line を follow-up maintenance として揃える。
3. **Phase 6 first widened authored row actualization**
   - current widen order `e1 -> e21 -> e3` の first slot `e1` を actual source row / runner / regression に反映する。

## いま自走で進めてよい範囲

### 着手可能

- proof-notebook bridge-sketch reopen ordering
- checkpoint close 済み package の drift suppression
- first widened authored row `e1` actualization

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
| source-backed sample corpus / verification ladder | 89% | 90% | 86% | 着手可能 | mapping / lowering / runner / ladder / policy / theorem-first pilot / widen sequencing は fixed。bridge sketch と widened actual row が残る |
| shared-space / dynamic membership boundary | 90% | 84% | 12% | 一部自走可能 | self-driven closeout は fixed。final catalog は user spec required |
| static analysis / type / theorem prover / async-control boundary | 99% | 98% | 50% | 着手可能 | `297...298` で closeout fixed。tool-neutral formal hook と theorem-first pilot は actualize 済みで、bridge sketch / concrete tool binding は still later |
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
9. `e3` widening を current theorem-side family とどう接続するか

## 次に進めるべき task

1. **Phase 6 proof-notebook bridge-sketch reopen ordering** を immediate line として扱う
2. drift suppression は follow-up maintenance として継続する
3. その後に first widened authored row `e1` actualization を扱う

## 作業ログ（簡潔）

- 注記: この欄は **recent log** として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-12 16:49 JST — `specs/examples/329...330` を追加し、deferred authored-row widen sequencing を `e1 -> e21 -> e3` に固定した。`e1` / `e21` を current runtime formal-hook family の内側で先に扱い、`e3` は admit-family / theorem-side guard を伴う third slot に残し、current mainline は `proof-notebook bridge-sketch reopen ordering` に切り替わった。
- 2026-04-12 15:08 JST — `docs/reports/0633` で post-task drift suppression / document consistency audit を記録し、`Documentation.md`、`progress.md`、`tasks.md`、relevant `plan/`、Phase 5 / Phase 6 abstract の stale current-line wording を同期した。current mainline は `deferred authored-row widen sequencing` に切り替わった。
- 2026-04-12 14:49 JST — `specs/examples/327...328` と `docs/reports/0630` を追加し、tool-neutral formal hook artifact を入力にする non-production `proof_notebook_review_unit` theorem-side pilot を fixed した。supported pair は runtime 1 件 + static 2 件に留め、bridge sketch / compare-bless metadata / concrete theorem-model-check tool binding は still later に残した。
- 2026-04-12 14:21 JST — `specs/examples/325...326` と `docs/reports/0629` を追加し、source-sample authoring / bless / regression policy を fixed した。`.docs/current-l2-source-sample-authoring-policy.md` と `scripts/current_l2_source_sample_regression.py` を repo-local policy anchor に置いた。
- 2026-04-12 14:02 JST — `specs/examples/323...324` と `docs/reports/0628` を追加し、first authored trio `e2` / `e4` / `e23` の verification ladder を fixed した。`e1` / `e3` / `e21` は source target only / not yet authored row に残した。
