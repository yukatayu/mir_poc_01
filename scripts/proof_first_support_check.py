#!/usr/bin/env python3
"""Execute LAB Lean/Python support comparison; not a Python refinement proof.

All compiler outputs and copied reference inputs go in a fresh work directory.
The handoff is read-only. No production executor or source semantics is inferred.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile


def sha(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    if sys.flags.optimize:
        raise RuntimeError("Run without Python -O: countermodel assertions are required")
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--work-root", type=Path, required=True)
    parser.add_argument("--manifest", type=Path,
                        help="Frozen LAB cut manifest; defaults to docs/proof-first/SUPPORT_CUT.json")
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    if not args.work_root.is_dir():
        parser.error("--work-root must be an existing work directory")
    work = Path(tempfile.mkdtemp(prefix="support-check-", dir=args.work_root.resolve()))
    lean = subprocess.check_output(["elan", "which", "lean"], cwd=root, text=True).strip()
    version = subprocess.check_output([lean, "--version"], text=True).strip()
    manifest_path = args.manifest or root / "docs/proof-first/SUPPORT_CUT.json"
    manifest_bytes = manifest_path.read_bytes()
    manifest = json.loads(manifest_bytes)
    proof_paths = ["samples/lean/foundations/" + name for name in
                   ("MirroreaProofFirstSupport.lean", "MirroreaProofFirstSupportDifferential.lean")]
    reference_path = "sub-agent-pro/mirrorea-proof-first-handoff-v1/materials/mir_foundation_F0_3/model/support.py"
    runner_path = "scripts/proof_first_support_check.py"
    expected_paths = set(proof_paths + [reference_path, runner_path])
    if manifest["schema"] != "mirrorea-proof-first-support-cut-1" or set(manifest["sources"]) != expected_paths:
        raise RuntimeError("Unsupported or incomplete cut manifest")
    if version != manifest["lean_version"]:
        raise RuntimeError("Lean toolchain differs from the frozen cut")
    sources = {}
    for name in sorted(expected_paths):
        data = (root / name).read_bytes()
        digest = hashlib.sha256(data).hexdigest()
        if digest != manifest["sources"][name]:
            raise RuntimeError("Source differs from cut manifest: " + name)
        copied = work / Path(name).name
        copied.write_bytes(data)
        if sha(copied) != digest:
            raise RuntimeError("Executed copy differs from frozen input: " + name)
        sources[name] = digest
    (work / "SUPPORT_CUT.json").write_bytes(manifest_bytes)
    with (work / "lean-proof.log").open("w") as out:
        subprocess.run([lean, "--trust=0", "-o", "MirroreaProofFirstSupport.olean",
                        "MirroreaProofFirstSupport.lean"], cwd=work, stdout=out,
                       stderr=subprocess.STDOUT, check=True)
    with (work / "lean-cases.csv").open("w") as out:
        import os
        env = dict(os.environ, LEAN_PATH=str(work))
        subprocess.run([lean, "--run", "MirroreaProofFirstSupportDifferential.lean"],
                       cwd=work, env=env, stdout=out, check=True)
    spec = importlib.util.spec_from_file_location("proof_first_support_reference", work / "support.py")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    m = module
    keys = ("a", "b", "c")
    choices = (m.Top(), m.Bot(), m.Ref("a"), m.Ref("b"), m.Ref("c"),
               m.All(m.Ref("a"), m.Ref("b")), m.Any(m.Ref("b"), m.Ref("c")))
    seen = set()
    for row in (work / "lean-cases.csv").read_text().splitlines():
        code, mask, bits, *ranks = map(int, row.split(","))
        if (code, mask) in seen or not 0 <= code < 343 or not 0 <= mask < 8:
            raise AssertionError("Duplicate or out-of-domain Lean case")
        seen.add((code, mask))
        forms = {k: choices[(code // 7**i) % 7] for i, k in enumerate(keys)}
        eligible = {k for i, k in enumerate(keys) if mask & (1 << i)}
        live, rank = m.derive(forms, eligible)
        actual_bits = sum(1 << i for i, k in enumerate(keys) if k in live)
        if (actual_bits, [rank.get(k, -1) for k in keys]) != (bits, ranks):
            raise AssertionError((code, mask, live, rank, bits, ranks))
        if not m.check_closure(forms, eligible, live, rank):
            raise AssertionError("Python rejected generated certificate")
    if len(seen) != 2744:
        raise AssertionError(f"Incomplete enumeration: {len(seen)}")

    # Oracle countermodels: test the claimed distinction, not merely acceptance.
    forms = {"s": m.Top(), "a": m.Any(m.Ref("s"), m.Ref("b")), "b": m.Ref("a")}
    live, _ = m.derive(forms, set(forms))
    retained = set(live) - {"s"}
    while True:
        reduced = {k for k in retained if m.holds(forms[k], retained)}
        if reduced == retained:
            break
        retained = reduced
    after, _ = m.derive(forms, {"a", "b"})
    assert retained == {"a", "b"} and not after
    forms = {"p": m.Top(), "q": m.Top(), "u": m.Ref("q"),
             "v": m.Any(m.Ref("p"), m.Ref("u"))}
    before, r0 = m.derive(forms, set(forms))
    after, r1 = m.derive(forms, {"q", "u", "v"})
    assert before - {"p"} == after and r0["v"] == 1 and r1["v"] == 2
    try:
        m.check_closure(forms, after, after, {k: r0[k] for k in after})
    except m.Invalid:
        pass
    else:
        raise AssertionError("Restricted stale ranks accepted")
    forms = {"p": m.Top(), "q": m.Top(), "a": m.Any(m.Ref("p"), m.Ref("b")),
             "b": m.Any(m.Ref("q"), m.Ref("a"))}
    for eligible, expected_live in (({"q", "a", "b"}, {"q", "a", "b"}),
                                    ({"q", "a"}, {"q"}),
                                    ({"p", "a", "b"}, {"p", "a", "b"}),
                                    ({"p", "b"}, {"p"})):
        assert m.derive(forms, eligible)[0] == expected_live
    formula = m.Top()
    for _ in range(128):
        formula = m.All(formula)
    assert m.derive({"a": formula}, {"a"}) == (frozenset({"a"}), {"a": 0})
    try:
        m.derive({"a": m.All(formula)}, {"a"})
    except m.Invalid:
        pass
    else:
        raise AssertionError("Depth budget boundary changed")
    result = {"status": "PASS", "lean": version, "sources": sources,
              "manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
              "reference_sha256": sha(work / "support.py"), "differential_cases": len(seen),
              "countermodels": ["cyclic residual after root withdrawal", "survivor rank and low rank leak",
                                 "four-node uniform eligibility separator", "depth 128/129 boundary"],
              "csv_sha256": sha(work / "lean-cases.csv"),
              "limits": "Finite Python comparison, not general Python refinement or authority/runtime proof"}
    (work / "result.json").write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"work": str(work), **result}, indent=2))


if __name__ == "__main__":
    main()
