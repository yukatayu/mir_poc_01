import json
import sys
import unittest
from pathlib import Path
from unittest import mock


REPO_ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO_ROOT / "scripts"))

import alpha_cut_save_load_samples as runner  # noqa: E402


class AlphaCutSaveLoadSamplesTests(unittest.TestCase):
    def sidecar_path(self, relative: str) -> Path:
        return REPO_ROOT / relative

    def test_closeout_reports_runtime_and_checker_rows(self) -> None:
        payload = runner.closeout()
        self.assertEqual(payload["implemented_rows"], ["CUT-04", "CUT-17"])
        self.assertIn("CUT-05", payload["checker_backed_rows"])
        self.assertIn("CUT-11", payload["checker_backed_rows"])
        self.assertIn("CUT-10", payload["planned_only_rows"])
        self.assertIn("CUT-12", payload["planned_only_rows"])
        self.assertIn("CUT-16", payload["planned_only_rows"])
        self.assertIn(
            "cargo test -p mirrorea-core --test runtime_substrate",
            payload["validation_floor"],
        )
        self.assertFalse(payload["distributed_save_load_claimed"])
        self.assertFalse(payload["durable_cut_claimed"])

    def test_list_samples_exposes_runtime_row(self) -> None:
        rows = runner.list_samples()
        self.assertEqual([row["sample_id"] for row in rows], ["CUT-04", "CUT-17"])
        self.assertTrue(all(row["family"] == "alpha-cut-save-load" for row in rows))

    def test_run_sample_accepts_exact_sidecar(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        with mock.patch.object(runner, "_build_runtime_report", return_value=sidecar):
            report = runner.run_sample(row["sample_id"])
        self.assertEqual(report, sidecar)

    def test_run_sample_rejects_sidecar_drift(self) -> None:
        row = runner.IMPLEMENTED_ROWS[0]
        sidecar = json.loads(self.sidecar_path(row["expected_sidecar"]).read_text())
        drifted = dict(sidecar)
        drifted["terminal_outcome"] = "rejected"
        with mock.patch.object(runner, "_build_runtime_report", return_value=drifted):
            with self.assertRaisesRegex(RuntimeError, "expected terminal_outcome"):
                runner.run_sample(row["sample_id"])


if __name__ == "__main__":
    unittest.main()
