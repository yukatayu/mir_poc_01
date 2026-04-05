# examples/06 — current L2 interpreter skeleton

この文書は、current L2 の representative examples、AST fixture schema、evaluation state schema、step semantics、oracle API を前提に、**parser-free 最小 interpreter skeleton** の実装境界をまとめる補助文書である。
ここで扱うのは `crates/mir-semantics` に置く current L2 専用の direct-style evaluator skeleton であり、full parser、full scheduler、production runtime を定義するものではない。

## この文書の位置づけ

- representative examples を parser なしで machine-readable fixture から実行するための最小骨組みを示す。
- `specs/examples/02-current-l2-ast-fixture-schema.md`、`specs/examples/03-current-l2-evaluation-state-schema.md`、`specs/examples/04-current-l2-step-semantics.md`、`specs/examples/05-current-l2-oracle-api.md` の実装側 mirror を与える。
- static gate と runtime evaluation の境界、machine-checked な expectation と narrative explanation の境界を明示する。

## current L2 で実装する最小対象

current L2 の parser-free 最小 interpreter skeleton は、少なくとも次を扱う。

- `Program`
- `PlaceBlock`
- `PerformOn`
- `PerformVia`
- `TryFallback`
- `AtomicCut`
- `OptionDecl`
- `ChainDecl`

ただし `OptionDecl` / `ChainDecl` は runtime declaration-only no-op として扱い、実行中に副作用を起こさない。

## static gate と runtime evaluation の境界

### static gate が扱うこと

- fixture の `expected_static.verdict` に相当する `valid` / `malformed` / `underdeclared`
- chain head / edge / lineage assertion / declared access target / monotone capability の基本整合
- runtime に入る前に止めるべき E4 / E5 のような例

### runtime evaluation が扱うこと

- `PlaceBlock` に沿った `place_stack` の push / pop
- `TryFallback` に沿った local rollback
- `PerformOn` / `PerformVia` の request-local contract evaluation
- chain left-to-right evaluation
- option-local `admit`
- `lease-expired` metadata
- final `success` / `explicit_failure` / `Reject`

current L2 では、`expected_static.reasons` は human-readable explanation として保持し、interpreter skeleton で exact string compare を必須にはしない。
一方で `expected_runtime` と `expected_trace_audit` のうち、

- `enters_evaluation`
- `final_outcome`
- `event_kinds`
- `non_admissible_metadata`
- `narrative_explanations`

は machine-check 可能な最小 expectation として扱う。

## evaluator skeleton の最小 API

`crates/mir-semantics` の current L2 skeleton は、少なくとも次を public API として持てばよい。

1. `load_fixture_from_path`
   - fixture JSON を読み、machine-readable AST carrier に落とす。
2. `static_gate` または `static_gate_detailed`
   - runtime evaluation 前の gate を行う。
3. `EvaluationState`
   - parser-free 実行に必要な最小状態を保持する。
4. `step_once`
   - current state を 1 step 進める。
5. `run_to_completion`
   - `step_once` を反復して terminal outcome まで進める。

## machine-checked と narrative の境界

### machine-checked に残すもの

- static verdict
- runtime final outcome
- event kind 列
- formal な non-admissible metadata
  - `admit-miss`
  - `lease-expired`
- capability mismatch narrative explanation の exact string
  - current L2 では formal subreason に上げず、narrative explanation として比較してよい

### narrative explanation に残すもの

- `must_explain`
- static verdict reason の prose
- rollback / frontier / chain exhaustion の長い説明文

これにより、current L2 の skeleton は「何が起きたか」を最小限検証できる一方で、human-facing explanation の wording まで interpreter に埋め込まない。

## representative examples の coverage

current L2 の parser-free 最小 interpreter skeleton は、少なくとも次の representative fixture を動かす。

- E1
  - `PerformOn` success、`AtomicCut`、続く `PerformOn` explicit failure
- E2
  - `TryFallback` による local rollback と fallback branch success
- E3 variant
  - option-local `admit` miss を metadata に残しつつ次候補へ進む via-chain success
- E6
  - `lease-expired` metadata、capability mismatch narrative explanation、final `Reject`
- E6 補完 regression fixtures
  - `e7-write-fallback-after-expiry`
    - earlier writer の `lease-expired` metadata を残しつつ、同じ lineage 上の後段 write-capable option で success する
  - `e8-monotone-degradation-reject`
    - `admit-miss`、middle option の explicit failure、final `Reject` を left-to-right degradation として読む
  - `e9-monotone-degradation-success`
    - `admit-miss`、middle option の explicit failure のあとでも、later same-lineage write-capable option が request を満たせば left-to-right degradation のまま success しうる

また E4 / E5 は static gate で止まり、runtime evaluation に入らないことを確認する。

## current L2 でまだ実装しないもの

- full parser
- multi-request scheduler
- detached trace / audit serialization
- richer predicate evaluator API
- richer host effect interface
- `Approximate` / `Compensate`
- coroutine / overlay / Prism 連携

これらは **未決定** または future work であり、current L2 の interpreter skeleton には入れない。

## current L2 の実装境界

current L2 の parser-free 最小 interpreter skeleton は、次を目標にする。

- representative fixtures を仕様どおりに進められる
- static gate と runtime evaluation を分離できる
- trace / audit expectation を event surface を増やさずに最低限照合できる

一方で、次は目標にしない。

- production runtime としての完全性
- host serialization の固定
- parser syntax の固定
- distributed scheduler の導入

したがって、この skeleton は「今ある理論を実行準備できる形へ落とすための橋」であり、Mir 全体の final runtime ではない。
