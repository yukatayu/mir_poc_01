# 13 — Codex operational rules

## 1. Do not ask unnecessary clarification

This package contains enough decisions to proceed. If an ambiguity remains, choose the conservative non-overclaiming path.

## 2. Maintain source hierarchy

- normative decisions go to `specs/`
- roadmap memory goes to `plan/`
- current snapshot goes to `progress.md` / `tasks.md` / `samples_progress.md`
- evidence goes to `docs/reports/`
- FAQ / handoff stays helper-level only

## 3. Keep categories separate

Do not mix:

- `samples/clean-near-end/` active current-L2 suite
- `samples/alpha/` alpha-0 evidence root
- `samples/practical-alpha1/` first-floor fixture root
- `samples/product-alpha1/demo/` release-candidate demo
- `samples/product-alpha1/operational/` new operational sample suite
- `samples/not_implemented/` planned residual root

## 4. Commit/push discipline

Always end with:

```bash
git status --short
git add <changed files>
git commit --no-gpg-sign -m "mirrorea: add operational product sample suite"
git push -u origin feature/operational-product-sample-001
```

If report metadata needs follow-up, make a separate docs-only commit.

## 5. Report discipline

Create report:

```text
docs/reports/<next>-p-ops-01-operational-product-sample-suite.md
```

Include:

- objective
- scope
- docs consulted
- actions
- files changed
- commands run
- evidence
- skipped validations
- non-claims
- reviewer findings
- commit/push status

## 6. Conservative claim language

Use:

```text
non-final
alpha-local
product alpha operational sample
native host launch bundle
projection intent
planned future portal/spatial boundary
```

Do not use unless actually true:

```text
final grammar
final ABI
LLVM backend
server/client native binaries emitted
WAN federation
distributed durable save/load
arbitrary native execution
final viewer API
```

## 7. If implementation gets too large

Prioritize:

1. spec / plan / sample root / docs
2. executable package check path
3. local run path
4. attach/devtools/save path
5. Docker/native bundle path
6. portal/spatial future skeleton

If time is limited, leave future skeletons as clearly planned-only, not half-implemented.

## 8. Do not delete existing demo behavior

Do not break `samples/product-alpha1/demo/` or existing release-check.
The operational sample suite should coexist with demo.
