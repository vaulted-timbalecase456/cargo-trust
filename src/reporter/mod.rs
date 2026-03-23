use anyhow::Result;
use std::path::Path;

use crate::cli::OutputFormat;
use crate::scanner::ScanResult;

mod formatters;

use formatters::{CompactFormatter, HumanFormatter, JsonFormatter};

/**
 * Reporting engine for displaying scan results in various formats
 */

pub struct Reporter {
    format: OutputFormat,
}

impl Reporter {
    pub fn new(format: OutputFormat) -> Self {
        Self { format }
    }
    
    pub fn report(&self, results: &ScanResult) -> Result<()> {
        match self.format {
            OutputFormat::Human => HumanFormatter::format(results),
            OutputFormat::Json => JsonFormatter::format(results),
            OutputFormat::Compact => CompactFormatter::format(results),
        }
    }
}

trait Formatter {
    fn format(results: &ScanResult) -> Result<()>;
}

fn make_relative_path(base: &Path, path: &Path) -> String {
    path.strip_prefix(base)
        .unwrap_or(path)
        .display()
        .to_string()
}