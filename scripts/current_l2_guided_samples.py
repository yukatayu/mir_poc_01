#!/usr/bin/env python3

from __future__ import annotations

import argparse
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


SCRIPT_DIR = Path(__file__).resolve().parent
REPO_ROOT = SCRIPT_DIR.parent


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
            str(sample.sample_path),
            "--format",
            output_format,
        ]
        for sample in selected
    ]


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

    return run_problem(spec, output_format=args.format, include_all=args.all)


if __name__ == "__main__":
    raise SystemExit(main())
