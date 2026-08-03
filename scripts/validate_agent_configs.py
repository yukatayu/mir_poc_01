#!/usr/bin/env python3
"""Validate the repository's delegated-agent configuration contract."""

from __future__ import annotations

import argparse
from pathlib import Path
import re
import sys
import tomllib
from typing import Any


ALLOWED_SANDBOX_MODES = (
    "read-only",
    "workspace-write",
    "danger-full-access",
)

ALLOWED_APPROVAL_POLICIES = (
    "untrusted",
    "on-request",
    "never",
)

EXPECTED_SANDBOX_MODES = {
    "code_mapper": "read-only",
    "docs_researcher": "read-only",
    "eval_runner": "workspace-write",
    "implementer": "workspace-write",
    "planner": "workspace-write",
    "reviewer": "read-only",
    "status_reporter": "workspace-write",
    "test_author": "workspace-write",
}

EXPECTED_APPROVAL_POLICIES = {
    "code_mapper": "untrusted",
    "docs_researcher": "untrusted",
    "eval_runner": "on-request",
    "implementer": "on-request",
    "planner": "on-request",
    "reviewer": "untrusted",
    "status_reporter": "on-request",
    "test_author": "on-request",
}

REQUIRED_AGENT_FIELDS = (
    "name",
    "description",
    "developer_instructions",
    "approval_policy",
)


def display_path(path: Path, root: Path) -> str:
    try:
        return path.relative_to(root).as_posix()
    except ValueError:
        return str(path)


def load_toml(path: Path, root: Path, errors: list[str]) -> dict[str, Any] | None:
    try:
        with path.open("rb") as source:
            data = tomllib.load(source)
    except FileNotFoundError:
        errors.append(f"{display_path(path, root)}: required file is missing")
        return None
    except OSError as error:
        errors.append(f"{display_path(path, root)}: cannot read TOML: {error}")
        return None
    except tomllib.TOMLDecodeError as error:
        errors.append(f"{display_path(path, root)}: invalid TOML: {error}")
        return None

    if not isinstance(data, dict):
        errors.append(f"{display_path(path, root)}: TOML root must be a table")
        return None
    return data


def validate_required_fields(
    role: str,
    agent_path: Path,
    agent: dict[str, Any],
    root: Path,
    errors: list[str],
) -> None:
    for field in REQUIRED_AGENT_FIELDS:
        value = agent.get(field)
        if not isinstance(value, str) or not value.strip():
            errors.append(
                f"{display_path(agent_path, root)}: role '{role}' field "
                f"'{field}' is required and must be a non-empty string"
            )


def validate_sandbox_mode(
    role: str,
    agent_path: Path,
    agent: dict[str, Any],
    root: Path,
    errors: list[str],
) -> None:
    value = agent.get("sandbox_mode")
    allowed = ", ".join(ALLOWED_SANDBOX_MODES)
    if not isinstance(value, str) or not value.strip():
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' sandbox_mode is "
            f"required; allowed values: {allowed}"
        )
        return
    if value not in ALLOWED_SANDBOX_MODES:
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' sandbox_mode "
            f"'{value}' is not allowed; allowed values: {allowed}"
        )
        return

    expected = EXPECTED_SANDBOX_MODES.get(role)
    if expected is not None and value != expected:
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' sandbox_mode "
            f"'{value}' must be '{expected}' (role policy; global allowed "
            f"values: {allowed})"
        )


def validate_approval_policy(
    role: str,
    agent_path: Path,
    agent: dict[str, Any],
    root: Path,
    errors: list[str],
) -> None:
    value = agent.get("approval_policy")
    allowed = ", ".join(ALLOWED_APPROVAL_POLICIES)
    if not isinstance(value, str) or not value.strip():
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' approval_policy is "
            f"required; allowed values: {allowed}"
        )
        return
    if value not in ALLOWED_APPROVAL_POLICIES:
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' approval_policy "
            f"'{value}' is not allowed; allowed values: {allowed}"
        )
        return

    expected = EXPECTED_APPROVAL_POLICIES.get(role)
    if expected is not None and value != expected:
        errors.append(
            f"{display_path(agent_path, root)}: role '{role}' approval_policy "
            f"'{value}' must be '{expected}' (role policy; global allowed "
            f"values: {allowed})"
        )


def validate_planner_instructions(
    agent_path: Path,
    agent: dict[str, Any],
    root: Path,
    errors: list[str],
) -> None:
    instructions = agent.get("developer_instructions")
    if not isinstance(instructions, str):
        return

    normalized = " ".join(instructions.casefold().split())
    allows_apply = re.search(r"\bappl(?:y|ying|ied)\b", normalized) is not None
    allows_validate = "validat" in normalized
    covers_planning_status = "planning/status" in normalized or (
        "planning" in normalized and "status" in normalized
    )
    if not (allows_apply and allows_validate and covers_planning_status):
        errors.append(
            f"{display_path(agent_path, root)}: planner developer_instructions "
            "must allow apply and validate planning/status changes"
        )

    protected_surfaces = (
        "production source",
        "tests",
        "lean",
        "normative semantics",
    )
    if not all(surface in normalized for surface in protected_surfaces):
        errors.append(
            f"{display_path(agent_path, root)}: planner developer_instructions "
            "must prohibit edits to production source, tests, Lean, and "
            "normative semantics without direct parent delegation"
        )
    if "parent explicitly delegates" not in normalized:
        errors.append(
            f"{display_path(agent_path, root)}: planner developer_instructions "
            "must retain the exception that the parent explicitly delegates "
            "the protected scope"
        )


def validate(root: Path) -> list[str]:
    errors: list[str] = []
    root = root.resolve()
    codex_path = root / ".codex" / "config.toml"
    codex = load_toml(codex_path, root, errors)
    if codex is None:
        return errors

    agents = codex.get("agents")
    if not isinstance(agents, dict):
        return [
            *errors,
            f"{display_path(codex_path, root)}: required [agents] table is missing",
        ]

    for role in EXPECTED_SANDBOX_MODES:
        if role not in agents:
            errors.append(
                f"{display_path(codex_path, root)}: required [agents.{role}] "
                "table is missing"
            )

    for role in sorted(agents):
        config = agents[role]
        if not isinstance(role, str) or not isinstance(config, dict):
            errors.append(
                f"{display_path(codex_path, root)}: [agents] entries must be tables"
            )
            continue

        if role == "planner":
            description = config.get("description")
            if not isinstance(description, str) or not description.strip():
                errors.append(
                    f"{display_path(codex_path, root)}: [agents.planner].description "
                    "is required and must be a non-empty string"
                )
            elif "read-only" in description.casefold():
                errors.append(
                    f"{display_path(codex_path, root)}: [agents.planner].description "
                    "must not describe planner as read-only"
                )

        config_file = config.get("config_file")
        if not isinstance(config_file, str) or not config_file.strip():
            errors.append(
                f"{display_path(codex_path, root)}: [agents.{role}].config_file "
                "is required and must be a non-empty string"
            )
            continue

        agent_path = (codex_path.parent / config_file).resolve()
        agent = load_toml(agent_path, root, errors)
        if agent is None:
            continue

        validate_required_fields(role, agent_path, agent, root, errors)
        validate_sandbox_mode(role, agent_path, agent, root, errors)
        validate_approval_policy(role, agent_path, agent, root, errors)
        if role == "planner":
            validate_planner_instructions(agent_path, agent, root, errors)

    return errors


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Validate delegated-agent TOML configuration."
    )
    parser.add_argument(
        "--root",
        type=Path,
        default=Path(__file__).resolve().parents[1],
        help="repository root containing .codex/config.toml",
    )
    args = parser.parse_args()

    errors = validate(args.root)
    if errors:
        print("Agent configuration validation failed:", file=sys.stderr)
        for error in errors:
            print(f" - {error}", file=sys.stderr)
        return 1

    print("Agent configuration validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
