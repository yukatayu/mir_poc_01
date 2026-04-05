# progress

最終更新: 2026-04-05（static reason code entry criteria 整理時点）

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
- detached validation loop は、docs-only judgment、bundle-first emitter、aggregate emitter、bundle diff helper、aggregate diff helper、tiny loop wrapper、fixture authoring template、fixture scaffold helper まで入った。
- runtime regression catalog に `e9-monotone-degradation-success` を追加し、middle explicit failure のあとでも later same-lineage write-capable option へ monotone degradation して success しうることを machine-check で固定した。
- runtime regression catalog に `e10-perform-on-ensure-failure` を追加し、direct `PerformOn` の request-local `ensure` unsatisfied が `Reject` ではなく `explicit_failure` であり、success-side carrier preview が commit されないことを machine-check で固定した。
- runtime regression catalog に `e11-perform-via-ensure-then-success` を追加し、via-chain の earlier option が request-local `ensure` で失敗しても later same-lineage option へ継続して success しうることを machine-check で固定した。
- fixture authoring bottleneck のうち boilerplate 部分は、`target/current-l2-fixture-scaffolds/` 下に required carrier と empty sidecar 骨格だけを出す non-production helper で narrow に補助できる状態になった。
- detached validation loop には `smoke-fixture` convenience が入り、1 fixture の bundle export、optional reference compare、single-fixture aggregate smoke を 1 command で回せるようになった。
- aggregate emitter 内 private transform を `examples/support/current_l2_detached_aggregate_support.rs` へ切り出し、`BatchRunSummary -> detached aggregate artifact` の repo 内 callable boundary を non-production のまま shared helper 化した。
- static gate verdict / reasons を `examples/support/current_l2_static_gate_support.rs` と `current_l2_emit_static_gate.rs` から detached artifact として保存できるようにし、static-only / malformed / underdeclared fixture を `smoke-static-gate` で compare できるようにした。
- `expected_static.reasons` を `run_bundle()` の actual machine-check に上げる試行を行い、current fixture corpus では explanatory note と machine-check 候補が混在しているため、そのまま昇格できないことを確認した。そのうえで additive optional `expected_static.checked_reasons` を導入し、field があるときだけ actual static gate reasons を fail-closed compare できる narrow carrier を入れた。
- `checked_reasons` の次段として、typed reason code へ進めてよい stable cluster と parametric shape の entry criteria を docs-only で棚卸しした。immediate 全面 code 化は避け、cluster inventory を先に固める方針になった。
- parser boundary については、final grammar を先に凍らせずに first parser cut に入れてよい semantic cluster を先に inventory 化する段階へ入った。
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
| Mir current L2 core semantics | 82% | 72% | 68% | 着手可能 | current task を回すには十分安定、ただし final formalization はまだ先 |
| fallback / notation / representative examples | 84% | 79% | 62% | 着手可能 | drift 抑制は進んだが final parser grammar は未決 |
| parser-free PoC execution stack | 87% | 81% | 95% | 着手可能 | interpreter / host / bundle / batch / selection / profile に加え、aggregate transform と static gate transform の repo 内 callable boundary が shared helper として入り、static gate reasons は detached helper compare と optional checked carrier の境界まで整理された |
| detached export / validation loop | 87% | 83% | 95% | 着手可能 | bundle / aggregate / static gate emitter、bundle / aggregate / static gate compare helper、wrapper、storage candidate、scaffold helper、fixture smoke helper まで揃い、runtime と static gate の loop 実地反復がさらに進んだ |
| fixture authoring / elaboration 実務 | 78% | 81% | 83% | 着手可能 | template / scaffold helper は揃い、runtime fixture と static-only fixture の detached compare path に加え、optional `checked_reasons` で bundle-level static compare を narrow に足せるようになった |
| parser / syntax finalization 準備 | 46% | 52% | 20% | 着手可能 | first parser cut に入れてよい semantic cluster の inventory までは見えたが、final grammar と exact lexical choice は未決 |
| richer host interface / coverage typed 化 | 24% | 22% | 16% | 後段依存 | comparison までは進んだが implementation cut は後段 |
| aggregate export の typed actualization | 58% | 50% | 61% | 着手可能 | non-production aggregate emitter と aggregate compare helperに加え、aggregate transform の actual narrow cut は shared support helper まで進んだが public API と final compare 契約は未決 |
| static analysis / type / theorem prover workstream | 37% | 31% | 24% | 着手可能 | first checker cut の local / structural floor に加え、static gate reasons をそのまま harness machine-check へ昇格できないことを確認し、additive optional `checked_reasons` を最小 dedicated carrier として実装した。typed code は stable cluster inventory と parametric shape 条件まで整理済みで、全面移行はまだ OPEN |
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

- **ある程度自律的に「追加し、回し、比較し、次へ進む」状態まで**: あと **3〜6** task 程度
- 想定する中身:
  1. detached validation loop の入口を安定化する
  2. fixture authoring / elaboration の反復コストをもう一段下げる
  3. parser 前 inventory を作る
  4. richer host interface を後段に送ったまま、今必要な host coverage の cut だけ固める
  5. current L2 semantics の追加 regression を数本回して、巻き戻りが必要か確認する
  6. portability / observability の boundary を壊さない helper cut を確認する

## いまの判断

- **今の mainline は正しい**。Mir current L2 と parser-free PoC を主線に据える方針は維持でよい。
- 直近の主ボトルネックは、意味論の大変更ではなく **validation loop の運用面** と **fixture authoring** である。
- final parser grammar や richer host interface を急ぐより、まず「artifact を保存し、比較し、fixture を増やしてまた回す」ループを安定化させるべき段階にある。
- portability / observability は今すぐ runtime core に組み込む話ではないが、helper cut と将来 API の形を歪めないよう early reminder として管理する。

## 作業ログ（簡潔）

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
- 2026-04-05 19:35 JST — additive optional `expected_static.checked_reasons` を導入し、field があるときだけ `run_bundle()` が actual static gate reasons を fail-closed compare する narrow carrier を追加した。targeted RED/GREEN が通ったので、次は typed reason code へ進むかこの carrier を fixture authoring に広げるかを比較する段階。
- 2026-04-05 19:52 JST — `checked_reasons` から typed reason code へ進める条件を docs-only で整理し、stable cluster と parametric shape の entry criteria を切り出した。next は static gate reason code を detached artifact 側へ mirror するか、fixture authoring で checked carrier 採用を広げるかの比較段階。
