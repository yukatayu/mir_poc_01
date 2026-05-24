from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path

import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import surface_mir_release_check as runner  # noqa: E402


REPO_ROOT = Path(__file__).resolve().parents[2]


class SurfaceMirReleaseCheckTests(unittest.TestCase):
    def test_plan_includes_p_surf_01_parser_floor_commands(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            plan = runner.plan_check_all(Path(tmp))

        names = [command.name for command in plan.commands]

        self.assertIn("test:surface-parser", names)
        self.assertIn("test:indexed-state-semantics", names)
        self.assertIn("test:surface-samples", names)
        self.assertIn("test:surface-release-check", names)
        self.assertIn("helper:surface-samples", names)
        self.assertIn("helper:surface-authoring", names)

    def test_helper_semantic_check_keeps_parser_floor_non_workflow_ready(self) -> None:
        command = runner.PlannedCommand(
            name="helper:surface-samples",
            argv=["python3", "scripts/surface_mir_samples.py", "check-all", "--format", "json"],
        )
        payload = {
            "sample_count": 14,
            "failed": [],
            "workflow_ready": False,
        }

        self.assertEqual(runner.semantic_errors_for_result(command, payload), [])

    def test_plan_command_accepts_global_format_before_subcommand(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            completed = subprocess.run(
                [
                    "python3",
                    "scripts/surface_mir_release_check.py",
                    "--format",
                    "json",
                    "plan",
                    "--out",
                    tmp,
                ],
                cwd=REPO_ROOT,
                text=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                check=False,
            )

        self.assertEqual(completed.returncode, 0, completed.stderr)
        payload = json.loads(completed.stdout)
        self.assertEqual(payload["surface_kind"], "surface_mir_release_check_plan")
        self.assertFalse(payload["final_public_grammar_frozen"])


if __name__ == "__main__":
    unittest.main()
