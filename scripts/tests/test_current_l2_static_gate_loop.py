import argparse
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


if __name__ == "__main__":
    unittest.main()
