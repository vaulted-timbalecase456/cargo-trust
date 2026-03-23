use anyhow::Result;
use clap::Parser;
use colored::Colorize;
use std::path::PathBuf;
use std::process;

mod cli;
mod error;
mod parser;
mod reporter;
mod scanner;

use cli::Args;
use reporter::Reporter;
use scanner::Scanner;

/**
 * Entry point for cargo-trust - a security tool for auditing unsafe code in Rust projects
 */

fn main() {
    if let Err(e) = run() {
        eprintln!("{} {}", "error:".red().bold(), e);
        process::exit(1);
    }
}

fn run() -> Result<()> {
    let mut args_vec: Vec<String> = std::env::args().collect();
    
    if args_vec.len() > 1 && args_vec[1] == "trust" {
        args_vec.remove(1);
    }
    
    let args = Args::parse_from(&args_vec);
    
    let path = args.path.unwrap_or_else(|| PathBuf::from("."));
    
    if !path.exists() {
        return Err(anyhow::anyhow!("Path does not exist: {}", path.display()));
    }
    
    let scanner = Scanner::new(path);
    let results = scanner.scan()?;
    
    let reporter = Reporter::new(args.format);
    reporter.report(&results)?;
    
    Ok(())
}