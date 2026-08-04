---
id: spec/02-surface-grammar
status: L2-working
maturity: draft
depends_on: [spec/01-lexical-and-modules, adr/ADR-0008, adr/ADR-0016, theory/06-existence-fallback]
summary: M6 前の Surface Mir grammar candidate。place/role ブロック、state、when、join、chain 宣言、式。
open_items: [OPEN-005, OPEN-006]
---

# 02 — Surface grammar (EBNF)

This is the retained **pre-M6 grammar candidate**, not a final public grammar.
It documents the current compatibility profile and its explicit rejects. M6
selects the final bounded grammar only after the shared semantic model, using
this candidate and at most one smallest viable alternative. No current LAB
parser, fixture, or package becomes normative merely because it follows this
profile.

```ebnf
Module        ::= "module" ModulePath { Item }
Item          ::= Import | PlaceDecl | RoleDecl | RecordDecl
                | ChainDecl | PlaceBlock | RoleInstanceBlock
Import        ::= "import" ModulePath
PlaceDecl     ::= "place" Ident
RoleDecl      ::= "role" Ident [ "{" { "supports" DottedName } "}" ]
RecordDecl    ::= "record" Ident "{" [ Field { "," Field } [","] ] "}"
Field         ::= Ident ":" Type
Type          ::= "Int64" | "Float64" | "Bool" | "Text" | TypePath

PlaceBlock    ::= PlacePath "{" { BlockItem } "}"
BlockItem     ::= StateDecl | Handler | Stmt
StateDecl     ::= "state" Ident "[" Ident ":" Keyspace "]" ":" Type
                  [ "init" Expr ]
                  [ "visible" VisLevel [ "fields" "{" Ident {"," Ident} "}" ] ]
Keyspace      ::= "Participant" | Ident          (* declared keyspaces *)
VisLevel      ::= "observer_safe" | "admin_debug"

RoleInstanceBlock ::= RolePath "[" Binder "]" "{" { Handler } "}"
Binder        ::= Ident                           (* e.g. self *)
Handler       ::= "when" Ident [ "(" [ Param {"," Param} ] ")" ]
                  [ "fails" FailName {"," FailName} ] Block
Block         ::= "{" { Stmt } "}"

Stmt          ::= Assign | Compound | Let | If | Return | Join
                | Grant | Require | Publish | PlaceBlock | Expr
Assign        ::= LValue "=" Expr
Compound      ::= LValue ("+=" | "-=") Expr
Let           ::= "let" Ident "=" Expr
If            ::= "if" Expr Block [ "else" Block ]
Return        ::= "return" Expr
Join          ::= "join" PlacePath "as" RolePath "via" PlacePath
Grant         ::= "grant" Ident "(" [ Expr {"," Expr} ] ")"
Require       ::= "require" Expr
Publish       ::= "publish" Expr [ "produces" "witness" Ident ]

LValue        ::= Path [ "[" Expr "]" ] { "." Ident }
Path          ::= Ident { "." Ident }

ChainDecl     ::= "chain" Ident ":" Type "=" OptionRef
                  { ">" OptionRef "@" "lineage" }
OptionRef     ::= Ident "on" LValue "cap" Ident "lease" Ident
                  [ "admit" Expr ]
```

Expression precedence (loosest→tightest): `or` < `and` < `not` <
comparisons `== != < <= > >=` (non-associative) < additive `+ -` <
multiplicative `* /` < unary `-` < postfix (index, field, call) < atoms
(literal, path, `(` Expr `)`, record literal `TypePath { f: e, ... }`).

## Brace disambiguation (normative, inherits LAB:specs/39)

1. `X { ... }` is a place block only when `X` resolves to a declared place
   path in item/statement context. 2. `X[b] { ... }` is a role-instance block
   only when `X` resolves to a declared role; bare `Role { ... }` is not.
3. `T { f: e }` is a record literal only when `T` resolves to a type in
   expression context. 4. Colliding place/role/type/value names that make a
   brace construct ambiguous are rejected (E-NAME-003). 5. `S[ ... ]` as
   place scope is rejected with E-PARSE-002; `[]` is indexing only.
6. Dynamic locus heads are not expressions; a future `at expr { ... }`
   requires a separate ADR.

## Notes

- `=` is assignment; `==` is equality (ADR-0008; the LAB FSV1 style
  `if seed = 1` is invalid here, E-PARSE-006).
- `fails` lists the failures the handler agrees to absorb; static containment
  per theory/03.
- ChainDecl: each `>` edge must carry `@ lineage` (edge-local annotation,
  theory/06). Missing annotation ⇒ E-DECL-001. This surface is OPEN-005.
