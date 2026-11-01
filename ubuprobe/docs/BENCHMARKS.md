# Benchmark methodology

Use fixed Ubuntu image digest, architecture, CPU/memory limits, test package version, and tool versions. Run each test at least 10 times after warmup; report median, p95, variance and failures.

Track: parser time per 1,000 stanzas; peak RSS; sbuild wall time; autopkgtest wall time; VM startup overhead; artifact size; stable report generation; concurrency scaling (1/2/4 workers). Separate cached from cold builds. Do not use benchmarks to make claims until real data are recorded.

Suggested fixtures: 10, 100 and 1,000 DEP-8 paragraphs; representative small source packages under a disposable QEMU testbed.
