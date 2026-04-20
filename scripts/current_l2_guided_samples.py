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
FAILURE_EXCERPT_LIMIT = 240


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

PROBLEM_SAMPLE_BUNDLE_DOCS = {
    "problem1": "samples/problem-bundles/problem1-typed-theorem-model-check.md",
    "problem2": "samples/problem-bundles/problem2-order-handoff-shared-space.md",
}

PROBLEM1_THEOREM_EMIT_SAMPLE_IDS = (
    "p06-typed-proof-owner-handoff",
    "p07-dice-late-join-visible-history",
    "p08-dice-stale-reconnect-refresh",
)

PROBLEM2_SCENARIO_EMIT_SAMPLE_IDS = (
    "p07-dice-late-join-visible-history",
    "p08-dice-stale-reconnect-refresh",
    "p09-dice-delegated-rng-provider-placement",
    "p13-dice-late-join-missing-publication-witness",
    "p14-dice-late-join-handoff-before-publication",
)

PROBLEM1_THEOREM_EMIT_STOP_LINE = (
    "final public theorem contract",
    "concrete theorem prover brand",
    "final public verifier contract",
)

PARSER_COMPANION_MAPPING_BUNDLE_ANCHORS = {
    "problem1": "specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md",
    "problem2": "specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md",
}

PARSER_COMPANION_MAPPING_SHARED_ANCHORS = (
    "specs/examples/577-current-l2-parser-side-companion-surface-bundle-actualization.md",
    "specs/examples/578-current-l2-parser-side-bundle-to-helper-bridge-actualization.md",
    "specs/examples/579-current-l2-parser-side-request-head-clause-bundle-inspector-actualization.md",
    "docs/reports/0857-package101-102-problem-bundles.md",
    "docs/reports/0858-package103-104-parser-side-companion-and-bridge.md",
    "docs/reports/0859-package105-parser-companion-inspector.md",
)

PARSER_COMPANION_MAPPING_STOP_LINE = (
    "exhaustive sample catalog",
    "final public tutorial surface",
    "final public parser / checker / runtime API",
)


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


@dataclass(frozen=True)
class ProblemSmokeStep:
    label: str
    command: list[str]


@dataclass(frozen=True)
class ProblemSmokeAggregateRow:
    problem_id: str
    title: str
    status: str
    step_count: int
    successful_steps: int
    failed_step: str | None
    smoke_command: str
    sample_bundle_doc: str
    primary_samples: list[str]
    step_labels: list[str]
    failed_command: str | None = None
    failed_return_code: int | None = None
    failed_output_excerpt: str | None = None


@dataclass(frozen=True)
class ProblemQuickstartStep:
    title: str
    command: str
    expected_results: tuple[str, ...]


@dataclass(frozen=True)
class ProblemReopenRow:
    problem_id: str
    title: str
    sample_bundle_doc: str
    representative_samples: tuple[str, ...]
    current_floor: tuple[str, ...]
    entry_commands: tuple[str, ...]
    mixed_gates: tuple[str, ...]
    reopen_guidance: tuple[str, ...]
    closed_split_packages: tuple[str, ...]
    split_packages: tuple[dict[str, object], ...]
    stop_line: tuple[str, ...]
    anchor_refs: tuple[str, ...]


@dataclass(frozen=True)
class ProblemTheoremEmitRow:
    sample_id: str
    reading: str
    output_path: str
    pilot_status: str
    pilot_subject_ref: str | None
    lean_stub_artifact_count: int
    principal_review_unit_ref_count: int
    repo_local_emitted_artifact_ref_count: int
    compare_floor_ref_count: int


@dataclass(frozen=True)
class ProblemScenarioEmitRow:
    sample_id: str
    reading: str
    output_path: str
    static_gate: str
    terminal_outcome: str
    first_line_status: str
    reserve_lane_status: str


GLOBAL_TRUE_USER_SPEC_RESIDUALS = (
    "shared-space exhaustive final catalog beyond minimal working subset",
    "installed-binary / packaging / FFI / engine adapter / host integration target",
    "upper-layer application target beyond authoritative-room first scenario",
)

ONCE_THROUGH_CLOSEOUT_CURRENT_FIRST_LINES = (
    {
        "line_id": "problem1",
        "summary": (
            "checker-adjacent first strong typing layer / theorem-first notebook-first transport / "
            "row-local model-check carrier first"
        ),
    },
    {
        "line_id": "problem2",
        "summary": (
            "relation decomposition principal / authoritative-room first default / "
            "witness-provider public shape later keep"
        ),
    },
    {
        "line_id": "syntax-modality",
        "summary": (
            "semantic honesty first / lambda_circle_box partial basis keep / "
            "guarded・MDTT・MTT・Fitch-style stronger family keep"
        ),
    },
)

ONCE_THROUGH_CLOSEOUT_EXECUTABLE_ENTRY_COMMANDS = (
    (
        "cargo run -q -p mir-runtime --example mir_current_l2 -- "
        "check-source-sample "
        "samples/prototype/current-l2-typed-proof-model-check/"
        "p10-typed-authorized-fingerprint-declassification.txt "
        "--format pretty"
    ),
    "python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
    "python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
    "python3 scripts/current_l2_guided_samples.py reopen-map",
    "python3 scripts/current_l2_guided_samples.py residuals",
)

ONCE_THROUGH_CLOSEOUT_NEXT_SELF_DRIVEN_PACKAGES = (
    {
        "package_id": "133",
        "title": "reserve integration entrypoint sync",
        "summary": (
            "theorem-first external pilot / auditable_authority_witness / delegated_rng_service / "
            "model-check second-line reserve を reopen order 付きで保つ"
        ),
    },
    {
        "package_id": "134",
        "title": "parser-side residual closeout sync",
        "summary": (
            "companion surface / parser-side tranche / final parser-checker-runtime API residual を "
            "closeout queue と混ぜずに保つ"
        ),
    },
    {
        "package_id": "135",
        "title": "true user-spec residual freeze sync",
        "summary": (
            "packaging / FFI / engine adapter / exhaustive shared-space catalog / upper-layer app target "
            "を true user-spec residual として固定する"
        ),
    },
)

ONCE_THROUGH_CLOSEOUT_STOP_LINE = (
    "final public parser / checker / runtime API",
    "final public verifier contract",
    "final public witness/provider/artifact contract",
    "exhaustive shared-space catalog",
    "installed-binary / packaging / FFI / engine adapter / host integration target",
)

REMAINING_RESIDUAL_LANE_ORDER = (
    "problem1-final-public-seams",
    "problem2-final-public-seams",
    "syntax-modality-final-marker",
)

RESIDUAL_LANE_COMPONENT_ORDER = {
    "problem1-final-public-seams": (
        "typed source principal split",
        "theorem public-contract split",
        "model-check public-contract split",
    ),
    "problem2-final-public-seams": (
        "source wording / emitted schema split",
        "witness-provider public-shape split",
    ),
    "syntax-modality-final-marker": (
        "final modal foundation / final source marker",
    ),
}

RESIDUAL_LANE_CURRENT_RECOMMENDATIONS = {
    "problem1-final-public-seams": (
        "checker-adjacent principal keep + theorem-first/model-check-second residual keep"
    ),
    "problem2-final-public-seams": (
        "relation decomposition principal keep + witness/provider public-shape later keep"
    ),
    "syntax-modality-final-marker": (
        "partial basis keep + stronger family keep + readable source marker keep"
    ),
}

RESIDUAL_LANE_RETAINED_FAMILIES = {
    "problem1-final-public-seams": (
        "checker-adjacent structural marker keep",
        "notebook-first theorem / row-local model-check keep",
    ),
    "problem2-final-public-seams": (
        "edge-row principal keep",
        "stage-block secondary / serial reserve keep",
    ),
    "syntax-modality-final-marker": (
        "lambda_circle_box partial basis keep",
        "guarded / MDTT / MTT / Fitch-style stronger family keep",
    ),
}

RESIDUAL_LANE_SEPARATION_BOUNDARIES = {
    "problem1-final-public-seams": (
        "problem-local seam separation",
        "true user-spec residual separation",
    ),
    "problem2-final-public-seams": (
        "problem-local seam separation",
        "true user-spec residual separation",
    ),
    "syntax-modality-final-marker": (
        "problem-local seam separation",
        "true user-spec residual separation",
    ),
}

RESIDUAL_LANE_STOP_LINES = {
    "problem1-final-public-seams": (
        "final typed source principal",
        "final public theorem contract",
        "final public checker artifact",
        "final public verifier contract",
    ),
    "problem2-final-public-seams": (
        "final source-surface handoff wording",
        "final public witness/provider/artifact contract",
        "exhaustive shared-space catalog",
    ),
    "syntax-modality-final-marker": (
        "final modal foundation adoption",
        "final source marker adoption",
        "final parser grammar",
    ),
}


PROBLEM_REOPEN_ANCHOR_REFS = {
    "problem1": (
        "specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md",
        "specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md",
        "specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md",
        "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
        "docs/reports/0857-package101-102-problem-bundles.md",
        "docs/reports/0867-package113-representative-problem-quickstart-parity-checks.md",
    ),
    "problem2": (
        "specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md",
        "specs/examples/576-current-l2-problem2-authoritative-room-scenario-bundle-actualization.md",
        "specs/examples/587-current-l2-representative-problem-quickstart-parity-checks-actualization.md",
        "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
        "docs/reports/0857-package101-102-problem-bundles.md",
        "docs/reports/0867-package113-representative-problem-quickstart-parity-checks.md",
    ),
}

PROBLEM_SPLIT_PACKAGE_DETAILS = {
    "problem1": {
        "typed-source-principal": {
            "package_name": "typed source principal split",
            "summary": (
                "checker-adjacent principal / structural marker first / finite decidable index fragment first "
                "を保ったまま、typed source principal residual だけを theorem/model-check residual から切り離して読む。"
            ),
            "supporting_samples": (
                "p10-typed-authorized-fingerprint-declassification",
                "p11-typed-unauthorized-fingerprint-release",
                "p12-typed-classified-fingerprint-publication-block",
                "p15-typed-capture-escape-rejected",
                "p16-typed-remote-call-budget-exceeded",
            ),
            "kept_separate": (
                "theorem public-contract split",
                "model-check public-contract split",
            ),
            "stop_line": (
                "final typed source principal",
                "final typed calculus",
                "final public verifier contract",
            ),
            "anchor_refs": (
                "specs/examples/566-current-l2-first-strong-typing-finite-index-layer-actualization.md",
                "specs/examples/567-current-l2-lean-first-formal-skeleton-hardening-after-finite-index-widening.md",
                "specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md",
                "specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md",
                "docs/reports/0849-package92-first-strong-typing-finite-index-layer.md",
                "docs/reports/0850-package93-lean-first-formal-skeleton-hardening.md",
                "docs/reports/0869-package115-116-representative-problem-split-package-map-refresh.md",
            ),
        },
        "theorem-public-contract": {
            "package_name": "theorem public-contract split",
            "summary": (
                "review-unit transport first / notebook-consumer object first を保ったまま、theorem public-contract residual を typed residual と model-check residual から切り離して読む。"
            ),
            "supporting_samples": ("p06-typed-proof-owner-handoff",),
            "kept_separate": (
                "typed source principal split",
                "model-check public-contract split",
            ),
            "stop_line": (
                "final public theorem contract",
                "concrete theorem prover brand",
                "final public verifier contract",
            ),
            "anchor_refs": (
                "specs/examples/575-current-l2-problem1-theorem-first-pilot-bundle-actualization.md",
                "specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md",
                "docs/reports/0857-package101-102-problem-bundles.md",
                "docs/reports/0869-package115-116-representative-problem-split-package-map-refresh.md",
            ),
        },
        "model-check-public-contract": {
            "package_name": "model-check public-contract split",
            "summary": (
                "row-local property route first / checker-artifact route first を保ったまま、model-check public-contract residual を typed residual と theorem residual から切り離して読む。"
            ),
            "supporting_samples": (
                "p06-typed-proof-owner-handoff",
                "p10-typed-authorized-fingerprint-declassification",
                "p11-typed-unauthorized-fingerprint-release",
                "p12-typed-classified-fingerprint-publication-block",
                "p15-typed-capture-escape-rejected",
                "p16-typed-remote-call-budget-exceeded",
            ),
            "kept_separate": (
                "typed source principal split",
                "theorem public-contract split",
            ),
            "stop_line": (
                "first settled property language",
                "final public checker artifact",
                "final public verifier contract",
            ),
            "anchor_refs": (
                "specs/examples/573-current-l2-problem1-public-seam-residual-bundle-matrix.md",
                "specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md",
                "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
                "docs/reports/0869-package115-116-representative-problem-split-package-map-refresh.md",
            ),
        },
    },
    "problem2": {
        "source-wording-emitted-schema": {
            "package_name": "source wording / emitted schema split",
            "summary": (
                "edge-row principal / stage-block secondary / serial reserve を保ったまま、source wording residual と emitted schema residual を witness-provider public shape residual から切り離して読む。"
            ),
            "supporting_samples": (
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
                "p13-dice-late-join-missing-publication-witness",
                "p14-dice-late-join-handoff-before-publication",
            ),
            "kept_separate": ("witness-provider public-shape split",),
            "stop_line": (
                "final source-surface handoff wording",
                "final emitted-artifact schema",
                "final public parser / checker / runtime API",
            ),
            "anchor_refs": (
                "specs/examples/569-current-l2-order-handoff-source-surface-artifact-route-tightening.md",
                "specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md",
                "docs/reports/0852-package95-order-handoff-source-surface-artifact-tightening.md",
                "docs/reports/0869-package115-116-representative-problem-split-package-map-refresh.md",
            ),
        },
        "witness-provider-public-shape": {
            "package_name": "witness-provider public-shape split",
            "summary": (
                "claim/payload split first / route-schema split first を保ったまま、witness/provider public-shape residual を source wording residual から切り離して読む。"
            ),
            "supporting_samples": (
                "p07-dice-late-join-visible-history",
                "p08-dice-stale-reconnect-refresh",
                "p09-dice-delegated-rng-provider-placement",
                "p13-dice-late-join-missing-publication-witness",
                "p14-dice-late-join-handoff-before-publication",
            ),
            "kept_separate": ("source wording / emitted schema split",),
            "stop_line": (
                "final public witness/provider/artifact contract",
                "stronger fairness / replay profile",
                "exhaustive shared-space catalog",
            ),
            "anchor_refs": (
                "specs/examples/574-current-l2-problem2-public-shape-residual-bundle-matrix.md",
                "specs/examples/589-current-l2-representative-problem-split-package-map-refresh.md",
                "docs/reports/0856-package99-100-problem-residual-bundle-matrices.md",
                "docs/reports/0869-package115-116-representative-problem-split-package-map-refresh.md",
            ),
        },
    },
}


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


def display_path(path: Path) -> str:
    try:
        return relative_path(path)
    except ValueError:
        return str(path)


def lean_artifact_paths(sample: GuidedSample) -> tuple[str, ...]:
    lean_dir = REPO_ROOT / "samples" / "lean" / "current-l2" / sample.sample_id
    candidates = (
        lean_dir / "README.md",
        lean_dir / f"{sample.sample_id}.lean",
        lean_dir / f"{sample.sample_id}.bundle.json",
    )
    return tuple(relative_path(path) for path in candidates if path.exists())


def parser_companion_path(sample: GuidedSample) -> str | None:
    companion = (
        REPO_ROOT
        / "samples"
        / "prototype"
        / "current-l2-parser-companion"
        / f"{sample.sample_id}.request.txt"
    )
    if companion.exists():
        return relative_path(companion)
    return None


def all_guided_samples() -> tuple[GuidedSample, ...]:
    rows: list[GuidedSample] = []
    for spec in problem_specs().values():
        rows.extend(spec.samples)
    return tuple(rows)


def guided_sample_by_id(sample_id: str) -> GuidedSample:
    for sample in all_guided_samples():
        if sample.sample_id == sample_id:
            return sample
    raise KeyError(f"unknown guided sample `{sample_id}`")


def bundle_commands(spec: ProblemSpec) -> tuple[str, ...]:
    primary = next(sample for sample in spec.samples if sample.primary)
    commands = [
        f"python3 scripts/current_l2_guided_samples.py show {spec.problem_id}",
        " ".join(build_single_run_command(primary, output_format="pretty")),
        f"python3 scripts/current_l2_guided_samples.py matrix {spec.problem_id}",
    ]
    if spec.problem_id == "problem1":
        commands.append("python3 scripts/current_l2_guided_samples.py emit-theorem problem1")
    if spec.problem_id == "problem2":
        commands.append("python3 scripts/current_l2_guided_samples.py emit-scenario problem2")
    commands.extend(
        [
            "python3 scripts/current_l2_guided_samples.py mapping",
            f"python3 scripts/current_l2_guided_samples.py run {spec.problem_id} --all --format json",
        ]
    )
    return tuple(commands)


def default_problem1_theorem_emit_output_dir() -> Path:
    return REPO_ROOT / "target" / "current-l2-guided" / "problem1-theorem-pilot"


def problem1_theorem_emit_reading(sample_id: str) -> str:
    if sample_id == "p06-typed-proof-owner-handoff":
        return "representative theorem-first sample"
    return "theorem-reached support sample"


def problem1_theorem_emit_command(
    sample_id: str,
    output_path: Path,
) -> list[str]:
    sample = guided_sample_by_id(sample_id)
    sample_argument = relative_path(sample.sample_path)
    host_plan_argument = relative_path(sample.sample_path.with_suffix(".host-plan.json"))
    return [
        "cargo",
        "run",
        "-q",
        "-p",
        "mir-runtime",
        "--example",
        "current_l2_emit_theorem_lean_bundle",
        "--",
        sample_argument,
        "--host-plan",
        host_plan_argument,
        "--output",
        str(output_path),
    ]


def emit_theorem_bundle_payload(
    sample_id: str,
    output_path: Path,
    *,
    runner: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> Mapping[str, object]:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    command = problem1_theorem_emit_command(sample_id, output_path)
    completed = runner(
        command,
        cwd=REPO_ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"emit-theorem `{sample_id}` failed with exit code {completed.returncode}: "
            f"{compact_text_for_summary((completed.stderr or completed.stdout or '').strip())}"
        )
    if not output_path.is_file():
        raise RuntimeError(f"emit-theorem `{sample_id}` did not create {display_path(output_path)}")
    return json.loads(output_path.read_text(encoding="utf-8"))


def build_problem1_theorem_emit_rows(
    *,
    output_dir: Path,
    emitter: Callable[[str, Path], Mapping[str, object]] = emit_theorem_bundle_payload,
) -> list[ProblemTheoremEmitRow]:
    rows: list[ProblemTheoremEmitRow] = []
    for sample_id in PROBLEM1_THEOREM_EMIT_SAMPLE_IDS:
        output_path = output_dir / f"{sample_id}.lean-bundle.json"
        payload = emitter(sample_id, output_path)
        lean_stub_artifacts = payload.get("lean_stub_artifacts")
        principal_review_unit_refs = payload.get("principal_review_unit_refs")
        repo_local_emitted_artifact_refs = payload.get("repo_local_emitted_artifact_refs")
        compare_floor_refs = payload.get("compare_floor_refs")
        pilot_subject_ref = payload.get("pilot_subject_ref")
        pilot_status = payload.get("pilot_status")
        rows.append(
            ProblemTheoremEmitRow(
                sample_id=sample_id,
                reading=problem1_theorem_emit_reading(sample_id),
                output_path=display_path(output_path),
                pilot_status=pilot_status if isinstance(pilot_status, str) else "missing",
                pilot_subject_ref=pilot_subject_ref if isinstance(pilot_subject_ref, str) else None,
                lean_stub_artifact_count=len(lean_stub_artifacts)
                if isinstance(lean_stub_artifacts, list)
                else 0,
                principal_review_unit_ref_count=len(principal_review_unit_refs)
                if isinstance(principal_review_unit_refs, list)
                else 0,
                repo_local_emitted_artifact_ref_count=len(repo_local_emitted_artifact_refs)
                if isinstance(repo_local_emitted_artifact_refs, list)
                else 0,
                compare_floor_ref_count=len(compare_floor_refs)
                if isinstance(compare_floor_refs, list)
                else 0,
            )
        )
    return rows


def build_problem1_theorem_emit_manifest(
    spec: ProblemSpec,
    *,
    output_dir: Path,
    emitter: Callable[[str, Path], Mapping[str, object]] = emit_theorem_bundle_payload,
) -> dict[str, object]:
    rows = build_problem1_theorem_emit_rows(output_dir=output_dir, emitter=emitter)
    return {
        "problem_id": spec.problem_id,
        "title": "Problem 1 theorem-first emitted artifact loop",
        "current_reading": (
            "`current_l2_emit_theorem_lean_bundle` を `p06 / p07 / p08` representative theorem line に対して"
            " repo-local output dir へ materialize し、theorem-first pilot を executable artifact loop として"
            " 再確認する helper-local command。final public theorem contract ではない。"
        ),
        "command": "python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
        "output_dir": display_path(output_dir),
        "rows": [asdict(row) for row in rows],
        "stop_line": list(PROBLEM1_THEOREM_EMIT_STOP_LINE),
    }


def render_problem1_theorem_emit(
    spec: ProblemSpec,
    rows: list[ProblemTheoremEmitRow],
    *,
    output_dir: Path,
) -> str:
    lines = [
        "Problem 1 theorem-first emitted artifact loop",
        "",
        "Problem 1 theorem-first pilot を emitted artifact / Lean bundle / representative sample loop として再確認する repo-local helper。",
        "",
        f"sample bundle doc: {PROBLEM_SAMPLE_BUNDLE_DOCS[spec.problem_id]}",
        "command: python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
        f"output dir: {display_path(output_dir)}",
        "",
    ]
    for row in rows:
        lines.append(f"- {row.sample_id}: {row.reading}")
        lines.append(f"  output: {row.output_path}")
        lines.append(f"  pilot_status: {row.pilot_status}")
        if row.pilot_subject_ref is not None:
            lines.append(f"  pilot_subject_ref: {row.pilot_subject_ref}")
        lines.append(f"  lean_stub_artifact_count: {row.lean_stub_artifact_count}")
        lines.append(
            "  ref_counts: "
            f"review={row.principal_review_unit_ref_count}, "
            f"emitted={row.repo_local_emitted_artifact_ref_count}, "
            f"compare={row.compare_floor_ref_count}"
        )
        lines.append("")
    lines.append("stop line:")
    for item in PROBLEM1_THEOREM_EMIT_STOP_LINE:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "注意:",
            "- `p06` を representative theorem-first sample、`p07 / p08` を theorem-reached support pair として materialize する narrow helper である。",
            "- final public theorem contract、concrete theorem prover brand、final public verifier contract には上げない。",
        ]
    )
    return "\n".join(lines)


def render_problem1_theorem_emit_from_runtime(
    spec: ProblemSpec,
    *,
    output_format: str,
    output_dir: Path | None = None,
) -> str:
    output_dir = output_dir or default_problem1_theorem_emit_output_dir()
    manifest = build_problem1_theorem_emit_manifest(spec, output_dir=output_dir)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    rows = [
        ProblemTheoremEmitRow(**row)
        for row in manifest["rows"]
    ]
    return render_problem1_theorem_emit(spec, rows, output_dir=output_dir)


def default_problem2_scenario_emit_output_dir() -> Path:
    return REPO_ROOT / "target" / "current-l2-guided" / "problem2-scenario-bundle"


def problem2_scenario_emit_command(sample_id: str) -> list[str]:
    sample_argument = relative_path(guided_sample_by_id(sample_id).sample_path)
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
        sample_argument,
        "--format",
        "json",
    ]


def emit_problem2_scenario_payload(
    sample_id: str,
    output_path: Path,
    *,
    runner: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> Mapping[str, object]:
    output_path.parent.mkdir(parents=True, exist_ok=True)
    command = problem2_scenario_emit_command(sample_id)
    completed = runner(
        command,
        cwd=REPO_ROOT,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"emit-scenario `{sample_id}` failed with exit code {completed.returncode}: "
            f"{compact_text_for_summary((completed.stderr or completed.stdout or '').strip())}"
        )
    output_path.write_text(completed.stdout, encoding="utf-8")
    return json.loads(completed.stdout)


def terminal_outcome_text(report: Mapping[str, object]) -> str:
    runtime = report.get("runtime")
    if not isinstance(runtime, Mapping):
        return "missing"
    outcome = runtime.get("terminal_outcome")
    if outcome is None:
        return "none"
    if isinstance(outcome, str):
        return outcome.lower()
    return "missing"


def build_problem2_scenario_emit_rows(
    *,
    output_dir: Path,
    emitter: Callable[[str, Path], Mapping[str, object]] = emit_problem2_scenario_payload,
) -> list[ProblemScenarioEmitRow]:
    rows: list[ProblemScenarioEmitRow] = []
    for sample_id in PROBLEM2_SCENARIO_EMIT_SAMPLE_IDS:
        output_path = output_dir / f"{sample_id}.run.json"
        report = emitter(sample_id, output_path)
        rows.append(
            ProblemScenarioEmitRow(
                sample_id=sample_id,
                reading=problem2_residual_reading(report),
                output_path=display_path(output_path),
                static_gate=checker_static_gate_verdict(report),
                terminal_outcome=terminal_outcome_text(report),
                first_line_status=helper_plain_status(
                    report, "authoritative_room_first_scenario_actual_adoption"
                ),
                reserve_lane_status=helper_plain_status(
                    report, "authoritative_room_reserve_strengthening_lane"
                ),
            )
        )
    return rows


def build_problem2_scenario_emit_manifest(
    spec: ProblemSpec,
    *,
    output_dir: Path,
    emitter: Callable[[str, Path], Mapping[str, object]] = emit_problem2_scenario_payload,
) -> dict[str, object]:
    rows = build_problem2_scenario_emit_rows(output_dir=output_dir, emitter=emitter)
    return {
        "problem_id": spec.problem_id,
        "title": "Problem 2 authoritative-room runnable scenario loop",
        "current_reading": (
            "`run-source-sample --format json` を representative / reserve / negative pair "
            "`p07 / p08 / p09 / p13 / p14` に対して repo-local output dir へ materialize し、"
            "authoritative-room first scenario current default を runnable scenario bundle として再確認する "
            "helper-local command。final public witness/provider/artifact contract ではない。"
        ),
        "command": "python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
        "output_dir": display_path(output_dir),
        "rows": [asdict(row) for row in rows],
        "stop_line": list(PROBLEM_BUNDLE_STOP_LINES["problem2"]),
    }


def render_problem2_scenario_emit(
    spec: ProblemSpec,
    rows: list[ProblemScenarioEmitRow],
    *,
    output_dir: Path,
) -> str:
    lines = [
        "Problem 2 authoritative-room runnable scenario loop",
        "",
        "Problem 2 first-line / reserve / negative pair を repo-local output dir に materialize し、authoritative-room current default を runnable scenario bundle として再確認する helper。",
        "",
        f"sample bundle doc: {PROBLEM_SAMPLE_BUNDLE_DOCS[spec.problem_id]}",
        "command: python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
        f"output dir: {display_path(output_dir)}",
        "",
    ]
    for row in rows:
        lines.append(f"- {row.sample_id}: {row.reading}")
        lines.append(f"  output: {row.output_path}")
        lines.append(f"  static_gate: {row.static_gate}")
        lines.append(f"  terminal_outcome: {row.terminal_outcome}")
        lines.append(f"  first_line_status: {row.first_line_status}")
        lines.append(f"  reserve_lane_status: {row.reserve_lane_status}")
        lines.append("")
    lines.append("stop line:")
    for item in PROBLEM_BUNDLE_STOP_LINES["problem2"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "注意:",
            "- `p07 / p08` representative pair、`p09` reserve route、`p13 / p14` negative pair を同じ output dir に materialize する narrow helper である。",
            "- final source wording、final public witness/provider/artifact contract、exhaustive shared-space catalog には上げない。",
        ]
    )
    return "\n".join(lines)


def render_problem2_scenario_emit_from_runtime(
    spec: ProblemSpec,
    *,
    output_format: str,
    output_dir: Path | None = None,
) -> str:
    output_dir = output_dir or default_problem2_scenario_emit_output_dir()
    manifest = build_problem2_scenario_emit_manifest(spec, output_dir=output_dir)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    rows = [ProblemScenarioEmitRow(**row) for row in manifest["rows"]]
    return render_problem2_scenario_emit(spec, rows, output_dir=output_dir)


def parser_companion_inspector_command(
    sample: GuidedSample,
    *,
    output_format: str = "pretty",
) -> str:
    return " ".join(
        parser_companion_inspector_command_argv(sample, output_format=output_format)
    )


def representative_parser_companion_rows() -> tuple[tuple[str, GuidedSample], ...]:
    rows: list[tuple[str, GuidedSample]] = []
    for problem_id, spec in problem_specs().items():
        for sample in spec.samples:
            if sample.primary and parser_companion_path(sample) is not None:
                rows.append((problem_id, sample))
    return tuple(rows)


def primary_sample(spec: ProblemSpec) -> GuidedSample:
    return next(sample for sample in spec.samples if sample.primary)


def build_parser_companion_mapping_manifest() -> dict[str, object]:
    rows = []
    for problem_id, sample in representative_parser_companion_rows():
        rows.append(
            {
                "sample_id": sample.sample_id,
                "problem_id": problem_id,
                "prototype_path": relative_path(sample.sample_path),
                "parser_companion_path": parser_companion_path(sample),
                "guided_bundle_command": f"python3 scripts/current_l2_guided_samples.py bundle {problem_id}",
                "guided_matrix_command": f"python3 scripts/current_l2_guided_samples.py matrix {problem_id}",
                "inspector_command": parser_companion_inspector_command(sample),
                "lean_artifacts": lean_artifact_paths(sample),
                "anchor_refs": (
                    PARSER_COMPANION_MAPPING_BUNDLE_ANCHORS[problem_id],
                    *PARSER_COMPANION_MAPPING_SHARED_ANCHORS,
                ),
            }
        )
    return {
        "mapping_kind": "current_l2_parser_companion_representative_mapping",
        "title": "parser companion representative mapping",
        "current_reading": (
            "representative slice `p06 / p07 / p08` を original prototype / parser companion / "
            "guided bundle / Lean artifact / anchor spec-report の 5 層で読み直すための helper-local matrix。"
        ),
        "rows": rows,
        "stop_line": PARSER_COMPANION_MAPPING_STOP_LINE,
    }


def render_parser_companion_mapping() -> str:
    manifest = build_parser_companion_mapping_manifest()
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
    ]

    for row in manifest["rows"]:
        lines.append(f"- {row['sample_id']} ({row['problem_id']})")
        lines.append(f"  original prototype: {row['prototype_path']}")
        lines.append(f"  parser companion: {row['parser_companion_path']}")
        lines.append(f"  guided bundle: {row['guided_bundle_command']}")
        lines.append(f"  guided matrix: {row['guided_matrix_command']}")
        lines.append(f"  inspector: {row['inspector_command']}")
        for artifact in row["lean_artifacts"]:
            lines.append(f"  Lean artifact: {artifact}")
        lines.append("  anchor refs:")
        for anchor in row["anchor_refs"]:
            lines.append(f"    - {anchor}")
        lines.append("")

    lines.append("stop line:")
    for item in manifest["stop_line"]:
        lines.append(f"- {item}")

    lines.extend(
        [
            "",
            "注意:",
            "- representative slice だけを対象にした mapping であり、exhaustive sample catalog ではない。",
            "- final public parser / checker / runtime API や final public tutorial surface を意味しない。",
        ]
    )
    return "\n".join(lines)


def build_problem_bundle_manifest(spec: ProblemSpec) -> dict[str, object]:
    primary_samples = [sample for sample in spec.samples if sample.primary]
    support_samples = [sample for sample in spec.samples if not sample.primary]

    return {
        "problem_id": spec.problem_id,
        "title": PROBLEM_BUNDLE_TITLES[spec.problem_id],
        "current_reading": PROBLEM_BUNDLE_READINGS[spec.problem_id],
        "summary": spec.summary,
        "sample_bundle_doc": PROBLEM_SAMPLE_BUNDLE_DOCS[spec.problem_id],
        "commands": bundle_commands(spec),
        "primary_samples": [
            {
                "sample_id": sample.sample_id,
                "prototype_path": relative_path(sample.sample_path),
                "parser_companion_path": parser_companion_path(sample),
                "summary": sample.summary,
                "lean_artifacts": lean_artifact_paths(sample),
            }
            for sample in primary_samples
        ],
        "support_samples": [
            {
                "sample_id": sample.sample_id,
                "prototype_path": relative_path(sample.sample_path),
                "parser_companion_path": parser_companion_path(sample),
                "summary": sample.summary,
                "lean_artifacts": lean_artifact_paths(sample),
            }
            for sample in support_samples
        ],
        "doc_refs": PROBLEM_BUNDLE_DOC_REFS[spec.problem_id],
        "stop_line": PROBLEM_BUNDLE_STOP_LINES[spec.problem_id],
    }


def parser_companion_inspector_command_argv(
    sample: GuidedSample,
    *,
    output_format: str = "json",
) -> list[str]:
    companion = parser_companion_path(sample)
    if companion is None:
        raise ValueError(f"sample `{sample.sample_id}` does not have a parser companion path")
    return [
        "cargo",
        "run",
        "-q",
        "-p",
        "mir-ast",
        "--example",
        "current_l2_inspect_request_head_clause_bundle",
        "--",
        companion,
        "--format",
        output_format,
    ]


def render_problem_bundle(spec: ProblemSpec) -> str:
    manifest = build_problem_bundle_manifest(spec)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
        str(manifest["summary"]),
        "",
        f"sample bundle doc: {manifest['sample_bundle_doc']}",
        "",
        "おすすめの追い方:",
    ]

    for index, command in enumerate(manifest["commands"], start=1):
        lines.append(f"{index}. {command}")

    lines.extend(["", "代表サンプルと Lean artifact:"])
    for sample in manifest["primary_samples"]:
        lines.append(f"- {sample['sample_id']}: {sample['summary']}")
        lines.append(f"  prototype: {sample['prototype_path']}")
        if sample["parser_companion_path"] is not None:
            lines.append(f"  parser companion: {sample['parser_companion_path']}")
        for artifact in sample["lean_artifacts"]:
            lines.append(f"  lean artifact: {artifact}")

    lines.extend(["", "補助サンプル:"])
    for sample in manifest["support_samples"]:
        lines.append(f"- {sample['sample_id']}: {sample['summary']}")
        lines.append(f"  prototype: {sample['prototype_path']}")
        if sample["parser_companion_path"] is not None:
            lines.append(f"  parser companion: {sample['parser_companion_path']}")
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


def build_problem_quickstart_steps(spec: ProblemSpec) -> tuple[ProblemQuickstartStep, ...]:
    first_primary = primary_sample(spec)

    if spec.problem_id == "problem1":
        return (
            ProblemQuickstartStep(
                title="`smoke problem1` で representative line を一度に確認する",
                command="python3 scripts/current_l2_guided_samples.py smoke problem1",
                expected_results=(
                    "`p06` の runtime / `matrix problem1` / `bundle problem1` / parser companion inspector / `mapping` が順に通る。",
                    "representative sample bundle の主要 command 群が drift していないことを 1 本で確認できる。",
                ),
            ),
            ProblemQuickstartStep(
                title="`matrix problem1` で representative と補助 sample の役割差を見る",
                command="python3 scripts/current_l2_guided_samples.py matrix problem1",
                expected_results=(
                    "`p06` が public-seam representative として先頭に出る。",
                    "`p10 / p11 / p12 / p15 / p16` は checker-adjacent bridge-floor 補助 sample として残る。",
                ),
            ),
            ProblemQuickstartStep(
                title="`bundle problem1` で docs / Lean artifact / anchor spec-report まで一本道で辿る",
                command="python3 scripts/current_l2_guided_samples.py bundle problem1",
                expected_results=(
                    "representative sample path、Lean artifact path、parser companion path、anchor spec / report が 1 画面で読める。",
                    "final public theorem contract や final public verifier contract に上げていない stop line も同時に確認できる。",
                ),
            ),
            ProblemQuickstartStep(
                title="parser companion inspector で request/head/clause bundle を直接見る",
                command=parser_companion_inspector_command(first_primary, output_format="pretty"),
                expected_results=(
                    "`p06` companion surface が repo-local parser-side carrier に戻っていることが分かる。",
                    "surface を final grammar に昇格せず、thin experimental companion として保っている current cut を追える。",
                ),
            ),
        )

    return (
        ProblemQuickstartStep(
            title="`smoke problem2` で representative pair を一度に確認する",
            command="python3 scripts/current_l2_guided_samples.py smoke problem2",
            expected_results=(
                "`p07 / p08` の runtime / `matrix problem2` / `bundle problem2` / parser companion inspector / `mapping` が順に通る。",
                "authoritative-room first completion line の representative pair が drift していないことを 1 本で確認できる。",
            ),
        ),
        ProblemQuickstartStep(
            title="`matrix problem2` で representative / reserve / negative pair を分けて読む",
            command="python3 scripts/current_l2_guided_samples.py matrix problem2",
            expected_results=(
                "`p07 / p08` が first-line representative pair として見える。",
                "`p09` が delegated RNG practical reserve route、`p13 / p14` が negative static-stop pair として分かれて見える。",
            ),
        ),
        ProblemQuickstartStep(
            title="`bundle problem2` で docs / Lean artifact / anchor spec-report まで一本道で辿る",
            command="python3 scripts/current_l2_guided_samples.py bundle problem2",
            expected_results=(
                "representative pair、reserve route、negative pair、Lean artifact、anchor spec / report が 1 画面で読める。",
                "final public witness/provider/artifact contract や exhaustive shared-space catalog をまだ確定していない stop line も確認できる。",
            ),
        ),
        ProblemQuickstartStep(
            title="parser companion inspector で order-handoff companion surface を直接見る",
            command=parser_companion_inspector_command(first_primary, output_format="pretty"),
            expected_results=(
                "edge-row principal / stage-block secondary の companion surface が parser-side carrier に戻っていることが分かる。",
                "final source wording を凍らせず、thin experimental companion として保っている current cut を追える。",
            ),
        ),
    )


def build_problem_quickstart_manifest(spec: ProblemSpec) -> dict[str, object]:
    steps = build_problem_quickstart_steps(spec)
    return {
        "problem_id": spec.problem_id,
        "title": f"{PROBLEM_BUNDLE_TITLES[spec.problem_id]} quickstart",
        "sample_bundle_doc": PROBLEM_SAMPLE_BUNDLE_DOCS[spec.problem_id],
        "current_reading": (
            "representative sample を見る最短 4 ステップを helper-side summary に mirror した "
            "repo-local quickstart。final public tutorial surface ではない。"
        ),
        "steps": [
            {
                "title": step.title,
                "command": step.command,
                "expected_results": list(step.expected_results),
            }
            for step in steps
        ],
    }


def render_problem_quickstart(spec: ProblemSpec) -> str:
    manifest = build_problem_quickstart_manifest(spec)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
        f"sample bundle doc: {manifest['sample_bundle_doc']}",
        "",
    ]
    for index, step in enumerate(manifest["steps"], start=1):
        lines.append(f"{index}. {step['title']}")
        lines.append(f"   command: {step['command']}")
        lines.append("   見るべき結果:")
        for item in step["expected_results"]:
            lines.append(f"   - {item}")
        lines.append("")
    lines.extend(
        [
            "注意:",
            "- current helper summary は representative 4 ステップだけに留め、exhaustive tutorial には広げない。",
            "- final public CLI / tutorial surface、final public parser / checker / runtime API を意味しない。",
        ]
    )
    return "\n".join(lines)


def render_problem_quickstart_from_runtime(spec: ProblemSpec, *, output_format: str) -> str:
    manifest = build_problem_quickstart_manifest(spec)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_problem_quickstart(spec)


def build_problem_reopen_row(spec: ProblemSpec) -> ProblemReopenRow:
    if spec.problem_id == "problem1":
        return ProblemReopenRow(
            problem_id="problem1",
            title="Problem 1 mixed-gate reopen map",
            sample_bundle_doc=PROBLEM_SAMPLE_BUNDLE_DOCS["problem1"],
            representative_samples=("p06-typed-proof-owner-handoff",),
            current_floor=(
                "quickstart / smoke / matrix / bundle / parser companion inspector / check-source-sample / emit-theorem problem1 まで drift suppression 済み。",
                "checker-adjacent first layer、notebook-first theorem line、row-local model-check carrier first の current cut を representative sample から追える。",
            ),
            entry_commands=(
                "python3 scripts/current_l2_guided_samples.py quickstart problem1",
                (
                    "cargo run -q -p mir-runtime --example mir_current_l2 -- "
                    "check-source-sample "
                    "samples/prototype/current-l2-typed-proof-model-check/"
                    "p10-typed-authorized-fingerprint-declassification.txt "
                    "--format pretty"
                ),
                "python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
                "python3 scripts/current_l2_guided_samples.py lane problem1-final-public-seams",
            ),
            mixed_gates=(
                "stronger typed-surface actual adoption",
                "final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract",
                "first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract",
            ),
            reopen_guidance=(
                "`check-source-sample` で `p10 / p11 / p12 / p15 / p16` の checker-adjacent executable slice を見て、typed source principal を premature に上げない current cut を保つ。",
                "`emit-theorem problem1` と `samples/lean/current-l2/p06-typed-proof-owner-handoff/` を起点に、theorem-first pilot artifact と notebook-first transport floor を確認する。",
                "`lane problem1-final-public-seams` と `matrix problem1` / `bundle problem1` を合わせて見て、model-check carrier first のまま final public checker 契約へ飛ばないことを確認する。",
            ),
            closed_split_packages=(
                "typed source principal split",
                "theorem public-contract split",
                "model-check public-contract split",
            ),
            split_packages=(
                {
                    "package_name": "typed source principal split",
                    "focus": "stronger typed-surface actual adoption",
                    "commands": (
                        "python3 scripts/current_l2_guided_samples.py matrix problem1",
                    ),
                    "reading": (
                        "`p06` representative と `p10 / p11 / p12 / p15 / p16` bridge-floor 補助 sample の役割差を保ち、typed source principal を premature に上げない current cut を切り出す。"
                    ),
                },
                {
                    "package_name": "theorem public-contract split",
                    "focus": "final public theorem result object / consumer-shaped theorem payload public contract / concrete theorem prover brand / proof object public schema / final public verifier contract",
                    "commands": (
                        "python3 scripts/current_l2_guided_samples.py bundle problem1",
                    ),
                    "reading": (
                        "review-unit first / notebook-consumer first のまま、theorem-first pilot artifact と Lean sample corpus から final public theorem contract residual を切り出す。"
                    ),
                },
                {
                    "package_name": "model-check public-contract split",
                    "focus": "first settled property language / concrete model-check tool brand / final public checker artifact / actual public checker migration / actual emitted verifier handoff artifact / production checker-runtime-policy contract / final public verifier contract",
                    "commands": (
                        "python3 scripts/current_l2_guided_samples.py matrix problem1",
                        "python3 scripts/current_l2_guided_samples.py bundle problem1",
                    ),
                    "reading": (
                        "row-local property route first / checker-artifact route first を保ったまま、model-check public-contract residual を theorem residual から分けて切り出す。"
                    ),
                },
            ),
            stop_line=PROBLEM_BUNDLE_STOP_LINES["problem1"],
            anchor_refs=PROBLEM_REOPEN_ANCHOR_REFS["problem1"],
        )

    return ProblemReopenRow(
        problem_id="problem2",
        title="Problem 2 mixed-gate reopen map",
        sample_bundle_doc=PROBLEM_SAMPLE_BUNDLE_DOCS["problem2"],
        representative_samples=(
            "p07-dice-late-join-visible-history",
            "p08-dice-stale-reconnect-refresh",
        ),
        current_floor=(
            "quickstart / smoke / matrix / bundle / parser companion inspector / emit-scenario problem2 まで drift suppression 済み。",
            "relation decomposition principal、authoritative-room first default、reserve lane split の current cut を representative pair から追える。",
        ),
        entry_commands=(
            "python3 scripts/current_l2_guided_samples.py quickstart problem2",
            "python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
            "python3 scripts/current_l2_guided_samples.py lane problem2-final-public-seams",
            "python3 scripts/current_l2_guided_samples.py residuals",
        ),
        mixed_gates=(
            "final source-surface handoff wording / final emitted-artifact schema",
            "final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract",
        ),
        reopen_guidance=(
            "`emit-scenario problem2` で `p07 / p08` representative pair、`p09` reserve route、`p13 / p14` negative static-stop pair の current split を repo-local output dir まで含めて再確認する。",
            "`lane problem2-final-public-seams` と `bundle problem2` / parser companion inspector を合わせて見て、edge-row principal / stage-block secondary のまま final source wording や emitted schema を凍らせない current cut を保つ。",
            "`residuals` と `matrix problem2` を合わせて見て、shared-space stronger public shape を final public witness/provider 契約へ上げずに stop line と user-spec residual を切り分ける。",
        ),
        closed_split_packages=(
            "source wording / emitted schema split",
            "witness-provider public-shape split",
        ),
        split_packages=(
            {
                "package_name": "source wording / emitted schema split",
                "focus": "final source-surface handoff wording / final emitted-artifact schema",
                "commands": (
                    "python3 scripts/current_l2_guided_samples.py bundle problem2",
                ),
                "reading": (
                    "edge-row principal / stage-block secondary を保ったまま、source wording と emitted schema residual を witness-provider public shape から分けて切り出す。"
                ),
            },
            {
                "package_name": "witness-provider public-shape split",
                "focus": "final public witness schema / provider receipt schema / combined public contract / emitted-handoff contract",
                "commands": (
                    "python3 scripts/current_l2_guided_samples.py matrix problem2",
                    "python3 scripts/current_l2_guided_samples.py bundle problem2",
                ),
                "reading": (
                    "representative / reserve / negative pair の current split を保ったまま、claim / payload split first の shared-space public-shape residual を切り出す。"
                ),
            },
        ),
        stop_line=PROBLEM_BUNDLE_STOP_LINES["problem2"],
        anchor_refs=PROBLEM_REOPEN_ANCHOR_REFS["problem2"],
    )


def build_problem_reopen_map_manifest(specs: Mapping[str, ProblemSpec]) -> dict[str, object]:
    rows = [build_problem_reopen_row(specs[problem_id]) for problem_id in sorted(specs.keys())]
    return {
        "map_kind": "current_l2_representative_problem_mixed_gate_reopen_map",
        "title": "representative problem mixed-gate reopen map",
        "current_reading": (
            "representative sample の quickstart / matrix / bundle / smoke floor を踏まえて、"
            "Problem 1 / Problem 2 の remaining mixed gate と true user-spec residual を "
            "entry command 付きで読み直す helper-local map。"
        ),
        "problem_rows": [
            {
                "problem_id": row.problem_id,
                "title": row.title,
                "sample_bundle_doc": row.sample_bundle_doc,
                "representative_samples": list(row.representative_samples),
                "current_floor": list(row.current_floor),
                "entry_commands": list(row.entry_commands),
                "mixed_gates": list(row.mixed_gates),
                "reopen_guidance": list(row.reopen_guidance),
                "closed_split_packages": list(row.closed_split_packages),
                "stop_line": list(row.stop_line),
                "anchor_refs": list(row.anchor_refs),
            }
            for row in rows
        ],
        "true_user_spec_residuals": list(GLOBAL_TRUE_USER_SPEC_RESIDUALS),
    }


def render_problem_reopen_map(specs: Mapping[str, ProblemSpec]) -> str:
    manifest = build_problem_reopen_map_manifest(specs)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
    ]

    for row in manifest["problem_rows"]:
        lines.append(f"- {row['problem_id']}: {row['title']}")
        lines.append(f"  sample bundle doc: {row['sample_bundle_doc']}")
        lines.append(f"  representative samples: {', '.join(row['representative_samples'])}")
        lines.append("  current floor:")
        for item in row["current_floor"]:
            lines.append(f"    - {item}")
        lines.append("  entry commands:")
        for command in row["entry_commands"]:
            lines.append(f"    - {command}")
        lines.append("  remaining mixed gates:")
        for gate in row["mixed_gates"]:
            lines.append(f"    - {gate}")
        lines.append("  reopen guidance:")
        for item in row["reopen_guidance"]:
            lines.append(f"    - {item}")
        if row["closed_split_packages"]:
            lines.append("  split package closeout:")
            for item in row["closed_split_packages"]:
                lines.append(f"    - {item}")
        lines.append("  anchor refs:")
        for ref in row["anchor_refs"]:
            lines.append(f"    - {ref}")
        lines.append("  stop line:")
        for item in row["stop_line"]:
            lines.append(f"    - {item}")
        lines.append("")

    lines.append("global true user-spec residuals:")
    for item in manifest["true_user_spec_residuals"]:
        lines.append(f"- {item}")

    lines.extend(
        [
            "",
            "注意:",
            "- current repo-local helper map であり、final public tutorial surface や final public parser / checker / runtime API を意味しない。",
            "- representative problem bundle から next reopen point を読みやすくする current cut に留める。",
        ]
    )
    return "\n".join(lines)


def render_problem_reopen_map_from_runtime(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
) -> str:
    manifest = build_problem_reopen_map_manifest(specs)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_problem_reopen_map(specs)


def build_remaining_residual_lane_manifest(specs: Mapping[str, ProblemSpec]) -> dict[str, object]:
    problem1_row = build_problem_reopen_row(specs["problem1"])
    problem2_row = build_problem_reopen_row(specs["problem2"])

    mixed_gate_lanes = [
        {
            "lane_id": "problem1-final-public-seams",
            "summary": (
                "Problem 1 remaining mixed gate を typed / theorem / model-check public seam cluster "
                "として圧縮し、true user-spec residual と混ぜずに読む。"
            ),
            "focus": list(problem1_row.mixed_gates),
            "entry_commands": [
                (
                    "cargo run -q -p mir-runtime --example mir_current_l2 -- "
                    "check-source-sample "
                    "samples/prototype/current-l2-typed-proof-model-check/"
                    "p10-typed-authorized-fingerprint-declassification.txt "
                    "--format pretty"
                ),
                "python3 scripts/current_l2_guided_samples.py emit-theorem problem1",
                "python3 scripts/current_l2_guided_samples.py reopen-map problem1",
            ],
            "anchor_refs": list(problem1_row.anchor_refs)
            + [
                "specs/examples/590-current-l2-problem1-typed-source-principal-split-helper-actualization.md",
                "specs/examples/591-current-l2-problem1-theorem-public-contract-split-helper-actualization.md",
                "specs/examples/592-current-l2-problem1-model-check-public-contract-split-helper-actualization.md",
            ],
        },
        {
            "lane_id": "problem2-final-public-seams",
            "summary": (
                "Problem 2 remaining mixed gate を final wording / witness-provider public-shape cluster "
                "として圧縮し、true user-spec residual と混ぜずに読む。"
            ),
            "focus": list(problem2_row.mixed_gates),
            "entry_commands": [
                "python3 scripts/current_l2_guided_samples.py emit-scenario problem2",
                "python3 scripts/current_l2_guided_samples.py reopen-map problem2",
                "python3 scripts/current_l2_guided_samples.py residuals",
            ],
            "anchor_refs": list(problem2_row.anchor_refs)
            + [
                "specs/examples/593-current-l2-problem2-source-wording-emitted-schema-split-helper-actualization.md",
                "specs/examples/594-current-l2-problem2-witness-provider-public-shape-split-helper-actualization.md",
            ],
        },
        {
            "lane_id": "syntax-modality-final-marker",
            "summary": (
                "syntax / modality line の final modal foundation / source marker を later mixed gate "
                "として keep し、problem-local final public seam と user-spec residual から切り分ける。"
            ),
            "focus": [
                "final modal foundation / final source marker",
            ],
            "entry_commands": [
                "python3 scripts/current_l2_guided_samples.py reopen-map",
                "python3 scripts/current_l2_guided_samples.py bundle problem1",
                "python3 scripts/current_l2_guided_samples.py bundle problem2",
            ],
            "anchor_refs": [
                "specs/10-open-questions.md",
                "plan/06-surface-notation-status.md",
                "plan/13-heavy-future-workstreams.md",
            ],
        },
    ]

    return {
        "manifest_kind": "current_l2_remaining_residual_lane_summary",
        "title": "current-l2 remaining residual lane summary",
        "current_reading": (
            "split package closeout 後に残る mixed gate と true user-spec residual を、"
            "next reopen order 付きで圧縮して読む helper-local summary。"
        ),
        "mixed_gate_lanes": mixed_gate_lanes,
        "true_user_spec_residuals": list(GLOBAL_TRUE_USER_SPEC_RESIDUALS),
        "recommended_order": list(REMAINING_RESIDUAL_LANE_ORDER),
    }


def render_remaining_residual_lane_summary(specs: Mapping[str, ProblemSpec]) -> str:
    manifest = build_remaining_residual_lane_manifest(specs)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
        "remaining mixed-gate lanes:",
    ]

    for lane in manifest["mixed_gate_lanes"]:
        lines.append(f"- {lane['lane_id']}")
        lines.append(f"  summary: {lane['summary']}")
        lines.append("  focus:")
        for item in lane["focus"]:
            lines.append(f"    - {item}")
        lines.append("  entry commands:")
        for command in lane["entry_commands"]:
            lines.append(f"    - {command}")
        lines.append("  anchor refs:")
        for ref in lane["anchor_refs"]:
            lines.append(f"    - {ref}")
        lines.append("")

    lines.append("recommended order:")
    for lane_id in manifest["recommended_order"]:
        lines.append(f"- {lane_id}")

    lines.append("")
    lines.append("true user-spec residuals:")
    for item in manifest["true_user_spec_residuals"]:
        lines.append(f"- {item}")

    lines.extend(
        [
            "",
            "注意:",
            "- current helper-local summary であり、final public parser / checker / runtime API や final public verifier contract を意味しない。",
            "- representative problem reopen-map と sample bundle から next reopen order を短く読み直す current cut に留める。",
        ]
    )
    return "\n".join(lines)


def render_remaining_residual_lane_summary_from_runtime(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
) -> str:
    manifest = build_remaining_residual_lane_manifest(specs)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_remaining_residual_lane_summary(specs)


def build_once_through_closeout_manifest(specs: Mapping[str, ProblemSpec]) -> dict[str, object]:
    residual_manifest = build_remaining_residual_lane_manifest(specs)
    return {
        "manifest_kind": "current_l2_once_through_closeout_summary",
        "title": "current-l2 once-through closeout summary",
        "current_reading": (
            "Package 127...131 で揃えた executable loop 群を踏まえて、repo-local once-through near-end "
            "completion reading と remaining mixed gate / true user-spec residual を 1 つの helper-local "
            "summary に再圧縮して読む。"
        ),
        "current_first_lines": [
            {
                "line_id": row["line_id"],
                "summary": row["summary"],
            }
            for row in ONCE_THROUGH_CLOSEOUT_CURRENT_FIRST_LINES
        ],
        "executable_entry_commands": list(ONCE_THROUGH_CLOSEOUT_EXECUTABLE_ENTRY_COMMANDS),
        "mixed_gate_lanes": list(residual_manifest["recommended_order"]),
        "next_self_driven_packages": [
            {
                "package_id": row["package_id"],
                "title": row["title"],
                "summary": row["summary"],
            }
            for row in ONCE_THROUGH_CLOSEOUT_NEXT_SELF_DRIVEN_PACKAGES
        ],
        "true_user_spec_residuals": list(GLOBAL_TRUE_USER_SPEC_RESIDUALS),
        "stop_line": list(ONCE_THROUGH_CLOSEOUT_STOP_LINE),
        "anchor_refs": [
            "specs/examples/603-current-l2-problem1-executable-residual-reopen-sync.md",
            "specs/examples/604-current-l2-problem2-executable-residual-reopen-sync.md",
            "specs/examples/596-current-l2-remaining-residual-lane-summary-actualization.md",
            "specs/examples/597-current-l2-problem1-final-public-seam-lane-helper-actualization.md",
            "specs/examples/598-current-l2-problem2-final-public-seam-lane-helper-actualization.md",
            "specs/examples/599-current-l2-syntax-modality-final-marker-lane-helper-actualization.md",
        ],
    }


def render_once_through_closeout_summary(specs: Mapping[str, ProblemSpec]) -> str:
    manifest = build_once_through_closeout_manifest(specs)
    lines = [
        str(manifest["title"]),
        "",
        str(manifest["current_reading"]),
        "",
        "current first lines:",
    ]
    for row in manifest["current_first_lines"]:
        lines.append(f"- {row['line_id']}: {row['summary']}")
    lines.extend(
        [
            "",
            "executable entry commands:",
        ]
    )
    for command in manifest["executable_entry_commands"]:
        lines.append(f"- {command}")
    lines.extend(
        [
            "",
            "mixed-gate lanes:",
        ]
    )
    for lane_id in manifest["mixed_gate_lanes"]:
        lines.append(f"- {lane_id}")
    lines.extend(
        [
            "",
            "next self-driven packages:",
        ]
    )
    for row in manifest["next_self_driven_packages"]:
        lines.append(f"- {row['package_id']}: {row['title']}")
        lines.append(f"  {row['summary']}")
    lines.extend(
        [
            "",
            "true user-spec residuals:",
        ]
    )
    for item in manifest["true_user_spec_residuals"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "stop line:",
        ]
    )
    for item in manifest["stop_line"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "anchor refs:",
        ]
    )
    for ref in manifest["anchor_refs"]:
        lines.append(f"- {ref}")
    lines.extend(
        [
            "",
            "注意:",
            "- current helper-local closeout summary であり、final public parser / checker / runtime API や final public verifier contract の確定を意味しない。",
            "- executable loop を揃えた後の near-end reading を短く保つための summary であり、mixed gate や true user-spec residual を消去しない。",
        ]
    )
    return "\n".join(lines)


def render_once_through_closeout_summary_from_runtime(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
) -> str:
    manifest = build_once_through_closeout_manifest(specs)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_once_through_closeout_summary(specs)


def residual_lane_ids() -> tuple[str, ...]:
    return REMAINING_RESIDUAL_LANE_ORDER


def build_residual_lane_manifest(
    lane_id: str,
    specs: Mapping[str, ProblemSpec],
) -> dict[str, object]:
    manifest = build_remaining_residual_lane_manifest(specs)
    try:
        lane = next(item for item in manifest["mixed_gate_lanes"] if item["lane_id"] == lane_id)
    except StopIteration as error:
        raise KeyError(f"residual lane `{lane_id}` is not defined") from error

    return {
        "lane_id": lane["lane_id"],
        "summary": lane["summary"],
        "focus": list(lane["focus"]),
        "entry_commands": list(lane["entry_commands"]),
        "current_recommendation": RESIDUAL_LANE_CURRENT_RECOMMENDATIONS[lane_id],
        "retained_families": list(RESIDUAL_LANE_RETAINED_FAMILIES[lane_id]),
        "separation_boundary": list(RESIDUAL_LANE_SEPARATION_BOUNDARIES[lane_id]),
        "component_order": list(RESIDUAL_LANE_COMPONENT_ORDER[lane_id]),
        "stop_line": list(RESIDUAL_LANE_STOP_LINES[lane_id]),
        "anchor_refs": list(lane["anchor_refs"]),
    }


def render_residual_lane(lane_id: str, specs: Mapping[str, ProblemSpec]) -> str:
    manifest = build_residual_lane_manifest(lane_id, specs)
    lines = [
        str(manifest["lane_id"]),
        "",
        str(manifest["summary"]),
        "",
        "focus:",
    ]
    for item in manifest["focus"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "entry commands:",
        ]
    )
    for command in manifest["entry_commands"]:
        lines.append(f"- {command}")
    lines.extend(
        [
            "",
            "current recommendation:",
            f"- {manifest['current_recommendation']}",
            "",
            "retained families:",
        ]
    )
    for item in manifest["retained_families"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "separation boundary:",
        ]
    )
    for item in manifest["separation_boundary"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "component order:",
        ]
    )
    for item in manifest["component_order"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "stop line:",
        ]
    )
    for item in manifest["stop_line"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "anchor refs:",
        ]
    )
    for ref in manifest["anchor_refs"]:
        lines.append(f"- {ref}")
    return "\n".join(lines)


def render_residual_lane_from_runtime(
    lane_id: str,
    *,
    output_format: str,
) -> str:
    specs = problem_specs()
    manifest = build_residual_lane_manifest(lane_id, specs)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_residual_lane(lane_id, specs)


def problem_split_package_ids() -> tuple[str, ...]:
    return tuple(
        sorted(
            split_id
            for packages in PROBLEM_SPLIT_PACKAGE_DETAILS.values()
            for split_id in packages.keys()
        )
    )


def build_problem_split_package_manifest(problem_id: str, split_id: str) -> dict[str, object]:
    package_details = PROBLEM_SPLIT_PACKAGE_DETAILS.get(problem_id, {}).get(split_id)
    if package_details is None:
        raise KeyError(f"split package `{split_id}` is not defined for `{problem_id}`")

    spec = problem_specs()[problem_id]
    reopen_row = build_problem_reopen_row(spec)
    split_package = next(
        item
        for item in reopen_row.split_packages
        if item["package_name"] == package_details["package_name"]
    )

    return {
        "package_id": split_id,
        "package_name": package_details["package_name"],
        "problem_id": problem_id,
        "problem_title": spec.title,
        "sample_bundle_doc": PROBLEM_SAMPLE_BUNDLE_DOCS[problem_id],
        "summary": package_details["summary"],
        "representative_samples": list(reopen_row.representative_samples),
        "supporting_samples": list(package_details["supporting_samples"]),
        "commands": list(split_package["commands"]),
        "reading": split_package["reading"],
        "current_floor": list(reopen_row.current_floor),
        "kept_separate": list(package_details["kept_separate"]),
        "stop_line": list(package_details["stop_line"]),
        "anchor_refs": list(package_details["anchor_refs"]),
    }


def render_problem_split_package(problem_id: str, split_id: str) -> str:
    manifest = build_problem_split_package_manifest(problem_id, split_id)
    lines = [
        str(manifest["package_name"]),
        "",
        str(manifest["summary"]),
        "",
        f"problem: {manifest['problem_title']}",
        f"sample bundle doc: {manifest['sample_bundle_doc']}",
        f"representative samples: {', '.join(manifest['representative_samples'])}",
        f"supporting samples: {', '.join(manifest['supporting_samples'])}",
        "",
        "primary commands:",
    ]
    for command in manifest["commands"]:
        lines.append(f"- {command}")
    lines.extend(
        [
            "",
            f"reading: {manifest['reading']}",
            "",
            "current floor:",
        ]
    )
    for item in manifest["current_floor"]:
        lines.append(f"- {item}")
    lines.append("")
    lines.append("kept separate:")
    for item in manifest["kept_separate"]:
        lines.append(f"- {item}")
    lines.append("")
    lines.append("anchor refs:")
    for item in manifest["anchor_refs"]:
        lines.append(f"- {item}")
    lines.append("")
    lines.append("stop line:")
    for item in manifest["stop_line"]:
        lines.append(f"- {item}")
    lines.extend(
        [
            "",
            "注意:",
            "- current split package helper であり、final public source principal や final public verifier contract を意味しない。",
            "- split package を independent に読むための narrow helper/doc cut に留める。",
        ]
    )
    return "\n".join(lines)


def render_problem_split_package_from_runtime(
    problem_id: str,
    split_id: str,
    *,
    output_format: str,
) -> str:
    manifest = build_problem_split_package_manifest(problem_id, split_id)
    if output_format == "json":
        return json.dumps(manifest, ensure_ascii=False, indent=2)
    return render_problem_split_package(problem_id, split_id)


def build_problem_quickstart_parity_rows(
    specs: Mapping[str, ProblemSpec],
    *,
    doc_loader: Callable[[str], str] | None = None,
) -> list[dict[str, object]]:
    rows: list[dict[str, object]] = []
    for problem_id in sorted(specs.keys()):
        spec = specs[problem_id]
        doc_path = PROBLEM_SAMPLE_BUNDLE_DOCS[problem_id]
        if doc_loader is None:
            text = (REPO_ROOT / doc_path).read_text(encoding="utf-8")
        else:
            text = doc_loader(doc_path)
        normalized_text = normalize_shell_text_for_search(text)
        steps = build_problem_quickstart_steps(spec)
        missing_titles = [step.title for step in steps if step.title not in text]
        missing_commands = [
            step.command
            for step in steps
            if normalize_shell_text_for_search(step.command) not in normalized_text
        ]
        rows.append(
            {
                "problem_id": problem_id,
                "sample_bundle_doc": doc_path,
                "status": "synced" if not missing_titles and not missing_commands else "mismatch",
                "missing_titles": missing_titles,
                "missing_commands": missing_commands,
            }
        )
    return rows


def render_problem_quickstart_parity(rows: list[dict[str, object]]) -> str:
    lines = [
        "representative problem quickstart parity",
        "",
        "sample bundle doc と quickstart helper の 4-step 導線が揃っているかを narrow に見る repo-local check。",
        "",
    ]
    for row in rows:
        lines.append(f"- {row['problem_id']}: {row['status']}")
        lines.append(f"  sample bundle doc: {row['sample_bundle_doc']}")
        missing_titles = row["missing_titles"]
        missing_commands = row["missing_commands"]
        if missing_titles:
            lines.append("  missing titles:")
            for item in missing_titles:
                lines.append(f"    - {item}")
        if missing_commands:
            lines.append("  missing commands:")
            for item in missing_commands:
                lines.append(f"    - {item}")
        if not missing_titles and not missing_commands:
            lines.append("  parity: quickstart title / command line ともに synced")
        lines.append("")
    lines.extend(
        [
            "注意:",
            "- representative 4-step quickstart だけを対象にした narrow parity check であり、exhaustive tutorial validation ではない。",
            "- final public CLI / tutorial surface や final public parser / checker / runtime API を意味しない。",
        ]
    )
    return "\n".join(lines)


def render_problem_quickstart_parity_from_runtime(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
) -> str:
    rows = build_problem_quickstart_parity_rows(specs)
    if output_format == "json":
        return json.dumps(rows, ensure_ascii=False, indent=2)
    return render_problem_quickstart_parity(rows)


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


def build_problem_smoke_steps(spec: ProblemSpec) -> list[ProblemSmokeStep]:
    primary_samples = [sample for sample in spec.samples if sample.primary]
    steps: list[ProblemSmokeStep] = [
        ProblemSmokeStep(
            label=f"runtime:{sample.sample_id}",
            command=build_single_run_command(sample, output_format="pretty"),
        )
        for sample in primary_samples
    ]
    steps.append(
        ProblemSmokeStep(
            label=f"matrix:{spec.problem_id}",
            command=[
                "python3",
                "scripts/current_l2_guided_samples.py",
                "matrix",
                spec.problem_id,
                "--format",
                "json",
            ],
        )
    )
    steps.append(
        ProblemSmokeStep(
            label=f"bundle:{spec.problem_id}",
            command=[
                "python3",
                "scripts/current_l2_guided_samples.py",
                "bundle",
                spec.problem_id,
                "--format",
                "json",
            ],
        )
    )
    steps.extend(
        ProblemSmokeStep(
            label=f"inspector:{sample.sample_id}",
            command=parser_companion_inspector_command_argv(sample, output_format="json"),
        )
        for sample in primary_samples
        if parser_companion_path(sample) is not None
    )
    steps.append(
        ProblemSmokeStep(
            label="mapping",
            command=[
                "python3",
                "scripts/current_l2_guided_samples.py",
                "mapping",
                "--format",
                "json",
            ],
        )
    )
    return steps


def execute_problem_smoke_steps(
    spec: ProblemSpec,
    *,
    runner: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
    verbose: bool,
    capture_output: bool,
) -> tuple[list[ProblemSmokeStep], int, str | None, subprocess.CompletedProcess[str] | None]:
    steps = build_problem_smoke_steps(spec)
    successful_steps = 0

    for step in steps:
        if verbose:
            print(f"== {step.label} ==", flush=True)
            print(f"$ {' '.join(step.command)}", flush=True)
        completed = runner(
            step.command,
            cwd=REPO_ROOT,
            check=False,
            capture_output=capture_output,
            text=capture_output,
        )
        if completed.returncode != 0:
            if verbose and capture_output:
                if completed.stdout:
                    print(
                        completed.stdout,
                        end="" if completed.stdout.endswith("\n") else "\n",
                    )
                if completed.stderr:
                    print(
                        completed.stderr,
                        file=sys.stderr,
                        end="" if completed.stderr.endswith("\n") else "\n",
                    )
            return steps, successful_steps, step.label, completed
        successful_steps += 1
        if verbose:
            print("", flush=True)

    return steps, successful_steps, None, None


def compact_text_for_summary(text: str, *, limit: int = FAILURE_EXCERPT_LIMIT) -> str:
    compact = " ".join(text.split())
    if len(compact) <= limit:
        return compact
    return compact[: limit - 3].rstrip() + "..."


def normalize_shell_text_for_search(text: str) -> str:
    return " ".join(text.replace("\\\n", " ").split())


def failure_output_excerpt(failure: subprocess.CompletedProcess[str] | None) -> str | None:
    if failure is None:
        return None

    fragments: list[str] = []
    if isinstance(failure.stderr, str) and failure.stderr.strip():
        fragments.append(f"stderr: {compact_text_for_summary(failure.stderr)}")
    if isinstance(failure.stdout, str) and failure.stdout.strip():
        fragments.append(f"stdout: {compact_text_for_summary(failure.stdout)}")
    if not fragments:
        return None
    return compact_text_for_summary(" | ".join(fragments))


def run_problem_smoke(spec: ProblemSpec) -> int:
    _, _, _, failure = execute_problem_smoke_steps(
        spec,
        verbose=True,
        capture_output=False,
    )
    if failure is not None:
        return failure.returncode
    return 0


def build_problem_smoke_aggregate_rows(
    specs: Mapping[str, ProblemSpec],
    *,
    runner: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> list[ProblemSmokeAggregateRow]:
    rows: list[ProblemSmokeAggregateRow] = []
    for problem_id in sorted(specs.keys()):
        spec = specs[problem_id]
        steps, successful_steps, failed_step, failure = execute_problem_smoke_steps(
            spec,
            runner=runner,
            verbose=False,
            capture_output=True,
        )
        rows.append(
            ProblemSmokeAggregateRow(
                problem_id=spec.problem_id,
                title=PROBLEM_BUNDLE_TITLES[spec.problem_id],
                status="passed" if failure is None else "failed",
                step_count=len(steps),
                successful_steps=successful_steps if failure is not None else len(steps),
                failed_step=failed_step,
                smoke_command=f"python3 scripts/current_l2_guided_samples.py smoke {spec.problem_id}",
                sample_bundle_doc=PROBLEM_SAMPLE_BUNDLE_DOCS[spec.problem_id],
                primary_samples=[sample.sample_id for sample in spec.samples if sample.primary],
                step_labels=[step.label for step in steps],
                failed_command=" ".join(steps[successful_steps].command) if failure is not None else None,
                failed_return_code=failure.returncode if failure is not None else None,
                failed_output_excerpt=failure_output_excerpt(failure),
            )
        )
    return rows


def render_problem_smoke_aggregate(rows: list[ProblemSmokeAggregateRow]) -> str:
    lines = [
        "representative problem bundle aggregate smoke summary",
        "",
        "Problem 1 / Problem 2 の representative smoke 実行結果を compact に俯瞰するための repo-local summary。",
        "",
    ]
    for row in rows:
        lines.append(
            f"- {row.problem_id}: {row.status} ({row.successful_steps}/{row.step_count} steps)"
        )
        lines.append(f"  title: {row.title}")
        lines.append(f"  smoke command: {row.smoke_command}")
        lines.append(f"  sample bundle doc: {row.sample_bundle_doc}")
        lines.append(f"  primary samples: {', '.join(row.primary_samples)}")
        lines.append(f"  steps: {', '.join(row.step_labels)}")
        if row.failed_step is not None:
            lines.append(f"  failed step: {row.failed_step}")
        if row.failed_command is not None:
            lines.append(f"  failed command: {row.failed_command}")
        if row.failed_return_code is not None:
            lines.append(f"  failed return code: {row.failed_return_code}")
        if row.failed_output_excerpt is not None:
            lines.append(f"  failure excerpt: {row.failed_output_excerpt}")
        lines.append("")
    lines.extend(
        [
            "current recommendation:",
            "- `smoke problem1` / `smoke problem2` 自体は残しつつ、aggregate 側は compact status 読みだけに留める。",
            "- exhaustive workflow automation、aggregate CI contract、final public CLI / tutorial surface には上げない。",
        ]
    )
    return "\n".join(lines)


def render_problem_smoke_aggregate_from_runtime(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
) -> str:
    _, rendered = run_problem_smoke_aggregate(specs, output_format=output_format)
    return rendered


def aggregate_smoke_exit_code(rows: list[ProblemSmokeAggregateRow]) -> int:
    return 1 if any(row.status != "passed" for row in rows) else 0


def run_problem_smoke_aggregate(
    specs: Mapping[str, ProblemSpec],
    *,
    output_format: str,
    runner: Callable[..., subprocess.CompletedProcess[str]] = subprocess.run,
) -> tuple[int, str]:
    rows = build_problem_smoke_aggregate_rows(specs, runner=runner)
    if output_format == "json":
        rendered = json.dumps([asdict(row) for row in rows], ensure_ascii=False, indent=2)
    else:
        rendered = render_problem_smoke_aggregate(rows)
    return aggregate_smoke_exit_code(rows), rendered


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

    emit_theorem_parser = subparsers.add_parser(
        "emit-theorem",
        help="Problem 1 theorem-first pilot の emitted artifact loop を repo-local output dir に materialize する",
    )
    emit_theorem_parser.add_argument("problem_id", choices=("problem1",))
    emit_theorem_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")
    emit_theorem_parser.add_argument(
        "--output-dir",
        type=Path,
        default=default_problem1_theorem_emit_output_dir(),
    )

    emit_scenario_parser = subparsers.add_parser(
        "emit-scenario",
        help="Problem 2 authoritative-room current default を repo-local output dir に materialize する",
    )
    emit_scenario_parser.add_argument("problem_id", choices=("problem2",))
    emit_scenario_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")
    emit_scenario_parser.add_argument(
        "--output-dir",
        type=Path,
        default=default_problem2_scenario_emit_output_dir(),
    )

    quickstart_parser = subparsers.add_parser(
        "quickstart",
        help="representative sample bundle の最短 4 ステップを helper-side summary として表示する",
    )
    quickstart_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))
    quickstart_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    quickstart_parity_parser = subparsers.add_parser(
        "quickstart-parity",
        help="sample bundle doc と quickstart helper の 4-step parity を表示する",
    )
    quickstart_parity_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    reopen_map_parser = subparsers.add_parser(
        "reopen-map",
        help="Problem 1 / Problem 2 の mixed-gate reopen point を aggregate summary で表示する",
    )
    reopen_map_parser.add_argument("problem_id", nargs="?", choices=sorted(problem_specs().keys()))
    reopen_map_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    residuals_parser = subparsers.add_parser(
        "residuals",
        help="remaining mixed gate と true user-spec residual を圧縮表示する",
    )
    residuals_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    closeout_parser = subparsers.add_parser(
        "closeout",
        help="once-through closeout の current reading を executable loop 起点で圧縮表示する",
    )
    closeout_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    lane_parser = subparsers.add_parser(
        "lane",
        help="remaining residual lane を 1 本ずつ表示する",
    )
    lane_parser.add_argument("lane_id", choices=residual_lane_ids())
    lane_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    split_parser = subparsers.add_parser(
        "split",
        help="next split package を problem ごとの narrow helper summary で表示する",
    )
    split_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))
    split_parser.add_argument("split_id", choices=problem_split_package_ids())
    split_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    mapping_parser = subparsers.add_parser(
        "mapping",
        help="parser companion representative slice の mapping matrix を表示する",
    )
    mapping_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    smoke_parser = subparsers.add_parser(
        "smoke",
        help="representative problem bundle guide に書いた主要 command 群を smoke 実行する",
    )
    smoke_parser.add_argument("problem_id", choices=sorted(problem_specs().keys()))

    smoke_all_parser = subparsers.add_parser(
        "smoke-all",
        help="Problem 1 / Problem 2 の representative smoke をまとめて実行し compact summary を出す",
    )
    smoke_all_parser.add_argument("--format", choices=("pretty", "json"), default="pretty")

    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(argv or sys.argv[1:])
    specs = problem_specs()

    if args.subcommand == "list":
        for problem_id, spec in specs.items():
            print(f"{problem_id}: {spec.summary}")
        return 0

    if args.subcommand == "mapping":
        manifest = build_parser_companion_mapping_manifest()
        if args.format == "json":
            print(json.dumps(manifest, ensure_ascii=False, indent=2))
            return 0
        print(render_parser_companion_mapping())
        return 0

    if args.subcommand == "smoke-all":
        exit_code, rendered = run_problem_smoke_aggregate(specs, output_format=args.format)
        print(rendered)
        return exit_code

    if args.subcommand == "quickstart-parity":
        print(render_problem_quickstart_parity_from_runtime(specs, output_format=args.format))
        return 0

    if args.subcommand == "reopen-map":
        selected_specs = specs if args.problem_id is None else {args.problem_id: specs[args.problem_id]}
        print(render_problem_reopen_map_from_runtime(selected_specs, output_format=args.format))
        return 0

    if args.subcommand == "residuals":
        print(render_remaining_residual_lane_summary_from_runtime(specs, output_format=args.format))
        return 0

    if args.subcommand == "closeout":
        print(render_once_through_closeout_summary_from_runtime(specs, output_format=args.format))
        return 0

    if args.subcommand == "lane":
        try:
            print(render_residual_lane_from_runtime(args.lane_id, output_format=args.format))
            return 0
        except KeyError as error:
            print(str(error), file=sys.stderr)
            return 1

    if args.subcommand == "split":
        try:
            print(
                render_problem_split_package_from_runtime(
                    args.problem_id,
                    args.split_id,
                    output_format=args.format,
                )
            )
            return 0
        except KeyError as error:
            print(str(error), file=sys.stderr)
            return 1

    spec = specs[args.problem_id]
    if args.subcommand == "quickstart":
        print(render_problem_quickstart_from_runtime(spec, output_format=args.format))
        return 0

    if args.subcommand == "show":
        print(render_problem_guide(spec))
        return 0

    if args.subcommand == "smoke":
        return run_problem_smoke(spec)

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

    if args.subcommand == "emit-theorem":
        try:
            print(
                render_problem1_theorem_emit_from_runtime(
                    spec,
                    output_format=args.format,
                    output_dir=args.output_dir,
                )
            )
            return 0
        except RuntimeError as error:
            print(str(error), file=sys.stderr)
            return 1

    if args.subcommand == "emit-scenario":
        try:
            print(
                render_problem2_scenario_emit_from_runtime(
                    spec,
                    output_format=args.format,
                    output_dir=args.output_dir,
                )
            )
            return 0
        except RuntimeError as error:
            print(str(error), file=sys.stderr)
            return 1

    return run_problem(spec, output_format=args.format, include_all=args.all)


if __name__ == "__main__":
    raise SystemExit(main())
