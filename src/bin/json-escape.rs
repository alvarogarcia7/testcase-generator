use anyhow::Result;
use clap::Parser;
use std::io::{self, Read, Write};

#[derive(Parser)]
#[command(name = "json-escape")]
#[command(about = "Read stdin and perform JSON string escaping", version)]
#[command(
    after_help = "ENVIRONMENT VARIABLES:\n    RUST_LOG    Set log level (trace, debug, info, warn, error). Overrides --log-level"
)]
struct Cli {
    /// Test mode: validate that the escaped output is valid JSON when wrapped in quotes
    #[arg(short, long)]
    test: bool,

    /// Set log level (trace, debug, info, warn, error)
    #[arg(long, value_name = "LEVEL", default_value = "warn")]
    log_level: String,

    /// Enable verbose output (equivalent to --log-level=info)
    #[arg(short, long)]
    verbose: bool,
}

fn escape_json_string(input: &str) -> String {
    let mut output = String::with_capacity(input.len());

    for ch in input.chars() {
        match ch {
            '"' => output.push_str("\\\""),
            '\\' => output.push_str("\\\\"),
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            '\x08' => output.push_str("\\b"),
            '\x0C' => output.push_str("\\f"),
            c if c.is_control() => {
                output.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => output.push(c),
        }
    }

    output
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let log_level = if cli.verbose { "info" } else { &cli.log_level };
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(log_level)).init();

    log::info!("Reading from stdin");
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .map_err(|e| anyhow::anyhow!("Failed to read from stdin: {}", e))?;

    log::info!("Escaping JSON string");
    let escaped = escape_json_string(&input);

    if cli.test {
        log::info!("Running validation test");
        let test_json = format!("\"{}\"", escaped);

        match serde_json::from_str::<serde_json::Value>(&test_json) {
            Ok(_) => {
                log::info!("✓ Validation successful! Escaped string is valid JSON.");
            }
            Err(e) => {
                anyhow::bail!("✗ Validation failed: {}", e);
            }
        }
    }

    io::stdout()
        .write_all(escaped.as_bytes())
        .map_err(|e| anyhow::anyhow!("Failed to write to stdout: {}", e))?;

    Ok(())
}
