# 518 — current L2 theorem actual Lean execution narrow probe after global toolchain install

## 目的

`specs/examples/516`、
global `elan` stable install、
`e5-underdeclared-lineage`
を前提に、

- actual Lean execution narrow probe
- global toolchain resolved version
- Lean-stub placeholder fix
- representative static sample evidence
- current recommendation
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**global Lean toolchain が ready になった後、representative static sample `e5` に対して actual Lean execution を narrow に通し、toolchain reserve を actual execution floor へ 1 段だけ進める current cut**
であり、

- prototype-wide exhaustive actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem toolchain probe / reopen manifest
   - `lean` / `lake` / `elan` ready を machine-check できる
2. theorem Lean-stub pipeline
   - formal hook
   - review-unit emit
   - Lean-stub emit
3. theorem actual Lean execution initial failure
   - stub theorem type に `Prop` sort を使うと Lean が reject する

したがって current open problem は、
toolchain availability の説明ではなく、
**actual Lean execution を通すための smallest source fix を helper-local artifact emitter に入れられるか**
である。

## current actualization cut

current package では、次を採る。

1. global install は official `elan` stable を用いる
2. `2026-04-19` の observed resolution は `Lean 4.29.1` / `Lake 5.0.0-src+f72c35b` / `elan 4.2.1`
3. Lean stub source は `theorem ... : True := by sorry` に直し、sort `Prop` そのものを theorem type に置かない
4. actual Lean execution evidence は representative static sample `e5-underdeclared-lineage` に narrow に取る
5. prototype/runtime side への widening は still later に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `crates/mir-semantics/examples/support/current_l2_lean_theorem_stub_support.rs` | actual Lean execution failure を起こしていた stub source shape を `True` placeholder に修正した source-backed emitter |
| `crates/mir-semantics/tests/current_l2_lean_theorem_stub_actual_probe.rs` | Lean が available なとき、`e5-underdeclared-lineage` generated stubs を actual `.lean` file として実行する focused actual probe |
| `scripts/current_l2_theorem_toolchain_probe.py` | global toolchain ready を machine-readable に示す reserve/reopen helper |

## current recommendation

1. actual Lean execution は representative static sample まで narrow actualization に上げてよい。
2. toolchain ready 後の first fix は public theorem contract 拡張ではなく、generated stub source の compile-valid placeholder 化に置く。
3. prototype-wide actual Lean execution、public theorem contract、proof object public schema、final public verifier contract は still later に残す。

## retained alternatives

- runtime/prototype corpus first widening
- hermetic toolchain pin first
- proof object public schema first reopen

## stop line

current package は次で止める。

- prototype-wide exhaustive actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract
