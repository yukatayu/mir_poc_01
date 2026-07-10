# MIR Codex agent upgrade

## Goal

Upgrade the repository-local agent definitions to the GPT-5.5/GPT-5.6 model
policy, preserve their existing reasoning effort and filesystem permissions,
and add a registered strategic planner.

## Scope

- Update `code_mapper`, `docs_researcher`, `eval_runner`, `implementer`,
  `reviewer`, and `test_author` from `gpt-5.4` to `gpt-5.5`.
- Update `status_reporter` from `gpt-5.4-mini` to `gpt-5.5`; no
  `gpt-5.5-mini` is available in the local model catalogue, and the user
  approved matching the stronger global `status_reporter` setting.
- Preserve every existing `model_reasoning_effort` and `sandbox_mode` value.
- Add `agents/planner.toml` with `gpt-5.6-sol`, `max` effort, and read-only
  filesystem access.
- Add `.codex/config.toml` entries that explicitly register all existing agent
  definitions plus the new planner, without overriding the global default
  model, effort, approval policy, or sandbox policy.

## Planner behavior

The planner reads the repository source hierarchy defined by `AGENTS.md`,
especially the canonical `mirrorea_canon/` documents before LAB status
material. It returns an evidence-based task sequence, dependencies, validation
criteria, risks, and decision checkpoints to its parent. It does not edit
files, run state-changing commands, or override user instructions.

## Verification

Parse the project config and all eight agent TOML files, assert the complete
model/effort/sandbox mapping, and run a strict Codex configuration load from
the repository. Separately run `gpt-5.6-sol` at `max` effort in read-only mode
to confirm the planner's model configuration is available.
