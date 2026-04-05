import contextlib
import io
import json
import shlex
import sys
import tempfile
import unittest
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_scaffold_fixture as scaffold


class CurrentL2ScaffoldFixtureTests(unittest.TestCase):
    def test_runtime_scaffold_creates_fixture_and_host_plan(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            output_dir = Path(tmpdir)

            with contextlib.redirect_stdout(io.StringIO()):
                exit_code = scaffold.main(
                    [
                        "e10-new-runtime-case",
                        "--kind",
                        "runtime",
                        "--output-dir",
                        str(output_dir),
                        "--source-example-id",
                        "E10-TODO",
                    ]
                )

            self.assertEqual(exit_code, 0)

            fixture_path = output_dir / "e10-new-runtime-case.json"
            host_plan_path = output_dir / "e10-new-runtime-case.host-plan.json"
            self.assertTrue(fixture_path.exists())
            self.assertTrue(host_plan_path.exists())

            fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
            self.assertEqual(fixture["fixture_id"], "e10_new_runtime_case")
            self.assertEqual(fixture["source_example_id"], "E10-TODO")
            self.assertEqual(fixture["expected_runtime"]["enters_evaluation"], True)
            self.assertEqual(
                fixture["expected_runtime"]["final_outcome"], "OPEN_RUNTIME_OUTCOME"
            )
            self.assertEqual(
                fixture["expected_static"]["verdict"], "OPEN_STATIC_VERDICT"
            )

            host_plan = json.loads(host_plan_path.read_text(encoding="utf-8"))
            self.assertEqual(host_plan["schema_version"], "current-l2-host-plan-v0")
            self.assertEqual(host_plan["predicate_rules"], [])
            self.assertEqual(host_plan["effect_rules"], [])

    def test_static_only_scaffold_creates_fixture_without_host_plan(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            output_dir = Path(tmpdir)

            stdout = io.StringIO()
            stderr = io.StringIO()
            with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                exit_code = scaffold.main(
                    [
                        "e11-new-static-case",
                        "--kind",
                        "static-only",
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 0)

            fixture_path = output_dir / "e11-new-static-case.json"
            host_plan_path = output_dir / "e11-new-static-case.host-plan.json"
            self.assertTrue(fixture_path.exists())
            self.assertFalse(host_plan_path.exists())

            fixture = json.loads(fixture_path.read_text(encoding="utf-8"))
            self.assertEqual(fixture["expected_runtime"]["enters_evaluation"], False)
            self.assertEqual(fixture["expected_runtime"]["final_outcome"], "not_evaluated")
            self.assertIn("suggest-checked-reasons", stderr.getvalue())
            self.assertIn("after first authoring pass", stderr.getvalue())
            self.assertIn(str(fixture_path), stdout.getvalue())

    def test_runtime_scaffold_does_not_print_checked_reasons_followup_reminder(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            output_dir = Path(tmpdir)
            stdout = io.StringIO()
            stderr = io.StringIO()

            with contextlib.redirect_stdout(stdout), contextlib.redirect_stderr(stderr):
                exit_code = scaffold.main(
                    [
                        "e14-runtime-no-static-followup",
                        "--kind",
                        "runtime",
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 0)
            self.assertNotIn("suggest-checked-reasons", stderr.getvalue())

    def test_static_only_followup_message_shell_quotes_fixture_path(self) -> None:
        fixture_path = Path("dir with space") / "e11 (new).json"

        message = scaffold.static_only_followup_message(fixture_path)

        expected_path = shlex.quote(str(fixture_path))
        self.assertIn(
            f"suggest-checked-reasons {expected_path} --run-label TODO --overwrite",
            message,
        )

    def test_existing_target_requires_overwrite_flag(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            output_dir = Path(tmpdir)
            fixture_path = output_dir / "e12-repeat.json"
            fixture_path.write_text("{}", encoding="utf-8")

            with contextlib.redirect_stderr(io.StringIO()):
                exit_code = scaffold.main(
                    [
                        "e12-repeat",
                        "--kind",
                        "static-only",
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 2)

    def test_explicit_fixture_id_overrides_stem_default(self) -> None:
        with tempfile.TemporaryDirectory() as tmpdir:
            output_dir = Path(tmpdir)

            with contextlib.redirect_stdout(io.StringIO()):
                exit_code = scaffold.main(
                    [
                        "e13-custom-id",
                        "--kind",
                        "runtime",
                        "--fixture-id",
                        "custom_fixture_id",
                        "--output-dir",
                        str(output_dir),
                    ]
                )

            self.assertEqual(exit_code, 0)
            fixture = json.loads(
                (output_dir / "e13-custom-id.json").read_text(encoding="utf-8")
            )
            self.assertEqual(fixture["fixture_id"], "custom_fixture_id")


if __name__ == "__main__":
    unittest.main()
