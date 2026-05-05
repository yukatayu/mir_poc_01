#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parent.parent

REQUIRED_PATHS: dict[str, list[str]] = {
    "root_docs": [
        "AGENTS.md",
        "README.md",
        "Documentation.md",
        "progress.md",
        "tasks.md",
        "samples_progress.md",
    ],
    "specs": [
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
    ],
    "policies": [
        ".docs/progress-task-axes.md",
        ".docs/continuous-task-policy.md",
        ".docs/current-l2-source-sample-authoring-policy.md",
    ],
    "plan": [
        "plan/00-index.md",
        "plan/01-status-at-a-glance.md",
        "plan/11-roadmap-near-term.md",
        "plan/17-research-phases-and-autonomy-gates.md",
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
    ],
    "scripts": [
        "scripts",
        "scripts/README.md",
        "scripts/product_alpha1_release_check.py",
        "scripts/tests/test_product_alpha1_release_check.py",
    ],
    "samples": [
        "samples/README.md",
        "samples/alpha/README.md",
        "samples/product-alpha1/README.md",
        "samples/product-alpha1/demo/README.md",
        "samples/product-alpha1/demo/package.mir.json",
        "samples/product-alpha1/demo/packages/debug-layer/package.mir.json",
        "samples/product-alpha1/demo/packages/auth-layer/package.mir.json",
        "samples/product-alpha1/demo/packages/rate-limit-layer/package.mir.json",
        "samples/product-alpha1/demo/packages/placeholder-object/package.mir.json",
        "samples/product-alpha1/demo/packages/custom-avatar-preview/package.mir.json",
        "samples/product-alpha1/docker/README.md",
        "samples/product-alpha1/docker/docker-compose.product-alpha1.yml",
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
    ],
    "supporting_dirs": [
        "docs/reports",
        "docs/hands_on/README.md",
        "docs/hands_on/product_alpha1_01.md",
        "docs/research_abstract/README.md",
        "docs/research_abstract",
        "docs/research_abstract/product_alpha1_01.md",
        "samples/alpha",
        "samples/product-alpha1",
        "samples/product-alpha1/demo",
        "samples/product-alpha1/docker",
        "samples/practical-alpha1",
        "samples/practical-alpha1/packages",
        "samples/practical-alpha1/source",
        "samples/practical-alpha1/expected",
        "samples/practical-alpha1/docker",
        "samples/clean-near-end",
        "samples/clean-near-end/sugoroku-world",
        "crates/mir-runtime",
        "sub-agent-pro",
        "sub-agent-pro/alpha-0",
        "sub-agent-pro/alpha-1",
    ],
}


def build_status() -> dict[str, object]:
    present: list[str] = []
    missing: list[str] = []
    for paths in REQUIRED_PATHS.values():
        for rel_path in paths:
            if (REPO_ROOT / rel_path).exists():
                present.append(rel_path)
            else:
                missing.append(rel_path)
    return {
        "repo_root": str(REPO_ROOT),
        "required_count": len(present) + len(missing),
        "present_count": len(present),
        "missing_count": len(missing),
        "present": present,
        "missing": missing,
        "status": "ok" if not missing else "missing",
    }


def format_pretty(status: dict[str, object]) -> str:
    lines = [
        "source hierarchy check",
        f"  repo_root: {status['repo_root']}",
        f"  required: {status['required_count']}",
        f"  present: {status['present_count']}",
        f"  missing: {status['missing_count']}",
    ]
    missing = status["missing"]
    if missing:
        lines.append("  missing paths:")
        for rel_path in missing:
            lines.append(f"    - {rel_path}")
    else:
        lines.append("  all required paths present")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--format", choices=["pretty", "json"], default="pretty")
    args = parser.parse_args()

    status = build_status()
    if args.format == "json":
        print(json.dumps(status, indent=2, ensure_ascii=False))
    else:
        print(format_pretty(status))
    return 0 if status["missing_count"] == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
