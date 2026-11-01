use ubuprobe::dep8::parse;

#[test] fn empty_file() { assert!(parse("").unwrap().stanzas.is_empty()); }
#[test] fn comments_only() { assert!(parse("# hi\n# world\n").unwrap().stanzas.is_empty()); }
#[test] fn single_test() { assert_eq!(parse("Tests: smoke\n").unwrap().stanzas[0].tests, ["smoke"]); }
#[test] fn multiple_tests() { assert_eq!(parse("Tests: smoke integration\n").unwrap().stanzas[0].tests.len(), 2); }
#[test] fn comma_tests() { assert_eq!(parse("Tests: smoke, integration\n").unwrap().stanzas[0].tests.len(), 2); }
#[test] fn command() { assert_eq!(parse("Test-Command: true\n").unwrap().stanzas[0].test_command.as_deref(), Some("true")); }
#[test] fn multiple_stanzas() { assert_eq!(parse("Tests: a\n\nTests: b\n").unwrap().stanzas.len(), 2); }
#[test] fn blank_lines() { assert_eq!(parse("\n\nTests: a\n\n\n").unwrap().stanzas.len(), 1); }
#[test] fn crlf() { assert_eq!(parse("Tests: a\r\nDepends: @\r\n").unwrap().stanzas[0].depends, ["@"]); }
#[test] fn comments_between_fields() { assert_eq!(parse("Tests: a\n# comment\nDepends: @\n").unwrap().stanzas[0].depends, ["@"]); }
#[test] fn leading_comments() { assert_eq!(parse("# comment\nTests: a\n").unwrap().stanzas[0].line, 2); }
#[test] fn continuation() { assert_eq!(parse("Tests: a\nDepends: python3,\n libc6\n").unwrap().stanzas[0].depends, ["python3", "libc6"]); }
#[test] fn multiline_command() { assert!(parse("Test-Command: echo foo\n && echo bar\n").unwrap().stanzas[0].test_command.as_ref().unwrap().contains("echo bar")); }
#[test] fn case_insensitive_keys() { assert_eq!(parse("tests: a\nDEPENDS: @\n").unwrap().stanzas[0].depends, ["@"]); }
#[test] fn dependencies_with_or() { assert_eq!(parse("Tests: a\nDepends: foo | bar (>= 1), baz [amd64]\n").unwrap().stanzas[0].depends, ["foo | bar (>= 1)", "baz [amd64]"]); }
#[test] fn restrictions_spaces() { assert_eq!(parse("Tests: a\nRestrictions: needs-root isolation-machine\n").unwrap().stanzas[0].restrictions.len(), 2); }
#[test] fn restrictions_commas() { assert_eq!(parse("Tests: a\nRestrictions: needs-root, isolation-machine\n").unwrap().stanzas[0].restrictions.len(), 2); }
#[test] fn features() { assert_eq!(parse("Test-Command: true\nFeatures: test-name=smoke\n").unwrap().stanzas[0].features, ["test-name=smoke"]); }
#[test] fn architecture() { assert_eq!(parse("Tests: a\nArchitecture: amd64 arm64\n").unwrap().stanzas[0].architecture.as_deref(), Some("amd64 arm64")); }
#[test] fn tests_directory() { assert_eq!(parse("Tests: a\nTests-Directory: tests/integration\n").unwrap().stanzas[0].tests_directory.as_deref(), Some("tests/integration")); }
#[test] fn classes() { assert_eq!(parse("Tests: a\nClasses: desktop, graphics-driver\n").unwrap().stanzas[0].classes.len(), 2); }
#[test] fn unknown_field_warns() { assert_eq!(parse("Tests: a\nMystery: x\n").unwrap().diagnostics[0].code, "unknown-field"); }
#[test] fn unknown_restriction_warns() { assert_eq!(parse("Tests: a\nRestrictions: strange-new-rule\n").unwrap().diagnostics[0].code, "unknown-restriction"); }
#[test] fn known_restriction_no_warning() { assert!(parse("Tests: a\nRestrictions: needs-root\n").unwrap().diagnostics.is_empty()); }
#[test] fn comment_preserves_stanza() { assert_eq!(parse("Tests: a\n# hi\nDepends: @\n").unwrap().stanzas.len(), 1); }
#[test] fn missing_selector_rejected() { assert!(parse("Depends: @\n").is_err()); }
#[test] fn both_selectors_rejected() { assert!(parse("Tests: a\nTest-Command: true\n").is_err()); }
#[test] fn empty_tests_rejected() { assert!(parse("Tests: \n").is_err()); }
#[test] fn empty_command_rejected() { assert!(parse("Test-Command: \n").is_err()); }
#[test] fn bad_field_rejected() { assert!(parse("bad_field: x\n").is_err()); }
#[test] fn no_colon_rejected() { assert!(parse("Tests a\n").is_err()); }
#[test] fn duplicate_key_rejected() { assert!(parse("Tests: a\ntests: b\n").is_err()); }
#[test] fn orphan_continuation_rejected() { assert!(parse(" hello\n").is_err()); }
#[test] fn bad_test_name_rejected() { assert!(parse("Tests: ../oops\n").is_err()); }
#[test] fn unclosed_dependency_rejected() { assert!(parse("Tests: a\nDepends: foo (>= 1\n").is_err()); }
#[test] fn empty_dependency_rejected() { assert!(parse("Tests: a\nDepends: foo,,bar\n").is_err()); }
#[test] fn trailing_dependency_comma_rejected() { assert!(parse("Tests: a\nDepends: foo,\n").is_err()); }
#[test] fn absolute_tests_directory_rejected() { assert!(parse("Tests: a\nTests-Directory: /tmp\n").is_err()); }
#[test] fn traversal_tests_directory_rejected() { assert!(parse("Tests: a\nTests-Directory: ../temp\n").is_err()); }
#[test] fn reports_nonzero_line() { let e = parse("# hi\nDepends: @\n").unwrap_err().to_string(); assert!(e.contains("line 2")); }
#[test] fn huge_input_rejected() { let s = "x".repeat(2_000_001); assert!(parse(&s).is_err()); }
