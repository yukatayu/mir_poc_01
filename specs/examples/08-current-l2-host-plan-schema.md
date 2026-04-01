# examples/08 — current L2 host plan schema

この文書は、current L2 の parser-free minimal interpreter / host harness を前提に、**machine-readable host plan schema** の最小 shape を整理する補助文書である。
ここで定めるのは production host interface ではなく、fixture ごとに predicate verdict、effect outcome、success-side carrier、trace expectation override、narrative explanation override を外部 asset として差し替えるための current L2 companion schema である。

## この文書の位置づけ

- `specs/examples/07-current-l2-host-stub-harness.md` が定める declarative host plan を machine-readable asset に落とす。
- parser-free minimal interpreter の semantics は変えず、Rust 側の ad hoc plan builder を sidecar JSON へ寄せる。
- machine-check する項目と human-facing explanation obligation の境界を、asset shape 側でも維持する。

## current L2 で固定すること

- host plan は fixture JSON に埋め込まず、fixture の隣に置く sidecar JSON として扱ってよい。
- sidecar JSON には、少なくとも次を declarative に表せればよい。
  - predicate verdict
  - effect outcome
  - success-side carrier / tentative post-state
  - non-admissible metadata override
  - exact compare する narrative explanation override
- `must_explain` は sidecar JSON に移さず、fixture / report 側の human-facing obligation に残す。
- runtime に入る fixture で実際に発生した oracle call は、host plan rule によって明示的に被覆されていなければならない。
- broader wildcard rule は許してよいが、複数 rule が同じ input を同時に受理できる overlap は current L2 では禁止し、loader が reject してよい。

## sidecar JSON を採る理由

current L2 では、fixture 本体に host plan を埋め込むより、sidecar JSON に分ける方が小さく整合的である。

### fixture JSON に埋め込む場合

- parser-free asset が 1 本で閉じるので読み込みは単純になる。
- 一方で、AST fixture schema と host-side verification plan が同じ carrier に混ざり、`program` 本体の意味構造と host override の境界が曖昧になる。
- `expected_trace_audit` と host-side override が 1 つの JSON に同居し、machine-check oracle と host plan の責務が重く見えやすい。

### sidecar JSON に分ける場合

- AST fixture schema は interpreter が読む semantics asset として保てる。
- host plan は verification harness 専用の差し替え asset として分離できる。
- parser 実装を先送りしたままでも、fixture 本体を安定させつつ host stub の差し替えだけを繰り返しやすい。

したがって current L2 では、**fixture JSON の隣に `.host-plan.json` sidecar を置く方針**を採る。

## 最小 top-level shape

sidecar JSON は、概念的には次だけを持てばよい。

1. `schema_version`
   - current L2 host plan schema の version marker
2. `predicate_rules`
   - `PredicateOracle` call を declarative に差し替える rule 列
3. `effect_rules`
   - `EffectOracle` call を declarative に差し替える rule 列
4. `trace_expectation_override`
   - exact compare したい formal metadata / short narrative explanation の override

field 名や serialization の長期固定は current L2 では行わない。

## predicate rule の最小 shape

predicate rule は、少なくとも次を持てばよい。

- `site`
  - `request-require`
  - `request-ensure`
  - `option-admit`
- `op`
- optional `mode`
  - `on`
  - `via`
- optional `target`
- optional `chain_ref`
- optional `option_ref`
- `verdict`
  - `satisfied`
  - `unsatisfied`

optional field を省略した場合は wildcard として読んでよい。
ただし current L2 では、複数 rule が同じ input を同時に受理できる overlap を許さない。

## effect rule の最小 shape

effect rule は、少なくとも次を持てばよい。

- `op`
- optional `mode`
- optional `selected_target`
- optional `chain_ref`
- optional `selected_option_ref`
- `verdict`
  - `success`
  - `explicit-failure`

`success` のときは、success-side carrier として次を持てばよい。

- `commit`
  - `mutations`

current L2 では representative fixtures に必要な mutation は `append-record` だけで足りる。

## trace expectation override の最小 shape

`trace_expectation_override` は、少なくとも次を持てばよい。

- optional `non_admissible_metadata`
- optional `narrative_explanations`

ここで `non_admissible_metadata` は current L2 の formal carrier であり、

- `option_ref`
- `subreason`

だけを要求すれば足りる。

`narrative_explanations` は short narrative の exact compare を許す optional carrier である。
一方で `must_explain` は host plan schema に含めない。

## machine-check と narrative explanation の境界

### host plan schema に含めてよいもの

- predicate verdict
- effect outcome
- success-side carrier
- formal な non-admissible metadata override
- short narrative explanation override

### host plan schema に含めないもの

- `must_explain`
- long-form audit explanation
- prose としての static verdict reason
- detached trace / audit serialization

これにより、current L2 は

- event surface
- formal metadata
- human-facing explanation obligation

の三層分離を保ったまま、machine-readable host plan を追加できる。

## representative fixtures での最小読み

- E1
  - effect success / explicit failure と、request-local predicate verdict を sidecar で明示する。
- E2
  - `validate_profile_patch` の `request-require` unsatisfied と、success-side carrier を sidecar で与える。
- E3 variant
  - `owner_writer` の `option-admit` unsatisfied、`delegated_writer` の success を sidecar で与える。
- E6
  - host plan 自体は空でもよいが、formal metadata override や narrative explanation override を sidecar で持てる。

## current L2 でまだ決めないこと

- field 名の長期固定
- reason code 名の長期固定
- detached trace / audit serialization
- richer host interface
- multi-request scheduler
- `Approximate` / `Compensate`
- parser 実装と host plan asset の対応関係

これらは **未決定** である。current L2 で固定するのは、parser なし PoC の実験ループを回すための最小 schema と sidecar 方針だけである。
