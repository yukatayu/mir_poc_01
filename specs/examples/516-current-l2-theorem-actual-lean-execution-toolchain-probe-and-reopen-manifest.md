# 516 — current L2 theorem actual Lean execution toolchain probe and reopen manifest

## 目的

`specs/examples/513`、
`514`、
既存の Lean-stub pipeline helper と
`e5-underdeclared-lineage`
を前提に、

- actual Lean execution toolchain probe
- sample-aware reopen manifest
- Lean-stub pipeline carry-over
- environment stop line
- current recommendation
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**actual Lean execution unavailable を単なる report 文言に留めず、`lean` / `lake` / `elan` availability と sample-aware reopen plan を repo-local helper/CLI に actualize する current cut**
であり、

- actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem actual Lean execution availability probe
   - local environment unavailable なら actual Lean execution は reserve に戻す
2. theorem public seam compression after local Lean-unavailable probe
   - Lean-stub bridge keep
   - actual Lean execution environment-conditional
3. theorem Lean-stub pipeline helper
   - formal hook
   - review-unit emit
   - Lean-stub emit

したがって current open problem は、
Lean unavailable の説明を増やすことではなく、
**toolchain availability と sample-aware reopen path を再現可能な helper/CLI へ actualize できるか**
である。

## current helper cut

current package では、次を採る。

1. required toolchain は `lean` / `lake` / `elan` の 3 command に narrow する
2. probe result は
   - `ready`
   - `partial`
   - `unavailable`
   の 3 値で返す
3. sample を与えたときは、
   existing Lean-stub pipeline の formal-hook / review-unit / Lean-stub emit plan を reopen manifest に carry-over する
4. toolchain unavailable の間は、
   Lean-stub representative bridge と public-seam compression を current theorem evidence として keep する
5. actual Lean execution 自体は still reserve に残す

## actual runnable evidence

| evidence | current reading |
|---|---|
| `scripts/current_l2_theorem_toolchain_probe.py` | local `PATH` 上の `lean` / `lake` / `elan` を probe し、sample-aware reopen manifest を JSON として emit する repo-local helper/CLI |
| `scripts/tests/test_current_l2_theorem_toolchain_probe.py` | unavailable / ready / sample-aware reopen manifest を machine-check する focused Python unit test |
| `current_l2_theorem_lean_stub_pipeline.py` carry-over | toolchain ready 時の next narrow probe で reuse する existing pipeline plan anchor |

## current recommendation

1. actual Lean execution reserve は toolchain probe helper と sample-aware reopen manifest まで actualize してよい。
2. toolchain unavailable の間は Lean-stub bridge keep を current theorem execution evidence に置く。
3. actual Lean execution 自体は toolchain ready が出た時だけ narrow probe として reopen する。
4. public theorem contract / proof object public schema / final public verifier contract は still later に残す。

## retained alternatives

- local install first
- hermetic/container toolchain first
- different theorem backend first

## stop line

current package は次で止める。

- actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract
