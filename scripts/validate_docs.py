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
    "samples/full-system-v1/README.md",
    "samples/full-system-v1/computational/README.md",
    "samples/full-system-v1/computational/matrix.json",
    "samples/full-system-v1/computational/add-one-positive/README.md",
    "samples/full-system-v1/computational/add-one-positive/src/add-one.mir",
    "samples/full-system-v1/computational/add-one-positive/expected/parse.json",
    "samples/full-system-v1/computational/host-boundary-positive/README.md",
    "samples/full-system-v1/computational/host-boundary-positive/src/host-boundary-add-one.mir",
    "samples/full-system-v1/computational/host-boundary-positive/expected/parse.json",
    "samples/full-system-v1/computational/malformed-function-negative/README.md",
    "samples/full-system-v1/computational/malformed-function-negative/src/malformed-function.mir",
    "samples/full-system-v1/computational/malformed-function-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-perform-negative/README.md",
    "samples/full-system-v1/computational/malformed-perform-negative/src/malformed-perform.mir",
    "samples/full-system-v1/computational/malformed-perform-negative/expected/parse.json",
    "samples/full-system-v1/computational/unresolved-import-negative/README.md",
    "samples/full-system-v1/computational/unresolved-import-negative/src/unresolved-import.mir",
    "samples/full-system-v1/computational/unresolved-import-negative/expected/parse.json",
    "samples/full-system-v1/computational/missing-type-annotation-negative/README.md",
    "samples/full-system-v1/computational/missing-type-annotation-negative/src/missing-type-annotation.mir",
    "samples/full-system-v1/computational/missing-type-annotation-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-record-negative/README.md",
    "samples/full-system-v1/computational/malformed-record-negative/src/malformed-record.mir",
    "samples/full-system-v1/computational/malformed-record-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-transition-negative/README.md",
    "samples/full-system-v1/computational/malformed-transition-negative/src/malformed-transition.mir",
    "samples/full-system-v1/computational/malformed-transition-negative/expected/parse.json",
    "samples/full-system-v1/computational/malformed-capability-negative/README.md",
    "samples/full-system-v1/computational/malformed-capability-negative/src/malformed-capability.mir",
    "samples/full-system-v1/computational/malformed-capability-negative/expected/parse.json",
    "samples/full-system-v1/computational/contract-clause-position-negative/README.md",
    "samples/full-system-v1/computational/contract-clause-position-negative/src/contract-clause-position.mir",
    "samples/full-system-v1/computational/contract-clause-position-negative/expected/parse.json",
    "samples/full-system-v1/world-core/README.md",
    "samples/full-system-v1/world-core/matrix.json",
    "samples/full-system-v1/world-core/world-bootstrap-positive/README.md",
    "samples/full-system-v1/world-core/world-bootstrap-positive/main/src/world-bootstrap-positive.mir",
    "samples/full-system-v1/world-core/world-bootstrap-positive/expected/manifest.json",
    "samples/full-system-v1/world-core/world-bootstrap-positive/expected/run.json",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/README.md",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/main/src/world-observe-before-bootstrap-negative.mir",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/expected/manifest.json",
    "samples/full-system-v1/world-core/world-observe-before-bootstrap-negative/expected/run.json",
    "samples/full-system-v1/membership-chat/README.md",
    "samples/full-system-v1/membership-chat/matrix.json",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/README.md",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/main/src/chat-room-message-positive.mir",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/expected/manifest.json",
    "samples/full-system-v1/membership-chat/chat-room-message-positive/expected/run.json",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/README.md",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/main/src/chat-stale-membership-negative.mir",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/expected/manifest.json",
    "samples/full-system-v1/membership-chat/chat-stale-membership-negative/expected/run.json",
    "samples/full-system-v1/sugoroku-world/README.md",
    "samples/full-system-v1/sugoroku-world/matrix.json",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/README.md",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/main/src/sugoroku-turn-positive.mir",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/expected/manifest.json",
    "samples/full-system-v1/sugoroku-world/sugoroku-turn-positive/expected/run.json",
    "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/README.md",
    "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/main/src/sugoroku-stale-membership-negative.mir",
    "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/expected/manifest.json",
    "samples/full-system-v1/sugoroku-world/sugoroku-stale-membership-negative/expected/run.json",
    "samples/alpha/README.md",
    "samples/product-alpha1/README.md",
    "samples/product-alpha1/demo/README.md",
    "samples/product-alpha1/demo/package.mir.json",
    "samples/product-alpha1/operational/README.md",
    "samples/product-alpha1/operational/world-core/README.md",
    "samples/product-alpha1/operational/world-core/package.mir.json",
    "samples/product-alpha1/operational/membership-chat/README.md",
    "samples/product-alpha1/operational/membership-chat/package.mir.json",
    "samples/product-alpha1/operational/sugoroku-world/README.md",
    "samples/product-alpha1/operational/sugoroku-world/package.mir.json",
    "samples/product-alpha1/demo/packages/debug-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/auth-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/rate-limit-layer/package.mir.json",
    "samples/product-alpha1/demo/packages/placeholder-object/package.mir.json",
    "samples/product-alpha1/demo/packages/custom-avatar-preview/package.mir.json",
    "samples/product-alpha1/docker/README.md",
    "samples/product-alpha1/docker/docker-compose.product-alpha1.yml",
    "samples/product-alpha1/computational/README.md",
    "samples/product-alpha1/computational/matrix.json",
    "samples/product-alpha1/computational/add-one-pure-mir/README.md",
    "samples/product-alpha1/computational/add-one-pure-mir/add-one-pure-mir.mir",
    "samples/product-alpha1/computational/variables-scope/README.md",
    "samples/product-alpha1/computational/variables-scope/variables-scope.mir",
    "samples/product-alpha1/computational/arrays-bounds/README.md",
    "samples/product-alpha1/computational/arrays-bounds/arrays-bounds.mir",
    "samples/product-alpha1/computational/records-vec3/README.md",
    "samples/product-alpha1/computational/records-vec3/records-vec3.mir",
    "samples/product-alpha1/computational/control-flow/README.md",
    "samples/product-alpha1/computational/control-flow/control-flow.mir",
    "samples/product-alpha1/computational/imports-functions/README.md",
    "samples/product-alpha1/computational/imports-functions/imports-functions.mir",
    "samples/product-alpha1/computational/host-io-internal-transform/README.md",
    "samples/product-alpha1/computational/host-io-internal-transform/host-io-internal-transform.mir",
    "samples/product-alpha1/posegraph/README.md",
    "samples/product-alpha1/posegraph/matrix.json",
    "samples/product-alpha1/posegraph/avatar-head-transform/README.md",
    "samples/product-alpha1/posegraph/avatar-head-transform/avatar-head-transform.mir",
    "samples/product-alpha1/posegraph/anchored-object/README.md",
    "samples/product-alpha1/posegraph/anchored-object/anchored-object.mir",
    "samples/product-alpha1/posegraph/sparkle-fallback-anchor/README.md",
    "samples/product-alpha1/posegraph/sparkle-fallback-anchor/sparkle-fallback-anchor.mir",
    "samples/product-alpha1/posegraph/no-split-frame-positive/README.md",
    "samples/product-alpha1/posegraph/no-split-frame-positive/no-split-frame-positive.mir",
    "samples/product-alpha1/posegraph/split-frame-negative/README.md",
    "samples/product-alpha1/posegraph/split-frame-negative/split-frame-negative.mir",
    "samples/product-alpha1/posegraph/save-load-roundtrip/README.md",
    "samples/product-alpha1/posegraph/save-load-roundtrip/save-load-roundtrip.mir",
    "samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/README.md",
    "samples/product-alpha1/posegraph/stale-anchor-after-membership-advance/stale-anchor-after-membership-advance.mir",
    "samples/product-alpha1/posegraph/anchor-switch-frontier-negative/README.md",
    "samples/product-alpha1/posegraph/anchor-switch-frontier-negative/anchor-switch-frontier-negative.mir",
    "samples/product-alpha1/posegraph/stale-anchor-reacquire-required/README.md",
    "samples/product-alpha1/posegraph/stale-anchor-reacquire-required/stale-anchor-reacquire-required.mir",
    "samples/product-alpha1/projection/README.md",
    "samples/product-alpha1/projection/matrix.json",
    "samples/product-alpha1/projection/server-client-target-manifest/server-client-target-manifest.json",
    "samples/product-alpha1/projection/packet-boundary-schema/packet-boundary-schema.json",
    "samples/product-alpha1/projection/ffi-boundary-schema/ffi-boundary-schema.json",
    "samples/product-alpha1/projection/manifest-provider-compatibility/manifest-provider-compatibility.json",
    "samples/product-alpha1/engine-adapter/README.md",
    "samples/product-alpha1/engine-adapter/matrix.json",
    "samples/product-alpha1/engine-adapter/renderer/renderer.contract.json",
    "samples/product-alpha1/engine-adapter/input-device/input-device.contract.json",
    "samples/product-alpha1/engine-adapter/asset-loader/asset-loader.contract.json",
    "samples/product-alpha1/engine-adapter/physics-spatial-query/physics-spatial-query.contract.json",
    "samples/product-alpha1/engine-adapter/host-runtime-bridge/host-runtime-bridge.contract.json",
    "samples/product-alpha1/engine-adapter/wasm-sandbox/wasm-sandbox.contract.json",
    "samples/product-alpha1/engine-adapter/native-library-bridge/native-library-bridge.contract.json",
    "samples/product-alpha1/engine-adapter/viewer-diagnostic-exporter/viewer-diagnostic-exporter.contract.json",
    "docs/hands_on/README.md",
    "docs/hands_on/product_alpha1_01.md",
    "docs/hands_on/operational_product_sample_01.md",
    "docs/hands_on/mir_computational_core_01.md",
    "docs/hands_on/transform_posegraph_01.md",
    "docs/hands_on/autonomous_execution_01.md",
    "docs/hands_on/full_system_v1_roadmap_01.md",
    "docs/research_abstract/README.md",
    "docs/research_abstract/product_alpha1_01.md",
    "docs/research_abstract/operational_product_sample_01.md",
    "docs/research_abstract/mir_computational_core_01.md",
    "docs/research_abstract/autonomous_execution_01.md",
    "docs/research_abstract/full_system_v1_roadmap_01.md",
    "scripts/textual_mir_samples.py",
    "scripts/mir_computational_samples.py",
    "scripts/posegraph_samples.py",
    "scripts/projection_boundary_samples.py",
    "scripts/engine_adapter_boundary_samples.py",
    "scripts/product_alpha1_release_check.py",
    "scripts/operational_product_samples.py",
    "scripts/tests/test_mir_computational_samples.py",
    "scripts/tests/test_posegraph_samples.py",
    "scripts/tests/test_projection_boundary_samples.py",
    "scripts/tests/test_engine_adapter_boundary_samples.py",
    "scripts/tests/test_product_alpha1_release_check.py",
    "scripts/tests/test_operational_product_samples.py",
    "scripts/tests/test_textual_mir_samples.py",
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
    "plan/45-operational-alpha05-roadmap.md",
    "plan/46-operational-alpha08-roadmap.md",
    "plan/47-operational-alpha09-devtools-roadmap.md",
    "plan/48-theory-freeze-proof-obligations.md",
    "plan/49-host-io-and-session-runtime-roadmap.md",
    "plan/50-product-alpha1-public-boundary-roadmap.md",
    "plan/51-operational-product-sample-roadmap.md",
    "plan/52-portal-spatial-world-roadmap.md",
    "plan/53-mir-computational-core-roadmap.md",
    "plan/54-transform-posegraph-roadmap.md",
    "plan/55-projection-backend-roadmap.md",
    "plan/56-engine-adapter-roadmap.md",
    "plan/57-autonomous-computational-core-master-plan.md",
    "plan/58-full-system-v1-roadmap.md",
    "plan/59-textual-mir-roadmap.md",
    "plan/60-computational-runtime-roadmap.md",
    "plan/61-posegraph-runtime-roadmap.md",
    "plan/62-projection-backend-roadmap.md",
    "plan/63-engine-provider-roadmap.md",
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
    "specs/19-verification-stratification.md",
    "specs/20-cut-save-load-semantics.md",
    "specs/21-auth-layer-algebra.md",
    "specs/22-observability-devtools-semantics.md",
    "specs/23-typed-external-host-boundary.md",
    "specs/24-operational-alpha05-alpha08-readiness.md",
    "specs/25-product-alpha1-public-boundary.md",
    "specs/26-operational-product-sample-suite.md",
    "specs/27-spatial-portal-and-shard-extension-boundary.md",
    "specs/28-mir-computational-core.md",
    "specs/29-transform-posegraph-semantics.md",
    "specs/30-projection-and-backend-boundary.md",
    "specs/31-engine-wasm-ffi-adapter-boundary.md",
    "specs/32-autonomous-execution-and-completion-contract.md",
    "specs/33-full-system-v1-scope.md",
    "specs/34-textual-mir-alpha-grammar.md",
    "specs/35-mir-typed-ir-and-interpreter.md",
    "specs/36-projection-ir-and-boundary-preservation.md",
    "specs/37-posegraph-runtime-semantics.md",
    "specs/38-engine-provider-admission.md",
    ".docs/progress-task-axes.md",
    ".docs/continuous-task-policy.md",
    ".docs/current-l2-source-sample-authoring-policy.md",
    "sub-agent-pro/mirrorea_mir_computational_core_handoff.md",
    "sub-agent-pro/full-system-completion-001/20-progress-tasks-replacement-model.md",
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

PROGRESS_REQUIRED_HEADINGS = [
    "## document role",
    "## project axis",
    "## final ideal",
    "## current milestone position",
    "## milestone map",
    "## line snapshots",
    "### Product Alpha line",
    "### Operational Suite line",
    "### Mir Language line",
    "### PoseGraph line",
    "### Projection/Backend line",
    "### Engine/Provider line",
    "## validation floor",
    "## non-claims",
    "## user decision items vs research-discovery items",
    "## macro phase map",
    "## feature maturity rows",
    "## recent log",
]

TASKS_REQUIRED_HEADINGS = [
    "## document role",
    "## current promoted package",
    "## ordered self-driven packages",
    "## self-driven macro phase reading",
    "## user decision gates",
    "## research discovery items",
    "## maintenance tasks",
    "## non-promoted references",
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


def _heading_positions_for(text: str, headings: list[str]) -> dict[str, int]:
    positions = {}
    for heading in headings:
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


def missing_headings(text: str, headings: list[str]) -> list[str]:
    positions = _heading_positions_for(text, headings)
    return [heading for heading in headings if heading not in positions]


def out_of_order_headings(text: str, headings: list[str]) -> list[str]:
    positions = _heading_positions_for(text, headings)
    if len(positions) != len(headings):
        return []
    ordered_positions = [positions[heading] for heading in headings]
    if ordered_positions == sorted(ordered_positions):
        return []
    return headings


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

    progress_text = (ROOT / "progress.md").read_text(encoding="utf-8")
    missing_progress_sections = missing_headings(
        progress_text, PROGRESS_REQUIRED_HEADINGS
    )
    if missing_progress_sections:
        print("progress.md is missing required snapshot sections:")
        for heading in missing_progress_sections:
            print(" -", heading)
        return 1
    out_of_order_progress_sections = out_of_order_headings(
        progress_text, PROGRESS_REQUIRED_HEADINGS
    )
    if out_of_order_progress_sections:
        print("progress.md has required snapshot sections out of order:")
        for heading in out_of_order_progress_sections:
            print(" -", heading)
        return 1

    tasks_text = (ROOT / "tasks.md").read_text(encoding="utf-8")
    missing_tasks_sections = missing_headings(tasks_text, TASKS_REQUIRED_HEADINGS)
    if missing_tasks_sections:
        print("tasks.md is missing required task-map sections:")
        for heading in missing_tasks_sections:
            print(" -", heading)
        return 1
    out_of_order_tasks_sections = out_of_order_headings(
        tasks_text, TASKS_REQUIRED_HEADINGS
    )
    if out_of_order_tasks_sections:
        print("tasks.md has required task-map sections out of order:")
        for heading in out_of_order_tasks_sections:
            print(" -", heading)
        return 1

    print("Documentation scaffold looks complete.")
    print(f"Found {len(reports)} numbered report(s).")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
