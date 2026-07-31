use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    Info,
    Low,
    Medium,
    High,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Info => "info",
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
            Self::Critical => "critical",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Confidence {
    Low,
    Medium,
    High,
}

impl fmt::Display for Confidence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            Self::Low => "low",
            Self::Medium => "medium",
            Self::High => "high",
        };
        formatter.write_str(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLocation {
    pub path: PathBuf,
    pub line: usize,
    pub column: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub rule_id: String,
    pub title: String,
    pub severity: Severity,
    pub confidence: Confidence,
    pub message: String,
    pub location: SourceLocation,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScanReport {
    pub root: PathBuf,
    pub scanned_files: usize,
    pub findings: Vec<Finding>,
}

impl ScanReport {
    #[must_use]
    pub fn to_json(&self) -> String {
        let findings = self
            .findings
            .iter()
            .map(finding_to_json)
            .collect::<Vec<_>>()
            .join(",");

        format!(
            "{{\"schema_version\":\"0.1.0\",\"root\":\"{}\",\"scanned_files\":{},\"findings\":[{}]}}",
            escape_json(&self.root.display().to_string()),
            self.scanned_files,
            findings
        )
    }
}

pub fn scan_project(root: impl AsRef<Path>) -> io::Result<ScanReport> {
    let root = root.as_ref().canonicalize()?;
    let mut solidity_files = Vec::new();
    collect_solidity_files(&root, &mut solidity_files)?;
    solidity_files.sort();

    let mut report = ScanReport {
        root,
        scanned_files: solidity_files.len(),
        findings: Vec::new(),
    };

    for path in solidity_files {
        scan_solidity_file(&path, &mut report)?;
    }

    Ok(report)
}

fn collect_solidity_files(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if path.extension().and_then(|extension| extension.to_str()) == Some("sol") {
            files.push(path.to_path_buf());
        }
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();

        if entry_path.is_dir() {
            if is_ignored_directory(&entry_path) {
                continue;
            }
            collect_solidity_files(&entry_path, files)?;
        } else if entry_path
            .extension()
            .and_then(|extension| extension.to_str())
            == Some("sol")
        {
            files.push(entry_path);
        }
    }

    Ok(())
}

fn is_ignored_directory(path: &Path) -> bool {
    matches!(
        path.file_name().and_then(|name| name.to_str()),
        Some(".git" | "node_modules" | "target" | "out" | "cache")
    )
}

fn scan_solidity_file(path: &Path, report: &mut ScanReport) -> io::Result<()> {
    let source = fs::read_to_string(path)?;
    let report_path = path
        .strip_prefix(&report.root)
        .unwrap_or(path)
        .to_path_buf();

    for (line_index, line) in source.lines().enumerate() {
        for marker in [
            "verifyProof(",
            ".verifyProof(",
            "verify_proof(",
            ".verify_proof(",
        ] {
            if let Some(column) = line.find(marker) {
                report.findings.push(Finding {
                    rule_id: "ZKB000".to_owned(),
                    title: "Verifier call site discovered".to_owned(),
                    severity: Severity::Info,
                    confidence: Confidence::High,
                    message: "ZKBind identified a likely proof-verification call. This is inventory evidence, not a vulnerability.".to_owned(),
                    location: SourceLocation {
                        path: report_path.clone(),
                        line: line_index + 1,
                        column: column + 1,
                    },
                });
                break;
            }
        }
    }

    Ok(())
}

fn finding_to_json(finding: &Finding) -> String {
    format!(
        "{{\"rule_id\":\"{}\",\"title\":\"{}\",\"severity\":\"{}\",\"confidence\":\"{}\",\"message\":\"{}\",\"location\":{{\"path\":\"{}\",\"line\":{},\"column\":{}}}}}",
        escape_json(&finding.rule_id),
        escape_json(&finding.title),
        finding.severity,
        finding.confidence,
        escape_json(&finding.message),
        escape_json(&finding.location.path.display().to_string()),
        finding.location.line,
        finding.location.column
    )
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            character if character.is_control() => {
                use std::fmt::Write as _;
                let _ = write!(escaped, "\\u{:04x}", character as u32);
            }
            character => escaped.push(character),
        }
    }
    escaped
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn temporary_directory(test_name: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock must be after Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("zkbind-{test_name}-{unique}"))
    }

    #[test]
    fn discovers_solidity_verifier_call_sites() {
        let directory = temporary_directory("discovery");
        fs::create_dir_all(&directory).expect("fixture directory must be created");

        let contract = directory.join("Membership.sol");
        fs::write(
            &contract,
            "contract Membership { function claim() external { verifier.verifyProof(a, b, c, input); } }",
        )
        .expect("fixture contract must be written");

        let report = scan_project(&directory).expect("scan must succeed");

        assert_eq!(report.scanned_files, 1);
        assert_eq!(report.findings.len(), 1);
        assert_eq!(report.findings[0].rule_id, "ZKB000");
        assert_eq!(
            report.findings[0].location.path,
            PathBuf::from("Membership.sol")
        );
        assert_eq!(report.findings[0].location.line, 1);

        fs::remove_dir_all(directory).expect("fixture directory must be removed");
    }

    #[test]
    fn orders_findings_by_source_path() {
        let directory = temporary_directory("ordering");
        fs::create_dir_all(&directory).expect("fixture directory must be created");

        for name in ["ZVerifier.sol", "AVerifier.sol"] {
            fs::write(
                directory.join(name),
                "contract VerifierUser { function run() external { verifier.verifyProof(a, b, c, input); } }",
            )
            .expect("fixture contract must be written");
        }

        let report = scan_project(&directory).expect("scan must succeed");
        let paths = report
            .findings
            .iter()
            .map(|finding| finding.location.path.clone())
            .collect::<Vec<_>>();

        assert_eq!(
            paths,
            vec![
                PathBuf::from("AVerifier.sol"),
                PathBuf::from("ZVerifier.sol")
            ]
        );

        fs::remove_dir_all(directory).expect("fixture directory must be removed");
    }

    #[test]
    fn json_output_escapes_paths_and_messages() {
        let report = ScanReport {
            root: PathBuf::from("project\"name"),
            scanned_files: 0,
            findings: Vec::new(),
        };

        assert!(report.to_json().contains("project\\\"name"));
    }
}
