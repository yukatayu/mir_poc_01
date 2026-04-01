# examples/04 — current L2 step semantics

この文書は、current L2 の representative examples、AST fixture schema、evaluation state schema を前提に、**parser-free 最小 interpreter**がどの粒度で状態遷移すればよいかを整理する補助文書である。
ここで定めるのは full interpreter 実装ではなく、`specs/examples/03-current-l2-evaluation-state-schema.md` にある最小 state carrier を、1-step 単位でどう進めれば E1 / E2 / E3 variant / E6 を読めるかという current L2 の companion semantics である。

## この文書の位置づけ

- parser なし最小 interpreter が持つべき step 粒度を定める。
- AST fixture schema と evaluation state schema の間を、「どの node をどう 1 step で進めるか」という規則で接続する。
- E1 / E2 / E3 variant / E6 の walkthrough を与え、最小 state が実際に足りることを確認する。
- predicate / effect oracle がどこで呼ばれるかという責務境界は `specs/examples/05-current-l2-oracle-api.md` を参照する。

## ここで固定すること / しないこと

- 固定すること:
  - Program / PlaceBlock / PerformOn / PerformVia / TryFallback / AtomicCut の最小 1-step 規則
  - `cursor_stack`、`place_stack`、`place_store`、`current_request`、`chain_cursor`、`rollback_stack`、`trace_audit_sink`、`terminal_outcome` の更新タイミング
  - static gate が runtime state の外にあること
- 固定しないこと:
  - predicate / effect oracle の richer API
  - `place_store` や snapshot ref の concrete layout
  - detached trace / audit serialization
  - multi-request scheduler
  - full interpreter の最終 object model

## 実行前段の gate

runtime の 1-step semantics に入る前に、fixture は static gate を通る。

1. `expected_static.verdict = valid` の fixtureだけが evaluation state を初期化して実行へ入る。
2. `malformed` / `underdeclared` は runtime state を作らずに停止する。
3. したがって `not_evaluated` は runtime state の terminal value ではなく、static gate で弾かれた fixture expectation 側の結果である。

## 1-step の最小単位

current L2 の parser-free 最小 interpreter では、1 回の `step` は「現在の cursor が指す node を 1 段だけ進め、必要なら state component を更新し、meta-level control result を返すもの」と読む。

この meta-level control result は evaluation state の field ではなく、文書上の補助概念に留める。

- `Continue`
  - runtime evaluation を続ける
- `BubbleFailure(kind)`
  - current node が失敗を親文脈へ返す
- `Halt`
  - `terminal_outcome` が確定し、評価を止める

`kind` に current L2 で最低限必要なのは次である。

- `explicit_failure`
- `Reject`

`Approximate` / `Compensate` は Mir-0 の failure space に残るが、E1 / E2 / E3 variant / E6 を動かす最小 step semantics では必須でないため、ここでは固定しない。

## state component 更新の全体像

| component | いつ更新するか |
|---|---|
| `cursor_stack` | block へ入る / block を抜ける / 同一 block 内で次 statement へ進むとき |
| `place_stack` | `PlaceBlock` へ入るとき push、抜けるとき pop |
| `place_store` | `PerformOn` / admissible な `PerformVia` が success-side carrier を得て、request-local `ensure` まで通過したときに mutate、rollback で restore、`AtomicCut` で snapshot を読む |
| `current_request` | `PerformOn` / `PerformVia` の開始時に install、終了時に clear |
| `chain_cursor` | `PerformVia` の開始時に生成、option miss / failure / success ごとに更新し、終了時に破棄 |
| `rollback_stack` | `TryFallback` 開始時に push、body failure 時に top frame を参照して rollback、`AtomicCut` で top frame を更新、終了時に pop |
| `trace_audit_sink` | success / failure / rollback / `atomic-cut` / non-admissible metadata / narrative explanation が生じた直後に追記 |
| `terminal_outcome` | unhandled `explicit_failure` / `Reject`、または Program 正常完了時に確定 |

## node ごとの最小 1-step semantics

### 1. `Program`

`Program` は top-level statement list を持つ root node である。

1. 初期化 step
   - evaluation 開始時、`cursor_stack` が空なら `Program.body` を指す root frame を push する。
   - `terminal_outcome` はまだ空のままにする。
2. 前進 step
   - root frame が未消費 statement を持つなら、その statement を次の step の対象にする。
3. 完了 step
   - root frame が尽き、かつ unhandled な `BubbleFailure` が無いなら `terminal_outcome = success` として `Halt` する。
   - unhandled な `BubbleFailure(kind)` が root まで届いた場合は、その `kind` を `terminal_outcome` に写して `Halt` する。

### 2. `PlaceBlock`

`PlaceBlock` は current `place` の入れ子を作る構造 node である。

1. enter step
   - 対象 node が `PlaceBlock(place = p, body = ...)` のとき、`place_stack.push(p)` する。
   - あわせて `body` を走査する child frame を `cursor_stack.push(...)` する。
   - result は `Continue`。
2. exit step
   - child frame が尽きたら、その frame を `cursor_stack` から pop し、対応する `place` を `place_stack` から pop する。
   - 親 frame の statement cursor を次へ進める。
   - result は `Continue`。

`PlaceBlock` 自体は `place_store` を直接 mutate しない。mutation は内側の `Perform*` / rollback step が担う。

### 3. `OptionDecl`

`OptionDecl` は current L2 では declaration-only node であり、runtime state を mutate しない。

1. declaration step
   - node 自体は immutable fixture carrier に残り続ける。
   - current step では `place_store`、`current_request`、`chain_cursor`、`rollback_stack` を更新しない。
   - current cursor だけを次 statement へ進める。
   - result は `Continue`。

`OptionDecl` を runtime no-op として扱ってよいのは、`PerformVia` が必要になった時点で chain / option 情報を immutable fixture carrier から解決すれば足りるためである。

### 4. `ChainDecl`

`ChainDecl` も current L2 では declaration-only node である。

1. declaration step
   - node 自体は immutable fixture carrier に残り続ける。
   - current step では runtime state を mutate しない。
   - current cursor だけを次 statement へ進める。
   - result は `Continue`。

`PerformVia` は `chain_ref` を受け取った時点で、current `place` から見える `ChainDecl` と、それが指す `OptionDecl` 群を immutable fixture carrier から解決し、その結果を `chain_cursor.candidate_order` に入れてよい。

### 5. `PerformOn`

`PerformOn` は direct target に対する request である。

1. request install step
   - `current_request = { op, mode = on, target, contract }` を install する。
2. request-local predicate step
   - request-local `require` を predicate oracle に委ねて評価する。
   - `require` が不成立なら `trace_audit_sink.events` に `perform-failure` を追記し、`current_request` を clear して `BubbleFailure(explicit_failure)` を返す。
3. effect attempt step
   - `require` が通ったら、target operation を effect oracle に委ねて評価する。
   - effect oracle は最小でも success-side carrier か `explicit_failure` のいずれかを返せばよい。
4. success commit step
   - effect oracle が success-side carrier を返したら、その carrier から読める tentative post-state を使って request-local `ensure` を predicate oracle に委ねて評価する。
   - `ensure` が通った場合にだけ success-side carrier を `place_store` へ反映し、`trace_audit_sink.events` に `perform-success` を追記する。
   - `current_request` を clear し、同一 block の cursor を次 statement へ進める。
   - result は `Continue`。
5. failure branch
   - effect oracle が `explicit_failure` を返した場合、または `ensure` が不成立だった場合は、`trace_audit_sink.events` に `perform-failure` を追記する。
   - `current_request` を clear する。
   - enclosing `TryFallback` がいれば `BubbleFailure(kind)` を返し、いなければ `terminal_outcome = kind` として `Halt` する。

current representative set では、`PerformOn` の failure branch に request-level `Reject` を要求しない。direct target に対する `Reject` を最小 oracle carrier に入れる必要があるかは、current L2 では未決定である。

### 6. `PerformVia`

`PerformVia` は canonical option chain を left-to-right に辿る request である。

1. request / chain install step
   - `current_request = { op, mode = via, chain_ref, contract }` を install する。
   - current `place` から見える immutable fixture carrier を参照し、`chain_ref` に対応する `ChainDecl` とその `OptionDecl` 群から canonical option order を解決する。
   - `chain_cursor = { chain_ref, candidate_order, next_index = 0, current_option_ref = head }` を生成する。
2. option inspection step
   - `current_option_ref` が指す option を見て、次の順で判定する。
   - `lease` が失効していれば、`trace_audit_sink.non_admissible_metadata` に `{ option_ref, subreason = lease-expired }` を追記し、`chain_cursor` を次候補へ進める。
   - option-local `admit` が不成立なら、`trace_audit_sink.non_admissible_metadata` に `{ option_ref, subreason = admit-miss }` を追記し、`chain_cursor` を次候補へ進める。
   - request-local `require` と option capability が両立しないなら、formal subreason は増やさず `trace_audit_sink.narrative_explanations` に capability mismatch explanation を追記し、`chain_cursor` を次候補へ進める。
3. admitted option request predicate step
   - option が admissible かつ request-compatible なら、その option 上で request-local `require` を predicate oracle に委ねて評価する。
   - `require` が不成立なら `trace_audit_sink.events` に `perform-failure` を追記する。
   - 後続 option が残っていれば `chain_cursor` を次候補へ進め、同じ `PerformVia` を継続する。
   - 後続 option が残っていなければ、`trace_audit_sink.events` に `Reject` を追記し、`current_request` と `chain_cursor` を clear して `BubbleFailure(Reject)` を返す。enclosing `TryFallback` が無ければ `terminal_outcome = Reject` で `Halt` する。
4. admitted option success step
   - request-local `require` が通った option に対して operation を試し、effect oracle から success-side carrier または `explicit_failure` を受け取る。
   - success-side carrier を得たら、その carrier から読める tentative post-state を使って request-local `ensure` を predicate oracle に委ねて評価する。
   - `ensure` が通った場合にだけ `place_store` を更新し、`trace_audit_sink.events` に `perform-success` を追記し、`current_request` と `chain_cursor` を clear して親 cursor を次 statement へ進める。
   - result は `Continue`。
5. admissible option explicit failure step
   - admitted option の operation が `explicit_failure` を返した場合、または admitted option success 後に `ensure` が不成立だった場合は、`trace_audit_sink.events` に `perform-failure` を追記する。
   - 後続 option が残っていれば `chain_cursor` を次候補へ進め、同じ `PerformVia` を継続する。
   - 後続 option が残っていなければ、`trace_audit_sink.events` に `Reject` を追記し、`current_request` と `chain_cursor` を clear して `BubbleFailure(Reject)` を返す。enclosing `TryFallback` が無ければ `terminal_outcome = Reject` で `Halt` する。
6. chain exhaustion step
   - success を返す admissible candidateが 1 つも見つからず `candidate_order` が尽きた場合、`current_request` と `chain_cursor` を clear する。
   - その exhaustion が non-admissible skip / mismatch だけで起きた場合でも、途中に request-local `require` 不成立や admitted option の `explicit_failure` が含まれていた場合でも、current L2 では「well-formed chain が success を返さずに尽きた」という request-level outcome として `trace_audit_sink.events` に `Reject` を追記したうえで、result は `BubbleFailure(Reject)` である。enclosing `TryFallback` が無ければ `terminal_outcome = Reject` として `Halt` する。

この規則により、`admit` miss は non-admissible skip、`lease` expiry も non-admissible skip、capability mismatch は narrative explanation に留まり、`ensure` は semantically dead にならず、event surface は request-level outcome のまま保たれる。

### 7. `TryFallback`

`TryFallback` は local rollback region を作り、body failure を fallback body へ変換する node である。

1. enter step
   - current `place` の snapshot を `place_store` から取得する。
   - `rollback_stack.push({ place_anchor, restore_snapshot_ref = entry_snapshot, fallback_cursor_ref })` する。
   - `cursor_stack` の current branch を `body` 側に切り替える。
   - result は `Continue`。
2. body success step
   - body が `BubbleFailure` を出さずに尽きたら、top rollback frame を pop し、親 cursor を `TryFallback` の次 statement へ進める。
   - result は `Continue`。
3. body failure catch step
   - body から `BubbleFailure(kind)` が返ったら、その `kind` はまだ `terminal_outcome` に写さない。
   - `place_store` を top frame の `restore_snapshot_ref` まで戻す。
   - `trace_audit_sink.events` に `rollback` を追記する。
   - `cursor_stack` の current branch を `fallback_body` 側へ切り替える。
   - result は `Continue`。
4. fallback body success step
   - fallback body が成功したら top rollback frame を pop し、親 cursor を `TryFallback` の次 statement へ進める。
   - result は `Continue`。
5. fallback body failure rethrow step
   - fallback body から `BubbleFailure(kind)` が返ったら top rollback frame を pop し、その `kind` を親文脈へ再送する。
   - enclosing `TryFallback` が無ければ `terminal_outcome = kind` として `Halt` する。

### 8. `AtomicCut`

`AtomicCut` は place-local finalizing boundary である。

1. always step
   - `trace_audit_sink.events` に `atomic-cut` を追記する。
2. rollback frame update step
   - `rollback_stack` の top frame が存在し、かつその `place_anchor` が current `place` と一致するなら、top frame の `restore_snapshot_ref` を current `place_store` snapshot へ更新する。
   - これが current L2 における `atomic_cut frontier` の最小 carrier である。
3. no-frame step
   - active rollback frame が無い場合、`AtomicCut` は `trace_audit_sink` だけ更新し、専用 frontier state は新設しない。
4. exit
   - current cursor を次 statement へ進める。
   - result は `Continue`。

## E1 / E2 / E3 variant / E6 walkthrough

### E1 — place 入れ子 + `atomic_cut`

1. static gate が `valid` を返す。
2. `Program` が root frame を push する。
3. `PlaceBlock(root)`、`PlaceBlock(session)`、`PlaceBlock(authority_cell)` が順に `place_stack` を push する。
4. `PerformOn(update_authority)` が `current_request` を install し、success する。
   - effect oracle が success-side carrier を返し、`ensure owner_is(session_user)` が通る。
   - その後にだけ `place_store` が mutate される。
   - `trace_audit_sink.events += perform-success`
   - `current_request` を clear
5. `AtomicCut` が走る。
   - active rollback frame は無いので `rollback_stack` は更新しない。
   - `trace_audit_sink.events += atomic-cut`
6. `PerformOn(append_audit)` が `current_request` を install し、explicit failure する。
   - `trace_audit_sink.events += perform-failure`
   - `current_request` を clear
   - enclosing `TryFallback` が無いので `terminal_outcome = explicit_failure`
7. `Halt`

この例で重要なのは、`atomic_cut` が rollback frame を更新しないことではなく、active rollback frame が無いなら frontier carrier を増やさず event / audit 説明だけで足りる、という点である。

### E2 — local `try` + `fallback`

1. static gate が `valid` を返す。
2. `Program` と 3 段の `PlaceBlock` を enter する。
3. `TryFallback` enter:
   - `draft_profile` の entry snapshot を取り、top rollback frame を push
4. `PerformOn(stage_profile_patch)` success:
   - success-side carrier が `ensure` 空のまま commit され、`place_store` を mutate
   - `trace_audit_sink.events += perform-success`
5. `PerformOn(validate_profile_patch)` explicit failure:
   - `trace_audit_sink.events += perform-failure`
   - `BubbleFailure(explicit_failure)`
6. `TryFallback` catch:
   - `place_store` を top frame の `restore_snapshot_ref` へ戻す
   - `trace_audit_sink.events += rollback`
   - `fallback_body` へ切り替える
7. `PerformOn(load_last_snapshot)` success:
   - `trace_audit_sink.events += perform-success`
8. `TryFallback` を success で抜けて rollback frame を pop
9. Program が尽き、`terminal_outcome = success`

current representative set には `try` body 内の `atomic_cut` 例は入っていないが、同じ規則の下では、もし body 内で `AtomicCut` を踏めば top rollback frame の `restore_snapshot_ref` が post-cut snapshot に更新され、以後の rollback はそこまでしか戻らない。

### E3 variant — option-local `admit`

1. static gate が `valid` を返す。
2. `Program` と place 入れ子を enter し、`OptionDecl` / `ChainDecl` は state mutation なしに cursor を前進する。
3. `PerformVia(write_profile via profile_ref)` enter:
   - `current_request` を install
   - `chain_cursor = owner_writer` で開始
4. `owner_writer` inspection:
   - `admit owner_is(session_user)` が不成立
   - `trace_audit_sink.non_admissible_metadata += { option_ref = owner_writer, subreason = admit-miss }`
   - `chain_cursor` を `delegated_writer` へ進める
5. `delegated_writer` inspection:
   - admissible かつ request-compatible
   - operation が success-side carrier を返し、`ensure` 空のまま commit される
   - `place_store` が mutate される
   - `trace_audit_sink.events += perform-success`
   - `current_request` と `chain_cursor` を clear
6. Program が尽き、`terminal_outcome = success`

この例で重要なのは、`admit` miss が explicit failure でも `Reject` でもなく、chain 継続を伴う non-admissible metadata としてだけ観測される点である。

### E6 — `lease` expiry と final `Reject`

1. static gate が `valid` を返す。
2. `PerformVia(write_profile via profile_ref)` enter:
   - `current_request` を install
   - `chain_cursor = writer`
3. `writer` inspection:
   - `lease = expired`
   - `trace_audit_sink.non_admissible_metadata += { option_ref = writer, subreason = lease-expired }`
   - `chain_cursor` を `readonly` へ進める
4. `readonly` inspection:
   - option 自体は live だが request-local `require write` と capability `read` が両立しない
   - `trace_audit_sink.narrative_explanations += "readonly is request/capability mismatch"`
   - `chain_cursor` を exhaust する
5. chain exhaustion:
   - admissible かつ request-compatible な candidate が尽きたので `trace_audit_sink.events += Reject`
   - `BubbleFailure(Reject)`
   - enclosing `TryFallback` が無いため `terminal_outcome = Reject`
6. `Halt`

この例で、`lease` expiry は formal subreason、capability mismatch は narrative explanation、最終 outcome は request-level `Reject` のまま保たれる。

## fixture expectation との対応

- `expected_static`
  - step semantics に入る前段 gate で消費する。
- `expected_runtime`
  - `terminal_outcome` が何で `Halt` したかを照合する。
- `expected_trace_audit`
  - `trace_audit_sink.events`
  - `trace_audit_sink.non_admissible_metadata`
  - `trace_audit_sink.narrative_explanations`
  の抽象 shape を照合する。

## current L2 に残し、ここで決めないこと

- predicate / effect oracle の richer API、host integration、success-side carrier の concrete layout は **未決定**。
- direct target に対する request-level `Reject` を将来 oracle carrier に入れる必要があるかは **未決定**。
- `place_store` snapshot の concrete representation は **未決定**。
- `cursor_stack` / `chain_cursor` / `rollback_stack` の最終 field naming は **未決定**。
- detached trace / audit serialization と event id は **未決定**。
- multi-request scheduler は **未決定**。
- `Approximate` / `Compensate` を parser-free 最小 interpreter の step signal にどう織り込むかは、current representative set には不要なので **未決定**。

current L2 で固定するのは、representative examples を parser なしで動かすための最小遷移規則だけである。
