# plan/152 - Discord notification file inputs

## status

LAB repository-memory / notification operation hardening.

This note records the P105 change that added file-based message inputs to the
repo-scoped `discord-report` notifier. It does not change project phase, canon
state, security status, webhook storage, or notification delivery guarantees.

## trigger

During P104 closeout, a Discord notification command used inline shell text
containing a backtick-wrapped commit hash. The shell interpreted the backticks
as command substitution before `discord_notify.py` received the summary,
producing a local `/usr/bin/bash: ... command not found` message and risking a
truncated notification body.

The root cause was not Discord payload construction. It was the shell boundary
around inline `--summary` text.

## decision

`discord_notify.py` now accepts UTF-8 text-file inputs:

- `--summary-file` for `progress`, `complete`, `test`, and `check` parser
  surfaces that share the common message options.
- `--next-step-file` for `progress`.

When a notification summary or next-step text contains backticks, quotes,
newlines, shell metacharacters, or long commit lists, agents should write the
message to a temporary text file and pass the file path instead of placing the
message directly inside a shell command argument.

Short plain-text summaries may still use inline `--summary` / `--next-step`.

## evidence

P105 added `scripts/tests/test_discord_notify_skill.py`, which imports the
repo-scoped notifier, mocks the webhook send, and checks that file contents are
used literally in the outgoing payload:

- `test_progress_accepts_summary_and_next_step_files`
- `test_complete_accepts_summary_file`

The RED run failed with argparse `unrecognized arguments` for
`--summary-file` / `--next-step-file`. The GREEN run passed after adding the
file options and message-resolution helper.

## non-claims

This note does not:

- change Discord webhook URL storage;
- claim delivery reliability beyond best effort;
- claim broad shell-safety for arbitrary commands;
- change notification rate limiting;
- change task begin / progress / complete semantics;
- alter commit / push policy;
- edit canon;
- move any phase, gate, proof, conformance, runtime, sample, or workflow
  readiness status.

## next use

Use file-based message inputs whenever notification text includes Markdown
inline code, commit hashes wrapped in backticks, quotes, multiline summaries,
or generated text whose shell quoting would be tedious to audit.
