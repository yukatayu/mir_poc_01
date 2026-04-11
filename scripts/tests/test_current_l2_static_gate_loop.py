import argparse
import json
import tempfile
import sys
import unittest
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_detached_loop as loop  # noqa: E402


class StaticGateLoopTests(unittest.TestCase):
    def test_static_gate_artifact_path_uses_run_label_and_stem(self) -> None:
        path = loop.static_gate_artifact_path(
            REPO_ROOT / "target" / "current-l2-detached",
            "static-smoke",
            REPO_ROOT / "crates/mir-ast/tests/fixtures/current-l2/e4-malformed-lineage.json",
        )
        self.assertEqual(
            path,
            REPO_ROOT
            / "target"
            / "current-l2-detached"
            / "static-gates"
            / "static-smoke"
            / "e4-malformed-lineage.static-gate.json",
        )

    def test_formal_hook_artifact_path_uses_run_label_and_stem(self) -> None:
        path = loop.formal_hook_artifact_path(
            REPO_ROOT / "target" / "current-l2-detached",
            "formal-smoke",
            REPO_ROOT / "crates/mir-ast/tests/fixtures/current-l2/e5-underdeclared-lineage.json",
        )
        self.assertEqual(
            path,
            REPO_ROOT
            / "target"
            / "current-l2-detached"
            / "formal-hooks"
            / "formal-smoke"
            / "e5-underdeclared-lineage.formal-hook.json",
        )

    def test_smoke_static_gate_emits_and_compares_reference(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")
            reference_path = fixture_dir / "right.json"
            reference_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            compared: list[tuple[Path, Path]] = []

            original_emit = loop.emit_static_gate
            original_compare = loop.compare_static_gates

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                return 0

            def fake_compare(left: Path, right: Path) -> int:
                compared.append((left, right))
                return 1

            loop.emit_static_gate = fake_emit
            loop.compare_static_gates = fake_compare
            try:
                exit_code = loop.command_smoke_static_gate(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        reference_fixture=str(reference_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="left-run",
                        reference_label="right-run",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.compare_static_gates = original_compare

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted), 2)
        self.assertEqual(
            emitted[0][1],
            temp_root
            / "artifacts"
            / "static-gates"
            / "left-run"
            / "left.static-gate.json",
        )
        self.assertEqual(
            emitted[1][1],
            temp_root
            / "artifacts"
            / "static-gates"
            / "right-run"
            / "right.static-gate.json",
        )
        self.assertEqual(
            compared,
            [
                (
                    temp_root
                    / "artifacts"
                    / "static-gates"
                    / "left-run"
                    / "left.static-gate.json",
                    temp_root
                    / "artifacts"
                    / "static-gates"
                    / "right-run"
                    / "right.static-gate.json",
                )
            ],
        )

    def test_suggest_checked_reasons_emits_artifact_then_delegates_to_assist(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            assisted: list[list[str]] = []

            original_emit = loop.emit_static_gate
            original_assist = loop.checked_reasons_assist.main

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_assist(argv: list[str] | None = None) -> int:
                assisted.append(list(argv or []))
                return 0

            loop.emit_static_gate = fake_emit
            loop.checked_reasons_assist.main = fake_assist
            try:
                exit_code = loop.command_suggest_checked_reasons(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="assist-run",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.checked_reasons_assist.main = original_assist

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted), 1)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "assist-run"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted[0], (fixture_path, expected_artifact, True))
        self.assertEqual(assisted, [[str(fixture_path), str(expected_artifact)]])

    def test_smoke_formal_hook_static_emits_static_gate_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            delegated: list[tuple[str, Path, Path, bool]] = []

            original_emit = loop.emit_static_gate
            original_formal = loop.emit_formal_hook

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_formal(
                source_kind: str,
                source_artifact: Path,
                output: Path,
                overwrite: bool,
            ) -> int:
                delegated.append((source_kind, source_artifact, output, overwrite))
                return 0

            loop.emit_static_gate = fake_emit
            loop.emit_formal_hook = fake_formal
            try:
                exit_code = loop.command_smoke_formal_hook_static(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="formal-static",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.emit_formal_hook = original_formal

        self.assertEqual(exit_code, 0)
        static_gate_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "formal-static"
            / "left.static-gate.json"
        )
        formal_hook_artifact = (
            temp_root
            / "artifacts"
            / "formal-hooks"
            / "formal-static"
            / "left.formal-hook.json"
        )
        self.assertEqual(emitted, [(fixture_path, static_gate_artifact, True)])
        self.assertEqual(
            delegated,
            [("static-gate", static_gate_artifact, formal_hook_artifact, True)],
        )

    def test_emit_formal_hook_static_writes_expected_contract_rows(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            source_artifact = temp_root / "left.static-gate.json"
            output_artifact = temp_root / "left.formal-hook.json"
            source_artifact.write_text(
                json.dumps(
                    {
                        "schema_version": "draft-current-l2-static-gate-v0",
                        "artifact_kind": "current-l2-static-gate-detached-sketch",
                        "fixture_context": {
                            "fixture_id": "e5_underdeclared_lineage",
                            "fixture_path": "/tmp/e5-underdeclared-lineage.json",
                            "source_example_id": "E5",
                        },
                        "checker_core": {
                            "static_verdict": "underdeclared",
                            "reasons": [
                                "missing lineage assertion for primary -> mirror"
                            ],
                        },
                    }
                ),
                encoding="utf-8",
            )

            exit_code = loop.emit_formal_hook(
                "static-gate",
                source_artifact,
                output_artifact,
                overwrite=True,
            )
            payload = json.loads(output_artifact.read_text(encoding="utf-8"))

        self.assertEqual(exit_code, 0)
        self.assertEqual(payload["subject_kind"], "fixture_static_cluster")
        self.assertEqual(payload["subject_ref"], "e5_underdeclared_lineage")
        self.assertEqual(
            payload["contract_rows"],
            [
                {
                    "obligation_kind": "canonical_normalization_law",
                    "evidence_refs": [
                        {
                            "ref_kind": "fixture",
                            "ref_id": "e5_underdeclared_lineage",
                        },
                        {
                            "ref_kind": "static_gate_artifact",
                            "ref_id": "e5_underdeclared_lineage",
                        },
                    ],
                },
                {
                    "obligation_kind": "no_re_promotion",
                    "evidence_refs": [
                        {
                            "ref_kind": "fixture",
                            "ref_id": "e5_underdeclared_lineage",
                        }
                    ],
                },
            ],
        )

    def test_suggest_reason_codes_emits_artifact_then_delegates_to_assist(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            assisted: list[list[str]] = []

            original_emit = loop.emit_static_gate
            original_assist = loop.reason_codes_assist.main

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_assist(argv: list[str] | None = None) -> int:
                assisted.append(list(argv or []))
                return 0

            loop.emit_static_gate = fake_emit
            loop.reason_codes_assist.main = fake_assist
            try:
                exit_code = loop.command_suggest_reason_codes(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="reason-codes-run",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.reason_codes_assist.main = original_assist

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted), 1)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "reason-codes-run"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted[0], (fixture_path, expected_artifact, True))
        self.assertEqual(assisted, [[str(fixture_path), str(expected_artifact)]])

    def test_scan_reason_code_readiness_emits_static_only_artifacts_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()

            static_fixture = fixture_dir / "left.json"
            static_fixture.write_text(
                '{"expected_runtime": {"enters_evaluation": false}}',
                encoding="utf-8",
            )
            runtime_fixture = fixture_dir / "right.json"
            runtime_fixture.write_text(
                '{"expected_runtime": {"enters_evaluation": true}}',
                encoding="utf-8",
            )

            emitted: list[tuple[Path, Path, bool]] = []
            assisted: list[list[str]] = []

            original_emit = loop.emit_static_gate
            original_readiness = loop.reason_code_readiness.main

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_readiness(argv: list[str] | None = None) -> int:
                assisted.append(list(argv or []))
                return 0

            loop.emit_static_gate = fake_emit
            loop.reason_code_readiness.main = fake_readiness
            try:
                exit_code = loop.command_scan_reason_code_readiness(
                    argparse.Namespace(
                        fixture_directory=str(fixture_dir),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="readiness-run",
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.reason_code_readiness.main = original_readiness

        self.assertEqual(exit_code, 0)

    def test_smoke_capability_checker_emits_artifact_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            checked: list[tuple[Path, Path]] = []

            original_emit = loop.emit_static_gate
            original_checker = loop.check_capability_third_checker

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_check(fixture: Path, artifact: Path) -> int:
                checked.append((fixture, artifact))
                return 0

            loop.emit_static_gate = fake_emit
            loop.check_capability_third_checker = fake_check
            try:
                exit_code = loop.command_smoke_capability_checker(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="cap-run",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.check_capability_third_checker = original_checker

        self.assertEqual(exit_code, 0)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "cap-run"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted, [(fixture_path, expected_artifact, True)])
        self.assertEqual(checked, [(fixture_path, expected_artifact)])

    def test_smoke_same_lineage_checker_emits_artifact_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            checked: list[list[str]] = []

            original_emit = loop.emit_static_gate
            original_checker = loop.same_lineage_checker.main

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_checker(argv: list[str] | None = None) -> int:
                checked.append(list(argv or []))
                return 0

            loop.emit_static_gate = fake_emit
            loop.same_lineage_checker.main = fake_checker
            try:
                exit_code = loop.command_smoke_same_lineage_checker(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="same-lineage-run",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.same_lineage_checker.main = original_checker

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted), 1)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "same-lineage-run"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted[0], (fixture_path, expected_artifact, True))
        self.assertEqual(checked, [[str(fixture_path), str(expected_artifact)]])

    def test_smoke_missing_option_checker_emits_artifact_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            checked: list[list[str]] = []

            original_emit = loop.emit_static_gate
            original_checker = loop.missing_option_checker.main

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_checker(argv: list[str] | None = None) -> int:
                checked.append(list(argv or []))
                return 0

            loop.emit_static_gate = fake_emit
            loop.missing_option_checker.main = fake_checker
            try:
                exit_code = loop.command_smoke_missing_option_checker(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="missing-option-run",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.missing_option_checker.main = original_checker

        self.assertEqual(exit_code, 0)
        self.assertEqual(len(emitted), 1)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "missing-option-run"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted[0], (fixture_path, expected_artifact, True))
        self.assertEqual(checked, [[str(fixture_path), str(expected_artifact)]])

    def test_smoke_try_rollback_structural_checker_emits_artifact_then_delegates(self) -> None:
        with tempfile.TemporaryDirectory() as temp_dir:
            temp_root = Path(temp_dir)
            fixture_dir = temp_root / "fixtures"
            fixture_dir.mkdir()
            fixture_path = fixture_dir / "left.json"
            fixture_path.write_text("{}", encoding="utf-8")

            emitted: list[tuple[Path, Path, bool]] = []
            checked: list[tuple[Path, Path]] = []

            original_emit = loop.emit_static_gate
            original_checker = loop.check_try_rollback_structural_checker

            def fake_emit(fixture: Path, output: Path, overwrite: bool) -> int:
                emitted.append((fixture, output, overwrite))
                output.parent.mkdir(parents=True, exist_ok=True)
                output.write_text("{}", encoding="utf-8")
                return 0

            def fake_check(fixture: Path, artifact: Path) -> int:
                checked.append((fixture, artifact))
                return 0

            loop.emit_static_gate = fake_emit
            loop.check_try_rollback_structural_checker = fake_check
            try:
                exit_code = loop.command_smoke_try_rollback_structural_checker(
                    argparse.Namespace(
                        fixture_path=str(fixture_path),
                        artifact_root=str(temp_root / "artifacts"),
                        run_label="try-rollback-structural",
                        output_path=None,
                        overwrite=True,
                    )
                )
            finally:
                loop.emit_static_gate = original_emit
                loop.check_try_rollback_structural_checker = original_checker

        self.assertEqual(exit_code, 0)
        expected_artifact = (
            temp_root
            / "artifacts"
            / "static-gates"
            / "try-rollback-structural"
            / "left.static-gate.json"
        )
        self.assertEqual(emitted, [(fixture_path, expected_artifact, True)])
        self.assertEqual(checked, [(fixture_path, expected_artifact)])


if __name__ == "__main__":
    unittest.main()
