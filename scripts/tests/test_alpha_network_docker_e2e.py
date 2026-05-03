from __future__ import annotations

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

    def test_closeout_records_binary_compose_and_planned_rows(self) -> None:
        payload = runner.closeout()

        self.assertIn("docker-compose.alpha-net.yml", payload["compose_file"])
        self.assertIn("mirrorea_alpha_network_runtime", payload["binary_path"])
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

    def test_missing_binary_is_reported_honestly(self) -> None:
        original_binary = runner.BINARY_PATH
        try:
            runner.BINARY_PATH = Path("/tmp/definitely-missing-alpha-network-runtime")
            with self.assertRaises(RuntimeError):
                runner._check_binary_available()
        finally:
            runner.BINARY_PATH = original_binary

    def test_format_pretty_for_run_payload(self) -> None:
        pretty = runner.format_pretty(
            {
                "sample_id": "NET-03",
                "participant": {"terminal_outcome": "rejected", "reason_family": "membership_freshness"},
            }
        )
        self.assertIn("NET-03 docker_compose_tcp", pretty)
        self.assertIn("membership_freshness", pretty)


if __name__ == "__main__":
    unittest.main()
