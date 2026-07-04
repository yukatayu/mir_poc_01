# plan/151 - Discord webhook secret validator guard

## status

LAB repository-memory / validator guard hardening.

This note records the package that added a concrete Discord webhook URL leak
guard to `scripts/validate_docs.py`. It does not change Discord notification
behavior and does not store webhook credentials.

## purpose

The repository uses a local Discord webhook configuration for progress
notifications. That configuration is intentionally local and must not become
tracked repository state.

This package adds a docs validation guard for concrete Discord webhook URLs so
future reports, docs, or source files do not accidentally commit the live
webhook URL.

## guard shape

`scripts/validate_docs.py` now scans candidate repository files for concrete
Discord webhook URL shape. When it finds a match, it reports only:

- relative path;
- line number;
- the label `concrete Discord webhook URL`.

It intentionally does not print the matched URL. Generic prose such as
operation notes or a command name is not the target; the guard is for concrete
URL-shaped credentials.

## TDD evidence

The package added a failing test before production code:

- `test_main_rejects_concrete_discord_webhook_without_printing_secret`

The RED run failed because `validate_docs.main()` still returned `0` for a
temporary file containing a fake concrete webhook URL. The GREEN run passed
after adding the secret-safe guard.

## non-claims

This note does not:

- commit or reveal any webhook URL;
- scan external systems or Discord itself;
- replace local `.codex-discord/config.local.json` handling;
- validate all possible secret formats;
- claim security completeness;
- change notification semantics;
- change canon, phase, gate, proof, conformance, runtime, sample, or workflow
  readiness.

## next use

Use this note when updating Discord notification operations, report templates,
or docs validation. Keep concrete credentials out of tracked files and out of
validator output.
