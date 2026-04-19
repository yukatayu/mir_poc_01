# 513 — current L2 theorem actual Lean execution availability probe

## 目的

`specs/examples/508`、
`509`、
`510`
を前提に、

- actual Lean execution availability probe
- local environment gate
- Lean-stub floor keep
- current recommendation
- retained alternatives
- stop line

を 1 本に束ねる。

ここで actualize するのは、
**actual Lean tool execution へ進む前に local environment availability を narrow に probe し、tool unavailable なら Lean-stub floor を current stop line として維持する current cut**
であり、

- actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract

は fixed しない。

## source-backed floor

current repo では、少なくとも次が source-backed である。

1. theorem Lean-first non-production stub pilot actualization
2. theorem review-unit to Lean-stub repo-local artifact-conformance bridge
3. theorem Lean-stub representative trace-alignment bridge

したがって current open problem は、
Lean-stub floor の前段を作り直すことではなく、
**actual Lean tool execution を local environment で今すぐ probe できるか**
である。

## probe result

2026-04-19 時点の local environment では、

- `lean --version`
- `lake --version`
- `elan --version`

はいずれも `command not found` であった。

したがって current package の current result は、

1. actual Lean execution は local environment gate により **not available**
2. current theorem floor は Lean-stub pilot / artifact-conformance bridge / representative trace-alignment bridge に留める
3. actual Lean execution は environment-conditional reserve line に戻す

である。

## current recommendation

1. local environment で Lean toolchain が unavailable な間は、Lean-stub floor を current theorem execution line に保つ。
2. actual Lean execution は、toolchain availability が出た時だけ narrow probe として reopen する。
3. public theorem contract / proof object public schema への promotion は進めない。

## retained alternatives

- Lean toolchain を local install して reopen
- external container / hermetic toolchain で reopen
- different theorem backend first

## stop line

current package は次で止める。

- actual Lean execution
- public theorem contract
- proof object public schema
- final public verifier contract
