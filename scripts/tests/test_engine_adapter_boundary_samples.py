from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
SCRIPT_PATH = REPO_ROOT / "scripts" / "engine_adapter_boundary_samples.py"
SAMPLE_ROOT = REPO_ROOT / "samples" / "product-alpha1" / "engine-adapter"
MATRIX_PATH = SAMPLE_ROOT / "matrix.json"

EXPECTED_PROVIDER_IDS = [
    "renderer",
    "input-device",
    "asset-loader",
    "physics-spatial-query",
    "host-runtime-bridge",
    "wasm-sandbox",
    "native-library-bridge",
    "viewer-diagnostic-exporter",
]


def load_module():
    if not SCRIPT_PATH.exists():
        raise AssertionError(f"missing helper script `{SCRIPT_PATH}`")
    spec = importlib.util.spec_from_file_location(
        "engine_adapter_boundary_samples",
        SCRIPT_PATH,
    )
    if spec is None or spec.loader is None:
        raise AssertionError(f"unable to load helper script `{SCRIPT_PATH}`")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class EngineAdapterBoundarySamplesTests(unittest.TestCase):
    def test_inventory_files_exist(self) -> None:
        self.assertTrue(SCRIPT_PATH.exists())
        self.assertTrue(MATRIX_PATH.exists())

    def test_matrix_file_lists_all_provider_rows(self) -> None:
        self.assertTrue(MATRIX_PATH.exists())
        matrix = json.loads(MATRIX_PATH.read_text(encoding="utf-8"))

        self.assertEqual(
            [row["provider_id"] for row in matrix["rows"]],
            EXPECTED_PROVIDER_IDS,
        )
        self.assertTrue(all(row["current_status"] == "planned_only" for row in matrix["rows"]))
        self.assertTrue(all("rollback_replay_cut_policy" in row for row in matrix["rows"]))
        self.assertTrue(all("native_execution_policy" in row for row in matrix["rows"]))
        self.assertTrue(all("wasm_execution_policy" in row for row in matrix["rows"]))

    def test_list_contains_all_planned_provider_rows(self) -> None:
        module = load_module()

        rows = module.list_providers()

        self.assertEqual([row["provider_id"] for row in rows], EXPECTED_PROVIDER_IDS)
        self.assertTrue(all(row["current_status"] == "planned_only" for row in rows))

    def test_matrix_reports_planned_only_inventory(self) -> None:
        module = load_module()

        result = module.matrix()

        self.assertEqual(result["provider_count"], 8)
        self.assertEqual(result["planned_count"], 8)
        self.assertEqual(result["executable_count"], 0)
        self.assertEqual(result["matrix_status"], "planned_only")
        self.assertEqual(result["world_semantics_owner"], "mir_mirrorea")
        self.assertFalse(result["workflow_ready"])

    def test_matrix_exposes_default_execution_gating(self) -> None:
        module = load_module()

        result = module.matrix()

        self.assertEqual(result["default_native_execution_policy"], "Disabled")
        self.assertEqual(result["default_wasm_execution_policy"], "InventoryOnly")
        self.assertTrue(
            all(row["native_execution_policy"] == "Disabled" for row in result["rows"])
        )
        self.assertTrue(
            all(
                row["wasm_execution_policy"] == "InventoryOnly"
                for row in result["rows"]
            )
        )

    def test_run_wasm_sandbox_rejects_as_planned_only(self) -> None:
        module = load_module()

        result = module.run_provider("wasm-sandbox")

        self.assertEqual(result["current_status"], "planned_only")
        self.assertEqual(result["terminal_outcome"], "planned_only")
        self.assertIn("inventory-only", result["rejection_reason"])

    def test_check_all_passes_when_provider_roots_exist(self) -> None:
        module = load_module()

        result = module.check_all()

        self.assertEqual(result["failed"], [])
        self.assertEqual(result["planned"], EXPECTED_PROVIDER_IDS)
        self.assertEqual(result["provider_count"], 8)

    def test_closeout_records_stop_lines_and_validation_floor(self) -> None:
        module = load_module()

        result = module.closeout()

        self.assertEqual(result["provider_ids"], EXPECTED_PROVIDER_IDS)
        self.assertIn("no arbitrary native package execution", result["stop_lines"])
        self.assertIn(
            "python3 scripts/engine_adapter_boundary_samples.py check-all --format json",
            result["validation_floor"],
        )

    def test_validate_rows_rejects_missing_required_field(self) -> None:
        module = load_module()

        with tempfile.TemporaryDirectory() as tmp:
            sample_root = Path(tmp)
            provider_root = sample_root / "renderer"
            provider_root.mkdir()
            (provider_root / "renderer.contract.json").write_text("{}", encoding="utf-8")
            rows = [
                {
                    "provider_id": "renderer",
                    "provider_kind": "renderer",
                    "root_name": "renderer",
                    "stage": "P-ENG-01",
                    "current_status": "planned_only",
                    "representative_source": "renderer/renderer.contract.json",
                    "input_schema": "frame_graph_commands",
                    "output_schema": "render_events",
                    "effect_row": ["RenderFrame"],
                    "failure_row": ["RendererUnavailable"],
                    "required_capability": ["render_frame"],
                    "authority_policy": {"semantic_authority_owner": "mir_mirrorea"},
                    "observation_policy": {"provider_receives_redacted_observation_only": True},
                    "redaction_policy": {"provider_may_emit_unredacted_debug": False},
                    "packet_boundary": {"boundary_name": "renderer_frame_packet"},
                    "resource_policy": {"semantic_state_owner": "mir_mirrorea"},
                    "sandbox_policy": {"sandbox_required": False},
                    "native_execution_policy": "Disabled",
                    "wasm_execution_policy": "InventoryOnly",
                    "rollback_replay_cut_policy": "Replayable",
                }
            ]

            errors = module.validate_rows(
                sample_root,
                rows,
                "Disabled",
                "InventoryOnly",
            )

        self.assertEqual(len(errors), 1)
        self.assertEqual(errors[0]["kind"], "missing_required_field")
        self.assertIn("ffi_boundary", errors[0]["detail"])

    def test_pretty_formats_check_all_summary(self) -> None:
        module = load_module()

        pretty = module.format_pretty(module.check_all())

        self.assertIn("CHECK-ALL SUMMARY", pretty)
        self.assertIn("planned-only: 8", pretty)

    def test_normalize_argv_hoists_root_format_before_known_subcommand(self) -> None:
        module = load_module()

        args = module.normalize_argv(["check-all", "--format", "json"])

        self.assertEqual(args, ["--format", "json", "check-all"])

    def test_normalize_argv_promotes_bare_provider_id_to_run(self) -> None:
        module = load_module()

        args = module.normalize_argv(["wasm-sandbox", "--format", "json"])

        self.assertEqual(args, ["--format", "json", "run", "wasm-sandbox"])


if __name__ == "__main__":
    unittest.main()
