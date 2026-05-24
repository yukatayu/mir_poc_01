# 34 — Textual Mir Alpha Grammar

## role

This document fixes the first **textual Mir alpha grammar** boundary for Full System V1 and records the Surface Mir alpha extension boundary that starts at `P-SURF-00B`.

The alpha grammar is real source input for roadmap implementation. It is not final public grammar and does not freeze all keywords, punctuation, or public parser API.

Surface Mir constructs described here are parser targets for `P-SURF-01` and later packages. They are normative syntax decisions for the Surface line, but they are not evidence that the existing Full System V1 parser already implements them.

## decision level

- `L1`
  - Mir source files are the intended semantic source of truth.
  - `package.mir.json` remains alpha compatibility and package artifact, not final source authority.
  - Parser diagnostics must be explicit and source-spanned.
- `L2`
  - The first textual grammar supports a safe C-like baseline plus minimal effect declarations and boundary calls.
  - The Surface Mir alpha extension adds place / role-instance / indexed-state / admission syntax while preserving the Full System V1 implementation status as closed evidence, not a live parser claim.
  - Syntax may be revised before final public grammar if docs, samples, parser, and validators are updated together.

## minimum supported surface

The first alpha grammar must support:

- `module` and `import`.
- `role`, `principal`, and `place` declarations for the Surface Mir alpha line.
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
- Surface Mir place blocks using the canonical brace syntax `S { ... }`.
- role-instance blocks such as `Participant[self] { ... }`; bare
  `Participant { ... }` is not a role-instance block in alpha.
- state declarations such as `state player[p: Participant]: Player` inside a
  place block.
- visibility clauses such as `visible observer_safe fields { hp }`.
- role capability declarations such as `supports renderer.pose_v1`.
- event behavior blocks such as
  `when attack(target: Participant) fails MissingCapability { ... }`.
- admission and authority statements:
  `join World as BrowserClient via WorldAdmission`, `grant Capability(args)`,
  `publish event(args)`, and `produces witness witness_name`.
- minimal arithmetic, comparison, and boolean operators required by the alpha samples, including `+`, `-`, `*`, `/`, `=`, `!=`, `<`, `<=`, `>`, `>=`, `and`, `or`, and `not`.

Surface Mir place scope must not use bracket syntax. `S[ ... ]` is rejected and
is not accepted as sugar. `[]` remains reserved for expression indexing,
including arrays, maps, role-instance heads, and indexed state access.

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
AstPlaceDecl
AstRoleDecl
AstPrincipalDecl
AstPlaceBlock
AstRoleInstanceBlock
AstStateDecl
AstVisibilityClause
AstWhenBlock
AstFailureRow
AstJoinStmt
AstGrantStmt
AstPublishStmt
AstWitnessProduction
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

Surface Mir place scope and indexed state:

```mir
module Surface.Attack

role Participant
place S

record Player {
  hp: Int64,
  atk: Int64,
}

S {
  state player[p: Participant]: Player
    init Player { hp: 100, atk: 10 }
}

Participant[self] {
  when attack(target: Participant)
    fails StaleMembership, MissingCapability, MissingWitness, RouteUnavailable {
    S {
      player[target].hp = player[target].hp - player[self].atk
    }
  }
}
```

The canonical place-scope form in this example is `S { ... }`. The rejected
form is:

```mir
S[
  state player[p: Participant]: Player
]
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
- unsupported bracket place scope such as `S[ ... ]`.
- cross-locus behavior that omits generated failure containment, such as a
  `when` block missing required `fails` entries.
- ambiguous brace construct where a head could be both a place / role path and
  a record literal head.

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
- Do not accept `S[ ... ]` as canonical syntax or sugar.
- Do not treat Surface Mir extension syntax in this document as evidence that
  the current closed Full System V1 parser already implements it.
