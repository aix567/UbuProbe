//! Read-only DEP-8 control-file parser and conservative validator.
//! This is not a full dpkg dependency grammar validator or a test executor.
use serde::Serialize;
use std::{collections::BTreeMap, fs, path::Path};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ParseError {
    #[error("could not read {path}: {source}")]
    Io { path: String, #[source] source: std::io::Error },
    #[error("line {line}: {message}")]
    Invalid { line: usize, message: String },
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Diagnostic {
    pub line: usize,
    pub severity: String,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct TestDefinition {
    pub line: usize,
    pub tests: Vec<String>,
    pub test_command: Option<String>,
    /// Raw Debian dependency expressions split at top-level commas, NOT resolved.
    pub depends: Vec<String>,
    pub restrictions: Vec<String>,
    pub features: Vec<String>,
    pub architecture: Option<String>,
    pub tests_directory: Option<String>,
    pub classes: Vec<String>,
    pub other_fields: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct Inspection {
    pub stanzas: Vec<TestDefinition>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug)]
struct Field { name: String, value: String, line: usize }
#[derive(Debug)]
struct Paragraph { fields: Vec<Field>, line: usize }

pub fn parse_file(path: &Path) -> Result<Inspection, ParseError> {
    let input = fs::read_to_string(path).map_err(|source| ParseError::Io {
        path: path.display().to_string(), source
    })?;
    parse(&input)
}

fn invalid(line: usize, message: impl Into<String>) -> ParseError {
    ParseError::Invalid { line, message: message.into() }
}
fn allowed_key(k: &str) -> bool {
    !k.is_empty() && k.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'-')
}
fn tokens(s: &str) -> Vec<String> {
    s.split(|c: char| c.is_whitespace() || c == ',')
        .filter(|s| !s.is_empty()).map(str::to_owned).collect()
}
fn is_test_name(name: &str) -> bool {
    !name.is_empty() && name.bytes().all(|b| b.is_ascii_alphanumeric() || b"+.-".contains(&b))
}
fn split_dependencies(raw: &str, line: usize) -> Result<Vec<String>, ParseError> {
    if raw.trim().is_empty() { return Ok(vec![]); }
    let mut result = Vec::new();
    let mut start = 0;
    let mut depth = 0i32;
    let mut square = 0i32;
    let mut angle = 0i32;
    for (i, ch) in raw.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => depth -= 1,
            '[' => square += 1,
            ']' => square -= 1,
            '<' => angle += 1,
            '>' => angle -= 1,
            ',' if depth == 0 && square == 0 && angle == 0 => {
                let value = raw[start..i].trim();
                if value.is_empty() { return Err(invalid(line, "empty dependency between commas")); }
                result.push(value.to_owned());
                start = i + 1;
            }
            _ => {}
        }
        if depth < 0 || square < 0 || angle < 0 {
            return Err(invalid(line, "unbalanced dependency delimiters"));
        }
    }
    if depth != 0 || square != 0 || angle != 0 {
        return Err(invalid(line, "unbalanced dependency delimiters"));
    }
    let value = raw[start..].trim();
    if value.is_empty() { return Err(invalid(line, "trailing dependency comma")); }
    result.push(value.to_owned());
    Ok(result)
}

/// Parse and diagnose DEP-8 control syntax. Unrecognized fields are preserved.
/// Semantic decisions requiring an autopkgtest testbed are NOT performed here.
pub fn parse(input: &str) -> Result<Inspection, ParseError> {
    if input.len() > 2_000_000 { return Err(invalid(1, "control file exceeds 2 MB limit")); }
    let mut paragraphs = Vec::new();
    let mut fields: Vec<Field> = Vec::new();
    let mut start_line = 0;
    for (i, raw) in input.lines().enumerate() {
        let line = i + 1;
        let raw = raw.trim_end_matches('\r');
        if raw.trim().is_empty() {
            if !fields.is_empty() {
                paragraphs.push(Paragraph { fields: std::mem::take(&mut fields), line: start_line });
            }
            continue;
        }
        // RFC822-style comments may be interspersed between fields.
        if raw.starts_with('#') { continue; }
        if raw.starts_with(' ') || raw.starts_with('\t') {
            let field = fields.last_mut().ok_or_else(|| invalid(line, "continuation without a field"))?;
            // A single leading whitespace is folding syntax. Preserve remaining text.
            field.value.push('\n');
            field.value.push_str(&raw[1..]);
            continue;
        }
        let (name, value) = raw.split_once(':').ok_or_else(|| invalid(line, "expected Field: value"))?;
        if !allowed_key(name) { return Err(invalid(line, "invalid field name")); }
        if fields.iter().any(|f| f.name.eq_ignore_ascii_case(name)) {
            return Err(invalid(line, format!("duplicate field {name}")));
        }
        if fields.is_empty() { start_line = line; }
        fields.push(Field { name: name.to_owned(), value: value.trim_start().to_owned(), line });
    }
    if !fields.is_empty() { paragraphs.push(Paragraph { fields, line: start_line }); }

    let mut stanzas = Vec::new();
    let mut diagnostics = Vec::new();
    const KNOWN: &[&str] = &["Tests", "Test-Command", "Depends", "Restrictions", "Features", "Architecture", "Tests-Directory", "Classes"];
    const RESTRICTIONS: &[&str] = &["allow-stderr", "breaks-testbed", "build-needed", "flaky", "hint-testsuite-triggers", "isolation-container", "isolation-machine", "needs-internet", "needs-reboot", "needs-reboot-with-bootloader", "needs-root", "needs-sudo", "rw-build-tree", "skip-not-installable", "skippable", "superficial", "skip-foreign-architecture"];
    for para in paragraphs {
        let field = |key: &str| para.fields.iter().find(|f| f.name.eq_ignore_ascii_case(key));
        let value = |key: &str| field(key).map(|f| f.value.as_str());
        let tests = tokens(value("Tests").unwrap_or(""));
        let command = value("Test-Command").map(str::trim).filter(|s| !s.is_empty()).map(str::to_owned);
        let has_tests = field("Tests").is_some();
        let has_command = field("Test-Command").is_some();
        if has_tests == has_command { return Err(invalid(para.line, "expected exactly one of Tests or Test-Command")); }
        if has_tests && tests.is_empty() { return Err(invalid(para.line, "Tests is empty")); }
        if has_command && command.is_none() { return Err(invalid(para.line, "Test-Command is empty")); }
        for name in &tests {
            if !is_test_name(name) { return Err(invalid(field("Tests").unwrap().line, format!("invalid test name: {name}"))); }
        }
        let depends = split_dependencies(value("Depends").unwrap_or(""), field("Depends").map_or(para.line, |f| f.line))?;
        let restrictions = tokens(value("Restrictions").unwrap_or(""));
        for r in &restrictions {
            if !RESTRICTIONS.contains(&r.as_str()) {
                diagnostics.push(Diagnostic { line: field("Restrictions").unwrap().line, severity: "warning".into(), code: "unknown-restriction".into(), message: format!("unknown restriction '{r}' can cause autopkgtest to skip this test") });
            }
        }
        let mut other_fields = BTreeMap::new();
        for f in &para.fields {
            if !KNOWN.iter().any(|k| f.name.eq_ignore_ascii_case(k)) {
                diagnostics.push(Diagnostic { line: f.line, severity: "warning".into(), code: "unknown-field".into(), message: format!("unknown field '{}' can cause autopkgtest to skip this stanza", f.name) });
                other_fields.insert(f.name.clone(), f.value.clone());
            }
        }
        if let Some(d) = value("Tests-Directory") {
            let p = Path::new(d.trim());
            if d.trim().is_empty() || p.is_absolute() || d.split('/').any(|c| c == "..") {
                return Err(invalid(field("Tests-Directory").unwrap().line, "Tests-Directory must be a nonempty relative path without '..'"));
            }
        }
        stanzas.push(TestDefinition {
            line: para.line, tests, test_command: command, depends, restrictions,
            features: tokens(value("Features").unwrap_or("")),
            architecture: value("Architecture").map(str::trim).map(str::to_owned),
            tests_directory: value("Tests-Directory").map(str::trim).map(str::to_owned),
            classes: tokens(value("Classes").unwrap_or("")), other_fields,
        });
    }
    Ok(Inspection { stanzas, diagnostics })
}
