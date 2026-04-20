#!/usr/bin/env python3

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from collections.abc import Callable, Mapping
from dataclasses import asdict
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent


PROBLEM_BUNDLE_TITLES = {
    "problem1": "Problem 1 theorem-first pilot bundle",
    "problem2": "Problem 2 authoritative-room scenario bundle",
}

PROBLEM_BUNDLE_READINGS = {
    "problem1": (
        "Problem 1 の current first line を、guided sample、residual matrix、Lean sample corpus、"
        "repo-local emitted artifact refs まで一本道で辿るための helper-local bundle。"
    ),
    "problem2": (
        "Problem 2 の current first line / reserve lane / negative static-stop pair を、guided sample、"
        "residual matrix、authoritative-room helper summary、Lean artifact まで一本道で辿るための helper-local bundle。"
    ),
}

PROBLEM_BUNDLE_DOC_REFS = {
    "problem1": (
        "specs/examples/466-current-l2-problem1-actual-adoption-package-and-theorem-first-pilot.md",
        "specs/examples/508-current-l2-theorem-lean-first-nonproduction-stub-pilot-actualization.md",
        "specs/examples/509-current-l2-theorem-review-unit-to-lean-stub-repo-local-artifact-conformance-bridge.md",
        "specs/examples/568-current-l2-theorem-model-check-bridge-carrier-reconnect-after-finite-index-widening.md",
        "specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md",
        "specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md",
        "docs/reports/0851-package94-theorem-model-check-bridge-carrier-reconnect.md",
        "docs/reports/0855-package98-guided-problem-sample-entrypoints-and-runner.md",
        "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
    ),
    "problem2": (
        "specs/examples/467-current-l2-problem2-actual-adoption-package-and-authoritative-room-default-profile.md",
        "specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md",
        "specs/examples/570-current-l2-authoritative-room-first-scenario-helper-summary-tightening.md",
        "specs/examples/571-current-l2-authoritative-room-reserve-strengthening-lane-tightening.md",
        "specs/examples/572-current-l2-guided-problem-sample-entrypoints-and-runner.md",
        "specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md",
        "docs/reports/0853-package96-authoritative-room-first-scenario-tightening.md",
        "docs/reports/0854-package97-authoritative-room-reserve-strengthening-lane-tightening.md",
        "docs/reports/0855-package98-guided-problem-sample-entrypoints-and-runner.md",
        "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
    ),
}

PROBLEM_BUNDLE_STOP_LINES = {
    "problem1": (
        "final public theorem contract",
        "concrete theorem prover brand",
        "final public verifier contract",
    ),
    "problem2": (
        "final public witness schema",
        "final public provider receipt schema",
        "final public witness/provider/artifact contract",
        "exhaustive shared-space catalog",
    ),
}


@dataclass(frozen=True)
class GuidedSample:
    sample_id: str
    sample_path: Path
    summary: str
    notes: str
    primary: bool


@dataclass(frozen=True)
class ProblemSpec:
    problem_id: str
    title: str
    summary: str
    focus_points: tuple[str, ...]
    samples: tuple[GuidedSample, ...]


@dataclass(frozen=True)
class Problem1ResidualRow:
    sample_id: str
    primary: bool
    typed_hint: str
    theorem_preview: str
    theorem_reopen: str
    model_check_preview: str
    model_check_reopen: str
    residual_reading: str


@dataclass(frozen=True)
class Problem2ResidualRow:
    sample_id: str
    primary: bool
    static_gate: str
    surface: str
    first_line: str
    public_seam: str
    reserve_lane: str
    reserve_detail: str
    residual_reading: str


def problem_specs() -> dict[str, ProblemSpec]:
    typed_root = REPO_ROOT / "samples" / "prototype" / "current-l2-typed-proof-model-check"
    order_root = REPO_ROOT / "samples" / "prototype" / "current-l2-order-handoff"

    return {
        "problem1": ProblemSpec(
            problem_id="problem1",
            title="Problem 1: typed / theorem / model-check",
            summary=(
                "checker-adjacent first layer、notebook-first theorem line、"
                "row-local model-check carrier first の current cut を追うための入口。"
            ),
            focus_points=(
                "verification_preview",
                "artifact_preview",
                "typed_checker_hint_preview",
            ),
            samples=(
                GuidedSample(
                    sample_id="p06-typed-proof-owner-handoff",
                    sample_path=typed_root / "p06-typed-proof-owner-handoff.txt",
                    summary="typed marker / theorem / model-check bridge の入口。",
                    notes=(
                        "Problem 1 の代表 sample。typed marker family、proof notebook review unit、"
                        "model-check concrete carrier を 1 本で追える。"
                    ),
                    primary=True,
                ),
                GuidedSample(
                    sample_id="p10-typed-authorized-fingerprint-declassification",
                    sample_path=typed_root / "p10-typed-authorized-fingerprint-declassification.txt",
                    summary="authority 付き declassification success。",
                    notes="IFC success 側の current floor を補助的に見る sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p11-typed-unauthorized-fingerprint-release",
                    sample_path=typed_root / "p11-typed-unauthorized-fingerprint-release.txt",
                    summary="authority 欠如で止まる IFC negative。",
                    notes="declassification authority が足りないときの stop を見る sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p12-typed-classified-fingerprint-publication-block",
                    sample_path=typed_root / "p12-typed-classified-fingerprint-publication-block.txt",
                    summary="label-flow mismatch による publication block。",
                    notes="authority 欠如ではなく label-flow mismatch で止まる sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p15-typed-capture-escape-rejected",
                    sample_path=typed_root / "p15-typed-capture-escape-rejected.txt",
                    summary="capture / lifetime negative。",
                    notes="finite-index first layer の capture escape rejection を見る sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p16-typed-remote-call-budget-exceeded",
                    sample_path=typed_root / "p16-typed-remote-call-budget-exceeded.txt",
                    summary="simple cost bound negative。",
                    notes="zero budget 以後の remote call rejection を見る sample。",
                    primary=False,
                ),
            ),
        ),
        "problem2": ProblemSpec(
            problem_id="problem2",
            title="Problem 2: order / handoff / shared-space",
            summary=(
                "authoritative-room first line、reserve strengthening line、"
                "negative static-stop pair を current helper summary で追うための入口。"
            ),
            focus_points=(
                "surface_preview",
                "authoritative_room_first_scenario_actual_adoption",
                "authoritative_room_reserve_strengthening_lane",
            ),
            samples=(
                GuidedSample(
                    sample_id="p07-dice-late-join-visible-history",
                    sample_path=order_root / "p07-dice-late-join-visible-history.txt",
                    summary="late join visible history の representative sample。",
                    notes="first completion line の代表 1 本目。publication / witness / history visibility を見る。",
                    primary=True,
                ),
                GuidedSample(
                    sample_id="p08-dice-stale-reconnect-refresh",
                    sample_path=order_root / "p08-dice-stale-reconnect-refresh.txt",
                    summary="stale reconnect fail-then-refresh の representative sample。",
                    notes="first completion line の代表 2 本目。reconnect handling の current default を見る。",
                    primary=True,
                ),
                GuidedSample(
                    sample_id="p09-dice-delegated-rng-provider-placement",
                    sample_path=order_root / "p09-dice-delegated-rng-provider-placement.txt",
                    summary="delegated RNG practical reserve route。",
                    notes="reserve strengthening lane 側で provider placement を追う sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p13-dice-late-join-missing-publication-witness",
                    sample_path=order_root / "p13-dice-late-join-missing-publication-witness.txt",
                    summary="publication witness 欠如 negative。",
                    notes="late-join visibility line の negative static-stop pair を見る sample。",
                    primary=False,
                ),
                GuidedSample(
                    sample_id="p14-dice-late-join-handoff-before-publication",
                    sample_path=order_root / "p14-dice-late-join-handoff-before-publication.txt",
                    summary="handoff-before-publication negative。",
                    notes="order violation による static stop を見る sample。",
                    primary=False,
                ),
            ),
        ),
    }


def build_run_commands(
    spec: ProblemSpec,
    *,
    output_format: str,
    include_all: bool,
) -> list[list[str]]:
    selected = spec.samples if include_all else tuple(sample for sample in spec.samples if sample.primary)
    return [
        [
            "cargo",
            "run",
            "-q",
            "-p",
            "mir-runtime",
            "--example",
            "mir_current_l2",
            "--",
            "run-source-sample",
            relative_path(sample.sample_path),
            "--format",
            output_format,
        ]
        for sample in selected
    ]


def build_single_run_command(sample: GuidedSample, *, output_format: str) -> list[str]:
    return [
        "cargo",
        "run",
        "-q",
        "-p",
        "mir-runtime",
        "--example",
        "mir_current_l2",
        "--",
        "run-source-sample",
        relative_path(sample.sample_path),
        "--format",
        output_format,
    ]


def relative_path(path: Path) -> str:
    return str(path.relative_to(REPO_ROOT))


def lean_artifact_paths(sample: GuidedSample) -> tuple[str, ...]:
    lean_dir = REPO_ROOT / "samples" / "lean" / "current-l2" / sample.sample_id
    candidates = (
        lean_dir / "README.md",
        lean_dir / f"{sample.sample_id}.lean",
        lean_dir / f"{sample.sample_id}.bundle.json",
    )
    return tuple(relative_path(path) for path in candidates if path.exists())


def bundle_commands(spec: ProblemSpec) -> tuple[str, ...]:
    primary = next(sample for sample in spec.samples if sample.primary)
    return (
        f"python3 scripts/current_l2_guided_samples.py show {spec.problem_id}",
        " ".join(build_single_run_command(primary, output_format="pretty")),
        f"python3 scripts/current_l2_guided_samples.py matrix {spec.problem_id}",
        f"python3 scripts/current_l2_guided_samples.py run {spec.problem_id} --all --format json",
    )


def build_problem_bundle_manifest(spec: ProblemSpec) -> dict[str, object]:
    primary_samples = [sample for sample in spec.samples if sample.primary]
    support_samples = [sample for sample in spec.samples if not sample.primary]

    return {
        "problem_id": spec.problem_id,
        "title": PROBLEM_BUNDLE_TITLES[spec.problem_id],
        "current_reading": PROBLEM_BUNDLE_READINGS[spec.problem_id],
        "summary": spec.summary,
        "commands": bundle_commands(spec),
        "primary_samples": [
            {
                "sample_id": sample.sample_id,
                "prototype_path": relative_path(sample.sample_path),
                "summary": sample.summary,
                "lean_artifacts": lean_artifact_paths(sample),
            }
            for sample in primary_samples
        ],
        "support_samples": [
            {
                "sample_id": sample.sample_id,
                "prototype_path": relative_path(sample.sample_path),
                "summary": sample.summary,
                "lean_artifacts": lean_artifact_paths(sample),
            }
            for sample in support_samples
        ],
        "doc_refs": PROBLEM_BUNDLE_DOC_REFS[spec.problem_id],
        "stop_line": PROBLEM_BUNDLE_STOP_LINES[spec.problem_id],
    }


def render_problem_bundle(spec: ProblemSpec) -> str:
    manifest = build_problem_bundle_manifest(spec)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
        str(manifest["summary"]),
        "",
        "おすすめの追い方:",
    ]

    for index, command in enumerate(manifest["commands"], start=1):
        lines.append(f"{index}. {command}")

    lines.extend(["", "代表サンプルと Lean artifact:"])
    for sample in manifest["primary_samples"]:
        lines.append(f"- {sample['sample_id']}: {sample['summary']}")
        lines.append(f"  prototype: {sample['prototype_path']}")
        for artifact in sample["lean_artifacts"]:
            lines.append(f"  lean artifact: {artifact}")

    lines.extend(["", "補助サンプル:"])
    for sample in manifest["support_samples"]:
        lines.append(f"- {sample['sample_id']}: {sample['summary']}")
        lines.append(f"  prototype: {sample['prototype_path']}")
        for artifact in sample["lean_artifacts"]:
            lines.append(f"  lean artifact: {artifact}")

    lines.extend(["", "anchor docs / reports:"])
    for doc_ref in manifest["doc_refs"]:
        lines.append(f"- {doc_ref}")

    lines.extend(["", "stop line:"])
    for item in manifest["stop_line"]:
        lines.append(f"- {item}")

    lines.extend(
        [
            "",
            "注意:",
            "- これは repo-local / helper-local bundle であり、final public contract や final public grammar を意味しない。",
            "- representative sample から docs / reports / Lean artifact を一本道で辿るための案内に留める。",
        ]
    )
    return "\n".join(lines)


def render_problem_guide(spec: ProblemSpec) -> str:
    lines = [
        spec.title,
        "",
        spec.summary,
        "",
        "見るポイント:",
    ]
    for focus in spec.focus_points:
        lines.append(f"- {focus}")

    lines.append("")
    lines.append("サンプル:")
    for sample in spec.samples:
        prefix = "primary" if sample.primary else "support"
        lines.append(f"- [{prefix}] {sample.sample_id}: {sample.summary}")
        lines.append(f"  {sample.notes}")

    lines.append("")
    lines.append("primary 実行コマンド:")
    for command in build_run_commands(spec, output_format="pretty", include_all=False):
        lines.append(f"- {' '.join(command)}")

    lines.append("")
    lines.append(
        "注意: これは helper-local / non-production の guided sample であり、final public contract や final grammar を意味しない。"
    )
    return "\n".join(lines)


def run_problem(spec: ProblemSpec, *, output_format: str, include_all: bool) -> int:
    commands = build_run_commands(spec, output_format=output_format, include_all=include_all)

    for sample, command in zip(
        spec.samples if include_all else tuple(item for item in spec.samples if item.primary),
        commands,
        strict=True,
    ):
        print(f"== {sample.sample_id} ==", flush=True)
        print(sample.summary, flush=True)
        print(f"$ {' '.join(command)}", flush=True)
        completed = subprocess.run(command, cwd=REPO_ROOT, check=False)
        if completed.returncode != 0:
            return completed.returncode
        print("", flush=True)

    return 0


def load_sample_report(sample: GuidedSample) -> dict:
    completed = subprocess.run(
        build_single_run_command(sample, output_format="json"),
        cwd=REPO_ROOT,
        capture_output=True,
        text=True,
        check=False,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"sample `{sample.sample_id}` failed with exit code {completed.returncode}:\n{completed.stderr}"
        )
    return json.loads(completed.stdout)


def helper_status(report: Mapping[str, object], key: str) -> str:
    preview = report.get(key)
    if not isinstance(preview, Mapping):
        return "missing"

    status = preview.get("status")
    if status == "reached":
        return "reached"
    if status == "guarded_not_reached":
        bridge_floor_refs = preview.get("bridge_floor_refs")
        if isinstance(bridge_floor_refs, list) and bridge_floor_refs:
            return f"bridge-only({len(bridge_floor_refs)})"
        return "guarded"
    if isinstance(status, str):
        return status
    return "missing"


def helper_plain_status(report: Mapping[str, object], key: str) -> str:
    preview = report.get(key)
    if not isinstance(preview, Mapping):
        return "missing"

    status = preview.get("status")
    if isinstance(status, str):
        if status == "guarded_not_reached":
            return "guarded"
        return status
    return "missing"


def problem1_residual_reading(report: Mapping[str, object]) -> str:
    theorem_preview = helper_status(report, "theorem_result_object_preview")
    theorem_reopen = helper_status(report, "theorem_final_public_contract_reopen_threshold")
    model_preview = helper_status(report, "model_check_public_checker_preview")
    model_reopen = helper_status(report, "model_check_final_public_contract_reopen_threshold")
    typed_hint = helper_status(report, "typed_checker_hint_preview")
    verification_preview = report.get("verification_preview")
    formal_hook_status = None
    if isinstance(verification_preview, Mapping):
        formal_hook_status = verification_preview.get("formal_hook_status")

    if (
        theorem_preview == "reached"
        and theorem_reopen == "reached"
        and model_preview == "reached"
        and model_reopen == "reached"
    ):
        return "public-seam representative"
    if typed_hint == "reached" and formal_hook_status == "reached":
        return "checker-adjacent bridge-floor"
    if formal_hook_status == "reached":
        return "formal-hook bridge-floor"
    return "guarded"


def build_problem1_residual_rows(
    spec: ProblemSpec,
    *,
    loader: Callable[[GuidedSample], Mapping[str, object]] = load_sample_report,
) -> list[Problem1ResidualRow]:
    rows: list[Problem1ResidualRow] = []
    for sample in spec.samples:
        report = loader(sample)
        rows.append(
            Problem1ResidualRow(
                sample_id=sample.sample_id,
                primary=sample.primary,
                typed_hint=helper_status(report, "typed_checker_hint_preview"),
                theorem_preview=helper_status(report, "theorem_result_object_preview"),
                theorem_reopen=helper_status(
                    report, "theorem_final_public_contract_reopen_threshold"
                ),
                model_check_preview=helper_status(report, "model_check_public_checker_preview"),
                model_check_reopen=helper_status(
                    report, "model_check_final_public_contract_reopen_threshold"
                ),
                residual_reading=problem1_residual_reading(report),
            )
        )
    return rows


def checker_static_gate_verdict(report: Mapping[str, object]) -> str:
    checker_floor = report.get("checker_floor")
    if not isinstance(checker_floor, Mapping):
        return "missing"
    static_gate = checker_floor.get("static_gate")
    if not isinstance(static_gate, Mapping):
        return "missing"
    verdict = static_gate.get("verdict")
    if isinstance(verdict, str):
        return verdict
    return "missing"


def problem2_reserve_detail(report: Mapping[str, object]) -> str:
    lane = report.get("authoritative_room_reserve_strengthening_lane")
    if not isinstance(lane, Mapping):
        return "missing"

    parts: list[str] = []
    if lane.get("witness_strengthening_status") == "reached":
        parts.append("witness")
    if lane.get("delegated_rng_service_status") == "reached":
        parts.append("delegated-rng")
    if lane.get("model_check_second_line_status") == "reached":
        parts.append("model-check")
    if parts:
        return "+".join(parts)
    if lane.get("status") == "guarded_not_reached":
        return "guarded"
    return "missing"


def problem2_residual_reading(report: Mapping[str, object]) -> str:
    static_gate = checker_static_gate_verdict(report)
    first_line = helper_plain_status(report, "authoritative_room_first_scenario_actual_adoption")
    reserve_lane = helper_plain_status(report, "authoritative_room_reserve_strengthening_lane")
    reserve_detail = problem2_reserve_detail(report)

    if static_gate in {"underdeclared", "malformed"}:
        return "negative static-stop"
    if first_line == "reached":
        return "first-line representative"
    if reserve_lane == "reached" and reserve_detail == "delegated-rng+model-check":
        return "reserve practical route"
    if reserve_lane == "reached":
        return "reserve strengthening route"
    return "guarded"


def build_problem2_residual_rows(
    spec: ProblemSpec,
    *,
    loader: Callable[[GuidedSample], Mapping[str, object]] = load_sample_report,
) -> list[Problem2ResidualRow]:
    rows: list[Problem2ResidualRow] = []
    for sample in spec.samples:
        report = loader(sample)
        rows.append(
            Problem2ResidualRow(
                sample_id=sample.sample_id,
                primary=sample.primary,
                static_gate=checker_static_gate_verdict(report),
                surface=helper_plain_status(
                    report, "order_handoff_source_surface_artifact_actual_adoption"
                ),
                first_line=helper_plain_status(
                    report, "authoritative_room_first_scenario_actual_adoption"
                ),
                public_seam=helper_plain_status(
                    report, "order_handoff_witness_provider_public_seam_compression"
                ),
                reserve_lane=helper_plain_status(
                    report, "authoritative_room_reserve_strengthening_lane"
                ),
                reserve_detail=problem2_reserve_detail(report),
                residual_reading=problem2_residual_reading(report),
            )
        )
    return rows


def render_problem1_residual_matrix(
    spec: ProblemSpec,
    rows: list[Problem1ResidualRow],
) -> str:
    headers = [
        "sample",
        "typed hint",
        "theorem preview",
        "theorem reopen",
        "model-check preview",
        "model-check reopen",
        "reading",
    ]
    table_rows = [
        [
            row.sample_id + (" [primary]" if row.primary else ""),
            row.typed_hint,
            row.theorem_preview,
            row.theorem_reopen,
            row.model_check_preview,
            row.model_check_reopen,
            row.residual_reading,
        ]
        for row in rows
    ]

    widths = [len(header) for header in headers]
    for row in table_rows:
        for index, cell in enumerate(row):
            widths[index] = max(widths[index], len(cell))

    def render_row(cells: list[str]) -> str:
        return " | ".join(
            cell.ljust(widths[index]) for index, cell in enumerate(cells)
        )

    lines = [
        "Problem 1 residual bundle",
        "",
        spec.summary,
        "",
        render_row(headers),
        render_row(["-" * width for width in widths]),
    ]
    lines.extend(render_row(row) for row in table_rows)
    lines.extend(
        [
            "",
            "読み方:",
            "- `reached` は representative public-seam preview / reopen threshold がその sample で actualize していることを示す。",
            "- `bridge-only(n)` は final public seam 自体は guarded のまま、bridge floor が `n` 個の ref で sample-visible になっていることを示す。",
            "- `typed hint` は checker-adjacent first layer の到達度であり、theorem/model-check public seam そのものではない。",
            "",
            "current recommendation:",
            "- `p06` を Problem 1 public-seam representative とし、`p10 / p11 / p12 / p15 / p16` は checker-adjacent / Lean-first theorem bridge / row-local model-check bridge の residual bundle として読む。",
            "- stronger typed surface、final public theorem result object、final public checker artifact、final public verifier contract はこの matrix でも still later に留める。",
        ]
    )
    return "\n".join(lines)


def render_problem2_residual_matrix(
    spec: ProblemSpec,
    rows: list[Problem2ResidualRow],
) -> str:
    headers = [
        "sample",
        "static gate",
        "surface",
        "first line",
        "public seam",
        "reserve lane",
        "reserve detail",
        "reading",
    ]
    table_rows = [
        [
            row.sample_id + (" [primary]" if row.primary else ""),
            row.static_gate,
            row.surface,
            row.first_line,
            row.public_seam,
            row.reserve_lane,
            row.reserve_detail,
            row.residual_reading,
        ]
        for row in rows
    ]

    widths = [len(header) for header in headers]
    for row in table_rows:
        for index, cell in enumerate(row):
            widths[index] = max(widths[index], len(cell))

    def render_row(cells: list[str]) -> str:
        return " | ".join(
            cell.ljust(widths[index]) for index, cell in enumerate(cells)
        )

    lines = [
        "Problem 2 residual bundle",
        "",
        spec.summary,
        "",
        render_row(headers),
        render_row(["-" * width for width in widths]),
    ]
    lines.extend(render_row(row) for row in table_rows)
    lines.extend(
        [
            "",
            "読み方:",
            "- `first line = reached` は authoritative-room first completion pair (`p07 / p08`) の current default がその sample で actualize していることを示す。",
            "- `reserve lane = reached` は witness strengthening / delegated RNG / model-check second line の reserve package がその sample で見えていることを示す。",
            "- `public seam` は final public witness/provider/artifact contract ではなく、helper-local residual compression の到達度だけを示す。",
            "- `static gate` が `underdeclared` または `malformed` の行は negative static-stop pair として読む。",
            "",
            "current recommendation:",
            "- `p07 / p08` を first-line representative とし、`p09` は delegated RNG practical reserve route、`p13 / p14` は negative static-stop pair として読む。",
            "- final public witness schema、final public provider receipt schema、final emitted-handoff contract、exhaustive shared-space catalog はこの matrix でも still later に留める。",
        ]
    )
    return "\n".join(lines)


def render_problem1_residual_matrix_from_runtime(
    spec: ProblemSpec,
    *,
    output_format: str,
) -> str:
    rows = build_problem1_residual_rows(spec)
    if output_format == "json":
        return json.dumps([asdict(row) for row in rows], ensure_ascii=False, indent=2)
    return render_problem1_residual_matrix(spec, rows)


def render_problem2_residual_matrix_from_runtime(
    spec: ProblemSpec,
    *,
    output_format: str,
) -> str:
    rows = build_problem2_residual_rows(spec)
    if output_format == "json":
        return json.dumps([asdict(row) for row in rows], ensure_ascii=False, indent=2)
    return render_problem2_residual_matrix(spec, rows)


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Problem 1 / Problem 2 の guided sample を repo-local command と一緒に案内する helper。"
    )
    subparsers = parser.add_subparsers(dest="subcommand", required=True)

    subparsers.add_parser("list", help="利用できる guided problem id を一覧する")

    show_parser = subparsers.add_parser("show", help="guided explanation を表示する")
    show_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))

    run_parser = subparsers.add_parser("run", help="guided sample を実行する")
    run_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))
    run_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")
    run_parser.add_argument(
        "--all",
        action="store_true",
        help="primary sample だけでなく reserve / negative sample まで流す",
    )

    matrix_parser = subparsers.add_parser(
        "matrix",
        help="guided problem の residual bundle を sample 単位で表示する",
    )
    matrix_parser.add_argument("problem_id", choices=("problem1", "problem2"))
    matrix_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    bundle_parser = subparsers.add_parser(
        "bundle",
        help="guided problem を docs / Lean artifact / residual matrix まで一本道で案内する",
    )
    bundle_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))
    bundle_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])
    specs = problem_specs()

    if args.subcommand == "list":
        for problem_id, spec in specs.items():
            print(f"{problem_id}: {spec.summary}")
        return 0

    spec = specs[args.problem_id]
    if args.subcommand == "show":
        print(render_problem_guide(spec))
        return 0

    if args.subcommand == "matrix":
        try:
            if args.problem_id == "problem1":
                print(render_problem1_residual_matrix_from_runtime(spec, output_format=args.format))
                return 0
            print(render_problem2_residual_matrix_from_runtime(spec, output_format=args.format))
            return 0
        except RuntimeError as error:
            print(str(error), file=sys.stderr)
            return 1

    if args.subcommand == "bundle":
        manifest = build_problem_bundle_manifest(spec)
        if args.format == "json":
            print(json.dumps(manifest, ensure_ascii=False, indent=2))
            return 0
        print(render_problem_bundle(spec))
        return 0

    return run_problem(spec, output_format=args.format, include_all=args.all)


if __name__ == "__main__":
    raise SystemExit(main())
