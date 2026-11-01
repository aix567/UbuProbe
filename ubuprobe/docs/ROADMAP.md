# Milestones and exit criteria

1. **M0 CLI / Parser** (weeks 1–2): CLI, DEP-8 inspection, fixture and unit tests. Parser is a subset, not spec-complete.
2. **M1 Robust model** (weeks 3–4): DEP-8 compliance corpus, correct dependency grammar, malformed fixtures, coverage/fuzzing.
3. **M2 isolated runner** (weeks 5–7): QEMU testbed adapter, timeout/cancellation, logs, clear exit statuses.
4. **M3 package builds** (weeks 8–10): `sbuild`, lintian, source/build metadata, SHA-256 artifact digests.
5. **M4 qualification report** (weeks 11–12): schema version, JSON reports, comparison against baseline, failure grouping.
6. **M5 reproducibility** (weeks 13–15): two controlled builds, `diffoscope`, environment/provenance capture.
7. **M6 upstream** (weeks 16+): actual test regressions, upstream package patch and review via Launchpad/Debian.

Before tagging 1.0: pass conformance fixtures, verify isolation, run CI across Ubuntu versions, publish reproducible benchmarks, document all failure modes.
