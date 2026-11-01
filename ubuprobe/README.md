# UbuProbe — Sprint 1

**Read-only DEP-8 inspection tool (prototype).** UbuProbe parses `debian/tests/control`, preserves source line numbers, outputs JSON, and reports unknown fields/restrictions without executing package code.

## Build and test

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo run -- inspect fixtures/sample/debian/tests/control --json
cargo run -- inspect fixtures/sample/debian/tests/control --strict
```

`--strict` exits with an error for warnings. Without `--strict`, unknown fields and restrictions are preserved as diagnostics. The CLI never executes test commands.

## Supported in Sprint 1

- RFC822-like fields and continuation lines; blank-line-separated stanzas; comments
- `Tests` / `Test-Command` exclusivity and nonempty selectors
- `Depends`, `Restrictions`, `Features`, `Architecture`, `Tests-Directory`, `Classes`
- Case-insensitive field-name matching, duplicate-field diagnostics, line locations
- Debian dependency groups preserved verbatim with conservative delimiter checks (NOT a full dpkg grammar validator)
- Unknown field/restriction warnings; stable JSON output and 40 focused regression tests

## Known limitations

- No package building, test execution, archive verification, or safe testbed yet.
- DEP-8 behavior depends on installed autopkgtest versions; restriction whitelist is advisory and requires upkeep.
- The dependency grammar, full architecture expressions, detailed feature constraints, and exact Debian control-file folding semantics are not exhaustively checked.
- No fuzz harness or performance data yet. Use `autopkgtest` as the behavioral authority.

## Security

Untrusted control files are *data*, never executed. Planned package execution must only occur in disposable isolated testbeds with no credentials or host mounts.

See `docs/SPRINT1.md` for a teaching walkthrough and contribution path.
