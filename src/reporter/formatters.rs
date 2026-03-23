use anyhow::Result;
use colored::Colorize;

use crate::scanner::ScanResult;
use super::{Formatter, make_relative_path};

/**
 * Different output formatters for displaying scan results
 */

pub struct HumanFormatter;

impl Formatter for HumanFormatter {
    fn format(results: &ScanResult) -> Result<()> {
        if results.is_empty() {
            println!("{}", "✓ No unsafe code found!".green().bold());
            return Ok(());
        }
        
        println!("{}", "╔══════════════════════════════════════╗".blue());
        println!("{}", "║     cargo-trust Security Report      ║".blue().bold());
        println!("{}", "╚══════════════════════════════════════╝".blue());
        println!();
        
        let mut file_count = 0;
        for (path, occurrences) in &results.files_with_unsafe {
            file_count += 1;
            let relative_path = make_relative_path(&results.project_root, path);
            
            println!("{} {}", "►".yellow(), relative_path.cyan().bold());
            
            for occurrence in occurrences {
                let kind_str = format!("{}", occurrence.kind);
                let location = format!("{}:{}", occurrence.line, occurrence.column);
                
                println!(
                    "  {} {} at {}",
                    "├─".bright_black(),
                    kind_str.red(),
                    location.bright_black()
                );
                
                if let Some(context) = &occurrence.context {
                    println!("  {}  {}", "│".bright_black(), context.bright_black());
                }
            }
            println!();
        }
        
        println!("{}", "═══════════════════════════════════════".blue());
        println!(
            "{} Found {} unsafe occurrences in {} files",
            "Summary:".yellow().bold(),
            results.total_unsafe_count.to_string().red().bold(),
            file_count.to_string().cyan().bold()
        );
        
        if !results.errors.is_empty() {
            println!();
            println!("{} {} files could not be parsed", 
                "Warnings:".yellow(), 
                results.errors.len()
            );
        }
        
        Ok(())
    }
}

pub struct JsonFormatter;

impl Formatter for JsonFormatter {
    fn format(results: &ScanResult) -> Result<()> {
        let json = serde_json::to_string_pretty(results)?;
        println!("{}", json);
        Ok(())
    }
}

pub struct CompactFormatter;

impl Formatter for CompactFormatter {
    fn format(results: &ScanResult) -> Result<()> {
        if results.is_empty() {
            println!("No unsafe code found");
            return Ok(());
        }
        
        for (path, occurrences) in &results.files_with_unsafe {
            let relative_path = make_relative_path(&results.project_root, path);
            for occurrence in occurrences {
                println!(
                    "{}:{}:{}: {}",
                    relative_path,
                    occurrence.line,
                    occurrence.column,
                    occurrence.kind
                );
            }
        }
        
        Ok(())
    }
}