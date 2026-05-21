# 17 — Sub-agent and Autonomy Protocol

## Use agents

Codex must inspect `agents/` and use available sub-agents or equivalent focused reviews.

At minimum use perspectives:

1. language/parser/type reviewer
2. runtime/effect/cut reviewer
3. projection/backend reviewer
4. posegraph/VR semantic reviewer
5. security/auth/provider reviewer
6. docs/status/release reviewer

If concrete named agents are available, assign accordingly.

## Autonomy rule

Do not stop after each package asking for confirmation.

Once execution prompt is given:

- proceed through milestone sequence
- update `progress.md` after large milestones
- write reports
- validate
- commit and push
- continue to next package

Stop only if:

- repository cannot build due to external missing dependency that cannot be skipped
- safety/security issue prevents progress
- irreconcilable spec contradiction is found
- Git push fails repeatedly

Even then, write a report and precise blocker.

## Commit rule

After each package:

```bash
git status --short
git add <files>
git commit --no-gpg-sign -m "<package>: <summary>"
git push
```

## Progress update rule

At each large milestone, `progress.md` must show:

- current position
- exact milestone status
- runnable commands
- remaining gap
- non-claims

`tasks.md` must show:

- next promoted package
- ordered self-driven packages
- user decision items
- research-discovery items

## No overclaim

Never claim:

- final public grammar unless delivered
- final ABI unless delivered
- LLVM/backend unless delivered
- engine provider execution unless delivered
- production WAN/federation unless delivered
- distributed durable save/load unless delivered
