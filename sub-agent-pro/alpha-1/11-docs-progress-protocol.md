# 11 — Docs and Progress Protocol

## 1. Progress semantics reset

`progress.md` must state:

- Current Stage A..F closeouts are evidence closeouts, not practical alpha-1 readiness.
- Practical alpha-1 readiness has its own percentages.
- Public product readiness / U1 is separate.

## 2. Recommended progress tables

### Current evidence closeout table

Keep existing Stage A..F evidence in a table named:

```text
Current-scope evidence closeout map
```

### Practical alpha-1 readiness table

Add a new table:

```text
Practical Alpha-1 Readiness
```

Rows:

- PA1-0 rebaseline
- PA1-1 source front-door
- PA1-2 typed IR/checker
- PA1-3 runtime plan execution
- PA1-4 package/hot-plug API
- PA1-5 transport/Docker product E2E
- PA1-6 devtools/viewer
- PA1-7 local save/load
- PA1-8 product prototype

## 3. samples_progress rules

- Existing `samples/alpha/` rows remain evidence rows.
- New `samples/practical-alpha1/` rows may be introduced when actual samples exist.
- A practical sample only counts if it goes through front-door/toolchain.

## 4. tasks.md rules

- Add a clear `P-A1` lane.
- Keep `U1` separate.
- If no safe package exists, state exact blocker.
- Avoid saying “alpha complete” without practical qualifier.

## 5. Documentation.md / README

Update short current status so a reader immediately sees:

- current-scope alpha evidence is closed
- practical alpha-1 is not yet done
- next work is toolchain/product-readiness

## 6. Report naming

Use package names such as:

- `P-A1-00 practical alpha rebaseline`
- `P-A1-01 alpha source front-door`
- etc.

Do not reuse `P-A0` for practical alpha-1 package line.
