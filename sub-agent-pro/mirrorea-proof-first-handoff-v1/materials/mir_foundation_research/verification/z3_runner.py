"""Run SMT-LIB2 through an installed Z3 binary or libz3. No network required.
A solver result is not independent proof-kernel verification.
"""
from __future__ import annotations
import ctypes, ctypes.util, json, pathlib, shutil, subprocess, sys, os, hashlib

def run_text(text: str) -> tuple[str, str]:
    exe = shutil.which("z3")
    if exe:
        version = subprocess.run([exe, "-version"], capture_output=True, text=True, check=True).stdout.strip()
        p = subprocess.run([exe, "-in", "-smt2"], input=text, capture_output=True, text=True, timeout=30)
        if p.returncode != 0:
            raise RuntimeError(p.stdout + p.stderr)
        return p.stdout, version
    name = ctypes.util.find_library("z3")
    if not name:
        raise RuntimeError("Install Z3 (executable or shared library); no solver result has been produced.")
    lib = ctypes.CDLL(name)
    P = ctypes.c_void_p
    lib.Z3_mk_config.restype = P
    lib.Z3_set_param_value.argtypes = [P, ctypes.c_char_p, ctypes.c_char_p]
    lib.Z3_mk_context.argtypes = [P]; lib.Z3_mk_context.restype = P
    lib.Z3_del_config.argtypes = [P]; lib.Z3_del_context.argtypes = [P]
    lib.Z3_eval_smtlib2_string.argtypes = [P, ctypes.c_char_p]
    lib.Z3_eval_smtlib2_string.restype = ctypes.c_char_p
    lib.Z3_get_full_version.restype = ctypes.c_char_p
    cfg = lib.Z3_mk_config()
    lib.Z3_set_param_value(cfg, b"proof", b"true")
    ctx = lib.Z3_mk_context(cfg)
    lib.Z3_del_config(cfg)
    try:
        result = lib.Z3_eval_smtlib2_string(ctx, text.encode("utf-8"))
        return result.decode("utf-8"), lib.Z3_get_full_version().decode()
    finally:
        lib.Z3_del_context(ctx)

def main() -> None:
    root = pathlib.Path(__file__).resolve().parent
    manifest = json.loads((root / "manifest.json").read_text())
    start = int(os.environ.get("VC_START", "0"))
    stop = int(os.environ.get("VC_STOP", str(len(manifest))))
    selected = manifest[start:stop]
    results = []
    for item in selected:
        path = root / "vcs" / item["file"]
        # Each file in a child process, so malformed solver input cannot
        # corrupt the suite or silently make the remaining checks pass.
        p = subprocess.run([sys.executable, str(__file__), "--one", str(path)], capture_output=True, text=True, timeout=35)
        if p.returncode:
            raise RuntimeError(f"Solver invocation failed: {path.name}\n{p.stdout}\n{p.stderr}")
        record = json.loads(p.stdout)
        output = record["output"]
        lines = output.strip().splitlines()
        actual = lines[0].strip() if lines else "missing"
        passed = actual == item["expected"] and "(error" not in output
        (root / "logs" / (path.stem + ".txt")).write_text(output)
        results.append({**item, "actual": actual, "passed": passed, "solver": record["version"], "input_sha256": hashlib.sha256(path.read_bytes()).hexdigest()})
        print(f"{item['id']}: {actual} ({'PASS' if passed else 'FAIL'})")
    (root / (f"results_{start}_{stop}.json" if start != 0 or stop != len(manifest) else "results.json")).write_text(json.dumps(results, ensure_ascii=False, indent=2) + "\n")
    print(f"{sum(x['passed'] for x in results)}/{len(results)} expected results")
    if not all(x["passed"] for x in results):
        raise SystemExit(1)

if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "--one":
        output, version = run_text(pathlib.Path(sys.argv[2]).read_text())
        print(json.dumps({"output": output, "version": version}))
    else:
        main()
