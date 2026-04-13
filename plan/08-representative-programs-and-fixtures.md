# plan/08 — representative programs と fixtures

## representative programs の位置づけ

- representative programs は current L2 の説明用コードである
- parser-ready な最終 syntax ではない
- parser-free PoC では、これらを machine-readable fixture へ正規化して検証する

## layered sample stack の current reading

- current roadmap では、少なくとも 3 層の sample 表現を区別して扱うのが自然である。
  - representative programs
  - machine-readable fixture corpus
  - fixed-subset source-sample corpus
- representative programs は人間向けの説明用であり、final grammar を固定しない。
- fixture corpus は machine-check の current baseline であり、parser-free / detached loop / static gate / interpreter の正本に近い。
- source-sample corpus は、**representative programs をそのまま昇格するものでも、fixture JSON を逆生成するものでもなく**、fixed subset の syntax-backed regression layer として別に置くのが current first choice であり、repo-root `samples/current-l2/` flat `.txt` layer に置く current cut まで fixed 済みである。
- current near-term では、source-sample corpus は
  - fixed subset だけを対象にする
  - representative prose と fixture corpus の対応表を持つ
  - `static gate` / `interpreter` / `formal hook` reached stage を sample ごとに段階化する
  という条件で整備するのが自然である。

## source-sample corpus の current path policy

- current path は `samples/current-l2/` に置く。
- current file layout は flat / one-file-per-sample である。
- current extension policy は `.txt` であり、final grammar を意味しない。
- current naming policy は fixture stem / `fixture_id` aligned kebab-case である。
- current initial cluster は `e1` / `e2` / `e3` / `e4` / `e21` / `e23` に留める。
- current authored corpus はその initial sextet に `e22-try-atomic-cut-place-mismatch` と `e19-malformed-target-mismatch` を加えた octet に進んでいる。

## current mapping matrix cut

- `specs/examples/317...318` により、initial cluster 6 本は representative prose / fixture corpus / source target path を 1 row で結ぶ current matrix cut に fixed 済みである。
- row minimum は
  - `ladder_order`
  - `sample_stem`
  - `representative_anchor_ref`
  - `representative_status`
  - `source_example_id`
  - `fixture_ref`
  - `fixture_id`
  - `fixture_mode`
  - `source_sample_target_ref`
  - `coverage_focus`
  - `expected_static_verdict`
  - `expected_runtime_outcome`
  である。
- `representative_status` は current cut で `direct` / `variant` / `unresolved` を使う。
- `e3-option-admit-chain` は representative prose plain `E3` ではなく `E3-variant` row として扱う。
- `e23-malformed-try-fallback-missing-fallback-body` は fixture-side `source_example_id = E23` を already 持つが、current representative prose row は未整備なので `representative_status = unresolved` に留める。
- current matrix 自体では reached stage や bless policy をまだ持ち込まず、source target path ref までを minimum に残す。
- source sample の actual file current authored octet は `e1` / `e2` / `e3` / `e4` / `e19` / `e21` / `e22` / `e23` に置き、これらは current runner first cut の input として actualize 済みである。`e3` は admit-family third slot として source row / runner / inventory / ladder までは実装済みだが、formal hook stage は guarded に留めるのが current first choice である。
- verification ladder current cut では、current authored octet に reached stage row を付ける。
  - `e1` は `static gate = reached(valid)`、`interpreter = reached(explicit_failure)`、`formal hook = reached(runtime_try_cut_cluster)` と読む。
  - `e2` は `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = reached(runtime_try_cut_cluster)` と読む。
  - `e3` は `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = not reached (guarded)` と読む。
  - `e4` は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` と読む。
  - `e19` は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` と読む。
  - `e21` は `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = reached(runtime_try_cut_cluster)` と読む。
  - `e22` は `static gate = reached(valid)`、`interpreter = reached(success)`、`formal hook = reached(runtime_try_cut_cluster)` と読む。
  - `e23` は `static gate = reached(malformed)`、`interpreter = not reached (static stop)`、`formal hook = reached(fixture_static_cluster)` と読む。
- source-sample authoring policy current cut では、repo-local flow を `.docs/current-l2-source-sample-authoring-policy.md` と `scripts/current_l2_source_sample_regression.py` に寄せる。
  - `inventory` は current authored octet の file presence と guarded row status を確認する。
  - `regression` は lowering / runner / ladder bundle を current authored octet に流し、formal-hook smoke sub-bundleは `runtime_try_cut_cluster` / `fixture_static_cluster` current top に入る 7 row だけに流す。
  - current `bless` は retained artifact archive ではなく、source / fixture / matrix / ladder / snapshot docs の reviewed sync、`inventory` / `regression` success、必要なら emitted review-unit / model-check carrier helper output の inspection を意味する。
- post-sextet first cluster current cut では、first broader authored-row family は `e21` / `e22` try-rollback locality contrast に置く。
  - `e21` は source-authored frontier-update anchor である。
  - `e22` は source-authored place-mismatch contrast row として actualize 済みである。
  - stable-static edge-pair first reopen では、existing `e4` row と new `e19` row を source-backed static-stop pair に actualize 済みである。
  - parser / checker / runtime public surface inventory、Mirrorea/shared-space docs-first re-entry、model-check/public-checker second reserve inventory、public operational surface actualization gate は fixed 済みである。
  - stable malformed broader follow-up inventory も fixed 済みであり、broader stable malformed next reopen order は missing-option family first、capability family second、duplicate cluster と `TryFallback` / `AtomicCut` malformed-static family は later に残す。
  - public operational CLI / final public contract later gate も fixed 済みであり、public-side later ordering は final public parser / checker / runtime API first、public operational CLI second に残す。
  - source-sample emitted verification artifact wiring も fixed 済みであり、`run_current_l2_source_sample` の public/report shape を変えず、runtime test/support helper-local route として reached row を review-unit / model-check carrier helper output へ fan-out する current cutを actualize 済みである。
  - sample-facing theorem/model-check evidence summary and bless/review flow も fixed 済みであり、README / `.docs` / snapshot docs を sample-facing surface、reviewed repo-local sync + inventory/regression success を current bless に置く docs-first cut を採っている。
  - repo-level current line は docs-first I/O / host-facing port boundary に置く。
  - expiry / monotone degradation family と request-contract family は current formal-hook top や parser pressure を広げやすいため still later に残す。

## fixture 一覧

### runtime fixture

- `e1-place-atomic-cut`
- `e2-try-fallback`
- `e21-try-atomic-cut-frontier`
- `e22-try-atomic-cut-place-mismatch`
- `e3-option-admit-chain`
- `e6-write-after-expiry`
- `e7-write-fallback-after-expiry`
- `e8-monotone-degradation-reject`
- `e9-monotone-degradation-success`
- `e10-perform-on-ensure-failure`
- `e11-perform-via-ensure-then-success`

### static-only fixture

- `e4-malformed-lineage`
- `e23-malformed-try-fallback-missing-fallback-body`
- `e5-underdeclared-lineage`
- `e12-underdeclared-target-missing`
- `e13-malformed-capability-strengthening`
- `e14-malformed-duplicate-option-declaration`
- `e15-malformed-duplicate-chain-declaration`
- `e16-malformed-missing-chain-head-option`
- `e17-malformed-missing-predecessor-option`
- `e18-malformed-missing-successor-option`
- `e19-malformed-target-mismatch`
- `e20-malformed-late-capability-strengthening`

## fixture catalog

| fixture | 主題 | static verdict | runtime outcome | trace / audit expectation の要点 | host plan sidecar | 関連 sample / report |
|---|---|---|---|---|---|---|
| `e1-place-atomic-cut` | `place` 入れ子、success、`atomic_cut`、後続 failure | `valid` | `explicit_failure` | `perform-success` → `atomic-cut` → `perform-failure` | あり | E1, `0047`, `0054` |
| `e2-try-fallback` | local rollback と explicit fallback branch | `valid` | `success` | `perform-success` → `perform-failure` → `rollback` → `perform-success` | あり | E2, `0047`, `0049`, `0054` |
| `e21-try-atomic-cut-frontier` | `try` body 内 `atomic_cut` による rollback frontier 更新 | `valid` | `success` | `perform-success` → `atomic-cut` → `perform-success` → `perform-failure` → `rollback` → `perform-success` | あり | E21, `0184` |
| `e22-try-atomic-cut-place-mismatch` | nested place 内 `atomic_cut` が frontier を更新しない contrast case | `valid` | `success` | `perform-success` → `atomic-cut` → `perform-failure` → `rollback` → `perform-success` | あり | E22, `0186` |
| `e3-option-admit-chain` | option-local `admit` miss と later success | `valid` | `success` | event は `perform-success`、formal metadata に `admit-miss` | あり | E3 variant, `0037`, `0039`, `0078` |
| `e4-malformed-lineage` | edge-local lineage annotation mismatch | `malformed` | `not_evaluated` | runtime に入らない | なし | E4, `0023`, `0047`, `0147` |
| `e23-malformed-try-fallback-missing-fallback-body` | empty `fallback_body` を dedicated structural malformed として止める stage 2 try/rollback malformed pair | `malformed` | `not_evaluated` | runtime に入らない。stage 2 dedicated structural checker と parser reconnect evidence にも使う | なし | E23, `0222`, `0339` |
| `e5-underdeclared-lineage` | lineage 証拠不足 | `underdeclared` | `not_evaluated` | runtime に入らない | なし | E5, `0021`, `0022`, `0047`, `0147` |
| `e12-underdeclared-target-missing` | declared access target 欠落 | `underdeclared` | `not_evaluated` | runtime に入らない | なし | E12, `0155` |
| `e13-malformed-capability-strengthening` | capability strengthening | `malformed` | `not_evaluated` | runtime に入らない | なし | E13, `0155` |
| `e14-malformed-duplicate-option-declaration` | duplicate option declaration | `malformed` | `not_evaluated` | runtime に入らない | なし | E14, `0157` |
| `e15-malformed-duplicate-chain-declaration` | duplicate chain declaration | `malformed` | `not_evaluated` | runtime に入らない | なし | E15, `0157` |
| `e16-malformed-missing-chain-head-option` | missing chain head option | `malformed` | `not_evaluated` | runtime に入らない | なし | E16, `0159` |
| `e17-malformed-missing-predecessor-option` | missing predecessor option | `malformed` | `not_evaluated` | runtime に入らない | なし | E17, `0159` |
| `e18-malformed-missing-successor-option` | missing successor option | `malformed` | `not_evaluated` | runtime に入らない | なし | E18, `0159` |
| `e19-malformed-target-mismatch` | declared target mismatch | `malformed` | `not_evaluated` | runtime に入らない | なし | E19, `0161` |
| `e20-malformed-late-capability-strengthening` | earlier read-to-read continuation の後に later read-to-write escalation を置く malformed static stop | `malformed` | `not_evaluated` | runtime に入らない | なし | E13 late-edge variant, `0178` |
| `e6-write-after-expiry` | write-capable option expiry + later read-only only | `valid` | `Reject` | `lease-expired` formal metadata、capability mismatch narrative explanation、final `Reject` | あり | E6, `0039`, `0045`, `0078` |
| `e7-write-fallback-after-expiry` | expiry 後に later write-capable option で成功 | `valid` | `success` | `lease-expired` formal metadata を残しつつ later success | あり | E6 補完, `0078` |
| `e8-monotone-degradation-reject` | `admit-miss`、middle failure、final `Reject` | `valid` | `Reject` | `perform-failure`、`Reject`、formal `admit-miss`、capability mismatch narrative | あり | canonical law / no re-promotion, `0078` |
| `e9-monotone-degradation-success` | `admit-miss`、middle failure、later success | `valid` | `success` | `perform-failure` の後に later write-capable option で `perform-success`、formal `admit-miss` のみを残す | あり | E3 / E6 success-side 補完, `0121` |
| `e10-perform-on-ensure-failure` | direct `PerformOn` の request-local `ensure` failure | `valid` | `explicit_failure` | `perform-failure` のみ、formal metadata なし、success-side carrier を preview しても commit しない | あり | E1 direct-target ensure variant, `0126` |
| `e11-perform-via-ensure-then-success` | via-chain の request-local `ensure` failure から later success へ継続 | `valid` | `success` | first option で `perform-failure`、later same-lineage option で `perform-success`、formal metadata なし | あり | E3 via ensure continuation variant, `0128` |

## fixture ごとの補足

### `e1-place-atomic-cut`

- `atomic_cut` の後に failure が来ても pre-cut prefix を rollback しない
- rollback frontier の読みを固定する fixture

### `e2-try-fallback`

- `try` body の失敗を local rollback + explicit fallback へ変換する
- current `place` local rollback の読みを固定する fixture

### `e21-try-atomic-cut-frontier`

- `try` body の途中にある `atomic_cut` が active rollback frame の frontier を更新する
- current implementation では whole `place_store` snapshot を restore するが、この fixture では post-cut snapshot への frontier update が `profile_draft` mutation にだけ現れる
- fallback branch 選択や chain order 自体は変えない

### `e22-try-atomic-cut-place-mismatch`

- nested place 内の `AtomicCut` は event を残してよい
- ただし `place_anchor != current_place` なら frontier update は起きず、rollback は try-entry snapshot へ戻る
- `e21` と対で読むことで、frontier update gate と restore scope を checker floor に混ぜない current judgmentを支える

### `e3-option-admit-chain`

- option-local `admit` miss は explicit failure でも `Reject` でもなく non-admissible skip
- later admissible option で success できることを固定する fixture

### `e4-malformed-lineage`

- malformed fallback branch は dynamic `Reject` ではなく static rejection
- explanatory `reasons` は残しつつ、actual static gate wording を `checked_reasons` へ narrow transfer する最初の採用例

### `e23-malformed-try-fallback-missing-fallback-body`

- empty `fallback_body` は generic malformed text ではなく、stage 2 `try` / rollback structural floor の dedicated malformed cluster として読む
- source-sample corpus initial cluster に含めるのは、representative / fixture / source mapping を stage 2 reconnect evidence と同じ stem で揃えるためである

### `e5-underdeclared-lineage`

- static evidence floor を満たさない branch は underdeclared で止める
- explanatory `reasons` と actual wording を分ける `checked_reasons` bridge の最初の採用例

### `e12-underdeclared-target-missing`

- same-lineage continuation には declared access target が両端に必要である
- lineage 証拠不足とは別の underdeclared cluster を static gate / detached loop / `checked_reasons` bridge に通す fixture

### `e13-malformed-capability-strengthening`

- same-lineage edge で successor capability が predecessor より強くなってはならない
- malformed lineage mismatch とは別の malformed cluster を static gate / detached loop / `checked_reasons` bridge に通す fixture

### `e14-malformed-duplicate-option-declaration`

- visible option 名の duplicate は hidden shadowing ではなく malformed static stop として扱う
- duplicate cluster は actual corpus に入れてよいが、current helper cut では `checked_reasons` と detached `reason_codes` の stable cluster には上げない

### `e15-malformed-duplicate-chain-declaration`

- visible chain 名の duplicate は competing head selection に落とさず malformed static stop として扱う
- duplicate cluster は actual corpus に入れてよいが、current helper cut では `checked_reasons` と detached `reason_codes` の stable cluster には上げない

### `e16-malformed-missing-chain-head-option`

- visible chain head は declared option を指していなければならない
- missing chain head option は stable malformed cluster として actual corpus / `checked_reasons` / detached `reason_codes` に通してよい

### `e17-malformed-missing-predecessor-option`

- edge predecessor が visible option declaration を指していない chain は malformed static stop である
- missing predecessor option は lineage annotation の有無にかかわらず stable malformed cluster として actual corpus / `checked_reasons` / detached `reason_codes` に通してよい

### `e18-malformed-missing-successor-option`

- edge successor が visible option declaration を指していない chain は malformed static stop である
- missing successor option は hidden later fallback 候補へ repair せず、stable malformed cluster として actual corpus / `checked_reasons` / detached `reason_codes` に通してよい

### `e19-malformed-target-mismatch`

- same-lineage edge の declared access target が一致しない chain は malformed static stop である
- declared target mismatch は stable malformed cluster として actual corpus / `checked_reasons` / detached `reason_codes` に通してよい

### `e20-malformed-late-capability-strengthening`

- same-lineage chain の earlier edge が read-to-read でも、later edge で read-to-write escalation を入れると capability strengthening として malformed static stop である
- capability floor を singleton fixture から外し、helper-local checker spike を `e13` と対で smoke できる actual corpus として扱ってよい

### `e6-write-after-expiry`

- `lease-expired` 後に write-admissible option が残らないと `Reject`
- capability mismatch は narrative explanation で足りる

### `e7-write-fallback-after-expiry`

- `e6` の補完
- expiry があっても later write-capable option があり、その option 自体が admissible で request を満たせるなら success しうる

### `e8-monotone-degradation-reject`

- `e3` / `e6` の補完
- monotone degradation と no re-promotion を `Reject` まで含めて固定する

### `e9-monotone-degradation-success`

- `e8` の success-side 補完
- middle option の explicit failure があっても、later same-lineage write-capable option が request を満たせば success しうる
- no re-promotion を保ったまま、middle failure のあとに later success へ進めることを固定する

### `e10-perform-on-ensure-failure`

- `PerformOn` の direct target で request-local `ensure` が unsatisfied なら `explicit_failure`
- effect が success-side carrier を返していても tentative commit は適用しない
- この分岐は `Reject` や non-admissible skip ではなく、request contract failure として読む

### `e11-perform-via-ensure-then-success`

- via-chain の earlier option が request-local `ensure` で失敗しても、later same-lineage option が残っていれば chain evaluation を継続する
- ensure failure では tentative commit を破棄し、formal non-admissible metadata を fabricated しない
- later option が `require` と `ensure` を満たせば final outcome は `success` になりうる

## runtime fixture と static-only fixture の区別

| 種別 | 条件 | sidecar |
|---|---|---|
| runtime fixture | `expected_runtime.enters_evaluation = true` | 必須 |
| static-only fixture | `expected_runtime.enters_evaluation = false` | 不要 |

## trace / audit expectation の現時点の方針

### machine-check するもの

- event kinds
- formal non-admissible metadata
- short narrative explanations

### prose に残すもの

- `must_explain`
- static verdict reason の長文
- fallback drift の説明

## 関連文書の読み順

1. representative examples の prose: `specs/examples/00-representative-mir-programs.md`
2. fixture schema: `specs/examples/02-current-l2-ast-fixture-schema.md`
3. step semantics: `specs/examples/04-current-l2-step-semantics.md`
4. regression fixture 追加経緯: `docs/reports/0078-current-l2-fallback-lease-regression-fixtures.md`
