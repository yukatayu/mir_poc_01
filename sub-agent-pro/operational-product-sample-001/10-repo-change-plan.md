# 10 — repository change plan

## 1. Specs

Add:

```text
specs/26-operational-product-sample-suite.md
specs/27-spatial-portal-and-shard-extension-boundary.md
```

Update:

```text
specs/00-document-map.md
```

## 2. Plans

Add:

```text
plan/51-operational-product-sample-roadmap.md
plan/52-portal-spatial-world-roadmap.md
```

Update:

```text
plan/00-index.md
plan/50-product-alpha1-public-boundary-roadmap.md
```

## 3. Samples

Add:

```text
samples/product-alpha1/operational/
```

Update:

```text
samples/README.md
samples/product-alpha1/README.md
samples_progress.md
```

## 4. Scripts

Add if needed:

```text
scripts/operational_product_samples.py
scripts/tests/test_operational_product_samples.py
```

Update:

```text
scripts/README.md
scripts/check_source_hierarchy.py
scripts/validate_docs.py
```

Only add script tests if the script exists.
If all functionality is direct CLI and docs-only, skip script but document why.

## 5. Docs

Add:

```text
docs/hands_on/operational_product_sample_01.md
docs/research_abstract/operational_product_sample_01.md
```

Update:

```text
docs/hands_on/README.md
docs/research_abstract/README.md
Documentation.md
README.md
progress.md
tasks.md
```

## 6. Reports

Add:

```text
docs/reports/<next>-p-ops-01-operational-product-sample-suite.md
```

Report must include:

- objective
- scope and assumptions
- docs consulted
- actions taken
- files changed
- commands run
- evidence/results
- non-claims
- reviewer findings
- skipped validations
- commit/push status

## 7. Branch and commit

Branch:

```text
feature/operational-product-sample-001
```

Commit message:

```text
mirrorea: add operational product sample suite
```

Push:

```bash
git push -u origin feature/operational-product-sample-001
```
