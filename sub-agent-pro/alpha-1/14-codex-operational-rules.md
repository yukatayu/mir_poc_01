# 14 — Codex Operational Rules

## 1. Reading order

Always read:

1. `README.md`
2. `Documentation.md`
3. `AGENTS.md`
4. `progress.md`
5. `tasks.md`
6. `samples_progress.md`
7. relevant `specs/`
8. relevant `plan/`
9. latest relevant reports
10. this handoff

## 2. Do not ask unless necessary

If a next step can be done without user decision, do it. If user decision is necessary, record exactly what and why.

## 3. Validation honesty

Never claim success for skipped validation. Record skipped validations and reasons.

## 4. Reports

Every non-trivial package must write a new report.

## 5. Git

Use:

```bash
git commit --no-gpg-sign -m "..."
git push
```

Push every package unless user says otherwise.

## 6. Discord notify

If repo-scoped discord skill is in use, run begin before edits as required by repo policy.

## 7. Do not destroy user work

Check `git status --short` before edits. If unrelated dirty files exist, avoid touching them unless necessary and document.

## 8. Storage

For heavy tasks, check disk/memory and use external workdir.

## 9. Style

Use precise Japanese docs. Mark:

- 決定済み
- current alpha-local
- helper-local
- runtime-private
- final public deferred
- planned-only

## 10. Exit condition

A package is closed only when:

- docs updated
- samples/dashboard updated if relevant
- report added
- validations run
- commit/push done
- worktree clean or dirty state explained
