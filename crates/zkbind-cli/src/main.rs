use std::env;
use std::path::PathBuf;
use std::process::ExitCode;
use zkbind_core::{scan_project, ScanReport};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Human,
    Json,
}

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(message) => {
            eprintln!("error: {message}");
            ExitCode::FAILURE
        }
    }
}

fn run(arguments: Vec<String>) -> Result<(), String> {
    if arguments.is_empty() || matches!(arguments[0].as_str(), "-h" | "--help") {
        print_help();
        return Ok(());
    }

    if matches!(arguments[0].as_str(), "-V" | "--version") {
        println!("zkbind {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    if arguments[0] != "scan" {
        return Err(format!(
            "unknown command '{}'; expected 'scan'",
            arguments[0]
        ));
    }

    if arguments[1..]
        .iter()
        .any(|argument| matches!(argument.as_str(), "-h" | "--help"))
    {
        print_help();
        return Ok(());
    }

    let (path, format) = parse_scan_arguments(&arguments[1..])?;
    let report = scan_project(path).map_err(|error| format!("scan failed: {error}"))?;

    match format {
        OutputFormat::Human => print_human_report(&report),
        OutputFormat::Json => println!("{}", report.to_json()),
    }

    Ok(())
}

fn parse_scan_arguments(arguments: &[String]) -> Result<(PathBuf, OutputFormat), String> {
    let mut path = None;
    let mut format = OutputFormat::Human;
    let mut index = 0;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--format" => {
                index += 1;
                let value = arguments
                    .get(index)
                    .ok_or_else(|| "--format requires 'human' or 'json'".to_owned())?;
                format = match value.as_str() {
                    "human" => OutputFormat::Human,
                    "json" => OutputFormat::Json,
                    other => return Err(format!("unsupported output format '{other}'")),
                };
            }
            value if value.starts_with('-') => {
                return Err(format!("unknown option '{value}'"));
            }
            value => {
                if path.replace(PathBuf::from(value)).is_some() {
                    return Err("scan accepts exactly one project path".to_owned());
                }
            }
        }
        index += 1;
    }

    Ok((path.unwrap_or_else(|| PathBuf::from(".")), format))
}

fn print_human_report(report: &ScanReport) {
    println!("ZKBind scan");
    println!("root: {}", report.root.display());
    println!("Solidity files: {}", report.scanned_files);
    println!("verifier call sites: {}", report.findings.len());

    for finding in &report.findings {
        println!(
            "{}:{}:{}  {}  {}",
            finding.location.path.display(),
            finding.location.line,
            finding.location.column,
            finding.rule_id,
            finding.title
        );
    }
}

fn print_help() {
    println!(
        "ZKBind — cross-layer security analysis for zero-knowledge proof integrations\n\nUSAGE:\n    zkbind scan [PATH] [--format human|json]\n\nCOMMANDS:\n    scan    Discover Solidity verifier call sites in a project\n\nOPTIONS:\n    -h, --help       Print help\n    -V, --version    Print version\n        --format     Output format (default: human)"
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_json_output_format() {
        let arguments = vec![
            "contracts".to_owned(),
            "--format".to_owned(),
            "json".to_owned(),
        ];
        let (path, format) = parse_scan_arguments(&arguments).expect("arguments must parse");

        assert_eq!(path, PathBuf::from("contracts"));
        assert_eq!(format, OutputFormat::Json);
    }

    #[test]
    fn rejects_multiple_paths() {
        let arguments = vec!["one".to_owned(), "two".to_owned()];
        assert!(parse_scan_arguments(&arguments).is_err());
    }
}
