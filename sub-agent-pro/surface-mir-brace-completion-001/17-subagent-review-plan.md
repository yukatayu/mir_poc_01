# 17 — Sub-Agent Review Plan

Use agents under `agents/` if available. Do not block forever if unavailable.

## Review roles

1. Syntax/parser reviewer
   - Check `S { ... }` vs record literal ambiguity.
   - Confirm no `S[]` sugar.

2. Type/elaboration reviewer
   - Check Surface -> Core obligations.
   - Verify failure row completion.

3. Indexed-state reviewer
   - Storage owner / keyspace separation.
   - Join/leave/tombstone/compaction.

4. Runtime/hot-plug reviewer
   - Source patch pipeline.
   - Activation cut.
   - No direct eval.

5. Security/auth reviewer
   - role claim != authority.
   - admission grant / capability grant.
   - spoofing non-benefit.

6. PoseGraph/backend reviewer
   - no-split-frame consistency.
   - backend semantic owner boundary.

7. Docs/status reviewer
   - progress/tasks replacement.
   - non-claims.

## If agents unavailable

Record in report:

```text
No addressable sub-agent was available; performed local review for each perspective.
```
