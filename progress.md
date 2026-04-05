# progress

最終更新: 2026-04-06（stage 1 parser spike first tranche actualization まで反映）

## 位置づけ

- この文書は repo の**簡潔な進捗スナップショット**である。
- 規範判断の正本は `specs/`、長期参照用の repository memory は `plan/` である。
- ここに書く進捗率と残ステップは **rough estimate** であり、問題が見つかれば巻き戻る。
- 進捗率は、可能な限り次の 3 軸で書く。
  - **論理仕様**: semantics / invariants / formal boundary の整備度
  - **ユーザ向け仕様**: companion notation / representative examples / human-facing guidance の整備度
  - **実装 / 運用**: parser-free PoC / helper / validation loop / 実務フローの整備度

## いまどこまで来ているか

- Mir current L2 の核心意味論は、current task を回すにはかなり安定している。
- parser-free PoC 基盤は、fixture / interpreter / host harness / bundle / batch / selection / profile / catalog まで揃っている。
- detached validation loop は、docs-only judgment、bundle-first emitter、aggregate emitter、bundle diff helper、aggregate diff helper、tiny loop wrapper、representative contrast smoke convenience、fixture authoring template、fixture scaffold helper まで入った。
- runtime regression catalog に `e9-monotone-degradation-success` を追加し、middle explicit failure のあとでも later same-lineage write-capable option へ monotone degradation して success しうることを machine-check で固定した。
- runtime regression catalog に `e10-perform-on-ensure-failure` を追加し、direct `PerformOn` の request-local `ensure` unsatisfied が `Reject` ではなく `explicit_failure` であり、success-side carrier preview が commit されないことを machine-check で固定した。
- runtime regression catalog に `e11-perform-via-ensure-then-success` を追加し、via-chain の earlier option が request-local `ensure` で失敗しても later same-lineage option へ継続して success しうることを machine-check で固定した。
- runtime regression catalog に `e21-try-atomic-cut-frontier` を追加し、`TryFallback` body 内の `AtomicCut` が active rollback frame の frontier を post-cut snapshot へ更新し、pre-cut mutation を残したまま fallback へ進むことを machine-check と detached loop smoke で固定した。
- runtime regression catalog に `e22-try-atomic-cut-place-mismatch` を追加し、nested place 内の `AtomicCut` は event を残しても `place_anchor` mismatch では frontier を進めず、rollback が try-entry snapshot へ戻ることを machine-check と detached loop smoke で固定した。
- `TryFallback` / `AtomicCut` の checker boundary では、structural floor だけを first checker cut 候補に残し、`place_anchor == current_place` gate と whole-store restore scope は runtime / proof boundary に残す current judgment を docs / plan / representative runtime evidence で揃えた。
- その次段 comparison として、`TryFallback` / `AtomicCut` structural floor は current phase では existing reason-row family helper の fourth spike に actualize せず、docs/runtime representative に留める current judgment を固定した。
- dedicated `TryFallback` / `AtomicCut` AST structural helper については、entry criteria、malformed source placement、helper-local compare contract、field 名、verdict carrier、shared carrier threshold、wrapper family、generic/public checker boundary、first tranche cut、two-fixture pair、slot selectionまでの docs-only chain を前提に、helper-local first tranche を actualize した。
- current next phase では、dedicated `TryFallback` / `AtomicCut` AST structural helper first tranche の wording / row family を exact working set に留め、`TryFallback` / `missing_fallback_body`、`AtomicCut` / `disallowed_fallback_placement` の rename や alias / synonym 導入は shared carrier threshold の再比較と later generic/public comparison まで deferred にする。
- current recheck では、saved artifact compare need の narrow version は helper-local checker が saved static gate artifact path を直接 compare することで満たせるため、shared detached carrier threshold はまだ未充足と読む。
- current recheck では、generic structural checker family / public checker API comparison に進む concrete pressure もまだ不足しているため、current try/rollback checker line は source-backed trigger が出るまで一旦 pause とみなす。
- current parser boundary 主線では、actual stage 1 parser spike の private helper 配置 / compare surface / input surface / internal carrier / helper naming に続き、`crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` で `e4` / `e7` の lowered fixture-subset compare と guard-slot retention smoke を actual code anchor として通す first tranche まで actualize できた。
- current first tranche の actualized scope は
  - `expected_static.checked_try_rollback_structural_verdict`
  - `expected_static.checked_try_rollback_structural_findings`
  - `scripts/current_l2_try_rollback_structural_checker.py`
  - `scripts/current_l2_detached_loop.py smoke-try-rollback-structural-checker`
  - `e23-malformed-try-fallback-missing-fallback-body`
  - `e24-malformed-atomic-cut-fallback-placement`
  である。
- current first tranche の finding family は `TryFallback` / `missing_fallback_body` と `AtomicCut` / `disallowed_fallback_placement` の 2 件に留め、shared detached carrier、generic structural checker family、public checker API にはまだ上げていない。
- fixture authoring bottleneck のうち boilerplate 部分は、`target/current-l2-fixture-scaffolds/` 下に required carrier と empty sidecar 骨格だけを出す non-production helper で narrow に補助できる状態になった。
- static-only malformed / underdeclared fixture に対しては、`scripts/current_l2_detached_loop.py suggest-checked-reasons` から actual static gate `checker_core.reasons` を見て `expected_static.checked_reasons` 候補を display-only で確認できるようになった。helper は fixture JSON を自動更新しない。
- helper-local / reference-only `detached_noncore.reason_codes` についても、`scripts/current_l2_detached_loop.py suggest-reason-codes` から future typed carrier 候補 row を display-only で確認できるようになった。これは current fixture schema の field ではなく、reference-only assist に留まり、unsupported fixture-side typed field を見つけたら fail-closed に止まる。
- static-only fixture corpus に対しては、`scripts/current_l2_detached_loop.py scan-reason-code-readiness` から `checked_reasons` adoption、`reason_codes` suggestion availability、stable coexistence anchor 数、follow-up 必要 fixture を batch で display-only 要約できるようになった。current corpus では stable cluster 8 kind を覆う 9 fixture が suggestion / `checked_reason_codes` / `checked_reasons` の 3 者で揃い、duplicate cluster 2 件だけが no-suggestion に留まる。
- future typed static reason carrier の first actualization family は lineage edge pair family、次点は declared target edge pair family と source-backed に絞れた。
- first-class typed carrier の最小 placement は detached-side mirror 昇格ではなく fixture-side additive optional `expected_static.checked_reason_codes` であり、current stable cluster inventory 8 kind は actualize 済みである。
- current readiness scan では、static-only fixture corpus に対して `checked_reason_codes` adoption 9 件 / `reason_codes` suggestion 9 件 / duplicate cluster 2 件 absent が揃っている。
- `checked_reasons` と `checked_reason_codes` の関係は current では additive coexistence 維持であり、stable cluster 8 kind を覆う 9 fixture の 3-way alignment が zero follow-up でも、wording anchor を失わないため shrink はまだ行わない。
- same-lineage floor `4`、capability floor `2`、missing-option structure floor `3` という checker cluster coverage が current static-only corpus から source-backed に見えるようになり、mainline を first checker cut 側へ戻す根拠が揃い始めた。
- static-only malformed fixture として `e20-malformed-late-capability-strengthening` を actual corpus に追加し、capability strengthening floor を late-edge variant まで広げた。
- capability strengthening floor については、`e13` / `e20` を fixture-side `checked_reason_codes` と detached static gate artifact `reason_codes` で narrow compare する helper-local third checker spike を追加し、family 単位の smoke を取れるようになった。
- same-lineage / missing-option / capability の 3 checker spike で重複していた compare contract は、family facade script と detached loop command 名を保ったまま `scripts/current_l2_family_checker_support.py` へ薄く寄せた。
- shared support helper 導入後も、generic checker-side shared family compare entry はまだ切らず、family facade script を current command surface として維持する判断を docs-only で固定した。
- first checker cut に戻す current judgment は、carrier migration の追加 refinement より checker-boundary 整理を優先してよいという sequencing judgment として docs / plan / report chain へ固定された。
- same-lineage static evidence floor については、fixture-side `checked_reason_codes` と static gate artifact `reason_codes` を narrow compare する helper-local first checker spike を追加し、`e4` / `e12` の smoke まで通した。
- missing-option structure floor については、same-lineage の次段として fixture-side `checked_reason_codes` と static gate artifact `reason_codes` を narrow compare する helper-local second checker spike を追加し、`e16` / `e17` の smoke まで通した。
- static-only malformed fixture の stable cluster として、`e16-malformed-missing-chain-head-option` / `e17-malformed-missing-predecessor-option` / `e18-malformed-missing-successor-option` を actual corpus に追加し、`checked_reasons` と detached static gate `reason_codes` の両方へ narrow adoption した。
- static-only malformed fixture の stable cluster として、`e19-malformed-target-mismatch` も actual corpus に追加し、current docs で stable と列挙している same-lineage static reason clusters は一通り corpus / `checked_reasons` / detached `reason_codes` mirror に揃った。
- static-only scaffold helper は fixture JSON を変えないまま、authoring 後段で `suggest-checked-reasons` を使えることを stderr reminder として案内できるようになった。reminder command の fixture path は shell-safe quoting で copyable に保っている。
- detached validation loop には `smoke-fixture` convenience が入り、1 fixture の bundle export、optional reference compare、single-fixture aggregate smoke を 1 command で回せるようになった。
- aggregate emitter 内 private transform を `examples/support/current_l2_detached_aggregate_support.rs` へ切り出し、`BatchRunSummary -> detached aggregate artifact` の repo 内 callable boundary を non-production のまま shared helper 化した。
- bundle emitter 内 private transform を `examples/support/current_l2_detached_bundle_support.rs` へ切り出し、`FixtureBundle + BundleRunReport -> detached bundle artifact` の repo 内 callable boundary を non-production のまま shared helper 化した。
- static gate verdict / reasons を `examples/support/current_l2_static_gate_support.rs` と `current_l2_emit_static_gate.rs` から detached artifact として保存できるようにし、static-only / malformed / underdeclared fixture を `smoke-static-gate` で compare できるようにした。
- `expected_static.reasons` を `run_bundle()` の actual machine-check に上げる試行を行い、current fixture corpus では explanatory note と machine-check 候補が混在しているため、そのまま昇格できないことを確認した。そのうえで additive optional `expected_static.checked_reasons` を導入し、field があるときだけ actual static gate reasons を fail-closed compare できる narrow carrier を入れた。
- `checked_reasons` の次段として、typed reason code へ進めてよい stable cluster と parametric shape の entry criteria を docs-only で棚卸しした。immediate 全面 code 化は避け、cluster inventory を先に固める方針になった。
- detached static gate artifact 側には helper-local / reference-only な `detached_noncore.reason_codes` mirror を actualize し、stable cluster だけを best-effort に写す narrow helper cut を code / docs / diff helper で揃えた。これは `checker_core` や `checked_reasons` を置き換える typed carrier ではない。
- `e4-malformed-lineage`、`e5-underdeclared-lineage`、`e12-underdeclared-target-missing`、`e13-malformed-capability-strengthening` では `checked_reasons` を actual corpus に narrow adoption し、explanatory `reasons` を保ったまま actual static gate wording を bundle machine-check bridge として保持する cut を stable cluster 側へ広げた。
- `e14-malformed-duplicate-option-declaration` と `e15-malformed-duplicate-chain-declaration` を static-only corpus に追加し、duplicate declaration cluster も detached loop に流せることを確認した。ただし current helper cut では duplicate cluster を `checked_reasons` / detached `reason_codes` の stable cluster に上げず、actual wording は `checker_core.reasons` と focused test / smoke で見る方針を維持している。
- current valid fixture の代表例（`e1` / `e3` / `e6` / `e10`）では actual static gate `reasons` が空であることを確認し、`checked_reasons = []` を広く足す task は current L2 では優先しないと判断した。
- parser boundary については、final grammar を先に凍らせずに first parser cut に入れてよい semantic cluster を先に inventory 化し、actual parser spike を切るなら chain / declaration structural floor → `try` / rollback structural floor → request / admissibility cluster の checker-led staged spike が current baseline であり、その stage 1 では declaration-side guard slot を predicate parser の入口ではなく opaque attached slot に留め、actual handoff では parser-side opaque slot carrier から current fixture schema の `OptionDecl.lease` へ thin lowering bridge を使う。さらに naming / bridge API の next narrow cut として、carrier 名は `decl_guard_slot` を第一候補にし、bridge は slot-only helper ではなく option-level structural transfer として読み、stage 1 actual parser spike の最小 smoke family は `e4` + `e7` の two-fixture pair に留めて `e3` を later-stage contrast に残し、actual spike 自体は `crates/mir-ast/tests/support/` 配置の private helper + lowered fixture-subset compare から始める judgement まで source-backed に見えた。
- static analysis / theorem prover 境界については、first checker cut に入れてよい local / structural judgment と external verifier 側へ残す global property の floor を docs-only で切り始めた。
- いま重いのは semantics そのものより、**fixture authoring / elaboration** と **detached validation loop の実運用面**である。
- richer host interface、final parser grammar、static analysis / type / theorem prover、multi-request scheduler はまだ後段である。
- 実装上の非本質だが忘れてはいけない制約として、**OS / hardware 非依存性** と **step 実行 / graph 可視化へ伸ばせる observability 境界** を、早期固定しすぎずに守る必要がある。

## バリデーションループ前の残課題

### 1. detached validation loop の actual narrow cut

- bundle export / compare / aggregate summary の最小手順は揃った
- storage / path policy の current candidate を使った smoke を数回増やし、artifact naming / discovery を実地で安定化させる段階に入った
  - aggregate 側の typed count field は non-production aggregate artifact と shared transform helper にまで進んだが、actual public API cut はまだ未決

### 2. fixture authoring / elaboration bottleneck

- 新しい fixture を 1 本足す手順は template 化され、scaffold helper で boilerplate だけ先に起こせるようになった
- `e9` / `e10` / `e11` により、success-side 補完、direct-target ensure failure、via-chain ensure continuation を machine-check で増やせた
- ただし authoring / sidecar / expected trace-audit / profile 影響確認は、まだ人手依存が大きい
- 「1 本足して detached artifact を保存し、aggregate summary も取り、既存 artifact と比べる」運用をあと数回回して固める必要がある

### 3. parser 前の boundary inventory

- current companion notation はあるが final parser grammar は未決
- parser 導入前に、何を syntax で固定し、何を current L2 companion notation に残すかの棚卸しがまだ必要

### 4. 後段に残すもの

- richer host interface の typed carrier 化
- static analysis / theorem prover 境界
- multi-request scheduler
- `Approximate` / `Compensate`
- portability / observability hook の final contract

## 章別 rough progress

| 章 / 層 | 論理仕様 | ユーザ向け仕様 | 実装 / 運用 | 着手可否 | 補足 |
|---|---:|---:|---:|---|---|
| 基礎文書・decision level・invariants | 92% | 86% | 70% | 着手可能 | repo の基礎境界はかなり揃っている |
| Mir current L2 core semantics | 85% | 77% | 75% | 着手可能 | current task を回すには十分安定しており、`TryFallback` body 内 `AtomicCut` frontier と nested-place mismatch の runtime representative に加え、empty `fallback_body` と `fallback_body` 内 `AtomicCut` の structural malformed first tranche が helper-local に actualize 済みである。ただし final formalization はまだ先 |
| fallback / notation / representative examples | 85% | 81% | 65% | 着手可能 | drift 抑制は進み、`atomic_cut` rollback frontier の try-body representative と nested-place mismatch contrast まで actualize できたが final parser grammar は未決 |
| parser-free PoC execution stack | 89% | 84% | 98% | 着手可能 | interpreter / host / bundle / batch / selection / profile に加え、bundle / aggregate / static gate transform の repo 内 callable boundary が shared helper として入り、static gate reasons は detached helper compare、optional checked carrier、helper-local reason-code mirror、display-only authoring assist の境界まで整理された。さらに `TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche が helper-local compare family として static gate loop に actualize 済みである |
| detached export / validation loop | 98% | 96% | 99% | 着手可能 | bundle / aggregate / static gate emitter、bundle / aggregate / static gate compare helper、wrapper、storage candidate、scaffold helper、fixture smoke helper に加え、bundle / aggregate / static gate transform helper の shared support cut、static gate detached non-core reason-code mirror を reference-only compare で扱う cut、checked_reasons suggestion と future typed carrier 候補 row を display-only で返す wrapper convenience、static-only corpus 横断の readiness scan、stable coexistence anchor / follow-up scan、fixture-side `checked_reason_codes` adoption の stable tranche 表示、`e21` / `e22` representative pair を 1 command で回す `smoke-try-rollback-locality` convenience、`e23` / `e24` の dedicated structural smoke を 1 command で回す `smoke-try-rollback-structural-checker` まで揃った |
| fixture authoring / elaboration 実務 | 97% | 98% | 99% | 着手可能 | template / scaffold helper は揃い、runtime fixture と static-only fixture の detached compare path に加え、stable malformed / underdeclared cluster `e4` / `e5` / `e12` / `e13` / `e16` / `e17` / `e18` / `e19` / `e20` を `checked_reasons` と `checked_reason_codes` の current stable tranche まで actualize した。runtime 側でも `e21-try-atomic-cut-frontier` と `e22-try-atomic-cut-place-mismatch` を actual corpus へ追加し、bundle / directory / detached smoke まで通した。さらに dedicated try/rollback first tranche の `e23` / `e24` と helper-local expected field を actualize 済みである。duplicate declaration cluster `e14` / `e15` は display-only actual wording に留め、stable cluster 8 kind を覆う 9 fixture と duplicate 2 件の split が actual corpus / readiness scan / detached mirror で揃い、coexistence follow-up は現状 `none` である |
| parser / syntax finalization 準備 | 69% | 73% | 36% | 着手可能 | first parser cut inventory と checker-led staged orderに加え、stage 1 では declaration-side guard slot を opaque attached slot に留め、thin lowering bridge で current fixture schema の `OptionDecl.lease` へ接続する docs-only cutまで揃っている。さらに actual first tranche として `crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs`、`crates/mir-ast/tests/current_l2_stage1_parser_spike.rs`、local `serde_json` dev-dependency を用いた `e4` / `e7` lowered fixture-subset compare と guard-slot retention smoke が actualize 済みである。ただし final grammar、exact lexical choice、predicate completion、malformed-source parser smoke、actual parser API は未決 |
| richer host interface / coverage typed 化 | 24% | 22% | 16% | 後段依存 | comparison までは進んだが implementation cut は後段 |
| aggregate export の typed actualization | 58% | 50% | 61% | 着手可能 | non-production aggregate emitter と aggregate compare helperに加え、aggregate transform の actual narrow cut は shared support helper まで進んだが public API と final compare 契約は未決 |
| static analysis / type / theorem prover workstream | 85% | 78% | 76% | 着手可能 | first checker cut の local / structural floor に加え、static gate reasons をそのまま harness machine-check へ昇格できないことを確認し、additive optional `checked_reasons` を最小 dedicated carrier として実装した。typed code は stable cluster inventory と parametric shape 条件まで整理済みで、detached-side `reason_codes` は helper-local / reference-only mirror に留めつつ、display-only assist と readiness scan で corpus 横断の actual split を観察できる。さらに current stable cluster inventory 8 kind を覆う 9 fixture を optional `checked_reason_codes` に actualize し、coexistence scan を zero follow-up まで揃え、same-lineage / capability / missing-option checker cluster coverage を source-backed に確認し、same-lineage first checker spike、missing-option second checker spike、capability third checker spike、shared family checker support helper を固めた。そのうえで generic checker-side shared family entry は current phase では切らず、family facade 維持で止め、`TryFallback` / `AtomicCut` structural floor は dedicated helper-local first tranche として `e23` / `e24`、`checked_try_rollback_structural_*`、`smoke-try-rollback-structural-checker` まで actualize した。shared carrier、generic family 昇格、public checker cut と shrink policy はまだ OPEN |
| portability / observability / debug hook 設計 | 20% | 14% | 10% | 後段依存 | HW 非依存と step / graph 可視化余地は要件化したが contract はまだない |
| Mirrorea fabric | 18% | 12% | 8% | 要仕様確認 | 境界整理はあるが current mainline 実装はまだ先 |
| Typed-Effect Wiring Platform | 12% | 8% | 6% | 要仕様確認 | 位置づけはあるが concrete architecture は後段 |
| PrismCascade / 上位空間 | 12% | 8% | 6% | 要仕様確認 | 分離方針はあるが mainline の進捗対象ではない |

## 着手可否の読み方

- `着手可能`
  - 非本質部分を先に進めても手戻りが比較的小さい。
  - current repo の mainline として、agent が narrow task を自走しやすい。
- `要仕様確認`
  - user 側の目的、保証範囲、非機能要件、上位層の具体像が不足しており、勝手に詰めると手戻りが大きい。
  - たとえば上位空間の UX や社会システム仕様はここに入る。
- `後段依存`
  - 先行 layer / 先行 decision が固まるまで、本格着手を急がない方がよい。
  - comparison、inventory、entry criteria 整理までは進めてよいが、mainline 実装にはまだ早い。

## いまから validation loop 入口まで何手か

- **detached validation loop の入口まで**: **到達済み**
- 主な中身:
  1. fixture authoring を detached artifact + aggregate summary loop 前提でさらに 1〜2 回実地に回す
  2. aggregate export の actual narrow API cut をもう一段整理する
  3. storage / naming / compare discovery の current candidate を smoke evidence で固める

## ある程度自律的な Mir 構築ループまで何手か

- **ある程度自律的に「追加し、回し、比較し、次へ進む」状態まで**: あと **1〜3** task 程度
- 想定する中身:
  1. detached validation loop の入口を安定化する
  2. fixture authoring / elaboration の反復コストをもう一段下げる
  3. parser spike の stage 2 比較か malformed-source smoke の narrow cut を進める
  4. richer host interface を後段に送ったまま、今必要な host coverage の cut だけ固める
  5. current L2 semantics の追加 regression を数本回して、巻き戻りが必要か確認する
  6. portability / observability の boundary を壊さない helper cut を確認する

## いまの判断

- **今の mainline は正しい**。Mir current L2 と parser-free PoC を主線に据える方針は維持でよい。
- 直近の主ボトルネックは、意味論の大変更ではなく **validation loop の運用面** と **fixture authoring** である。
- final parser grammar や richer host interface を急ぐより、まず「artifact を保存し、比較し、fixture を増やしてまた回す」ループを安定化させるべき段階にある。
- portability / observability は今すぐ runtime core に組み込む話ではないが、helper cut と将来 API の形を歪めないよう early reminder として管理する。

## 作業ログ（簡潔）

- append-only の簡潔ログであり、task consolidation の都合で必ずしも時刻順に並べ替えない。厳密な trail は report と git history を参照。

- 2026-04-05 15:14 JST — detached validation loop の aggregate emitter・wrapper・diff 周辺を検証し、bundle artifact と aggregate summary を保存して比較する current non-production loop が通った。次は fixture authoring の実地反復を増やす段階。
- 2026-04-05 16:04 JST — `e9-monotone-degradation-success` fixture を追加し、admit miss・middle explicit failure・later success を同じ runtime chain で固定する regression を検証した。green と detached smoke が通ったので、次は aggregate compare helper を詰める段階。
- 2026-04-05 16:13 JST — aggregate artifact の `summary_core` だけを比較する helper と `compare-aggregates` wrapper を追加し、run label から aggregate path を導出して partial histogram を比較できるようにした。次は fixture authoring の実地反復をもう 1 段増やすか、aggregate actual API cut を narrow に詰める段階。
- 2026-04-05 16:47 JST — `e10-perform-on-ensure-failure` fixture を追加し、direct `PerformOn` の request-local `ensure` unsatisfied が `Reject` ではなく `explicit_failure` であることを machine-check と detached smoke で固定した。review fallback 記録まで残したので、next は `PerformVia` 側 ensure variant か aggregate actual API cut 比較の段階。
- 2026-04-05 17:01 JST — `e11-perform-via-ensure-then-success` fixture を追加し、via-chain の earlier option が request-local `ensure` で失敗しても later same-lineage option へ継続して success しうることと、earlier tentative commit が破棄され later success だけが place store に反映されることを machine-check で固定した。次は aggregate actual API cut か fixture authoring / elaboration の narrow helper を詰める段階。
- 2026-04-05 17:02 JST — fixture authoring の boilerplate だけを出す scaffold helper を追加し、runtime/static-only skeleton と empty sidecar を `target/current-l2-fixture-scaffolds/` 下へ安全に作れるようにした。validation loop の入口は到達済みで、次は actual API cut か新 fixture の反復段階。
- 2026-04-05 17:15 JST — `smoke-fixture` helper を detached loop wrapper に追加し、1 fixture の bundle export、optional reference compare、single-fixture aggregate smoke を 1 command で回せるようにした。次は aggregate actual API cut か parser 前 inventory の narrow task を詰める段階。
- 2026-04-05 17:15 JST — first parser cut inventory を追加し、`place` / `try-fallback` / `perform on|via` / `require` / `ensure` / option declaration core / `admit` / explicit edge-row family を parser 候補 cluster として整理した。次は aggregate actual API cut を operational に寄せるか、この inventory を checker / proof entry criteria に接続する段階。
- 2026-04-05 17:30 JST — first checker cut entry criteria を追加し、same-lineage static evidence floor、malformed / underdeclared rejection、minimal capability strengthening prohibition、clause attachment、minimal predicate fragment、`try` / rollback locality の structural floor を local checker 側へ残し、global invariant proof は external verifier 側へ送る cut を docs-only で固定し始めた。次は aggregate actual API cut を進めるか、この gate を actual checker spike へ接続する段階。
- 2026-04-05 17:55 JST — aggregate emitter 内 private transform を `examples/support/current_l2_detached_aggregate_support.rs` へ切り出し、`BatchRunSummary -> detached aggregate artifact` の repo 内 callable boundary を non-production のまま shared helper 化した。full `cargo test -p mir-semantics` と detached loop smoke が通ったので、次は actual public exporter API をまだ凍らせずに、この cut を loop smoke と docs mirror で安定化させる段階。
- 2026-04-05 18:05 JST — aggregate transform helper task の report / review / traceability を閉じ、shared helper cut が public API へ越境していないことを reviewer と fresh verification で確認した。次は aggregate compare contract と storage/path policy の current candidate を smoke evidence でさらに固める段階。
- 2026-04-05 18:25 JST — static gate verdict / reasons を detached validation loop に接続する helper と `smoke-static-gate` wrapper を追加し、`e4-malformed-lineage` と `e5-underdeclared-lineage` の compare smoke が通った。次は aggregate compare contract / storage candidate をさらに固めるか、first checker cut を actual checker spike に一段寄せる段階。
- 2026-04-05 18:37 JST — static gate artifact loop の review fix を反映し、multi-reason fixture で `checker_core.reasons` の順序を deterministic に固定した。full `cargo test -p mir-semantics`、Python helper tests、`smoke-static-gate`、既存 `smoke-fixture`、docs validation が通ったので、次は `expected_static.reasons` を actual machine-check に寄せる段階。
- 2026-04-05 18:45 JST — `expected_static.reasons` を `run_bundle()` の actual machine-check に上げる試行を行い、invalid fixture だけでなく valid fixture 群とも衝突することを failing test と full cargo test で確認した。current fixture corpus では explanatory note と machine-check 候補が混在しているため、この field はそのまま core に昇格させず、future checker API では dedicated carrier を別立てにする方針を docs / plan に反映する段階。
- 2026-04-05 19:03 JST — `expected_static.reasons` は current fixture corpus で dual-use carrier だと確定し、current harness core へは昇格させない判断を report / review / traceability / roadmap に反映した。full `cargo test -p mir-semantics` と `smoke-static-gate` は green を維持し、次は dedicated static reason carrier の最小 cut を比較する段階。
- 2026-04-05 19:32 JST — detached static gate artifact に helper-local / reference-only `detached_noncore.reason_codes` mirror を actualize し、static gate diff helper でも reference-only difference として扱う cut を code / docs / plan / progress へ揃えた。full `cargo test -p mir-semantics`、Python diff helper tests、`smoke-static-gate`、docs validation が通ったので、次は `checked_reasons` bridge を広げるか typed carrier 昇格比較へ進める段階。
- 2026-04-05 19:35 JST — additive optional `expected_static.checked_reasons` を導入し、field があるときだけ `run_bundle()` が actual static gate reasons を fail-closed compare する narrow carrier を追加した。targeted RED/GREEN が通ったので、次は typed reason code へ進むかこの carrier を fixture authoring に広げるかを比較する段階。
- 2026-04-05 19:41 JST — `e4` / `e5` に `checked_reasons` を narrow adoption し、static-only malformed / underdeclared fixture で explanatory `reasons` を残したまま actual static gate wording を bundle machine-check bridge に接続した。full `cargo test -p mir-semantics` と docs validation は green を維持し、次は valid fixture まで adoption を広げる価値があるかの比較段階。
- 2026-04-05 19:52 JST — `checked_reasons` から typed reason code へ進める条件を docs-only で整理し、stable cluster と parametric shape の entry criteria を切り出した。next は static gate reason code を detached artifact 側へ mirror するか、fixture authoring で checked carrier 採用を広げるかの比較段階。
- 2026-04-05 20:00 JST — valid fixture に `checked_reasons = []` を広く足す task は current L2 では優先しないと判断し、`specs/examples/33`、`plan/11`、`plan/15`、`progress.md` に non-adoption policy を明記した。次は scaffold / authoring helper が checked bridge 候補をどこまで assist してよいかを比較する段階。
- 2026-04-05 20:24 JST — `checked_reasons` の authoring assist は auto-fill ではなく display-only を current cut とし、static gate artifact の actual `checker_core.reasons` を読んで suggestion を返す helper と `suggest-checked-reasons` wrapper を追加した。これで malformed / underdeclared fixture の narrow adoption を hand-edit + review 前提で補助できるようになった。
- 2026-04-05 23:10 JST — display-only `checked_reasons` assist まわりの spec / report / traceability / progress を整合させ、targeted tests・real smoke・full cargo test・docs validation が揃った。次は assist を数回実地で回し、typed reason code への first-class 昇格を急がず authoring 実務の摩擦点をさらに絞る段階。
- 2026-04-05 23:34 JST — static-only scaffold helper から `suggest-checked-reasons` への stderr follow-up reminder を追加し、fixture JSON を変えずに authoring 次段の narrow helper を案内する cut を docs / tests / progress に揃えた。
- 2026-04-05 20:34 JST — `checked_reasons` assist reconciliation を閉じ、existing spec / helper / report / traceability / progress の factual trail を揃えたうえで、static-only scaffold reminder の shell-safe quoting を test-first で固定した。次は current detached validation loop を実地の fixture authoring へさらに接続する段階。
- 2026-04-05 20:42 JST — bundle-first detached artifact の private transform を `examples/support/current_l2_detached_bundle_support.rs` へ切り出し、runtime / static-only の両 fixture で repo 内 callable boundary と emitter smoke を揃えた。次は fixture authoring 実地反復か、aggregate / storage policy の actual narrow API cut をさらに詰める段階。
- 2026-04-05 20:55 JST — detached bundle transform helper task を reviewer 付きで閉じ、generated bytecode の混入を除去したうえで `e3` の formal `non_admissible_metadata` 保持を direct test に追加した。full `cargo test -p mir-semantics`、bundle smoke、docs validation、`git diff --check` が通ったので、次は static-only / malformed fixture authoring の実地反復へ進める状態。
- 2026-04-05 21:18 JST — static-only fixture authoring tranche として `e12-underdeclared-target-missing` / `e13-malformed-capability-strengthening` を actual corpus に追加し、`checked_reasons` を stable cluster へ広げた。scaffold helper、`suggest-checked-reasons`、static gate smoke、bundle/aggregate smoke、full `cargo test -p mir-semantics` を通したので、次は duplicate declaration 系を actual corpus に入れる前に fixture 増加の運用コストを比較する段階。
- 2026-04-05 23:58 JST — duplicate declaration static-only tranche として `e14-malformed-duplicate-option-declaration` / `e15-malformed-duplicate-chain-declaration` を actual corpus に追加し、count / selection / static-gate-support / detached smoke を更新した。duplicate cluster は current cut のまま `checked_reasons` と detached `reason_codes` に昇格させず、`checker_core.reasons` と focused smoke で actual wording を見る運用へ固定した。
- 2026-04-05 21:57 JST — missing-option static-only tranche として `e16` / `e17` / `e18` を actual corpus に追加し、`checked_reasons` と detached `reason_codes` の stable malformed cluster を full cargo test・static gate smoke・bundle smoke で実地確認した。次は static-only stable cluster の残 inventory を洗い、actual corpus 追加を続けるか typed carrier 側の narrow actualization に進むかを決める段階。
- 2026-04-05 22:16 JST — `e19-malformed-target-mismatch` を actual corpus に追加し、current docs で stable と列挙している same-lineage static cluster を `checked_reasons` / detached `reason_codes` mirror まで一巡させた。full cargo test・static gate smoke・bundle smoke・review が通ったので、次は typed carrier の first-class actualization を narrow に始めるか、fixture authoring 実地反復をもう少し積むかを選ぶ段階。
- 2026-04-05 22:42 JST — `suggest-reason-codes` display-only assist に fixture-side typed field の fail-closed 検査を追加し、stable cluster では suggestion、duplicate cluster では no-suggestion、unsupported `expected_static.reason_codes` では explicit rejection になることを targeted tests・real smoke・full cargo test・docs validation で確認した。次は assist を実地の fixture authoring 反復へ数回つなぎ、future typed carrier をどの cluster から narrow actualization するかを比較する段階。
- 2026-04-05 23:05 JST — static-only corpus 横断の `scan-reason-code-readiness` helper を追加し、`checked_reasons` adoption と `reason_codes` suggestion availability を batch で display-only 要約できるようにした。current corpus では stable cluster 8 件が suggestion あり、duplicate cluster 2 件が no-suggestion で分かれることを real smoke で確認したので、次は future typed carrier actualization をどの cluster から始めるかを source-backed に絞る段階。
- 2026-04-05 23:45 JST — future typed static reason carrier の first family を lineage edge pair に固定し、その first-class placement を fixture-side additive optional `expected_static.checked_reason_codes` に絞って `e4` / `e5` へ narrow actualize した。`run_bundle()` fail-closed compare、assist、readiness scan の追従まで通ったので、次は declared target edge pair family を次点 tranche として比較する段階。
- 2026-04-05 23:25 JST — declared target edge pair family と remaining stable cluster を `expected_static.checked_reason_codes` へ追加で actualize し、current stable cluster inventory 8 kind が fixture corpus / `run_bundle()` / readiness scan で 8/8 揃う状態まで進めた。duplicate declaration cluster 2 件は引き続き非昇格に留め、次は `checked_reasons` / `checked_reason_codes` coexistence の縮退条件を比較する段階。
- 2026-04-05 23:42 JST — stable static reason `checked_reason_codes` tranche の reviewer finding 2 件を反映し、wording-derived row transform を `lib.rs` public API から外して helper-local に戻し、`plan/07` / `plan/15` / `plan/90` の drift を補正した。Python targeted tests、full `cargo test -p mir-semantics`、docs validation、`git diff --check` が揃ったので、この tranche は commit / push 可能な状態。
- 2026-04-05 23:55 JST — `checked_reasons` / `checked_reason_codes` coexistence task として、readiness scan に stable coexistence anchor / follow-up summary を追加し、stable cluster 8 fixture が 3-way alignment・zero follow-up で収束していることを確認した。そのうえで current judgment を additive coexistence 維持 / shrink 保留に固定し、次は parser boundary / first checker cut 側へ主線を戻すか、deprecation 条件をもう一段比較する段階。
- 2026-04-06 00:05 JST — readiness scan に checker cluster roll-up を追加し、same-lineage `4` / capability `1` / missing-option `3` の source-backed baseline を確認した。mainline を first checker cut 側へ戻す sequencing judgment を docs / plan / report chain に固定し、次は actual checker helper cut を same-lineage floor から切るか、capability floor の corpus coverage を先に厚くするかを比較する段階。
- 2026-04-06 00:14 JST — first checker cut baseline task の review fallback と fresh verification を閉じ、helper-local display-only boundary が壊れていないことを local diff inspection と smoke で確認した。次は actual checker helper cut の first spike を same-lineage floor / missing-option structure floor のどちらから始めるかを narrow に決める段階。
- 2026-04-06 00:27 JST — same-lineage first checker spike として helper-local compare script と `smoke-same-lineage-checker` wrapper を追加し、`e4` / `e12` の matched smoke を確認した。次は missing-option structure floor を second spike にするか、capability floor の corpus coverage を先に厚くするかを比較する段階。
- 2026-04-06 00:39 JST — missing-option second checker spike として helper-local compare script と `smoke-missing-option-checker` wrapper を追加し、`e16` / `e17` の matched smoke を確認した。次は capability floor fixture を増やすか、same-lineage / missing-option spike を shared support helper に薄く寄せるかを比較する段階。
- 2026-04-06 00:56 JST — `e20-malformed-late-capability-strengthening` を actual static-only corpus に追加し、capability strengthening floor を `2` fixture に広げたうえで、helper-local third checker spike と `smoke-capability-checker` wrapper を追加した。`e13` / `e20` の family smoke が通ったので、次は 3 family spike を shared support helper に寄せるか、checker-side shared family compare boundary を別 helper として切る段階。
- 2026-04-06 01:34 JST — same-lineage / missing-option / capability の 3 checker spike で重複していた compare contract を `scripts/current_l2_family_checker_support.py` へ寄せ、family facade script と detached loop command 名は維持した。support tests、existing family tests、3 family smoke が通ったので、次は generic checker-side shared entry が本当に要るかを比較する段階。
- 2026-04-06 01:52 JST — shared support helper 導入後の次段として generic checker-side shared family compare entry を比較し、current phase では追加せず family facade 維持で止める判断を docs / plan / progress / traceability に固定した。次は public checker cut comparison と generic entry を同時に扱う timing を見極める段階。
- 2026-04-06 01:39 JST — `e21-try-atomic-cut-frontier` を runtime corpus に追加し、`TryFallback` body 内 `AtomicCut` が rollback frontier を post-cut snapshot へ進めることを direct evaluator / detached loop smoke / directory summary count まで通して固定した。次は `try` / rollback locality の structural floor を checker 側へどう接続するかを narrow に比較する段階。
- 2026-04-06 02:11 JST — `e22-try-atomic-cut-place-mismatch` を runtime corpus に追加し、nested place の `AtomicCut` が event-only に留まる contrast case を direct evaluator / detached loop smoke / readiness scan / full cargo test まで通した。これにより `TryFallback` / `AtomicCut` の first checker cut には structural floor だけを残し、`place_anchor == current_place` gate と whole-store restore scope は runtime / proof boundary に残す current judgmentを docs / plan / progress で揃えた。
- 2026-04-06 02:30 JST — `scripts/current_l2_detached_loop.py smoke-try-rollback-locality` を追加し、`e22` mismatch と `e21` frontier の representative contrast pair を既定 label つきで 1 command で回せるようにした。validation loop の運用面では、pair-specific smoke を thin wrapper に閉じ込め、public helper boundary は広げない current cut を維持した。
- 2026-04-06 02:37 JST — `TryFallback` / `AtomicCut` structural floor の fourth checker spike 要否を比較し、current phase では existing reason-row family helper に actualize せず、docs/runtime representative に留める current judgment を `specs/examples/52` と plan/progress に固定した。次は parser boundary と first checker API cut がもう一段見えた段階で、dedicated AST structural helper の要否を比較する段階。
- 2026-04-06 03:10 JST — `TryFallback` / `AtomicCut` dedicated AST structural helper の entry criteria を docs-only で固定し、parser/loader malformed source、AST-only floor、dedicated carrier、runtime gate 非依存、複数 family が揃うまでは actual helper を切らない current judgment を整理した。次は structural malformed source を parser / loader / static gate のどこへ置くかを比べる段階。
- 2026-04-06 03:45 JST — `TryFallback` / `AtomicCut` の malformed static family actualization を比較し、current phase では runtime representative `E2` / `E21` / `E22` を current evidence として維持し、malformed static family はまだ actual corpus に増やさない judgment を整理した。次は future dedicated AST structural helper の compare contract を切る段階。
- 2026-04-06 04:02 JST — future dedicated AST structural helper の compare contract を比較し、current phase では row-family 流用や detached artifact shared carrier 先行を避け、helper-local dedicated compare から始める judgment を整理した。次は expected field 名と focused compare shape を narrow に詰める段階。
- 2026-04-06 03:17 JST — future dedicated AST structural helper の expected field 名と focused compare shape を比較し、`expected_static.checked_try_rollback_structural_findings` と `subject_kind` / `finding_kind` の helper-local row list を current docs-only minimum に固定した。次は dedicated helper を detached validation loop のどこへ差し込むのが最小かを比べる段階。
- 2026-04-06 03:24 JST — future dedicated AST structural helper の loop 差し込み位置を比較し、bundle-first runtime path ではなく static gate artifact loop の helper-local smoke family に留める current judgment を固定した。次は structural verdict carrier / name をどこまで narrow に切るかを比べる段階。
- 2026-04-06 03:31 JST — future dedicated AST structural helper の structural verdict carrier / name を比較し、`expected_static.checked_try_rollback_structural_verdict` と helper-local string enum `no_findings` / `findings_present` を current docs-only minimum に固定した。次は detached artifact shared carrier へ上げる閾値をどこに置くかを比べる段階。
- 2026-04-06 04:18 JST — future dedicated AST structural helper を detached artifact shared carrier へ上げる threshold を比較し、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の 5 条件が揃うまでは helper-local dedicated contract に留める current judgment を固定した。次は actual subcommand 名と wrapper family をいつ narrow に切ってよいかを比べる段階。
- 2026-04-06 04:32 JST — future dedicated AST structural helper の wrapper family を比較し、family-specific wrapper は維持しつつ exact subcommand 名は actual helper actualization task まで deferred にする current judgment を固定した。次は future generic structural checker family とどこで合流させるかを比べる段階。
- 2026-04-06 04:46 JST — future dedicated AST structural helper の generic family boundary を比較し、generic structural checker family への合流は later public checker API comparison と同時に扱い、actual helper actualization や wrapper naming cut と混ぜない current judgment を固定した。次は later public checker API comparison に載せる entry criteria をどこまで narrow に切るかを比べる段階。
- 2026-04-06 05:02 JST — future dedicated AST structural helper family の public checker entry criteria を比較し、generic family 合流とは別に、actual helper / fixture contract / corpus / loop stabilization / public comparison pressure の additional threshold を要求する current judgment を固定した。次は malformed static family を actual corpus に増やす timing をどこに置くかを比べる段階。
- 2026-04-06 05:18 JST — future dedicated AST structural helper の malformed static family timing を比較し、current phase の今すぐではなく dedicated helper actualization first tranche と同時に actual corpus へ足す current judgment を固定した。次は helper code / fixture fields / malformed static family / smoke path を first tranche としてどこまで一体で切るかを比べる段階。
- 2026-04-06 05:34 JST — future dedicated AST structural helper の first tranche cut を比較し、helper code / fixture-side fields / minimal malformed static family / static gate smoke path を一体で切り、shared carrier / public checker API は外に残す current judgment を固定した。次は minimal malformed static family tranche の exact size をどこまで narrow に切るかを比べる段階。
- 2026-04-06 05:48 JST — future dedicated AST structural helper の minimal malformed static family tranche size を比較し、`TryFallback` 1 件 + `AtomicCut` 1 件の two-fixture pair を first tranche の最小とする current judgment を固定した。次はその two-fixture first tranche の各 slot に最初に入れる malformed pattern をどこまで narrow に切るかを比べる段階。
- 2026-04-06 05:56 JST — `TryFallback` / `AtomicCut` dedicated AST structural helper の first tranche を actualize し、`e23` / `e24`、helper-local compare、static gate smoke family を実装したうえで、reviewer 指摘だった artifact false-green と public API leak を修正した。full cargo・Python tests・real smoke・docs validation が揃ったので、次は second tranche と helper-local wording stability のどちらを先に比較するかを決める段階。
- 2026-04-06 06:03 JST — second malformed static tranche comparison を先に閉じ、current source だけでは concrete decode-valid family がまだ不足しているため actual tranche 追加は保留と判断した。next は first-tranche wording / finding family stability を narrow に点検する段階。
- 2026-04-06 06:19 JST — parser boundary 主線を再開し、first parser cut inventory 自体は維持したまま、actual parser spike を切るなら chain / declaration structural floor → `try` / rollback structural floor → request / admissibility cluster の checker-led staged order が自然だという current judgment を spec / plan / progress に反映した。次はこの staging を actual parser spike の entry criteria と non-goal 境界へどこまで落とすかを narrow に詰める段階。
- 2026-04-06 06:24 JST — first-tranche wording / finding family stability を比較し、`TryFallback` / `missing_fallback_body` と `AtomicCut` / `disallowed_fallback_placement` を current next phase の exact working set に固定した。generic 化や alias は hidden acceptance と premature genericization を招くため deferred にし、next は shared carrier threshold の再比較へ進む段階。
- 2026-04-06 06:41 JST — first-tranche shared carrier threshold を再比較し、saved artifact compare need の narrow version は helper-local checker が `fixture + artifact_path` で満たせるため、shared detached carrier threshold はまだ未充足と判断した。next は generic structural checker family / public checker API comparison の前提条件を narrow に整理する段階。
- 2026-04-06 06:42 JST — parser boundary 主線の次段として、checker-led staged spike の stage 1 accepted parse cluster / non-goal 境界を比較し、declaration-side guard slot は predicate parser の入口ではなく option declaration に attached した opaque slot に留める current judgment を spec / plan / progress に反映した。次はこの opaque slot を actual parser / checker handoff に送るときの最小 carrier 名と parser-free AST fixture schema との handoff cut を narrow に詰める段階。
- 2026-04-06 06:48 JST — stage 1 の declaration-side guard slot handoff を比較し、parser-side opaque slot carrier と current parser-free AST fixture schema を同一視せず、`OptionDecl.lease` へ thin lowering bridge を介して接続する current judgment を spec / plan / progress に反映した。次はこの bridge の actual carrier 名と actual parser / checker handoff API をどこまで narrow に決めるかを比べる段階。
- 2026-04-06 07:02 JST — first-tranche generic/public recheck を行い、second concrete family と shared/public pressure がまだ不足しているため、current try/rollback checker line はここで一旦 pause と判断した。next self-drivable mainline は parser boundary / first parser cut inventory 側へ移す段階。
- 2026-04-06 07:12 JST — stage 1 parser-side opaque slot carrier の naming candidate と thin lowering bridge API surface を比較し、`decl_guard_slot` を第一候補にし bridge は slot-only ではなく option-level structural transfer として読む current judgment を spec / plan / progress に反映した。次はこの option-level bridge を actual parser spike の smoke family にどこまで narrow に接続するかを比べる段階。
- 2026-04-06 07:23 JST — stage 1 actual parser spike の smoke family を比較し、`e4-malformed-lineage` と `e7-write-fallback-after-expiry` の two-fixture pair を最小 working set にし、`e3-option-admit-chain` は later-stage contrast reference に残す current judgment を spec / plan / progress に反映した。次はこの working set を actual private parser spike module / helper 配置へどこまで narrow に落とすかを比べる段階。
- 2026-04-06 07:34 JST — actual stage 1 parser spike の private helper 配置と compare surface を比較し、`crates/mir-ast/tests/support/` 配置の private helper + lowered fixture-subset compare を最小 cut とする current judgment を spec / plan / progress に反映した。次はこの配置で non-production の actual stage 1 parser spike を実装に入れてよいか、input surface と test inline string 方針を narrow に詰める段階。
- 2026-04-06 07:43 JST — actual stage 1 parser spike の input surface / internal carrier / private helper naming を比較し、test inline string、dedicated wrapper + owned `surface_text`、`current_l2_stage1_parser_spike_support` を第一候補とする current judgment を spec / plan / progress に反映した。次は non-production の actual stage 1 parser spike を `crates/mir-ast/tests/support/` で実装し、`e4` / `e7` の lowered fixture-subset compare smoke を通す段階。
- 2026-04-06 08:08 JST — stage 1 parser spike の first tranche を actualize し、`crates/mir-ast/tests/support/current_l2_stage1_parser_spike_support.rs` と `crates/mir-ast/tests/current_l2_stage1_parser_spike.rs` で `e4` / `e7` の lowered fixture-subset compare と guard-slot retention smoke を通した。`cargo test -p mir-ast`、`cargo test -p mir-semantics`、docs validation まで揃ったので、次は malformed-source smoke か stage 2 contrast (`e3`) のどちらを先に narrow に切るかを比べる段階。
