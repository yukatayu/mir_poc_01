# 19 — Package Sequence

## P-SURF-00 — Documentation / roadmap rebaseline

Close when specs/plans/progress/tasks updated.

## P-SURF-01 — Surface place-scope parser

Implement `S { ... }`, role instance blocks, `state`, `when`, `join` parsing.
Reject `S[ ... ]`.

## P-SURF-02 — Indexed-state semantics

Implement AST/IR/checker support for:

```mir
S { state player[p: Participant]: Player }
```

## P-SURF-03 — Surface-to-Core elaboration

Generate Core Mir for cross-locus read/write.

## P-SURF-04 — Auto communication

Generate MessageEnvelope / publish / observe and failure row obligations.

## P-SURF-05 — Role admission

Implement role claim, admission request, membership/capability grant.

## P-SURF-06 — Source patch hot-plug

Implement `patch-source` pipeline.

## P-SURF-07 — Source operational suite

Create source-first WorldCore / MembershipChat / Sugoroku / Portal / Shard roots.

## P-SURF-08 — Devtools and diagnostics

Show Surface source, Core IR, generated communication, indexed state, role admission, patch lifecycle.

## P-SURF-99 — final audit

Run full validation and close Surface Mir alpha chain.
