---
id: meta/proposal-007
status: L2-working
maturity: draft
depends_on: [adr/ADR-0014, meta/proposal-006]
summary: WRK の L3 evidence を append-only の明示 commit list と DAG history audit で帰属させる運用精密化。既存の standing boundary・L2 fail-close・reserved surface は変更しない。
open_items: []
---

# PROPOSAL-007 - WRK evidence-commit integrity

> Adopted as an L2 operating refinement within the owner's standing bounded
> autonomy direction of 2026-07-21. It changes neither an L0/L1 theory
> decision nor the authority needed for L2 activation. Its effective boundary
> remains ADR-0014; this memo and `working/README.md` make the existing L3
> retained-evidence condition mechanically auditable.

## Problem

Assigning every commit reachable after a WRK registration to that WRK is not a
sound provenance rule. It incorrectly attributes an independent candidate or
a merged side branch to each active record, and a per-path history scan misses
temporary malformed or restored states. Conversely, an unlisted source commit
must not silently become retained evidence for a WRK.

## Operating refinement

Each WRK records `Evidence commits: none` until it retains evidence. Thereafter
the field is an append-only, comma-separated list of unique full 40-hex Git
commit IDs. A listed commit belongs to exactly one WRK and must be reachable,
strictly follow that record's one-parent L3 registration, and have a local
delta confined to the record's already-declared LAB locations plus exact
operational metadata. An evidence artifact snapshot must name one of those
listed commits.

The validator audits every HEAD-reachable working-annex tree in parent-before-
child order. It preserves each record's ID, final path, three pre-registration
sections, declared LAB locations, registration commit, and append-only evidence
list. Merge validation uses the combined local result, not per-parent `-m`
diffs, so unchanged imported paths are not falsely attributed as authored
evidence. Authoritative evidence validation additionally requires a clean,
disposable worktree.

## Limits and non-effects

This is a bounded Git provenance check. It does not establish that an unlisted
experiment was never run, that an external tool or network response was
authentic, that a source file is truly not a helper family, or that rewritten-
away history was reviewed. It does not activate L2, alter review keys, amend
ADR-0014's reserved boundaries, change `theory/11`, create a new evidence lane,
or create a Gate, Phase, proof, implementation, or public claim.

## Alternatives rejected

1. Attribute every descendant commit: rejects independent work and mishandles
   merge imports.
2. Trust commit-message trailers: the ownership declaration would be outside
   the canonical record being reviewed and is easier to omit or reinterpret.
3. Permit a broad control-path allowlist: it would turn helpers, schemas, or
   policy files into accidental research authorization.
