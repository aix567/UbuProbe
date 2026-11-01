# Sprint 1 study guide

## DEP-8 concepts

`debian/tests/control` is a set of RFC822-style paragraphs. A stanza names test scripts with `Tests:` or an inline command with `Test-Command:` (not both). `Depends` declares prerequisites and supports Debian dependency syntax plus special tokens such as `@`. `Restrictions` request special capabilities, and `Features` indicate behaviors. The runner ultimately executes installed-package tests, not test code from the built source.

Example:

```text
Tests: smoke integration
Depends: @, python3 (>= 3.10)
Restrictions: needs-root, isolation-machine

Test-Command: myapp --version
Features: test-name=cli-version
```

## Architecture

Lexical scanner → paragraphs with line numbers → field extraction → conservative semantic validation → diagnostics → CLI JSON/text. `main.rs` handles CLI; `dep8.rs` never executes shell commands.

## Practice labs

1. Write fixtures for `Tests` and `Test-Command`.
2. Add a duplicate-field fixture and confirm the exact line.
3. Compare unknown restrictions and unknown field diagnostics.
4. Write a valid multiline dependency expression.
5. Run 40 regression tests, then `cargo fmt` and Clippy.
6. Add proptest or cargo-fuzz as a later exercise; do not claim fuzz coverage before running it.
7. Test with control files from real Debian/Ubuntu packages and compare behavior with the current autopkgtest parser.

## Contribution path

Choose a package with missing or broken autopkgtest coverage. Reproduce in a disposable Ubuntu VM, implement a small test/fix in the package source, submit via the package maintainer's upstream workflow or an Ubuntu merge proposal, and request sponsorship when needed. Do not submit UbuProbe itself as a proposed Ubuntu dependency without maintainer interest and sustained maintenance.

References:
- https://people.debian.org/~eriberto/README.package-tests.html
- https://ubuntu.com/project/docs/how-ubuntu-is-made/processes/automatic-package-testing-autopkgtest/
