# 68 — current L2 try/rollback AST helper first tranche actualization

## 目的

この文書は、current L2 parser-free PoC と `specs/examples/65`〜`67` の docs-only judgment を前提に、
**future dedicated AST structural helper の first tranche が current repo でどこまで actualize 済みか**
を整理する。

ここで固定するのは final public checker API ではない。
固定するのは、

- first tranche で actualize 済みの helper-local carrier
- fixture-side expected field の current actual shape
- minimal malformed static family の actual corpus
- static gate artifact loop に接続された smoke path
- まだ外に残すべき boundary

である。

## current code anchor

current repo では、次の code anchor が first tranche actualization の正本である。

- `crates/mir-semantics/src/lib.rs`
  - `TryRollbackStructuralVerdict`
  - `TryRollbackStructuralSubjectKind`
  - `TryRollbackStructuralFindingKind`
  - `TryRollbackStructuralFindingRow`
  - `ExpectedStatic.checked_try_rollback_structural_verdict`
  - `ExpectedStatic.checked_try_rollback_structural_findings`
  - `static_gate_detailed()`
  - `collect_try_rollback_structural_reasons()`
- `scripts/current_l2_try_rollback_structural_checker.py`
  - dedicated helper-local compare helper
- `scripts/current_l2_detached_loop.py`
  - `smoke-try-rollback-structural-checker`
- `crates/mir-ast/tests/fixtures/current-l2/e23-malformed-try-fallback-missing-fallback-body.json`
- `crates/mir-ast/tests/fixtures/current-l2/e24-malformed-atomic-cut-fallback-placement.json`
- `crates/mir-semantics/tests/current_l2_minimal_interpreter.rs`

## actualized first tranche

current repo で actualize 済みとみなしてよいのは、次の 4 点である。

1. dedicated helper-local compare helper
2. fixture-side expected field の actual schema
3. minimal malformed static family の two-fixture pair
4. static gate artifact loop の family-specific smoke path

これは `specs/examples/65` の first tranche cut と、`specs/examples/66` / `67` の two-fixture / slot selection judgment を
**docs-only comparison に留めず actual helper family へ narrow に下ろした current state**
である。

## helper-local carrier

current first tranche の helper-local carrier は次である。

### structural verdict

- field 名:
  - `expected_static.checked_try_rollback_structural_verdict`
- current enum:
  - `no_findings`
  - `findings_present`

この field は `expected_static.verdict` を置き換えない。
`expected_static.verdict` は引き続き current L2 static gate の public verdict を表し、
structural helper verdict は helper-local dedicated compare のための additive field に留まる。

### finding rows

- field 名:
  - `expected_static.checked_try_rollback_structural_findings`
- row shape:
  - `subject_kind`
  - `finding_kind`

current first tranche で actualize 済みの `subject_kind` は次である。

- `TryFallback`
- `AtomicCut`

current first tranche で actualize 済みの `finding_kind` は次である。

- `missing_fallback_body`
- `disallowed_fallback_placement`

## current first-tranche finding family

current repo で actualize 済みの first-tranche family は次の 2 件だけである。

### 1. `TryFallback` + empty `fallback_body`

- AST shape:
  - `TryFallback`
  - `body` は decode 可能
  - `fallback_body = []`
- helper-local finding:
  - `subject_kind = TryFallback`
  - `finding_kind = missing_fallback_body`
- static gate wording:
  - `try fallback body must not be empty`

ここで `missing_fallback_body` は field absence ではなく、
**decode 後の empty `fallback_body` を semantic structural malformed と読む first-tranche rule**
である。

### 2. `AtomicCut` inside `fallback_body`

- AST shape:
  - `TryFallback.fallback_body` の内部に `AtomicCut`
- helper-local finding:
  - `subject_kind = AtomicCut`
  - `finding_kind = disallowed_fallback_placement`
- static gate wording:
  - `atomic cut may not appear inside fallback body`

ここで `disallowed_fallback_placement` は first tranche の working family であり、
nested place mismatch や raw schema malformed を一般に表す final taxonomy ではない。

## static gate との current 接続

current first tranche では、helper-local compare helper だけでなく、
`static_gate_detailed()` 自体も次を current structural malformed floor として actualize している。

- `fallback_body.is_empty()` の `TryFallback`
- `fallback_body` 内に現れた `AtomicCut`

これらの理由文字列が出たとき、current static gate verdict は `Malformed` へ escalate する。

ただし current state でも、次は **まだ外に残す**。

- `place_anchor == current_place` gate
- whole-store restore scope
- nested place mismatch `AtomicCut`
- runtime representative `E21` / `E22` の dynamic frontier / contrast reading

したがって current actualization は、
**`TryFallback` / `AtomicCut` の structural floor 全体を public checker core に入れた**
という意味ではない。

## minimal malformed static corpus

current first tranche の minimal malformed static corpus は次の 2 fixture である。

### `e23-malformed-try-fallback-missing-fallback-body`

- `fixture_id`
  - `e23_malformed_try_fallback_missing_fallback_body`
- static verdict
  - `malformed`
- structural verdict
  - `findings_present`
- finding row
  - `TryFallback` / `missing_fallback_body`

### `e24-malformed-atomic-cut-fallback-placement`

- `fixture_id`
  - `e24_malformed_atomic_cut_fallback_placement`
- static verdict
  - `malformed`
- structural verdict
  - `findings_present`
- finding row
  - `AtomicCut` / `disallowed_fallback_placement`

この 2 fixture により、`specs/examples/66` の two-fixture pair judgment と
`specs/examples/67` の slot selection judgment が current corpus に actualize 済みになった。

## detached validation loop の current cut

current first tranche は bundle-first runtime path に入れず、
static gate artifact loop の helper-local smoke family にだけ接続する。

current actual command surface は次である。

```text
python3 scripts/current_l2_detached_loop.py \
  smoke-try-rollback-structural-checker <fixture_path> \
  --run-label <label> \
  --overwrite
```

この wrapper は、

1. static gate artifact を emit する
2. `scripts/current_l2_try_rollback_structural_checker.py` を回す

という narrow smoke family に留まる。

current helper-local compare helper は fixture AST を再走査して success を作らない。
current helper は emitted artifact の

- `checker_core.static_verdict`
- `checker_core.reasons`

を読み、first-tranche family の current reason wording を helper-local row family へ写して compare する。
したがって current smoke family は、
**static gate artifact loop に実際に emit された contract が first-tranche expectation を満たすか**
を narrow に検査する。

## current helper boundary

first tranche actualization 後も、次は **まだ current helper boundary の外**である。

### まだ actualize しないもの

- detached artifact shared carrier
- bundle-first runtime artifact への shared mirror
- generic structural checker family 合流
- public checker API
- saved artifact compare core への昇格

### current helper-local に留めるもの

- `checked_try_rollback_structural_verdict`
- `checked_try_rollback_structural_findings`
- `smoke-try-rollback-structural-checker`
- `scripts/current_l2_try_rollback_structural_checker.py`

## current judgment

current L2 の settled current state としては、次が成り立つ。

1. first tranche は docs-only judgment ではなく actual helper family まで下りた
2. ただし helper-local dedicated contract のままであり、public checker core には上げていない
3. `TryFallback` / `AtomicCut` の runtime representative `E2` / `E21` / `E22` は引き続き current evidence として維持する
4. `place_anchor == current_place` gate や restore scope は current phase でも runtime / proof boundary に残す

## next narrow step

current first tranche actualization の次段として自然なのは、次のいずれかを narrow に比較することである。

1. second malformed static tranche を足す threshold
2. helper-local wording / `finding_kind` family を current first tranche のまま維持する期間
3. saved artifact compare need が本当に shared carrier threshold を満たしたかどうか

current task では、これらをまだ決めない。

## OPEN に残すもの

- second malformed static tranche の exact family
- `missing_fallback_body` / `disallowed_fallback_placement` の長期 wording 固定
- generic structural checker family との合流 timing
- public checker API comparison へ進める条件
- shared detached carrier へ上げる timing
