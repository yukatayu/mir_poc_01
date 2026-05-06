from __future__ import annotations

import io
import json
import sys
import unittest
from contextlib import redirect_stdout
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import operational_product_samples


class OperationalProductSamplesTests(unittest.TestCase):
    def test_list_samples_includes_operational_roots(self) -> None:
        payload = operational_product_samples.list_samples()

        self.assertEqual(
            payload["package_name"],
            "P-OPS-01 operational product sample suite scaffold and first workflow",
        )
        roots = {row["root"] for row in payload["samples"]}
        self.assertIn("samples/product-alpha1/operational/world-core", roots)
        self.assertIn("samples/product-alpha1/operational/membership-chat", roots)
        self.assertIn("samples/product-alpha1/operational/sugoroku-world", roots)

    def test_sample_rows_marks_future_portal_as_non_runnable(self) -> None:
        rows = operational_product_samples.sample_rows()
        portal = next(row for row in rows if row["sample_id"] == "OPS-06")

        self.assertFalse(portal["runnable"])
        self.assertEqual(portal["package_kind"], "portal_worldlink")

    def test_operational_attach_specs_include_deferred_boundaries(self) -> None:
        specs = operational_product_samples.operational_attach_specs()

        self.assertIn(
            ("placeholder-object", operational_product_samples.LAYERS_ROOT / "placeholder-object", "deferred"),
            specs,
        )
        self.assertIn(
            (
                "custom-avatar-preview",
                operational_product_samples.LAYERS_ROOT / "custom-avatar-preview",
                "deferred",
            ),
            specs,
        )

    def test_main_accepts_global_format_before_subcommand(self) -> None:
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            exit_code = operational_product_samples.main(["--format", "json", "list"])

        self.assertEqual(exit_code, 0)
        payload = json.loads(stdout.getvalue())
        self.assertEqual(
            payload["surface_kind"], "operational_product_sample_suite_list"
        )

    def test_main_accepts_format_after_subcommand(self) -> None:
        stdout = io.StringIO()
        with redirect_stdout(stdout):
            exit_code = operational_product_samples.main(["list", "--format", "json"])

        self.assertEqual(exit_code, 0)
        payload = json.loads(stdout.getvalue())
        self.assertEqual(
            payload["surface_kind"], "operational_product_sample_suite_list"
        )


if __name__ == "__main__":
    unittest.main()
