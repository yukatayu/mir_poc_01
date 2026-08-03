from __future__ import annotations

import subprocess
import sys
import tempfile
import textwrap
import tomllib
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
VALIDATOR = REPO_ROOT / "scripts" / "validate_agent_configs.py"

EXPECTED_SANDBOXES = {
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

ROLE_DESCRIPTIONS = {
    "code_mapper": "Read-only codebase mapper.",
    "docs_researcher": "Read-only external documentation verifier.",
    "eval_runner": "Verification command runner.",
    "implementer": "Production-source implementation writer.",
    "planner": "Strategic planner for sequencing and planning/status edits.",
    "reviewer": "Read-only correctness and regression reviewer.",
    "status_reporter": "Documentation and status-maintenance writer.",
    "test_author": "Test-only author.",
}

ROLE_INSTRUCTIONS = {
    "planner": """
        Plan repository-wide sequencing, dependencies, evidence gates, and
        decision checkpoints for the parent agent.

        You may apply and validate planning/status edits in plan/,
        Documentation.md, docs/project-status.md, progress.md, tasks.md, and
        samples_progress.md when that is the delegated task.

        Do not edit production source, tests, Lean files, or normative
        semantics unless the parent explicitly delegates that scope.
    """,
    "code_mapper": "Stay read-only. Map files, symbols, invariants, and dependencies.",
    "docs_researcher": "Stay read-only. Verify external documentation and assumptions.",
    "eval_runner": "Run verification commands and report exact failures.",
    "implementer": "Write production source only when explicitly assigned.",
    "reviewer": "Stay read-only. Review correctness, regressions, and missing tests.",
    "status_reporter": "Update Documentation.md and status diagrams when assigned.",
    "test_author": "Write or update tests only.",
}


def run_validator(root: Path, *, cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(VALIDATOR), "--root", str(root)],
        cwd=cwd or root,
        text=True,
        capture_output=True,
        check=False,
    )


def combined_output(completed: subprocess.CompletedProcess[str]) -> str:
    return completed.stdout + completed.stderr


def write_valid_fixture(
    root: Path,
    *,
    agent_overrides: dict[str, dict[str, str | None]] | None = None,
    codex_description_overrides: dict[str, str] | None = None,
) -> None:
    agent_overrides = agent_overrides or {}
    codex_description_overrides = codex_description_overrides or {}

    codex_dir = root / ".codex"
    agents_dir = root / "agents"
    codex_dir.mkdir(parents=True)
    agents_dir.mkdir(parents=True)

    codex_entries = []
    for role in sorted(EXPECTED_SANDBOXES):
        description = codex_description_overrides.get(role, ROLE_DESCRIPTIONS[role])
        codex_entries.append(
            textwrap.dedent(
                f"""
                [agents.{role}]
                description = "{description}"
                config_file = "../agents/{role}.toml"
                """
            ).strip()
        )

        fields: dict[str, str | None] = {
            "name": role,
            "description": ROLE_DESCRIPTIONS[role],
            "model": "gpt-5.6-terra",
            "model_reasoning_effort": "medium",
            "sandbox_mode": EXPECTED_SANDBOXES[role],
            "approval_policy": EXPECTED_APPROVAL_POLICIES[role],
            "developer_instructions": textwrap.dedent(ROLE_INSTRUCTIONS[role]).strip(),
        }
        fields.update(agent_overrides.get(role, {}))
        write_agent_config(agents_dir / f"{role}.toml", fields)

    (codex_dir / "config.toml").write_text(
        "\n\n".join(codex_entries) + "\n", encoding="utf-8"
    )


def write_agent_config(path: Path, fields: dict[str, str | None]) -> None:
    lines = []
    for key in (
        "name",
        "description",
        "model",
        "model_reasoning_effort",
        "sandbox_mode",
        "approval_policy",
        "developer_instructions",
    ):
        value = fields.get(key)
        if value is None:
            continue
        if "\n" in value:
            lines.append(f'{key} = """\n{value}\n"""')
        else:
            lines.append(f'{key} = "{value}"')
    path.write_text("\n".join(lines) + "\n", encoding="utf-8")


class ValidateAgentConfigsTests(unittest.TestCase):
    def assert_validator_ok(self, root: Path, *, cwd: Path | None = None) -> None:
        completed = run_validator(root, cwd=cwd)
        self.assertEqual(completed.returncode, 0, combined_output(completed))

    def assert_validator_rejects(
        self,
        root: Path,
        *expected_fragments: str,
        cwd: Path | None = None,
    ) -> None:
        completed = run_validator(root, cwd=cwd)
        self.assertNotEqual(completed.returncode, 0, combined_output(completed))
        output = combined_output(completed).casefold()
        for fragment in expected_fragments:
            self.assertIn(fragment.casefold(), output)

    def test_config_file_paths_resolve_relative_to_codex_config(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(root)
            nested_cwd = root / "outside-cwd"
            nested_cwd.mkdir()

            self.assert_validator_ok(root, cwd=nested_cwd)

    def test_agent_configs_require_declared_contract_fields(self) -> None:
        for required_key in (
            "name",
            "description",
            "developer_instructions",
            "approval_policy",
        ):
            with self.subTest(required_key=required_key):
                with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
                    root = Path(tmp)
                    write_valid_fixture(
                        root,
                        agent_overrides={"test_author": {required_key: None}},
                    )

                    self.assert_validator_rejects(
                        root,
                        "test_author",
                        required_key,
                        "required",
                    )

    def test_sandbox_mode_allows_only_read_only_or_workspace_write(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(
                root,
                agent_overrides={"implementer": {"sandbox_mode": "danger-full-access"}},
            )

            self.assert_validator_rejects(
                root,
                "implementer",
                "sandbox_mode",
                "danger-full-access",
                "read-only",
                "workspace-write",
            )

    def test_role_sandboxes_match_expected_permissions(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(
                root,
                agent_overrides={"planner": {"sandbox_mode": "read-only"}},
            )

            self.assert_validator_rejects(
                root,
                "planner",
                "sandbox_mode",
                "workspace-write",
            )

    def test_approval_policy_allows_only_supported_values(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(
                root,
                agent_overrides={"implementer": {"approval_policy": "unsafe"}},
            )

            self.assert_validator_rejects(
                root,
                "implementer",
                "approval_policy",
                "unsafe",
                "untrusted",
                "on-request",
                "never",
            )

    def test_role_approval_policies_match_expected_permissions(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(
                root,
                agent_overrides={"planner": {"approval_policy": "untrusted"}},
            )

            self.assert_validator_rejects(
                root,
                "planner",
                "approval_policy",
                "on-request",
            )

    def test_planner_instructions_allow_planning_status_edits_and_forbid_source_edits(self) -> None:
        valid_instructions = textwrap.dedent(ROLE_INSTRUCTIONS["planner"]).strip()
        invalid_cases = {
            "missing planning/status write allowance": (
                valid_instructions.replace(
                    "You may apply and validate planning/status edits in plan/,\n"
                    "Documentation.md, docs/project-status.md, progress.md, tasks.md, and\n"
                    "samples_progress.md when that is the delegated task.",
                    "Stay read-only.",
                ),
                ("planner", "planning/status", "apply", "validate"),
            ),
            "missing production source ban": (
                valid_instructions.replace(
                    "production source, tests, Lean files, or normative\nsemantics",
                    "unrelated files",
                ),
                ("planner", "production source", "tests", "Lean", "normative semantics"),
            ),
            "missing parent delegation exception": (
                valid_instructions.replace(
                    " unless the parent explicitly delegates that scope",
                    "",
                ),
                ("planner", "parent", "explicitly delegates"),
            ),
        }

        for label, (instructions, expected_fragments) in invalid_cases.items():
            with self.subTest(label=label):
                with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
                    root = Path(tmp)
                    write_valid_fixture(
                        root,
                        agent_overrides={
                            "planner": {"developer_instructions": instructions}
                        },
                    )

                    self.assert_validator_rejects(root, *expected_fragments)

    def test_codex_planner_description_must_not_call_planner_read_only(self) -> None:
        with tempfile.TemporaryDirectory(prefix="mirrorea-agent-configs-") as tmp:
            root = Path(tmp)
            write_valid_fixture(
                root,
                codex_description_overrides={
                    "planner": "Read-only strategic planner for task sequencing."
                },
            )

            self.assert_validator_rejects(
                root,
                ".codex/config.toml",
                "planner",
                "description",
                "read-only",
            )

    def test_repository_codex_planner_description_is_not_read_only(self) -> None:
        config = tomllib.loads((REPO_ROOT / ".codex" / "config.toml").read_text())

        planner_description = config["agents"]["planner"]["description"]
        self.assertNotIn("read-only", planner_description.casefold())


if __name__ == "__main__":
    unittest.main()
