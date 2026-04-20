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
        self.assertIn("python3 scripts/current_l2_guided_samples.py matrix problem1", text)
        self.assertIn("python3 scripts/current_l2_guided_samples.py bundle problem2", text)
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
        self.assertIn("bundle problem2", payload["problem_rows"][1]["entry_commands"][2])
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

    def test_problem1_reopen_map_json_contains_three_split_packages(self) -> None:
        rendered = guided.render_problem_reopen_map_from_runtime(
            {"problem1": guided.problem_specs()["problem1"]},
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual([row["problem_id"] for row in payload["problem_rows"]], ["problem1"])
        self.assertEqual(len(payload["problem_rows"][0]["split_packages"]), 3)
        self.assertEqual(
            payload["problem_rows"][0]["split_packages"][0]["package_name"],
            "typed source principal split",
        )
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py matrix problem1",
            payload["problem_rows"][0]["split_packages"][0]["commands"][0],
        )

    def test_problem1_reopen_map_text_mentions_split_packages(self) -> None:
        text = guided.render_problem_reopen_map({"problem1": guided.problem_specs()["problem1"]})

        self.assertIn("next split packages:", text)
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

    def test_problem2_reopen_map_json_contains_two_split_packages(self) -> None:
        rendered = guided.render_problem_reopen_map_from_runtime(
            {"problem2": guided.problem_specs()["problem2"]},
            output_format="json",
        )
        payload = guided.json.loads(rendered)

        self.assertEqual([row["problem_id"] for row in payload["problem_rows"]], ["problem2"])
        self.assertEqual(len(payload["problem_rows"][0]["split_packages"]), 2)
        self.assertEqual(
            payload["problem_rows"][0]["split_packages"][0]["package_name"],
            "source wording / emitted schema split",
        )
        self.assertIn(
            "python3 scripts/current_l2_guided_samples.py bundle problem2",
            payload["problem_rows"][0]["split_packages"][0]["commands"][0],
        )

    def test_problem2_reopen_map_text_mentions_split_packages(self) -> None:
        text = guided.render_problem_reopen_map({"problem2": guided.problem_specs()["problem2"]})

        self.assertIn("next split packages:", text)
        self.assertIn("source wording / emitted schema split", text)
        self.assertIn("witness-provider public-shape split", text)

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
