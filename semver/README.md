# surrealism-semver

Semantic version parsing and comparison for Surrealism.

```surql
DEFINE MODULE mod::semver AS f"bucket:/semver.surli";

RETURN mod::semver::parse("1.2.3-alpha+build.1"); -- { major: 1, minor: 2, patch: 3, pre: "alpha", build: "build.1" }
RETURN mod::semver::compare("1.2.3", "1.2.4");    -- -1
RETURN mod::semver::satisfies("1.5.0", "^1.0.0"); -- true
```

| Function | Signature | Description |
|---|---|---|
| `parse` | `(version: string) -> object` | Parses a version into `major`, `minor`, `patch`, `pre`, `build` |
| `compare` | `(a: string, b: string) -> number` | Compares two versions: -1, 0, or 1 |
| `satisfies` | `(version: string, requirement: string) -> bool` | Whether the version matches the requirement (e.g. `^1.0.0`) |

`pre` and `build` are empty strings when the version has no pre-release or build metadata.
