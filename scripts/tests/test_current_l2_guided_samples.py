import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_guided_samples as guided  # noqa: E402


class CurrentL2GuidedSamplesTests(unittest.TestCase):
    def test_problem_specs_cover_two_big_problems(self) -> None:
        specs = guided.problem_specs()

        self.assertEqual(set(specs.keys()), {"problem1", "problem2"})

    def test_problem1_show_text_mentions_primary_and_supporting_samples(self) -> None:
        spec = guided.problem_specs()["problem1"]

        text = guided.render_problem_guide(spec)

        self.assertIn("Problem 1", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p10-typed-authorized-fingerprint-declassification", text)
        self.assertIn("p12-typed-classified-fingerprint-publication-block", text)
        self.assertIn("verification_preview", text)
        self.assertIn("artifact_preview", text)

    def test_problem2_primary_run_commands_use_representative_pair(self) -> None:
        spec = guided.problem_specs()["problem2"]

        commands = guided.build_run_commands(spec, output_format="pretty", include_all=False)
        command_texts = [" ".join(command) for command in commands]

        self.assertEqual(len(commands), 2)
        self.assertTrue(
            any("p07-dice-late-join-visible-history.txt" in text for text in command_texts)
        )
        self.assertTrue(
            any("p08-dice-stale-reconnect-refresh.txt" in text for text in command_texts)
        )
        self.assertFalse(
            any("p09-dice-delegated-rng-provider-placement.txt" in text for text in command_texts)
        )

    def test_problem2_all_run_commands_include_reserve_and_negative_samples(self) -> None:
        spec = guided.problem_specs()["problem2"]

        commands = guided.build_run_commands(spec, output_format="json", include_all=True)
        command_texts = [" ".join(command) for command in commands]

        self.assertTrue(
            any("p09-dice-delegated-rng-provider-placement.txt" in text for text in command_texts)
        )
        self.assertTrue(
            any(
                "p13-dice-late-join-missing-publication-witness.txt" in text
                for text in command_texts
            )
        )
        self.assertTrue(
            any(
                "p14-dice-late-join-handoff-before-publication.txt" in text
                for text in command_texts
            )
        )

    def test_problem1_residual_bundle_summarizes_reached_vs_bridge_only(self) -> None:
        spec = guided.problem_specs()["problem1"]

        reports = {
            "p06-typed-proof-owner-handoff": {
                "verification_preview": {"formal_hook_status": "reached"},
                "artifact_preview": {"proof_notebook_review_units": [{}]},
                "typed_checker_hint_preview": {"status": "guarded_not_reached"},
                "theorem_result_object_preview": {"status": "reached", "bridge_floor_refs": []},
                "theorem_final_public_contract_reopen_threshold": {
                    "status": "reached",
                    "bridge_floor_refs": [],
                },
                "model_check_public_checker_preview": {
                    "status": "reached",
                    "bridge_floor_refs": [],
                },
                "model_check_final_public_contract_reopen_threshold": {
                    "status": "reached",
                    "bridge_floor_refs": [],
                },
            },
            "p10-typed-authorized-fingerprint-declassification": {
                "verification_preview": {"formal_hook_status": "reached"},
                "artifact_preview": {"proof_notebook_review_units": [{}]},
                "typed_checker_hint_preview": {"status": "reached"},
                "theorem_result_object_preview": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:a", "bridge:b"],
                },
                "theorem_final_public_contract_reopen_threshold": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:c"],
                },
                "model_check_public_checker_preview": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:d", "bridge:e", "bridge:f"],
                },
                "model_check_final_public_contract_reopen_threshold": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:g"],
                },
            },
        }

        def fake_loader(sample: guided.GuidedSample) -> dict:
            return reports.get(
                sample.sample_id,
                {
                    "verification_preview": {"formal_hook_status": "reached"},
                    "artifact_preview": {"proof_notebook_review_units": [{}]},
                    "typed_checker_hint_preview": {"status": "reached"},
                    "theorem_result_object_preview": {
                        "status": "guarded_not_reached",
                        "bridge_floor_refs": ["bridge:default"],
                    },
                    "theorem_final_public_contract_reopen_threshold": {
                        "status": "guarded_not_reached",
                        "bridge_floor_refs": ["bridge:default"],
                    },
                    "model_check_public_checker_preview": {
                        "status": "guarded_not_reached",
                        "bridge_floor_refs": ["bridge:default"],
                    },
                    "model_check_final_public_contract_reopen_threshold": {
                        "status": "guarded_not_reached",
                        "bridge_floor_refs": ["bridge:default"],
                    },
                },
            )

        rows = guided.build_problem1_residual_rows(spec, loader=fake_loader)

        first = rows[0]
        second = rows[1]

        self.assertEqual(first.sample_id, "p06-typed-proof-owner-handoff")
        self.assertEqual(first.theorem_preview, "reached")
        self.assertEqual(first.model_check_preview, "reached")
        self.assertEqual(first.residual_reading, "public-seam representative")

        self.assertEqual(second.sample_id, "p10-typed-authorized-fingerprint-declassification")
        self.assertEqual(second.theorem_preview, "bridge-only(2)")
        self.assertEqual(second.theorem_reopen, "bridge-only(1)")
        self.assertEqual(second.model_check_preview, "bridge-only(3)")
        self.assertEqual(second.model_check_reopen, "bridge-only(1)")
        self.assertEqual(second.residual_reading, "checker-adjacent bridge-floor")

    def test_problem1_residual_matrix_text_mentions_columns_and_notes(self) -> None:
        spec = guided.problem_specs()["problem1"]
        rows = [
            guided.Problem1ResidualRow(
                sample_id="p06-typed-proof-owner-handoff",
                primary=True,
                typed_hint="guarded",
                theorem_preview="reached",
                theorem_reopen="reached",
                model_check_preview="reached",
                model_check_reopen="reached",
                residual_reading="public-seam representative",
            ),
            guided.Problem1ResidualRow(
                sample_id="p10-typed-authorized-fingerprint-declassification",
                primary=False,
                typed_hint="reached",
                theorem_preview="bridge-only(2)",
                theorem_reopen="bridge-only(1)",
                model_check_preview="bridge-only(3)",
                model_check_reopen="bridge-only(1)",
                residual_reading="checker-adjacent bridge-floor",
            ),
        ]

        text = guided.render_problem1_residual_matrix(spec, rows)

        self.assertIn("Problem 1 residual bundle", text)
        self.assertIn("theorem preview", text)
        self.assertIn("model-check reopen", text)
        self.assertIn("bridge-only(n)", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("checker-adjacent bridge-floor", text)

    def test_main_matrix_command_uses_problem1_residual_renderer(self) -> None:
        fake_text = "Problem 1 residual bundle\n..."

        with mock.patch.object(guided, "render_problem1_residual_matrix_from_runtime", return_value=fake_text):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["matrix", "problem1"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("Problem 1 residual bundle" in call.args[0] for call in write.mock_calls))

    def test_problem1_bundle_text_mentions_commands_lean_and_doc_refs(self) -> None:
        spec = guided.problem_specs()["problem1"]

        text = guided.render_problem_bundle(spec)

        self.assertIn("Problem 1 theorem-first pilot bundle", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem1", text)
        self.assertIn("specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md", text)
        self.assertIn("final public theorem contract", text)

    def test_problem2_bundle_text_mentions_representative_reserve_negative_and_doc_refs(self) -> None:
        spec = guided.problem_specs()["problem2"]

        text = guided.render_problem_bundle(spec)

        self.assertIn("Problem 2 authoritative-room scenario bundle", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p09-dice-delegated-rng-provider-placement", text)
        self.assertIn("p13-dice-late-join-missing-publication-witness", text)
        self.assertIn("samples/lean/current-l2/p07-dice-late-join-visible-history/p07-dice-late-join-visible-history.bundle.json", text)
        self.assertIn("specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md", text)
        self.assertIn("final public witness schema", text)

    def test_main_bundle_command_uses_bundle_renderer(self) -> None:
        fake_text = "Problem 2 authoritative-room scenario bundle\n..."

        with mock.patch.object(guided, "render_problem_bundle", return_value=fake_text):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["bundle", "problem2"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("Problem 2 authoritative-room scenario bundle" in call.args[0] for call in write.mock_calls)
        )

    def test_problem2_residual_bundle_summarizes_first_line_reserve_and_negative(self) -> None:
        spec = guided.problem_specs()["problem2"]

        reports = {
            "p07-dice-late-join-visible-history": {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "order_handoff_source_surface_artifact_actual_adoption": {"status": "reached"},
                "authoritative_room_first_scenario_actual_adoption": {"status": "reached"},
                "order_handoff_witness_provider_public_seam_compression": {"status": "reached"},
                "authoritative_room_reserve_strengthening_lane": {
                    "status": "reached",
                    "witness_strengthening_status": "reached",
                    "delegated_rng_service_status": "guarded_not_reached",
                    "model_check_second_line_status": "reached",
                },
            },
            "p09-dice-delegated-rng-provider-placement": {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "order_handoff_source_surface_artifact_actual_adoption": {
                    "status": "guarded_not_reached",
                },
                "authoritative_room_first_scenario_actual_adoption": {
                    "status": "guarded_not_reached",
                },
                "order_handoff_witness_provider_public_seam_compression": {
                    "status": "guarded_not_reached",
                },
                "authoritative_room_reserve_strengthening_lane": {
                    "status": "reached",
                    "witness_strengthening_status": "guarded_not_reached",
                    "delegated_rng_service_status": "reached",
                    "model_check_second_line_status": "reached",
                },
            },
            "p13-dice-late-join-missing-publication-witness": {
                "checker_floor": {"static_gate": {"verdict": "underdeclared"}},
                "order_handoff_source_surface_artifact_actual_adoption": {
                    "status": "guarded_not_reached",
                },
                "authoritative_room_first_scenario_actual_adoption": {
                    "status": "guarded_not_reached",
                },
                "order_handoff_witness_provider_public_seam_compression": {
                    "status": "guarded_not_reached",
                },
                "authoritative_room_reserve_strengthening_lane": {
                    "status": "guarded_not_reached",
                    "witness_strengthening_status": "guarded_not_reached",
                    "delegated_rng_service_status": "guarded_not_reached",
                    "model_check_second_line_status": "guarded_not_reached",
                },
            },
        }

        def fake_loader(sample: guided.GuidedSample) -> dict:
            return reports.get(
                sample.sample_id,
                {
                    "checker_floor": {"static_gate": {"verdict": "valid"}},
                    "order_handoff_source_surface_artifact_actual_adoption": {"status": "reached"},
                    "authoritative_room_first_scenario_actual_adoption": {"status": "reached"},
                    "order_handoff_witness_provider_public_seam_compression": {"status": "reached"},
                    "authoritative_room_reserve_strengthening_lane": {
                        "status": "reached",
                        "witness_strengthening_status": "guarded_not_reached",
                        "delegated_rng_service_status": "guarded_not_reached",
                        "model_check_second_line_status": "reached",
                    },
                },
            )

        rows = guided.build_problem2_residual_rows(spec, loader=fake_loader)

        self.assertEqual(rows[0].sample_id, "p07-dice-late-join-visible-history")
        self.assertEqual(rows[0].reserve_detail, "witness+model-check")
        self.assertEqual(rows[0].residual_reading, "first-line representative")

        self.assertEqual(rows[2].sample_id, "p09-dice-delegated-rng-provider-placement")
        self.assertEqual(rows[2].reserve_detail, "delegated-rng+model-check")
        self.assertEqual(rows[2].residual_reading, "reserve practical route")

        self.assertEqual(rows[3].sample_id, "p13-dice-late-join-missing-publication-witness")
        self.assertEqual(rows[3].residual_reading, "negative static-stop")

    def test_problem2_residual_matrix_text_mentions_first_line_and_reserve(self) -> None:
        spec = guided.problem_specs()["problem2"]
        rows = [
            guided.Problem2ResidualRow(
                sample_id="p07-dice-late-join-visible-history",
                primary=True,
                static_gate="valid",
                surface="reached",
                first_line="reached",
                public_seam="reached",
                reserve_lane="reached",
                reserve_detail="witness+model-check",
                residual_reading="first-line representative",
            ),
            guided.Problem2ResidualRow(
                sample_id="p09-dice-delegated-rng-provider-placement",
                primary=False,
                static_gate="valid",
                surface="guarded",
                first_line="guarded",
                public_seam="guarded",
                reserve_lane="reached",
                reserve_detail="delegated-rng+model-check",
                residual_reading="reserve practical route",
            ),
        ]

        text = guided.render_problem2_residual_matrix(spec, rows)

        self.assertIn("Problem 2 residual bundle", text)
        self.assertIn("reserve detail", text)
        self.assertIn("first-line representative", text)
        self.assertIn("delegated-rng+model-check", text)


if __name__ == "__main__":
    unittest.main()
