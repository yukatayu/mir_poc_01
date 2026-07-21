from __future__ import annotations

import importlib
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

try:
    provider_admission_samples = importlib.import_module("provider_admission_samples")
except ModuleNotFoundError:
    provider_admission_samples = None


REPO_ROOT = Path(__file__).resolve().parents[2]
EXPECTED_SAMPLE_IDS = [
    "eng-02-viewer-diagnostic-positive",
    "eng-02-over-capability-negative",
    "eng-02-missing-rollback-negative",
    "eng-02-native-disabled-negative",
    "eng-02-wasm-inventory-positive",
]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/provider_admission_samples.py", *args, "--format", "json"],
        cwd=REPO_ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if completed.returncode != 0:
        raise AssertionError(
            f"helper failed ({completed.returncode}): {completed.stderr}\n{completed.stdout}"
        )
    return json.loads(completed.stdout)


class ProviderAdmissionSamplesTests(unittest.TestCase):
    def test_helper_module_exists(self) -> None:
        self.assertIsNotNone(
            provider_admission_samples,
            "scripts/provider_admission_samples.py must exist for P-ENG-02",
        )

    def test_provider_root_and_matrix_exist(self) -> None:
        provider_root = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter"

        self.assertTrue(provider_root.exists(), "provider-adapter sample root is missing")
        self.assertTrue(
            (provider_root / "matrix.json").exists(),
            "provider-adapter matrix.json is missing",
        )

    def test_matrix_declares_provider_rows(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        matrix = provider_admission_samples.matrix()

        self.assertEqual(matrix["family"], "full_system_v1_provider_admission")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )

    def test_matrix_reports_executable_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["sample_count"], 5)
        self.assertEqual(payload["executable_count"], 5)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_viewer_diagnostic_row_reports_inventory_admission(self) -> None:
        payload = _run_helper("run", "eng-02-viewer-diagnostic-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["provider_id"], "viewer-diagnostic-exporter")
        self.assertEqual(payload["actual"]["target_id"], "diagnostic-adapter")
        self.assertEqual(payload["actual"]["terminal_outcome"], "inventory_admitted")
        self.assertFalse(payload["actual"]["execution_admitted"])
        self.assertEqual(payload["actual"]["diagnostic_codes"], [])
        self.assertEqual(payload["actual"]["local_split_launched_targets"], ["world-client"])

    def test_run_reads_committed_generated_evidence_without_writing(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        with patch.object(Path, "write_text", autospec=True) as write_text:
            payload = provider_admission_samples.run("eng-02-viewer-diagnostic-positive")

        write_text.assert_not_called()
        self.assertTrue(payload["matches_generated"])

    def test_run_rejects_mismatched_committed_generated_evidence(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        with tempfile.TemporaryDirectory() as tmpdir:
            generated_path = Path(tmpdir) / "provider-admission-report.json"
            generated_path.write_text("{}\n", encoding="utf-8")
            with patch.object(
                provider_admission_samples,
                "_row_generated_path",
                return_value=generated_path,
            ):
                payload = provider_admission_samples.run("eng-02-viewer-diagnostic-positive")

        self.assertIn("matches_generated", payload)
        self.assertFalse(payload["matches_generated"])

    def test_negative_over_capability_row_reports_rejection(self) -> None:
        payload = _run_helper("run", "eng-02-over-capability-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["provider_over_capability"],
        )

    def test_negative_missing_rollback_row_reports_rejection(self) -> None:
        payload = _run_helper("run", "eng-02-missing-rollback-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["missing_rollback_replay_cut_policy"],
        )

    def test_negative_native_disabled_row_reports_disabled_execution(self) -> None:
        payload = _run_helper("run", "eng-02-native-disabled-negative")

        self.assertFalse(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "native_execution_disabled")
        self.assertEqual(
            payload["actual"]["diagnostic_codes"],
            ["native_execution_disabled_by_default"],
        )

    def test_positive_wasm_inventory_row_reports_deferred_execution(self) -> None:
        payload = _run_helper("run", "eng-02-wasm-inventory-positive")

        self.assertTrue(payload["accepted"])
        self.assertEqual(payload["actual"]["terminal_outcome"], "wasm_inventory_only")
        self.assertFalse(payload["actual"]["execution_admitted"])
        self.assertIn(
            "sandboxed_wasm_execution_deferred",
            payload["actual"]["residual_obligation_codes"],
        )

    def test_helper_executes_cli_surface_with_repo_relative_paths(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        source = (
            REPO_ROOT
            / "samples"
            / "full-system-v1"
            / "provider-adapter"
            / "viewer-diagnostic-positive"
            / "main"
            / "src"
            / "viewer-diagnostic-positive.mir"
        )
        request = (
            REPO_ROOT
            / "samples"
            / "full-system-v1"
            / "provider-adapter"
            / "viewer-diagnostic-positive"
            / "requests"
            / "viewer-diagnostic.request.json"
        )
        provider = (
            REPO_ROOT
            / "samples"
            / "full-system-v1"
            / "provider-adapter"
            / "viewer-diagnostic-positive"
            / "providers"
            / "viewer-diagnostic.provider.json"
        )
        completed = subprocess.CompletedProcess(
            args=[],
            returncode=0,
            stdout=json.dumps(
                {
                    "accepted": True,
                    "local_split_report": {"target_reports": []},
                    "diagnostics": [],
                    "residual_obligations": [],
                }
            ),
            stderr="",
        )

        with patch.object(
            provider_admission_samples.subprocess,
            "run",
            return_value=completed,
        ) as patched_run:
            provider_admission_samples._run_provider_admission(
                source,
                request,
                provider,
                7,
            )

        command = patched_run.call_args.args[0]
        self.assertEqual(command[:6], ["cargo", "run", "-q", "-p", "mir-runtime", "--example"])
        self.assertEqual(
            command[6:9],
            [
                "mir_full_system_v1_provider_admission",
                "--",
                "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/main/src/viewer-diagnostic-positive.mir",
            ],
        )
        self.assertIn(
            "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/requests/viewer-diagnostic.request.json",
            command,
        )
        self.assertIn(
            "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/providers/viewer-diagnostic.provider.json",
            command,
        )

    def test_repo_relative_arg_preserves_external_paths(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        external_path = Path("/var/tmp/mirrorea-external/provider.mir")

        self.assertEqual(
            provider_admission_samples._repo_relative_arg(external_path),
            str(external_path),
        )

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(payload["passed"], EXPECTED_SAMPLE_IDS)

    def test_main_returns_failure_when_check_all_fails(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        with patch.object(
            provider_admission_samples,
            "check_all",
            return_value={"failed": ["forced-regression"], "validation_errors": []},
        ):
            self.assertEqual(provider_admission_samples.main(["check-all"]), 2)

    def test_check_all_reports_invalid_matrix_before_running_samples(self) -> None:
        if provider_admission_samples is None:
            self.fail("provider admission helper missing")

        status = {
            "family": "full_system_v1_provider_admission",
            "sample_root": "samples/full-system-v1/provider-adapter",
            "matrix_path": "samples/full-system-v1/provider-adapter/matrix.json",
            "sample_count": 5,
            "validation_errors": [{"kind": "missing_generated"}],
        }
        with patch.object(provider_admission_samples, "matrix", return_value=status), patch.object(
            provider_admission_samples, "run"
        ) as run:
            payload = provider_admission_samples.check_all()

        run.assert_not_called()
        self.assertEqual(payload["passed"], [])
        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], status["validation_errors"])


if __name__ == "__main__":
    unittest.main()
