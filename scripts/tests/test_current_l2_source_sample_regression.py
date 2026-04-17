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
                "e1-place-atomic-cut",
                "e2-try-fallback",
                "e3-option-admit-chain",
                "e4-malformed-lineage",
                "e14-malformed-duplicate-option-declaration",
                "e15-malformed-duplicate-chain-declaration",
                "e16-malformed-missing-chain-head-option",
                "e13-malformed-capability-strengthening",
                "e19-malformed-target-mismatch",
                "e21-try-atomic-cut-frontier",
                "e22-try-atomic-cut-place-mismatch",
                "e18-malformed-missing-successor-option",
                "e20-malformed-late-capability-strengthening",
                "e23-malformed-try-fallback-missing-fallback-body",
            ],
        )
        self.assertEqual(
            [row.authored_status for row in rows[:14]],
            [
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
                "source-authored",
            ],
        )
        self.assertEqual(rows[2].formal_hook, "not_reached_guarded")
        self.assertEqual(rows[0].formal_hook, "runtime_try_cut_cluster")
        self.assertEqual(rows[3].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[4].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[5].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[6].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[7].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[8].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[9].formal_hook, "runtime_try_cut_cluster")
        self.assertEqual(rows[10].formal_hook, "runtime_try_cut_cluster")
        self.assertEqual(rows[11].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[12].formal_hook, "fixture_static_cluster")
        self.assertEqual(rows[13].formal_hook, "fixture_static_cluster")

    def test_inventory_statuses_mark_current_repo_layout_as_consistent(self) -> None:
        statuses = regression.inventory_statuses()

        self.assertTrue(all(status.status_ok for status in statuses))
        self.assertEqual(
            [status.file_exists for status in statuses],
            [True, True, True, True, True, True, True, True, True, True, True, True, True, True],
        )

    def test_format_inventory_text_includes_header_rows_and_file_state(self) -> None:
        rendered = regression.format_inventory_text(regression.inventory_statuses())

        self.assertIn("current L2 fixed-subset authored inventory", rendered)
        self.assertIn("sample stem | authored status | expected static", rendered)
        self.assertIn("e1-place-atomic-cut | source-authored | valid | explicit_failure", rendered)
        self.assertIn("e3-option-admit-chain | source-authored | valid | success", rendered)
        self.assertIn(
            "e14-malformed-duplicate-option-declaration | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e15-malformed-duplicate-chain-declaration | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e16-malformed-missing-chain-head-option | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e13-malformed-capability-strengthening | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e19-malformed-target-mismatch | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e21-try-atomic-cut-frontier | source-authored | valid | success",
            rendered,
        )
        self.assertIn(
            "e22-try-atomic-cut-place-mismatch | source-authored | valid | success",
            rendered,
        )
        self.assertIn(
            "e18-malformed-missing-successor-option | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn(
            "e20-malformed-late-capability-strengthening | source-authored | malformed | not_evaluated",
            rendered,
        )
        self.assertIn("e2-try-fallback | source-authored | valid | success", rendered)
        self.assertIn("present | third widened authored row runtime path", rendered)
        self.assertIn("not_reached_guarded", rendered)

    def test_inventory_mismatches_reports_missing_authored_sample(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            sample_root = Path(temp_dir)
            authored = [
                "e1-place-atomic-cut",
                "e2-try-fallback",
                "e3-option-admit-chain",
                "e14-malformed-duplicate-option-declaration",
                "e15-malformed-duplicate-chain-declaration",
                "e16-malformed-missing-chain-head-option",
                "e13-malformed-capability-strengthening",
                "e21-try-atomic-cut-frontier",
                "e22-try-atomic-cut-place-mismatch",
                "e4-malformed-lineage",
                "e18-malformed-missing-successor-option",
                "e20-malformed-late-capability-strengthening",
                "e23-malformed-try-fallback-missing-fallback-body",
            ]
            for stem in authored:
                (sample_root / f"{stem}.txt").write_text("placeholder\n", encoding="utf-8")

            mismatches = regression.inventory_mismatches(
                regression.inventory_statuses(sample_root)
            )

        self.assertEqual(
            mismatches,
            [
                "e19-malformed-target-mismatch: expected sample file present, observed absent"
            ],
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
                "emitted artifact wiring test",
                "formal hook support test",
                "runtime formal hook smoke for e1-place-atomic-cut",
                "runtime formal hook smoke for e2-try-fallback",
                "runtime formal hook smoke for e21-try-atomic-cut-frontier",
                "runtime formal hook smoke for e22-try-atomic-cut-place-mismatch",
                "static formal hook smoke for e4-malformed-lineage",
                "static formal hook smoke for e14-malformed-duplicate-option-declaration",
                "static formal hook smoke for e15-malformed-duplicate-chain-declaration",
                "static formal hook smoke for e16-malformed-missing-chain-head-option",
                "static formal hook smoke for e13-malformed-capability-strengthening",
                "static formal hook smoke for e19-malformed-target-mismatch",
                "static formal hook smoke for e18-malformed-missing-successor-option",
                "static formal hook smoke for e20-malformed-late-capability-strengthening",
                "static formal hook smoke for e23-malformed-try-fallback-missing-fallback-body",
            ],
        )
        self.assertEqual(
            commands[0].argv,
            ("cargo", "test", "-p", "mir-runtime", "--test", "current_l2_source_lowering"),
        )
        self.assertEqual(
            commands[3].argv,
            (
                "cargo",
                "test",
                "-p",
                "mir-runtime",
                "--test",
                "current_l2_source_sample_emitted_artifact_wiring",
            ),
        )
        self.assertEqual(commands[5].argv[0], "/usr/bin/python3")
        self.assertEqual(commands[5].argv[2], "smoke-formal-hook-runtime")
        self.assertIn("--artifact-root", commands[5].argv)
        self.assertIn("--run-label", commands[5].argv)
        self.assertEqual(
            commands[5].argv[commands[5].argv.index("--run-label") + 1],
            "phase6-smoke-e1-place-atomic-cut",
        )
        self.assertEqual(
            commands[6].argv[commands[6].argv.index("--run-label") + 1],
            "phase6-smoke-e2-try-fallback",
        )
        self.assertEqual(
            commands[7].argv[commands[7].argv.index("--run-label") + 1],
            "phase6-smoke-e21-try-atomic-cut-frontier",
        )
        self.assertEqual(
            commands[8].argv[commands[8].argv.index("--run-label") + 1],
            "phase6-smoke-e22-try-atomic-cut-place-mismatch",
        )
        self.assertEqual(
            commands[9].argv[commands[9].argv.index("--run-label") + 1],
            "phase6-smoke-e4-malformed-lineage",
        )
        self.assertEqual(
            commands[10].argv[commands[10].argv.index("--run-label") + 1],
            "phase6-smoke-e14-malformed-duplicate-option-declaration",
        )
        self.assertEqual(
            commands[11].argv[commands[11].argv.index("--run-label") + 1],
            "phase6-smoke-e15-malformed-duplicate-chain-declaration",
        )
        self.assertEqual(
            commands[12].argv[commands[12].argv.index("--run-label") + 1],
            "phase6-smoke-e16-malformed-missing-chain-head-option",
        )
        self.assertEqual(
            commands[13].argv[commands[13].argv.index("--run-label") + 1],
            "phase6-smoke-e13-malformed-capability-strengthening",
        )
        self.assertEqual(
            commands[14].argv[commands[14].argv.index("--run-label") + 1],
            "phase6-smoke-e19-malformed-target-mismatch",
        )
        self.assertEqual(
            commands[15].argv[commands[15].argv.index("--run-label") + 1],
            "phase6-smoke-e18-malformed-missing-successor-option",
        )
        self.assertEqual(
            commands[16].argv[commands[16].argv.index("--run-label") + 1],
            "phase6-smoke-e20-malformed-late-capability-strengthening",
        )
        self.assertEqual(
            commands[17].argv[commands[17].argv.index("--run-label") + 1],
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
        self.assertIn("not_reached_guarded", stdout.getvalue())
        self.assertIn("present", stdout.getvalue())

    def test_main_inventory_reports_mismatch_from_custom_sample_root(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            sample_root = Path(temp_dir)
            (sample_root / "e2-try-fallback.txt").write_text("placeholder\n", encoding="utf-8")

            stdout = io.StringIO()
            stderr = io.StringIO()
            exit_code = regression.main(
                ["inventory", "--sample-root", str(sample_root)],
                stdout=stdout,
                stderr=stderr,
            )

        self.assertEqual(exit_code, 2)
        self.assertIn("e23-malformed-try-fallback-missing-fallback-body", stderr.getvalue())
        self.assertIn("observed absent", stderr.getvalue())

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
        self.assertEqual(len(captured_commands), 18)
        self.assertEqual(
            captured_commands[5].argv[captured_commands[5].argv.index("--run-label") + 1],
            "phase6-helper-e1-place-atomic-cut",
        )
        self.assertEqual(
            captured_commands[6].argv[captured_commands[6].argv.index("--run-label") + 1],
            "phase6-helper-e2-try-fallback",
        )
        self.assertEqual(
            captured_commands[7].argv[captured_commands[7].argv.index("--run-label") + 1],
            "phase6-helper-e21-try-atomic-cut-frontier",
        )
        self.assertEqual(
            captured_commands[8].argv[captured_commands[8].argv.index("--run-label") + 1],
            "phase6-helper-e22-try-atomic-cut-place-mismatch",
        )
        self.assertEqual(
            captured_commands[9].argv[captured_commands[9].argv.index("--run-label") + 1],
            "phase6-helper-e4-malformed-lineage",
        )
        self.assertEqual(
            captured_commands[10].argv[captured_commands[10].argv.index("--run-label") + 1],
            "phase6-helper-e14-malformed-duplicate-option-declaration",
        )
        self.assertEqual(
            captured_commands[11].argv[captured_commands[11].argv.index("--run-label") + 1],
            "phase6-helper-e15-malformed-duplicate-chain-declaration",
        )
        self.assertEqual(
            captured_commands[12].argv[captured_commands[12].argv.index("--run-label") + 1],
            "phase6-helper-e16-malformed-missing-chain-head-option",
        )
        self.assertEqual(
            captured_commands[13].argv[captured_commands[13].argv.index("--run-label") + 1],
            "phase6-helper-e13-malformed-capability-strengthening",
        )
        self.assertEqual(
            captured_commands[14].argv[captured_commands[14].argv.index("--run-label") + 1],
            "phase6-helper-e19-malformed-target-mismatch",
        )
        self.assertEqual(
            captured_commands[15].argv[captured_commands[15].argv.index("--run-label") + 1],
            "phase6-helper-e18-malformed-missing-successor-option",
        )
        self.assertEqual(
            captured_commands[16].argv[captured_commands[16].argv.index("--run-label") + 1],
            "phase6-helper-e20-malformed-late-capability-strengthening",
        )
        self.assertEqual(
            captured_commands[17].argv[captured_commands[17].argv.index("--run-label") + 1],
            "phase6-helper-e23-malformed-try-fallback-missing-fallback-body",
        )

    def test_main_regression_rejects_inventory_mismatch_before_running_commands(self) -> None:
        original_inventory_statuses = regression.inventory_statuses
        stdout = io.StringIO()
        stderr = io.StringIO()

        def fake_inventory_statuses(sample_root=None):
            return [
                regression.InventoryStatus(
                    row=regression.InventoryRow(
                        sample_stem="e2-try-fallback",
                        authored_status="source-authored",
                        expected_static="valid",
                        expected_runtime="success",
                        formal_hook="runtime_try_cut_cluster",
                        note="first authored trio runtime path",
                    ),
                    sample_path=Path("/tmp/e2-try-fallback.txt"),
                    file_exists=False,
                    status_ok=False,
                )
            ]

        regression.inventory_statuses = fake_inventory_statuses
        try:
            exit_code = regression.main(
                ["regression"],
                stdout=stdout,
                stderr=stderr,
            )
        finally:
            regression.inventory_statuses = original_inventory_statuses

        self.assertEqual(exit_code, 2)
        self.assertEqual(stdout.getvalue(), "")
        self.assertIn("expected sample file present, observed absent", stderr.getvalue())

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
