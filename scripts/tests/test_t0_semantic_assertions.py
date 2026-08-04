from __future__ import annotations

import importlib.util
import unittest
from copy import deepcopy
from pathlib import Path


REPO_ROOT = Path(__file__).resolve().parents[2]
PRODUCER = REPO_ROOT / "scripts" / "evaluate_t0_semantic_assertions.py"


def load_producer():
    specification = importlib.util.spec_from_file_location(
        "t0_semantic_assertions", PRODUCER
    )
    assert specification is not None and specification.loader is not None
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


VALID_SOURCES = {
    "CANON.md": "`mirrorea_canon/` is the normative canon. Everything outside is LAB.",
    "README.md": "`mirrorea_canon/` remains the sole normative source.",
    "mirrorea_canon/DESIGN-CONSTITUTION.md": (
        "Communication is a projection of checked meaning. `World`, `Room`, "
        "`Avatar`, and `Game` remain domain/library vocabulary, never Mir Core primitives."
    ),
    "mirrorea_canon/NORTH-STAR.md": "Do not start with an RPC/API boundary.",
    "mirrorea_canon/adr/ADR-0001.md": "World is not a core primitive.",
    "mirrorea_canon/adr/ADR-0012.md": "Canon is normative and LAB is evidence.",
    "mirrorea_canon/meta/source-hierarchy.md": "canon(規範) > LAB(evidence)",
    "samples/clean-near-end/README.md": "Canon note: this is LAB evidence.",
    "AGENTS.md": (
        "Canon notice: `mirrorea_canon/` is the normative source. "
        "Start with `mirrorea_canon/README.md`, then `mirrorea_canon/MAP.md`."
    ),
}


class T0SemanticAssertionTests(unittest.TestCase):
    def test_semantic_assertions_pass_and_ignore_unselected_wording(self) -> None:
        producer = load_producer()

        rows = producer.evaluate_assertions(VALID_SOURCES)
        self.assertEqual([row["result"] for row in rows], ["pass"] * 6)

        changed = dict(VALID_SOURCES)
        changed["README.md"] += "\nA reader-facing note changed without changing the assertion."
        self.assertEqual(rows, producer.evaluate_assertions(changed))

    def test_clean_suite_and_agent_hierarchy_drift_are_valid_failures(self) -> None:
        producer = load_producer()

        clean_drift = dict(VALID_SOURCES)
        clean_drift["samples/clean-near-end/README.md"] = "Current runnable sample."
        clean_rows = producer.evaluate_assertions(clean_drift)
        self.assertEqual(clean_rows[4]["id"], "SA-05-clean-suite-is-lab-evidence")
        self.assertEqual(clean_rows[4]["result"], "fail")

        agent_drift = dict(VALID_SOURCES)
        agent_drift["AGENTS.md"] = "Start from any file."
        agent_rows = producer.evaluate_assertions(agent_drift)
        self.assertEqual(agent_rows[5]["id"], "SA-06-agents-follow-source-hierarchy")
        self.assertEqual(agent_rows[5]["result"], "fail")

    def test_artifact_digest_and_tampering_are_detected(self) -> None:
        producer = load_producer()
        rows = producer.evaluate_assertions(VALID_SOURCES)
        artifact = producer.make_artifact(
            source_revision="a" * 40,
            profile_source_sha256="b" * 64,
            producer_sha256="c" * 64,
            rows=rows,
        )

        producer.validate_artifact(artifact)
        self.assertEqual(
            producer.canonical_artifact_bytes(artifact),
            producer.canonical_artifact_bytes(deepcopy(artifact)),
        )

        tampered = deepcopy(artifact)
        tampered["assertions"][0]["result"] = "fail"
        with self.assertRaisesRegex(ValueError, "artifact digest"):
            producer.validate_artifact(tampered)


if __name__ == "__main__":
    unittest.main()
