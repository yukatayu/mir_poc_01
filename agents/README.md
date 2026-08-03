# Agent configuration notes

These TOML files are the project-local custom-agent configurations registered by `.codex/config.toml`.

They target the installed Codex custom-agent schema and use:
- `name`, `description`, and `developer_instructions`,
- an explicit supported `sandbox_mode`, and
- an explicit `approval_policy` default for the role.

Codex live parent-session permission overrides are reapplied to children. These
files therefore state role intent and least-privilege defaults; the parent must
still delegate exact write ownership. Run `python3 scripts/validate_agent_configs.py`
and `codex --strict-config -C . --help` after changes.
