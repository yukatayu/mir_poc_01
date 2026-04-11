# progress

最終更新: 2026-04-12 00:03 JST

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
- **Phase 1** は `specs/examples/291...292` により self-driven closeout fixed と読んでよい。semantic core 自体は変えず、`specs/09` invariants と Phase 5 proof-obligation wording の橋、explicit edge-row family と A2/A1 boundary を narrow に固定した。
- **Phase 2** は `specs/examples/293...294` により self-driven closeout fixed と読んでよい。parser-free companion baseline の compile/test/smoke gate、helper boundary、detached loop compare-only policy は fixed し、reference update / bless、final retention/path policy、public exporter API は later に残している。
- **Phase 3** は reopen line の self-driven freeze が `specs/examples/287...290` で fixed 済みである。actual parser first tranche は stage 1 + stage 2 structural floor、first checker reconnect bridge は stage 1 summary + stage 2 try/rollback structural contract に留め、stage 3 request / admit / predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残している。
- **Phase 4** は `specs/examples/295...296` により self-driven closeout fixed と読んでよい。self-driven current package は `specs/examples/121...125` までで checkpoint close とし、final activation / authority / auth / identity / admission / consistency / fairness catalog は user-spec-required に、`delegated_provider_attestation` non-core line、control-plane separated carrier actualization、distributed fairness protocol、final operational realizationは later に残している。
- **Phase 5** は `specs/examples/297...298` により self-driven closeout fixed と読んでよい。checker-side verifier handoff surface docs-only mixed-row bridge、theorem-side retained bridge stop line、proof / protocol / runtime-policy inventory、retained-later lineを 1 本の closeout bundle に固定し、actual subject row / boundary-specific artifact / actual emitted verifier artifact / concrete tool binding / public checker migration / low-level memory-order family は still later に残している。
- **実装面の現在地** は still uneven である。`mir-semantics` には parser-free current L2 minimal interpreter と harness があり、`cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --list` で 46 tests を確認できる。`mir-ast` には `crates/mir-ast/src/current_l2.rs` として stage 1 option/chain と stage 2 try/fallback structural floor の non-production carrier が入り、`cargo test -p mir-ast` が通る。一方で `mir-runtime` / `mir-lsp` の public `src/lib.rs` は placeholder skeleton のままであり、actual checker / runtime compile path はまだ薄い。
- current main bottleneck は semantics の大崩れではなく、**Phase 6 front-half actual checker / runtime compile path actualization** と **formal tool binding をどこで narrow に選ぶか** である。

## compile-ready PoC の rough 読み

| 目標 | rough 進捗 | rough 残量 | 補足 |
|---|---:|---|---|
| parser-free current L2 PoC | 90%+ | maintenance closeout 数 package | `mir-semantics` 主線はすでに compile / test 可能 |
| Phase 6 前半 compile-ready minimal actual PoC | 68% 前後 | 2〜3 package 前後 / 1〜3週 | parser first tranche は fixed。残りは checker/runtime bridge、formal hook、checkpoint sweep |

## 研究フェーズ（大局）

| Phase | rough % | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---:|---|---|---|---|---|
| Phase 0 | 94% | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / snapshot の整合維持 |
| Phase 1 | 96% | current L2 semantics stabilization | self-driven closeout fixed | 中 | 自走可能 | semantic core、invariant bridge、notation boundaryは fixed。final grammar / type / schema は later |
| Phase 2 | 98% | parser-free PoC / detached validation loop | self-driven closeout fixed | 中 | 自走可能 | compile/test/smoke gate、helper boundary、compare-only policyは fixed。bless / retention / exporter API は later |
| Phase 3 | 93% | parser boundary / first checker cut | self-driven freeze fixed | 中〜やや重い | 自走可能 | `287...290` で subset / reconnect freeze は fixed。later widen は reserve path |
| Phase 4 | 90% | shared-space / membership / practical example boundary | self-driven closeout fixed | 重い | 一部自走可能 | `121...125` current package と final catalog / later line の境界は fixed。final catalog は user spec 依存 |
| Phase 5 | 98% | static analysis / type / theorem prover / async-control boundary | self-driven closeout fixed | とても重い | 自走可能 | `297...298` で stop line と retained-later inventory は fixed。actual external contract は later |
| Phase 6 | 42% | actual parser / checker / runtime commitment | parser first tranche fixed / checker-runtime thin | 重い | 自走可能 | `mir-ast` stage 1 / 2 carrier は actualize 済み。runtime bridge と formal hook が主残量 |
| Phase 7 | 3% | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Prism / 上位アプリは user specification が要る |

## 現在の主線

1. **Phase 6 front-half actual checker / runtime skeleton first tranche**
   - parsed subset から checker floor / runtime skeleton までの compile path を通す。
2. **compile-ready verification and formal hook**
   - cargo / smoke gate と tool-neutral or narrow tool first cut を揃える。
3. **Phase 6 compile-ready checkpoint drift suppression / mirror sweep**
   - specs / plan / research_abstract / snapshot を checkpoint wording に揃える。

## いま自走で進めてよい範囲

### 着手可能

- Phase 6 front-half actual checker / runtime skeleton first tranche
- compile-ready verification and formal hook
- Phase 6 compile-ready checkpoint drift suppression / mirror sweep
- checkpoint close 済み package の drift suppression

### 後段依存

- final parser grammar
- public parser / checker API の finalization
- richer host interface の全面 actualization
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
| Mir current L2 core semantics | 92% | 88% | 79% | 着手可能 | self-driven closeout は fixed。final grammar / type / schema は later |
| fallback / notation / representative examples | 91% | 88% | 69% | 着手可能 | explicit edge-row family と A2/A1 boundary は fixed。final lexical choice は later |
| parser-free PoC execution stack | 93% | 89% | 99% | 着手可能 | compile/test/smoke baseline と helper boundaryは fixed |
| detached export / validation loop | 99% | 98% | 99% | 着手可能 | compare-only / non-production helper と default candidate stop line は fixed |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 92% | 85% | 88% | 着手可能 | subset freeze と reconnect freeze は fixed。stage 3 reconnect は still later |
| first checker cut / helper-local compare family | 91% | 82% | 89% | 着手可能 | stage 1 + stage 2 bridge は fixed。`e19` / `E21` / `E22` は still later |
| shared-space / dynamic membership boundary | 90% | 84% | 12% | 一部自走可能 | self-driven closeout は fixed。final catalog は user spec required |
| static analysis / type / theorem prover / async-control boundary | 99% | 98% | 40% | 着手可能 | `297...298` で closeout fixed。tool binding は still later |
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

## 次に進めるべき task

1. **Phase 6 front-half actual checker / runtime skeleton first tranche** を immediate line として扱う
2. その後に **compile-ready verification and formal hook** を checkpoint close まで揃える
3. その次に **Phase 6 compile-ready checkpoint drift suppression / mirror sweep** を閉じる

## 作業ログ（簡潔）

- 注記: この欄は **recent log** として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-12 00:03 JST — `specs/examples/299...300` を追加し、Phase 6 parser first tranche を `carrier_kind + accepted_surface_refs + code_anchor_refs + retained_later_refs` minimum に固定した。`mir-ast/src/current_l2.rs` へ stage 1 option/chain と stage 2 try/fallback structural floor を non-production carrier として actualize し、stage 3 helper は retained-later evidence に残した。next promoted line は `Phase 6 actual checker / runtime skeleton first tranche` に切り替わった。
- 2026-04-11 23:06 JST — `specs/examples/297...298` を追加し、Phase 5 closeout を `closeout_kind + verifier_handoff_surface_ref + theorem_retained_bridge_stop_ref + boundary_inventory_ref + retained_later_refs` bundle に固定した。checker-side verifier handoff surface、theorem retained bridge stop line、proof / protocol / runtime-policy inventory を current package に残し、actual artifact / tool binding / low-level memory-order family は retained-later に分離した。next promoted line は `Phase 6 actual parser / AST carrier first tranche` に切り替わった。
- 2026-04-11 22:57 JST — `specs/examples/295...296` を追加し、Phase 4 self-driven closeout を `closeout_kind + current_package_refs + user_spec_required_catalog_refs + retained_later_refs` bundle に固定した。`specs/examples/121...125` を checkpoint-close current package とし、`append_friendly_room_with_rng_capability` row は optional capability source に留め、final catalog は user-spec-required に分離した。next promoted line は `Phase 5 proof / protocol / runtime-policy handoff closeout` に切り替わった。
- 2026-04-11 22:45 JST — `specs/examples/293...294` を追加し、Phase 2 closeout を `closeout_kind + compile_gate_refs + helper_boundary_refs + detached_loop_policy_refs + retained_later_refs` bundle に固定した。parser-free companion baseline の compile/test/smoke gate と detached loop compare-only policy を明示し、reference update / bless、final retention/path policy、public exporter API は later に残した。next promoted line は `Phase 4 shared-space self-driven closeout` に切り替わった。
- 2026-04-11 22:24 JST — `specs/examples/291...292` を追加し、Phase 1 closeout を `closeout_kind + core_semantics_refs + invariant_bridge_refs + notation_status_refs + retained_later_refs` bundle に固定した。`specs/09` invariants と Phase 5 proof-obligation row の橋、explicit edge-row family と A2/A1 boundary を明示し、next promoted line は `Phase 2 parser-free PoC / detached loop closeout` に切り替わった。
- 2026-04-11 21:12 JST — Phase 3 reopen line として `specs/examples/289...290` を追加し、parser-to-checker reconnect freeze を `reconnect_kind + parser_subset_freeze_ref + checker_floor_refs + retained_helper_refs` bundle に固定した。first checker bridge は stage 1 summary + stage 2 try/rollback structural contract に留め、stage 3 request/predicate reconnect、`e19` redesign、`E21` / `E22` contrast は retained-later line に残し、next promoted line は `Phase 1 semantics / invariants / notation final sweep` に切り替わった。
- 2026-04-11 20:48 JST — Phase 3 reopen line として `specs/examples/287...288` を追加し、minimal parser subset freeze を `freeze_kind + accepted_cluster_refs + reject_cluster_refs + retention_floor_refs` bundle に固定した。accepted cluster は stage 1 + stage 2 structural floor、stage 3 request/admit/predicate line は retained-later floor に残し、next promoted line は `minimal-parser-subset-freeze-ready parser-to-checker-reconnect-freeze comparison` に切り替わった。
- 2026-04-11 20:41 JST — Phase 5 checker-side current promoted line として `specs/examples/285...286` を追加し、verifier handoff surface を `handoff_surface_kind + public_checker_boundary_ref + proof_obligation_matrix_ref + handoff_artifact_mode` docs-only mixed-row bridge に留めた。actual subject row / dedicated handoff artifact / actual emitter は still later に残し、next promoted line は `minimal-verifier-handoff-surface-ready minimal-parser-subset-freeze comparison` に切り替わった。
- 2026-04-11 20:22 JST — `tasks.md` を全面書き直し、Phase 1〜5 closeout と Phase 6 front-half compile-ready minimal PoC までの ordered package map を fixed した。`mir-semantics` の parser-free compile evidence と `mir-ast` / `mir-runtime` placeholder 状態を踏まえ、actual compile-ready PoC は rough 25% / 10 package 前後 / 4〜8週と読む snapshot に更新した。
- 2026-04-09 13:20 JST — detached validation loop の second friction tranche として `compare-fixture-aggregates` を追加し、single-fixture aggregate 同士の compare を noisy な full-vs-single contrast から分離した。次の friction は reference update / longer compare triage 側に寄ることを確認した。
- 2026-04-10 08:44 JST — stage 3 request / predicate / attachment branch の current snapshot を整理し、Phase 3 を reserve path として読む phase gate を固定した。next は shared-space の identity / auth / admission / fairness line を narrow に比較する段階。
