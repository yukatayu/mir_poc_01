import json
import sys
import tempfile
import unittest
from io import StringIO
from pathlib import Path
from contextlib import redirect_stdout


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import current_l2_diff_static_gate_artifacts as static_gate_diff  # noqa: E402


def write_json(path: Path, payload: dict) -> None:
    path.write_text(json.dumps(payload, ensure_ascii=False), encoding="utf-8")


class StaticGateDiffTests(unittest.TestCase):
    def test_compare_checker_core_detects_verdict_and_reason_difference(self) -> None:
        left = {
            "checker_core": {
                "static_verdict": "malformed",
                "reasons": ["lineage assertion does not describe writer -> readonly"],
            }
        }
        right = {
            "checker_core": {
                "static_verdict": "underdeclared",
                "reasons": ["missing lineage assertion for writer -> readonly"],
            }
        }

        differences = static_gate_diff.compare_checker_core(
            static_gate_diff.read_checker_core(left),
            static_gate_diff.read_checker_core(right),
        )

        self.assertIn(
            '- checker_core.static_verdict: left="malformed" right="underdeclared"',
            differences,
        )
        self.assertIn(
            '- checker_core.reasons: left=["lineage assertion does not describe writer -> readonly"] right=["missing lineage assertion for writer -> readonly"]',
            differences,
        )

    def test_main_returns_zero_when_checker_core_matches(self) -> None:
        payload = {
            "fixture_context": {
                "fixture_id": "e4_malformed_lineage",
                "fixture_path": "fixtures/e4-malformed-lineage.json",
                "source_example_id": "e4",
            },
            "checker_core": {
                "static_verdict": "malformed",
                "reasons": ["lineage assertion does not describe writer -> readonly"],
            },
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            left_path = Path(temp_dir) / "left.json"
            right_path = Path(temp_dir) / "right.json"
            write_json(left_path, payload)
            write_json(right_path, payload)

            exit_code = static_gate_diff.main([str(left_path), str(right_path)])

        self.assertEqual(exit_code, 0)

    def test_main_keeps_detached_noncore_reason_code_diff_reference_only(self) -> None:
        left = {
            "fixture_context": {
                "fixture_id": "e4_malformed_lineage",
                "fixture_path": "fixtures/e4-malformed-lineage.json",
                "source_example_id": "e4",
            },
            "checker_core": {
                "static_verdict": "malformed",
                "reasons": ["lineage assertion does not describe writer -> readonly"],
            },
            "detached_noncore": {
                "reason_codes_scope": "stable-clusters-only",
                "reason_codes": [
                    {
                        "kind": "lineage_assertion_edge_mismatch",
                        "predecessor": "writer",
                        "successor": "readonly",
                    }
                ],
            },
        }
        right = {
            "fixture_context": {
                "fixture_id": "e4_malformed_lineage",
                "fixture_path": "fixtures/e4-malformed-lineage.json",
                "source_example_id": "e4",
            },
            "checker_core": {
                "static_verdict": "malformed",
                "reasons": ["lineage assertion does not describe writer -> readonly"],
            },
            "detached_noncore": {
                "reason_codes_scope": "stable-clusters-only",
                "reason_codes": [
                    {
                        "kind": "missing_lineage_assertion",
                        "predecessor": "writer",
                        "successor": "readonly",
                    }
                ],
            },
        }

        with tempfile.TemporaryDirectory() as temp_dir:
            left_path = Path(temp_dir) / "left.json"
            right_path = Path(temp_dir) / "right.json"
            write_json(left_path, left)
            write_json(right_path, right)

            buffer = StringIO()
            with redirect_stdout(buffer):
                exit_code = static_gate_diff.main([str(left_path), str(right_path)])
            output = buffer.getvalue()

        self.assertEqual(exit_code, 0)
        self.assertIn("reference-only differences:", output)
        self.assertIn("- detached_noncore.reason_codes:", output)
        self.assertNotIn("- detached_noncore: left=", output)


if __name__ == "__main__":
    unittest.main()
