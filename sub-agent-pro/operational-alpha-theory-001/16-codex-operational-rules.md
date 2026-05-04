# 16 — Codex operational rules

## source hierarchy

Respect hierarchy:

```text
specs/ > plan/ > progress.md/tasks.md/samples_progress.md > docs/reports/ > README/Documentation > tmp_faq
```

FAQ files are helper memos, not normative sources.

## no overclaim

Do not write:

- practical alpha-1 complete
- α-0.5 complete
- α-0.8 complete
- public alpha complete
- final API / ABI complete

unless the operational completion conditions are met.

## category vocabulary

Use:

- evidence closeout
- first-floor closeout
- operational readiness
- product/public readiness

## not allowed

- Promote `samples/alpha/` or `samples/practical-alpha1/` to active canonical runnable root unless explicitly validated and documented.
- Add parser/runtime claims where only JSON loader or helper runner exists.
- Treat exact report bundle as runtime execution.
- Treat preview bundle as same-session product runtime.
- Treat local save/load as distributed durable save/load.
- Treat Docker/local TCP as production WAN.
- Treat signature as semantic safety.
- Treat auth/rate-limit as transparent overlay without declared failure/contract update.
- Treat debug/telemetry as untyped leak.

## branch / working tree

Work on the current repository branch unless user explicitly asks for a docs-only explanation branch. Check dirty state before editing:

```bash
git status --short
```

If unrelated user changes exist, do not overwrite them.

## reports

Every package closeout needs report.

Do not leave report metadata as TBD after push.

## validations

Run validation before commit. If skipped, write reason in report.

## sub-agent behavior

If a sub-agent returns findings, address them or record why not. If it times out, do not count it as approval.

