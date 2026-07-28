# WRK-0029 C0-B - noncircular domain-staging conditional lemma

## Role and evidence boundary

This is a **LAB** result artifact for `working/WRK-0029`. It is ordinary
Markdown evidence, pinned to its owning commit after retention. Canon remains
normative. The graph below is a conditional mathematical object, not a Canon
grammar, static judgment, parser pipeline, implementation plan, or a claim
that the current documents define its nodes or edges.

## Pinned source observations

| Opaque label | Source-local observation | Explicit non-claim |
| --- | --- | --- |
| `Lex` | `spec/01` describes lexical structure and module text. | No lexical set, token stream, or accepted source class is defined here. |
| `Parse` | `spec/02` displays the Surface EBNF over lexical spellings. | No parser algorithm, AST, parse result, or exact accepted domain is inferred. |
| `Static` | `spec/03` names surface-visible obligations and binds typing/elaboration authority to theory/01--03. | No complete static judgment, ordering, or satisfaction relation is supplied. |
| `WS` | BND-001 takes a well-scoped Surface item as input before its terminal branch. | `WellScoped` is not defined, and no relation equates it with parsing or static success. |
| `Terminal` | BND-001 describes a tuple-or-Diagnostic branch; P008 separately records that totality's exact domain and carrier remain open. | No outcome, equality, totality proof, or Diagnostic assignment is supplied. |

P004 and P015 remain bounded directions rather than current grammar/static
rules. P008 does not select the statement domain, `WellScoped` predicate, or
Diagnostic carrier. These qualifiers are preserved from WRK-0028's source-local
manifest and the current pinned sources.

## Conditional lemma

Let `G` be the finite directed graph with opaque nodes

```text
{ Lex, Parse, Static, WS, Terminal }
```

and, **only as the lemma hypothesis**, edges

```text
Lex -> Parse -> Static -> WS -> Terminal
```

with no edge out of `Terminal` and no input node defined by elaboration
success, an outcome tuple, or a Diagnostic. Give the nodes ranks `0, 1, 2, 3,
4` in the order shown. Every hypothesized edge strictly increases rank; hence
`G` is acyclic.

This is the complete retained result. It does not assert that Canon supplies
the four input-to-input edges, that every input belongs to the next role, or
that `Terminal` is a concrete result carrier. It only shows that an eventual
front-end design can keep those roles noncircular if it proves and adopts the
stated orientation separately.

## Falsifier audit

The registered source marker was absent, all nine pinned source files existed,
every SHA-256 matched WRK-0029, and `git diff --check` passed. No registered
falsifier occurred because this artifact did not need a role member, grammar
production, acceptance/rejection classification, `WellScoped` definition,
outcome relation, Diagnostic family, Core/judgment, source reconciliation, or
new tool/interface.

An attempt to add an edge from `Terminal` to an input role, or to assert that
any candidate edge is a current Canon dependency, would be outside this result
and triggers the pre-registered freeze/escalation path. It must not be repaired
inside this artifact.

## Consequences and non-effects

The result narrows one design hygiene requirement: do not use an elaboration
outcome or Diagnostic to define the input role that the same elaboration step
requires. It does not decide whether the actual project should use this graph,
what each role contains, how an implementation infers any role, or what a user
may omit in `.mir` source. Ergonomic inference remains deferred to C7 and is
admissible only after explicit semantics uniquely reconstruct omitted facts.

No grammar, static semantics, `WellScoped` predicate, Core form, Diagnostic
carrier, totality/equality relation, OBL/theory status, Gate/Phase, runtime,
wire, API, public behavior, parser/checker, or implementation readiness changes.
