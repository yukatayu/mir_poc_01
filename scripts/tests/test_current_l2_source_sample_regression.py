import io
import sys
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_source_sample_regression as regression  # noqa: E402


class SourceSampleRegressionInventoryTests(unittest.TestCase):
    def test_inventory_rows_cover_authored_and_deferred_entries(self) -> None:
        rows = regression.inventory_rows()

        self.assertEqual(
            [row.sample_stem for row in rows],
            [
                "e2-try-fallback",
                "e4-malformed-lineage",
                "e23-malformed-try-fallback-missing-fallback-body",
                "e1-place-atomic-cut",
                "e3-option-admit-chain",
                "e21-try-atomic-cut-frontier",
            ],
        )
        self.assertEqual(
            [row.authored_status for row in rows[:3]],
            ["source-authored", "source-authored", "source-authored"],
        )
        self.assertEqual(
            [row.authored_status for row in rows[3:]],
            ["source-target-only", "source-target-only", "source-target-only"],
        )
        self.assertEqual(rows[0].formal_hook, "runtime_try_cut_cluster")
        self.assertEqual(rows[1].formal_hook, "fixture_static_cluster")

    def test_format_inventory_text_includes_header_and_rows(self) -> None:
        rendered = regression.format_inventory_text(regression.inventory_rows())

        self.assertIn("current L2 fixed-subset first-cluster inventory", rendered)
        self.assertIn("sample stem | authored status | expected static", rendered)
        self.assertIn("e2-try-fallback | source-authored | valid | success", rendered)
        self.assertIn(
            "e21-try-atomic-cut-frontier | source-target-only | not_yet_authored",
            rendered,
        )


class SourceSampleRegressionPlanningTests(unittest.TestCase):
    def test_plan_regression_commands_uses_expected_order_and_smoke_plumbing(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            commands = regression.plan_regression_commands(
                artifact_root=Path(temp_dir),
                run_label="phase6-smoke",
                python_executable="/usr/bin/python3",
            )

        self.assertEqual(
            [command.name for command in commands],
            [
                "runtime lowering test",
                "source sample runner test",
                "verification ladder test",
                "formal hook support test",
                "runtime formal hook smoke for e2-try-fallback",
                "static formal hook smoke for e4-malformed-lineage",
                "static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body",
            ],
        )
        self.assertEqual(
            commands[0].argv,
            ("cargo", "test", "-p", "mir-runtime", "--test", "current_l2_source_lowering"),
        )
        self.assertEqual(commands[4].argv[0], "/usr/bin/python3")
        self.assertEqual(commands[4].argv[2], "smoke-formal-hook-runtime")
        self.assertIn("--artifact-root", commands[4].argv)
        self.assertIn("--run-label", commands[4].argv)
        self.assertEqual(
            commands[4].argv[commands[4].argv.index("--run-label") + 1],
            "phase6-smoke-e2-try-fallback",
        )
        self.assertEqual(
            commands[6].argv[commands[6].argv.index("--run-label") + 1],
            "phase6-smoke-e23-malformed-try-fallback-missing-fallback-body",
        )

    def test_plan_regression_commands_rejects_invalid_run_label(self) -> None:
        with self.assertRaises(ValueError):
            regression.plan_regression_commands(
                artifact_root=REPO_ROOT / "target" / "regression-test",
                run_label="bad/label",
            )


class SourceSampleRegressionExecutionTests(unittest.TestCase):
    def test_run_regression_commands_short_circuits_on_failure(self) -> None:
        commands = [
            regression.RegressionCommand(name="first", argv=("cmd-1",)),
            regression.RegressionCommand(name="second", argv=("cmd-2",)),
            regression.RegressionCommand(name="third", argv=("cmd-3",)),
        ]
        seen: list[tuple[str, ...]] = []

        def fake_runner(argv: tuple[str, ...]) -> int:
            seen.append(argv)
            if argv == ("cmd-2",):
                return 9
            return 0

        stdout = io.StringIO()
        exit_code = regression.run_regression_commands(
            commands,
            runner=fake_runner,
            stdout=stdout,
        )

        self.assertEqual(exit_code, 9)
        self.assertEqual(seen, [("cmd-1",), ("cmd-2",)])
        self.assertIn("[1/3] first", stdout.getvalue())
        self.assertIn("stopped after failure in 'second' (exit 9)", stdout.getvalue())

    def test_run_regression_commands_reports_success(self) -> None:
        commands = [
            regression.RegressionCommand(name="only", argv=("cmd",)),
        ]
        stdout = io.StringIO()

        exit_code = regression.run_regression_commands(
            commands,
            runner=lambda argv: 0,
            stdout=stdout,
        )

        self.assertEqual(exit_code, 0)
        self.assertIn("all regression commands passed", stdout.getvalue())


class SourceSampleRegressionCliTests(unittest.TestCase):
    def test_main_inventory_prints_inventory(self) -> None:
        stdout = io.StringIO()
        stderr = io.StringIO()

        exit_code = regression.main(["inventory"], stdout=stdout, stderr=stderr)

        self.assertEqual(exit_code, 0)
        self.assertEqual(stderr.getvalue(), "")
        self.assertIn("e4-malformed-lineage", stdout.getvalue())
        self.assertIn("source-target-only", stdout.getvalue())

    def test_main_regression_uses_planned_commands(self) -> None:
        original_run_regression_commands = regression.run_regression_commands
        captured_commands: list[regression.RegressionCommand] = []

        def fake_run_regression_commands(commands, runner=regression.run_subprocess, stdout=sys.stdout):
            captured_commands.extend(commands)
            print("fake regression ran", file=stdout)
            return 0

        regression.run_regression_commands = fake_run_regression_commands
        try:
            stdout = io.StringIO()
            stderr = io.StringIO()
            exit_code = regression.main(
                [
                    "regression",
                    "--artifact-root",
                    "/tmp/current-l2-source-regression",
                    "--run-label",
                    "phase6-helper",
                ],
                stdout=stdout,
                stderr=stderr,
            )
        finally:
            regression.run_regression_commands = original_run_regression_commands

        self.assertEqual(exit_code, 0)
        self.assertEqual(stderr.getvalue(), "")
        self.assertIn("fake regression ran", stdout.getvalue())
        self.assertEqual(len(captured_commands), 7)
        self.assertEqual(
            captured_commands[4].argv[captured_commands[4].argv.index("--run-label") + 1],
            "phase6-helper-e2-try-fallback",
        )

    def test_main_regression_rejects_invalid_run_label(self) -> None:
        stdout = io.StringIO()
        stderr = io.StringIO()

        exit_code = regression.main(
            ["regression", "--run-label", "bad/label"],
            stdout=stdout,
            stderr=stderr,
        )

        self.assertEqual(exit_code, 2)
        self.assertIn("run label must match", stderr.getvalue())


if __name__ == "__main__":
    unittest.main()
