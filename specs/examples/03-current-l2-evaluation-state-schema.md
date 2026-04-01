# examples/03 — current L2 evaluation state schema

この文書は、current L2 の AST fixture を**parser なし最小 interpreter**で実行するために必要な、最小 evaluation state schema を整理する補助文書である。
ここで定めるのは full interpreter の実装詳細ではなく、`specs/examples/02-current-l2-ast-fixture-schema.md` にある fixture を評価するために、どの抽象状態を最低限持てばよいかという current L2 の companion schema である。

## この文書の位置づけ

- current L2 representative examples を parser なしで実行準備できる形に落とす。
- AST fixture schema と runtime semantics の間にある最小 state carrier を定める。
- `E1`、`E2`、`E3` 比較用 variant、`E6` を動かすのに必要な state 粒度だけを固定する。
- node ごとの進め方自体は `specs/examples/04-current-l2-step-semantics.md` に分離する。

## ここで固定すること / しないこと

- 固定すること:
  - parser なし最小 interpreter に必要な最小 state component
  - static gate と runtime state の責務分離
  - request-level trace / audit sink の最小保持先
- 固定しないこと:
  - in-memory field 名の最終固定
  - state serialization
  - concrete store layout
  - scheduler や concurrency model
  - full step semantics

## 実行 pipeline の最小読み

current L2 の parser-free 実行準備は、概念的に次の 3 段に分ければよい。

1. static gate
   - fixture の `program` を見て `valid` / `malformed` / `underdeclared` を判定する。
2. runtime evaluation
   - `valid` な fixture だけが evaluation state を初期化して実行へ入る。
3. expectation check
   - 実行後の terminal outcome と trace / audit sink を fixture の `expected_runtime` / `expected_trace_audit` と照合する。

したがって `expected_static` は runtime state の一部ではなく、evaluation state へ入る前の gate で消費される。

## 最小 evaluation state

current L2 の parser-free 最小 interpreter は、概念的には次の 8 要素だけ持てばよい。

| component | 役割 |
|---|---|
| `cursor_stack` | どの block / statement を今評価しているか |
| `place_stack` | current `place` とその入れ子 |
| `place_store` | place-local state と linear resource の抽象 carrier |
| `current_request` | 今評価中の `perform` request |
| `chain_cursor` | `perform ... via` の chain evaluation 状態 |
| `rollback_stack` | active な local rollback region。存在しないときは空でよい |
| `trace_audit_sink` | request-level trace / audit の蓄積先 |
| `terminal_outcome` | 実行終了時の outcome |

### 1. `cursor_stack`

`cursor_stack` は、最低限次を持てばよい。

- 今いる block node の参照
- その block の次に評価する statement index
- `TryFallback` の場合は現在が `body` か `fallback_body` か

これは parser token ではなく、AST node path を辿る cursor として持てばよい。

### 2. `place_stack`

`place_stack` は、入れ子になった `PlaceBlock` の path を保持する。

- top が current `place`
- 各 frame は `place` 名だけでもよい
- 追加で ambient metadata を持つ場合も、current L2 では未固定

これにより、same-place local rollback と current `place` の `atomic_cut` frontier を区別する anchor が得られる。

### 3. `place_store`

current L2 では、rollback と `atomic_cut` を説明するために、place-local state の抽象 carrier が必要である。

- `place_store` は opaque でよい
- 少なくとも snapshot を取れること
- 少なくとも current `place` ごとに state fragment を辿れること

値表現や ownership cell の具体 layout は current L2 では固定しない。

### 4. `current_request`

`current_request` は `PerformOn` / `PerformVia` を評価するときだけ non-null になる request frame である。

最低限必要なのは次である。

- `op`
- access mode
  - `on`
  - `via`
- direct target または chain ref
- request-local contract
  - `require`
  - `ensure`

current L2 では `require` / `ensure` は surface clause ではなく semantic role object として持てばよい。

### 5. `chain_cursor`

`chain_cursor` は `PerformVia` のときだけ必要になる。

最低限必要なのは次である。

- `chain_ref`
- canonical option order
- 次に見る option index
- 今見ている option ref

current L2 では、admissibility miss を dedicated event にせず metadata に留めるので、`chain_cursor` は option evaluation の途中結果を trace sink に流せれば十分であり、複雑な backtracking state は要らない。
`canonical option order` は、current L2 では mutable runtime state ではなく、current request から参照される `ChainDecl` と `OptionDecl` を immutable な fixture carrier から引いて初期化すれば足りる。

### 6. `rollback_stack`

`rollback_stack` は local `try` を評価するときだけ active frame を持つ。

各 rollback frame は、最低限次を持てばよい。

- `place_anchor`
  - どの `place` に局所な rollback か
- `restore_snapshot_ref`
  - 現時点で rollback が戻る先
- `fallback_cursor_ref`
  - body 失敗後にどの fallback body を実行するか

### 7. `atomic_cut` frontier

current L2 では、`atomic_cut` frontier は global cut state にしない。
active な rollback frame がある場合に限り、最小 interpreter では `rollback_stack` 内の `restore_snapshot_ref` を更新することで表せば足りる。

- `try` に入った直後は `restore_snapshot_ref = entry snapshot`
- 同じ rollback region 内で `atomic_cut` を踏んだら、`restore_snapshot_ref` をその時点の snapshot に進める
- したがって rollback は、その時点の frontier を越えて戻れない
- active な rollback frame が無い場合、`atomic_cut` は local rollback frontier を更新する state carrierを新設せず、その request の trace / audit 説明だけで足りる

必要なら ambient place-level frontier を別に持つ拡張余地はあるが、E1 / E2 / E3 variant / E6 を動かす current L2 最小 schema には必須ではない。

### 8. `trace_audit_sink`

current L2 では event surface を増やしすぎないので、`trace_audit_sink` は request-level sink として持てばよい。

最低限必要なのは次である。

- event kind の列
- non-admissible metadata
  - `option_ref`
  - `subreason`
- narrative explanation

current L2 では `admit-miss` と `lease-expired` だけが formal subreason であり、capability mismatch は narrative explanation に留める。

### 9. `terminal_outcome`

runtime の最終結果は separate field として持てばよい。

- `success`
- `explicit_failure`
- `Reject`

`not_evaluated` は runtime state の内部値ではなく、static gate で runtime evaluation に入らなかった fixture expectation 側の結果として扱うのが自然である。

## fixture expectation と evaluation state の役割分担

### `expected_static`

- runtime state ではなく、evaluation 開始前の gate で消費する。
- `malformed` / `underdeclared` の fixture は、evaluation state を full に構築しなくてよい。

### `expected_runtime`

- 実行後の `terminal_outcome` と比較する oracle である。
- evaluation state 自体に expected value を持たせない。

### `expected_trace_audit`

- `trace_audit_sink` の最小抽象 shape と比較する oracle である。
- field 名や serialization の完全一致ではなく、
  - event kind
  - non-admissible metadata
  - must explain
  を満たすかで読む。

## representative examples ごとの必要 state 粒度

### E1 — `place` 入れ子 + `atomic_cut`

最低限必要なもの:

- `cursor_stack`
- `place_stack`
- `place_store`
- `current_request`
- `trace_audit_sink`
- `terminal_outcome`

E1 は `try` を使わないため `rollback_stack` は必須ではないが、`atomic_cut` の後に pre-cut update を rollback しないことを説明するため、少なくとも cut 後に hidden rollback が起きていないことを trace sink で説明できなければならない。
この例では active な rollback frame が無いため、`atomic_cut` frontier は独立 state としては持たず、local rollback を制約すべき場面自体が存在しないと読めば足りる。

### E2 — local `try` + `fallback`

最低限必要なもの:

- `cursor_stack`
- `place_stack`
- `place_store`
- `current_request`
- `rollback_stack`
- `trace_audit_sink`
- `terminal_outcome`

E2 では `rollback_stack` に `restore_snapshot_ref` が必要であり、validation failure 後にその snapshot へ戻してから fallback body を実行できなければならない。

### E3 variant — option-local `admit`

最低限必要なもの:

- `cursor_stack`
- `place_stack`
- `current_request`
- `chain_cursor`
- `trace_audit_sink`
- `terminal_outcome`

この例では `owner_writer` の `admit` miss を explicit failure にせず、`trace_audit_sink` の non-admissible metadata として残しつつ、`chain_cursor` を次の option へ進められればよい。

### E6 — `lease` expiry と final `Reject`

最低限必要なもの:

- `cursor_stack`
- `place_stack`
- `current_request`
- `chain_cursor`
- `trace_audit_sink`
- `terminal_outcome`

E6 では、

1. `writer` を `lease-expired` で non-admissible metadata に落とす
2. `readonly` を request / capability mismatch の narrative explanation として扱う
3. write-admissible candidate が尽きた時点で `terminal_outcome = Reject`

という順で読めれば足りる。

## current L2 の最小 conceptual shape 例

以下は final serialization ではなく、概念 shape の例である。

```text
EvaluationState {
  cursor_stack: [ ... ],
  place_stack: [root, session, profile_access],
  place_store: opaque snapshotable place-local state,
  current_request: {
    op: write_profile,
    mode: via,
    chain_ref: profile_ref,
    require: [write],
    ensure: []
  },
  chain_cursor: {
    chain_ref: profile_ref,
    candidate_order: [writer, readonly],
    next_index: 0,
    current_option_ref: writer
  },
  rollback_stack: [],
  trace_audit_sink: {
    events: [],
    non_admissible_metadata: [
      { option_ref: writer, subreason: lease-expired }
    ],
    narrative_explanations: [
      "readonly is request/capability mismatch"
    ]
  },
  terminal_outcome: Reject
}
```

## ここであえて決めていないこと

- state object の final field 名
- `place_store` の具体 layout
- snapshot ref の具体表現
- detached trace serialization
- multi-request scheduler を入れるかどうか
- step counter や event id を current L2 で必須にするかどうか
- ambient place frontier を `rollback_stack` と別 field に切り出すかどうか

これらは **未決定** とする。current L2 で固定するのは、representative examples を parser なしで実行準備できる最小 evaluation state だけである。
