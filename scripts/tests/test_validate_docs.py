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
import check_source_hierarchy


class ValidateDocsTests(unittest.TestCase):
    def _canon_notice_text(self) -> str:
        return (
            "# canon notice\n\n"
            "`mirrorea_canon/` is the normative source. Everything outside "
            "`mirrorea_canon/` is LAB; if LAB text conflicts, canon wins.\n"
        )

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
            elif relative == "progress.md":
                path.write_text(
                    self._canon_notice_text()
                    + "\n\n"
                    + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS),
                    encoding="utf-8",
                )
            elif relative == "tasks.md":
                path.write_text(
                    self._canon_notice_text()
                    + "\n\n"
                    + "\n\n".join(validate_docs.TASKS_REQUIRED_HEADINGS),
                    encoding="utf-8",
                )
            elif relative in validate_docs.CANON_NOTICE_FILES:
                path.write_text(self._canon_notice_text(), encoding="utf-8")
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
            "samples/practical-alpha1/README.md",
            "samples/practical-alpha1/packages/README.md",
            "samples/practical-alpha1/source/README.md",
            "samples/practical-alpha1/expected/README.md",
            "samples/practical-alpha1/docker/README.md",
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
            "plan/44-practical-alpha1-roadmap.md",
            "specs/13-type-system-lifetime-fallback.md",
            "specs/14-contract-subtyping-layer-compatibility.md",
            "specs/15-cut-save-load-checkpoint.md",
            "specs/16-runtime-package-adapter-hotplug.md",
            "specs/17-mirrorea-spaces-alpha-scope.md",
            "specs/18-practical-alpha1-scope.md",
        }
        for path in alpha0_required:
            self.assertIn(path, required)

    def test_required_scaffold_includes_canon_entry_docs(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        canon_required = {
            "CANON.md",
            "mirrorea_canon/README.md",
            "mirrorea_canon/MAP.md",
            "mirrorea_canon/INDEX.json",
            "mirrorea_canon/meta/source-hierarchy.md",
            "mirrorea_canon/adr/ADR-0012.md",
            "mirrorea_canon/plan/00-gates.md",
            "mirrorea_canon/plan/01-phases.md",
            "mirrorea_canon/spec/06-conformance.md",
            "mirrorea_canon/theory/11-metatheory-ledger.md",
        }

        for path in canon_required:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

    def test_required_scaffold_includes_product_alpha1_boundary_docs(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        product_alpha1_required = {
            "specs/25-product-alpha1-public-boundary.md",
            "plan/50-product-alpha1-public-boundary-roadmap.md",
            "specs/26-operational-product-sample-suite.md",
            "specs/27-spatial-portal-and-shard-extension-boundary.md",
            "plan/51-operational-product-sample-roadmap.md",
            "plan/52-portal-spatial-world-roadmap.md",
            "specs/28-mir-computational-core.md",
            "specs/29-transform-posegraph-semantics.md",
            "specs/30-projection-and-backend-boundary.md",
            "specs/31-engine-wasm-ffi-adapter-boundary.md",
            "specs/32-autonomous-execution-and-completion-contract.md",
            "plan/53-mir-computational-core-roadmap.md",
            "plan/54-transform-posegraph-roadmap.md",
            "plan/55-projection-backend-roadmap.md",
            "plan/56-engine-adapter-roadmap.md",
            "plan/57-autonomous-computational-core-master-plan.md",
        }

        for path in product_alpha1_required:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

    def test_required_scaffold_includes_full_system_v1_docs(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        full_system_required = {
            "specs/33-full-system-v1-scope.md",
            "specs/34-textual-mir-alpha-grammar.md",
            "specs/35-mir-typed-ir-and-interpreter.md",
            "specs/36-projection-ir-and-boundary-preservation.md",
            "specs/37-posegraph-runtime-semantics.md",
            "specs/38-engine-provider-admission.md",
            "plan/58-full-system-v1-roadmap.md",
            "plan/59-textual-mir-roadmap.md",
            "plan/60-computational-runtime-roadmap.md",
            "plan/61-posegraph-runtime-roadmap.md",
            "plan/62-projection-backend-roadmap.md",
            "plan/63-engine-provider-roadmap.md",
            "docs/hands_on/full_system_v1_roadmap_01.md",
            "docs/research_abstract/full_system_v1_roadmap_01.md",
            "samples/full-system-v1/README.md",
            "samples/full-system-v1/computational/README.md",
            "samples/full-system-v1/computational/matrix.json",
            "samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir",
            "samples/full-system-v1/computational/missing-type-annotation-negative/src/missing-type-annotation.mir",
            "samples/full-system-v1/computational/malformed-record-negative/src/malformed-record.mir",
            "samples/full-system-v1/computational/malformed-transition-negative/src/malformed-transition.mir",
            "samples/full-system-v1/computational/malformed-capability-negative/src/malformed-capability.mir",
            "samples/full-system-v1/computational/contract-clause-position-negative/src/contract-clause-position.mir",
            "samples/full-system-v1/projection/effectful-sugoroku-positive/main/src/effectful-sugoroku-positive.mir",
            "samples/full-system-v1/projection/effectful-sugoroku-positive/projection.request.json",
            "samples/full-system-v1/server-client/role-split-positive/main/src/role-split-positive.mir",
            "samples/full-system-v1/server-client/role-split-positive/projection.request.json",
            "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/main/src/viewer-diagnostic-positive.mir",
            "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/projection.request.json",
            "samples/full-system-v1/provider-adapter/viewer-diagnostic-positive/provider.manifest.json",
            "samples/full-system-v1/provider-adapter/renderer-pose-positive/main/src/renderer-pose-positive.mir",
            "samples/full-system-v1/provider-adapter/renderer-pose-positive/projection.request.json",
            "samples/full-system-v1/provider-adapter/renderer-pose-positive/provider.manifest.json",
            "samples/full-system-v1/provider-adapter/renderer-pose-positive/package.mir.json",
            "scripts/textual_mir_samples.py",
            "scripts/tests/test_textual_mir_samples.py",
        }

        for path in full_system_required:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

        self.assertIn(
            "sub-agent-pro/full-system-completion-001/20-progress-tasks-replacement-model.md",
            required_docs,
        )
        self.assertIn("sub-agent-pro/full-system-completion-001", required_hierarchy)

    def test_required_scaffold_includes_surface_mir_rebaseline_docs(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        surface_required = {
            "specs/39-surface-mir-placement-elaboration.md",
            "specs/40-indexed-state-semantics.md",
            "specs/41-role-admission-and-capability-grant.md",
            "specs/42-source-patch-hotplug-semantics.md",
            "specs/43-surface-mir-v1-alpha-scope.md",
            "plan/64-surface-mir-placement-roadmap.md",
            "plan/65-indexed-state-roadmap.md",
            "plan/66-role-admission-roadmap.md",
            "plan/67-source-patch-hotplug-roadmap.md",
            "plan/68-surface-full-system-v1-roadmap.md",
            "plan/69-consultation-synthesis-and-management-roadmap.md",
            "plan/70-lab-to-canon-reconciliation-ledger.md",
            "plan/71-g1-ordinary-assignment-target.md",
            "plan/72-g1-scn01-scn02-static-consequence-drilldown.md",
            "plan/73-g1-obl001-lean-statement-inventory.md",
            "plan/74-g1-obl001-lean-statement-draft.md",
            "plan/75-g1-scn-rhs-dependency-gap-evidence.md",
            "plan/76-g1-obl020-021-dependency-inventory.md",
            "plan/77-g1-obl021-lean-statement-draft.md",
            "plan/78-g1-obl020-lean-statement-draft.md",
            "plan/79-g1-erow-diagnostic-alignment.md",
            "plan/80-g1-diagnostic-carrier-inventory.md",
            "plan/81-g1-obl024-statement-shape-inventory.md",
            "plan/82-g1-obl025-statement-shape-inventory.md",
            "plan/83-g1-erow-repair-payload-inventory.md",
            "plan/90-source-traceability.md",
            "plan/91-maintenance-rules.md",
            "docs/hands_on/surface_mir_alpha_01.md",
            "docs/hands_on/source_patch_hotplug_01.md",
            "docs/research_abstract/surface_mir_alpha_01.md",
            "samples/full-system-v1-surface/README.md",
            "samples/full-system-v1-surface/syntax/README.md",
            "samples/full-system-v1-surface/syntax/matrix.json",
            "samples/full-system-v1-surface/indexed-state/README.md",
            "samples/full-system-v1-surface/indexed-state/matrix.json",
            "samples/full-system-v1-surface/elaboration/README.md",
            "samples/full-system-v1-surface/elaboration/matrix.json",
            "samples/full-system-v1-surface/role-admission/README.md",
            "samples/full-system-v1-surface/role-admission/matrix.json",
            "samples/full-system-v1-surface/source-patch/README.md",
            "samples/full-system-v1-surface/source-patch/matrix.json",
            "samples/full-system-v1-surface/devtools/README.md",
            "samples/full-system-v1-surface/devtools/matrix.json",
            "samples/full-system-v1-surface/operational-matrix.json",
            "samples/full-system-v1-surface/world-core/README.md",
            "samples/full-system-v1-surface/membership-chat/README.md",
            "samples/full-system-v1-surface/sugoroku-world/README.md",
            "samples/full-system-v1-surface/portal-worldlink/README.md",
            "samples/full-system-v1-surface/two-shard-hard-boundary/README.md",
            "samples/full-system-v1-surface/gradient-observation/README.md",
            "scripts/surface_mir_samples.py",
            "scripts/surface_mir_release_check.py",
            "scripts/surface_mir_authoring_check.py",
            "scripts/tests/test_surface_mir_samples.py",
            "scripts/tests/test_surface_mir_release_check.py",
        }

        for path in surface_required:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

    def test_snapshot_heading_contracts_include_full_system_rebaseline_shape(self) -> None:
        progress_headings = validate_docs.PROGRESS_REQUIRED_HEADINGS
        tasks_headings = validate_docs.TASKS_REQUIRED_HEADINGS

        for heading in [
            "## current milestone position",
            "## milestone map",
            "### Product Alpha line",
            "### Operational Suite line",
            "### Mir Language line",
            "### PoseGraph line",
            "### Projection/Backend line",
            "### Engine/Provider line",
            "## validation floor",
            "## non-claims",
            "## user decision items vs research-discovery items",
        ]:
            self.assertIn(heading, progress_headings)

        for heading in [
            "## current promoted package",
            "## ordered self-driven packages",
            "## self-driven macro phase reading",
            "## user decision gates",
            "## research discovery items",
            "## maintenance tasks",
            "## non-promoted references",
        ]:
            self.assertIn(heading, tasks_headings)

    def test_current_snapshot_docs_have_required_heading_order(self) -> None:
        root = Path(__file__).resolve().parents[2]
        progress_text = (root / "progress.md").read_text(encoding="utf-8")
        tasks_text = (root / "tasks.md").read_text(encoding="utf-8")

        self.assertEqual(
            [],
            validate_docs.missing_headings(
                progress_text, validate_docs.PROGRESS_REQUIRED_HEADINGS
            ),
        )
        self.assertEqual(
            [],
            validate_docs.out_of_order_headings(
                progress_text, validate_docs.PROGRESS_REQUIRED_HEADINGS
            ),
        )
        self.assertEqual(
            [],
            validate_docs.missing_headings(
                tasks_text, validate_docs.TASKS_REQUIRED_HEADINGS
            ),
        )
        self.assertEqual(
            [],
            validate_docs.out_of_order_headings(
                tasks_text, validate_docs.TASKS_REQUIRED_HEADINGS
            ),
        )

    def test_required_scaffold_includes_product_alpha1_sample_docs(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        product_alpha1_sample_docs = {
            "samples/product-alpha1/README.md",
            "samples/product-alpha1/computational/README.md",
            "samples/product-alpha1/computational/matrix.json",
            "samples/product-alpha1/computational/add-one-pure-mir/README.md",
            "samples/product-alpha1/computational/add-one-pure-mir/add-one-pure-mir.mir",
            "samples/product-alpha1/posegraph/README.md",
            "samples/product-alpha1/posegraph/matrix.json",
            "samples/product-alpha1/posegraph/avatar-head-transform/README.md",
            "samples/product-alpha1/posegraph/avatar-head-transform/avatar-head-transform.mir",
            "samples/product-alpha1/projection/README.md",
            "samples/product-alpha1/projection/matrix.json",
            "samples/product-alpha1/projection/server-client-target-manifest/server-client-target-manifest.json",
            "samples/product-alpha1/engine-adapter/README.md",
            "samples/product-alpha1/engine-adapter/matrix.json",
            "samples/product-alpha1/engine-adapter/renderer/renderer.contract.json",
            "samples/product-alpha1/demo/README.md",
            "samples/product-alpha1/demo/package.mir.json",
            "samples/product-alpha1/operational/README.md",
            "samples/product-alpha1/operational/world-core/README.md",
            "samples/product-alpha1/operational/world-core/package.mir.json",
            "samples/product-alpha1/operational/membership-chat/README.md",
            "samples/product-alpha1/operational/membership-chat/package.mir.json",
            "samples/product-alpha1/operational/sugoroku-world/README.md",
            "samples/product-alpha1/operational/sugoroku-world/package.mir.json",
        }

        for path in product_alpha1_sample_docs:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

    def test_required_scaffold_includes_computational_helper_surface(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        computational_helper_docs = {
            "scripts/mir_computational_samples.py",
            "scripts/tests/test_mir_computational_samples.py",
            "scripts/posegraph_samples.py",
            "scripts/tests/test_posegraph_samples.py",
            "scripts/projection_boundary_samples.py",
            "scripts/tests/test_projection_boundary_samples.py",
            "scripts/engine_adapter_boundary_samples.py",
            "scripts/tests/test_engine_adapter_boundary_samples.py",
        }

        for path in computational_helper_docs:
            self.assertIn(path, required_docs)
            self.assertIn(path, required_hierarchy)

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

    def test_main_rejects_missing_canon_notice(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "README.md").write_text("# README without notice\n", encoding="utf-8")

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Root entry documents are missing canon notices", stdout.getvalue())
        self.assertIn("README.md", stdout.getvalue())

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
