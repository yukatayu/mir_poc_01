# examples/05 — current L2 oracle API

この文書は、current L2 の representative examples、AST fixture schema、evaluation state schema、step semantics を前提に、**parser-free 最小 interpreter**が predicate / effect oracle に何を渡し、何を受け取ればよいかを整理する補助文書である。
ここで定めるのは full interpreter の host interface ではなく、`PerformOn` / `PerformVia` を E1 / E2 / E3 variant / E6 の範囲で動かすための最小 companion API である。

## この文書の位置づけ

- parser なし最小 interpreter が oracle に委ねる責務境界を定める。
- `specs/examples/04-current-l2-step-semantics.md` にある `Continue` / `BubbleFailure(kind)` / `Halt` と、oracle の戻り値を接続する。
- representative examples を動かす上で、どこまでを interpreter 側の structural rule に残し、どこからを oracle に送るかを明示する。

## ここで固定すること / しないこと

- 固定すること:
  - current L2 では predicate evaluator と effect executor を分ける
  - option-local non-admissible skip は oracle の failure carrier に入れない
  - `Reject` は request-level outcome に留め、oracle の local verdict に入れない
  - effect success は success-side carrier を返し、`ensure` 通過後にだけ `place_store` へ反映する
- 固定しないこと:
  - concrete trait 名、field 名、serialization
  - predicate evaluator の richer API
  - effect oracle の host binding / host capability surface
  - `Approximate` / `Compensate` を oracle carrier に入れるかどうか

## 最小分割

current L2 の parser-free 最小 interpreter では、oracle を 1 つにまとめず、次の 2 つに分ける方が小さく整合的である。

1. `PredicateOracle`
   - request-local `require`
   - request-local `ensure`
   - option-local `admit`
   を評価する。
2. `EffectOracle`
   - admitted な request / option の effect attempt を実行する。

### unified oracle にしない理由

unified oracle にまとめると、少なくとも次の 3 種を同じ戻り値空間で区別しなければならない。

- option-local non-admissible skip
- admitted request の explicit failure
- request-level `Reject`

E3 variant と E6 では、`admit` miss / `lease` expiry / capability mismatch の段階では effect attempt 自体が起きないため、これらを effect outcome と同じ carrier に押し込むと、current L2 が維持している

- event surface を増やさない
- non-admissible skip を metadata に留める
- `Reject` を request-level outcome に留める

という境界がぼやける。

そのため current L2 では、

- interpreter 側:
  - chain resolution
  - `lease` 判定
  - capability mismatch 判定
  - non-admissible metadata / narrative explanation の記録
- `PredicateOracle`:
  - `admit` / `require` / `ensure` の真偽評価
- `EffectOracle`:
  - admitted request の effect attempt

と分ける。

## interpreter 側に残す structural rule

次は current L2 では oracle に送らず、step semantics 側で処理する。

- `ChainDecl` / `OptionDecl` から canonical option order を解決すること
- `lease = expired` を `lease-expired` metadata に落とすこと
- request-local `require` と option capability が両立しない場合に capability mismatch を narrative explanation として残すこと
- option miss が起きたときに `chain_cursor` を次候補へ進めること
- chain exhaustion から request-level `Reject` を導くこと

これにより、`admit-miss` / `lease-expired` は formal subreason として残しつつ、capability mismatch は narrative explanation に留める current L2 読みをそのまま保てる。

## oracle に渡す最小 input

field 名は未固定だが、conceptual shape として最低限必要なのは次である。

### 1. `PredicateOracle` 用 input

`PredicateOracle` は、少なくとも次の情報を見られればよい。

- `site`
  - `request-require`
  - `request-ensure`
  - `option-admit`
- current `place`
- request-local view
  - `op`
  - `mode` (`on` / `via`)
  - direct target または `chain_ref`
  - request-local contract object
- option-local view（`option-admit` のときだけ）
  - `option_ref`
  - `target`
  - `capability`
  - `lease`
- predicate tree
- current `place_store` への read-only view
- success-side carrier または tentative post-state view（`request-ensure` のときだけ）

ここで重要なのは、`PredicateOracle` が punctuation や parser token を見ないことである。
受け取るのは surface 文字列ではなく、fixture schema が正規化した predicate tree と request / option の意味構造だけでよい。
`request-ensure` の場合だけは、effect success をどう post-state として読むかを伝えるために、success-side carrier または tentative post-state view を追加で見られなければならない。

### 2. `EffectOracle` 用 input

`EffectOracle` は、少なくとも次の情報を見られればよい。

- current `place`
- request-local view
  - `op`
  - `mode`
  - direct target または `chain_ref`
  - request-local contract object
- selected target
  - `PerformOn` なら direct target
  - `PerformVia` なら leftmost admitted option が指す target
- selected option view（`PerformVia` のときだけ）
  - `option_ref`
  - `capability`
  - `lease`
- current `place_store` への host-facing access

current L2 では、`EffectOracle` に `admit-miss` / `lease-expired` / capability mismatch を直接返させない。
それらは effect attempt に入る前の structural rule として interpreter 側で処理する。

## oracle が返す最小 return carrier

### 1. `PredicateOracle` の最小 return carrier

`PredicateOracle` が返す最小 carrier は、真偽だけでよい。

- `Satisfied`
- `Unsatisfied`

current L2 では、`PredicateOracle` 自体に `explicit_failure` / `Reject` / skip reason を返させない。
それらへの写像は site ごとに step semantics 側で行う。

### 2. `EffectOracle` の最小 return carrier

`EffectOracle` が返す最小 carrier は、次の二通りで足りる。

- `Success(success_side_carrier)`
- `ExplicitFailure`

ここで `success_side_carrier` は final field 名ではないが、最低限「effect attempt が成功したときに、`ensure` 通過後の `place_store` 反映へ渡せる opaque carrier」を意味する。

current L2 では、

- `place_store` を oracle 側で即時 mutate するか
- oracle が store delta / host token / commit witness を返すか

の concrete layout は固定しない。
ただし、**可視な `place_store` 更新は `ensure` 通過後にだけ起こる**、という順序だけは固定する。

## step semantics への写像

### `PredicateOracle`

| site | oracle verdict | step 側の処理 | control result |
|---|---|---|---|
| `option-admit` | `Satisfied` | option inspection を継続 | `Continue` |
| `option-admit` | `Unsatisfied` | `non_admissible_metadata += { option_ref, subreason = admit-miss }`、`chain_cursor` を前進 | `Continue` |
| `request-require` (`PerformOn`) | `Satisfied` | effect attempt へ進む | `Continue` |
| `request-require` (`PerformOn`) | `Unsatisfied` | `perform-failure` を記録し `current_request` を clear | `BubbleFailure(explicit_failure)` |
| `request-require` (`PerformVia`) | `Satisfied` | admitted option の effect attempt へ進む | `Continue` |
| `request-require` (`PerformVia`) | `Unsatisfied` | `perform-failure` を記録し `current_request` と `chain_cursor` を clear | `BubbleFailure(explicit_failure)` |
| `request-ensure` | `Satisfied` | success-side carrier を `place_store` へ反映し `perform-success` を記録 | `Continue` |
| `request-ensure` | `Unsatisfied` | success-side carrier を破棄し `perform-failure` を記録 | `BubbleFailure(explicit_failure)` |

`request-ensure` の不成立を `explicit_failure` に寄せることで、`ensure` は semantically dead にならず、しかも request-level `Reject` や non-admissible skip と混ざらない。

### `EffectOracle`

| node / context | oracle verdict | step 側の処理 | control result |
|---|---|---|---|
| `PerformOn` | `Success(success_side_carrier)` | `ensure` 評価へ進む | `Continue` |
| `PerformOn` | `ExplicitFailure` | `perform-failure` を記録し `current_request` を clear | `BubbleFailure(explicit_failure)` |
| `PerformVia` admitted option | `Success(success_side_carrier)` | `ensure` 評価へ進む | `Continue` |
| `PerformVia` admitted option | `ExplicitFailure` + next option あり | `perform-failure` を記録し `chain_cursor` を前進 | `Continue` |
| `PerformVia` admitted option | `ExplicitFailure` + next option なし | `perform-failure` を記録し `current_request` と `chain_cursor` を clear | `BubbleFailure(explicit_failure)` |

### interpreter 側だけで生じる branch

| branch | step 側の処理 | control result |
|---|---|---|
| `lease` expiry | `non_admissible_metadata += { option_ref, subreason = lease-expired }`、`chain_cursor` を前進 | `Continue` |
| capability mismatch | `narrative_explanations += ...`、`chain_cursor` を前進 | `Continue` |
| non-admissible skip だけで chain exhaustion | `Reject` を記録し `current_request` と `chain_cursor` を clear | `BubbleFailure(Reject)` |
| direct target で request-level `Reject` を返す必要が生じる将来拡張 | current L2 では未採用 | **未決定** |

## representative examples での読み

### E1

- `PerformOn(update_authority)` は
  1. `request-require` を `PredicateOracle` で評価
  2. `EffectOracle` が success-side carrier を返す
  3. `request-ensure` を `PredicateOracle` で評価
  4. `Satisfied` なら `place_store` に反映
という順で読めば足りる。

これにより、`ensure owner_is(session_user)` は semantically dead にならず、`atomic_cut` は commit 済みの state に対してだけ frontier を与える。

### E2

- `stage_profile_patch` は success-side carrier がそのまま commit される。
- `validate_profile_patch` は current representative set では `request-require` 不成立として読めば十分であり、`BubbleFailure(explicit_failure)` を返す。
- `TryFallback` はその `BubbleFailure(explicit_failure)` を catch して rollback / fallback へ変換する。

この例は、predicate failure と effect failure を同じ `explicit_failure` family に残しても、rollback semantics が壊れないことを示す。

### E3 variant

- `owner_writer` は `option-admit` を `PredicateOracle` で評価し、`Unsatisfied` なら `admit-miss` metadata だけを残して skip する。
- `delegated_writer` は admitted option なので `EffectOracle` まで進み、success-side carrier を `ensure` 空のまま commit する。

この例は、option-local non-admissible skip を effect oracle の戻り値に入れなくても十分であることを示す。

### E6

- `writer` は `lease-expired` なので oracle を呼ばずに skip する。
- `readonly` は capability mismatch なので oracle を呼ばずに narrative explanation を残して skip する。
- chain が尽きた時点でだけ `Reject` を立てる。

この例は、non-admissible reason family と request-level `Reject` を same oracle carrier に混ぜない方が current L2 では読みやすいことを示す。

## current L2 に残し、ここで決めないこと

- concrete trait 名、field 名、serialization は **未決定**。
- `success_side_carrier` の concrete layout、host への受け渡し方法は **未決定**。
- predicate evaluator に richer verdict や explanation payload を持たせるかは **未決定**。
- `EffectOracle` が `Approximate` / `Compensate` を返す必要があるかは **未決定**。
- direct target 用に request-level `Reject` を oracle carrier に入れる必要が将来あるかは **未決定**。
- multi-request scheduler と detached trace / audit serialization は **未決定**。

current L2 で固定するのは、representative examples を parser なしで動かすための最小 oracle boundary だけである。
