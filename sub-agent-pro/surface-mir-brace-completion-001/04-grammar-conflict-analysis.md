# 04 — Grammar Conflict Analysis

## 1. Potential conflict: Place block vs record literal

```mir
S { ... }
Player { hp: 100 }
```

Resolution:

- Place block is a statement/item construct.
- Record literal is an expression construct.
- Place path must resolve in Place/Role namespace.
- Record literal head must resolve in Type namespace.
- Alpha disallows same name in both namespaces in the same scope.

## 2. Potential conflict: Place block vs function block

```mir
fn f() { ... }
S { ... }
```

Resolution:

- `fn` introduces function body.
- bare path + block is place/role block only if path resolves as place/role.

## 3. Potential conflict: Indexed state vs array indexing

```mir
player[p]
arr[i]
```

Resolution:

Both are expression indexing. Type checker distinguishes:

```text
IndexedState<Key, T>
Array<T>
Map<K, V>
```

This is acceptable because both are value-level access.

## 4. Potential conflict: Role instance block vs index expression

```mir
Participant[self] { ... }
player[self]
```

Resolution:

- `Participant[self] { ... }` is role-instance block statement.
- `player[self]` is expression.
- Body block after index expression is allowed only if the indexed expression resolves to RoleInstance / PlaceInstance locus.
- In alpha, only declared Role path followed by `[expr] { block }` is accepted as role block.
- Arbitrary `expr[expr] { block }` is not accepted.

## 5. Dynamic place expression

If future code wants dynamic place scope:

```mir
let owner = resolve_owner(object)
at owner { ... }
```

For alpha brace syntax, we do not use `owner { ... }` unless `owner` is statically known place/role path. If dynamic place block is needed, introduce explicit `at expr { ... }` as expert syntax later.

## 6. Why no `S[]` sugar

No sugar because:

- It visually conflicts with arrays.
- It makes tutorials inconsistent.
- It burdens parser/IDE/highlighter.
- It encourages two equivalent styles too early.

Single style wins:

```mir
S { ... }
```

## 7. Equality / assignment

Decide:

```text
assignment: =
equality: ==
result binding: <-
```

Examples:

```mir
let y: Int64 = x + 1
require phase(game) == Running
draw <- perform roll_dice via authority_rng
```

## 8. Parsing stages

1. Parse token stream into preliminary AST.
2. Resolve namespaces: type / place-role / value / capability-effect.
3. Disambiguate brace constructs.
4. Produce diagnostics if ambiguous.
5. Lower to typed AST.
6. Elaborate to Core Mir.
