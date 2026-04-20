import subprocess
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
        self.assertIn("samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt", text)
        self.assertIn("samples/problem-bundles/problem1-typed-theorem-model-check.md", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-theorem problem1", text)
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
        self.assertIn("samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt", text)
        self.assertIn("samples/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt", text)
        self.assertIn("samples/problem-bundles/problem2-order-handoff-shared-space.md", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-scenario problem2", text)
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

    def test_problem2_scenario_emit_rows_track_representative_reserve_and_negative(self) -> None:
        output_dir = guided.REPO_ROOT / "target" / "guided-sample-tests" / "problem2-scenario-bundle"

        def fake_emitter(sample_id: str, output_path: Path) -> dict[str, object]:
            payload = {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "runtime": {"terminal_outcome": "Success"},
                "order_handoff_source_surface_artifact_actual_adoption": {"status": "reached"},
                "authoritative_room_first_scenario_actual_adoption": {"status": "reached"},
                "order_handoff_witness_provider_public_seam_compression": {"status": "reached"},
                "authoritative_room_reserve_strengthening_lane": {"status": "guarded_not_reached"},
            }
            if sample_id == "p09-dice-delegated-rng-provider-placement":
                payload["authoritative_room_first_scenario_actual_adoption"] = {"status": "guarded_not_reached"}
                payload["authoritative_room_reserve_strengthening_lane"] = {
                    "status": "reached",
                    "delegated_rng_service_status": "reached",
                    "model_check_second_line_status": "reached",
                }
            if sample_id.startswith("p13") or sample_id.startswith("p14"):
                payload["checker_floor"] = {"static_gate": {"verdict": "underdeclared"}}
                payload["runtime"] = {"terminal_outcome": None}
                payload["authoritative_room_first_scenario_actual_adoption"] = {"status": "guarded_not_reached"}
            return payload

        rows = guided.build_problem2_scenario_emit_rows(output_dir=output_dir, emitter=fake_emitter)

        self.assertEqual(rows[0].sample_id, "p07-dice-late-join-visible-history")
        self.assertEqual(rows[0].reading, "first-line representative")
        self.assertEqual(rows[2].sample_id, "p09-dice-delegated-rng-provider-placement")
        self.assertEqual(rows[2].reading, "reserve practical route")
        self.assertEqual(rows[3].reading, "negative static-stop")
        self.assertIn("target/guided-sample-tests/problem2-scenario-bundle", rows[4].output_path)

    def test_problem2_scenario_emit_command_uses_prototype_sample_path(self) -> None:
        command = guided.problem2_scenario_emit_command(
            "p07-dice-late-join-visible-history",
        )

        self.assertEqual(
            command[9],
            "samples/prototype/current-l2-order-handoff/p07-dice-late-join-visible-history.txt",
        )
        self.assertEqual(command[-2:], ["--format", "json"])

    def test_problem2_scenario_emit_text_mentions_output_dir_command_and_samples(self) -> None:
        output_dir = guided.REPO_ROOT / "target" / "guided-sample-tests" / "problem2-scenario-bundle"
        rows = [
            guided.ProblemScenarioEmitRow(
                sample_id="p07-dice-late-join-visible-history",
                reading="first-line representative",
                output_path="target/guided-sample-tests/problem2-scenario-bundle/p07-dice-late-join-visible-history.run.json",
                static_gate="valid",
                terminal_outcome="success",
                first_line_status="reached",
                reserve_lane_status="guarded",
            ),
            guided.ProblemScenarioEmitRow(
                sample_id="p09-dice-delegated-rng-provider-placement",
                reading="reserve practical route",
                output_path="target/guided-sample-tests/problem2-scenario-bundle/p09-dice-delegated-rng-provider-placement.run.json",
                static_gate="valid",
                terminal_outcome="success",
                first_line_status="guarded",
                reserve_lane_status="reached",
            ),
        ]

        text = guided.render_problem2_scenario_emit(
            guided.problem_specs()["problem2"],
            rows,
            output_dir=output_dir,
        )

        self.assertIn("Problem 2 authoritative-room runnable scenario loop", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-scenario problem2", text)
        self.assertIn("target/guided-sample-tests/problem2-scenario-bundle", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p09-dice-delegated-rng-provider-placement", text)
        self.assertIn("final public witness/provider/artifact contract", text)

    def test_main_emit_scenario_problem2_uses_runtime_renderer(self) -> None:
        fake_text = "Problem 2 authoritative-room runnable scenario loop\n..."

        with mock.patch.object(
            guided,
            "render_problem2_scenario_emit_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["emit-scenario", "problem2"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("Problem 2 authoritative-room runnable scenario loop" in call.args[0] for call in write.mock_calls)
        )

    def test_problem1_theorem_emit_rows_track_representative_and_support_pair(self) -> None:
        output_dir = guided.REPO_ROOT / "target" / "guided-sample-tests" / "problem1-theorem-pilot"

        def fake_emitter(sample_id: str, output_path: Path) -> dict[str, object]:
            return {
                "sample_id": sample_id,
                "pilot_status": "reached",
                "pilot_subject_ref": f"fixture:{sample_id}",
                "lean_stub_artifacts": [{"theorem_name": "demo"}],
                "principal_review_unit_refs": [f"review:{sample_id}"],
                "repo_local_emitted_artifact_refs": [f"artifact:{sample_id}"],
                "compare_floor_refs": [f"compare:{sample_id}"],
            }

        rows = guided.build_problem1_theorem_emit_rows(output_dir=output_dir, emitter=fake_emitter)

        self.assertEqual(
            [row.sample_id for row in rows],
            [
                "p06-typed-proof-owner-handoff",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
            ],
        )
        self.assertEqual(rows[0].reading, "representative theorem-first sample")
        self.assertEqual(rows[1].reading, "theorem-reached support sample")
        self.assertEqual(rows[2].pilot_status, "reached")
        self.assertEqual(rows[0].lean_stub_artifact_count, 1)
        self.assertIn("target/guided-sample-tests/problem1-theorem-pilot", rows[0].output_path)

    def test_problem1_theorem_emit_command_uses_prototype_sample_path(self) -> None:
        command = guided.problem1_theorem_emit_command(
            "p06-typed-proof-owner-handoff",
            guided.REPO_ROOT / "target" / "guided-sample-tests" / "p06.bundle.json",
        )

        self.assertEqual(
            command[8],
            "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt",
        )
        self.assertIn("--host-plan", command)
        self.assertIn(
            "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.host-plan.json",
            command,
        )

    def test_problem1_theorem_emit_text_mentions_output_dir_command_and_samples(self) -> None:
        output_dir = guided.REPO_ROOT / "target" / "guided-sample-tests" / "problem1-theorem-pilot"
        rows = [
            guided.ProblemTheoremEmitRow(
                sample_id="p06-typed-proof-owner-handoff",
                reading="representative theorem-first sample",
                output_path="target/guided-sample-tests/problem1-theorem-pilot/p06-typed-proof-owner-handoff.lean-bundle.json",
                pilot_status="reached",
                pilot_subject_ref="fixture:p06",
                lean_stub_artifact_count=2,
                principal_review_unit_ref_count=2,
                repo_local_emitted_artifact_ref_count=1,
                compare_floor_ref_count=2,
            ),
            guided.ProblemTheoremEmitRow(
                sample_id="p07-dice-late-join-visible-history",
                reading="theorem-reached support sample",
                output_path="target/guided-sample-tests/problem1-theorem-pilot/p07-dice-late-join-visible-history.lean-bundle.json",
                pilot_status="reached",
                pilot_subject_ref="fixture:p07",
                lean_stub_artifact_count=1,
                principal_review_unit_ref_count=1,
                repo_local_emitted_artifact_ref_count=1,
                compare_floor_ref_count=1,
            ),
        ]

        text = guided.render_problem1_theorem_emit(
            guided.problem_specs()["problem1"],
            rows,
            output_dir=output_dir,
        )

        self.assertIn("Problem 1 theorem-first emitted artifact loop", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-theorem problem1", text)
        self.assertIn("target/guided-sample-tests/problem1-theorem-pilot", text)
        self.assertIn("pilot-summary.md", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("final public theorem contract", text)

    def test_problem1_theorem_emit_manifest_writes_pilot_index_files(self) -> None:
        output_dir = guided.REPO_ROOT / "target" / "guided-sample-tests" / "problem1-theorem-pilot-index"

        def fake_emitter(sample_id: str, output_path: guided.Path) -> dict[str, object]:
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text("{\"ok\": true}", encoding="utf-8")
            return {
                "pilot_status": "reached",
                "pilot_subject_ref": f"fixture:{sample_id}",
                "lean_stub_artifacts": [f"lean:{sample_id}"],
                "principal_review_unit_refs": [f"review:{sample_id}"],
                "repo_local_emitted_artifact_refs": [f"artifact:{sample_id}"],
                "compare_floor_refs": [f"compare:{sample_id}"],
            }

        manifest = guided.build_problem1_theorem_emit_manifest(
            guided.problem_specs()["problem1"],
            output_dir=output_dir,
            emitter=fake_emitter,
        )

        summary_md = output_dir / "pilot-summary.md"
        summary_json = output_dir / "pilot-summary.json"
        self.assertEqual(
            manifest["pilot_notebook_index_markdown"],
            guided.display_path(summary_md),
        )
        self.assertEqual(
            manifest["pilot_notebook_index_json"],
            guided.display_path(summary_json),
        )
        self.assertTrue(summary_md.is_file())
        self.assertTrue(summary_json.is_file())
        self.assertIn("Problem 1 theorem-first emitted artifact loop", summary_md.read_text(encoding="utf-8"))
        payload = guided.json.loads(summary_json.read_text(encoding="utf-8"))
        self.assertEqual(payload["problem_id"], "problem1")
        self.assertEqual(
            [row["sample_id"] for row in payload["rows"]],
            [
                "p06-typed-proof-owner-handoff",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
            ],
        )

    def test_main_emit_theorem_problem1_uses_runtime_renderer(self) -> None:
        fake_text = "Problem 1 theorem-first emitted artifact loop\n..."

        with mock.patch.object(
            guided,
            "render_problem1_theorem_emit_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["emit-theorem", "problem1"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("Problem 1 theorem-first emitted artifact loop" in call.args[0] for call in write.mock_calls)
        )

    def test_auditable_authority_witness_emit_text_mentions_output_dir_command_and_samples(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "auditable-authority-witness"
        )
        rows = [
            guided.ReservePackageEmitRow(
                sample_id="p07-dice-late-join-visible-history",
                reading="witness-strengthening reached",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "auditable-authority-witness/p07-dice-late-join-visible-history.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="witness+model-check",
                witness_strengthening_status="reached",
                first_line_status="reached",
                reserve_lane_status="reached",
            ),
            guided.ReservePackageEmitRow(
                sample_id="p08-dice-stale-reconnect-refresh",
                reading="guard-only non-witness-bearing contrast",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "auditable-authority-witness/p08-dice-stale-reconnect-refresh.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="model-check",
                witness_strengthening_status="guarded_not_reached",
                first_line_status="reached",
                reserve_lane_status="reached",
            ),
            guided.ReservePackageEmitRow(
                sample_id="p05-dice-owner-guarded-chain",
                reading="guard-only pre-default comparison",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "auditable-authority-witness/p05-dice-owner-guarded-chain.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="guarded",
                witness_strengthening_status="guarded_not_reached",
                first_line_status="guarded",
                reserve_lane_status="guarded",
            ),
        ]

        text = guided.render_auditable_authority_witness_emit(
            rows,
            output_dir=output_dir,
        )

        self.assertIn("auditable authority witness reserve package", text)
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py emit-reserve auditable-authority-witness",
            text,
        )
        self.assertIn("package-summary.md", text)
        self.assertIn("witness_kind", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p08-dice-stale-reconnect-refresh", text)
        self.assertIn("p05-dice-owner-guarded-chain", text)
        self.assertIn("final public witness schema", text)

    def test_auditable_authority_witness_emit_manifest_writes_summary_files(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "auditable-authority-witness-index"
        )

        def fake_emitter(sample_id: str, output_path: guided.Path) -> dict[str, object]:
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text("{\"ok\": true}", encoding="utf-8")
            if sample_id == "p07-dice-late-join-visible-history":
                lane_status = "reached"
                witness_status = "reached"
                reserve_detail = "witness+model-check"
                first_line_status = "reached"
            elif sample_id == "p08-dice-stale-reconnect-refresh":
                lane_status = "reached"
                witness_status = "guarded_not_reached"
                reserve_detail = "model-check"
                first_line_status = "reached"
            else:
                lane_status = "guarded_not_reached"
                witness_status = "guarded_not_reached"
                reserve_detail = "guarded"
                first_line_status = "guarded_not_reached"
            return {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "runtime": {"terminal_outcome": "success"},
                "authoritative_room_first_scenario_actual_adoption": {
                    "status": first_line_status,
                },
                "authoritative_room_reserve_strengthening_lane": {
                    "status": lane_status,
                    "witness_strengthening_status": witness_status,
                    "delegated_rng_service_status": "guarded_not_reached",
                    "model_check_second_line_status": "reached"
                    if reserve_detail != "guarded"
                    else "guarded_not_reached",
                },
            }

        manifest = guided.build_auditable_authority_witness_emit_manifest(
            output_dir=output_dir,
            emitter=fake_emitter,
        )

        summary_md = output_dir / "package-summary.md"
        summary_json = output_dir / "package-summary.json"
        self.assertEqual(manifest["package_id"], "auditable-authority-witness")
        self.assertEqual(
            manifest["package_summary_markdown"],
            guided.display_path(summary_md),
        )
        self.assertEqual(
            manifest["package_summary_json"],
            guided.display_path(summary_json),
        )
        self.assertTrue(summary_md.is_file())
        self.assertTrue(summary_json.is_file())
        self.assertIn("auditable authority witness reserve package", summary_md.read_text(encoding="utf-8"))
        payload = guided.json.loads(summary_json.read_text(encoding="utf-8"))
        self.assertEqual(payload["room_profile_claim"], "fairness_claim = auditable_authority_witness")
        self.assertEqual(
            payload["witness_core_fields"],
            ["witness_kind", "action_ref", "draw_slot", "draw_result"],
        )
        self.assertEqual(
            [row["sample_id"] for row in payload["rows"]],
            [
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
                "p05-dice-owner-guarded-chain",
            ],
        )

    def test_delegated_rng_service_emit_text_mentions_output_dir_command_and_samples(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "delegated-rng-service"
        )
        rows = [
            guided.DelegatedRngServiceEmitRow(
                sample_id="p09-dice-delegated-rng-provider-placement",
                reading="delegated provider placement reached",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "delegated-rng-service/p09-dice-delegated-rng-provider-placement.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="delegated-rng+model-check",
                delegated_rng_service_status="reached",
                first_line_status="guarded",
                reserve_lane_status="reached",
            ),
            guided.DelegatedRngServiceEmitRow(
                sample_id="p07-dice-late-join-visible-history",
                reading="guard-only authority-rng baseline contrast",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "delegated-rng-service/p07-dice-late-join-visible-history.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="witness+model-check",
                delegated_rng_service_status="guarded_not_reached",
                first_line_status="reached",
                reserve_lane_status="reached",
            ),
            guided.DelegatedRngServiceEmitRow(
                sample_id="p08-dice-stale-reconnect-refresh",
                reading="guard-only reconnect contrast",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "delegated-rng-service/p08-dice-stale-reconnect-refresh.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                reserve_detail="model-check",
                delegated_rng_service_status="guarded_not_reached",
                first_line_status="reached",
                reserve_lane_status="reached",
            ),
        ]

        text = guided.render_delegated_rng_service_emit(rows, output_dir=output_dir)

        self.assertIn("delegated RNG service reserve package", text)
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py emit-reserve delegated-rng-service",
            text,
        )
        self.assertIn("fairness_source = delegated_rng_service", text)
        self.assertIn("fairness_claim = opaque_authority_trust", text)
        self.assertIn("provider boundary refs", text)
        self.assertIn("optional attachment refs", text)
        self.assertIn("p09-dice-delegated-rng-provider-placement", text)
        self.assertIn("delegated_rng_service_status: reached", text)

    def test_delegated_rng_service_emit_manifest_writes_summary_files(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "delegated-rng-service-index"
        )

        def fake_emitter(sample_id: str, output_path: guided.Path) -> dict[str, object]:
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text("{\"ok\": true}", encoding="utf-8")
            if sample_id == "p09-dice-delegated-rng-provider-placement":
                delegated_status = "reached"
                reserve_detail = "delegated-rng+model-check"
                first_line_status = "guarded_not_reached"
            elif sample_id == "p07-dice-late-join-visible-history":
                delegated_status = "guarded_not_reached"
                reserve_detail = "witness+model-check"
                first_line_status = "reached"
            else:
                delegated_status = "guarded_not_reached"
                reserve_detail = "model-check"
                first_line_status = "reached"
            return {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "runtime": {"terminal_outcome": "success"},
                "authoritative_room_first_scenario_actual_adoption": {
                    "status": first_line_status,
                },
                "authoritative_room_reserve_strengthening_lane": {
                    "status": "reached",
                    "witness_strengthening_status": "reached"
                    if reserve_detail == "witness+model-check"
                    else "guarded_not_reached",
                    "delegated_rng_service_status": delegated_status,
                    "model_check_second_line_status": "reached",
                },
            }

        manifest = guided.build_delegated_rng_service_emit_manifest(
            output_dir=output_dir,
            emitter=fake_emitter,
        )

        summary_md = output_dir / "package-summary.md"
        summary_json = output_dir / "package-summary.json"
        self.assertEqual(manifest["package_id"], "delegated-rng-service")
        self.assertEqual(
            manifest["package_summary_markdown"],
            guided.display_path(summary_md),
        )
        self.assertEqual(
            manifest["package_summary_json"],
            guided.display_path(summary_json),
        )
        self.assertTrue(summary_md.is_file())
        self.assertTrue(summary_json.is_file())
        self.assertIn("delegated RNG service reserve package", summary_md.read_text(encoding="utf-8"))
        payload = guided.json.loads(summary_json.read_text(encoding="utf-8"))
        self.assertEqual(payload["room_profile_provider"], "fairness_source = delegated_rng_service")
        self.assertEqual(payload["room_profile_claim"], "fairness_claim = opaque_authority_trust")
        self.assertEqual(
            payload["provider_boundary_refs"],
            [
                "provider_boundary:placement:delegated_rng_service",
                "provider_boundary:authority_keeps_commit",
                "provider_boundary:provider_returns_draw_not_state_transition",
                "provider_boundary:room_state_mutation_by_authority",
                "provider_boundary:witness_core_unchanged",
            ],
        )
        self.assertEqual(
            [row["sample_id"] for row in payload["rows"]],
            [
                "p09-dice-delegated-rng-provider-placement",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
            ],
        )

    def test_main_emit_reserve_auditable_authority_witness_uses_runtime_renderer(self) -> None:
        fake_text = "auditable authority witness reserve package\n..."

        with mock.patch.object(
            guided,
            "render_auditable_authority_witness_emit_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["emit-reserve", "auditable-authority-witness"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("auditable authority witness reserve package" in call.args[0] for call in write.mock_calls)
        )

    def test_main_emit_reserve_delegated_rng_service_uses_runtime_renderer(self) -> None:
        fake_text = "delegated RNG service reserve package\n..."

        with mock.patch.object(
            guided,
            "render_delegated_rng_service_emit_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["emit-reserve", "delegated-rng-service"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("delegated RNG service reserve package" in call.args[0] for call in write.mock_calls)
        )

    def test_supported_emit_reserve_package_ids_include_model_check_second_line(self) -> None:
        self.assertEqual(
            guided.supported_emit_reserve_package_ids(),
            (
                "auditable-authority-witness",
                "delegated-rng-service",
                "model-check-second-line",
            ),
        )

    def test_model_check_second_line_emit_text_mentions_output_dir_command_and_samples(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "model-check-second-line"
        )
        rows = [
            guided.ModelCheckSecondLineEmitRow(
                sample_id="p06-typed-proof-owner-handoff",
                reading="representative theorem-model-check bridge",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "model-check-second-line/p06-typed-proof-owner-handoff.run.json"
                ),
                static_gate="valid",
                terminal_outcome="success",
                typed_hint_status="guarded",
                theorem_preview_status="reached",
                model_check_preview_status="reached",
                model_check_reopen_status="reached",
            ),
            guided.ModelCheckSecondLineEmitRow(
                sample_id="p11-typed-unauthorized-fingerprint-release",
                reading="authority miss rejection",
                output_path=(
                    "target/guided-sample-tests/reserve-packages/"
                    "model-check-second-line/p11-typed-unauthorized-fingerprint-release.run.json"
                ),
                static_gate="valid",
                terminal_outcome="reject",
                typed_hint_status="reached",
                theorem_preview_status="bridge-only(2)",
                model_check_preview_status="bridge-only(3)",
                model_check_reopen_status="bridge-only(1)",
            ),
        ]

        text = guided.render_model_check_second_line_emit(rows, output_dir=output_dir)

        self.assertIn("model-check second-line reserve package", text)
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py emit-reserve model-check-second-line",
            text,
        )
        self.assertIn("row-local property carrier first", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p11-typed-unauthorized-fingerprint-release", text)
        self.assertIn("final public checker artifact", text)

    def test_model_check_second_line_emit_manifest_writes_summary_files(self) -> None:
        output_dir = (
            guided.REPO_ROOT
            / "target"
            / "guided-sample-tests"
            / "reserve-packages"
            / "model-check-second-line-index"
        )

        def fake_emitter(sample_id: str, output_path: guided.Path) -> dict[str, object]:
            output_path.parent.mkdir(parents=True, exist_ok=True)
            output_path.write_text("{\"ok\": true}", encoding="utf-8")
            if sample_id == "p06-typed-proof-owner-handoff":
                return {
                    "checker_floor": {"static_gate": {"verdict": "valid"}},
                    "runtime": {"terminal_outcome": "success"},
                    "typed_checker_hint_preview": {"status": "guarded_not_reached"},
                    "theorem_result_object_preview": {"status": "reached", "bridge_floor_refs": []},
                    "model_check_public_checker_preview": {"status": "reached", "bridge_floor_refs": []},
                    "model_check_final_public_contract_reopen_threshold": {
                        "status": "reached",
                        "bridge_floor_refs": [],
                    },
                }
            return {
                "checker_floor": {"static_gate": {"verdict": "valid"}},
                "runtime": {"terminal_outcome": "reject"},
                "typed_checker_hint_preview": {"status": "reached"},
                "theorem_result_object_preview": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:theorem:a", "bridge:theorem:b"],
                },
                "model_check_public_checker_preview": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": [
                        "bridge:model:a",
                        "bridge:model:b",
                        "bridge:model:c",
                    ],
                },
                "model_check_final_public_contract_reopen_threshold": {
                    "status": "guarded_not_reached",
                    "bridge_floor_refs": ["bridge:reopen:a"],
                },
            }

        manifest = guided.build_model_check_second_line_emit_manifest(
            output_dir=output_dir,
            emitter=fake_emitter,
        )

        summary_md = output_dir / "package-summary.md"
        summary_json = output_dir / "package-summary.json"
        self.assertEqual(manifest["package_id"], "model-check-second-line")
        self.assertEqual(manifest["package_summary_markdown"], guided.display_path(summary_md))
        self.assertEqual(manifest["package_summary_json"], guided.display_path(summary_json))
        self.assertTrue(summary_md.is_file())
        self.assertTrue(summary_json.is_file())
        self.assertIn("model-check second-line reserve package", summary_md.read_text(encoding="utf-8"))
        payload = guided.json.loads(summary_json.read_text(encoding="utf-8"))
        self.assertEqual(
            payload["current_route"],
            "model_check = row-local property carrier first / public checker artifact later",
        )
        self.assertEqual(
            [row["sample_id"] for row in payload["rows"]],
            [
                "p06-typed-proof-owner-handoff",
                "p10-typed-authorized-fingerprint-declassification",
                "p11-typed-unauthorized-fingerprint-release",
                "p12-typed-classified-fingerprint-publication-block",
                "p15-typed-capture-escape-rejected",
                "p16-typed-remote-call-budget-exceeded",
            ],
        )

    def test_main_emit_reserve_model_check_second_line_uses_runtime_renderer(self) -> None:
        fake_text = "model-check second-line reserve package\n..."

        with mock.patch.object(
            guided,
            "render_model_check_second_line_emit_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["emit-reserve", "model-check-second-line"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("model-check second-line reserve package" in call.args[0] for call in write.mock_calls)
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

    def test_parser_companion_mapping_manifest_tracks_representative_slice(self) -> None:
        manifest = guided.build_parser_companion_mapping_manifest()

        self.assertEqual(
            manifest["mapping_kind"],
            "current_l2_parser_companion_representative_mapping",
        )
        rows = manifest["rows"]
        self.assertEqual(
            [row["sample_id"] for row in rows],
            [
                "p06-typed-proof-owner-handoff",
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
            ],
        )

        first = rows[0]
        self.assertEqual(first["problem_id"], "problem1")
        self.assertEqual(
            first["prototype_path"],
            "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt",
        )
        self.assertEqual(
            first["parser_companion_path"],
            "samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt",
        )
        self.assertEqual(
            first["guided_bundle_command"],
            "python3 scripts/current_l2_guided_samples.py bundle problem1",
        )
        self.assertEqual(
            first["guided_matrix_command"],
            "python3 scripts/current_l2_guided_samples.py matrix problem1",
        )
        self.assertEqual(
            first["inspector_command"],
            "cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt --format pretty",
        )
        self.assertIn(
            "samples/lean/current-l2/p06-typed-proof-owner-handoff/p06-typed-proof-owner-handoff.lean",
            first["lean_artifacts"],
        )
        self.assertIn(
            "specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md",
            first["anchor_refs"],
        )
        self.assertIn(
            "specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md",
            first["anchor_refs"],
        )

        second = rows[1]
        self.assertEqual(second["problem_id"], "problem2")
        self.assertIn(
            "specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md",
            second["anchor_refs"],
        )

    def test_render_parser_companion_mapping_mentions_all_layers(self) -> None:
        text = guided.render_parser_companion_mapping()

        self.assertIn("parser companion representative mapping", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p08-dice-stale-reconnect-refresh", text)
        self.assertIn("original prototype", text)
        self.assertIn("parser companion", text)
        self.assertIn("guided bundle", text)
        self.assertIn("guided matrix", text)
        self.assertIn("Lean artifact", text)
        self.assertIn("anchor refs", text)
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py bundle problem2",
            text,
        )
        self.assertIn(
            "cargo run -q -p mir-ast --example current_l2_inspect_request_head_clause_bundle -- samples/prototype/current-l2-parser-companion/p08-dice-stale-reconnect-refresh.request.txt --format pretty",
            text,
        )
        self.assertIn("exhaustive sample catalog", text)

    def test_main_mapping_command_uses_mapping_renderer(self) -> None:
        fake_text = "parser companion representative mapping\n..."

        with mock.patch.object(guided, "render_parser_companion_mapping", return_value=fake_text):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["mapping"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("parser companion representative mapping" in call.args[0] for call in write.mock_calls)
        )

    def test_problem1_smoke_steps_cover_runtime_bundle_matrix_mapping_and_inspector(self) -> None:
        steps = guided.build_problem_smoke_steps(guided.problem_specs()["problem1"])

        self.assertEqual(
            [step.label for step in steps],
            [
                "runtime:p06-typed-proof-owner-handoff",
                "matrix:problem1",
                "bundle:problem1",
                "inspector:p06-typed-proof-owner-handoff",
                "mapping",
            ],
        )
        self.assertEqual(
            steps[0].command[-3:],
            [
                "samples/prototype/current-l2-typed-proof-model-check/p06-typed-proof-owner-handoff.txt",
                "--format",
                "pretty",
            ],
        )
        self.assertEqual(
            steps[3].command,
            [
                "cargo",
                "run",
                "-q",
                "-p",
                "mir-ast",
                "--example",
                "current_l2_inspect_request_head_clause_bundle",
                "--",
                "samples/prototype/current-l2-parser-companion/p06-typed-proof-owner-handoff.request.txt",
                "--format",
                "json",
            ],
        )

    def test_problem2_smoke_steps_cover_pair_bundle_matrix_inspector_and_mapping(self) -> None:
        steps = guided.build_problem_smoke_steps(guided.problem_specs()["problem2"])

        self.assertEqual(
            [step.label for step in steps],
            [
                "runtime:p07-dice-late-join-visible-history",
                "runtime:p08-dice-stale-reconnect-refresh",
                "matrix:problem2",
                "bundle:problem2",
                "inspector:p07-dice-late-join-visible-history",
                "inspector:p08-dice-stale-reconnect-refresh",
                "mapping",
            ],
        )
        self.assertEqual(
            steps[4].command[-3:],
            [
                "samples/prototype/current-l2-parser-companion/p07-dice-late-join-visible-history.request.txt",
                "--format",
                "json",
            ],
        )

    def test_main_smoke_command_uses_smoke_runner(self) -> None:
        with mock.patch.object(guided, "run_problem_smoke", return_value=0) as runner:
            exit_code = guided.main(["smoke", "problem2"])

        self.assertEqual(exit_code, 0)
        runner.assert_called_once()

    def test_problem_smoke_aggregate_rows_track_both_problems(self) -> None:
        completed = mock.Mock(returncode=0, stdout="", stderr="")

        def fake_runner(command: list[str], cwd: Path, check: bool, capture_output: bool, text: bool):
            self.assertEqual(cwd, guided.REPO_ROOT)
            self.assertFalse(check)
            self.assertTrue(capture_output)
            self.assertTrue(text)
            return completed

        rows = guided.build_problem_smoke_aggregate_rows(
            guided.problem_specs(),
            runner=fake_runner,
        )

        self.assertEqual([row.problem_id for row in rows], ["problem1", "problem2"])
        self.assertEqual(rows[0].status, "passed")
        self.assertEqual(rows[0].step_count, 5)
        self.assertEqual(rows[0].successful_steps, 5)
        self.assertIsNone(rows[0].failed_step)
        self.assertEqual(
            rows[0].smoke_command,
            "python3 scripts/current_l2_guided_samples.py smoke problem1",
        )
        self.assertEqual(
            rows[1].sample_bundle_doc,
            "samples/problem-bundles/problem2-order-handoff-shared-space.md",
        )
        self.assertEqual(rows[1].primary_samples, ["p07-dice-late-join-visible-history", "p08-dice-stale-reconnect-refresh"])

    def test_problem_smoke_aggregate_rows_capture_failure_diagnostics(self) -> None:
        def fake_runner(command: list[str], cwd: Path, check: bool, capture_output: bool, text: bool):
            self.assertEqual(cwd, guided.REPO_ROOT)
            self.assertFalse(check)
            self.assertTrue(capture_output)
            self.assertTrue(text)
            if command[:4] == [
                "python3",
                "scripts/current_l2_guided_samples.py",
                "bundle",
                "problem1",
            ]:
                return subprocess.CompletedProcess(
                    args=command,
                    returncode=7,
                    stdout="bundle note: missing witness artifact\nsecond line",
                    stderr="error: simulated bundle drift\ntry refresh",
                )
            return subprocess.CompletedProcess(
                args=command,
                returncode=0,
                stdout="",
                stderr="",
            )

        rows = guided.build_problem_smoke_aggregate_rows(
            guided.problem_specs(),
            runner=fake_runner,
        )

        self.assertEqual(rows[0].status, "failed")
        self.assertEqual(rows[0].successful_steps, 2)
        self.assertEqual(rows[0].failed_step, "bundle:problem1")
        self.assertEqual(
            rows[0].failed_command,
            "python3 scripts/current_l2_guided_samples.py bundle problem1 --format json",
        )
        self.assertEqual(rows[0].failed_return_code, 7)
        self.assertIn("stderr: error: simulated bundle drift try refresh", rows[0].failed_output_excerpt)
        self.assertIn(
            "stdout: bundle note: missing witness artifact second line",
            rows[0].failed_output_excerpt,
        )
        self.assertEqual(rows[1].status, "passed")
        self.assertIsNone(rows[1].failed_output_excerpt)

    def test_render_problem_smoke_aggregate_mentions_failure_diagnostics(self) -> None:
        text = guided.render_problem_smoke_aggregate(
            [
                guided.ProblemSmokeAggregateRow(
                    problem_id="problem1",
                    title="Problem 1 theorem-first pilot bundle",
                    status="failed",
                    step_count=5,
                    successful_steps=2,
                    failed_step="bundle:problem1",
                    smoke_command="python3 scripts/current_l2_guided_samples.py smoke problem1",
                    sample_bundle_doc="samples/problem-bundles/problem1-typed-theorem-model-check.md",
                    primary_samples=["p06-typed-proof-owner-handoff"],
                    step_labels=[
                        "runtime:p06-typed-proof-owner-handoff",
                        "matrix:problem1",
                        "bundle:problem1",
                    ],
                    failed_command="python3 scripts/current_l2_guided_samples.py bundle problem1 --format json",
                    failed_return_code=7,
                    failed_output_excerpt="stderr: error: simulated bundle drift try refresh",
                )
            ]
        )

        self.assertIn("failed step: bundle:problem1", text)
        self.assertIn(
            "failed command: python3 scripts/current_l2_guided_samples.py bundle problem1 --format json",
            text,
        )
        self.assertIn("failed return code: 7", text)
        self.assertIn("failure excerpt: stderr: error: simulated bundle drift try refresh", text)

    def test_main_smoke_all_command_returns_nonzero_when_any_problem_fails(self) -> None:
        fake_text = "representative problem bundle aggregate smoke summary\n..."

        with mock.patch.object(
            guided,
            "run_problem_smoke_aggregate",
            return_value=(1, fake_text),
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["smoke-all"])

        self.assertEqual(exit_code, 1)
        self.assertTrue(
            any(
                "representative problem bundle aggregate smoke summary" in call.args[0]
                for call in write.mock_calls
            )
        )

    def test_problem1_quickstart_text_mentions_four_steps_and_expected_results(self) -> None:
        spec = guided.problem_specs()["problem1"]

        text = guided.render_problem_quickstart(spec)

        self.assertIn("Problem 1", text)
        self.assertIn("quickstart", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py smoke problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem1", text)
        self.assertIn("current_l2_inspect_request_head_clause_bundle", text)
        self.assertIn("見るべき結果", text)

    def test_problem2_quickstart_json_contains_expected_steps(self) -> None:
        spec = guided.problem_specs()["problem2"]

        rendered = guided.render_problem_quickstart_from_runtime(spec, output_format="json")
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["problem_id"], "problem2")
        self.assertEqual(len(payload["steps"]), 4)
        self.assertEqual(payload["steps"][0]["command"], "python3 scripts/current_l2_guided_samples.py smoke problem2")
        self.assertIn("representative pair", payload["steps"][1]["expected_results"][0])
        self.assertIn("current_l2_inspect_request_head_clause_bundle", payload["steps"][3]["command"])

    def test_main_quickstart_command_uses_quickstart_renderer(self) -> None:
        fake_text = "Problem 1 quickstart\n..."

        with mock.patch.object(
            guided,
            "render_problem_quickstart_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["quickstart", "problem1"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("Problem 1 quickstart" in call.args[0] for call in write.mock_calls))

    def test_problem_quickstart_parity_rows_report_synced_docs(self) -> None:
        rows = guided.build_problem_quickstart_parity_rows(guided.problem_specs())

        self.assertEqual([row["problem_id"] for row in rows], ["problem1", "problem2"])
        self.assertTrue(all(row["status"] == "synced" for row in rows))
        self.assertTrue(all(not row["missing_titles"] for row in rows))
        self.assertTrue(all(not row["missing_commands"] for row in rows))

    def test_problem_quickstart_parity_rows_report_missing_items(self) -> None:
        def fake_loader(path: str) -> str:
            if path.endswith("problem1-typed-theorem-model-check.md"):
                return "python3 scripts/current_l2_guided_samples.py smoke problem1\n"
            return "ok"

        rows = guided.build_problem_quickstart_parity_rows(
            guided.problem_specs(),
            doc_loader=fake_loader,
        )

        self.assertEqual(rows[0]["status"], "mismatch")
        self.assertIn("`matrix problem1` で representative と補助 sample の役割差を見る", rows[0]["missing_titles"])
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem1", rows[0]["missing_commands"])
        self.assertEqual(rows[1]["status"], "mismatch")

    def test_main_quickstart_parity_command_uses_renderer(self) -> None:
        fake_text = "representative problem quickstart parity\n..."

        with mock.patch.object(
            guided,
            "render_problem_quickstart_parity_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["quickstart-parity"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("representative problem quickstart parity" in call.args[0] for call in write.mock_calls))

    def test_problem_reopen_map_text_mentions_problem_specific_reopen_points(self) -> None:
        text = guided.render_problem_reopen_map(guided.problem_specs())

        self.assertIn("representative problem mixed-gate reopen map", text)
        self.assertIn("check-source-sample", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-theorem problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py emit-scenario problem2", text)
        self.assertIn("stronger typed-surface actual adoption", text)
        self.assertIn("final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract", text)
        self.assertIn("installed-binary / packaging / FFI / engine adapter / host integration target", text)

    def test_problem_reopen_map_json_contains_problem_rows_and_global_user_spec_residuals(self) -> None:
        rendered = guided.render_problem_reopen_map_from_runtime(
            guided.problem_specs(),
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["map_kind"], "current_l2_representative_problem_mixed_gate_reopen_map")
        self.assertEqual([row["problem_id"] for row in payload["problem_rows"]], ["problem1", "problem2"])
        self.assertIn("quickstart problem1", payload["problem_rows"][0]["entry_commands"][0])
        self.assertIn("check-source-sample", payload["problem_rows"][0]["entry_commands"][1])
        self.assertIn("emit-theorem problem1", payload["problem_rows"][0]["entry_commands"][2])
        self.assertIn("emit-scenario problem2", payload["problem_rows"][1]["entry_commands"][1])
        self.assertIn(
            "upper-layer application target beyond authoritative-room first scenario",
            payload["true_user_spec_residuals"],
        )

    def test_main_reopen_map_command_uses_renderer(self) -> None:
        fake_text = "representative problem mixed-gate reopen map\n..."

        with mock.patch.object(
            guided,
            "render_problem_reopen_map_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["reopen-map"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("representative problem mixed-gate reopen map" in call.args[0] for call in write.mock_calls))

    def test_problem1_reopen_map_json_contains_closed_split_packages(self) -> None:
        rendered = guided.render_problem_reopen_map_from_runtime(
            {"problem1": guided.problem_specs()["problem1"]},
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual([row["problem_id"] for row in payload["problem_rows"]], ["problem1"])
        self.assertEqual(len(payload["problem_rows"][0]["closed_split_packages"]), 3)
        self.assertEqual(
            payload["problem_rows"][0]["closed_split_packages"][0],
            "typed source principal split",
        )
        self.assertNotIn("split_packages", payload["problem_rows"][0])

    def test_problem1_reopen_map_text_mentions_closed_split_packages(self) -> None:
        text = guided.render_problem_reopen_map({"problem1": guided.problem_specs()["problem1"]})

        self.assertIn("split package closeout:", text)
        self.assertIn("typed source principal split", text)
        self.assertIn("theorem public-contract split", text)
        self.assertIn("model-check public-contract split", text)

    def test_main_reopen_map_command_accepts_problem_id(self) -> None:
        fake_text = "problem1 reopen map\n..."

        with mock.patch.object(
            guided,
            "render_problem_reopen_map_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["reopen-map", "problem1"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("problem1 reopen map" in call.args[0] for call in write.mock_calls))

    def test_problem2_reopen_map_json_contains_closed_split_packages(self) -> None:
        rendered = guided.render_problem_reopen_map_from_runtime(
            {"problem2": guided.problem_specs()["problem2"]},
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual([row["problem_id"] for row in payload["problem_rows"]], ["problem2"])
        self.assertEqual(len(payload["problem_rows"][0]["closed_split_packages"]), 2)
        self.assertEqual(
            payload["problem_rows"][0]["closed_split_packages"][0],
            "source wording / emitted schema split",
        )
        self.assertNotIn("split_packages", payload["problem_rows"][0])

    def test_problem2_reopen_map_text_mentions_closed_split_packages(self) -> None:
        text = guided.render_problem_reopen_map({"problem2": guided.problem_specs()["problem2"]})

        self.assertIn("split package closeout:", text)
        self.assertIn("source wording / emitted schema split", text)
        self.assertIn("witness-provider public-shape split", text)

    def test_remaining_residual_lane_summary_text_mentions_mixed_gate_lanes_and_user_spec_residuals(self) -> None:
        text = guided.render_remaining_residual_lane_summary(guided.problem_specs())

        self.assertIn("current-l2 remaining residual lane summary", text)
        self.assertIn("remaining mixed-gate lanes:", text)
        self.assertIn("problem1-final-public-seams", text)
        self.assertIn("problem2-final-public-seams", text)
        self.assertIn("parser-side-residual", text)
        self.assertIn("syntax-modality-final-marker", text)
        self.assertIn("true user-spec residuals:", text)
        self.assertIn(
            "installed-binary / packaging / FFI / engine adapter / host integration target",
            text,
        )

    def test_remaining_residual_lane_summary_json_separates_mixed_gate_lanes_and_user_spec_residuals(self) -> None:
        rendered = guided.render_remaining_residual_lane_summary_from_runtime(
            guided.problem_specs(),
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(
            payload["manifest_kind"],
            "current_l2_remaining_residual_lane_summary",
        )
        self.assertEqual(
            [lane["lane_id"] for lane in payload["mixed_gate_lanes"]],
            [
                "problem1-final-public-seams",
                "problem2-final-public-seams",
                "parser-side-residual",
                "syntax-modality-final-marker",
            ],
        )
        self.assertIn(
            "upper-layer application target beyond authoritative-room first scenario",
            payload["true_user_spec_residuals"],
        )
        self.assertEqual(
            payload["recommended_order"],
            [
                "problem1-final-public-seams",
                "problem2-final-public-seams",
                "parser-side-residual",
                "syntax-modality-final-marker",
            ],
        )

    def test_main_residuals_command_uses_renderer(self) -> None:
        fake_text = "current-l2 remaining residual lane summary\n..."

        with mock.patch.object(
            guided,
            "render_remaining_residual_lane_summary_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["residuals"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("current-l2 remaining residual lane summary" in call.args[0] for call in write.mock_calls)
        )

    def test_once_through_closeout_summary_text_mentions_first_lines_and_residual_boundaries(
        self,
    ) -> None:
        text = guided.render_once_through_closeout_summary_from_runtime(
            guided.problem_specs(),
            output_format="pretty",
        )

        self.assertIn("current-l2 once-through closeout summary", text)
        self.assertIn("current first lines:", text)
        self.assertIn("problem1", text)
        self.assertIn("problem2", text)
        self.assertIn("syntax-modality", text)
        self.assertIn("emit-theorem problem1", text)
        self.assertIn("emit-scenario problem2", text)
        self.assertIn("mixed-gate lanes:", text)
        self.assertIn("problem1-final-public-seams", text)
        self.assertIn("problem2-final-public-seams", text)
        self.assertIn("parser-side-residual", text)
        self.assertIn("hold-line", text)
        self.assertIn("(none; closeout numbered queue closed)", text)
        self.assertIn("true user-spec residuals:", text)
        self.assertIn("final public verifier contract", text)

    def test_once_through_closeout_summary_json_contains_next_packages(self) -> None:
        rendered = guided.render_once_through_closeout_summary_from_runtime(
            guided.problem_specs(),
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(
            payload["manifest_kind"],
            "current_l2_once_through_closeout_summary",
        )
        self.assertEqual(
            [row["line_id"] for row in payload["current_first_lines"]],
            ["problem1", "problem2", "syntax-modality"],
        )
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
            payload["executable_entry_commands"],
        )
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
            payload["executable_entry_commands"],
        )
        self.assertEqual(payload["next_self_driven_packages"], [])
        self.assertEqual(
            payload["true_user_spec_hold_line_command"],
            "python3 scripts/current_l2_guided_samples.py hold-line",
        )
        self.assertIn(
            "installed-binary / packaging / FFI / engine adapter / host integration target",
            payload["true_user_spec_residuals"],
        )

    def test_main_closeout_command_uses_renderer(self) -> None:
        fake_text = "current-l2 once-through closeout summary\n..."

        with mock.patch.object(
            guided,
            "render_once_through_closeout_summary_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["closeout"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any("current-l2 once-through closeout summary" in call.args[0] for call in write.mock_calls)
        )

    def test_reserve_integration_summary_text_mentions_four_reserve_lanes(self) -> None:
        text = guided.render_reserve_integration_summary_from_runtime(
            guided.problem_specs(),
            output_format="pretty",
        )

        self.assertIn("current-l2 reserve integration entrypoint summary", text)
        self.assertIn("theorem-first-external-pilot", text)
        self.assertIn("auditable-authority-witness", text)
        self.assertIn("delegated-rng-service", text)
        self.assertIn("model-check-second-line", text)
        self.assertIn("emit-reserve auditable-authority-witness", text)
        self.assertIn("emit-reserve delegated-rng-service", text)
        self.assertIn("emit-reserve model-check-second-line", text)
        self.assertIn("emit-theorem problem1", text)
        self.assertIn("closeout", text)
        self.assertIn("hold-line", text)
        self.assertIn(
            "(none; reserve reopen order is available, but closeout queue is closed)",
            text,
        )

    def test_reserve_integration_summary_json_contains_package_ids_and_next_queue(self) -> None:
        rendered = guided.render_reserve_integration_summary_from_runtime(
            guided.problem_specs(),
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(
            payload["manifest_kind"],
            "current_l2_reserve_integration_entrypoint_summary",
        )
        self.assertEqual(
            [row["package_id"] for row in payload["reserve_packages"]],
            [
                "theorem-first-external-pilot",
                "auditable-authority-witness",
                "delegated-rng-service",
                "model-check-second-line",
            ],
        )
        self.assertEqual(payload["next_queue"], [])
        self.assertEqual(
            payload["true_user_spec_hold_line_command"],
            "python3 scripts/current_l2_guided_samples.py hold-line",
        )
        self.assertIn(
            "packaging / FFI / engine adapter / exhaustive shared-space catalog / upper-layer app target",
            payload["stop_line"],
        )

    def test_main_reserve_command_uses_renderer(self) -> None:
        fake_text = "current-l2 reserve integration entrypoint summary\n..."

        with mock.patch.object(
            guided,
            "render_reserve_integration_summary_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["reserve"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any(
                "current-l2 reserve integration entrypoint summary" in call.args[0]
                for call in write.mock_calls
            )
        )

    def test_true_user_spec_hold_line_text_mentions_defaults_and_reopen_triggers(self) -> None:
        text = guided.render_true_user_spec_hold_line_summary_from_runtime(
            guided.problem_specs(),
            output_format="pretty",
        )

        self.assertIn("current-l2 true user-spec hold line summary", text)
        self.assertIn("repo-local near-end success defaults:", text)
        self.assertIn("true user-spec residuals:", text)
        self.assertIn("reopen triggers:", text)
        self.assertIn(
            "repo-local runnable CLI + tests + emitted artifacts + reproducible compare floor",
            text,
        )
        self.assertIn(
            "user が distribution / embedding / host target を指定したとき",
            text,
        )

    def test_true_user_spec_hold_line_json_contains_items_and_hold_command(self) -> None:
        rendered = guided.render_true_user_spec_hold_line_summary_from_runtime(
            guided.problem_specs(),
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(
            payload["manifest_kind"],
            "current_l2_true_user_spec_hold_line_summary",
        )
        self.assertEqual(
            payload["hold_line_command"],
            "python3 scripts/current_l2_guided_samples.py hold-line",
        )
        self.assertEqual(
            [row["residual_id"] for row in payload["residual_rows"]],
            [
                "shared-space-catalog",
                "packaging-and-host-integration",
                "upper-layer-application-target",
            ],
        )
        self.assertIn(
            "authoritative-room minimal working subset first",
            payload["repo_local_near_end_success_defaults"],
        )

    def test_main_hold_line_command_uses_renderer(self) -> None:
        fake_text = "current-l2 true user-spec hold line summary\n..."

        with mock.patch.object(
            guided,
            "render_true_user_spec_hold_line_summary_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["hold-line"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any(
                "current-l2 true user-spec hold line summary" in call.args[0]
                for call in write.mock_calls
            )
        )

    def test_problem1_final_public_seam_lane_text_mentions_component_order_and_stop_line(self) -> None:
        text = guided.render_residual_lane_from_runtime(
            "problem1-final-public-seams",
            output_format="pretty",
        )

        self.assertIn("problem1-final-public-seams", text)
        self.assertIn("component order:", text)
        self.assertIn("typed source principal split", text)
        self.assertIn("theorem public-contract split", text)
        self.assertIn("model-check public-contract split", text)
        self.assertIn("final public verifier contract", text)

    def test_problem1_final_public_seam_lane_json_contains_component_order(self) -> None:
        rendered = guided.render_residual_lane_from_runtime(
            "problem1-final-public-seams",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["lane_id"], "problem1-final-public-seams")
        self.assertEqual(
            payload["component_order"],
            [
                "typed source principal split",
                "theorem public-contract split",
                "model-check public-contract split",
            ],
        )
        self.assertTrue(any("emit-theorem problem1" in command for command in payload["entry_commands"]))
        self.assertTrue(any("check-source-sample" in command for command in payload["entry_commands"]))

    def test_problem2_final_public_seam_lane_text_mentions_component_order_and_stop_line(self) -> None:
        text = guided.render_residual_lane_from_runtime(
            "problem2-final-public-seams",
            output_format="pretty",
        )

        self.assertIn("problem2-final-public-seams", text)
        self.assertIn("component order:", text)
        self.assertIn("source wording / emitted schema split", text)
        self.assertIn("witness-provider public-shape split", text)
        self.assertIn("final public witness/provider/artifact contract", text)

    def test_problem2_final_public_seam_lane_json_contains_component_order(self) -> None:
        rendered = guided.render_residual_lane_from_runtime(
            "problem2-final-public-seams",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["lane_id"], "problem2-final-public-seams")
        self.assertEqual(
            payload["component_order"],
            [
                "source wording / emitted schema split",
                "witness-provider public-shape split",
            ],
        )
        self.assertTrue(any("emit-scenario problem2" in command for command in payload["entry_commands"]))
        self.assertTrue(any("residuals" in command for command in payload["entry_commands"]))

    def test_parser_side_residual_lane_text_mentions_component_order_and_stop_line(self) -> None:
        text = guided.render_residual_lane_from_runtime(
            "parser-side-residual",
            output_format="pretty",
        )

        self.assertIn("parser-side-residual", text)
        self.assertIn("component order:", text)
        self.assertIn("parser companion surface bundle", text)
        self.assertIn("request-head / clause-bundle inspector", text)
        self.assertIn("final public parser / checker / runtime API", text)

    def test_parser_side_residual_lane_json_contains_component_order(self) -> None:
        rendered = guided.render_residual_lane_from_runtime(
            "parser-side-residual",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["lane_id"], "parser-side-residual")
        self.assertEqual(
            payload["component_order"],
            [
                "parser companion surface bundle",
                "bundle-to-helper bridge",
                "request-head / clause-bundle inspector",
                "representative mapping matrix",
            ],
        )
        self.assertTrue(any("mapping" in command for command in payload["entry_commands"]))
        self.assertTrue(
            any("current_l2_inspect_request_head_clause_bundle" in command for command in payload["entry_commands"])
        )

    def test_syntax_modality_final_marker_lane_text_mentions_recommendation_and_retained_families(
        self,
    ) -> None:
        text = guided.render_residual_lane_from_runtime(
            "syntax-modality-final-marker",
            output_format="pretty",
        )

        self.assertIn("syntax-modality-final-marker", text)
        self.assertIn("current recommendation:", text)
        self.assertIn("lambda_circle_box partial basis keep", text)
        self.assertIn("guarded / MDTT / MTT / Fitch-style stronger family keep", text)
        self.assertIn("problem-local seam separation", text)
        self.assertIn("true user-spec residual separation", text)
        self.assertIn("final parser grammar", text)

    def test_syntax_modality_final_marker_lane_json_contains_recommendation_and_separation(
        self,
    ) -> None:
        rendered = guided.render_residual_lane_from_runtime(
            "syntax-modality-final-marker",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["lane_id"], "syntax-modality-final-marker")
        self.assertEqual(
            payload["current_recommendation"],
            "partial basis keep + stronger family keep + readable source marker keep",
        )
        self.assertEqual(
            payload["retained_families"],
            [
                "lambda_circle_box partial basis keep",
                "guarded / MDTT / MTT / Fitch-style stronger family keep",
            ],
        )
        self.assertEqual(
            payload["separation_boundary"],
            [
                "problem-local seam separation",
                "true user-spec residual separation",
            ],
        )

    def test_main_lane_command_uses_renderer(self) -> None:
        fake_text = "problem1-final-public-seams\n..."

        with mock.patch.object(
            guided,
            "render_residual_lane_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["lane", "problem1-final-public-seams"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("problem1-final-public-seams" in call.args[0] for call in write.mock_calls))

    def test_problem1_typed_split_text_mentions_supporting_samples_and_kept_separate(self) -> None:
        text = guided.render_problem_split_package_from_runtime(
            "problem1",
            "typed-source-principal",
            output_format="pretty",
        )

        self.assertIn("typed source principal split", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("p10-typed-authorized-fingerprint-declassification", text)
        self.assertIn("p15-typed-capture-escape-rejected", text)
        self.assertIn("kept separate:", text)
        self.assertIn("theorem public-contract split", text)
        self.assertIn("model-check public-contract split", text)
        self.assertIn("final typed source principal", text)

    def test_problem1_typed_split_json_contains_expected_manifest(self) -> None:
        rendered = guided.render_problem_split_package_from_runtime(
            "problem1",
            "typed-source-principal",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["package_id"], "typed-source-principal")
        self.assertEqual(payload["package_name"], "typed source principal split")
        self.assertEqual(payload["problem_id"], "problem1")
        self.assertIn("p06-typed-proof-owner-handoff", payload["representative_samples"])
        self.assertIn("p16-typed-remote-call-budget-exceeded", payload["supporting_samples"])
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py matrix problem1",
            payload["commands"][0],
        )
        self.assertIn("theorem public-contract split", payload["kept_separate"])

    def test_problem1_theorem_split_text_mentions_bundle_command_and_kept_separate(self) -> None:
        text = guided.render_problem_split_package_from_runtime(
            "problem1",
            "theorem-public-contract",
            output_format="pretty",
        )

        self.assertIn("theorem public-contract split", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem1", text)
        self.assertIn("typed source principal split", text)
        self.assertIn("model-check public-contract split", text)
        self.assertIn("final public theorem contract", text)

    def test_problem1_theorem_split_json_contains_expected_manifest(self) -> None:
        rendered = guided.render_problem_split_package_from_runtime(
            "problem1",
            "theorem-public-contract",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["package_id"], "theorem-public-contract")
        self.assertEqual(payload["package_name"], "theorem public-contract split")
        self.assertEqual(payload["problem_id"], "problem1")
        self.assertEqual(payload["representative_samples"], ["p06-typed-proof-owner-handoff"])
        self.assertEqual(payload["supporting_samples"], ["p06-typed-proof-owner-handoff"])
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py bundle problem1",
            payload["commands"][0],
        )
        self.assertIn("typed source principal split", payload["kept_separate"])
        self.assertIn("model-check public-contract split", payload["kept_separate"])

    def test_problem1_model_check_split_text_mentions_matrix_bundle_and_kept_separate(self) -> None:
        text = guided.render_problem_split_package_from_runtime(
            "problem1",
            "model-check-public-contract",
            output_format="pretty",
        )

        self.assertIn("model-check public-contract split", text)
        self.assertIn("p06-typed-proof-owner-handoff", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem1", text)
        self.assertIn("typed source principal split", text)
        self.assertIn("theorem public-contract split", text)
        self.assertIn("final public checker artifact", text)

    def test_problem1_model_check_split_json_contains_expected_manifest(self) -> None:
        rendered = guided.render_problem_split_package_from_runtime(
            "problem1",
            "model-check-public-contract",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["package_id"], "model-check-public-contract")
        self.assertEqual(payload["package_name"], "model-check public-contract split")
        self.assertEqual(payload["problem_id"], "problem1")
        self.assertIn("p06-typed-proof-owner-handoff", payload["representative_samples"])
        self.assertIn("p16-typed-remote-call-budget-exceeded", payload["supporting_samples"])
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py matrix problem1",
            payload["commands"][0],
        )
        self.assertIn("typed source principal split", payload["kept_separate"])
        self.assertIn("theorem public-contract split", payload["kept_separate"])

    def test_problem2_source_wording_split_text_mentions_bundle_and_kept_separate(self) -> None:
        text = guided.render_problem_split_package_from_runtime(
            "problem2",
            "source-wording-emitted-schema",
            output_format="pretty",
        )

        self.assertIn("source wording / emitted schema split", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p08-dice-stale-reconnect-refresh", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem2", text)
        self.assertIn("witness-provider public-shape split", text)
        self.assertIn("final source-surface handoff wording", text)

    def test_problem2_source_wording_split_json_contains_expected_manifest(self) -> None:
        rendered = guided.render_problem_split_package_from_runtime(
            "problem2",
            "source-wording-emitted-schema",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["package_id"], "source-wording-emitted-schema")
        self.assertEqual(payload["package_name"], "source wording / emitted schema split")
        self.assertEqual(payload["problem_id"], "problem2")
        self.assertIn("p07-dice-late-join-visible-history", payload["representative_samples"])
        self.assertIn("p14-dice-late-join-handoff-before-publication", payload["supporting_samples"])
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py bundle problem2",
            payload["commands"][0],
        )
        self.assertIn("witness-provider public-shape split", payload["kept_separate"])

    def test_problem2_witness_provider_split_text_mentions_matrix_bundle_and_kept_separate(self) -> None:
        text = guided.render_problem_split_package_from_runtime(
            "problem2",
            "witness-provider-public-shape",
            output_format="pretty",
        )

        self.assertIn("witness-provider public-shape split", text)
        self.assertIn("p07-dice-late-join-visible-history", text)
        self.assertIn("p09-dice-delegated-rng-provider-placement", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem2", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem2", text)
        self.assertIn("source wording / emitted schema split", text)
        self.assertIn("final public witness/provider/artifact contract", text)

    def test_problem2_witness_provider_split_json_contains_expected_manifest(self) -> None:
        rendered = guided.render_problem_split_package_from_runtime(
            "problem2",
            "witness-provider-public-shape",
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual(payload["package_id"], "witness-provider-public-shape")
        self.assertEqual(payload["package_name"], "witness-provider public-shape split")
        self.assertEqual(payload["problem_id"], "problem2")
        self.assertIn("p07-dice-late-join-visible-history", payload["representative_samples"])
        self.assertIn("p09-dice-delegated-rng-provider-placement", payload["supporting_samples"])
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py matrix problem2",
            payload["commands"][0],
        )
        self.assertIn("source wording / emitted schema split", payload["kept_separate"])

    def test_main_split_command_uses_split_renderer(self) -> None:
        fake_text = "typed source principal split\n..."

        with mock.patch.object(
            guided,
            "render_problem_split_package_from_runtime",
            return_value=fake_text,
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["split", "problem1", "typed-source-principal"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(any("typed source principal split" in call.args[0] for call in write.mock_calls))

    def test_main_smoke_all_command_uses_aggregate_renderer(self) -> None:
        fake_text = "representative problem bundle aggregate smoke summary\n..."

        with mock.patch.object(
            guided,
            "run_problem_smoke_aggregate",
            return_value=(0, fake_text),
        ):
            with mock.patch("sys.stdout.write") as write:
                exit_code = guided.main(["smoke-all"])

        self.assertEqual(exit_code, 0)
        self.assertTrue(
            any(
                "representative problem bundle aggregate smoke summary" in call.args[0]
                for call in write.mock_calls
            )
        )


if __name__ == "__main__":
    unittest.main()
