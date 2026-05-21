# 34 — Textual Mir Alpha Grammar

## role

This document fixes the first **textual Mir alpha grammar** boundary for Full System V1.

The alpha grammar is real source input for roadmap implementation. It is not final public grammar and does not freeze all keywords, punctuation, or public parser API.

## decision level

- `L1`
  - Mir source files are the intended semantic source of truth.
  - `package.mir.json` remains alpha compatibility and package artifact, not final source authority.
  - Parser diagnostics must be explicit and source-spanned.
- `L2`
  - The first textual grammar supports a safe C-like baseline plus minimal effect declarations and boundary calls.
  - Syntax may be revised before final public grammar if docs, samples, parser, and validators are updated together.

## minimum supported surface

The first alpha grammar must support:

- `module` and `import`.
- `fn` declarations.
- `let` / `mut` bindings.
- primitive types: `Bool`, `Int64`, `UInt64`, `Float64`, `Text`, `Unit`.
- records and field access.
- fixed arrays / vectors and indexing.
- `if` / `else`.
- minimal `for` and `while`.
- `return`.
- effect declarations.
- effect members: input/output values, failure rows, and required capability refs.
- capability declarations and capability requirement clauses.
- `transition ... at ...` runtime entrypoints.
- `perform` boundary calls with explicit `via` boundary/provider refs.
- `require` / `ensure` contract clauses.
- minimal arithmetic, comparison, and boolean operators required by the alpha samples, including `+`, `-`, `*`, `/`, `=`, `!=`, `<`, `<=`, `>`, `>=`, `and`, `or`, and `not`.

## parser output

The parser must produce:

```text
AstModule
AstImport
AstFunction
AstRecord
AstCapabilityDecl
AstBlock
AstStmt
AstExpr
AstEffectDecl
AstEffectMember
AstTransition
AstBoundaryRef
AstContractClause
AstBinaryOp
AstUnaryOp
SourceSpan
Diagnostic
```

The output must include source spans sufficient for diagnostics and later devtools linking.

## alpha examples

Pure computation:

```mir
module Computational.AddOne

fn add_one(x: Int64) -> Int64 {
  let y: Int64 = x + 1
  return y
}
```

Host boundary plus Mir-owned transform:

```mir
module Computational.HostIoAddOne

import Computational.AddOne

capability HostRead
capability HostWrite

effect read_int {
  requires HostRead
  output x: Int64
  failure AdapterUnavailable
}

effect write_int(y: Int64) {
  requires HostWrite
  failure AdapterUnavailable
}

transition main at ComputationalHostPlace requires HostRead, HostWrite {
  x <- perform read_int via host_input
  y <- add_one(x)
  perform write_int(y) via host_output
    ensure y = x + 1
}
```

Transform types:

```mir
module Pose.Basic

record Vec3 {
  x: Float64,
  y: Float64,
  z: Float64,
}

record Transform {
  position: Vec3,
  pose_version: UInt64,
}
```

These examples are alpha source examples, not final syntax commitments.

## negative grammar rows

The grammar line must include negative samples for:

- unresolved import.
- malformed function signature.
- missing type annotation where alpha grammar requires one.
- malformed record field.
- malformed `perform` boundary.
- malformed `transition` entrypoint.
- malformed capability requirement.
- contract clause outside an allowed position.

## source/package relation

During alpha:

```text
Mir source files
  -> parsed AST
  -> typed IR
  -> optional generated package artifact / package.mir.json
```

The reverse direction may exist for migration helpers, but it must not become the final semantic source of truth.

## stop line

- Do not call this final public grammar.
- Do not treat `package.mir.json` as final source grammar.
- Do not accept parser-free source claims for `FS-01`.
- Do not silently change syntax without updating samples, docs, and validation.
