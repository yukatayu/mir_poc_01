---
id: meta/proposal-004
status: L3-open
maturity: draft
depends_on: [adr/ADR-0001, adr/ADR-0002, adr/ADR-0005, adr/ADR-0008, adr/ADR-0009, spec/01-lexical-and-modules, spec/02-surface-grammar, spec/03-static-semantics, theory/01-mircore-v0, theory/02-types-effects-failures, scenarios/readme, plan/02-operating-model]
summary: Surface v0 EBNF 閉包の owner decision record。Participant-only index と既存語彙だけによる最小閉包を採択し、custom keyspace / 新 builtin は後段へ送る。
open_items: []
---

# PROPOSAL-004 - Surface v0 grammar closure

> Decision-request artifact. The owner disposition is recorded below and has
> only the stated design-package effect; it has no automatic implementation or
> lifecycle effect.
>
> It does not change Surface syntax, lexical rules, static semantics, Core,
> SCN expectations, proof status, Gate/Phase state, implementation, or public
> API.

## Target and Authority Boundary

The target is the exact-parser boundary of `spec/01`--`03`. `spec/02` is
labelled L1-fixed EBNF but currently leaves several nonterminals undefined and
permits a named keyspace without a declaration form. Only the human owner may
select the language direction; an accepted L1 change requires the ordinary
ADR/CHANGELOG/index process. This proposal asks for that direction only.

The proposal must not make `World`, `Game`, `Object`, or `Avatar` a core
primitive. It must preserve ADR-0001's domain-neutral vocabulary, ADR-0002's
non-source occurrence/request mechanics, ADR-0005's authority separation, and
ADR-0009's `.mir` source authority.

## Current Evidence

- All six canonical scenario indexed-state declarations use `Participant`.
  No canonical scenario uses a named custom keyspace.
- The 47 indexed-state declarations in the active LAB Full System V1 Surface
  corpus also all use `Participant`. This is LAB evidence, not canon.
- `LAB:specs/40` says the alpha role keyspace is `Participant` and lists
  `Object` and `Avatar` keyspaces as later. It therefore supports narrowing v0
  rather than importing an unexercised declaration form.
- `theory/01` and theory/02 intentionally retain an abstract finite Core
  keyspace family. That abstraction does not require every Core keyspace to be
  directly writable in Surface v0.
- Current canonical and active Surface sources contain no signed numeric
  literal. The existing lexical optional sign and the stated unary-minus
  precedence nevertheless leave tokenization of unspaced subtraction unclear.

## Question Presented

> Which Surface v0 grammar-closure direction should the owner authorize?
>
> **A**, close a Participant-only Surface v0 with the existing vocabulary and
> the bounded EBNF below; **B**, add named custom keyspaces and an explicit
> declaration surface now; or **C**, leave the grammar intentionally partial
> and make no exact parser/checker claim yet?

## Owner disposition

Recorded on 2026-07-28: **A accepted — Participant-only closure.**

The accepted direction authorizes a later Canon wording package to make
`Participant` the sole Surface v0 indexed-state keyspace, retain the abstract
finite-keyspace family in Core, and use the bounded lexical/expression closure
below. It does not itself amend `spec/01`, `spec/02`, `spec/03`, a scenario, a
parser, or a checker. Scalar-state/terminal-fallback alignment and the
unelaborated `return` token are separate decisions in PROPOSAL-015.

Custom keyspaces remain a later extension requiring their own declaration,
resolution, diagnostics, examples, compatibility analysis, and ordinary Canon
integration. The accepted direction does not introduce a `keyspace` builtin.

## Alternatives

| Option | Immediate effect if selected | Semantic and implementation delta |
| --- | --- | --- |
| A - Participant-only closure (recommended) | Exact EBNF is closed for the current v0 construct set. `Participant` is the only Surface indexed-state keyspace; custom keyspaces are later. | No new core primitive, declaration keyword, authority rule, effect, transport behavior, or implementation. Core remains abstract over finite keyspaces. |
| B - custom-keyspace v0 | Add a declaration form, resolver scope, diagnostic behavior, and SCN/sample evidence for named keyspaces. | Adds a source-language family not required by current canon scenarios or active LAB corpus. Must separately decide its syntax and static semantics. |
| C - defer closure | Preserve the present partial document. | No language change, but an exact parser/checker grammar remains unavailable. |

## Candidate Detail for Option A

Option A makes no new user-facing concept. It turns the existing prose and
scenario forms into an exact, deliberately small grammar. The corresponding
canon package would use the following closure direction:

```ebnf
DottedName       ::= Ident { "." Ident }
ModulePath       ::= DottedName
TypePath         ::= Ident
PlacePath        ::= Ident
RolePath         ::= Ident
Param            ::= Ident ":" Type
FailName         ::= Ident
Keyspace         ::= "Participant"      (* contextual, predeclared only *)

Expr             ::= OrExpr
OrExpr           ::= AndExpr { "or" AndExpr }
AndExpr          ::= NotExpr { "and" NotExpr }
NotExpr          ::= "not" NotExpr | CompareExpr
CompareExpr      ::= AddExpr [ CompareOp AddExpr ]
CompareOp        ::= "==" | "!=" | "<" | "<=" | ">" | ">="
AddExpr          ::= MulExpr { ("+" | "-") MulExpr }
MulExpr          ::= UnaryExpr { ("*" | "/") UnaryExpr }
UnaryExpr        ::= "-" UnaryExpr | PostfixExpr
PostfixExpr      ::= Primary { "[" Expr "]" | "." Ident | CallArgs }
CallArgs         ::= "(" [ Expr { "," Expr } ] ")"
Primary          ::= Literal | Ident | "(" Expr ")" | RecordLiteral
RecordLiteral    ::= TypePath "{" [ RecordField { "," RecordField } [","] ] "}"
RecordField      ::= Ident ":" Expr
Literal          ::= Int64Literal | Float64Literal | "true" | "false" | TextLiteral
```

The lexical companion is deliberately narrow: numeric literal tokens are
unsigned, and negation is the existing unary `-` production. Thus `-1` parses
as unary negation and `1-2` remains unambiguous without whitespace-sensitive
tokenization. `Participant` remains an ordinary identifier outside `Keyspace`
position; it is a contextual predeclared name there, so a role or value may
still use that spelling. A non-`Participant` keyspace is rejected as a grammar
error using existing generic `E-PARSE-001`; no new diagnostic family is needed.

`spec/03` would consequently state that the v0 indexed-state keyspace is the
predeclared `Participant`, rather than claiming an undeclared general
keyspace-resolution mechanism. A later custom-keyspace proposal must add its
own syntax, resolution, diagnostics, examples, and compatibility analysis.

This closure preserves the existing brace disambiguation. It does not decide
call resolution, function declaration syntax, constructor semantics, imported
type namespaces, dynamic locus expressions, a final AST/ABI, or any runtime
behavior. Those remain subject to their existing specifications or later
decisions.

## Advisory Recommendation

T-RESEARCH-030 recommends **A - Participant-only closure**. It is the smallest
choice compatible with the frozen scenarios, active LAB evidence, minimal-core
axis, and the existing abstract Core theory. It removes an unsupported source
feature rather than creating a `keyspace` builtin or inferring declaration
semantics from a bare identifier.

This recommendation is advisory. It is not an owner disposition, grammar
change, or claim that the candidate is already parsed or implemented.

## Requested Owner Output

Record one of:

- `A accepted`: authorize the bounded Participant-only grammar closure above;
- `B accepted`: authorize a separate custom-keyspace syntax/semantics design;
- `C deferred`: retain the current partial grammar until a later need; or
- `return for clarification`, naming the disputed construct.

Recorded output on 2026-07-28: `A accepted`.

For C, reopen when an exact parser/checker claim or a canonical scenario needs
a non-`Participant` indexed-state keyspace. For B, the owner record must name
the declaration design package; it does not itself choose a keyword or make
custom keyspaces available.

## Non-effects

This proposal does not:

- change ADR-0001/0002/0005/0008/0009, any L0/L1 status, theory/11, SCN,
  Gate, Phase, or conformance result;
- add a `keyspace` keyword, custom keyspace, `Object`/`Avatar` builtin,
  domain primitive, event primitive, effect, capability, transport behavior,
  provider, or runtime operation;
- choose a final parser, AST, Core IR, JSON carrier, code generator, public
  API, syntax-highlighting vocabulary, or implementation schedule;
- alter the Core's abstract finite-keyspace model, authority, ownership,
  membership, failure-row, visibility, cut, fallback, or patch semantics; or
- establish parser correctness, C-static/C-runtime/C-distributed conformance,
  proof discharge, Gate/Phase exit, or product readiness.
