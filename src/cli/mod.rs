use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/**
 * Command line interface definitions and argument parsing for cargo-trust
 */

#[derive(Parser, Debug)]
#[command(
    name = "cargo-trust",
    bin_name = "cargo trust",
    about = "Security-focused tool for auditing unsafe code in Rust projects",
    version,
    author
)]
pub struct Args {
    #[arg(help = "Path to the Rust project to scan")]
    pub path: Option<PathBuf>,
    
    #[arg(
        short, 
        long, 
        value_enum, 
        default_value = "human",
        help = "Output format"
    )]
    pub format: OutputFormat,
    
    #[arg(
        short = 'v',
        long,
        help = "Verbose output with more details"
    )]
    pub verbose: bool,
    
    #[arg(
        long,
        help = "Include unsafe code snippets in output"
    )]
    pub show_code: bool,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum OutputFormat {
    Human,
    Json,
    Compact,
}