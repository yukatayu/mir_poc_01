# 18 — done criteria

## P-A1-18 is done when

- New specs 19..24 exist and are linked.
- New plan 45..49 exists and is linked.
- `Documentation.md`, `progress.md`, `tasks.md`, `samples_progress.md`, `README.md`, and `samples/practical-alpha1/README.md` distinguish evidence / first-floor / operational readiness.
- α-0.5 / α-0.8 / α-0.9 operational conditions are written.
- Host-I/O minimal demo requirement is written.
- Verification stratification is written.
- Cut/save-load theory is written.
- Auth/layer algebra is written.
- Observability/devtools theory is written.
- Report exists with validation and reviewer findings.
- Validation passes.
- Commit and push are done.

## P-A1-18 is not done if

- Any new document claims practical alpha completion.
- α-0.5 / α-0.8 are still defined by artifact closeout only.
- Host-I/O direct execution gap is not mentioned.
- Same-session runtime gap is not mentioned.
- Source hierarchy validation fails.
- Report is missing required sections.
- Commit/push not completed.

## next reopen candidates

### Candidate A — P-A1-19 session runtime carrier

Goal:

```text
check -> runtime plan -> local runtime -> observe -> save/load
```

with one session carrier.

### Candidate B — P-A1-20 typed external direct execution

Goal:

```text
host payload -> typed external request -> transform -> output / receipt -> view
```

`EchoText` or `AddOne`.

### Candidate C — P-A1-21 same-session hot-plug

Goal:

```text
session -> attach debug/auth/rate-limit/object -> observe behavior/lifecycle change
```

## choose next package rule

After P-A1-18, do not automatically promote a next package unless it is narrow and validation-backed.

