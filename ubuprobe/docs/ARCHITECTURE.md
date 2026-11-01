# Architecture

```text
CLI -> configuration -> package metadata/DEP-8 parser -> qualification planner
                                              |
                                              v
                   QEMU isolation boundary <- runner interface (future)
                                              |
                         +--------------------+--------------------+
                         |                    |                    |
                        sbuild             autopkgtest         diffoscope
                         |                    |                    |
                         +--------- logs, artifacts, exit codes -----+
                                               |
                                    deterministic JSON report
```

Current v0.1 contains only CLI, DEP-8 parser and read-only planning placeholder.

## Planned types
- `PackageSource { path, source_name, version, distro, arch }`
- `TestCase { name, depends, restrictions, command }`
- `Testbed { image_digest, distro, arch, backend }`
- `Execution { argv, exit_status, elapsed_ms, stdout_path, stderr_path }`
- `QualificationReport { schema_version, source, testbed, findings, artifacts }`

## Design rules
- Explicit backend allowlist and argument-vector subprocess calls; never form shell strings.
- Untrusted package builds require disposable VM isolation and no host secrets.
- Preserve raw logs and package version/architecture/testbed image provenance.
- Separate backend failures from package test failures.
- Reports sorted and schema-versioned. Retries should be labeled, not overwrite failures.
- Dependent commands should be feature-detected, with clear missing-tool errors.
