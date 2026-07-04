import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_model_check_carrier_pipeline as pipeline  # noqa: E402


class ModelCheckCarrierPipelinePlanningTests(unittest.TestCase):
    def test_runtime_sample_plan_uses_runtime_formal_hook_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            plan = pipeline.plan_model_check_carrier_pipeline(
                sample_stem="e2-try-fallback",
                artifact_root=Path(temp_dir),
                run_label="model-check-carrier",
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
            plan.model_check_command.argv[:7],
            (
                "cargo",
                "run",
                "-p",
                "mir-semantics",
                "--example",
                "current_l2_emit_model_check_carrier",
                "--",
            ),
        )
        self.assertTrue(str(plan.formal_hook_output).endswith(".formal-hook.json"))
        self.assertTrue(str(plan.model_check_output).endswith(".model-check-carrier.json"))

    def test_default_repo_artifact_plan_uses_repo_relative_command_paths(self) -> None:
        plan = pipeline.plan_model_check_carrier_pipeline(
            sample_stem="e2-try-fallback",
            artifact_root=pipeline.DEFAULT_ARTIFACT_ROOT,
            run_label="model-check-carrier",
            python_executable="/usr/bin/python3",
        )

        argv_values = [
            arg
            for command in [plan.formal_hook_command, plan.model_check_command]
            for arg in command.argv
        ]
        self.assertIn("scripts/current_l2_detached_loop.py", argv_values)
        self.assertIn("target/current-l2-model-check-carrier", argv_values)
        self.assertIn(
            "target/current-l2-model-check-carrier/formal-hooks/model-check-carrier-e2-try-fallback/e2-try-fallback.formal-hook.json",
            argv_values,
        )
        self.assertFalse(
            any(arg.startswith(f"{pipeline.REPO_ROOT}/") for arg in argv_values)
        )

    def test_static_sample_plan_uses_static_formal_hook_smoke(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            plan = pipeline.plan_model_check_carrier_pipeline(
                sample_stem="e5-underdeclared-lineage",
                artifact_root=Path(temp_dir),
                run_label="model-check-carrier",
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
                pipeline.plan_model_check_carrier_pipeline(
                    sample_stem="e3-option-admit-chain",
                    artifact_root=Path(temp_dir),
                    run_label="model-check-carrier",
                    python_executable="/usr/bin/python3",
                )


class ModelCheckCarrierPipelineConformanceTests(unittest.TestCase):
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
        formal_hook = {
            "subject_ref": "e2_try_fallback",
            "contract_rows": [
                {"obligation_kind": "rollback_cut_non_interference"},
            ],
        }
        carriers = [
            {
                "subject_ref": "e2_try_fallback",
                "case": {
                    "obligation_kind": "rollback_cut_non_interference",
                    "evidence_refs": [{"ref_kind": "fixture", "ref_id": "e2_try_fallback"}],
                },
            }
        ]

        summary = pipeline.validate_conformance(formal_hook, carriers)

        self.assertEqual(summary.formal_hook_pair_count, 1)
        self.assertEqual(summary.model_check_carrier_count, 1)
        self.assertEqual(summary.matched_pairs, 1)

    def test_validate_conformance_rejects_pair_drift(self) -> None:
        formal_hook = {
            "subject_ref": "e2_try_fallback",
            "contract_rows": [
                {"obligation_kind": "rollback_cut_non_interference"},
            ],
        }
        carriers = [
            {
                "subject_ref": "e5_underdeclared_lineage",
                "case": {
                    "obligation_kind": "canonical_normalization_law",
                    "evidence_refs": [{"ref_kind": "fixture", "ref_id": "e5_underdeclared_lineage"}],
                },
            }
        ]

        with self.assertRaisesRegex(ValueError, "pair mismatch"):
            pipeline.validate_conformance(formal_hook, carriers)


if __name__ == "__main__":
    unittest.main()
