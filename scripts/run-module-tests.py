#!/usr/bin/env python3
"""Run a module's `tests.toml` cases against its built `.surli` artifact.

Usage:
    scripts/run-module-tests.py <module-dir> [--surreal <path>] [--build]

Each module directory holds a `tests.toml` listing the cases to run:

    [[test]]
    name = "compares two versions"      # optional label used in output
    fn = "compare"                      # exported function name
    args = ["'1.2.3'", "'1.2.4'"]       # SurrealQL literals, one per parameter
    expect = "-1"                       # exact match against the returned value

Assertions, in order of precedence (pick one; omitting all of them just
asserts the call succeeds):

    expect   = "-1"           value matches exactly
    contains = "ok"           value contains this substring
    error    = true           the call must fail
    error    = "must be < size"   the call must fail with this text in stderr

Functions that call a host function (`http::post`, `http::get`) are fed a
mocked response instead of reaching the network:

    [[test]]
    fn = "send"
    args = ["'https://example.com/hook'", "'hi'"]
    stdin = "{ ok: true }"

Arguments are passed to the CLI verbatim as single argv entries, so no shell
quoting is applied. A negative number must be wrapped in parentheses --
`"(-5)"` -- because a bare `-5` is parsed as a flag.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
import tomllib
from pathlib import Path

PASS = "\033[32m✓\033[0m"
FAIL = "\033[31m✗\033[0m"


def build(module: Path, surreal: str) -> Path:
    artifact = module / f"{module.name}.surli"
    result = subprocess.run(
        [surreal, "module", "build", "--debug", "-o", artifact.name, "."],
        cwd=module,
        capture_output=True,
        text=True,
    )
    if result.returncode != 0:
        sys.exit(f"{FAIL} {module.name}: build failed\n{result.stdout}\n{result.stderr}")
    return artifact


def run_case(case: dict, module: Path, artifact: Path, surreal: str) -> tuple[bool, str]:
    """Runs one case, returning (passed, detail)."""
    command = [surreal, "module", "run", "--fnc", case["fn"]]
    for arg in case.get("args", []):
        command += ["--arg", arg]
    command += ["--log", "none", str(artifact)]

    result = subprocess.run(
        command,
        cwd=module,
        input=case.get("stdin", ""),
        capture_output=True,
        text=True,
        timeout=case.get("timeout", 120),
    )

    expected_error = case.get("error", False)
    failed = result.returncode != 0

    if expected_error:
        if not failed:
            return False, f"expected an error, got success: {value_of(result.stdout)!r}"
        if isinstance(expected_error, str) and expected_error not in result.stderr:
            return False, f"expected error containing {expected_error!r}, got: {result.stderr.strip()!r}"
        return True, ""

    if failed:
        return False, f"call failed: {result.stderr.strip()}"

    value = value_of(result.stdout)
    if value is None:
        return False, "no returned value found in output"

    if "expect" in case and value != case["expect"]:
        return False, f"expected {case['expect']!r}, got {value!r}"
    if "contains" in case and case["contains"] not in value:
        return False, f"expected value containing {case['contains']!r}, got {value!r}"
    return True, ""


def value_of(stdout: str) -> str | None:
    """Extracts the returned value, which the CLI prints after a check mark."""
    marker = stdout.find("✅")
    if marker == -1:
        return None
    return stdout[marker + len("✅") :].strip()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("module", type=Path)
    parser.add_argument("--surreal", default="surreal", help="path to the surreal CLI")
    parser.add_argument("--build", action="store_true", help="build before running")
    args = parser.parse_args()

    module = args.module.resolve()
    manifest = module / "tests.toml"
    if not manifest.is_file():
        sys.exit(f"{FAIL} {module.name}: no tests.toml")

    cases = tomllib.loads(manifest.read_text()).get("test", [])
    if not cases:
        sys.exit(f"{FAIL} {module.name}: tests.toml defines no cases")

    artifact = build(module, args.surreal) if args.build else module / f"{module.name}.surli"
    if not artifact.is_file():
        sys.exit(f"{FAIL} {module.name}: {artifact.name} not found (pass --build to build it)")

    failures = 0
    for case in cases:
        label = case.get("name") or f"{case['fn']}({', '.join(case.get('args', []))})"
        passed, detail = run_case(case, module, artifact, args.surreal)
        if passed:
            print(f"  {PASS} {label}")
        else:
            failures += 1
            print(f"  {FAIL} {label}\n      {detail}")

    total = len(cases)
    print(f"{module.name}: {total - failures}/{total} passed")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
