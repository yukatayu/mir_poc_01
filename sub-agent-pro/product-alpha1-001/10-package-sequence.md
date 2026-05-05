# Package sequence

## P-A1-25 product/public α-1 boundary recut

Purpose:

- Define product/public-ready α-1.
- Add specs/plans for product α-1.
- Reclassify current bounded workflow as not product/public-ready.
- Define U1 choices or alpha defaults.

Files:

- `specs/25-product-alpha1-public-boundary.md`
- `plan/50-product-alpha1-public-boundary-roadmap.md`
- update `progress.md`, `tasks.md`, `samples_progress.md`, `README.md`, `Documentation.md`.

Validation:

- docs validators, no behavior claim.

## P-A1-26 CLI and package schema

Purpose:

- Add `mirrorea-cli` or equivalent.
- Stabilize alpha package schema.
- Provide `check`, `run-local`, `demo` skeleton.

Validation:

- CLI tests.
- product demo package schema tests.

## P-A1-27 product demo same-session runtime

Purpose:

- Add product demo root.
- Connect check -> run -> host-I/O -> hot-plug -> observe.

Validation:

- product demo workflow passes.

## P-A1-28 message recovery + quiescent save

Purpose:

- Implement R2 quiescent save for controlled local/Docker scope.
- Add message failure/recovery typing/check/report.

Validation:

- quiescent save positive and negative.
- post-seal send reject/defer.
- no-inflight check.

## P-A1-29 product devtools viewer

Purpose:

- Build static viewer/local viewer.
- Render required panels.
- Add observer-safe leak tests.

Validation:

- viewer openability.
- devtools JSON structure.
- redaction tests.

## P-A1-30 native launch bundle

Purpose:

- `build-native-bundle`.
- Include binary/packages/viewer/reports/run script.
- Validate bundle.

Validation:

- bundle command works.
- native non-claims manifest.

## P-A1-31 product α-1 release candidate

Purpose:

- Full validation, docs, hands-on.
- Release report.
- Product/public α-1 statement with non-goals.

Validation:

- full release validation floor.
- clean worktree.
- commit/push.

## Autonomous continuation rule

Codex may continue from one package to the next if:

- previous package validation passed;
- no user decision blocker is hit;
- next package does not require final public ABI decisions beyond alpha defaults;
- report is committed and pushed.

Stop if:

- product/API decision is ambiguous;
- native execution policy would be broadened;
- distributed durable save/load would be claimed;
- final grammar/ABI would be frozen.
