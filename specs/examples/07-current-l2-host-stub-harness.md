# examples/07 — current L2 host stub harness

この文書は、current L2 の representative examples、AST fixture schema、evaluation state schema、step semantics、oracle API、parser-free minimal interpreter skeleton を前提に、**fixture runner 用 host stub layer / test harness** の最小境界をまとめる補助文書である。
ここで扱うのは `crates/mir-semantics` に置く current L2 専用の検証 harness であり、production host interface や detached trace runtime を定義するものではない。

## この文書の位置づけ

- parser-free minimal interpreter に対し、fixture ごとに declarative な host plan を差し替える最小 harness を与える。
- `specs/examples/05-current-l2-oracle-api.md` の predicate / effect oracle 境界を、test-only stub としてどう concretize するかを示す。
- machine-check する expectation と human-facing な説明責務の境界を明示する。
- machine-readable な host plan schema / sidecar asset 自体は `specs/examples/08-current-l2-host-plan-schema.md` を参照する。

## current L2 で固定すること

- `PredicateOracle` / `EffectOracle` の ad hoc match 文を直接増やさず、fixture ごとの declarative host plan から最小 stub を組み立ててよい。
- host plan で declarative に差し替える対象は、少なくとも次である。
  - predicate verdict
  - effect outcome
  - success-side carrier として読む tentative post-state
  - formal な non-admissible metadata expectation
  - exact compare する narrative explanation expectation
- `must_explain` のような human-facing explanation obligation は machine-check carrier に上げず、report / prose 側に残す。
- current L2 harness は、未指定 rule を permissive default として常用しない。runtime に入る fixture で実際に発生した predicate / effect oracle call は、host plan 側で明示的に被覆されていなければならない。
- wildcard を使った broader rule は許してよいが、同じ入力を複数 rule が同時に受理できる overlap は current L2 では禁止し、loader / harness 側で reject してよい。

## host plan の最小 shape

field 名や serialization は未固定だが、conceptual shape として current L2 に最低限必要なのは次である。

1. predicate rule
   - `site`
   - `op`
   - optional `mode`
   - optional `target`
   - optional `chain_ref`
   - optional `option_ref`
   - verdict (`Satisfied` / `Unsatisfied`)
2. effect rule
   - `op`
   - optional `mode`
   - optional `selected_target`
   - optional `chain_ref`
   - optional `selected_option_ref`
   - verdict
     - `Success(commit_plan)`
     - `ExplicitFailure`
3. success-side carrier / tentative post-state plan
   - current L2 では store mutation の小さな列で十分である。
   - 代表例では `append_record` だけあれば E1 / E2 / E3 variant を動かせる。
4. trace expectation override
   - optional `non_admissible_metadata`
   - optional `narrative_explanations`

## machine-check と narrative の境界

### harness が厳密比較してよいもの

- static gate verdict
- runtime final outcome
- `event_kinds`
- formal な `non_admissible_metadata`
- exact compare を許す `narrative_explanations`

### harness に入れないもの

- `must_explain`
- static verdict reason の prose
- long-form audit explanation
- detached trace serialization

current L2 では、`narrative_explanations` は machine-check 可能な短い explanation carrier として比較してよい。
一方で、`must_explain` は「人が説明責務を果たしたか」を見る prose obligation であり、harness に exact string compare を持ち込まない。

## host plan coverage の最小方針

- runtime に入る fixture で実際に発生した `PredicateOracle` / `EffectOracle` call は、host plan rule のどれか 1 つに明示的に対応づけられなければならない。
- 被覆されていない oracle call があった場合、current L2 harness はその run を invalid host plan として失敗させてよい。
- rule 同士の precedence を current L2 で語彙化しない代わりに、overlap を reject する。これにより first-match / last-match の hidden policy を持ち込まない。
- これは production host interface の制約ではなく、current L2 verification harness の最小方針である。

## representative fixtures での読み

### E1

- `update_authority` success は effect rule で success-side carrier を返す。
- `append_audit` explicit failure は effect rule で差し替える。
- `atomic-cut` は harness では駆動せず、interpreter 側 step semantics が event を出す。

### E2

- `validate_profile_patch` の `request-require` だけを predicate rule で `Unsatisfied` にする。
- rollback と fallback branch success は interpreter 側で起きる。
- harness は rollback event 自体を作らない。

### E3 variant

- `owner_writer` の `option-admit` だけを predicate rule で `Unsatisfied` にする。
- `admit-miss` metadata は interpreter 側が残す。
- `delegated_writer` success は effect rule で success-side carrier を返す。

### E6

- host plan は空でもよい。
- `lease-expired` metadata と final `Reject` は interpreter 側の structural rule だけで生じる。
- capability mismatch は formal subreason ではなく narrative explanation に留まる。

## current L2 でまだ実装しないもの

- production host interface
- detached trace / audit serialization
- multi-request scheduler
- richer predicate evaluator API
- `Approximate` / `Compensate`
- parser syntax に対応した host binding

これらは **未決定** または future work であり、current L2 host stub harness には入れない。
