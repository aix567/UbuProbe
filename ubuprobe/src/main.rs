use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ubuprobe", version, about = "Read-only Ubuntu/Debian DEP-8 test inspector")]
struct Cli { #[command(subcommand)] command: Commands }

#[derive(Subcommand)]
enum Commands {
    /// Inspect a debian/tests/control file; no tests are executed.
    Inspect { #[arg(default_value = "debian/tests/control")] control: PathBuf, #[arg(long)] json: bool, #[arg(long)] strict: bool },
    /// Print a test plan without executing anything.
    Plan { #[arg(long, default_value = ".")] source: PathBuf, #[arg(long)] image: PathBuf },
}

fn main() {
    if let Err(err) = run() { eprintln!("error: {err}"); std::process::exit(1); }
}
fn run() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Inspect { control, json, strict } => {
            let report = ubuprobe::dep8::parse_file(&control)?;
            if json { println!("{}", serde_json::to_string_pretty(&report)?); }
            else {
                println!("{} DEP-8 stanza(s) in {}", report.stanzas.len(), control.display());
                for (i, t) in report.stanzas.iter().enumerate() {
                    println!("{}: line={} tests={:?} command={:?} depends={:?}", i + 1, t.line, t.tests, t.test_command, t.depends);
                }
                for d in &report.diagnostics { eprintln!("{} [{}] line {}: {}", d.severity, d.code, d.line, d.message); }
            }
            if strict && !report.diagnostics.is_empty() {
                return Err(format!("{} diagnostic(s); strict mode failed", report.diagnostics.len()).into());
            }
        }
        Commands::Plan { source, image } => {
            println!("Plan only — no test execution.");
            println!("Selected source: {}", source.display());
            println!("Selected QEMU image: {}", image.display());
            println!("Suggested command: autopkgtest <source-package> -- qemu <testbed-image>");
        }
    }
    Ok(())
}
