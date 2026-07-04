from __future__ import annotations

import subprocess
import unittest
from pathlib import Path
import sys
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import alpha_network_docker_e2e as runner


class AlphaNetworkDockerE2ETests(unittest.TestCase):
    def test_list_contains_implemented_rows_only(self) -> None:
        rows = runner.list_samples()
        self.assertEqual(
            [row["sample_id"] for row in rows],
            ["NET-02", "NET-03", "NET-04", "NET-05", "NET-07", "NET-09"],
        )

    def test_list_reports_repo_relative_source_roots(self) -> None:
        rows = runner.list_samples()

        self.assertEqual(
            {row["source_root"] for row in rows},
            {"samples/alpha/network-docker"},
        )
        self.assertFalse(
            any(row["source_root"].startswith(f"{runner.REPO_ROOT}/") for row in rows)
        )

    def test_closeout_records_binary_compose_and_planned_rows(self) -> None:
        payload = runner.closeout()

        self.assertEqual(payload["sample_root"], "samples/alpha/network-docker")
        self.assertEqual(
            payload["compose_file"],
            "samples/alpha/network-docker/docker-compose.alpha-net.yml",
        )
        self.assertEqual(
            payload["binary_path"],
            "target/debug/examples/mirrorea_alpha_network_runtime",
        )
        self.assertEqual(
            payload["stage_c_required_rows"],
            ["NET-02", "NET-03", "NET-04", "NET-05", "NET-07", "NET-09"],
        )
        self.assertIn("NET-06", payload["planned_only_rows"])
        self.assertIn("NET-10", payload["planned_only_rows"])
        self.assertIn(
            "python3 scripts/alpha_network_docker_e2e.py stage-c-closeout --format json",
            payload["validation_floor"],
        )
        self.assertIn(
            "do not treat helper-local scripts/network_transport_samples.py as Alpha-0 Docker validation",
            payload["stop_lines"],
        )

    def test_validate_outputs_checks_net_02_expected_runtime_contract(self) -> None:
        row = runner._implemented_row("NET-02")
        sidecar = runner._load_expected_sidecar(row)
        world = {"sample_id": "NET-02"}
        participant = {
            "sample_id": "NET-02",
            "terminal_outcome": "accepted",
            "reason_family": None,
            "transport_surface": "tcp_process_boundary",
            "required_witness_refs": ["draw_pub#1"],
            "observer_route_trace": [{}, {}],
            "retained_later_refs": [
                "route_rebinding_no_shadow",
                "network_partition_explicit_failure",
                "transport_medium_change_preserves_contract",
                "production_wan_federation",
                "final_public_transport_abi",
            ],
        }
        runner._validate_outputs(
            "NET-02",
            row,
            sidecar,
            world,
            participant,
            compose_transport_surface="docker_compose_tcp",
        )

    def test_validate_outputs_checks_net_07_redaction(self) -> None:
        row = runner._implemented_row("NET-07")
        sidecar = runner._load_expected_sidecar(row)
        world = {"sample_id": "NET-07"}
        participant = {
            "sample_id": "NET-07",
            "terminal_outcome": "accepted",
            "reason_family": None,
            "transport_surface": "tcp_process_boundary",
            "observer_route_trace": [
                {
                    "redaction": "observer_safe_route_trace",
                    "payload_kind": "dispatch_receipt",
                }
            ]
            * 2,
            "retained_later_refs": [
                "route_rebinding_no_shadow",
                "network_partition_explicit_failure",
                "transport_medium_change_preserves_contract",
                "production_wan_federation",
                "final_public_transport_abi",
            ],
        }
        runner._validate_outputs(
            "NET-07",
            row,
            sidecar,
            world,
            participant,
            compose_transport_surface="docker_compose_tcp",
        )

    def test_validate_outputs_rejects_raw_trace_leak(self) -> None:
        row = runner._implemented_row("NET-07")
        sidecar = runner._load_expected_sidecar(row)
        world = {"sample_id": "NET-07"}
        participant = {
            "sample_id": "NET-07",
            "terminal_outcome": "accepted",
            "reason_family": None,
            "transport_surface": "tcp_process_boundary",
            "observer_route_trace": [
                {
                    "redaction": "observer_safe_route_trace",
                    "principal": "Alice",
                }
            ]
            * 2,
            "retained_later_refs": [
                "route_rebinding_no_shadow",
                "network_partition_explicit_failure",
                "transport_medium_change_preserves_contract",
                "production_wan_federation",
                "final_public_transport_abi",
            ],
        }
        with self.assertRaises(RuntimeError):
            runner._validate_outputs(
                "NET-07",
                row,
                sidecar,
                world,
                participant,
                compose_transport_surface="docker_compose_tcp",
            )

    def test_validate_outputs_checks_net_09_auth_lane(self) -> None:
        row = runner._implemented_row("NET-09")
        sidecar = runner._load_expected_sidecar(row)
        world = {"sample_id": "NET-09"}
        participant = {
            "sample_id": "NET-09",
            "terminal_outcome": "accepted",
            "reason_family": None,
            "transport_surface": "tcp_process_boundary",
            "auth_lane": {
                "auth_present": True,
                "preserved_separately": True,
                "bindings": [
                    "route=GamePlace[SugorokuGame#1]",
                    "transport=network_transport_lane",
                ],
            },
            "retained_later_refs": [
                "route_rebinding_no_shadow",
                "network_partition_explicit_failure",
                "transport_medium_change_preserves_contract",
                "production_wan_federation",
                "final_public_transport_abi",
            ],
        }
        runner._validate_outputs(
            "NET-09",
            row,
            sidecar,
            world,
            participant,
            compose_transport_surface="docker_compose_tcp",
        )

    def test_stage_c_closeout_requires_all_rows(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 6,
                "passed": ["NET-02", "NET-03", "NET-04", "NET-05", "NET-07", "NET-09"],
                "failed": [],
            },
        ):
            payload = runner.stage_c_closeout()
        self.assertTrue(payload["stage_c_complete"])
        self.assertFalse(payload["wan_federation_claimed"])
        self.assertFalse(payload["network_partition_complete"])
        self.assertFalse(payload["final_public_transport_abi_claimed"])

    def test_stage_c_closeout_surfaces_failures(self) -> None:
        with mock.patch.object(
            runner,
            "check_all",
            return_value={
                "sample_count": 6,
                "passed": ["NET-02"],
                "failed": [{"sample_id": "NET-03", "reason": "boom"}],
            },
        ):
            payload = runner.stage_c_closeout()
        self.assertFalse(payload["stage_c_complete"])
        self.assertEqual(payload["network_check"]["failed"][0]["sample_id"], "NET-03")

    def test_run_compose_uses_repo_relative_compose_file_arg(self) -> None:
        runs = []

        def fake_run(argv, **kwargs):
            runs.append((argv, kwargs))
            return mock.Mock(stdout="compose ok\n")

        with mock.patch.object(runner, "_check_binary_available"), mock.patch.object(
            runner, "_check_docker_available"
        ), mock.patch.object(
            runner.subprocess, "run", side_effect=fake_run
        ), mock.patch.object(
            runner, "_load_expected_sidecar", return_value={}
        ), mock.patch.object(
            runner, "_validate_outputs"
        ), mock.patch.object(
            runner,
            "_read_json_file",
            side_effect=[{"sample_id": "NET-02"}, {"sample_id": "NET-02"}],
        ):
            payload = runner._run_compose("NET-02")

        up_argv, up_kwargs = runs[0]
        down_argv, _down_kwargs = runs[1]
        self.assertEqual(
            up_argv[up_argv.index("-f") + 1],
            "samples/alpha/network-docker/docker-compose.alpha-net.yml",
        )
        self.assertEqual(
            down_argv[down_argv.index("-f") + 1],
            "samples/alpha/network-docker/docker-compose.alpha-net.yml",
        )
        self.assertFalse(any(str(arg).startswith(f"{runner.REPO_ROOT}/") for arg in up_argv))
        self.assertTrue(
            up_kwargs["env"]["MIRROREA_ALPHA_NETWORK_BINARY"].startswith(
                f"{runner.REPO_ROOT}/"
            )
        )
        self.assertEqual(
            payload["compose_file"],
            "samples/alpha/network-docker/docker-compose.alpha-net.yml",
        )

    def test_run_compose_sanitizes_docker_stdout_paths(self) -> None:
        temp_dir = "/tmp/alpha network output"

        def fake_run(argv, **_kwargs):
            return mock.Mock(
                stdout=(
                    f"using {runner.REPO_ROOT}/samples/alpha/network-docker\n"
                    f"wrote {temp_dir}/world.json\n"
                )
            )

        with mock.patch.object(runner, "_check_binary_available"), mock.patch.object(
            runner, "_check_docker_available"
        ), mock.patch.object(
            runner.tempfile, "TemporaryDirectory", return_value=TempDirStub(temp_dir)
        ), mock.patch.object(
            runner.subprocess, "run", side_effect=fake_run
        ), mock.patch.object(
            runner, "_load_expected_sidecar", return_value={}
        ), mock.patch.object(
            runner, "_validate_outputs"
        ), mock.patch.object(
            runner,
            "_read_json_file",
            side_effect=[{"sample_id": "NET-02"}, {"sample_id": "NET-02"}],
        ):
            payload = runner._run_compose("NET-02")

        stdout_text = "\n".join(payload["docker_stdout"])
        self.assertIn("samples/alpha/network-docker", stdout_text)
        self.assertIn("world.json", stdout_text)
        self.assertNotIn(str(runner.REPO_ROOT), stdout_text)
        self.assertNotIn(temp_dir, stdout_text)

    def test_run_compose_failure_sanitizes_repo_and_temp_paths(self) -> None:
        temp_dir = "/tmp/alpha network output"
        failed = subprocess.CalledProcessError(
            1,
            ["docker", "compose"],
            stderr=(
                f"failed at {runner.REPO_ROOT}/samples/alpha/network-docker "
                f"and {temp_dir}/participant.json"
            ),
        )

        with mock.patch.object(runner, "_check_binary_available"), mock.patch.object(
            runner, "_check_docker_available"
        ), mock.patch.object(
            runner.tempfile, "TemporaryDirectory", return_value=TempDirStub(temp_dir)
        ), mock.patch.object(
            runner.subprocess,
            "run",
            side_effect=[failed, mock.Mock(stdout="down ok\n")],
        ):
            with self.assertRaisesRegex(RuntimeError, "participant[.]json") as error:
                runner._run_compose("NET-02")

        self.assertNotIn(str(runner.REPO_ROOT), str(error.exception))
        self.assertNotIn(temp_dir, str(error.exception))

    def test_missing_compose_output_uses_display_path(self) -> None:
        with self.assertRaisesRegex(RuntimeError, "world[.]json") as error:
            runner._read_json_file(
                Path("/tmp/alpha network output/world.json"),
                display_path="world.json",
            )

        self.assertNotIn("/tmp/alpha network output", str(error.exception))

    def test_missing_binary_is_reported_honestly(self) -> None:
        original_binary = runner.BINARY_PATH
        try:
            runner.BINARY_PATH = Path("/tmp/definitely-missing-alpha-network-runtime")
            with self.assertRaises(RuntimeError):
                runner._check_binary_available()
        finally:
            runner.BINARY_PATH = original_binary

    def test_missing_repo_binary_failure_reason_uses_repo_relative_path(self) -> None:
        original_binary = runner.BINARY_PATH
        try:
            runner.BINARY_PATH = (
                runner.REPO_ROOT
                / "target/debug/examples/definitely-missing-alpha-network-runtime"
            )
            payload = runner.check_all()
        finally:
            runner.BINARY_PATH = original_binary

        self.assertEqual(len(payload["failed"]), 6)
        for row in payload["failed"]:
            self.assertIn(
                "target/debug/examples/definitely-missing-alpha-network-runtime",
                row["reason"],
            )
            self.assertNotIn(str(runner.REPO_ROOT), row["reason"])

    def test_format_pretty_for_run_payload(self) -> None:
        pretty = runner.format_pretty(
            {
                "sample_id": "NET-03",
                "participant": {"terminal_outcome": "rejected", "reason_family": "membership_freshness"},
            }
        )
        self.assertIn("NET-03 docker_compose_tcp", pretty)
        self.assertIn("membership_freshness", pretty)


class TempDirStub:
    def __init__(self, path: str) -> None:
        self.path = path

    def __enter__(self) -> str:
        return self.path

    def __exit__(self, *_args: object) -> None:
        return None


if __name__ == "__main__":
    unittest.main()
