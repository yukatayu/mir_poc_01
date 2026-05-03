#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "README.md",
    "AGENTS.md",
    "Documentation.md",
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
    "samples/not_implemented/README.md",
    "scripts/README.md",
    "plan/00-index.md",
    "plan/01-status-at-a-glance.md",
    "plan/11-roadmap-near-term.md",
    "plan/19-repository-map-and-taxonomy.md",
    "plan/39-type-system-freeze-roadmap.md",
    "plan/40-layer-compatibility-freeze-roadmap.md",
    "plan/41-save-load-checkpoint-roadmap.md",
    "plan/42-runtime-package-avatar-roadmap.md",
    "plan/43-alpha-e2e-roadmap.md",
    "plan/44-practical-alpha1-roadmap.md",
    "specs/00-document-map.md",
    "specs/01-charter-and-decision-levels.md",
    "specs/02-system-overview.md",
    "specs/03-layer-model.md",
    "specs/04-mir-core.md",
    "specs/05-mirrorea-fabric.md",
    "specs/06-prismcascade-positioning.md",
    "specs/07-typed-effects-wiring-platform.md",
    "specs/08-cross-system-relations.md",
    "specs/09-invariants-and-constraints.md",
    "specs/10-open-questions.md",
    "specs/11-roadmap-and-workstreams.md",
    "specs/12-decision-register.md",
    "specs/13-type-system-lifetime-fallback.md",
    "specs/14-contract-subtyping-layer-compatibility.md",
    "specs/15-cut-save-load-checkpoint.md",
    "specs/16-runtime-package-adapter-hotplug.md",
    "specs/17-mirrorea-spaces-alpha-scope.md",
    "specs/18-practical-alpha1-scope.md",
    ".docs/progress-task-axes.md",
    ".docs/continuous-task-policy.md",
    ".docs/current-l2-source-sample-authoring-policy.md",
    "docs/reports/TEMPLATE.md",
]

REQUIRED_TEMPLATE_HEADINGS = [
    "## Objective",
    "## Scope and assumptions",
    "## Start state / dirty state",
    "## Documents consulted",
    "## Actions taken",
    "## Files changed",
    "## Commands run",
    "## Evidence / outputs / test results",
    "## What changed in understanding",
    "## Open questions",
    "## Suggested next prompt",
    "## Plan update status",
    "## Documentation.md update status",
    "## progress.md update status",
    "## tasks.md update status",
    "## samples_progress.md update status",
    "## Reviewer findings and follow-up",
    "## Skipped validations and reasons",
    "## Commit / push status",
    "## Sub-agent session close status",
]

UNRESOLVED_TEMPLATE_PLACEHOLDERS = [
    "更新不要 / 更新済み:",
]


def _heading_match(text: str, heading: str) -> re.Match[str] | None:
    return re.search(rf"^{re.escape(heading)}\s*$", text, re.MULTILINE)


def _heading_positions(text: str) -> dict[str, int]:
    positions = {}
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        match = _heading_match(text, heading)
        if match is not None:
            positions[heading] = match.start()
    return positions


def missing_template_headings(template_text: str) -> list[str]:
    positions = _heading_positions(template_text)
    return [heading for heading in REQUIRED_TEMPLATE_HEADINGS if heading not in positions]


def out_of_order_template_headings(template_text: str) -> list[str]:
    positions = _heading_positions(template_text)
    if len(positions) != len(REQUIRED_TEMPLATE_HEADINGS):
        return []
    ordered_positions = [positions[heading] for heading in REQUIRED_TEMPLATE_HEADINGS]
    if ordered_positions == sorted(ordered_positions):
        return []
    return REQUIRED_TEMPLATE_HEADINGS


def required_section_bodies(report_text: str) -> dict[str, str]:
    matches: list[tuple[str, re.Match[str]]] = []
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        match = _heading_match(report_text, heading)
        if match is not None:
            matches.append((heading, match))

    sorted_matches = sorted(matches, key=lambda item: item[1].start())
    bodies = {}
    for index, (heading, match) in enumerate(sorted_matches):
        next_start = (
            sorted_matches[index + 1][1].start()
            if index + 1 < len(sorted_matches)
            else len(report_text)
        )
        bodies[heading] = report_text[match.end() : next_start].strip()
    return bodies


def empty_required_sections(report_text: str) -> list[str]:
    bodies = required_section_bodies(report_text)
    return [
        heading
        for heading in REQUIRED_TEMPLATE_HEADINGS
        if heading in bodies and not bodies[heading]
    ]


def unresolved_template_placeholder_sections(report_text: str) -> list[str]:
    bodies = required_section_bodies(report_text)
    unresolved = []
    for heading in REQUIRED_TEMPLATE_HEADINGS:
        body = bodies.get(heading, "")
        if any(placeholder in body for placeholder in UNRESOLVED_TEMPLATE_PLACEHOLDERS):
            unresolved.append(heading)
    return unresolved


def main() -> int:
    missing = [p for p in REQUIRED if not (ROOT / p).exists()]
    if missing:
        print("Missing required files:")
        for p in missing:
            print(" -", p)
        return 1

    reports = sorted((ROOT / "docs" / "reports").glob("[0-9][0-9][0-9][0-9]-*.md"))
    if not reports:
        print("No numbered reports found in docs/reports")
        return 1

    template_text = (ROOT / "docs" / "reports" / "TEMPLATE.md").read_text(encoding="utf-8")
    missing_template_sections = missing_template_headings(template_text)
    if missing_template_sections:
        print("Report template is missing required sections:")
        for heading in missing_template_sections:
            print(" -", heading)
        return 1
    out_of_order_template_sections = out_of_order_template_headings(template_text)
    if out_of_order_template_sections:
        print("Report template has required sections out of order:")
        for heading in out_of_order_template_sections:
            print(" -", heading)
        return 1

    latest_report = reports[-1]
    latest_report_text = latest_report.read_text(encoding="utf-8")
    missing_latest_report_sections = missing_template_headings(latest_report_text)
    if missing_latest_report_sections:
        print(f"Latest report is missing required sections: {latest_report.name}")
        for heading in missing_latest_report_sections:
            print(" -", heading)
        return 1
    out_of_order_latest_report_sections = out_of_order_template_headings(latest_report_text)
    if out_of_order_latest_report_sections:
        print(f"Latest report has required sections out of order: {latest_report.name}")
        for heading in out_of_order_latest_report_sections:
            print(" -", heading)
        return 1
    empty_latest_report_sections = empty_required_sections(latest_report_text)
    if empty_latest_report_sections:
        print(f"Latest report has empty required sections: {latest_report.name}")
        for heading in empty_latest_report_sections:
            print(" -", heading)
        return 1
    unresolved_latest_report_sections = unresolved_template_placeholder_sections(
        latest_report_text
    )
    if unresolved_latest_report_sections:
        print(
            f"Latest report has unresolved template placeholders: {latest_report.name}"
        )
        for heading in unresolved_latest_report_sections:
            print(" -", heading)
        return 1

    print("Documentation scaffold looks complete.")
    print(f"Found {len(reports)} numbered report(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
