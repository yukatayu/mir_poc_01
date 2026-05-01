from __future__ import annotations

import io
import sys
import tempfile
import unittest
from contextlib import redirect_stdout
from pathlib import Path
from unittest import mock

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))

import validate_docs


class ValidateDocsTests(unittest.TestCase):
    def _valid_template_text(self) -> str:
        return "\n".join(validate_docs.REQUIRED_TEMPLATE_HEADINGS)

    def _valid_report_text(self) -> str:
        return "\n\n".join(
            f"{heading}\n\nRecorded content for {heading}."
            for heading in validate_docs.REQUIRED_TEMPLATE_HEADINGS
        )

    def _valid_report_text_without(self, omitted_heading: str) -> str:
        return "\n\n".join(
            f"{heading}\n\nRecorded content for {heading}."
            for heading in validate_docs.REQUIRED_TEMPLATE_HEADINGS
            if heading != omitted_heading
        )

    def _write_required_scaffold(self, root: Path, template_text: str) -> None:
        for relative in validate_docs.REQUIRED:
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            if relative == "docs/reports/TEMPLATE.md":
                path.write_text(template_text, encoding="utf-8")
            else:
                path.write_text(f"# {relative}\n", encoding="utf-8")
        (root / "docs" / "reports" / "0001-smoke.md").write_text(
            "# Report 0001\n", encoding="utf-8"
        )

    def test_report_template_requires_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = (
            Path(__file__).resolve().parents[2]
            / "docs"
            / "reports"
            / "TEMPLATE.md"
        ).read_text(encoding="utf-8")

        self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        self.assertIn(heading, template_text)

    def test_report_template_requires_documentation_update_status_section(self) -> None:
        heading = "## Documentation.md update status"
        template_text = (
            Path(__file__).resolve().parents[2]
            / "docs"
            / "reports"
            / "TEMPLATE.md"
        ).read_text(encoding="utf-8")

        self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        self.assertIn(heading, template_text)

    def test_report_template_requires_dirty_state_and_reviewer_sections(self) -> None:
        headings = [
            "## Start state / dirty state",
            "## Reviewer findings and follow-up",
        ]
        template_text = (
            Path(__file__).resolve().parents[2]
            / "docs"
            / "reports"
            / "TEMPLATE.md"
        ).read_text(encoding="utf-8")

        for heading in headings:
            self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)
            self.assertIn(heading, template_text)

    def test_required_scaffold_includes_alpha0_docs(self) -> None:
        required = set(validate_docs.REQUIRED)
        alpha0_required = {
            "progress.md",
            "tasks.md",
            "samples_progress.md",
            "samples/README.md",
            "samples/alpha/README.md",
            "samples/alpha/lifetime-fallback/README.md",
            "samples/alpha/contract-variance/README.md",
            "samples/alpha/cut-save-load/README.md",
            "samples/alpha/local-runtime/README.md",
            "samples/alpha/layer-insertion/README.md",
            "samples/alpha/network-docker/README.md",
            "samples/alpha/hotplug-runtime/README.md",
            "samples/alpha/avatar-runtime/README.md",
            "samples/alpha/visualization/README.md",
            "samples/alpha/e2e/README.md",
            "scripts/README.md",
            "plan/01-status-at-a-glance.md",
            "plan/11-roadmap-near-term.md",
            "plan/19-repository-map-and-taxonomy.md",
            "plan/39-type-system-freeze-roadmap.md",
            "plan/40-layer-compatibility-freeze-roadmap.md",
            "plan/41-save-load-checkpoint-roadmap.md",
            "plan/42-runtime-package-avatar-roadmap.md",
            "plan/43-alpha-e2e-roadmap.md",
            "specs/13-type-system-lifetime-fallback.md",
            "specs/14-contract-subtyping-layer-compatibility.md",
            "specs/15-cut-save-load-checkpoint.md",
            "specs/16-runtime-package-adapter-hotplug.md",
            "specs/17-mirrorea-spaces-alpha-scope.md",
        }
        for path in alpha0_required:
            self.assertIn(path, required)

    def test_main_rejects_template_missing_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = "\n".join(h for h in validate_docs.REQUIRED_TEMPLATE_HEADINGS if h != heading)

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Report template is missing required sections", stdout.getvalue())
        self.assertIn(heading, stdout.getvalue())

    def test_main_rejects_latest_report_missing_commands_run_section(self) -> None:
        heading = "## Commands run"
        template_text = self._valid_template_text()
        latest_report_text = self._valid_report_text_without(heading)

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report is missing required sections", stdout.getvalue())
        self.assertIn("0002-latest.md", stdout.getvalue())
        self.assertIn(heading, stdout.getvalue())

    def test_main_rejects_latest_report_missing_new_required_section(self) -> None:
        heading = "## Reviewer findings and follow-up"
        template_text = self._valid_template_text()
        latest_report_text = self._valid_report_text_without(heading)

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report is missing required sections", stdout.getvalue())
        self.assertIn(heading, stdout.getvalue())

    def test_main_rejects_latest_report_with_required_sections_out_of_order(self) -> None:
        template_text = self._valid_template_text()
        headings = list(validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        headings[1], headings[2] = headings[2], headings[1]
        latest_report_text = "\n\n".join(
            f"{heading}\n\nRecorded content for {heading}." for heading in headings
        )

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report has required sections out of order", stdout.getvalue())

    def test_main_rejects_latest_report_with_empty_required_section(self) -> None:
        template_text = self._valid_template_text()
        latest_report_text = self._valid_report_text().replace(
            "## Commands run\n\nRecorded content for ## Commands run.",
            "## Commands run\n\n",
        )

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report has empty required sections", stdout.getvalue())
        self.assertIn("## Commands run", stdout.getvalue())

    def test_main_rejects_latest_report_with_unresolved_template_placeholder(self) -> None:
        template_text = self._valid_template_text()
        latest_report_text = self._valid_report_text().replace(
            "Recorded content for ## Plan update status.",
            "`plan/` 更新不要 / 更新済み:",
        )

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report has unresolved template placeholders", stdout.getvalue())
        self.assertIn("## Plan update status", stdout.getvalue())

    def test_main_allows_historical_report_missing_heading_when_latest_is_valid(self) -> None:
        heading = "## Commands run"
        template_text = self._valid_template_text()
        historical_report_text = self._valid_report_text_without(heading)
        latest_report_text = self._valid_report_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0001-smoke.md").write_text(
                historical_report_text, encoding="utf-8"
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                latest_report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())


if __name__ == "__main__":
    unittest.main()
