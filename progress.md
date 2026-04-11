# progress

最終更新: 2026-04-11 20:22 JST

## この文書について

- この文書は repo 全体の **簡潔な進捗スナップショット** である。
- 規範判断の正本は `specs/`、長期比較と repository memory は `plan/`、詳細な経緯は `docs/reports/` に置く。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度
- phase 表の `%` は、その phase 全体に対する **rough overall estimate** であり、3 軸の平均を厳密計算した値ではない。後続 research や rollback で戻ることがある。

## 現在の要約

- **current L2 semantics** は、Phase 6 前半の minimal compile-ready PoC に入るための semantic floor としては十分安定している。
- **Phase 0 / 1 / 2** は broad mainline では closeout 寄りに入っている。parser-free PoC、detached validation loop、fixture authoring の operational baseline は成立済みであり、残りは drift suppression と closeout audit である。
- **Phase 3** は、長く reserve path だったが、Phase 6 前半へ実際に入るなら reopen が必要である。stage 1 / 2 / 3 の parser spike evidence は private / test-only にある一方、`mir-ast` public crate はまだ placeholder skeleton のままである。
- **Phase 4** は `specs/examples/121...125` までで current package close である。authoritative room baseline、working subset row、minimal witness core、delegated-provider practical cut、control-plane threshold comparison までは source-backed だが、Phase close と呼ぶには self-driven closeout と user-spec-required final catalog の切り分けをもう一段明示したい。
- **Phase 5** は `specs/examples/126...284` までで current package close である。theorem-side retained bridge は `retained_payload_body_materialization_theorem_export_handoff_transport_channel_body` を current stop line にし、checker-side では public checker boundary を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` docs-only parser-front bundle に留める current cut まで進んでいる。ここから Phase 6 前半へ入る immediate next line は **`minimal-public-checker-boundary-ready verifier-handoff-surface comparison`** である。
- **実装面の現在地** は uneven である。`mir-semantics` には parser-free current L2 minimal interpreter と harness があり、`cargo test -p mir-semantics --test current_l2_minimal_interpreter -- --list` で 46 tests、`cargo test -p mir-ast --test current_l2_stage1_parser_spike -- --list` で 14 tests、`cargo test -p mir-ast --test current_l2_stage2_try_rollback_spike -- --list` で 3 tests を確認できる。一方で `mir-ast` / `mir-runtime` / `mir-lsp` の public `src/lib.rs` は placeholder skeleton であり、actual parser / checker / runtime compile path はまだ薄い。
- current main bottleneck は semantics の大崩れではなく、**verifier handoff surface の凍結**、**actual parser subset の凍結**、**formal tool binding 前の proof / model-check handoff 整理**、**Phase 6 front-half compile path actualization** である。

## compile-ready PoC の rough 読み

| 目標 | rough 進捗 | rough 残量 | 補足 |
|---|---:|---|---|
| parser-free current L2 PoC | 90%+ | maintenance closeout 数 package | `mir-semantics` 主線はすでに compile / test 可能 |
| Phase 6 前半 compile-ready minimal actual PoC | 25% 前後 | 10 package 前後 / 4〜8週 | docs / test-only spike / parser-free harness はあるが public crates actualization はこれから |

## 研究フェーズ（大局）

| Phase | rough % | 主眼 | 現在位置 | 重さ | 自走可否 | 補足 |
|---|---:|---|---|---|---|---|
| Phase 0 | 94% | repository memory / decision boundary | maintenance | 低い | 自走可能 | `specs/` / `plan/` / report / snapshot の整合維持 |
| Phase 1 | 90% | current L2 semantics stabilization | closeout 前 | 中 | 自走可能 | semantic core は安定。残りは invariants / notation / proof-obligation sweep |
| Phase 2 | 96% | parser-free PoC / detached validation loop | closeout 前 | 中 | 自走可能 | mainline は成立。残りは compile gate / retention policy closeout |
| Phase 3 | 84% | parser boundary / first checker cut | reopen 準備済み | 中〜やや重い | 自走可能 | private staged spike はある。Phase 6 前半のために reserve path を reopen する段階 |
| Phase 4 | 68% | shared-space / membership / practical example boundary | self-driven closeout 前 | 重い | 一部自走可能 | `121...125` は current package close。final catalog は user spec / later pressure 依存 |
| Phase 5 | 90% | static analysis / type / theorem prover / async-control boundary | closeout 前 | とても重い | 自走可能 | `126...284` package close の先に verifier handoff / proof-model-check handoff closeout が残る |
| Phase 6 | 12% | actual parser / checker / runtime commitment | entry criteria visible | 重い | 後段依存 | parser-free harness と test-only spike はあるが public crates actualization はまだ薄い |
| Phase 7 | 3% | higher-layer integration / domain realization | 未着手 | とても重い | 要仕様確認 | Mirrorea / Prism / 上位アプリは user specification が要る |

## 現在の主線

1. **Phase 5 verifier-handoff gate**
   - `minimal-public-checker-boundary-ready verifier-handoff-surface comparison` を immediate next line に置く。
2. **Phase 3 reopen for Phase 6 front-half**
   - minimal parser subset freeze と parser-to-checker reconnect freeze を取りに行く。
3. **Phase 1 / 2 / 4 / 5 closeout sweep**
   - self-driven scope を phase-complete と読める snapshot まで揃える。
4. **Phase 6 front-half actualization**
   - `mir-ast` / `mir-semantics` / `mir-runtime` をまたぐ compile-ready minimal PoC へ入る。

## いま自走で進めてよい範囲

### 着手可能

- current L2 semantics の narrow regression
- parser-free PoC / detached validation loop の maintenance residual
- Phase 3 minimal parser subset freeze / parser-to-checker reconnect freeze
- Phase 5 checker-cluster matrix / evidence attachment line の docs-first comparison
- shared-space の docs-first boundary comparison と practical example 整理
- checkpoint close 済み package の drift suppression

### 後段依存

- final parser grammar
- public checker API
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
| Mir current L2 core semantics | 86% | 81% | 79% | 着手可能 | mainline には十分安定 |
| fallback / notation / representative examples | 86% | 83% | 69% | 着手可能 | final grammar は still later |
| parser-free PoC execution stack | 90% | 85% | 98% | 着手可能 | runtime / bundle / batch / selection / profile は揃っている |
| detached export / validation loop | 98% | 97% | 99% | 着手可能 | current self-driven friction reduction は checkpoint close |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold / smoke convenience は強い |
| parser boundary / staged parser spike | 88% | 81% | 86% | 着手可能 | Phase 6 前半に向けて minimal subset freeze を reopen する段階 |
| first checker cut / helper-local compare family | 89% | 79% | 88% | 着手可能 | reconnect subline は freeze threshold まで整理済みで、次は actual parser gate と接続する |
| shared-space / dynamic membership boundary | 83% | 76% | 12% | 一部自走可能 | `121...125` まで current package close |
| static analysis / type / theorem prover / async-control boundary | 99% | 98% | 35% | 着手可能 | `126...284` まで current package close。next は minimal-public-checker-boundary-ready verifier-handoff-surface comparison |
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

1. **Phase 5 verifier-handoff gate** として `minimal-public-checker-boundary-ready verifier-handoff-surface comparison` を扱う
2. その次に **Phase 3 reopen** として minimal parser subset freeze / parser-to-checker reconnect freeze を扱う
3. その後に **Phase 1 / 2 / 4 / 5 closeout sweep** を順に進め、self-driven scope を phase-complete snapshot へ揃える
4. 最後に **Phase 6 front-half** の actual parser / checker / runtime minimal PoC を compile-ready checkpoint まで actualize する

## 作業ログ（簡潔）

- 注記: この欄は **recent log** として保つ。詳細な履歴は `docs/reports/` を正本にする。
- 2026-04-11 20:22 JST — `tasks.md` を全面書き直し、Phase 1〜5 closeout と Phase 6 front-half compile-ready minimal PoC までの ordered package map を fixed した。`mir-semantics` の parser-free compile evidence と `mir-ast` / `mir-runtime` placeholder 状態を踏まえ、actual compile-ready PoC は rough 25% / 10 package 前後 / 4〜8週と読む snapshot に更新した。
- 2026-04-09 13:20 JST — detached validation loop の second friction tranche として `compare-fixture-aggregates` を追加し、single-fixture aggregate 同士の compare を noisy な full-vs-single contrast から分離した。次の friction は reference update / longer compare triage 側に寄ることを確認した。
- 2026-04-10 08:44 JST — stage 3 request / predicate / attachment branch の current snapshot を整理し、Phase 3 を reserve path として読む phase gate を固定した。next は shared-space の identity / auth / admission / fairness line を narrow に比較する段階。
- 2026-04-10 09:19 JST — Phase 5 theorem-line later package を `archive_bless_update_policy_ref` まで current first choice に伸ばし、next later reopen を retained archive payload comparison に置いた。Phase 5 は current package close と読める状態になった。
- 2026-04-10 09:50 JST — `tasks.md` と `progress.md` を current snapshot として全面整理し、review finding に合わせて `plan/11` / `plan/17` も current promoted line へ追随させた。next は retained archive payload comparison を promoted line として進める段階。
- 2026-04-10 10:23 JST — Phase 5 theorem-line later reopen として `retained_archive_payload_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は archive retention layout comparison で、Phase 5 は `126...180` まで current package close と読める状態になった。
- 2026-04-10 11:11 JST — Phase 5 theorem-line later reopen として `archive_retention_layout_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained archive payload body family comparison で、Phase 5 は `126...181` まで current package close と読める状態になった。
- 2026-04-10 11:16 JST — Phase 5 theorem-line later reopen として `retained_archive_payload_body_family_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は retained payload materialization family comparison で、Phase 5 は `126...182` まで current package close と読める状態になった。
- 2026-04-10 11:26 JST — Phase 5 theorem-line later reopen として `retained_payload_materialization_family_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained payload body materialization detail comparison で、Phase 5 は `126...183` まで current package close と読める状態になった。
- 2026-04-10 11:35 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_detail_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained payload body materialization payload comparison で、Phase 5 は `126...184` まで current package close と読める状態になった。
- 2026-04-10 13:40 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_payload_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained payload body materialization carrier detail comparison で、Phase 5 は `126...185` まで current package close と読める状態になった。
- 2026-04-10 13:56 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_carrier_detail_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual retained payload body materialization carrier payload comparison で、Phase 5 は `126...186` まで current package close と読める状態になった。
- 2026-04-10 14:06 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_carrier_payload_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual bless-side / update-side row split comparison で、Phase 5 は `126...187` まで current package close と読める状態になった。
- 2026-04-10 14:37 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_bless_update_split_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual bless-side / update-side row pair comparison で、Phase 5 は `126...188` まで current package close と読める状態になった。
- 2026-04-10 14:49 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_bless_update_row_pair_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual bless-side row / update-side row ref family comparison で、Phase 5 は `126...189` まで current package close と読める状態になった。
- 2026-04-11 14:48 JST — Phase 5 checker-side line として `specs/examples/259...261` を追加し、`typed_reason_family_hint` を optional attachment、payload を `family_refs[]` minimal bundle、multi-family row には lightweight `coverage_state` までに留める cut を固定した。next promoted line は `typed-family-coverage-state-ready supported-kind-summary threshold` に切り替わった。
- 2026-04-10 14:59 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_bless_update_row_ref_family_ref` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は actual bless-side row / update-side row dual-ref comparison で、Phase 5 は `126...190` まで current package close と読める状態になった。
- 2026-04-10 15:54 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_bless_update_pair` を current first choice に追加し、review / mirror sweep / validation まで完了した。next promoted line は consumer-visible-pair-ready boundary-specific handoff pair comparison で、Phase 5 は `126...193` まで current package close と読める状態になった。
- 2026-04-10 16:17 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_boundary_handoff_pair_ref` を current first choice に追加し、mirror sweep と docs validation を通した。next promoted line は boundary-specific-handoff-pair-ready actual handoff pair shape comparison で、Phase 5 は `126...194` まで current package close と読める状態になった。
- 2026-04-10 16:28 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_boundary_handoff_pair` を current first choice に追加し、actual handoff pair shape を retained bridge に narrow に actualize した。next promoted line は actual-handoff-pair-shape-ready boundary-specific handoff artifact row comparison で、Phase 5 は `126...195` まで current package close と読める状態になった。
- 2026-04-10 16:39 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_boundary_handoff_artifact_row` を current first choice に追加し、boundary-specific handoff artifact row を theorem-side retained bridge に narrow に actualize した。next promoted line は boundary-specific-handoff-artifact-row-ready external-contract-facing handoff row comparison で、Phase 5 は `126...196` まで current package close と読める状態になった。
- 2026-04-10 16:50 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_handoff_row` を current first choice に追加し、external-contract-facing handoff row を theorem-side retained bridge に narrow に actualize した。next promoted line は external-contract-facing-handoff-row-ready actual external contract comparison で、Phase 5 は `126...197` まで current package close と読める状態になった。
- 2026-04-10 17:00 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_contract` を current first choice に追加し、actual external contract を theorem-side retained bridge に narrow に actualize した。next promoted line は actual-external-contract-ready consumer-specific external contract payload comparison で、Phase 5 は `126...198` まで current package close と読める状態になった。
- 2026-04-10 17:16 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_contract_payload` を current first choice に追加し、`proof_notebook` first consumer 向け minimal payload を theorem-side retained bridge に narrow に actualize した。next promoted line は external-contract-payload-ready payload enrichment / second-consumer-pressure comparison で、Phase 5 は `126...199` まで current package close と読める状態になった。
- 2026-04-10 17:27 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_contract_proof_hint` を current first choice に追加し、`proof_notebook` first consumer 向け minimal proof hint を theorem-side retained bridge に narrow に actualize した。next promoted line は proof-hint-ready consumer-hint / second-consumer-pressure comparison で、Phase 5 は `126...200` まで current package close と読める状態になった。
- 2026-04-10 17:57 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_contract_consumer_hint` を current first choice に追加し、`proof_notebook` first consumer 向け minimal consumer hint を theorem-side retained bridge に narrow に actualize した。next promoted line は consumer-hint-ready second-consumer-pressure comparison で、Phase 5 は `126...201` まで current package close と読める状態になった。
- 2026-04-10 17:59 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_external_contract_second_consumer_pressure` を current first choice に追加し、second consumer pressure 自体を theorem-side retained bridge に symbolic marker として narrow に actualize した。next promoted line は second-consumer-pressure-ready proof-assistant-adapter contract comparison で、Phase 5 は `126...202` まで current package close と読める状態になった。
- 2026-04-10 18:19 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_proof_assistant_adapter_contract` を current first choice に追加し、actual `proof_assistant_adapter` contract 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は proof-assistant-adapter-contract-ready theorem-export-checker-pressure comparison で、Phase 5 は `126...203` まで current package close と読める状態になった。
- 2026-04-10 18:33 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_pressure` を current first choice に追加し、`theorem_export_checker` pressure 自体を theorem-side retained bridge に symbolic marker として narrow に actualize した。next promoted line は theorem-export-checker-pressure-ready checker-facing contract comparison で、Phase 5 は `126...204` まで current package close と読める状態になった。
- 2026-04-10 20:23 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_contract` を current first choice に追加し、actual checker-facing contract 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は theorem-export-checker-contract-ready exported-checker-payload-pressure comparison で、Phase 5 は `126...205` まで current package close と読める状態になった。
- 2026-04-10 20:38 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_payload_pressure` を current first choice に追加し、exported checker payload pressure 自体を theorem-side retained bridge に symbolic marker として narrow に actualize した。next promoted line は theorem-export-checker-payload-pressure-ready actual-exported-checker-payload comparison で、Phase 5 は `126...206` まで current package close と読める状態になった。
- 2026-04-10 20:43 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_payload` を current first choice に追加し、actual exported checker payload 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は theorem-export-checker-payload-ready checker-result-materialization-family comparison で、Phase 5 は `126...207` まで current package close と読める状態になった。
- 2026-04-10 21:07 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_result_materialization_family` を current first choice に追加し、checker result materialization family 自体を theorem-side retained bridge に symbolic marker として narrow に actualize した。next promoted line は actual-exported-checker-payload-ready actual-checker-result-payload comparison で、Phase 5 は `126...208` まで current package close と読める状態になった。
- 2026-04-10 21:21 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_result_payload` を current first choice に追加し、actual checker result payload 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は checker-result-materialization-family-ready checker-verdict-carrier-detail comparison で、Phase 5 は `126...209` まで current package close と読める状態になった。
- 2026-04-10 21:24 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_verdict_carrier_detail` を current first choice に追加し、checker verdict carrier detail 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は checker-verdict-carrier-detail-ready checker-verdict-payload-family comparison で、Phase 5 は `126...210` まで current package close と読める状態になった。
- 2026-04-11 15:14 JST — Phase 5 checker-side line として `specs/examples/262...264` を追加し、`supported kind` summary は current checker-cluster matrix に足さず、actual checker payload family を `payload_family_kind + source_refs` minimal bundle まで docs-first に切る cut を固定した。next promoted line は `minimal-checker-payload-family-ready checker-payload-row-family comparison` に切り替わった。
- 2026-04-10 21:40 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_verdict_payload_family` を current first choice に追加し、checker verdict payload family marker 自体を theorem-side retained bridge に narrow に actualize した。next promoted line は checker-verdict-payload-family-ready checker-verdict-witness-family comparison で、Phase 5 は `126...211` まで current package close と読める状態になった。
- 2026-04-11 00:57 JST — `progress.md` の phase table に rough overall percent を追加し、Phase 0〜7 の進み具合を current snapshot として見やすく補正した。規範判断は変えず、next promoted line は引き続き Phase 5 の checker-verdict-witness-family comparison である。
- 2026-04-11 01:07 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_verdict_witness_family` と `retained_payload_body_materialization_theorem_export_checker_verdict_transport_family` を current first choice に追加し、checker verdict witness / transport family marker までを theorem-side retained bridge に narrow に actualize した。next promoted line は checker-verdict-transport-family-ready checker-verdict-transport-carrier-detail comparison で、Phase 5 は `126...213` まで current package close と読める状態になった。
- 2026-04-11 10:07 JST — Phase 5 theorem-line later reopen として `retained_payload_body_materialization_theorem_export_checker_verdict_transport_carrier_detail`、`..._transport_payload`、`..._transport_receipt`、`..._transport_channel_body` を current first choice に追加し、checker verdict transport line を channel body まで theorem-side retained bridge に narrow に actualize した。next promoted line は checker-verdict-transport-channel-body-ready low-level-memory-order-family comparison で、Phase 5 は `126...217` まで current package close と読める状態になった。
- 2026-04-11 10:39 JST — Phase 5 theorem-line later reopen として `specs/examples/218` を追加し、checker verdict transport channel body を theorem-line retained bridge の current stop line に固定した。low-level memory-order family は bridge に入れず、next promoted line は checker-verdict-transport-channel-body-ready higher-level-async-control-family comparison へ切り替わり、Phase 5 は `126...218` まで current package close と読める状態になった。
- 2026-04-11 10:43 JST — Phase 5 theorem-line later reopen として `specs/examples/219` と `220` を追加し、higher-level async-control family の current first cut を `authority_serial_transition_family` に固定した。theorem-side retained bridge は `retained_payload_body_materialization_theorem_export_authority_serial_transition_family` まで current first choice に入り、next promoted line は authority-serial-transition-family-ready authority-serial-transition-contract comparison に切り替わり、Phase 5 は `126...220` まで current package close と読める状態になった。
- 2026-04-11 11:00 JST — reviewer completion を反映し、`plan/12` の stale async-control wording、Phase 5 research abstract の current-state lag、report hygiene を補正した。Phase 5 の current promoted line と rough progress 読みは維持しつつ、`126...220` package を review closeout まで揃えた。
- 2026-04-11 11:06 JST — Phase 5 theorem-line later reopen として `specs/examples/221`〜`224` を追加し、`authority_serial_transition_family` の次段を minimal contract row と owner-slot detail まで narrow に actualize した。next promoted line は authority-owner-ref-ready authority-transition-stage-family comparison に切り替わり、Phase 5 は `126...224` まで current package close と読める状態になった。
- 2026-04-11 11:06 JST — 同 tranche で `specs/examples/225` と `226` を追加し、authority transition line を symbolic stage family まで narrow に actualize した。next promoted line は minimal-authority-transition-stage-family-ready authority-transition-stage-sequence-shape comparison に切り替わり、Phase 5 は `126...226` まで current package close と読める状態になった。
- 2026-04-11 11:20 JST — Phase 5 theorem-line later reopen として `specs/examples/227` と `228` を追加し、authority transition line を actual ordered stage sequence まで narrow に actualize した。next promoted line は minimal-authority-transition-stage-sequence-ready stage-local-obligation-family comparison に切り替わり、Phase 5 は `126...228` まで current package close と読める状態になった。
- 2026-04-11 11:30 JST — `specs/examples/227...228` package の report / review closeout を完了し、0554 report の evidence と consulted-doc provenance を補正した。Phase 5 の current promoted line と checkpoint 読み自体は維持されている。
- 2026-04-11 11:32 JST — Phase 5 theorem-line later reopen として `specs/examples/229` と `230` を追加し、authority transition line を symbolic stage-local obligation family まで narrow に actualize した。next promoted line は minimal-authority-stage-local-obligation-family-ready stage-local-obligation-row-detail comparison に切り替わり、Phase 5 は `126...230` まで current package close と読める状態になった。
- 2026-04-11 11:47 JST — `specs/examples/229...230` package の validation と local review fallback closeout を完了し、snapshot を `126...230` package close に揃えた。next promoted line は minimal-authority-stage-local-obligation-family-ready stage-local-obligation-row-detail comparison のままである。
- 2026-04-11 11:47 JST — Phase 5 theorem-line later reopen として `specs/examples/231` と `232` を追加し、authority transition line を actual stage-local obligation row detail まで narrow に actualize した。next promoted line は minimal-authority-stage-local-obligation-row-detail-ready authority-handoff-epoch-ref comparison に切り替わり、Phase 5 は `126...232` まで current package close と読める状態になった。
- 2026-04-11 11:52 JST — `specs/examples/231...232` package の validation と local review fallback closeout を完了し、snapshot を `126...232` package close に揃えた。next promoted line は minimal-authority-stage-local-obligation-row-detail-ready authority-handoff-epoch-ref comparison のままである。
- 2026-04-11 11:52 JST — Phase 5 theorem-line later reopen として `specs/examples/233` と `234` を追加し、authority transition line を symbolic authority handoff epoch ref まで narrow に actualize した。next promoted line は minimal-authority-handoff-epoch-ref-ready witness-aware-handoff-family comparison に切り替わり、Phase 5 は `126...234` まで current package close と読める状態になった。
- 2026-04-11 11:57 JST — `specs/examples/233...234` package の validation と local review fallback closeout を完了し、snapshot を `126...234` package close に揃えた。next promoted line は minimal-authority-handoff-epoch-ref-ready witness-aware-handoff-family comparison のままである。
- 2026-04-11 12:01 JST — `specs/examples/235...236` package の validation と local review fallback closeout を完了し、snapshot を `126...236` package close に揃えた。next promoted line は minimal-witness-aware-handoff-family-ready handoff-witness-row-detail comparison のままである。
- 2026-04-11 11:57 JST — Phase 5 theorem-line later reopen として `specs/examples/235` と `236` を追加し、authority transition line を symbolic witness-aware handoff family まで narrow に actualize した。next promoted line は minimal-witness-aware-handoff-family-ready handoff-witness-row-detail comparison に切り替わり、Phase 5 は `126...236` まで current package close と読める状態になった。
- 2026-04-11 12:02 JST — Phase 5 theorem-line later reopen として `specs/examples/237` と `238` を追加し、authority transition line を actual handoff witness row detail まで narrow に actualize した。next promoted line は minimal-handoff-witness-row-detail-ready replay-attachment-ref comparison に切り替わり、Phase 5 は `126...238` まで current package close と読める状態になった。
- 2026-04-11 12:23 JST — Phase 5 theorem-line later reopen として `specs/examples/239` と `240` を追加し、authority transition line を symbolic replay attachment ref まで narrow に actualize した。next promoted line は minimal-replay-attachment-ref-ready handoff-payload-ref comparison に切り替わり、Phase 5 は `126...240` まで current package close と読める状態になった。
- 2026-04-11 12:33 JST — Phase 5 theorem-line later reopen として `specs/examples/241` と `242` を追加し、authority transition line を symbolic handoff payload ref まで narrow に actualize した。next promoted line は minimal-handoff-payload-ref-ready handoff-carrier-detail comparison に切り替わり、Phase 5 は `126...242` まで current package close と読める状態になった。
- 2026-04-11 12:44 JST — Phase 5 theorem-line later reopen として `specs/examples/243` と `244` を追加し、authority transition line を minimal handoff carrier detail まで narrow に actualize した。next promoted line は minimal-handoff-carrier-detail-ready handoff-transport-family comparison に切り替わり、Phase 5 は `126...244` まで current package close と読める状態になった。
- 2026-04-11 12:58 JST — Phase 5 theorem-line later reopen として `specs/examples/245` と `246` を追加し、authority transition line を symbolic handoff transport family まで narrow に actualize した。next promoted line は minimal-handoff-transport-family-ready handoff-transport-carrier-detail comparison に切り替わり、Phase 5 は `126...246` まで current package close と読める状態になった。
- 2026-04-11 13:28 JST — Phase 5 theorem-line later reopen として `specs/examples/247`〜`252` を追加し、authority transition line を minimal handoff transport carrier detail、symbolic handoff transport payload、minimal handoff transport receipt row まで narrow に actualize した。next promoted line は minimal-handoff-transport-receipt-ready handoff-transport-channel-body comparison に切り替わり、Phase 5 は `126...252` まで current package close と読める状態になった。
- 2026-04-11 13:36 JST — Phase 5 theorem-line later reopen として `specs/examples/253` と `254` を追加し、authority transition line を minimal handoff transport channel body まで narrow に actualize した。next promoted line は minimal-handoff-transport-channel-body-ready low-level-memory-order-family comparison に切り替わり、Phase 5 は `126...254` まで current package close と読める状態になった。
- 2026-04-11 11:17 JST — Phase 5 authority-serial transition package の docs validation と local review fallback closeout を完了し、snapshot を `126...226` package close に揃えた。next promoted line は minimal-authority-transition-stage-family-ready authority-transition-stage-sequence-shape comparison のままである。
- 2026-04-11 15:29 JST — Phase 5 checker-side current promoted line として `specs/examples/265...266` を追加し、actual checker payload family の次段を `payload_family_ref + row_family_kind` minimal checker payload row family まで narrow に actualize した。next promoted line は minimal-checker-payload-row-family-ready checker-payload-row-detail comparison に切り替わり、Phase 5 は `126...266` まで current package close と読める状態になった。
- 2026-04-11 16:25 JST — Phase 5 checker-side current promoted line として `specs/examples/269...270` を追加し、checker payload row body を `row_body` variant-local slot bundle まで narrow に actualize した。next promoted line は minimal-checker-payload-row-body-ready checker-payload-supported-kind-summary comparison に切り替わり、Phase 5 は `126...270` まで current package close と読める状態になった。
- 2026-04-11 16:36 JST — Phase 5 checker-side current promoted line として `specs/examples/271...272` を追加し、checker payload supported kind summary を `payload_row_family_ref + supported_kind_scope + supported_kind_refs` minimal bundle まで narrow に actualize した。next promoted line は minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison に切り替わり、Phase 5 は `126...272` まで current package close と読める状態になった。
- 2026-04-11 17:07 JST — current snapshot audit を行い、Phase 5 の current promoted line は引き続き `minimal-checker-payload-supported-kind-summary-ready public-checker-payload-schema comparison` と読め、shared-space finalization と final parser / public checker boundary は later blocker / open question として mirror 文書側で明示するのが自然だと確認した。
- 2026-04-11 17:46 JST — Phase 5 checker-side current promoted line として `specs/examples/273...274` を追加し、public checker payload schema を `actual_checker_payload_family_ref + checker_payload_row_family_ref + checker_payload_row_detail_ref + checker_payload_row_body_ref + checker_payload_supported_kind_summary_ref` minimal wrapper まで narrow に actualize した。next promoted line は minimal-public-checker-payload-schema-ready public-checker-api comparison に切り替わり、Phase 5 は `126...274` まで current package close と読める状態になった。
- 2026-04-11 18:25 JST — Phase 5 checker-side current promoted line として `specs/examples/275...276` を追加し、public checker API を `checker_subject + public_checker_payload_schema_ref` docs-only read relation まで narrow に actualize した。next promoted line は minimal-public-checker-api-ready public-checker-entry-criteria comparison に切り替わり、Phase 5 は `126...276` まで current package close と読める状態になった。
- 2026-04-11 18:40 JST — Phase 5 checker-side current promoted line として `specs/examples/277...278` を追加し、public checker entry criteria を docs-only minimal API fixed + source-backed family-facade command-surface pressure に留め、shared output contract / parser boundary は still later に残す current first choice を固定した。next promoted line は minimal-public-checker-entry-criteria-ready public-checker-command-surface comparison に切り替わり、Phase 5 は `126...278` まで current package close と読める状態になった。
- 2026-04-11 19:02 JST — Phase 5 checker-side current promoted line として `specs/examples/279...280` を追加し、public checker command surface を `command_surface_kind + family_facade_command_refs + public_checker_api_ref` minimal family-facade bundle に留め、detached loop `smoke-*` wrapper / shared generic entry / shared output contract / parser boundary は still later に残す current first choice を固定した。next promoted line は minimal-public-checker-command-surface-ready shared-output-contract comparison に切り替わり、Phase 5 は `126...280` まで current package close と読める状態になった。
- 2026-04-11 19:31 JST — Phase 5 checker-side current promoted line として `specs/examples/281...284` を追加し、shared output contract を `output_contract_kind + checker_cluster_name + checker_status + public_checker_payload_schema_ref` minimal summary bundle、public checker boundary を `boundary_kind + public_checker_command_surface_ref + shared_output_contract_ref` docs-only parser-front bundle に留めた。next promoted line は `minimal-public-checker-boundary-ready verifier-handoff-surface comparison` に切り替わり、Phase 5 は `126...284` まで current package close と読める状態になった。
