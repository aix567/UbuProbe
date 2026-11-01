# C helper (educational)

Build: `make -C c`
Run: `./c/inspect-deb path/to/package.deb`

This tiny C11 program demonstrates controlled binary reads and checks only the ar archive signature. It is not a full Debian `.deb` validator, and it is not part of the Rust CLI yet. Do not use it as a security acceptance decision.
