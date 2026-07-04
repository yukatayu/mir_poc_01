import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_theorem_lean_stub_pipeline as pipeline  # noqa: E402


class TheoremLeanStubPipelinePlanningTests(unittest.TestCase):
    def test_runtime_sample_plan_uses_runtime_formal_hook_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            plan = pipeline.plan_theorem_lean_stub_pipeline(
                sample_stem="e2-try-fallback",
                artifact_root=Path(temp_dir),
                run_label="lean-stub-check",
                python_executable="/usr/bin/python3",
        )

        self.assertEqual(plan.smoke_mode, "runtime")
        self.assertEqual(
            plan.formal_hook_command.argv[1],
            "scripts/current_l2_detached_loop.py",
        )
        self.assertEqual(plan.formal_hook_command.argv[5], temp_dir)
        self.assertEqual(
            plan.formal_hook_command.argv[2:4],
            ("smoke-formal-hook-runtime", "e2-try-fallback"),
        )
        self.assertEqual(
            plan.review_unit_command.argv[:7],
            (
                "cargo",
                "run",
                "-p",
                "mir-semantics",
                "--example",
                "current_l2_emit_proof_notebook_review_unit",
                "--",
            ),
        )
        self.assertEqual(
            plan.lean_stub_command.argv[:7],
            (
                "cargo",
                "run",
                "-p",
                "mir-semantics",
                "--example",
                "current_l2_emit_lean_theorem_stub",
                "--",
            ),
        )
        self.assertTrue(str(plan.formal_hook_output).endswith(".formal-hook.json"))
        self.assertTrue(str(plan.review_units_output).endswith(".proof-notebook-review-unit.json"))
        self.assertTrue(str(plan.lean_stubs_output).endswith(".lean-theorem-stub.json"))

    def test_default_repo_artifact_plan_uses_repo_relative_command_paths(self) -> None:
        plan = pipeline.plan_theorem_lean_stub_pipeline(
            sample_stem="e2-try-fallback",
            artifact_root=pipeline.DEFAULT_ARTIFACT_ROOT,
            run_label="lean-stub-check",
            python_executable="/usr/bin/python3",
        )

        argv_values = [
            arg
            for command in [
                plan.formal_hook_command,
                plan.review_unit_command,
                plan.lean_stub_command,
            ]
            for arg in command.argv
        ]
        self.assertIn("scripts/current_l2_detached_loop.py", argv_values)
        self.assertIn("target/current-l2-theorem-lean-stub", argv_values)
        self.assertIn(
            "target/current-l2-theorem-lean-stub/formal-hooks/lean-stub-check-e2-try-fallback/e2-try-fallback.formal-hook.json",
            argv_values,
        )
        self.assertFalse(
            any(arg.startswith(f"{pipeline.REPO_ROOT}/") for arg in argv_values)
        )

    def test_static_sample_plan_uses_static_formal_hook_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            plan = pipeline.plan_theorem_lean_stub_pipeline(
                sample_stem="e5-underdeclared-lineage",
                artifact_root=Path(temp_dir),
                run_label="lean-stub-check",
                python_executable="/usr/bin/python3",
            )

        self.assertEqual(plan.smoke_mode, "static")
        self.assertEqual(
            plan.formal_hook_command.argv[2:4],
            ("smoke-formal-hook-static", "e5-underdeclared-lineage"),
        )

    def test_guarded_sample_is_rejected_for_pipeline(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            with self.assertRaisesRegex(ValueError, "not reach a formal hook"):
                pipeline.plan_theorem_lean_stub_pipeline(
                    sample_stem="e3-option-admit-chain",
                    artifact_root=Path(temp_dir),
                    run_label="lean-stub-check",
                    python_executable="/usr/bin/python3",
                )


class TheoremLeanStubPipelineConformanceTests(unittest.TestCase):
    def test_run_command_captures_nested_helper_output(self) -> None:
        command = pipeline.PipelineCommand(
            name="noop",
            argv=("python3", "scripts/current_l2_detached_loop.py", "--help"),
        )
        with mock.patch.object(
            pipeline.subprocess,
            "run",
            return_value=mock.Mock(returncode=0),
        ) as mocked_run:
            pipeline.run_command(command)

        self.assertTrue(mocked_run.call_args.kwargs["capture_output"])
        self.assertTrue(mocked_run.call_args.kwargs["text"])

    def test_validate_conformance_accepts_matching_pairs(self) -> None:
        review_units = [
            {
                "subject_ref": "e2_try_fallback",
                "row": {"obligation_kind": "rollback_cut_non_interference"},
            }
        ]
        stubs = [
            {
                "tool_family": "lean-first",
                "subject_ref": "e2_try_fallback",
                "obligation_kind": "rollback_cut_non_interference",
                "source_text": "theorem e2_try_fallback__rollback_cut_non_interference : True := by\n  sorry\n",
            }
        ]

        summary = pipeline.validate_conformance(review_units, stubs)

        self.assertEqual(summary.review_unit_count, 1)
        self.assertEqual(summary.lean_stub_count, 1)
        self.assertEqual(summary.matched_pairs, 1)

    def test_validate_conformance_rejects_pair_drift(self) -> None:
        review_units = [
            {
                "subject_ref": "e2_try_fallback",
                "row": {"obligation_kind": "rollback_cut_non_interference"},
            }
        ]
        stubs = [
            {
                "tool_family": "lean-first",
                "subject_ref": "e5_underdeclared_lineage",
                "obligation_kind": "canonical_normalization_law",
                "source_text": "theorem e5_underdeclared_lineage__canonical_normalization_law : True := by\n  sorry\n",
            }
        ]

        with self.assertRaisesRegex(ValueError, "pair mismatch"):
            pipeline.validate_conformance(review_units, stubs)


if __name__ == "__main__":
    unittest.main()
