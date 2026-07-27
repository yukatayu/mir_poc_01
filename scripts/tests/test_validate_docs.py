from __future__ import annotations

import hashlib
import io
import json
import os
import subprocess
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
            (
                f"{heading}\n\n"
                "更新不要: project-status update trigger did not change."
                if heading == "## docs/project-status.md update status"
                else f"{heading}\n\nRecorded content for {heading}."
            )
            for heading in validate_docs.REQUIRED_TEMPLATE_HEADINGS
        )

    def _valid_report_text_without(self, omitted_heading: str) -> str:
        return "\n\n".join(
            f"{heading}\n\nRecorded content for {heading}."
            for heading in validate_docs.REQUIRED_TEMPLATE_HEADINGS
            if heading != omitted_heading
        )

    def _valid_snapshot_text(
        self, headings: list[str], position_heading: str
    ) -> str:
        return self._canon_notice_text() + "\n\n" + "\n\n".join(
            (
                f"{heading}\n\n"
                "Future lifecycle example with cited sources:\n"
                "`mirrorea_canon/plan/01-phases.md`\n"
                "`plan/149-current-phase-position-reading.md`"
            )
            if heading == position_heading
            else heading
            for heading in headings
        )

    def _valid_project_status_text(self) -> str:
        sections = {
            "## この文書の役割": (
                "これは LAB 派生ビューであり、`mirrorea_canon/` が唯一の規範正本です。"
            ),
            "## 全体の進行チェックリスト": "[ ] G0\n[ ] T0",
            "## 現在地": "Current lifecycle: T0/G0 rebaseline.\n`mirrorea_canon/plan/01-phases.md`",
            "## 現在の停止線": "Current stop source: `plan/153-g0-closeout-evidence-and-exit-decision-packet.md`",
            "## オーナーの確認・判断待ち": (
                "Owner decision is unresolved.\n"
                "`plan/153-g0-closeout-evidence-and-exit-decision-packet.md`"
            ),
            "## 根拠と詳細": (
                "`plan/154-project-control-cockpit.md`\n"
                "`docs/reports/0001-smoke.md`\n"
                "`progress.md`\n`tasks.md`\n`samples_progress.md`"
            ),
            "## 更新規約": "Sources update before this derived view.",
        }
        return (
            self._canon_notice_text()
            + "\n\n# Project status\n\n"
            + "\n\n".join(
                f"{heading}\n\n{sections[heading]}"
                for heading in validate_docs.PROJECT_STATUS_REQUIRED_HEADINGS
            )
        )

    def _fake_concrete_discord_webhook_url(self) -> str:
        return (
            "https://discord.com/api/"
            + "webhooks/123456789012345678/"
            + ("A" * 48)
        )

    def _write_required_scaffold(self, root: Path, template_text: str) -> None:
        for relative in validate_docs.REQUIRED:
            path = root / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            if relative == "docs/reports/TEMPLATE.md":
                path.write_text(template_text, encoding="utf-8")
            elif relative == "progress.md":
                path.write_text(
                    self._valid_snapshot_text(
                        validate_docs.PROGRESS_REQUIRED_HEADINGS,
                        "## current milestone position",
                    ),
                    encoding="utf-8",
                )
            elif relative == "tasks.md":
                path.write_text(
                    self._valid_snapshot_text(
                        validate_docs.TASKS_REQUIRED_HEADINGS,
                        "## current promoted package",
                    ),
                    encoding="utf-8",
                )
            elif relative == "docs/project-status.md":
                path.write_text(
                    self._valid_project_status_text(), encoding="utf-8"
                )
            elif relative in validate_docs.CANON_NOTICE_FILES:
                path.write_text(self._canon_notice_text(), encoding="utf-8")
            else:
                path.write_text(f"# {relative}\n", encoding="utf-8")
        (root / "docs" / "reports" / "0001-smoke.md").write_text(
            "# Report 0001\n", encoding="utf-8"
        )

    def _git(
        self, root: Path, *args: str, env: dict[str, str] | None = None
    ) -> str:
        result = subprocess.run(
            ["git", "-C", str(root), *args],
            check=True,
            capture_output=True,
            text=True,
            env=env,
        )
        return result.stdout.strip()

    def _signed_identity(
        self, root: Path, email: str
    ) -> tuple[dict[str, str], str]:
        gnupg_home = root / "test-gnupg"
        gnupg_home.mkdir(mode=0o700, exist_ok=True)
        environment = {**os.environ, "GNUPGHOME": str(gnupg_home)}
        subprocess.run(
            [
                "gpg",
                "--batch",
                "--pinentry-mode",
                "loopback",
                "--passphrase",
                "",
                "--quick-generate-key",
                email,
                "ed25519",
                "sign",
                "never",
            ],
            check=True,
            capture_output=True,
            env=environment,
        )
        keys = subprocess.run(
            [
                "gpg",
                "--batch",
                "--with-colons",
                "--list-secret-keys",
                email,
            ],
            check=True,
            capture_output=True,
            text=True,
            env=environment,
        ).stdout.splitlines()
        fingerprint = next(line.split(":")[9] for line in keys if line.startswith("fpr:"))
        self._git(root, "config", "user.signingkey", fingerprint)
        self._git(root, "config", "gpg.program", "gpg")
        return environment, fingerprint

    def _write_review_keys(
        self, root: Path, author_fingerprint: str, reviewer_fingerprint: str
    ) -> None:
        (root / "mirrorea_canon" / "meta" / "review-keys.json").write_text(
            json.dumps(
                {
                    "format": 1,
                    "author_fingerprints": [author_fingerprint],
                    "reviewer_fingerprints": [reviewer_fingerprint],
                }
            ),
            encoding="utf-8",
        )

    def _working_record_context(self, root: Path) -> dict[str, str]:
        """Create a committed authority/evidence cut for WRK validator tests."""
        (root / "mirrorea_canon" / "adr").mkdir(parents=True, exist_ok=True)
        (root / "mirrorea_canon" / "adr" / "ADR-0014.md").write_text(
            "---\n"
            "id: adr/ADR-0014\n"
            "status: L0-frozen\n"
            "maturity: draft\n"
            "depends_on: []\n"
            "summary: test authority anchor\n"
            "open_items: []\n"
            "---\n\n"
            "# ADR-0014 test anchor\n",
            encoding="utf-8",
        )
        (root / "plan" / "156.md").write_text(
            "# retained LAB evidence\n", encoding="utf-8"
        )
        self._git(root, "init", "-q")
        self._git(root, "config", "user.name", "test reviewer")
        self._git(root, "config", "user.email", "reviewer@example.test")
        self._git(root, "add", ".")
        self._git(root, "commit", "-qm", "test: authority and evidence cut")
        base = self._git(root, "rev-parse", "HEAD")
        canon_blob = hashlib.sha256(
            (root / "mirrorea_canon" / "adr" / "ADR-0014.md").read_bytes()
        ).hexdigest()
        lab_blob = hashlib.sha256((root / "plan" / "156.md").read_bytes()).hexdigest()
        return {
            "base": base,
            "canon_anchor": f"adr/ADR-0014@{base}:{canon_blob}",
            "lab_input": f"LAB:plan/156.md@{base}:{lab_blob}",
        }

    def _reviewed_record_text(
        self,
        context: dict[str, str],
        frozen_base: str,
        author_fingerprint: str,
        reviewer_fingerprint: str,
    ) -> str:
        provisional = self._valid_working_record_text(
            status="L2-working",
            reliance="active",
            canon_anchor=context["canon_anchor"],
            lab_input=context["lab_input"],
            evidence_artifacts=context["lab_input"],
            author_fingerprint=author_fingerprint,
            review=(
                f"reviewer-fingerprint={reviewer_fingerprint}; "
                f"frozen-base={frozen_base}; "
                "record-sha256=" + ("0" * 64) + "; decision=approved"
            ),
        )
        normalized = provisional.replace(
            f"Independent review: reviewer-fingerprint={reviewer_fingerprint}; "
            f"frozen-base={frozen_base}; record-sha256=" + ("0" * 64)
            + "; decision=approved",
            "Independent review: <review-metadata>",
        )
        record_hash = hashlib.sha256(normalized.encode("utf-8")).hexdigest()
        return provisional.replace("record-sha256=" + ("0" * 64), f"record-sha256={record_hash}")

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

    def test_report_template_requires_project_status_update_section(self) -> None:
        heading = "## docs/project-status.md update status"
        template_text = (
            Path(__file__).resolve().parents[2]
            / "docs"
            / "reports"
            / "TEMPLATE.md"
        ).read_text(encoding="utf-8")

        self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)
        self.assertIn(heading, template_text)

    def test_project_status_is_registered_in_both_scaffolds(self) -> None:
        path = "docs/project-status.md"
        self.assertIn(path, validate_docs.REQUIRED)
        self.assertIn(path, check_source_hierarchy.REQUIRED_PATHS["root_docs"])

    def test_project_status_update_status_contract(self) -> None:
        heading = "## docs/project-status.md update status"
        unchanged = "更新不要: project-status update trigger did not change."
        updated = "更新済み: docs/project-status.md was updated in this package."

        self.assertEqual(
            [], validate_docs.project_status_update_status_errors(unchanged, "")
        )
        self.assertEqual(
            [],
            validate_docs.project_status_update_status_errors(
                updated, "- `docs/project-status.md`"
            ),
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors(
                f"{unchanged}\n{updated}", "- `docs/project-status.md`"
            )
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors("TBD", "")
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors("更新済み:", "")
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors(
                unchanged, "- `docs/project-status.md`"
            )
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors(
                updated, "No change to docs/project-status.md."
            )
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors(
                updated, "- `docs/project-status.md.bak`"
            )
        )
        self.assertTrue(
            validate_docs.project_status_update_status_errors(
                updated, "-\n`docs/project-status.md`"
            )
        )
        self.assertIn(heading, validate_docs.REQUIRED_TEMPLATE_HEADINGS)

    def test_project_status_checked_item_requires_one_safe_canon_file(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            text = self._valid_project_status_text().replace(
                "[ ] G0\n[ ] T0",
                "[X] G0 `mirrorea_canon/plan/01-phases.md`\n[ ] T0",
            )

            with mock.patch.object(validate_docs, "ROOT", root):
                self.assertEqual(
                    [], validate_docs.checked_project_status_item_errors(text)
                )
                self.assertTrue(
                    validate_docs.checked_project_status_item_errors(
                        text.replace(
                            "mirrorea_canon/plan/01-phases.md",
                            "mirrorea_canon/../progress.md",
                        )
                    )
                )
                self.assertTrue(
                    validate_docs.checked_project_status_item_errors(
                        text.replace(
                            "mirrorea_canon/plan/01-phases.md", "mirrorea_canon/plan"
                        )
                    )
                )
                self.assertTrue(
                    validate_docs.checked_project_status_item_errors(
                        text.replace(
                            "[X] G0 `mirrorea_canon/plan/01-phases.md`",
                            "[x] G0 `mirrorea_canon/plan/01-phases.md` "
                            "[X] T0 `mirrorea_canon/plan/01-phases.md`",
                        )
                    )
                )

    def test_project_status_source_paths_reject_escape_and_external_symlink(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            status_text = self._valid_project_status_text()
            escaped_path = root / "mirrorea_canon" / "escaped.md"
            escaped_path.symlink_to("/etc/hosts")

            with mock.patch.object(validate_docs, "ROOT", root):
                traversal_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "mirrorea_canon/plan/01-phases.md",
                        "mirrorea_canon/../progress.md",
                    )
                )
                mixed_traversal_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "`mirrorea_canon/plan/01-phases.md`",
                        "`mirrorea_canon/plan/01-phases.md`\n`../progress.md`",
                        1,
                    )
                )
                absolute_path_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "mirrorea_canon/plan/01-phases.md", "/etc/hosts"
                    )
                )
                whitespace_traversal_errors = (
                    validate_docs.project_status_source_path_errors(
                        status_text.replace(
                            "`mirrorea_canon/plan/01-phases.md`",
                            "`mirrorea_canon/plan/01-phases.md`\n` ../progress.md `",
                            1,
                        )
                    )
                )
                whitespace_absolute_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "`mirrorea_canon/plan/01-phases.md`",
                        "`mirrorea_canon/plan/01-phases.md`\n` /etc/hosts `",
                        1,
                    )
                )
                symlink_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "mirrorea_canon/plan/01-phases.md",
                        "mirrorea_canon/escaped.md",
                    )
                )
                directory_errors = validate_docs.project_status_source_path_errors(
                    status_text.replace(
                        "mirrorea_canon/plan/01-phases.md", "mirrorea_canon/plan"
                    )
                )

        self.assertIn("mirrorea_canon/../progress.md", traversal_errors)
        self.assertIn("../progress.md", mixed_traversal_errors)
        self.assertIn("/etc/hosts", absolute_path_errors)
        self.assertIn(" ../progress.md ", whitespace_traversal_errors)
        self.assertIn(" /etc/hosts ", whitespace_absolute_errors)
        self.assertIn("mirrorea_canon/escaped.md", symlink_errors)
        self.assertIn("mirrorea_canon/plan", directory_errors)

    def test_main_rejects_duplicate_project_status_update_heading(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            report_text = (
                self._valid_report_text()
                + "\n\n## docs/project-status.md update status\n\n"
                "更新不要: duplicate declaration."
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                report_text, encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn("Latest report has duplicate required sections", stdout.getvalue())
        self.assertIn("docs/project-status.md update status", stdout.getvalue())

    def test_main_rejects_project_status_missing_stop_source_path(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "project-status.md").write_text(
                self._valid_project_status_text().replace(
                    "plan/153-g0-closeout-evidence-and-exit-decision-packet.md",
                    "plan/does-not-exist.md",
                ),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(), encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Project status report has missing source paths",
            stdout.getvalue(),
        )
        self.assertIn("plan/does-not-exist.md", stdout.getvalue())

    def test_main_allows_project_status_with_future_state_references(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "project-status.md").write_text(
                self._valid_project_status_text()
                .replace("T0/G0 rebaseline", "T1/G1 example")
                .replace(
                    "plan/153-g0-closeout-evidence-and-exit-decision-packet.md",
                    "plan/154-project-control-cockpit.md",
                ),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(), encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())

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
            "plan/151-discord-webhook-secret-validator-guard.md",
            "plan/152-discord-notification-file-inputs.md",
            "plan/153-g0-closeout-evidence-and-exit-decision-packet.md",
            "plan/154-project-control-cockpit.md",
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

    def test_main_rejects_progress_missing_current_position_plan_source(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                self._valid_snapshot_text(
                    validate_docs.PROGRESS_REQUIRED_HEADINGS,
                    "## current milestone position",
                ).replace(
                    "plan/149-current-phase-position-reading.md",
                    "plan/missing-position-reading.md",
                ),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(), encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Snapshot docs are missing current-position source references",
            stdout.getvalue(),
        )
        self.assertIn("progress.md", stdout.getvalue())
        self.assertIn("plan/ source file", stdout.getvalue())

    def test_main_rejects_tasks_missing_current_position_canon_source(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "tasks.md").write_text(
                self._valid_snapshot_text(
                    validate_docs.TASKS_REQUIRED_HEADINGS,
                    "## current promoted package",
                ).replace(
                    "mirrorea_canon/plan/01-phases.md",
                    "mirrorea_canon/plan/missing-phases.md",
                ),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(), encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Snapshot docs are missing current-position source references",
            stdout.getvalue(),
        )
        self.assertIn("tasks.md", stdout.getvalue())
        self.assertIn("mirrorea_canon/ source file", stdout.getvalue())

    def test_main_allows_snapshot_position_with_future_state_sources(self) -> None:
        template_text = self._valid_template_text()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "progress.md").write_text(
                self._valid_snapshot_text(
                    validate_docs.PROGRESS_REQUIRED_HEADINGS,
                    "## current milestone position",
                ).replace("Future lifecycle example", "T2/G7 completed example"),
                encoding="utf-8",
            )
            (root / "tasks.md").write_text(
                self._valid_snapshot_text(
                    validate_docs.TASKS_REQUIRED_HEADINGS,
                    "## current promoted package",
                ).replace("Future lifecycle example", "I6 deployment example"),
                encoding="utf-8",
            )
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text(), encoding="utf-8"
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with redirect_stdout(stdout):
                    exit_code = validate_docs.main()

        self.assertEqual(exit_code, 0)
        self.assertIn("Documentation scaffold looks complete", stdout.getvalue())

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

    def test_required_scaffold_includes_storage_helper_surface(self) -> None:
        required_docs = set(validate_docs.REQUIRED)
        required_hierarchy = {
            path
            for paths in check_source_hierarchy.REQUIRED_PATHS.values()
            for path in paths
        }
        storage_helper_docs = {
            "scripts/env/mirrorea_storage_env.sh",
            "scripts/storage/setup_mirrorea_workdisk_root.sh",
            "scripts/storage/detach_prepare.sh",
            "scripts/storage/cleanup_disposable_artifacts.sh",
            "scripts/storage/tmp_mirrorea_artifacts.sh",
            "scripts/tests/test_storage_workdir_guards.py",
            "scripts/tests/test_tmp_mirrorea_artifacts.py",
        }

        for path in storage_helper_docs:
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

    def test_main_rejects_concrete_discord_webhook_without_printing_secret(self) -> None:
        template_text = self._valid_template_text()
        fake_webhook_url = self._fake_concrete_discord_webhook_url()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            leak_path = root / "docs" / "hands_on" / "leaky.md"
            leak_path.write_text(
                "# leaky\n\n"
                f"webhook = {fake_webhook_url}\n",
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

        output = stdout.getvalue()
        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Tracked files contain concrete Discord webhook URLs",
            output,
        )
        self.assertIn("docs/hands_on/leaky.md:3", output)
        self.assertNotIn(fake_webhook_url, output)

    def test_main_rejects_webhook_before_line_echoing_lints(self) -> None:
        template_text = self._valid_template_text()
        fake_webhook_url = self._fake_concrete_discord_webhook_url()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "hands_on" / "README.md").write_text(
                "# hands_on\n\n"
                f"Normative source remains `specs/00..09`; webhook={fake_webhook_url}\n",
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

        output = stdout.getvalue()
        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Tracked files contain concrete Discord webhook URLs",
            output,
        )
        self.assertIn("docs/hands_on/README.md:3", output)
        self.assertNotIn("Reader-facing docs contain stale source-hierarchy wording", output)
        self.assertNotIn(fake_webhook_url, output)

    def test_main_rejects_untracked_report_webhook_when_git_scan_succeeds(self) -> None:
        template_text = self._valid_template_text()
        fake_webhook_url = self._fake_concrete_discord_webhook_url()

        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, template_text)
            (root / "docs" / "reports" / "0002-latest.md").write_text(
                self._valid_report_text()
                + "\n\nAccidental concrete webhook: "
                + fake_webhook_url
                + "\n",
                encoding="utf-8",
            )

            stdout = io.StringIO()
            with mock.patch.object(validate_docs, "ROOT", root):
                with mock.patch.object(
                    validate_docs,
                    "_tracked_secret_scan_files",
                    return_value=[],
                ):
                    with redirect_stdout(stdout):
                        exit_code = validate_docs.main()

        output = stdout.getvalue()
        self.assertEqual(exit_code, 1)
        self.assertIn(
            "Tracked files contain concrete Discord webhook URLs",
            output,
        )
        self.assertIn("docs/reports/0002-latest.md", output)
        self.assertNotIn(fake_webhook_url, output)

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
                + self._valid_snapshot_text(
                    validate_docs.PROGRESS_REQUIRED_HEADINGS,
                    "## current milestone position",
                )
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

    def _valid_working_record_text(
        self,
        status: str = "L3-open",
        reliance: str = "not-promoted",
        positive_evidence: str | None = None,
        negative_evidence: str | None = None,
        review: str | None = None,
        author: str = "author-agent",
        author_fingerprint: str = "not-required-for-L3",
        canon_anchor: str = (
            "adr/ADR-0014@aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa:"
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"
        ),
        lab_input: str = (
            "LAB:plan/156.md@cccccccccccccccccccccccccccccccccccccccc:"
            "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd"
        ),
        evidence_artifacts: str = "pending",
        evidence_commits: str = "none",
        permitted_lab_locations: str = "plan",
    ) -> str:
        is_l2 = status == "L2-working"
        positive_evidence = positive_evidence or (
            "python3 scripts/validate_docs.py passed"
            if is_l2
            else "pending"
        )
        negative_evidence = negative_evidence or (
            "countermodel rejection passed" if is_l2 else "pending"
        )
        review = review or (
            "reviewer=independent-agent; "
            "frozen-cut=dddddddddddddddddddddddddddddddddddddddd; decision=approved"
            if is_l2
            else "not-required-for-L3"
        )
        return (
            "---\n"
            "id: working/WRK-0001\n"
            f"status: {status}\n"
            f"maturity: {'reviewed' if is_l2 else 'draft'}\n"
            "depends_on: [adr/ADR-0014]\n"
            "summary: test working record\n"
            "open_items: []\n"
            "---\n\n"
            "# WRK-0001 - test\n\n"
            "## Classification and authority cut\n\n"
            "Standing eligibility: pass\n"
            f"Author: {author}\n"
            f"Author fingerprint: {author_fingerprint}\n"
            f"Canon anchors: {canon_anchor}\n"
            f"LAB inputs: {lab_input}\n"
            f"Permitted LAB locations: {permitted_lab_locations}\n"
            "Reserved surfaces: excluded\n\n"
            "## Pre-registered working question\n\n"
            "Question: does the test record remain bounded?\n"
            "Status quo: no pilot result exists.\n"
            "Alternative: the route requires escalation.\n"
            "Expected falsifier: a reserved-surface dependency is required.\n"
            "Rollback / reopen trigger: a reproduced falsifier.\n\n"
            "## Method and evidence plan\n\n"
            "Result class: countermodel\n"
            "Commands: python3 scripts/validate_docs.py\n"
            "Non-claims: no Gate, Phase, proof, or public claim.\n\n"
            "## Results and review\n\n"
            f"Reliance status: {reliance}\n"
            f"Positive evidence: {positive_evidence}\n"
            f"Negative evidence: {negative_evidence}\n"
            f"Evidence artifacts: {evidence_artifacts}\n"
            f"Evidence commits: {evidence_commits}\n"
            "Impact / non-effects: no existing canon text changes.\n"
            f"Independent review: {review}\n\n"
            "## Supersession\n\n"
            "Supersession: none\n"
        )

    def test_working_record_requires_preregistration_sections_and_reliance_status(
        self,
    ) -> None:
        headings = (
            "## Classification and authority cut",
            "## Pre-registered working question",
            "## Method and evidence plan",
            "## Results and review",
            "## Supersession",
        )
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(record_text, encoding="utf-8")
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            self.assertEqual([], checker(root))

            record_path.write_text(
                record_text.replace("Reliance status: not-promoted", "Reliance status: active"),
                encoding="utf-8",
            )
            self.assertTrue(checker(root))

            record_path.write_text(
                record_text.replace(
                    "Reliance status: not-promoted",
                    "Reliance status: not-promoted\nReliance status: bypass",
                ),
                encoding="utf-8",
            )
            self.assertTrue(checker(root))

            record_path.write_text(
                record_text.replace(
                    headings[3] + "\n\nReliance status: not-promoted",
                    headings[3]
                    + "\n\nNo reliance marker here\n\n"
                    + headings[4]
                    + "\n\nReliance status: not-promoted",
                ),
                encoding="utf-8",
            )
            self.assertTrue(checker(root))

    def test_working_record_rejects_renamed_and_reidentified_l3_history(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            working_root = root / "mirrorea_canon" / "working"
            original = working_root / "WRK-0001-test.md"
            original.parent.mkdir(parents=True, exist_ok=True)
            original.write_text(record_text, encoding="utf-8")
            self._git(root, "add", original.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register L3 record")

            replacement = working_root / "WRK-0002-renamed.md"
            original.rename(replacement)
            replacement.write_text(
                record_text.replace("WRK-0001", "WRK-0002"), encoding="utf-8"
            )
            self._git(root, "add", "-A")
            self._git(root, "commit", "-qm", "research: rename L3 record")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("historical WRK identity" in error for error in errors), errors
        )

    def test_working_record_rejects_transient_merged_l3_rename_history(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            working_root = root / "mirrorea_canon" / "working"
            original = working_root / "WRK-0001-test.md"
            original.parent.mkdir(parents=True, exist_ok=True)
            original.write_text(record_text, encoding="utf-8")
            self._git(root, "add", original.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register L3 record")
            main_branch = self._git(root, "branch", "--show-current")

            self._git(root, "checkout", "-qb", "side")
            transient = working_root / "WRK-0002-transient.md"
            original.rename(transient)
            transient.write_text(
                record_text.replace("WRK-0001", "WRK-0002"), encoding="utf-8"
            )
            self._git(root, "add", "-A")
            self._git(root, "commit", "-qm", "research: transient L3 rename")
            transient.rename(original)
            original.write_text(record_text, encoding="utf-8")
            self._git(root, "add", "-A")
            self._git(root, "commit", "-qm", "research: restore L3 path")
            self._git(root, "checkout", "-q", main_branch)
            self._git(root, "merge", "--no-ff", "-qm", "merge transient L3 history", "side")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("historical WRK identity" in error for error in errors), errors
        )

    def test_working_record_rejects_uncommitted_record_deletion(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            record_path.unlink()
            helper_path = root / "scripts" / "out-of-lane-after-delete.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('out of lane')\n", encoding="utf-8")

            errors = validate_docs.working_annex_errors(root, authoritative=True)

        self.assertTrue(any("clean worktree" in error for error in errors), errors)
        self.assertTrue(any("WRK-0001-test.md" in error for error in errors), errors)
        self.assertTrue(any("out-of-lane-after-delete.py" in error for error in errors), errors)

    def test_working_record_rejects_committed_deletion_when_annex_disappears(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            self._git(root, "rm", "-r", "mirrorea_canon/working")
            self._git(root, "commit", "-qm", "research: delete working annex")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("historical WRK identity is absent" in error for error in errors), errors)

    def test_working_record_rejects_transient_malformed_identity_history(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register L3 record")
            main_branch = self._git(root, "branch", "--show-current")

            self._git(root, "checkout", "-qb", "side")
            record_path.write_text(
                record_text.replace("id: working/WRK-0001", "id: meta/temporary"),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: transient malformed identity")
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: restore L3 identity")
            self._git(root, "checkout", "-q", main_branch)
            self._git(root, "merge", "--no-ff", "-qm", "merge malformed identity history", "side")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("historical WRK identity" in error for error in errors), errors)

    def test_working_record_rejects_non_lane_change_since_preregistration(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            helper_path = root / "scripts" / "unregistered-l3-helper.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('out of lane')\n", encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "add", helper_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register with helper")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("outside its declared package" in error for error in errors), errors
        )

    def test_working_record_rejects_uncommitted_non_lane_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            helper_path = root / "scripts" / "uncommitted-l3-helper.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('out of lane')\n", encoding="utf-8")

            errors = validate_docs.working_annex_errors(root, authoritative=True)

        self.assertTrue(
            any("uncommitted-l3-helper.py" in error for error in errors), errors
        )

    def test_working_record_rejects_ignored_uncommitted_non_lane_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            (root / ".gitignore").write_text("scripts/ignored-l3-helper.py\n", encoding="utf-8")
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            helper_path = root / "scripts" / "ignored-l3-helper.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('out of lane')\n", encoding="utf-8")

            errors = validate_docs.working_annex_errors(root, authoritative=True)

        self.assertTrue(
            any("ignored-l3-helper.py" in error for error in errors), errors
        )

    def test_working_record_does_not_attribute_unmanifested_policy_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            readme_path = root / "mirrorea_canon" / "working" / "README.md"
            readme_path.parent.mkdir(parents=True, exist_ok=True)
            readme_path.write_text("# working policy\n", encoding="utf-8")
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            readme_path.write_text("# altered working policy\n", encoding="utf-8")
            self._git(root, "add", readme_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "docs: alter working policy")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_ignores_pre_registration_side_branch_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            main_branch = self._git(root, "branch", "--show-current")
            self._git(root, "checkout", "-qb", "side")
            side_path = root / "scripts" / "pre-registration-side-branch.py"
            side_path.parent.mkdir(parents=True, exist_ok=True)
            side_path.write_text("print('unrelated earlier work')\n", encoding="utf-8")
            self._git(root, "add", side_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: side branch before registration")
            self._git(root, "checkout", "-q", main_branch)

            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            self._git(root, "merge", "--no-ff", "-qm", "merge earlier side branch", "side")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_rejects_non_markdown_control_path_change(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            helper_path = root / "docs" / "reports" / "l3-helper.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('out of lane')\n", encoding="utf-8")
            prefix_path = root / "tasks.md-helper.py"
            prefix_path.write_text("print('out of lane')\n", encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "add", helper_path.relative_to(root).as_posix())
            self._git(root, "add", prefix_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: hide helpers under controls")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("outside its declared package" in error for error in errors), errors
        )
        self.assertTrue(any("docs/reports/l3-helper.py" in error for error in errors), errors)
        self.assertTrue(any("tasks.md-helper.py" in error for error in errors), errors)

    def test_working_record_allows_markdown_control_report(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            report_path = root / "docs" / "reports" / "0002-l3-note.md"
            report_path.write_text("# L3 note\n", encoding="utf-8")
            self._git(root, "add", report_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "docs: record L3 note")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_allows_declared_lane_change_since_preregistration(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")

            lane_path = root / "plan" / "157.md"
            lane_path.write_text("# declared lane experiment\n", encoding="utf-8")
            self._git(root, "add", lane_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: update declared lane")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_allows_product_alpha_computational_lane_and_descendants(self) -> None:
        root = "samples/product-alpha1/computational"
        for allowed in (
            root,
            "samples/product-alpha1/computational/control-flow/positive",
            "samples/product-alpha1/computational/variables-scope/negative",
        ):
            with self.subTest(allowed=allowed):
                self.assertEqual(validate_docs._permitted_lab_locations(allowed), [allowed])

        for rejected in (
            "samples",
            "samples/product-alpha1",
            "samples/product-alpha1/posegraph",
            "scripts",
            "crates/mir-runtime",
            "plan/arbitrary-unregistered-child",
            "samples/clean-near-end/arbitrary-unregistered-child",
            "samples/current-l2/arbitrary-unregistered-child",
            "samples/lean/arbitrary-unregistered-child",
        ):
            with self.subTest(rejected=rejected):
                self.assertIsNone(validate_docs._permitted_lab_locations(rejected))

    def test_working_record_allows_only_direct_numbered_reports_in_report_lane(self) -> None:
        locations = validate_docs._permitted_lab_locations("plan, docs/reports")

        self.assertEqual(locations, ["plan", "docs/reports"])
        self.assertTrue(
            validate_docs._is_permitted_lab_path(
                "docs/reports/2453-working-annex-report-lane-validation.md",
                locations,
            )
        )
        for rejected in (
            "docs/reports/TEMPLATE.md",
            "docs/reports/README.md",
            "docs/reports/helper.py",
            "docs/reports/nested/2453-report.md",
        ):
            with self.subTest(rejected=rejected):
                self.assertFalse(
                    validate_docs._is_permitted_lab_path(rejected, locations)
                )

    def test_working_record_accepts_declared_product_alpha_row_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            row = "samples/product-alpha1/computational/control-flow/positive"
            existing_package = root / row / "package.mir.json"
            existing_package.parent.mkdir(parents=True, exist_ok=True)
            existing_package.write_text("{\"fixture\": \"existing\"}\n", encoding="utf-8")
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"],
                lab_input=context["lab_input"],
                permitted_lab_locations=f"plan, {row}",
            )
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register Product Alpha row")

            artifact_path = root / row / "direct-world" / "package.mir.json"
            artifact_path.parent.mkdir(parents=True, exist_ok=True)
            artifact_path.write_text("{\"package_kind\": \"world\"}\n", encoding="utf-8")
            self._git(root, "add", artifact_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: retain row-local evidence")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            artifact_digest = hashlib.sha256(artifact_path.read_bytes()).hexdigest()
            record_path.write_text(
                record_text.replace(
                    "Evidence artifacts: pending",
                    f"Evidence artifacts: LAB:{row}/direct-world/package.mir.json@{evidence_commit}:{artifact_digest}",
                ).replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest row-local evidence")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_rejects_product_alpha_parent_evidence(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            row = "samples/product-alpha1/computational/control-flow/positive"
            existing_package = root / row / "package.mir.json"
            existing_package.parent.mkdir(parents=True, exist_ok=True)
            existing_package.write_text("{\"fixture\": \"existing\"}\n", encoding="utf-8")
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"],
                lab_input=context["lab_input"],
                permitted_lab_locations=f"plan, {row}",
            )
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register Product Alpha row")

            parent_artifact = root / "samples/product-alpha1/computational/control-flow/package.mir.json"
            parent_artifact.parent.mkdir(parents=True, exist_ok=True)
            parent_artifact.write_text("{\"fixture\": \"out-of-row\"}\n", encoding="utf-8")
            self._git(root, "add", parent_artifact.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: retain parent evidence")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            record_path.write_text(
                record_text.replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest parent evidence")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("control-flow/package.mir.json" in error for error in errors), errors
        )

    def test_product_alpha_row_location_limits_evidence_to_that_row(self) -> None:
        declared = ["samples/product-alpha1/computational/control-flow/positive"]

        self.assertTrue(
            validate_docs._is_permitted_lab_path(
                "samples/product-alpha1/computational/control-flow/positive/direct-world/package.mir.json",
                declared,
            )
        )
        for rejected in (
            "samples/product-alpha1/computational/control-flow/negative/package.mir.json",
            "samples/product-alpha1/computational/variables-scope/negative/direct-world/package.mir.json",
            "samples/product-alpha1/computational/matrix.json",
            "scripts/mir_computational_samples.py",
        ):
            with self.subTest(rejected=rejected):
                self.assertFalse(validate_docs._is_permitted_lab_path(rejected, declared))

    def test_working_record_accepts_manifested_evidence_in_declared_lane(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")

            artifact_path = root / "plan" / "157.md"
            artifact_path.write_text("# declared lane experiment\n", encoding="utf-8")
            self._git(root, "add", artifact_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: run declared experiment")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            artifact_digest = hashlib.sha256(artifact_path.read_bytes()).hexdigest()
            record_path.write_text(
                record_text.replace(
                    "Evidence artifacts: pending",
                    f"Evidence artifacts: LAB:plan/157.md@{evidence_commit}:{artifact_digest}",
                ).replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest evidence")

            errors = validate_docs.working_annex_errors(root)

        self.assertEqual([], errors)

    def test_working_record_rejects_manifested_evidence_outside_declared_lane(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")

            helper_path = root / "scripts" / "evidence-helper.py"
            helper_path.parent.mkdir(parents=True, exist_ok=True)
            helper_path.write_text("print('not a declared lane')\n", encoding="utf-8")
            self._git(root, "add", helper_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: out-of-lane evidence")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            record_path.write_text(
                record_text.replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest invalid evidence")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("evidence-helper.py" in error for error in errors), errors)

    def test_working_record_rejects_evidence_that_predates_registration(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            artifact_path = root / "plan" / "157.md"
            artifact_path.write_text("# too early\n", encoding="utf-8")
            self._git(root, "add", artifact_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: evidence before pre-registration")
            evidence_commit = self._git(root, "rev-parse", "HEAD")

            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"],
                    lab_input=context["lab_input"],
                    evidence_commits=evidence_commit,
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register too-late evidence")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("must follow registration" in error for error in errors), errors)

    def test_working_record_rejects_evidence_manifest_removal(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_text = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register working record")
            artifact_path = root / "plan" / "157.md"
            artifact_path.write_text("# evidence\n", encoding="utf-8")
            self._git(root, "add", artifact_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: run evidence")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            manifested_text = record_text.replace(
                "Evidence commits: none", f"Evidence commits: {evidence_commit}"
            )
            record_path.write_text(manifested_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest evidence")
            record_path.write_text(record_text, encoding="utf-8")
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: remove evidence manifest")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("Evidence commits are not append-only" in error for error in errors), errors)

    def test_working_record_rejects_duplicate_evidence_ownership(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            working_root = root / "mirrorea_canon" / "working"
            working_root.mkdir(parents=True, exist_ok=True)
            record_one = self._valid_working_record_text(
                canon_anchor=context["canon_anchor"], lab_input=context["lab_input"]
            )
            record_one_path = working_root / "WRK-0001-test.md"
            record_one_path.write_text(record_one, encoding="utf-8")
            self._git(root, "add", record_one_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register first record")
            record_two = record_one.replace("WRK-0001", "WRK-0002")
            record_two_path = working_root / "WRK-0002-test.md"
            record_two_path.write_text(record_two, encoding="utf-8")
            self._git(root, "add", record_two_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: pre-register second record")
            artifact_path = root / "plan" / "157.md"
            artifact_path.write_text("# shared evidence\n", encoding="utf-8")
            self._git(root, "add", artifact_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: run shared evidence")
            evidence_commit = self._git(root, "rev-parse", "HEAD")
            record_one_path.write_text(
                record_one.replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_one_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest first ownership")
            record_two_path.write_text(
                record_two.replace("Evidence commits: none", f"Evidence commits: {evidence_commit}"),
                encoding="utf-8",
            )
            self._git(root, "add", record_two_path.relative_to(root).as_posix())
            self._git(root, "commit", "-qm", "research: manifest duplicate ownership")

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(any("claimed by both" in error for error in errors), errors)

    def test_working_record_rejects_empty_required_fields(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text().replace(
                    "Alternative: the route requires escalation.", "Alternative:"
                ),
                encoding="utf-8",
            )
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))

    def test_working_record_rejects_front_matter_identity_mismatch(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text().replace(
                    "id: working/WRK-0001", "id: meta/not-a-wrk"
                ).replace("status: L3-open", "status: L1-fixed", 1).replace(
                    "# WRK-0001 - test\n\n",
                    "# WRK-0001 - test\n\nstatus: L3-open\n\n",
                ),
                encoding="utf-8",
            )
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))

    def test_working_record_reports_missing_sections_without_crashing(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text().replace(
                    "## Results and review\n\n", ""
                ),
                encoding="utf-8",
            )
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))

    def test_working_record_rejects_unrecognized_or_nested_markdown(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            working_root = root / "mirrorea_canon" / "working"
            working_root.mkdir(parents=True, exist_ok=True)
            (working_root / "notes.md").write_text("# unregistered note\n", encoding="utf-8")
            nested = working_root / "archive" / "WRK-0001-test.md"
            nested.parent.mkdir(parents=True, exist_ok=True)
            nested.write_text(self._valid_working_record_text(), encoding="utf-8")
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))

    def test_working_record_rejects_non_markdown_helper_in_working_annex(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            working_root = root / "mirrorea_canon" / "working"
            working_root.mkdir(parents=True, exist_ok=True)
            (working_root / "unregistered-helper.py").write_text(
                "print('not a WRK record')\n", encoding="utf-8"
            )

            errors = validate_docs.working_annex_errors(root)

        self.assertTrue(
            any("working annex permits only README.md or WRK records" in error for error in errors),
            errors,
        )

    def test_l2_working_record_requires_evidence_and_review_identity(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    status="L2-working",
                    reliance="active",
                    positive_evidence="pending",
                    negative_evidence="pending",
                    review="pending",
                ),
                encoding="utf-8",
            )
            checker = getattr(
                validate_docs,
                "working_annex_errors",
                lambda _root: ["working record validator is missing"],
            )
            self.assertTrue(checker(root))

            record_path.write_text(
                self._valid_working_record_text(
                    status="L2-working", reliance="active", review="none"
                ),
                encoding="utf-8",
            )
            self.assertTrue(checker(root))

            record_path.write_text(
                self._valid_working_record_text(
                    status="L2-working",
                    reliance="active",
                    review="reviewer=author-agent; "
                    "frozen-cut=dddddddddddddddddddddddddddddddddddddddd; decision=approved",
                ),
                encoding="utf-8",
            )
            self.assertTrue(checker(root))

    def test_l2_working_record_fail_closes_without_owner_trust_anchor(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            author_environment, author_fingerprint = self._signed_identity(
                root, "author@example.test"
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"],
                    lab_input=context["lab_input"],
                    author_fingerprint=author_fingerprint,
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "research: freeze L3 working material",
                env=author_environment,
            )
            frozen_base = self._git(root, "rev-parse", "HEAD")
            reviewer_environment, reviewer_fingerprint = self._signed_identity(
                root, "reviewer@example.test"
            )
            self._write_review_keys(root, author_fingerprint, reviewer_fingerprint)
            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "review: approve bounded working record",
                env=reviewer_environment,
            )
            with mock.patch.dict(os.environ, reviewer_environment):
                errors = validate_docs.working_annex_errors(root)
            self.assertTrue(
                any("owner-authenticated trust anchor" in error for error in errors),
                errors,
            )

    def test_l2_working_record_rejects_unknown_base_or_changed_review_material(
        self,
    ) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            author_environment, author_fingerprint = self._signed_identity(
                root, "author@example.test"
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"],
                    lab_input=context["lab_input"],
                    author_fingerprint=author_fingerprint,
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "research: freeze L3 working material",
                env=author_environment,
            )
            frozen_base = self._git(root, "rev-parse", "HEAD")
            reviewer_environment, reviewer_fingerprint = self._signed_identity(
                root, "reviewer@example.test"
            )
            self._write_review_keys(root, author_fingerprint, reviewer_fingerprint)
            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "review: approve bounded working record",
                env=reviewer_environment,
            )

            record_path.write_text(
                record_path.read_text(encoding="utf-8").replace(
                    frozen_base, "e" * 40, 1
                ),
                encoding="utf-8",
            )
            with mock.patch.dict(os.environ, reviewer_environment):
                self.assertTrue(validate_docs.working_annex_errors(root))

    def test_l2_working_record_rejects_admission_atop_a_stale_base(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            author_environment, author_fingerprint = self._signed_identity(
                root, "author@example.test"
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"],
                    lab_input=context["lab_input"],
                    author_fingerprint=author_fingerprint,
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "research: freeze L3 working material",
                env=author_environment,
            )
            frozen_base = self._git(root, "rev-parse", "HEAD")
            reviewer_environment, reviewer_fingerprint = self._signed_identity(
                root, "reviewer@example.test"
            )
            self._write_review_keys(root, author_fingerprint, reviewer_fingerprint)
            (root / "intervening.md").write_text("# intervening change\n", encoding="utf-8")
            self._git(root, "add", "intervening.md")
            self._git(
                root,
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-qm",
                "docs: intervening change",
            )
            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "review: stale-base approval",
                env=reviewer_environment,
            )

            with mock.patch.dict(os.environ, reviewer_environment):
                errors = validate_docs.working_annex_errors(root)
            self.assertTrue(
                any("one parent equal to frozen base" in error for error in errors), errors
            )

    def test_l2_working_record_requires_a_prior_l3_at_frozen_base(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            author_environment, author_fingerprint = self._signed_identity(
                root, "author@example.test"
            )
            reviewer_environment, reviewer_fingerprint = self._signed_identity(
                root, "reviewer@example.test"
            )
            self._write_review_keys(root, author_fingerprint, reviewer_fingerprint)
            (root / "unrelated.md").write_text("# signed but unrelated base\n", encoding="utf-8")
            self._git(root, "add", "unrelated.md")
            self._git(root, "config", "user.signingkey", author_fingerprint)
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "research: unrelated signed base",
                env=author_environment,
            )
            frozen_base = self._git(root, "rev-parse", "HEAD")
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(root, "config", "user.signingkey", reviewer_fingerprint)
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "review: direct L2 without L3",
                env=reviewer_environment,
            )

            with mock.patch.dict(os.environ, reviewer_environment):
                errors = validate_docs.working_annex_errors(root)
            self.assertTrue(
                any("must contain the prior L3 record" in error for error in errors),
                errors,
            )

    def test_l2_working_record_rejects_unsigned_admission(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            context = self._working_record_context(root)
            author_environment, author_fingerprint = self._signed_identity(
                root, "author@example.test"
            )
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text(
                    canon_anchor=context["canon_anchor"],
                    lab_input=context["lab_input"],
                    author_fingerprint=author_fingerprint,
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "research: freeze L3 working material",
                env=author_environment,
            )
            frozen_base = self._git(root, "rev-parse", "HEAD")
            reviewer_environment, reviewer_fingerprint = self._signed_identity(
                root, "reviewer@example.test"
            )
            self._write_review_keys(root, author_fingerprint, reviewer_fingerprint)
            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "-c",
                "commit.gpgsign=false",
                "commit",
                "-qm",
                "review: unsigned admission",
            )

            with mock.patch.dict(os.environ, reviewer_environment):
                errors = validate_docs.working_annex_errors(root)
            self.assertTrue(
                any("valid reviewer signature" in error for error in errors), errors
            )

            record_path.write_text(
                self._reviewed_record_text(
                    context, frozen_base, author_fingerprint, reviewer_fingerprint
                ),
                encoding="utf-8",
            )
            record_path.write_text(
                record_path.read_text(encoding="utf-8").replace(
                    "no existing canon text changes.", "different reviewed wording.", 1
                ),
                encoding="utf-8",
            )
            self._git(root, "add", record_path.relative_to(root).as_posix())
            self._git(
                root,
                "commit",
                "-S",
                "-qm",
                "review: alter working record",
                env=reviewer_environment,
            )
            with mock.patch.dict(os.environ, reviewer_environment):
                self.assertTrue(validate_docs.working_annex_errors(root))

    def test_working_record_rejects_duplicate_front_matter_fields(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self._write_required_scaffold(root, self._valid_template_text())
            record_path = root / "mirrorea_canon" / "working" / "WRK-0001-test.md"
            record_path.parent.mkdir(parents=True, exist_ok=True)
            record_path.write_text(
                self._valid_working_record_text().replace(
                    "status: L3-open\n", "status: L1-fixed\nstatus: L3-open\n", 1
                ),
                encoding="utf-8",
            )
            self.assertTrue(validate_docs.working_annex_errors(root))

if __name__ == "__main__":
    unittest.main()
