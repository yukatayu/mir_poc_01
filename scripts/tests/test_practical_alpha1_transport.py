import json
import subprocess
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import practical_alpha1_transport as runner  # noqa: E402


class PracticalAlpha1TransportTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_repo_cli_arg_uses_repo_relative_paths_for_transport_files(self) -> None:
        self.assertEqual(
            runner.repo_cli_arg(runner.COMPOSE_FILE),
            "samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml",
        )
        self.assertEqual(
            runner.repo_cli_arg(runner.BINARY_PATH),
            "target/debug/examples/mir_practical_alpha1_transport",
        )

    def test_repo_cli_arg_keeps_external_paths_absolute(self) -> None:
        self.assertEqual(
            runner.repo_cli_arg(Path("/tmp/mirrorea-practical-alpha1-external")),
            "/tmp/mirrorea-practical-alpha1-external",
        )

    def test_list_samples_matches_transport_rows(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [row["sample_id"] for row in runner.IMPLEMENTED_ROWS],
        )
        self.assertTrue(all(row["family"] == "practical-alpha1-transport" for row in rows))

    def test_closeout_marks_stage_pa1_5_complete_once_all_rows_pass(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 7,
                "passed": [
                    "TR-A1-01",
                    "TR-A1-02",
                    "TR-A1-03",
                    "TR-A1-04",
                    "TR-A1-05",
                    "TR-A1-06",
                    "TR-A1-07",
                ],
                "failed": [],
                "transport_first_floor_complete": True,
                "transport_plan_boundary_present": True,
                "docker_row_complete": True,
                "stale_membership_negative_complete": True,
                "missing_capability_negative_complete": True,
                "missing_witness_negative_complete": True,
                "route_trace_complete": True,
                "auth_lane_complete": True,
                "stage_pa1_5_complete": True,
                "wan_federation_claimed": False,
                "save_load_claimed": False,
                "final_public_transport_abi_claimed": False,
            },
        ):
            payload = runner.closeout()
        self.assertTrue(payload["stage_pa1_5_complete"])
        self.assertEqual(
            payload["compose_file"],
            "samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml",
        )
        self.assertEqual(
            payload["binary_path"],
            "target/debug/examples/mir_practical_alpha1_transport",
        )
        serialized = json.dumps(payload)
        self.assertNotIn(f"{runner.REPO_ROOT}/", serialized)
        self.assertFalse(payload["wan_federation_claimed"])
        self.assertFalse(payload["save_load_claimed"])
        self.assertFalse(payload["final_public_transport_abi_claimed"])

    def test_local_transport_invocation_uses_repo_relative_package_path(self) -> None:
        package_dir = runner.REPO_ROOT / runner.IMPLEMENTED_ROWS[0]["package_dir"]
        captured: list[list[str]] = []

        def fake_subprocess_run(argv, **kwargs):
            captured.append(argv)
            return subprocess.CompletedProcess(
                argv,
                0,
                stdout=json.dumps({"sample_id": "TR-A1-01"}),
                stderr="",
            )

        with mock.patch.object(runner.subprocess, "run", side_effect=fake_subprocess_run):
            payload = runner._build_local_report(package_dir)

        self.assertEqual(payload["sample_id"], "TR-A1-01")
        self.assertIn(
            "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            captured[0],
        )
        self.assertFalse(
            any(arg.startswith(f"{runner.REPO_ROOT}/") for arg in captured[0])
        )

    def test_missing_transport_surface_error_uses_repo_relative_package_path(self) -> None:
        package_dir = runner.REPO_ROOT / runner.IMPLEMENTED_ROWS[0]["package_dir"]
        with mock.patch.object(runner, "_load_package", return_value={}):
            with self.assertRaisesRegex(
                RuntimeError,
                "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            ) as caught:
                runner._transport_surface(package_dir)
        self.assertNotIn(str(runner.REPO_ROOT), str(caught.exception))

    def test_local_transport_json_decode_error_redacts_repo_owned_paths(self) -> None:
        package_dir = runner.REPO_ROOT / runner.IMPLEMENTED_ROWS[0]["package_dir"]
        leaked_stdout = (
            f"stdout mentions {runner.REPO_ROOT}/"
            "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted"
        )

        def fake_subprocess_run(argv, **kwargs):
            return subprocess.CompletedProcess(argv, 0, stdout=leaked_stdout, stderr="")

        with mock.patch.object(runner.subprocess, "run", side_effect=fake_subprocess_run):
            with self.assertRaisesRegex(RuntimeError, "did not return JSON") as caught:
                runner._build_local_report(package_dir)
        message = str(caught.exception)
        self.assertIn(
            "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            message,
        )
        self.assertNotIn(str(runner.REPO_ROOT), message)

    def test_docker_transport_invocation_uses_repo_relative_compose_file(self) -> None:
        package_dir = runner.REPO_ROOT / runner.IMPLEMENTED_ROWS[1]["package_dir"]
        captured: list[tuple[list[str], dict[str, str] | None]] = []

        def fake_subprocess_run(argv, **kwargs):
            env = kwargs.get("env")
            captured.append((argv, env))
            if "up" in argv and env is not None:
                output_dir = Path(env["MIRROREA_PRACTICAL_ALPHA1_OUTPUT_DIR"])
                payload = {
                    "sample_id": "TR-A1-02",
                    "terminal_outcome": "accepted",
                    "reason_family": "accepted",
                }
                (output_dir / "world.json").write_text(json.dumps(payload))
                (output_dir / "participant.json").write_text(json.dumps(payload))
            return subprocess.CompletedProcess(argv, 0, stdout="", stderr="")

        with mock.patch.object(runner, "_check_docker_available"), mock.patch.object(
            runner, "_ensure_binary_available"
        ), mock.patch.object(runner.subprocess, "run", side_effect=fake_subprocess_run):
            payload = runner._build_docker_report(package_dir)

        self.assertEqual(payload["sample_id"], "TR-A1-02")
        up_argv, up_env = captured[0]
        self.assertIn(
            "samples/practical-alpha1/docker/docker-compose.practical-alpha1.yml",
            up_argv,
        )
        self.assertFalse(any(arg.startswith(f"{runner.REPO_ROOT}/") for arg in up_argv))
        self.assertIsNotNone(up_env)
        assert up_env is not None
        self.assertTrue(
            up_env["MIRROREA_PRACTICAL_ALPHA1_BINARY"].startswith(
                f"{runner.REPO_ROOT}/"
            )
        )
        self.assertTrue(
            up_env["MIRROREA_PRACTICAL_ALPHA1_PACKAGE_DIR"].startswith(
                f"{runner.REPO_ROOT}/"
            )
        )

    def test_docker_transport_failure_redacts_repo_owned_paths(self) -> None:
        package_dir = runner.REPO_ROOT / runner.IMPLEMENTED_ROWS[1]["package_dir"]
        leaked_stderr = (
            f"binary={runner.BINARY_PATH} "
            f"package={runner.REPO_ROOT}/"
            "samples/practical-alpha1/packages/tr-a1-02-docker-two-node-accepted"
        )
        error = subprocess.CalledProcessError(
            1,
            ["docker", "compose", "up"],
            stderr=leaked_stderr,
            output="",
        )

        with mock.patch.object(runner, "_check_docker_available"), mock.patch.object(
            runner, "_ensure_binary_available"
        ), mock.patch.object(runner.subprocess, "run", side_effect=[error, mock.Mock()]):
            with self.assertRaisesRegex(RuntimeError, "Docker Compose run") as caught:
                runner._build_docker_report(package_dir)
        message = str(caught.exception)
        self.assertIn("target/debug/examples/mir_practical_alpha1_transport", message)
        self.assertIn(
            "samples/practical-alpha1/packages/tr-a1-02-docker-two-node-accepted",
            message,
        )
        self.assertNotIn(str(runner.REPO_ROOT), message)

    def test_run_sample_accepts_exact_expected_report(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        with mock.patch.object(runner, "run_path", return_value=expected):
            observed = runner.run_sample(row["sample_id"])
        self.assertEqual(observed, expected)

    def test_run_sample_rejects_report_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        expected = json.loads(self.sidecar_path(row["expected_report"]).read_text())
        drifted = dict(expected)
        drifted["terminal_outcome"] = "rejected"
        with mock.patch.object(runner, "run_path", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected transport report drift"):
                runner.run_sample(row["sample_id"])

    def test_check_all_requires_transport_plan_boundary(self) -> None:
        with mock.patch.object(
            runner,
            "run_sample",
            return_value={
                "sample_id": "TR-A1-01",
                "terminal_outcome": "accepted",
            },
        ):
            payload = runner.check_all()
        self.assertTrue(payload["transport_first_floor_complete"])
        self.assertFalse(payload["transport_plan_boundary_present"])

    def test_check_all_failure_error_redacts_repo_owned_paths(self) -> None:
        leaked = (
            f"leak {runner.REPO_ROOT}/"
            "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted"
        )
        with mock.patch.object(runner, "run_sample", side_effect=RuntimeError(leaked)):
            payload = runner.check_all()
        self.assertEqual(len(payload["failed"]), len(runner.IMPLEMENTED_ROWS))
        error = payload["failed"][0]["error"]
        self.assertIn(
            "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            error,
        )
        self.assertNotIn(str(runner.REPO_ROOT), error)

    def test_normalize_argv_promotes_direct_package_path_to_check(self) -> None:
        args = runner.normalize_argv(
            [
                "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
                "--format",
                "json",
            ]
        )
        self.assertEqual(
            args,
            [
                "--format",
                "json",
                "check",
                "samples/practical-alpha1/packages/tr-a1-01-local-tcp-accepted",
            ],
        )


if __name__ == "__main__":
    unittest.main()
