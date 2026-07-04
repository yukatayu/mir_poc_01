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

    def _snapshot_phase_position_guard_text(self) -> str:
        return (
            "\n\nplan/149-current-phase-position-reading.md"
            "\nT0/G0 rebaseline"
            "\nphase 1 of 9"
            "\nG0 exit"
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
                    + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS)
                    + self._snapshot_phase_position_guard_text(),
                    encoding="utf-8",
                )
            elif relative == "tasks.md":
                path.write_text(
                    self._canon_notice_text()
                    + "\n\n"
                    + "\n\n".join(validate_docs.TASKS_REQUIRED_HEADINGS)
                    + self._snapshot_phase_position_guard_text(),
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
            "plan/84-g1-erow-carrier-only-diagnostic-detail-prototype.md",
            "plan/85-g1-erow-carrier-precondition-hardening.md",
            "plan/86-g1-erow002-visibility-repair-carrier-prototype.md",
            "plan/87-g1-obl025-lean-statement-draft.md",
            "plan/88-g1-erow-repair-shape-inventory.md",
            "plan/89-g1-erow001-non-visibility-singleton-fixture.md",
            "plan/90-source-traceability.md",
            "plan/91-maintenance-rules.md",
            "plan/92-g1-erow001-base-singleton-fixture-closure.md",
            "plan/93-g1-erow001-singleton-repair-assumption.md",
            "plan/94-g1-erow001-singleton-repair-prototype.md",
            "plan/95-g1-erow-mixed-multi-repair-decomposition-inventory.md",
            "plan/96-g1-erow-set-insertion-bundle-payload-inventory.md",
            "plan/97-g1-erow07-set-insertion-gate-review.md",
            "plan/98-g1-erow04-mixed-visibility-branch-inventory.md",
            "plan/99-g1-erow07-set-insertion-executable-preflight.md",
            "plan/100-g1-erow07-set-insertion-assumption-acceptance.md",
            "plan/101-g1-erow07-set-insertion-payload-model-design.md",
            "plan/102-g1-erow07-set-insertion-executable-payload-prototype.md",
            "plan/103-g1-erow07-set-insertion-negative-guard-hardening.md",
            "plan/104-g1-erow07-set-insertion-row-identity-guard-hardening.md",
            "plan/105-g1-erow07-set-insertion-exact-locus-guard-hardening.md",
            "plan/106-g1-erow07-child-bundle-partial-exclusion-fixtures.md",
            "plan/107-g1-erow04-mixed-visibility-payload-model-preflight.md",
            "plan/108-g1-obl025-branch-local-noncoverage-refinement.md",
            "plan/109-g1-obl024-lean-statement-draft.md",
            "plan/110-g1-obl024-executable-projection-carrier.md",
            "plan/111-g1-obl024-projection-rust-fixture-guards.md",
            "plan/112-g1-obl024-replay-vocabulary-preflight.md",
            "plan/113-g1-obl024-lean-replay-vocabulary-refinement.md",
            "plan/114-g1-obl024-lean-association-vocabulary-refinement.md",
            "plan/115-g1-obl024-association-guard-hardening.md",
            "plan/116-g1-obl025-repair-completeness-guard-hardening.md",
            "plan/117-g1-obl001-020-021-statement-guard-hardening.md",
            "plan/118-g0-g1-ordinary-assignment-claim-family-drilldown.md",
            "plan/119-g0-remaining-claim-family-drilldown-priority.md",
            "plan/120-repo-triage-recut-matrix.md",
            "plan/121-g1-minimal-vertical-slice-candidate-map.md",
            "plan/122-g1-scn-exact-static-slice-manifest.md",
            "plan/123-g1-scn01-visibility-negative-actualization.md",
            "plan/124-g1-obl001-boundary-audit.md",
            "plan/125-g1-scn02-direct-local-write-blocker-review.md",
            "plan/126-g1-obl020-021-boundary-audit-and-obl021-guard-hardening.md",
            "plan/127-g1-ordinary-assignment-bridge-readiness-nonreadiness-map.md",
            "plan/128-g1-bridge-handoff-blocker-ledger.md",
            "plan/129-g1-acceptance-packet-preflight.md",
            "plan/130-g1-obl-statement-status-completion-criteria-inventory.md",
            "plan/131-g1-status-proposal-packet-outline.md",
            "plan/132-g1-status-evidence-readiness-dry-run.md",
            "plan/133-g1-requested-status-options-matrix.md",
            "plan/134-g1-obl020-scope-clarification-packet.md",
            "plan/135-g1-obl020-artifact-identity-wrapper-preflight.md",
            "plan/136-g1-obl020-artifact-annex-template.md",
            "plan/137-g1-obl001-artifact-identity-wrapper-preflight.md",
            "plan/138-g1-obl001-artifact-annex-template.md",
            "plan/139-g1-obl021-artifact-identity-wrapper-preflight.md",
            "plan/140-g1-obl021-artifact-annex-template.md",
            "plan/141-g1-status-packet-shell-unresolved-slots.md",
            "plan/142-g1-status-packet-shell-evidence-dry-run.md",
            "plan/143-g1-obl021-equality-diagnostic-abstraction-decision-packet.md",
            "plan/144-g1-obl020-scope-decision-reuse-audit.md",
            "plan/145-g1-obl001-artifact-decision-reuse-audit.md",
            "plan/146-g1-obl001-explanation-boundary-guard-hardening.md",
            "plan/147-g1-next-line-promotion-boundary-audit.md",
            "plan/148-storage-workdir-mountpoint-guard-hardening.md",
            "plan/149-current-phase-position-reading.md",
            "plan/150-phase-position-validator-guard.md",
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

    def test_main_rejects_progress_missing_phase_position_guard(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Snapshot docs are missing phase-position guard phrases",
            stdout.getvalue(),
        )
        self.assertIn("progress.md", stdout.getvalue())
        self.assertIn("plan/149-current-phase-position-reading.md", stdout.getvalue())

    def test_main_rejects_tasks_missing_phase_position_guard(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "tasks.md").write_text(
                self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.TASKS_REQUIRED_HEADINGS),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Snapshot docs are missing phase-position guard phrases",
            stdout.getvalue(),
        )
        self.assertIn("tasks.md", stdout.getvalue())
        self.assertIn("T0/G0 rebaseline", stdout.getvalue())

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

    def test_numbered_plan_required_scaffold_matches_source_hierarchy(self) -> None:
        required_plan_docs = {
            path
            for path in validate_docs.REQUIRED
            if path.startswith("plan/")
            and validate_docs.NUMBERED_PLAN_FILE_PATTERN.fullmatch(Path(path).name)
        }
        source_hierarchy_plan_docs = {
            path
            for path in check_source_hierarchy.REQUIRED_PATHS["plan"]
            if validate_docs.NUMBERED_PLAN_FILE_PATTERN.fullmatch(Path(path).name)
        }

        self.assertSetEqual(required_plan_docs, source_hierarchy_plan_docs)

    def test_source_hierarchy_status_uses_portable_repo_root_display(self) -> None:
        status = check_source_hierarchy.build_status()
        pretty = check_source_hierarchy.format_pretty(status)

        self.assertEqual(status["repo_root"], ".")
        self.assertNotIn(str(check_source_hierarchy.REPO_ROOT), pretty)
        self.assertNotIn(str(check_source_hierarchy.REPO_ROOT), str(status))

    def test_all_repo_numbered_plan_files_are_registered(self) -> None:
        repo_plan_docs = {
            path.relative_to(validate_docs.ROOT).as_posix()
            for path in (validate_docs.ROOT / "plan").iterdir()
            if path.is_file()
            and validate_docs.NUMBERED_PLAN_FILE_PATTERN.fullmatch(path.name)
        }
        required_plan_docs = {
            path
            for path in validate_docs.REQUIRED
            if path.startswith("plan/")
            and validate_docs.NUMBERED_PLAN_FILE_PATTERN.fullmatch(Path(path).name)
        }
        source_hierarchy_plan_docs = {
            path
            for path in check_source_hierarchy.REQUIRED_PATHS["plan"]
            if validate_docs.NUMBERED_PLAN_FILE_PATTERN.fullmatch(Path(path).name)
        }

        self.assertSetEqual(repo_plan_docs, required_plan_docs)
        self.assertSetEqual(repo_plan_docs, source_hierarchy_plan_docs)

    def test_main_rejects_unregistered_numbered_plan_file(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            unregistered_plan = root / "plan" / "120-unregistered-plan.md"
            unregistered_plan.write_text("# unregistered plan\n", encoding="utf-8")

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Numbered plan files are not registered", stdout.getvalue())
        self.assertIn("plan/120-unregistered-plan.md", stdout.getvalue())

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

    def test_main_rejects_reader_facing_specs_as_normative_wording(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "hands_on" / "README.md").write_text(
                "# hands_on\n\n- 規範判断の正本は `specs/`\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Reader-facing docs contain stale source-hierarchy wording",
            stdout.getvalue(),
        )
        self.assertIn("docs/hands_on/README.md:3", stdout.getvalue())
        self.assertIn("規範判断の正本は `specs/`", stdout.getvalue())

    def test_main_rejects_japanese_specs_normative_variants(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "hands_on" / "README.md").write_text(
                "# hands_on\n\n"
                "- 規範判断の正本は`specs/`\n"
                "- 規範正本は `specs/`\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("docs/hands_on/README.md:3", stdout.getvalue())
        self.assertIn("docs/hands_on/README.md:4", stdout.getvalue())

    def test_main_allows_negated_specs_as_normative_policy_wording(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "hands_on" / "README.md").write_text(
                "# hands_on\n\nDo not treat `specs/` as normative.\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())

    def test_main_rejects_stale_wording_in_canon_and_sample_entry_docs(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "CANON.md").write_text(
                "# Canon\n\nNormative source remains `specs/00..09`.\n",
                encoding="utf-8",
            )
            (root / "samples" / "README.md").write_text(
                "# samples\n\n- `specs/`\n  規範正本\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("CANON.md:3", stdout.getvalue())
        self.assertIn("samples/README.md:3", stdout.getvalue())

    def test_main_rejects_split_line_and_english_specs_normative_wording(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "plan" / "19-repository-map-and-taxonomy.md").write_text(
                "# plan/19\n\n- `specs/`\n  規範正本\n",
                encoding="utf-8",
            )
            (root / "plan" / "58-full-system-v1-roadmap.md").write_text(
                "# plan/58\n\nNormative source remains `specs/33..38`.\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("plan/19-repository-map-and-taxonomy.md:3", stdout.getvalue())
        self.assertIn("plan/58-full-system-v1-roadmap.md:3", stdout.getvalue())

    def test_main_rejects_active_reader_host_absolute_repo_paths(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "research_abstract" / "clean_near_end_typing_01_detail.md").write_text(
                "# detail\n\n"
                '"source_path": "/home/alice/dev/mir_poc_01/samples/clean-near-end/typing/01_authorized_declassification.mir"\n',
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Active reader-facing docs contain host-specific repo paths",
            stdout.getvalue(),
        )
        self.assertIn(
            "docs/research_abstract/clean_near_end_typing_01_detail.md:3",
            stdout.getvalue(),
        )

    def test_main_allows_historical_host_paths_outside_active_reader_lint(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text()
                + "\n\nHistorical output: /home/yukatayu/dev/mir_poc_01/target/debug\n",
                encoding="utf-8",
            )
            old_detail = (
                root
                / "docs"
                / "research_abstract"
                / "old"
                / "2026-04-22-pre-clean-near-end"
                / "order_01_detail.md"
            )
            old_detail.parent.mkdir(parents=True, exist_ok=True)
            old_detail.write_text(
                "sample_path: /home/yukatayu/dev/mir_poc_01/samples/prototype/example.txt\n",
                encoding="utf-8",
            )
            old_sample = root / "samples" / "old" / "historical.md"
            old_sample.parent.mkdir(parents=True, exist_ok=True)
            old_sample.write_text(
                "sample_path: /home/alice/dev/mir_poc_01/samples/old/historical.txt\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())

    def test_main_rejects_stale_snapshot_last_updated_header(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                "最終更新: 2026-07-04 12:13 JST\n\n"
                + self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS)
                + "\n\n- 2026-07-04 12:29 JST\n  later work log\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Snapshot docs have stale last-updated headers", stdout.getvalue())
        self.assertIn("progress.md", stdout.getvalue())
        self.assertIn("2026-07-04 12:13 JST", stdout.getvalue())
        self.assertIn("2026-07-04 12:29 JST", stdout.getvalue())

    def test_main_allows_current_snapshot_last_updated_header(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                "最終更新: 2026-07-04 12:29 JST\n\n"
                + self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS)
                + self._snapshot_phase_position_guard_text()
                + "\n\n- 2026-07-04 12:13 JST\n  earlier work log\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())

    def test_main_rejects_stale_tasks_last_updated_header(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "tasks.md").write_text(
                "最終更新: 2026-07-04 12:13 JST\n\n"
                + self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.TASKS_REQUIRED_HEADINGS)
                + "\n\n- 2026-07-04 12:38 JST\n  later task-map note\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Snapshot docs have stale last-updated headers", stdout.getvalue())
        self.assertIn("tasks.md", stdout.getvalue())

    def test_main_rejects_stale_samples_progress_last_updated_header(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "samples_progress.md").write_text(
                "# samples_progress.md\n\n"
                "Last updated: 2026-07-04 12:13 JST\n\n"
                "## Current status\n\n"
                "| sample | status |\n"
                "| --- | --- |\n"
                "| samples/clean-near-end | current |\n\n"
                "## Recent Validation Log\n\n"
                "- 2026-07-04 12:38 JST: later sample dashboard note\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Snapshot docs have stale last-updated headers", stdout.getvalue())
        self.assertIn("samples_progress.md", stdout.getvalue())
        self.assertIn("2026-07-04 12:13 JST", stdout.getvalue())
        self.assertIn("2026-07-04 12:38 JST", stdout.getvalue())

    def test_snapshot_top_last_updated_timestamp_accepts_english_label(self) -> None:
        text = "# samples_progress.md\n\nLast updated: 2026-07-04 12:38 JST\n\n"

        self.assertEqual(
            validate_docs.snapshot_top_last_updated_timestamp(text),
            "2026-07-04 12:38 JST",
        )

    def test_main_rejects_missing_top_snapshot_last_updated_header(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                "# progress\n\n"
                + self._canon_notice_text()
                + "\n\n"
                + "\n\n".join(validate_docs.PROGRESS_REQUIRED_HEADINGS)
                + "\n\n最終更新: 2026-07-04 12:38 JST\n",
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(),
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Snapshot docs have stale last-updated headers", stdout.getvalue())
        self.assertIn("progress.md", stdout.getvalue())
        self.assertIn("missing", stdout.getvalue())

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
