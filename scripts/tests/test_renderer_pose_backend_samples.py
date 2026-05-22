from __future__ import annotations

import importlib
import json
import subprocess
import sys
import unittest
from unittest import mock
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

try:
    renderer_pose_backend_samples = importlib.import_module(
        "renderer_pose_backend_samples"
    )
except ModuleNotFoundError:
    renderer_pose_backend_samples = None


REPO_ROOT = Path(__file__).resolve().parents[2]
EXPECTED_SAMPLE_IDS = [
    "eng-03-renderer-pose-positive",
    "eng-03-renderer-pose-split-frame-negative",
    "eng-03-renderer-pose-reacquire-negative",
]


def _run_helper(*args: str) -> dict:
    completed = subprocess.run(
        ["python3", "scripts/renderer_pose_backend_samples.py", *args, "--format", "json"],
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


class RendererPoseBackendSamplesTests(unittest.TestCase):
    def test_helper_module_exists(self) -> None:
        self.assertIsNotNone(
            renderer_pose_backend_samples,
            "scripts/renderer_pose_backend_samples.py must exist for P-ENG-03",
        )

    def test_provider_root_and_renderer_matrix_exist(self) -> None:
        provider_root = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter"

        self.assertTrue(provider_root.exists(), "provider-adapter sample root is missing")
        self.assertTrue(
            (provider_root / "renderer-pose-matrix.json").exists(),
            "provider-adapter renderer-pose-matrix.json is missing",
        )

    def test_matrix_declares_renderer_pose_rows(self) -> None:
        if renderer_pose_backend_samples is None:
            self.fail("renderer pose backend helper missing")

        matrix = renderer_pose_backend_samples.matrix()

        self.assertEqual(matrix["family"], "full_system_v1_renderer_pose_backend")
        self.assertEqual(
            [row["sample_id"] for row in matrix["rows"]],
            EXPECTED_SAMPLE_IDS,
        )

    def test_matrix_reports_executable_rows(self) -> None:
        payload = _run_helper("matrix")

        self.assertEqual(payload["sample_count"], 3)
        self.assertEqual(payload["executable_count"], 3)
        self.assertEqual(payload["validation_errors"], [])

    def test_positive_row_reports_delivery_admission(self) -> None:
        payload = _run_helper("run", "eng-03-renderer-pose-positive")

        self.assertTrue(payload["accepted"])
        self.assertTrue(payload["actual"]["delivery_admitted"])
        self.assertEqual(payload["actual"]["provider_id"], "renderer-pose-backend")
        self.assertEqual(payload["actual"]["target_id"], "renderer-adapter")
        self.assertEqual(payload["actual"]["terminal_outcome"], "delivery_admitted")
        self.assertEqual(payload["actual"]["pose_snapshot_frontier"], "snapshot#avatar-017")
        self.assertEqual(
            payload["actual"]["matched_packet_schema_refs"],
            ["packet.renderer.pose_snapshot"],
        )

    def test_split_frame_negative_row_reports_posegraph_block(self) -> None:
        payload = _run_helper("run", "eng-03-renderer-pose-split-frame-negative")

        self.assertFalse(payload["accepted"])
        self.assertFalse(payload["actual"]["delivery_admitted"])
        self.assertEqual(
            payload["actual"]["terminal_outcome"],
            "blocked_posegraph_violation_export",
        )
        self.assertEqual(payload["actual"]["blocked_reason"], "no_split_frame")

    def test_reacquire_negative_row_reports_runtime_rejection(self) -> None:
        payload = _run_helper("run", "eng-03-renderer-pose-reacquire-negative")

        self.assertFalse(payload["accepted"])
        self.assertFalse(payload["actual"]["delivery_admitted"])
        self.assertEqual(
            payload["actual"]["terminal_outcome"],
            "blocked_posegraph_runtime_rejection",
        )
        self.assertEqual(payload["actual"]["blocked_reason"], "reacquire_required")

    def test_check_all_passes_every_row(self) -> None:
        payload = _run_helper("check-all")

        self.assertEqual(payload["failed"], [])
        self.assertEqual(payload["validation_errors"], [])
        self.assertEqual(payload["passed"], EXPECTED_SAMPLE_IDS)

    def test_helper_executes_cli_surface(self) -> None:
        if renderer_pose_backend_samples is None:
            self.fail("renderer pose backend helper missing")

        completed = subprocess.CompletedProcess(
            args=[],
            returncode=0,
            stdout=json.dumps(
                {
                    "accepted": True,
                    "delivery_admitted": True,
                    "terminal_outcome": "delivery_admitted",
                    "blocked_reason": None,
                    "provider_id": "renderer-pose-backend",
                    "provider_kind": "renderer",
                    "target_id": "renderer-adapter",
                    "target_provider_policy": "provider_inventory_only",
                    "pose_snapshot_frontier": "snapshot#avatar-017",
                    "delivered_pose_snapshot_ref": "snapshot#avatar-017",
                    "delivered_nodes": [],
                    "matched_packet_schema_refs": ["packet.renderer.pose_snapshot"],
                    "matched_ffi_schema_refs": [],
                    "diagnostics": [],
                    "residual_obligations": [],
                    "provider_admission_report": {"terminal_outcome": "inventory_admitted"},
                    "posegraph_runtime_report": {"terminal_outcome": "Accepted"},
                }
            ),
            stderr="",
        )
        source = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter" / "renderer-pose-positive" / "main" / "src" / "renderer-pose-positive.mir"
        request = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter" / "renderer-pose-positive" / "projection.request.json"
        provider = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter" / "renderer-pose-positive" / "provider.manifest.json"
        posegraph = REPO_ROOT / "samples" / "full-system-v1" / "provider-adapter" / "renderer-pose-positive" / "package.mir.json"

        with mock.patch.object(
            renderer_pose_backend_samples.subprocess, "run", return_value=completed
        ) as patched_run:
            renderer_pose_backend_samples._run_renderer_pose_backend(
                source, request, provider, posegraph, 0
            )

        command = patched_run.call_args.args[0]
        self.assertEqual(command[:5], ["cargo", "run", "-q", "-p", "mirrorea-cli"])
        self.assertEqual(command[5:8], ["--", "render-pose-backend-v1", str(source)])


if __name__ == "__main__":
    unittest.main()
