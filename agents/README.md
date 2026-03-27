# Agent configuration notes

These TOML files are **project-local guidance stubs** intended to work with a Codex setup that points each helper agent to `./agents/<name>.toml`.

Because Codex host implementations may vary, these files intentionally use a conservative structure:
- human-readable TOML,
- an `instructions` multi-line string,
- lightweight metadata.

If your Codex installation expects a different schema, adapt these files while preserving the intent.
