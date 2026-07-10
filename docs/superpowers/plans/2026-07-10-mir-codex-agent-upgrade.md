# MIR Codex Agent Upgrade Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Upgrade the repository-local Codex agents, explicitly register them in project configuration, and add a read-only strategic planner.

**Architecture:** Existing role definitions remain in `agents/` to preserve their documented paths. `.codex/config.toml` becomes the project-level registry, mapping each named role to its existing TOML file; the global Codex defaults remain unmodified.

**Tech Stack:** Codex project configuration (TOML), Python 3 `tomllib`, Codex CLI strict-config validation.

---

### Task 1: Establish a failing configuration assertion

**Files:**
- Create: `/tmp/verify_mir_codex_agent_upgrade.py`
- Read: `/home/codex/dev/mir_poc_01/agents/*.toml`
- Read: `/home/codex/dev/mir_poc_01/.codex/config.toml`

- [ ] **Step 1: Write the failing assertion script**

```python
from pathlib import Path
import tomllib

root = Path('/home/codex/dev/mir_poc_01')
expected = {
    'code_mapper': ('gpt-5.5', 'medium', 'read-only'),
    'docs_researcher': ('gpt-5.5', 'high', 'read-only'),
    'eval_runner': ('gpt-5.5', 'high', 'workspace-write'),
    'implementer': ('gpt-5.5', 'xhigh', 'workspace-write'),
    'reviewer': ('gpt-5.5', 'high', 'read-only'),
    'status_reporter': ('gpt-5.5', 'medium', 'workspace-write'),
    'test_author': ('gpt-5.5', 'high', 'workspace-write'),
    'planner': ('gpt-5.6-sol', 'max', 'read-only'),
}

config_path = root / '.codex' / 'config.toml'
assert config_path.is_file(), config_path
config = tomllib.loads(config_path.read_text())
for name, (model, effort, sandbox) in expected.items():
    agent = tomllib.loads((root / 'agents' / f'{name}.toml').read_text())
    assert agent['model'] == model, (name, agent['model'], model)
    assert agent['model_reasoning_effort'] == effort, (name, agent['model_reasoning_effort'], effort)
    assert agent['sandbox_mode'] == sandbox, (name, agent['sandbox_mode'], sandbox)
    assert config['agents'][name]['config_file'] == f'../agents/{name}.toml'
print('MIR_AGENT_CONFIG_ASSERTIONS_OK')
```

- [ ] **Step 2: Run the assertion before editing**

Run: `python3 /tmp/verify_mir_codex_agent_upgrade.py`

Expected: assertion failure naming `.codex/config.toml`, because it does not yet exist.

### Task 2: Upgrade model strings and add the planner

**Files:**
- Modify: `/home/codex/dev/mir_poc_01/agents/code_mapper.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/docs_researcher.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/eval_runner.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/implementer.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/reviewer.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/status_reporter.toml:3`
- Modify: `/home/codex/dev/mir_poc_01/agents/test_author.toml:3`
- Create: `/home/codex/dev/mir_poc_01/agents/planner.toml`

- [ ] **Step 1: Replace models while preserving effort and sandbox settings**

```toml
# code_mapper, docs_researcher, eval_runner, implementer, reviewer, test_author
model = "gpt-5.5"

# status_reporter; this project has no gpt-5.5-mini model available
model = "gpt-5.5"
```

- [ ] **Step 2: Add the read-only planner definition**

```toml
name = "planner"
description = "Plans repository-wide work sequencing, dependencies, evidence gates, and decision checkpoints for the parent agent."
model = "gpt-5.6-sol"
model_reasoning_effort = "max"
sandbox_mode = "read-only"

developer_instructions = """
Act as the parent agent's strategic planning partner for this repository.

Read the source hierarchy in AGENTS.md. Begin with mirrorea_canon/ for
normative direction, then use README.md, Documentation.md, progress.md,
tasks.md, samples_progress.md, plan/, specs/, and reports/ as LAB evidence
when relevant.

Return:
1. the intended outcome and acceptance criteria;
2. ordered work phases with dependencies and ownership;
3. the source evidence and validation required for each phase;
4. risks, assumptions, unresolved questions, and decision checkpoints; and
5. the recommended next action.

Do not edit files, run state-changing commands, or override explicit user
instructions. Make uncertainty and blockers explicit. The parent agent owns
task assignment, plan updates, and all changes.
"""
```

### Task 3: Register every project agent

**Files:**
- Create: `/home/codex/dev/mir_poc_01/.codex/config.toml`

- [ ] **Step 1: Create the named-agent registry**

```toml
[agents.code_mapper]
description = "Read-only codebase mapper."
config_file = "../agents/code_mapper.toml"

[agents.docs_researcher]
description = "Read-only external documentation verifier."
config_file = "../agents/docs_researcher.toml"

[agents.eval_runner]
description = "Verification-only command runner."
config_file = "../agents/eval_runner.toml"

[agents.implementer]
description = "Production-source implementation writer."
config_file = "../agents/implementer.toml"

[agents.reviewer]
description = "Read-only correctness and regression reviewer."
config_file = "../agents/reviewer.toml"

[agents.status_reporter]
description = "Documentation and status-maintenance writer."
config_file = "../agents/status_reporter.toml"

[agents.test_author]
description = "Test-only author."
config_file = "../agents/test_author.toml"

[agents.planner]
description = "Read-only strategic planner for task sequencing and plan maintenance."
config_file = "../agents/planner.toml"
```

- [ ] **Step 2: Run the configuration assertion**

Run: `python3 /tmp/verify_mir_codex_agent_upgrade.py`

Expected: `MIR_AGENT_CONFIG_ASSERTIONS_OK`.

### Task 4: Verify Codex configuration loading and model availability

**Files:**
- Read: `/home/codex/dev/mir_poc_01/.codex/config.toml`
- Read: `/home/codex/dev/mir_poc_01/agents/planner.toml`

- [ ] **Step 1: Strict-load the project configuration**

Run:

```bash
codex exec --strict-config --ephemeral --skip-git-repo-check --sandbox read-only -C /home/codex/dev/mir_poc_01 'Return exactly MIR_PROJECT_CONFIG_OK and do not modify files.'
```

Expected: output contains `MIR_PROJECT_CONFIG_OK`.

- [ ] **Step 2: Smoke-test the planner model and effort**

Run:

```bash
codex exec --strict-config --ephemeral --skip-git-repo-check --sandbox read-only --model gpt-5.6-sol -c 'model_reasoning_effort="max"' -C /home/codex/dev/mir_poc_01 'Return exactly MIR_PLANNER_MODEL_OK and do not modify files.'
```

Expected: output contains `MIR_PLANNER_MODEL_OK`.

### Task 5: Record the repository-local change

**Files:**
- Create: `/home/codex/dev/mir_poc_01/docs/reports/2026-07-10-codex-agent-upgrade.md`

- [ ] **Step 1: Create a completion report**

Create the report with exactly these sections, filling the command-output
sentences from the completed validation steps:

```markdown
# Codex agent upgrade — 2026-07-10

## Objective

Upgrade the repository-local agent model mapping, add `planner`, and register all eight agents in `.codex/config.toml`.

## Start state

The seven existing role files were under `agents/`; `.codex/config.toml` and `agents/planner.toml` did not exist.

## Changes

- Six `gpt-5.4` roles and `status_reporter` were set to `gpt-5.5` with their prior effort and sandbox values unchanged.
- `planner` was added as `gpt-5.6-sol` / `max` / `read-only`.
- `.codex/config.toml` registered all eight role files.

## Validation

Include the assertion-script result, the strict-load result, and the planner model-and-effort smoke-test result verbatim.

## Repository-status documents

`plan/`, `Documentation.md`, `progress.md`, `tasks.md`, and `samples_progress.md` were not changed because this task changes local Codex orchestration settings only; it does not alter the Mirrorea architecture, runtime, active samples, or project status.

## Commit and push status

State the actual Git status, commit, and push result.

## Sub-agent status

No new task sub-agent was started for this configuration-only change.
```

- [ ] **Step 2: Inspect the scoped diff and report agent inventory**

Run: `git diff -- .codex agents docs/superpowers docs/reports`

Expected: only the upgrade mapping, project registry, planner definition, design/plan documents, and completion report are changed.
