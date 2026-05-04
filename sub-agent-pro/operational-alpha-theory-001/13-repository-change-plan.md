# 13 — repository change plan

## files to add

Normative specs:

```text
specs/19-verification-stratification.md
specs/20-cut-save-load-semantics.md
specs/21-auth-layer-algebra.md
specs/22-observability-devtools-semantics.md
specs/23-typed-external-host-boundary.md
specs/24-operational-alpha05-alpha08-readiness.md
```

Repository memory:

```text
plan/45-operational-alpha05-roadmap.md
plan/46-operational-alpha08-roadmap.md
plan/47-operational-alpha09-devtools-roadmap.md
plan/48-theory-freeze-proof-obligations.md
plan/49-host-io-and-session-runtime-roadmap.md
```

Report:

```text
docs/reports/NNNN-p-a1-18-operational-alpha-theory-freeze.md
```

## files to update

At minimum:

```text
specs/00-document-map.md
Documentation.md
README.md
progress.md
tasks.md
samples_progress.md
samples/practical-alpha1/README.md
scripts/README.md
```

Optional if source hierarchy requires:

```text
plan/00-index.md
samples/README.md
docs/hands_on/README.md
docs/research_abstract/README.md
```

## wording requirements

Use the four categories:

```text
evidence closeout
first-floor closeout
operational-layer-ready
product/public-ready
```

Avoid naked `complete` where category is unclear.

## progress update rule

In `progress.md` / `samples_progress.md`, do not lower or rewrite historical facts destructively. Add a clear current interpretation:

- previous Stage A..F 100% = current-scope evidence closeout
- practical alpha-1 first floors = useful but not product-ready
- α-0.5 / α-0.8 operational readiness = newly frozen target, not yet complete unless workflow exists

## report requirements

Report must include:

- which specs/plan files were added
- which decisions were fixed
- what remains intentionally unresolved
- which validations ran
- sub-agent findings
- commit/push status

## source hierarchy caution

If `check_source_hierarchy.py` fails because new specs/plan files are not in the source map, update the relevant hierarchy/index file rather than deleting the new docs.

