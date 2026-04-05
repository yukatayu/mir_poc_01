# plan/15 — current L2 fixture authoring template

## 目的

この文書は、current L2 parser-free PoC 基盤に新しい fixture を 1 本追加するときの最小テンプレートを与える。

ここでの目的は fixture authoring / elaboration を完全自動化することではなく、

- 何を必ず揃えるか
- runtime fixture と static-only fixture で何が違うか
- detached artifact loop に入った後に何を追加で見るか
- どこから先が host interface / exporter / batch responsibility か

を、task ごとに拾い直さなくて済む形で固定することにある。

## fixture authoring の最小 checklist

新しい current L2 fixture を 1 本足すときは、最低でも次を確認する。

1. AST fixture
2. runtime / static-only の判定
3. host plan sidecar の有無
4. `expected_static`
5. `expected_runtime`
6. `expected_trace_audit`
7. named profile / selection への影響
8. detached artifact loop に入れるなら比較観点の追加
9. detached artifact 保存先 / run label / compare 手順の確認
10. boilerplate を scaffold helper で起こすか、手で直接書き始めるかの判断

## 1. AST fixture

### 必須

- `schema_version`
- `fixture_id`
- `source_example_id`
- `program`
- `expected_static`
- `expected_runtime`
- `expected_trace_audit`

### 書くときの確認点

- current L2 parser-free AST fixture schema に乗っているか
- new semantics や new failure class を勝手に足していないか
- `perform` / option chain / `try` / `fallback` / `atomic_cut` の current reading を越えていないか

## 2. runtime fixture と static-only fixture の違い

### static-only fixture

- `expected_runtime.enters_evaluation = false`
- `.host-plan.json` sidecar は不要
- static gate verdict と reasons が主な比較軸になる
- current harness / `run_bundle()` は `expected_static.verdict` を fail-closed に照合する
- `expected_static.reasons` は current fixture corpus では explanatory note も兼ねるため、actual compare は detached static gate artifact 側に残す
- optional `expected_static.checked_reasons` を置くときだけ、`run_bundle()` は actual static gate reasons を fail-closed compare してよい
- future dedicated AST structural helper を actualize する場合の optional expected field 候補は `expected_static.checked_try_rollback_structural_findings` であるが、current phase では **まだ fixture schema の actual field ではない**
- 同様に future dedicated AST structural verdict 候補は `expected_static.checked_try_rollback_structural_verdict` であるが、current phase では **まだ fixture schema の actual field ではない**
- 同様に future dedicated AST structural helper を detached artifact shared carrier へ mirror するのも、helper actualization、fixture-side field actualization、static corpus、loop stabilization、saved artifact compare need の threshold が揃うまでは **まだ行わない**
- 同様に future dedicated AST structural helper の exact subcommand 名も、actual helper actualization task までは **まだ fix しない**
- 同様に future dedicated AST structural helper を generic structural checker family へ合流させる比較も、later public checker API comparison までは **まだ行わない**
- 同様に future dedicated AST structural helper family を later public checker API comparison に載せるのも、actual helper / fixture contract / corpus / loop stabilization / public comparison pressure が揃うまでは **まだ行わない**
- 同様に future dedicated AST structural helper の malformed static family も、current phase の今すぐではなく dedicated helper actualization first tranche で **初めて actual corpus に足す**
- current stable cluster tranche としては、stable inventory 8 kind に限って optional `expected_static.checked_reason_codes` を additive に置いてよい
- current tranche で allowed な kind は次である
  - `missing_lineage_assertion`
  - `lineage_assertion_edge_mismatch`
  - `declared_target_missing`
  - `declared_target_mismatch`
  - `capability_strengthens`
  - `missing_chain_head_option`
  - `missing_predecessor_option`
  - `missing_successor_option`
- first adoption は `e4` / `e5` から始まり、その後 `e12` / `e13` / `e16` / `e17` / `e18` / `e19` / `e20` まで stable cluster を広げてよい
- current corpus では `e12` / `e13` / `e20` のような declared target missing / capability strengthening family まで stable cluster を広げてよい
- current corpus では `e16` / `e17` / `e18` のような missing head / predecessor / successor option cluster まで stable malformed cluster を広げてよい
- current corpus では `e19` のような declared target mismatch cluster まで stable malformed cluster を広げてよい
- `e14` / `e15` のような duplicate declaration cluster は actual corpus に入れてよいが、current helper cut では `checked_reasons` と detached `reason_codes` を absent のままにして、actual wording は targeted test / detached smoke 側で見る方が自然である
- actual static gate `reasons` が空で、fixture 側 `reasons` だけが explanatory prose を担う valid fixture では、`checked_reasons` を無理に `[]` で足さない
- `checked_reasons` を採用するか迷うときは、`scripts/current_l2_detached_loop.py suggest-checked-reasons` で static gate artifact の actual `checker_core.reasons` を見て、display-only suggestion を確認してよい
- future typed carrier 候補 row を見たいときは、`scripts/current_l2_detached_loop.py suggest-reason-codes` で `detached_noncore.reason_codes` の display-only suggestion を確認してよい
- tranche 単位で static-only fixture corpus の current split を見たいときは、`scripts/current_l2_detached_loop.py scan-reason-code-readiness <fixture-dir> --run-label <label> --overwrite` を使い、`checked_reasons` adoption、`reason_codes` suggestion availability、stable coexistence anchor 数、follow-up 必要 fixture をまとめて確認してよい
- current stable cluster tranche では `checked_reason_codes` が fixture schema の field になってよく、対象は stable inventory 8 kind に限る
- future dedicated `TryFallback` / `AtomicCut` AST structural helper を actualize するなら、loop 接続の第一候補は bundle-first runtime path ではなく static gate artifact emit のあとに helper-local compare を回す dedicated smoke family である
- duplicate declaration cluster の `reason_codes` suggestion は引き続き absent であり、future row 候補としても current tranche には上げない
- helper は current fixture schema に無い `expected_static.reason_codes` を見つけたら fail-closed に止まる
- 同様に、current helper stack では `expected_static.checked_try_rollback_structural_findings` を actual field としてまだ受けない
- 同様に、current helper stack では `expected_static.checked_try_rollback_structural_verdict` も actual field としてまだ受けない
- 同様に、current helper stack では dedicated AST structural helper の shared detached carrier も actualize しない
- 同様に、current helper stack では dedicated AST structural helper の exact family-specific subcommand もまだ actual command surface に上げない
- 同様に、current helper stack では dedicated AST structural helper の generic structural checker family entry もまだ actualize しない
- 同様に、current helper stack では dedicated AST structural helper の public checker API family entry もまだ actualize しない
- 同様に、current helper stack では dedicated AST structural helper 向け malformed static family もまだ actual corpus に入れない
- ただし helper は fixture JSON を自動更新しない。採用は hand-edit と review で行う
- detached artifact loop に入れても、payload core の中心は `static_verdict` と `entered_evaluation = false` になる
- detached validation loop continuation では、runtime bundle artifact と別に static gate artifact を保存し、`checker_core.static_verdict` / `checker_core.reasons` を compare してよい
- optional `detached_noncore.reason_codes` が出ても current では reference-only に留め、fixture 側 expected carrier や machine-check core へはまだ上げない

### runtime fixture

- `expected_runtime.enters_evaluation = true`
- `.host-plan.json` sidecar が必須
- `terminal_outcome`、`event_kinds`、formal `non_admissible_metadata`、short `narrative_explanations` まで比較する
- detached artifact loop に入れるときは `bundle_context.host_plan_path` と `runtime_requirement` が必須になる

### profile-targeted run

- fixture 自体は runtime fixture か static-only fixture のどちらかである
- ただし detached validation loop に載せるときは、その fixture が
  - `runtime-only`
  - `static-only`
  - `single-fixture`
  - named profile
  のどれで拾われる想定かを別に確認する
- profile-targeted run は fixture authoring の主目的ではないが、fixture 追加が profile の意味を変える場合は authoring task 側で mirror 更新要否を確認する

## 3. host plan sidecar の有無

### sidecar が必要なとき

- runtime fixture である
- predicate / effect oracle call が入る
- current host harness の fail-closed behavior を通す必要がある

### sidecar が不要なとき

- static-only fixture で evaluation に入らない

### 注意

- sidecar を足すかどうかは fixture authoring の責務であり、exporter や batch helper の責務ではない
- `.host-plan.json` の placement / loading は current helper stack の既存 boundary を使う

## 4. `expected_static`

最低でも次を揃える。

- `verdict`
- `reasons`
- optional `checked_reasons`
- optional `checked_reason_codes`

確認点:

- `valid` / `malformed` / `underdeclared` の current gate judgment と一致しているか
- malformed / underdeclared を runtime 側でごまかしていないか
- `checked_reasons` を置くなら detached static gate artifact の actual `checker_core.reasons` と一致しているか
- `checked_reason_codes` を置くなら current tranche の allowed kind だけに留まっているか
- `checked_reason_codes` を置くなら `run_bundle()` の actual typed compare と detached static gate artifact の suggestion が一致しているか

## 5. `expected_runtime`

最低でも次を揃える。

- `enters_evaluation`
- `final_outcome`
- `notes`

確認点:

- runtime fixture なら `final_outcome` が current L2 reading と一致しているか
- static-only fixture なのに `enters_evaluation = true` になっていないか
- `notes` は machine-check core ではなく、補助説明であることを混同していないか

## 6. `expected_trace_audit`

最低でも次を揃える。

- `event_kinds`
- `non_admissible_metadata`
- `narrative_explanations`
- `must_explain`

### machine-check に残すもの

- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

### human-facing に残すもの

- `must_explain`

ここでは `must_explain` を current L2 policy に従って machine-check に上げない。

## 7. named profile / selection への影響

新しい fixture を足したときは、次を確認する。

- `runtime-only` / `static-only` の既存 selection に影響するか
- `single-fixture` selector で明示的に拾う必要があるか
- current named profile catalog
  - `smoke-runtime`
  - `smoke-static`
  - `runtime-e3`
  - `static-e4`
  の意味を変えるか

### 原則

- 既存 alias の意味を変える場合は helper stack / tests / docs mirror を同 task で更新する
- 単に fixture 数が増えるだけなら alias catalog をむやみに増やさない

## 8. detached artifact loop に入った後の追加観点

fixture を detached artifact loop に入れるなら、fixture expectation だけでなく次も見る。

### payload core

- `static_verdict`
- `entered_evaluation`
- `terminal_outcome`
- `event_kinds`
- formal `non_admissible_metadata`
- short `narrative_explanations`

### `bundle_context`

- `fixture_id`
- `fixture_path`
- `host_plan_path`
- `runtime_requirement`

### `detached_noncore`

- `steps_executed`
- optional coverage explanation
- optional host-plan explanation

### current non-production 保存先候補

- current detached validation loop の non-production default candidate は `target/current-l2-detached/`
- bundle artifact は `bundles/<run-label>/<fixture-stem>.detached.json` を基本にする
- compare は explicit artifact path を直接渡すか、thin wrapper が `artifact_root + run_label + fixture_stem` から path を導出する
- final path policy ではないので、repo 追跡下の固定保存先と誤読しない

### 比較 helper で見ないもの

- `must_explain`
- long-form audit
- why-this-is-good/bad の prose

## 9. どこから先が fixture authoring で、どこから先が別責務か

| 境界 | fixture authoring の責務 | 別責務 |
|---|---|---|
| AST shape | fixture JSON を current schema に合わせる | final parser syntax |
| runtime/static 判定 | `expected_runtime.enters_evaluation` と sidecar 必要性を決める | production host interface |
| expectation 記述 | `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える | exporter API / detached path policy |
| sidecar 記述 | `.host-plan.json` を必要な runtime fixture に付ける | richer host interface typed carrier |
| detached artifact 比較 | payload core / `bundle_context` / `detached_noncore` のどこを見るか決める | batch aggregate export の final shape |
| profile-targeted run | profile / selection への影響を確認する | alias catalog の final 設計、profile helper の public API |

## 10. 新しい fixture を足すときの実務テンプレート

### scaffold helper を使ってよい範囲

- `scripts/current_l2_scaffold_fixture.py` は non-production helper である
- default candidate は `target/current-l2-fixture-scaffolds/`
- helper が作ってよいのは
  - top-level required carrier
  - runtime / static-only の別
  - runtime fixture 用の empty `.host-plan.json` sidecar
  までである
- helper が作ってはいけないのは
  - completed `program`
  - completed `expected_static`
  - completed `expected_runtime`
  - completed `expected_trace_audit`
  である

scaffold を使うときは、まず `target/` 下に skeleton を出し、その後に review 可能な hand-written fixture へ詰める。

### static-only fixture

1. fixture JSON を追加する
2. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
3. static-only selection と representative catalog の更新要否を確認する
4. detached validation loop に入れるなら `emit-static-gate` で static gate artifact を保存する
5. `checked_reasons` の採用候補を narrow に見たいときは `suggest-checked-reasons` を使い、actual `checker_core.reasons` から display-only suggestion を得る
6. current stable cluster tranche を採用する場合は `suggest-reason-codes` で row を確認し、`expected_static.checked_reason_codes` を hand-edit で追加する
7. same-lineage family の fixture で first checker spike も見たい場合は `smoke-same-lineage-checker` を使い、static gate artifact の actual row と fixture-side row が一致しているかを narrow に確認する
8. missing-option family の fixture で second checker spike も見たい場合は `smoke-missing-option-checker` を使い、static gate artifact の actual row と fixture-side row が一致しているかを narrow に確認する
8. current tranche で fixture schema へ上げてよいのは stable inventory 8 kind に限り、duplicate declaration cluster は reference-only suggestion / actual wording のままにする
9. 必要なら既存 artifact と `checker_core` を比較する
10. helper は fixture JSON を更新しないので、採用する場合は hand-edit して `run_bundle()` と detached static gate compare の両方で確認する
11. runtime artifact も参考に見たいときだけ `emit-fixture` を併用する
12. 1 command でまとめたいときは `scripts/current_l2_detached_loop.py smoke-static-gate` を non-production convenience として使ってよい
13. directory-level の差を見たいときは `emit-aggregate` のあと `compare-aggregates` で `summary_core` を比較する

### runtime fixture

1. fixture JSON を追加する
2. `.host-plan.json` sidecar を追加する
3. `expected_static` / `expected_runtime` / `expected_trace_audit` を揃える
4. host plan sidecar が current host harness の fail-closed rule を満たすか確認する
5. bundle-first exporter で 1 bundle artifact を出す
6. diff helper で exact-compare core を比較する
7. directory-level の差も見たいときは `emit-aggregate` と `compare-aggregates` を使う
8. profile / selection への影響を確認する
9. current non-production convenience としては、`smoke-fixture` で
   - target fixture artifact
   - optional reference fixture compare
   - full directory vs single-fixture aggregate smoke
   を 1 command で回してよい

runtime fixture を最初から全部手で書く代わりに、scaffold helper で骨組みを起こしてから埋めてもよい。

### profile-targeted run

1. fixture を追加 / 更新する
2. その fixture がどの selection / named profile に入るべきかを確認する
3. detached bundle artifact 自体は bundle-first loop で確認する
4. aggregate summary の差を見たいときは aggregate artifact を保存して compare する
5. selection / profile の妥当性は batch / profiled run 側で確認する
6. profile alias の意味が変わるなら docs / tests / code mirror を同 task で更新する

## 11. detached exporter loop との接続

current detached export loop では、fixture は次の単位で PoC を回す。

1. fixture を追加 / 更新する
2. `scripts/current_l2_detached_loop.py emit-fixture` か `compare-fixtures` で bundle artifact を保存する
3. minimal diff helper で payload core を比較する
4. 必要なら `bundle_context` と detached_noncore を reference-only として読む
5. directory-level の smoke を取りたいときは `emit-aggregate` で aggregate summary も保存する
6. aggregate summary 同士を比べたいときは `compare-aggregates` で `summary_core` を比較する
7. batch / profile の論点があるときだけ別に `run_directory_profiled` / named profile 側を見る
8. fixture を追加してまた回す

この loop は fixture authoring を完全自動化しない。
ただし、「1 本ずつ狭く正確に回す」から「artifact を保存し、比較し、また 1 本足す」へ進む最小入口としては十分である。

## 12. fixture authoring と exporter / batch / host interface の責務境界

### fixture authoring の責務

- fixture JSON と `.host-plan.json` sidecar を揃える
- `expected_static` / `expected_runtime` / `expected_trace_audit` を current semantics に合わせる
- detached bundle artifact を出して payload core を確認する
- profile / selection への影響有無を確認する

### exporter / batch の責務

- bundle artifact の保存 path を導く
- batch aggregate export を coarse summary として扱う
- `bundle_failure_kind_counts` のような aggregate typed field を fixture JSON に押し込まない

### host interface 側へ送るもの

- typed uncovered-call carrier
- preflight coverage analysis
- richer host-plan explanation carrier

これらは fixture authoring task ではなく、後段の richer host interface workstream で扱う。

## 13. 依然として OPEN のもの

- actual exporter API
- detached artifact 保存先と path policy
- actual elaboration helper
- richer host interface の typed carrier 化
- final parser syntax
- multi-request scheduler
- `Approximate` / `Compensate`
